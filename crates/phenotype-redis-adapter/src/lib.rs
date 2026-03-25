#![doc = include_str!("../README.md")]

pub mod error;
pub mod domain;
pub mod application;
pub mod adapters;

pub use error::{AdapterError, Result};
