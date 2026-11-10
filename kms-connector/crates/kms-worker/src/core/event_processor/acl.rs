use crate::core::event_processor::{RequestCheckError, RequestCheckKind};
use alloy::{
    primitives::{Address, B256, Bytes, U256},
    sol_types::SolCall,
};
use anyhow::anyhow;
use fhevm_host_bindings::acl::ACL::{
    decryptionSignatureInvalidatedBeforeCall, isAllowedCall, isAllowedForDecryptionCall,
    isHandleDelegatedForUserDecryptionCall,
};
use kms_connector_api::ErrorCode;

pub enum AclCheck {
    /// Passes if `start_timestamp` is not before the user's signature invalidation timestamp.
    SignatureNotInvalidated {
        call: decryptionSignatureInvalidatedBeforeCall,
        start_timestamp: U256,
    },
    AllowedForDecryption(isAllowedForDecryptionCall),
    Allowed(isAllowedCall),
    Delegated(isHandleDelegatedForUserDecryptionCall),
    /// Passes if any of `accounts` is allowed on `handle`.
    AnyAllowed {
        handle: B256,
        accounts: Vec<Address>,
    },
}

impl AclCheck {
    /// Signature invalidation check. Rejects if `startTimestamp < invalidationTs`, meaning the
    /// user has invalidated all signatures issued before `invalidationTs`.
    pub fn signature_not_invalidated(user_address: Address, start_timestamp: U256) -> Self {
        Self::SignatureNotInvalidated {
            call: decryptionSignatureInvalidatedBeforeCall {
                account: user_address,
            },
            start_timestamp,
        }
    }

    /// Verifies the multicall `results` of all `checks` in order.
    ///
    /// Returns the error of the first failing check.
    pub fn verify_all(checks: &[AclCheck], results: &[Bytes]) -> Result<(), RequestCheckError> {
        let calls_len: usize = checks.iter().map(AclCheck::calls_len).sum();

        // Guards the slicing below. A mismatch can only come from a faulty RPC response: retry.
        if results.len() != calls_len {
            return Err(RequestCheckError::network(anyhow!(
                "Got {} ACL results for {calls_len} calls",
                results.len()
            )));
        }

        let mut results = results;
        for check in checks {
            let (check_results, rest) = results.split_at(check.calls_len());
            check.verify(check_results)?;
            results = rest;
        }
        Ok(())
    }

    /// Number of ACL calls made by this check.
    fn calls_len(&self) -> usize {
        match self {
            Self::AnyAllowed { accounts, .. } => accounts.len(),
            _ => 1,
        }
    }

    /// ABI-encodes the ACL calls of this check.
    pub fn abi_encode(&self) -> Vec<Bytes> {
        match self {
            Self::SignatureNotInvalidated { call, .. } => vec![call.abi_encode().into()],
            Self::AllowedForDecryption(call) => vec![call.abi_encode().into()],
            Self::Allowed(call) => vec![call.abi_encode().into()],
            Self::Delegated(call) => vec![call.abi_encode().into()],
            Self::AnyAllowed { handle, accounts } => accounts
                .iter()
                .map(|&account| {
                    isAllowedCall {
                        handle: *handle,
                        account,
                    }
                    .abi_encode()
                    .into()
                })
                .collect(),
        }
    }

    /// Interprets `results`, the results of this check's calls.
    fn verify(&self, results: &[Bytes]) -> Result<(), RequestCheckError> {
        let denied = |e| {
            Err(RequestCheckError::recoverable(
                RequestCheckKind::Acl,
                ErrorCode::AclDenied,
                e,
            ))
        };
        match self {
            Self::SignatureNotInvalidated {
                call,
                start_timestamp,
            } => {
                let invalidation_ts =
                    decode::<decryptionSignatureInvalidatedBeforeCall>(&results[0])?;
                if *start_timestamp < invalidation_ts {
                    return Err(RequestCheckError::irrecoverable(
                        RequestCheckKind::Signature,
                        ErrorCode::UserSignatureRejected,
                        anyhow!(
                            "Signature invalidated: startTimestamp {start_timestamp} < \
                            invalidatedBefore {invalidation_ts} for userAddress {}",
                            call.account
                        ),
                    ));
                }
            }
            Self::AllowedForDecryption(call) => {
                if !decode::<isAllowedForDecryptionCall>(&results[0])? {
                    return denied(anyhow!("Decryption is not allowed for {}", call.handle));
                }
            }
            Self::Allowed(call) => {
                if !decode::<isAllowedCall>(&results[0])? {
                    return denied(anyhow!(
                        "{} is not allowed to decrypt {}",
                        call.account,
                        call.handle
                    ));
                }
            }
            Self::Delegated(call) => {
                if !decode::<isHandleDelegatedForUserDecryptionCall>(&results[0])? {
                    return denied(anyhow!(
                        "{} is not a delegate of {} for contract {} and handle {}",
                        call.delegate,
                        call.delegator,
                        call.contractAddress,
                        call.handle
                    ));
                }
            }
            Self::AnyAllowed { handle, accounts } => {
                let mut any_allowed = false;
                for result in results {
                    any_allowed |= decode::<isAllowedCall>(result)?;
                }
                if !any_allowed {
                    return denied(anyhow!(
                        "No contract in allowedContracts ({accounts:?}) is allowed to decrypt \
                        handle {handle}"
                    ));
                }
            }
        }
        Ok(())
    }
}

/// ABI-decodes `result` as the return data of a `C` call.
///
/// A malformed result is treated as a faulty RPC response, so the request is retried.
fn decode<C: SolCall>(result: &Bytes) -> Result<C::Return, RequestCheckError> {
    C::abi_decode_returns(result).map_err(RequestCheckError::network)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::event_processor::ProcessingErrorKind;
    use alloy::sol_types::SolValue;
    use connector_utils::tests::rand::{rand_address, rand_handle};

    fn allowed() -> AclCheck {
        AclCheck::Allowed(isAllowedCall {
            handle: rand_handle(),
            account: rand_address(),
        })
    }

    fn any_allowed(accounts_len: usize) -> AclCheck {
        AclCheck::AnyAllowed {
            handle: rand_handle(),
            accounts: (0..accounts_len).map(|_| rand_address()).collect(),
        }
    }

    fn results(values: Vec<Vec<u8>>) -> Vec<Bytes> {
        values.into_iter().map(Bytes::from).collect()
    }

    #[test]
    fn verify_all_gives_each_check_its_own_results() {
        // Only the last check fails, with an error code no other check returns. Getting that code
        // shows each check read its own results, including `AnyAllowed`, which reads two.
        let checks = [
            allowed(),
            any_allowed(2),
            AclCheck::signature_not_invalidated(rand_address(), U256::ZERO),
        ];
        let results = results(vec![
            true.abi_encode(),      // Allowed
            false.abi_encode(),     // AnyAllowed, 1st account
            true.abi_encode(),      // AnyAllowed, 2nd account
            U256::ONE.abi_encode(), // invalidation timestamp > start timestamp (0): rejected
        ]);

        let err = AclCheck::verify_all(&checks, &results)
            .unwrap_err()
            .record();
        assert_eq!(err.kind, ProcessingErrorKind::Irrecoverable);
        assert_eq!(err.code, ErrorCode::UserSignatureRejected);
    }

    #[test]
    fn verify_all_denies_any_allowed_if_no_account_is_allowed() {
        let checks = [any_allowed(2)];
        let results = results(vec![false.abi_encode(), false.abi_encode()]);

        let err = AclCheck::verify_all(&checks, &results)
            .unwrap_err()
            .record();
        assert_eq!(err.kind, ProcessingErrorKind::Recoverable);
        assert_eq!(err.code, ErrorCode::AclDenied);
    }

    #[test]
    fn verify_all_rejects_wrong_results_count() {
        let checks = [allowed(), any_allowed(2)];
        let results = results(vec![true.abi_encode(), true.abi_encode()]);

        let err = AclCheck::verify_all(&checks, &results)
            .unwrap_err()
            .record();
        assert_eq!(err.kind, ProcessingErrorKind::Recoverable);
        assert_eq!(err.code, ErrorCode::UpstreamTransient);
    }
}
