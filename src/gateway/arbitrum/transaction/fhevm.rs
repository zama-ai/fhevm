use crate::gateway::arbitrum::bindings::{
    CiphertextCommits::{self, CiphertextCommitsErrors},
    Decryption::{self, DecryptionErrors},
    InputVerification::{self, InputVerificationErrors},
};
use alloy::contract::Error;
use fhevm_gateway_bindings::gateway_config::GatewayConfig::{self, GatewayConfigErrors};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum FhevmError {
    GatewayConfigError(GatewayConfigErrors),
    DecryptionError(DecryptionErrors),
    InputError(InputVerificationErrors),
    CiphertextError(CiphertextCommitsErrors),
    GenericError,
}

pub fn retryable_error(err: &FhevmError) -> bool {
    match err {
        // Errors that happen when the Ciphertext wasn't created and the ACL propagated yet
        FhevmError::DecryptionError(_) => true,
        FhevmError::CiphertextError(value) => match value {
            CiphertextCommitsErrors::CiphertextMaterialNotFound(_) => true,
            _ => true,
        },
        _ => false,
    }
}

pub fn parse_fhevm_error(err: &Error) -> FhevmError {
    // Decryption Errors
    if let Some(value) = err.as_decoded_error::<Decryption::AddressEmptyCode>() {
        return FhevmError::DecryptionError(DecryptionErrors::AddressEmptyCode(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::ContractAddressesMaxLengthExceeded>() {
        return FhevmError::DecryptionError(DecryptionErrors::ContractAddressesMaxLengthExceeded(
            value,
        ));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::ContractNotInContractAddresses>() {
        return FhevmError::DecryptionError(DecryptionErrors::ContractNotInContractAddresses(
            value,
        ));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::DecryptionNotRequested>() {
        return FhevmError::DecryptionError(DecryptionErrors::DecryptionNotRequested(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::DelegatorAddressInContractAddresses>() {
        return FhevmError::DecryptionError(DecryptionErrors::DelegatorAddressInContractAddresses(
            value,
        ));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::DifferentKeyIdsNotAllowed>() {
        return FhevmError::DecryptionError(DecryptionErrors::DifferentKeyIdsNotAllowed(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::ECDSAInvalidSignature>() {
        return FhevmError::DecryptionError(DecryptionErrors::ECDSAInvalidSignature(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::ECDSAInvalidSignatureLength>() {
        return FhevmError::DecryptionError(DecryptionErrors::ECDSAInvalidSignatureLength(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::ECDSAInvalidSignatureS>() {
        return FhevmError::DecryptionError(DecryptionErrors::ECDSAInvalidSignatureS(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::ERC1967InvalidImplementation>() {
        return FhevmError::DecryptionError(DecryptionErrors::ERC1967InvalidImplementation(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::ERC1967NonPayable>() {
        return FhevmError::DecryptionError(DecryptionErrors::ERC1967NonPayable(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::EmptyContractAddresses>() {
        return FhevmError::DecryptionError(DecryptionErrors::EmptyContractAddresses(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::EmptyCtHandleContractPairs>() {
        return FhevmError::DecryptionError(DecryptionErrors::EmptyCtHandleContractPairs(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::EmptyCtHandles>() {
        return FhevmError::DecryptionError(DecryptionErrors::EmptyCtHandles(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::EnforcedPause>() {
        return FhevmError::DecryptionError(DecryptionErrors::EnforcedPause(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::ExpectedPause>() {
        return FhevmError::DecryptionError(DecryptionErrors::ExpectedPause(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::FailedCall>() {
        return FhevmError::DecryptionError(DecryptionErrors::FailedCall(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::InvalidFHEType>() {
        return FhevmError::DecryptionError(DecryptionErrors::InvalidFHEType(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::InvalidInitialization>() {
        return FhevmError::DecryptionError(DecryptionErrors::InvalidInitialization(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::InvalidNullDurationDays>() {
        return FhevmError::DecryptionError(DecryptionErrors::InvalidNullDurationDays(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::InvalidUserSignature>() {
        return FhevmError::DecryptionError(DecryptionErrors::InvalidUserSignature(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::KmsNodeAlreadySigned>() {
        return FhevmError::DecryptionError(DecryptionErrors::KmsNodeAlreadySigned(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::MaxDecryptionRequestBitSizeExceeded>() {
        return FhevmError::DecryptionError(DecryptionErrors::MaxDecryptionRequestBitSizeExceeded(
            value,
        ));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::MaxDurationDaysExceeded>() {
        return FhevmError::DecryptionError(DecryptionErrors::MaxDurationDaysExceeded(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::NotGatewayOwner>() {
        return FhevmError::DecryptionError(DecryptionErrors::NotGatewayOwner(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::NotInitializing>() {
        return FhevmError::DecryptionError(DecryptionErrors::NotInitializing(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::NotInitializingFromEmptyProxy>() {
        return FhevmError::DecryptionError(DecryptionErrors::NotInitializingFromEmptyProxy(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::NotOwnerOrGatewayConfig>() {
        return FhevmError::DecryptionError(DecryptionErrors::NotOwnerOrGatewayConfig(value));
    }
    if let Some(value) = err.as_decoded_error::<GatewayConfig::NotPauser>() {
        return FhevmError::GatewayConfigError(GatewayConfigErrors::NotPauser(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::NotPauserOrGatewayConfig>() {
        return FhevmError::DecryptionError(DecryptionErrors::NotPauserOrGatewayConfig(value));
    }
    if let Some(value) = err.as_decoded_error::<GatewayConfig::OwnableInvalidOwner>() {
        return FhevmError::GatewayConfigError(GatewayConfigErrors::OwnableInvalidOwner(value));
    }
    if let Some(value) = err.as_decoded_error::<GatewayConfig::OwnableUnauthorizedAccount>() {
        return FhevmError::GatewayConfigError(GatewayConfigErrors::OwnableUnauthorizedAccount(
            value,
        ));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::StartTimestampInFuture>() {
        return FhevmError::DecryptionError(DecryptionErrors::StartTimestampInFuture(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::UUPSUnauthorizedCallContext>() {
        return FhevmError::DecryptionError(DecryptionErrors::UUPSUnauthorizedCallContext(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::UUPSUnsupportedProxiableUUID>() {
        return FhevmError::DecryptionError(DecryptionErrors::UUPSUnsupportedProxiableUUID(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::UnsupportedFHEType>() {
        return FhevmError::DecryptionError(DecryptionErrors::UnsupportedFHEType(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::UserAddressInContractAddresses>() {
        return FhevmError::DecryptionError(DecryptionErrors::UserAddressInContractAddresses(
            value,
        ));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::UserDecryptionRequestExpired>() {
        return FhevmError::DecryptionError(DecryptionErrors::UserDecryptionRequestExpired(value));
    }

    // InputVerification Errors
    if let Some(value) = err.as_decoded_error::<InputVerification::AddressEmptyCode>() {
        return FhevmError::InputError(InputVerificationErrors::AddressEmptyCode(value));
    }
    if let Some(value) = err.as_decoded_error::<InputVerification::CoprocessorAlreadyRejected>() {
        return FhevmError::InputError(InputVerificationErrors::CoprocessorAlreadyRejected(value));
    }
    if let Some(value) = err.as_decoded_error::<InputVerification::CoprocessorAlreadyVerified>() {
        return FhevmError::InputError(InputVerificationErrors::CoprocessorAlreadyVerified(value));
    }
    if let Some(value) = err.as_decoded_error::<InputVerification::ECDSAInvalidSignature>() {
        return FhevmError::InputError(InputVerificationErrors::ECDSAInvalidSignature(value));
    }
    if let Some(value) = err.as_decoded_error::<InputVerification::ECDSAInvalidSignatureLength>() {
        return FhevmError::InputError(InputVerificationErrors::ECDSAInvalidSignatureLength(value));
    }
    if let Some(value) = err.as_decoded_error::<InputVerification::ECDSAInvalidSignatureS>() {
        return FhevmError::InputError(InputVerificationErrors::ECDSAInvalidSignatureS(value));
    }
    if let Some(value) = err.as_decoded_error::<InputVerification::ERC1967InvalidImplementation>() {
        return FhevmError::InputError(InputVerificationErrors::ERC1967InvalidImplementation(
            value,
        ));
    }
    if let Some(value) = err.as_decoded_error::<InputVerification::ERC1967NonPayable>() {
        return FhevmError::InputError(InputVerificationErrors::ERC1967NonPayable(value));
    }
    if let Some(value) = err.as_decoded_error::<InputVerification::EnforcedPause>() {
        return FhevmError::InputError(InputVerificationErrors::EnforcedPause(value));
    }
    if let Some(value) = err.as_decoded_error::<InputVerification::ExpectedPause>() {
        return FhevmError::InputError(InputVerificationErrors::ExpectedPause(value));
    }
    if let Some(value) = err.as_decoded_error::<InputVerification::FailedCall>() {
        return FhevmError::InputError(InputVerificationErrors::FailedCall(value));
    }
    if let Some(value) = err.as_decoded_error::<InputVerification::InvalidInitialization>() {
        return FhevmError::InputError(InputVerificationErrors::InvalidInitialization(value));
    }
    if let Some(value) = err.as_decoded_error::<InputVerification::NotGatewayOwner>() {
        return FhevmError::InputError(InputVerificationErrors::NotGatewayOwner(value));
    }
    if let Some(value) = err.as_decoded_error::<InputVerification::NotInitializing>() {
        return FhevmError::InputError(InputVerificationErrors::NotInitializing(value));
    }
    if let Some(value) = err.as_decoded_error::<InputVerification::NotInitializingFromEmptyProxy>()
    {
        return FhevmError::InputError(InputVerificationErrors::NotInitializingFromEmptyProxy(
            value,
        ));
    }
    if let Some(value) = err.as_decoded_error::<InputVerification::NotOwnerOrGatewayConfig>() {
        return FhevmError::InputError(InputVerificationErrors::NotOwnerOrGatewayConfig(value));
    }
    if let Some(value) = err.as_decoded_error::<GatewayConfig::NotPauser>() {
        return FhevmError::GatewayConfigError(GatewayConfigErrors::NotPauser(value));
    }
    if let Some(value) = err.as_decoded_error::<InputVerification::NotPauserOrGatewayConfig>() {
        return FhevmError::InputError(InputVerificationErrors::NotPauserOrGatewayConfig(value));
    }
    if let Some(value) = err.as_decoded_error::<GatewayConfig::OwnableInvalidOwner>() {
        return FhevmError::GatewayConfigError(GatewayConfigErrors::OwnableInvalidOwner(value));
    }
    if let Some(value) = err.as_decoded_error::<GatewayConfig::OwnableUnauthorizedAccount>() {
        return FhevmError::GatewayConfigError(GatewayConfigErrors::OwnableUnauthorizedAccount(
            value,
        ));
    }
    if let Some(value) = err.as_decoded_error::<InputVerification::UUPSUnauthorizedCallContext>() {
        return FhevmError::InputError(InputVerificationErrors::UUPSUnauthorizedCallContext(value));
    }
    if let Some(value) = err.as_decoded_error::<InputVerification::UUPSUnsupportedProxiableUUID>() {
        return FhevmError::InputError(InputVerificationErrors::UUPSUnsupportedProxiableUUID(
            value,
        ));
    }
    if let Some(value) = err.as_decoded_error::<InputVerification::VerifyProofNotRequested>() {
        return FhevmError::InputError(InputVerificationErrors::VerifyProofNotRequested(value));
    }

    // CiphertextCommits Errors
    if let Some(value) = err.as_decoded_error::<CiphertextCommits::AddressEmptyCode>() {
        return FhevmError::CiphertextError(CiphertextCommitsErrors::AddressEmptyCode(value));
    }
    if let Some(value) = err.as_decoded_error::<CiphertextCommits::CiphertextMaterialNotFound>() {
        return FhevmError::CiphertextError(CiphertextCommitsErrors::CiphertextMaterialNotFound(
            value,
        ));
    }
    if let Some(value) = err.as_decoded_error::<CiphertextCommits::CoprocessorAlreadyAdded>() {
        return FhevmError::CiphertextError(CiphertextCommitsErrors::CoprocessorAlreadyAdded(
            value,
        ));
    }
    if let Some(value) = err.as_decoded_error::<CiphertextCommits::ERC1967InvalidImplementation>() {
        return FhevmError::CiphertextError(CiphertextCommitsErrors::ERC1967InvalidImplementation(
            value,
        ));
    }
    if let Some(value) = err.as_decoded_error::<CiphertextCommits::ERC1967NonPayable>() {
        return FhevmError::CiphertextError(CiphertextCommitsErrors::ERC1967NonPayable(value));
    }
    if let Some(value) = err.as_decoded_error::<InputVerification::EnforcedPause>() {
        return FhevmError::InputError(InputVerificationErrors::EnforcedPause(value));
    }
    if let Some(value) = err.as_decoded_error::<InputVerification::ExpectedPause>() {
        return FhevmError::InputError(InputVerificationErrors::ExpectedPause(value));
    }
    if let Some(value) = err.as_decoded_error::<CiphertextCommits::FailedCall>() {
        return FhevmError::CiphertextError(CiphertextCommitsErrors::FailedCall(value));
    }
    if let Some(value) = err.as_decoded_error::<CiphertextCommits::InvalidInitialization>() {
        return FhevmError::CiphertextError(CiphertextCommitsErrors::InvalidInitialization(value));
    }
    if let Some(value) = err.as_decoded_error::<CiphertextCommits::NotGatewayOwner>() {
        return FhevmError::CiphertextError(CiphertextCommitsErrors::NotGatewayOwner(value));
    }
    if let Some(value) = err.as_decoded_error::<CiphertextCommits::NotInitializing>() {
        return FhevmError::CiphertextError(CiphertextCommitsErrors::NotInitializing(value));
    }
    if let Some(value) = err.as_decoded_error::<CiphertextCommits::NotInitializingFromEmptyProxy>()
    {
        return FhevmError::CiphertextError(
            CiphertextCommitsErrors::NotInitializingFromEmptyProxy(value),
        );
    }
    if let Some(value) = err.as_decoded_error::<InputVerification::NotOwnerOrGatewayConfig>() {
        return FhevmError::InputError(InputVerificationErrors::NotOwnerOrGatewayConfig(value));
    }
    if let Some(value) = err.as_decoded_error::<GatewayConfig::NotPauser>() {
        return FhevmError::GatewayConfigError(GatewayConfigErrors::NotPauser(value));
    }
    if let Some(value) = err.as_decoded_error::<InputVerification::NotPauserOrGatewayConfig>() {
        return FhevmError::InputError(InputVerificationErrors::NotPauserOrGatewayConfig(value));
    }
    if let Some(value) = err.as_decoded_error::<GatewayConfig::OwnableInvalidOwner>() {
        return FhevmError::GatewayConfigError(GatewayConfigErrors::OwnableInvalidOwner(value));
    }
    if let Some(value) = err.as_decoded_error::<GatewayConfig::OwnableUnauthorizedAccount>() {
        return FhevmError::GatewayConfigError(GatewayConfigErrors::OwnableUnauthorizedAccount(
            value,
        ));
    }
    if let Some(value) = err.as_decoded_error::<CiphertextCommits::UUPSUnauthorizedCallContext>() {
        return FhevmError::CiphertextError(CiphertextCommitsErrors::UUPSUnauthorizedCallContext(
            value,
        ));
    }
    if let Some(value) = err.as_decoded_error::<CiphertextCommits::UUPSUnsupportedProxiableUUID>() {
        return FhevmError::CiphertextError(CiphertextCommitsErrors::UUPSUnsupportedProxiableUUID(
            value,
        ));
    }
    FhevmError::GenericError
}

impl Clone for FhevmError {
    fn clone(&self) -> Self {
        match self {
            FhevmError::GatewayConfigError(_) => FhevmError::GenericError,
            FhevmError::DecryptionError(_) => {
                // Since DecryptionErrors doesn't implement Clone, we fallback to GenericError
                FhevmError::GenericError
            }
            FhevmError::InputError(_) => FhevmError::GenericError,
            FhevmError::CiphertextError(_) => FhevmError::GenericError,
            FhevmError::GenericError => FhevmError::GenericError,
        }
    }
}
