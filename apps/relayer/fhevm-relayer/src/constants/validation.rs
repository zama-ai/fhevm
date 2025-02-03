#[cfg(test)]
pub mod validation {
    use crate::constants::DecryptionOracleEvents;
    use crate::constants::TfheExecutorEvents;
    use alloy::primitives::keccak256;

    impl DecryptionOracleEvents {
        /// Validates that the topic matches the event signature
        pub fn validate_topics() -> bool {
            let computed_topic = format!("0x{:x}", keccak256(Self::EVENT_SIGNATURE));
            computed_topic == Self::TOPIC_SIGNATURE
                && format!("0x{:x}", keccak256(Self::LEGACY_EVENT_SIGNATURE))
                    == Self::LEGACY_TOPIC_SIGNATURE
        }
    }

    impl TfheExecutorEvents {
        /// Validates that all event signatures are properly formatted
        pub fn validate_signatures() -> bool {
            Self::all_signatures()
                .iter()
                .all(|sig| sig.contains('(') && sig.contains(')') && sig.contains(','))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::constants;

    use super::*;

    #[test]
    fn test_decryption_oracle_topics() {
        assert!(constants::DecryptionOracleEvents);
    }

    #[test]
    fn test_tfhe_executor_signatures() {
        assert!(validation::TfheExecutorEvents::validate_signatures());
    }

    #[test]
    fn test_contract_addresses() {
        // Test address format
        assert!(ContractAddresses::DECRYPTION_ORACLE.starts_with("0x"));
        assert_eq!(ContractAddresses::DECRYPTION_ORACLE.len(), 42);

        assert!(ContractAddresses::TFHE_EXECUTOR.starts_with("0x"));
        assert_eq!(ContractAddresses::TFHE_EXECUTOR.len(), 42);
    }
}
