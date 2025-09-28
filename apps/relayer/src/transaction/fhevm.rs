use crate::blockchain::ethereum::bindings::{
    CiphertextCommits::{self, CiphertextCommitsErrors},
    Decryption::{self, DecryptionErrors},
    InputVerification::{self, InputVerificationErrors},
    MultichainAcl::{self, MultichainAclErrors},
};
use alloy::contract::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum FhevmError {
    DecryptionError(DecryptionErrors),
    InputError(InputVerificationErrors),
    AclError(MultichainAclErrors),
    CiphertextError(CiphertextCommitsErrors),
    GenericError,
}

pub fn retryable_error(err: &FhevmError) -> bool {
    match err {
        // Errors that happen when the Ciphertext wasn't created and the ACL propagated yet
        FhevmError::AclError(value) => match value {
            MultichainAclErrors::AccountNotAllowedToUseCiphertext(_)
            | MultichainAclErrors::PublicDecryptNotAllowed(_) => true,
            _ => true,
        },
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
    if let Some(value) = err.as_decoded_error::<Decryption::DecryptionNotDone>() {
        return FhevmError::DecryptionError(DecryptionErrors::DecryptionNotDone(value));
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
    if let Some(value) = err.as_decoded_error::<Decryption::NotPauser>() {
        return FhevmError::DecryptionError(DecryptionErrors::NotPauser(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::NotPauserOrGatewayConfig>() {
        return FhevmError::DecryptionError(DecryptionErrors::NotPauserOrGatewayConfig(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::OwnableInvalidOwner>() {
        return FhevmError::DecryptionError(DecryptionErrors::OwnableInvalidOwner(value));
    }
    if let Some(value) = err.as_decoded_error::<Decryption::OwnableUnauthorizedAccount>() {
        return FhevmError::DecryptionError(DecryptionErrors::OwnableUnauthorizedAccount(value));
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
    if let Some(value) = err.as_decoded_error::<InputVerification::NotPauser>() {
        return FhevmError::InputError(InputVerificationErrors::NotPauser(value));
    }
    if let Some(value) = err.as_decoded_error::<InputVerification::NotPauserOrGatewayConfig>() {
        return FhevmError::InputError(InputVerificationErrors::NotPauserOrGatewayConfig(value));
    }
    if let Some(value) = err.as_decoded_error::<InputVerification::OwnableInvalidOwner>() {
        return FhevmError::InputError(InputVerificationErrors::OwnableInvalidOwner(value));
    }
    if let Some(value) = err.as_decoded_error::<InputVerification::OwnableUnauthorizedAccount>() {
        return FhevmError::InputError(InputVerificationErrors::OwnableUnauthorizedAccount(value));
    }
    if let Some(value) = err.as_decoded_error::<InputVerification::ProofNotRejected>() {
        return FhevmError::InputError(InputVerificationErrors::ProofNotRejected(value));
    }
    if let Some(value) = err.as_decoded_error::<InputVerification::ProofNotVerified>() {
        return FhevmError::InputError(InputVerificationErrors::ProofNotVerified(value));
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

    // MultichainAcl Errors
    if let Some(value) = err.as_decoded_error::<MultichainAcl::AccountNotAllowedToUseCiphertext>() {
        return FhevmError::AclError(MultichainAclErrors::AccountNotAllowedToUseCiphertext(value));
    }
    if let Some(value) = err.as_decoded_error::<MultichainAcl::AccountNotDelegated>() {
        return FhevmError::AclError(MultichainAclErrors::AccountNotDelegated(value));
    }
    if let Some(value) = err.as_decoded_error::<MultichainAcl::AddressEmptyCode>() {
        return FhevmError::AclError(MultichainAclErrors::AddressEmptyCode(value));
    }
    if let Some(value) = err.as_decoded_error::<MultichainAcl::ContractsMaxLengthExceeded>() {
        return FhevmError::AclError(MultichainAclErrors::ContractsMaxLengthExceeded(value));
    }
    if let Some(value) = err.as_decoded_error::<MultichainAcl::CoprocessorAlreadyAllowedAccount>() {
        return FhevmError::AclError(MultichainAclErrors::CoprocessorAlreadyAllowedAccount(value));
    }
    if let Some(value) =
        err.as_decoded_error::<MultichainAcl::CoprocessorAlreadyAllowedPublicDecrypt>()
    {
        return FhevmError::AclError(MultichainAclErrors::CoprocessorAlreadyAllowedPublicDecrypt(
            value,
        ));
    }
    if let Some(value) = err.as_decoded_error::<MultichainAcl::CoprocessorAlreadyDelegated>() {
        return FhevmError::AclError(MultichainAclErrors::CoprocessorAlreadyDelegated(value));
    }
    if let Some(value) = err.as_decoded_error::<MultichainAcl::ERC1967InvalidImplementation>() {
        return FhevmError::AclError(MultichainAclErrors::ERC1967InvalidImplementation(value));
    }
    if let Some(value) = err.as_decoded_error::<MultichainAcl::ERC1967NonPayable>() {
        return FhevmError::AclError(MultichainAclErrors::ERC1967NonPayable(value));
    }
    if let Some(value) = err.as_decoded_error::<MultichainAcl::EmptyContractAddresses>() {
        return FhevmError::AclError(MultichainAclErrors::EmptyContractAddresses(value));
    }
    if let Some(value) = err.as_decoded_error::<MultichainAcl::EnforcedPause>() {
        return FhevmError::AclError(MultichainAclErrors::EnforcedPause(value));
    }
    if let Some(value) = err.as_decoded_error::<MultichainAcl::ExpectedPause>() {
        return FhevmError::AclError(MultichainAclErrors::ExpectedPause(value));
    }
    if let Some(value) = err.as_decoded_error::<MultichainAcl::FailedCall>() {
        return FhevmError::AclError(MultichainAclErrors::FailedCall(value));
    }
    if let Some(value) = err.as_decoded_error::<MultichainAcl::InvalidInitialization>() {
        return FhevmError::AclError(MultichainAclErrors::InvalidInitialization(value));
    }
    if let Some(value) = err.as_decoded_error::<MultichainAcl::NotGatewayOwner>() {
        return FhevmError::AclError(MultichainAclErrors::NotGatewayOwner(value));
    }
    if let Some(value) = err.as_decoded_error::<MultichainAcl::NotInitializing>() {
        return FhevmError::AclError(MultichainAclErrors::NotInitializing(value));
    }
    if let Some(value) = err.as_decoded_error::<MultichainAcl::NotInitializingFromEmptyProxy>() {
        return FhevmError::AclError(MultichainAclErrors::NotInitializingFromEmptyProxy(value));
    }
    if let Some(value) = err.as_decoded_error::<MultichainAcl::NotOwnerOrGatewayConfig>() {
        return FhevmError::AclError(MultichainAclErrors::NotOwnerOrGatewayConfig(value));
    }
    if let Some(value) = err.as_decoded_error::<MultichainAcl::NotPauser>() {
        return FhevmError::AclError(MultichainAclErrors::NotPauser(value));
    }
    if let Some(value) = err.as_decoded_error::<MultichainAcl::NotPauserOrGatewayConfig>() {
        return FhevmError::AclError(MultichainAclErrors::NotPauserOrGatewayConfig(value));
    }
    if let Some(value) = err.as_decoded_error::<MultichainAcl::OwnableInvalidOwner>() {
        return FhevmError::AclError(MultichainAclErrors::OwnableInvalidOwner(value));
    }
    if let Some(value) = err.as_decoded_error::<MultichainAcl::OwnableUnauthorizedAccount>() {
        return FhevmError::AclError(MultichainAclErrors::OwnableUnauthorizedAccount(value));
    }
    if let Some(value) = err.as_decoded_error::<MultichainAcl::PublicDecryptNotAllowed>() {
        return FhevmError::AclError(MultichainAclErrors::PublicDecryptNotAllowed(value));
    }
    if let Some(value) = err.as_decoded_error::<MultichainAcl::UUPSUnauthorizedCallContext>() {
        return FhevmError::AclError(MultichainAclErrors::UUPSUnauthorizedCallContext(value));
    }
    if let Some(value) = err.as_decoded_error::<MultichainAcl::UUPSUnsupportedProxiableUUID>() {
        return FhevmError::AclError(MultichainAclErrors::UUPSUnsupportedProxiableUUID(value));
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
    if let Some(value) = err.as_decoded_error::<CiphertextCommits::EnforcedPause>() {
        return FhevmError::CiphertextError(CiphertextCommitsErrors::EnforcedPause(value));
    }
    if let Some(value) = err.as_decoded_error::<CiphertextCommits::ExpectedPause>() {
        return FhevmError::CiphertextError(CiphertextCommitsErrors::ExpectedPause(value));
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
    if let Some(value) = err.as_decoded_error::<CiphertextCommits::NotOwnerOrGatewayConfig>() {
        return FhevmError::CiphertextError(CiphertextCommitsErrors::NotOwnerOrGatewayConfig(
            value,
        ));
    }
    if let Some(value) = err.as_decoded_error::<CiphertextCommits::NotPauser>() {
        return FhevmError::CiphertextError(CiphertextCommitsErrors::NotPauser(value));
    }
    if let Some(value) = err.as_decoded_error::<CiphertextCommits::NotPauserOrGatewayConfig>() {
        return FhevmError::CiphertextError(CiphertextCommitsErrors::NotPauserOrGatewayConfig(
            value,
        ));
    }
    if let Some(value) = err.as_decoded_error::<CiphertextCommits::OwnableInvalidOwner>() {
        return FhevmError::CiphertextError(CiphertextCommitsErrors::OwnableInvalidOwner(value));
    }
    if let Some(value) = err.as_decoded_error::<CiphertextCommits::OwnableUnauthorizedAccount>() {
        return FhevmError::CiphertextError(CiphertextCommitsErrors::OwnableUnauthorizedAccount(
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
            FhevmError::DecryptionError(_) => {
                // Since DecryptionErrors doesn't implement Clone, we fallback to GenericError
                FhevmError::GenericError
            }
            FhevmError::InputError(_) => FhevmError::GenericError,
            FhevmError::AclError(_) => FhevmError::GenericError,
            FhevmError::CiphertextError(_) => FhevmError::GenericError,
            FhevmError::GenericError => FhevmError::GenericError,
        }
    }
}
