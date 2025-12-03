use crate::core::{errors::EventProcessingError, event::RelayerEvent};
use crate::orchestrator::traits::EventDispatcher;
use alloy::primitives::U256;
use std::fmt::Display;
use tracing::error;

/// Type-aware SQL error helpers for each handler type
pub mod sql_errors {
    use super::*;
    use std::sync::Arc;

    /// User decrypt specific SQL error handler - knows about indexer_id context
    pub async fn user_decrypt_sql_error<D, T: Display>(
        dispatcher: &Arc<D>,
        event: RelayerEvent,
        operation: &str,
        sql_error: T,
        indexer_id: Option<&[u8]>,
    ) where
        D: EventDispatcher<RelayerEvent>,
    {
        if let Some(id) = indexer_id {
            error!(
                job_id = %event.job_id,
                indexer_id = %hex::encode(id),
                sql_operation = %operation,
                sql_error = %sql_error,
                handler_type = "user_decrypt",
                "SQL operation failed"
            );
        } else {
            error!(
                job_id = %event.job_id,
                sql_operation = %operation,
                sql_error = %sql_error,
                handler_type = "user_decrypt",
                "SQL operation failed"
            );
        }

        let error_event =
            event.derive_next_event(crate::core::event::RelayerEventData::UserDecrypt(
                crate::core::event::UserDecryptEventData::Failed {
                    error: EventProcessingError::SqlOperationFailed {
                        operation: operation.to_string(),
                        reason: sql_error.to_string(),
                    },
                },
            ));

        if let Err(e) = dispatcher.dispatch_event(error_event).await {
            error!(?e, "Failed to dispatch SQL error event");
        }
    }

    /// Public decrypt specific SQL error handler - knows about decryption context
    pub async fn public_decrypt_sql_error<D, T: Display>(
        dispatcher: &Arc<D>,
        event: RelayerEvent,
        operation: &str,
        sql_error: T,
        context: Option<(&str, &str)>,
    ) where
        D: EventDispatcher<RelayerEvent>,
    {
        match context {
            Some((key, value)) => {
                error!(
                    job_id = %event.job_id,
                    sql_operation = %operation,
                    sql_error = %sql_error,
                    handler_type = "public_decrypt",
                    context_key = %key,
                    context_value = %value,
                    "SQL operation failed"
                );
            }
            None => {
                error!(
                    job_id = %event.job_id,
                    sql_operation = %operation,
                    sql_error = %sql_error,
                    handler_type = "public_decrypt",
                    "SQL operation failed"
                );
            }
        }

        let error_event =
            event.derive_next_event(crate::core::event::RelayerEventData::PublicDecrypt(
                crate::core::event::PublicDecryptEventData::Failed {
                    error: EventProcessingError::SqlOperationFailed {
                        operation: operation.to_string(),
                        reason: sql_error.to_string(),
                    },
                },
            ));

        if let Err(e) = dispatcher.dispatch_event(error_event).await {
            error!(?e, "Failed to dispatch SQL error event");
        }
    }

    /// Input proof specific SQL error handler - knows about proof context
    pub async fn input_proof_sql_error<D, T: Display>(
        dispatcher: &Arc<D>,
        event: RelayerEvent,
        operation: &str,
        sql_error: T,
    ) where
        D: EventDispatcher<RelayerEvent>,
    {
        error!(
            job_id = %event.job_id,
            sql_operation = %operation,
            sql_error = %sql_error,
            handler_type = "input_proof",
            "SQL operation failed"
        );

        let error_event =
            event.derive_next_event(crate::core::event::RelayerEventData::InputProof(
                crate::core::event::InputProofEventData::Failed {
                    error: EventProcessingError::SqlOperationFailed {
                        operation: operation.to_string(),
                        reason: sql_error.to_string(),
                    },
                },
            ));

        if let Err(e) = dispatcher.dispatch_event(error_event).await {
            error!(?e, "Failed to dispatch SQL error event");
        }
    }
}

/// Converts U256 to i64 for database storage, returns error if value exceeds i64::MAX.
pub fn u256_to_i64(v: U256) -> Result<i64, &'static str> {
    if v > U256::from(i64::MAX) {
        return Err("U256 value too large for i64");
    }
    Ok(v.as_limbs()[0] as i64)
}

/// Converts U256 to i32 for database storage, returns error if value exceeds i32::MAX.
pub fn u256_to_i32(v: U256) -> Result<i32, &'static str> {
    if v > U256::from(i32::MAX) {
        return Err("U256 value too large for i32");
    }
    Ok(v.as_limbs()[0] as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::U256;

    #[test]
    fn test_u256_to_i64_small_values() {
        let small = U256::from(123);
        assert_eq!(u256_to_i64(small).unwrap(), 123i64);
    }

    #[test]
    fn test_u256_to_i64_max_i64() {
        let max_i64 = U256::from(i64::MAX);
        assert_eq!(u256_to_i64(max_i64).unwrap(), i64::MAX);
    }

    #[test]
    fn test_u256_to_i64_overflow() {
        let too_large = U256::from(i64::MAX) + U256::from(1);
        assert!(u256_to_i64(too_large).is_err());
    }

    #[test]
    fn test_u256_to_i64_zero() {
        let zero = U256::ZERO;
        assert_eq!(u256_to_i64(zero).unwrap(), 0i64);
    }

    #[test]
    fn test_u256_to_i32_small_values() {
        let small = U256::from(123);
        assert_eq!(u256_to_i32(small).unwrap(), 123i32);
    }

    #[test]
    fn test_u256_to_i32_max_i32() {
        let max_i32 = U256::from(i32::MAX);
        assert_eq!(u256_to_i32(max_i32).unwrap(), i32::MAX);
    }

    #[test]
    fn test_u256_to_i32_overflow() {
        let too_large = U256::from(i32::MAX) + U256::from(1);
        assert!(u256_to_i32(too_large).is_err());
    }

    #[test]
    fn test_u256_to_i32_zero() {
        let zero = U256::ZERO;
        assert_eq!(u256_to_i32(zero).unwrap(), 0i32);
    }
}
