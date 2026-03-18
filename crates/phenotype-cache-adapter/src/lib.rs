//! Generic two-tier cache with L1 (LRU) + L2 (DashMap), TTL expiration, and metrics hooks.
//!
//! L1 is a bounded LRU cache for hot data (fast, capacity-limited).
//! L2 is a concurrent DashMap for warm data (unbounded, TTL-evicted).
//! On L1 miss + L2 hit, entries are promoted to L1.

use dashmap::DashMap;
use lru::LruCache;
use parking_lot::RwLock;
use std::hash::Hash;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Cache metrics for observability.
#[derive(Debug, Clone, Default)]
pub struct CacheMetrics {
    pub l1_hits: u64,
    pub l2_hits: u64,
    pub misses: u64,
    pub promotions: u64,
    pub evictions: u64,
    pub expirations: u64,
}

/// Metrics hook trait for pluggable observability.
pub trait MetricsHook: Send + Sync {
    fn on_l1_hit(&self) {}
    fn on_l2_hit(&self) {}
    fn on_miss(&self) {}
    fn on_promotion(&self) {}
    fn on_eviction(&self) {}
    fn on_expiration(&self) {}
}

/// No-op metrics hook (default).
struct NoopMetrics;
impl MetricsHook for NoopMetrics {}

#[derive(Clone)]
struct CacheEntry<V> {
    value: V,
    expires_at: Instant,
}

impl<V> CacheEntry<V> {
    fn new(value: V, ttl: Duration) -> Self {
        Self {
            value,
            expires_at: Instant::now() + ttl,
        }
    }

    fn is_expired(&self) -> bool {
        Instant::now() >= self.expires_at
    }
}

/// Two-tier cache: L1 (LRU, bounded) + L2 (DashMap, concurrent).
pub struct TieredCache<K, V>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    l1: Arc<RwLock<LruCache<K, CacheEntry<V>>>>,
    l2: Arc<DashMap<K, CacheEntry<V>>>,
    default_ttl: Duration,
    metrics: Arc<RwLock<CacheMetrics>>,
    hook: Arc<dyn MetricsHook>,
}

impl<K, V> TieredCache<K, V>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    /// Create a new cache with the given L1 capacity and default TTL.
    pub fn new(l1_capacity: usize, default_ttl: Duration) -> Self {
        Self {
            l1: Arc::new(RwLock::new(LruCache::new(
                NonZeroUsize::new(l1_capacity.max(1)).unwrap(),
            ))),
            l2: Arc::new(DashMap::new()),
            default_ttl,
            metrics: Arc::new(RwLock::new(CacheMetrics::default())),
            hook: Arc::new(NoopMetrics),
        }
    }

    /// Create with a custom metrics hook.
    pub fn with_metrics_hook(
        l1_capacity: usize,
        default_ttl: Duration,
        hook: Arc<dyn MetricsHook>,
    ) -> Self {
        Self {
            l1: Arc::new(RwLock::new(LruCache::new(
                NonZeroUsize::new(l1_capacity.max(1)).unwrap(),
            ))),
            l2: Arc::new(DashMap::new()),
            default_ttl,
            metrics: Arc::new(RwLock::new(CacheMetrics::default())),
            hook,
        }
    }

    /// Get a value from the cache. Checks L1 first, then L2 (promoting on hit).
    pub fn get(&self, key: &K) -> Option<V> {
        // Try L1
        {
            let mut l1 = self.l1.write();
            if let Some(entry) = l1.get(key) {
                if !entry.is_expired() {
                    self.metrics.write().l1_hits += 1;
                    self.hook.on_l1_hit();
                    return Some(entry.value.clone());
                }
                l1.pop(key);
                self.metrics.write().expirations += 1;
                self.hook.on_expiration();
            }
        }

        // Try L2
        if let Some(entry_ref) = self.l2.get(key) {
            if !entry_ref.is_expired() {
                let entry = entry_ref.clone();
                drop(entry_ref);

                // Promote to L1
                {
                    let mut l1 = self.l1.write();
                    l1.put(key.clone(), entry.clone());
                }
                let mut m = self.metrics.write();
                m.l2_hits += 1;
                m.promotions += 1;
                self.hook.on_l2_hit();
                self.hook.on_promotion();
                return Some(entry.value);
            }
            drop(entry_ref);
            self.l2.remove(key);
            self.metrics.write().expirations += 1;
            self.hook.on_expiration();
        }

        self.metrics.write().misses += 1;
        self.hook.on_miss();
        None
    }

    /// Insert a value with the default TTL.
    pub fn insert(&self, key: K, value: V) {
        self.insert_with_ttl(key, value, self.default_ttl);
    }

    /// Insert a value with a custom TTL.
    pub fn insert_with_ttl(&self, key: K, value: V, ttl: Duration) {
        let entry = CacheEntry::new(value, ttl);
        {
            let mut l1 = self.l1.write();
            l1.put(key.clone(), entry.clone());
        }
        self.l2.insert(key, entry);
    }

    /// Remove a key from both tiers.
    pub fn remove(&self, key: &K) {
        self.l1.write().pop(key);
        self.l2.remove(key);
        self.metrics.write().evictions += 1;
        self.hook.on_eviction();
    }

    /// Clear all entries from both tiers.
    pub fn clear(&self) {
        self.l1.write().clear();
        self.l2.clear();
    }

    /// Get a snapshot of current metrics.
    pub fn metrics(&self) -> CacheMetrics {
        self.metrics.read().clone()
    }

    /// Number of entries in L1.
    pub fn l1_len(&self) -> usize {
        self.l1.read().len()
    }

    /// Number of entries in L2.
    pub fn l2_len(&self) -> usize {
        self.l2.len()
    }

    /// Check if the cache contains a key (in either tier, not checking expiry).
    pub fn contains_key(&self, key: &K) -> bool {
        self.l1.read().contains(key) || self.l2.contains_key(key)
    }
}

impl<K, V> Default for TieredCache<K, V>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new(1000, Duration::from_secs(3600))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::thread;

    #[test]
    fn basic_insert_and_get() {
        let cache = TieredCache::new(100, Duration::from_secs(60));
        cache.insert("key1".to_string(), "value1".to_string());
        assert_eq!(cache.get(&"key1".to_string()), Some("value1".to_string()));
    }

    #[test]
    fn ttl_expiration() {
        let cache = TieredCache::new(100, Duration::from_millis(50));
        cache.insert("key1".to_string(), 42);
        assert_eq!(cache.get(&"key1".to_string()), Some(42));
        thread::sleep(Duration::from_millis(100));
        assert_eq!(cache.get(&"key1".to_string()), None);
    }

    #[test]
    fn l2_promotion_to_l1() {
        let cache = TieredCache::new(2, Duration::from_secs(60));
        cache.insert("a".to_string(), 1);
        cache.insert("b".to_string(), 2);
        cache.insert("c".to_string(), 3);
        assert_eq!(cache.get(&"a".to_string()), Some(1));
        let m = cache.metrics();
        assert!(m.promotions >= 1);
        assert!(m.l2_hits >= 1);
    }

    #[test]
    fn remove_from_both_tiers() {
        let cache = TieredCache::new(100, Duration::from_secs(60));
        cache.insert("k".to_string(), "v".to_string());
        cache.remove(&"k".to_string());
        assert_eq!(cache.get(&"k".to_string()), None);
    }

    #[test]
    fn metrics_tracking() {
        let cache = TieredCache::new(100, Duration::from_secs(60));
        cache.insert("k".to_string(), 1);
        cache.get(&"k".to_string());
        cache.get(&"missing".to_string());
        let m = cache.metrics();
        assert_eq!(m.l1_hits, 1);
        assert_eq!(m.misses, 1);
    }

    #[test]
    fn custom_metrics_hook() {
        struct TestHook {
            hits: AtomicU64,
        }
        impl MetricsHook for TestHook {
            fn on_l1_hit(&self) {
                self.hits.fetch_add(1, Ordering::Relaxed);
            }
        }

        let hook = Arc::new(TestHook {
            hits: AtomicU64::new(0),
        });
        let cache = TieredCache::with_metrics_hook(
            100,
            Duration::from_secs(60),
            hook.clone() as Arc<dyn MetricsHook>,
        );
        cache.insert("k".to_string(), 1);
        cache.get(&"k".to_string());
        assert_eq!(hook.hits.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn default_cache() {
        let cache: TieredCache<String, i32> = TieredCache::default();
        cache.insert("x".to_string(), 99);
        assert_eq!(cache.get(&"x".to_string()), Some(99));
    }
}
