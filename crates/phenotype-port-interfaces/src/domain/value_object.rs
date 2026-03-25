//! # Value Object
//!
//! Value objects are immutable objects defined by their attributes rather than identity.

/// Marker trait for value objects.
///
/// Value objects are compared by their attributes, not by identity.
/// They should be immutable and created via factory methods.
pub trait ValueObject: 'static + Send + Sync + Clone + PartialEq {
    /// Type of the value contained in this value object.
    type Value: Clone + PartialEq + Send + Sync;

    /// Returns the underlying value.
    fn value(&self) -> &Self::Value;
}

/// Extension trait for value objects with validation.
pub trait ValueObjectExt: ValueObject {
    /// Validates the value, returning an error message if invalid.
    fn validate(value: &Self::Value) -> crate::error::Result<()> {
        let _ = value;
        Ok(())
    }

    /// Creates a new value object, returning an error if invalid.
    fn new(value: Self::Value) -> crate::error::Result<Self>
    where
        Self: Sized,
    {
        Self::validate(&value)?;
        Self::new_unchecked(value)
    }

    /// Creates a new value object without validation.
    fn new_unchecked(value: Self::Value) -> Self;
}

impl<T: ValueObject> ValueObjectExt for T {
    fn new_unchecked(value: Self::Value) -> Self {
        panic!(
            "ValueObject::new_unchecked must be implemented for {}",
            std::any::type_name::<T>()
        )
    }
}
