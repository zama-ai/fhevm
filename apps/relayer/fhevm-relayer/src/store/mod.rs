//! Store module provides functionality to persist different types of data.
//!
//! It operates in two layers.
//! 1. A generic key value store layer. Eg: in-memory, rocks db etc.
//! 2. A data translation layer for storing different kinds of data . Eg: EventStore.

// Export the store components and traits
pub mod key_value_db;
