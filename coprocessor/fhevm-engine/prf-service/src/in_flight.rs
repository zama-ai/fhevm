//! Allows one request for each connector and label at a time, in one pod.

use crate::path::PrfPath;
use alloy::primitives::{Address, B256};
use std::{collections::HashSet, sync::Mutex};

type Key = (Address, u16, B256);

#[derive(Default)]
pub struct InFlight {
    requests: Mutex<HashSet<Key>>,
}

impl InFlight {
    /// Starts a request of `connector`. Returns `None` if the connector already has a request for
    /// this label. Then the handler answers `429`.
    pub fn start(&self, connector: Address, path: &PrfPath) -> Option<InFlightRequest<'_>> {
        let key = (connector, path.prf, path.label);
        if !self.requests.lock().expect("requests lock").insert(key) {
            return None;
        }
        Some(InFlightRequest {
            in_flight: self,
            key,
        })
    }
}

/// A request in progress. On drop, it frees its place.
pub struct InFlightRequest<'a> {
    in_flight: &'a InFlight,
    key: Key,
}

impl Drop for InFlightRequest<'_> {
    fn drop(&mut self) {
        self.in_flight
            .requests
            .lock()
            .expect("requests lock")
            .remove(&self.key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONNECTOR: Address = Address::repeat_byte(0x11);

    fn path(label: u8) -> PrfPath {
        PrfPath {
            prf: 0,
            label: B256::repeat_byte(label),
            pad_length: 64,
        }
    }

    #[test]
    fn one_request_for_each_connector_and_label() {
        let in_flight = InFlight::default();
        let first = in_flight.start(CONNECTOR, &path(1));
        assert!(first.is_some());
        assert!(in_flight.start(CONNECTOR, &path(1)).is_none());
        // Another label, or another connector, is another request.
        assert!(in_flight.start(CONNECTOR, &path(2)).is_some());
        assert!(in_flight
            .start(Address::repeat_byte(0x22), &path(1))
            .is_some());
        drop(first);
        assert!(in_flight.start(CONNECTOR, &path(1)).is_some());
        assert!(in_flight.requests.lock().unwrap().is_empty());
    }
}
