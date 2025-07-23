//! Backpressure signaling for queue management
//!
//! This module provides backpressure signals that allow components to communicate
//! queue capacity and adjust processing rates accordingly. Used primarily by the
//! polling system to dynamically adjust polling speed based on event queue capacity.

/// Backpressure signals for queue capacity management
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackpressureSignal {
    /// Queue is available for new items
    QueueAvailable,
    /// Queue is getting full, slow down processing
    QueueFull,
    /// Queue is at critical capacity, pause processing
    QueueCritical,
}
