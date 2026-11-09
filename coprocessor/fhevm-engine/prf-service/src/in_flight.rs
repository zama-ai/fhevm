//! Limits the requests in progress of each connector, in one pod.

use alloy::primitives::{Address, B256};
use ciphertext_attestation::PrfOutputRef;
use std::{
    collections::{HashMap, HashSet},
    num::NonZeroUsize,
    sync::Mutex,
};

type Label = (u16, B256);

pub struct InFlight {
    max_per_connector: NonZeroUsize,
    requests: Mutex<HashMap<Address, HashSet<Label>>>,
}

impl InFlight {
    pub fn new(max_per_connector: NonZeroUsize) -> Self {
        Self {
            max_per_connector,
            requests: Mutex::default(),
        }
    }

    /// Returns `None` if `connector` has a request for this label, or `max_per_connector` requests.
    pub fn start(&self, connector: Address, path: &PrfOutputRef) -> Option<InFlightRequest<'_>> {
        let label = (path.prf_id, path.label);
        let mut requests = self.requests.lock().expect("requests lock");
        let labels = requests.entry(connector).or_default();
        // The limit is at least 1, so a refusal never leaves `labels` empty.
        if labels.len() >= self.max_per_connector.get() || !labels.insert(label) {
            return None;
        }
        Some(InFlightRequest {
            in_flight: self,
            connector,
            label,
        })
    }
}

/// A request in progress. On drop, it frees its place.
pub struct InFlightRequest<'a> {
    in_flight: &'a InFlight,
    connector: Address,
    label: Label,
}

impl Drop for InFlightRequest<'_> {
    fn drop(&mut self) {
        let mut requests = self.in_flight.requests.lock().expect("requests lock");
        if let Some(labels) = requests.get_mut(&self.connector) {
            labels.remove(&self.label);
            if labels.is_empty() {
                requests.remove(&self.connector);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONNECTOR: Address = Address::repeat_byte(0x11);
    const OTHER_CONNECTOR: Address = Address::repeat_byte(0x22);

    fn in_flight(max_per_connector: usize) -> InFlight {
        InFlight::new(NonZeroUsize::new(max_per_connector).unwrap())
    }

    fn path(label: u8) -> PrfOutputRef {
        PrfOutputRef {
            prf_id: 0,
            label: B256::repeat_byte(label),
            pad_length: 64,
        }
    }

    #[test]
    fn one_request_for_each_connector_and_label() {
        let in_flight = in_flight(10);
        let first = in_flight.start(CONNECTOR, &path(1));
        assert!(first.is_some());
        assert!(in_flight.start(CONNECTOR, &path(1)).is_none());
        // Another label, or another connector, is another request.
        assert!(in_flight.start(CONNECTOR, &path(2)).is_some());
        assert!(in_flight.start(OTHER_CONNECTOR, &path(1)).is_some());
        drop(first);
        assert!(in_flight.start(CONNECTOR, &path(1)).is_some());
        assert!(in_flight.requests.lock().unwrap().is_empty());
    }

    #[test]
    fn limits_the_requests_of_each_connector() {
        let in_flight = in_flight(2);
        let first = in_flight.start(CONNECTOR, &path(1));
        let _second = in_flight.start(CONNECTOR, &path(2)).unwrap();
        assert!(in_flight.start(CONNECTOR, &path(3)).is_none());
        assert!(in_flight.start(OTHER_CONNECTOR, &path(3)).is_some());
        drop(first);
        assert!(in_flight.start(CONNECTOR, &path(3)).is_some());
    }
}
