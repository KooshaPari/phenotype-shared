//! # Phenotype Port Interfaces
//!
//! Shared port interfaces for **Hexagonal Architecture** across the Phenotype ecosystem.
//!
//! This crate provides **domain-agnostic, technology-agnostic** port interfaces that can be
//! implemented by any adapter (database, cache, HTTP client, file system, etc.).

// === DOMAIN LAYER ===
pub mod domain;

// === OUTBOUND PORTS (Driven/Secondary) ===
pub mod outbound;

// === INBOUND PORTS (Driving/Primary) ===
pub mod inbound;

// === SHARED / CROSS-CUTTING ===
pub mod shared;

// === ERROR TYPES ===
pub mod error;
