///Module containing a contract's types and functions.
/**

```solidity
library IDecryption {
    struct ContractsInfo { uint256 chainId; address[] addresses; }
    struct RequestValidity { uint256 startTimestamp; uint256 durationDays; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IDecryption {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct ContractsInfo { uint256 chainId; address[] addresses; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ContractsInfo {
        #[allow(missing_docs)]
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub addresses: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ContractsInfo> for UnderlyingRustTuple<'_> {
            fn from(value: ContractsInfo) -> Self {
                (value.chainId, value.addresses)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ContractsInfo {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    chainId: tuple.0,
                    addresses: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ContractsInfo {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ContractsInfo {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.chainId),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.addresses),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for ContractsInfo {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for ContractsInfo {
            const NAME: &'static str = "ContractsInfo";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ContractsInfo(uint256 chainId,address[] addresses)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.chainId)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.addresses)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ContractsInfo {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.chainId,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.addresses,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.chainId,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.addresses,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct RequestValidity { uint256 startTimestamp; uint256 durationDays; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RequestValidity {
        #[allow(missing_docs)]
        pub startTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub durationDays: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<RequestValidity> for UnderlyingRustTuple<'_> {
            fn from(value: RequestValidity) -> Self {
                (value.startTimestamp, value.durationDays)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RequestValidity {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    startTimestamp: tuple.0,
                    durationDays: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for RequestValidity {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for RequestValidity {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.startTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.durationDays),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for RequestValidity {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for RequestValidity {
            const NAME: &'static str = "RequestValidity";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RequestValidity(uint256 startTimestamp,uint256 durationDays)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.startTimestamp,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.durationDays)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RequestValidity {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.startTimestamp,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.durationDays,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.startTimestamp,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.durationDays,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IDecryption`](self) contract instance.

See the [wrapper's documentation](`IDecryptionInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IDecryptionInstance<P, N> {
        IDecryptionInstance::<P, N>::new(address, provider)
    }
    /**A [`IDecryption`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IDecryption`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IDecryptionInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IDecryptionInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IDecryptionInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IDecryptionInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`IDecryption`](self) contract instance.

See the [wrapper's documentation](`IDecryptionInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> IDecryptionInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IDecryptionInstance<P, N> {
            IDecryptionInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IDecryptionInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IDecryptionInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library IDecryption {
    struct ContractsInfo {
        uint256 chainId;
        address[] addresses;
    }
    struct RequestValidity {
        uint256 startTimestamp;
        uint256 durationDays;
    }
}

interface Decryption {
    type FheType is uint8;
    struct CtHandleContractPair {
        bytes32 ctHandle;
        address contractAddress;
    }
    struct DelegationAccounts {
        address delegatorAddress;
        address delegatedAddress;
    }
    struct SnsCiphertextMaterial {
        bytes32 ctHandle;
        uint256 keyId;
        bytes32 snsCiphertextDigest;
        address[] coprocessorTxSenderAddresses;
    }

    error AddressEmptyCode(address target);
    error ContractAddressesMaxLengthExceeded(uint8 maxLength, uint256 actualLength);
    error ContractNotInContractAddresses(address contractAddress, address[] contractAddresses);
    error DecryptionNotDone(uint256 decryptionId);
    error DecryptionNotRequested(uint256 decryptionId);
    error DelegatorAddressInContractAddresses(address delegatorAddress, address[] contractAddresses);
    error DifferentKeyIdsNotAllowed(SnsCiphertextMaterial firstSnsCtMaterial, SnsCiphertextMaterial invalidSnsCtMaterial);
    error ECDSAInvalidSignature();
    error ECDSAInvalidSignatureLength(uint256 length);
    error ECDSAInvalidSignatureS(bytes32 s);
    error ERC1967InvalidImplementation(address implementation);
    error ERC1967NonPayable();
    error EmptyContractAddresses();
    error EmptyCtHandleContractPairs();
    error EmptyCtHandles();
    error EnforcedPause();
    error ExpectedPause();
    error FailedCall();
    error InvalidFHEType(uint8 fheTypeUint8);
    error InvalidInitialization();
    error InvalidNullDurationDays();
    error InvalidUserSignature(bytes signature);
    error KmsNodeAlreadySigned(uint256 decryptionId, address signer);
    error MaxDecryptionRequestBitSizeExceeded(uint256 maxBitSize, uint256 totalBitSize);
    error MaxDurationDaysExceeded(uint256 maxValue, uint256 actualValue);
    error NotGatewayOwner(address sender);
    error NotInitializing();
    error NotInitializingFromEmptyProxy();
    error NotOwnerOrGatewayConfig(address notOwnerOrGatewayConfig);
    error NotPauser(address notPauser);
    error NotPauserOrGatewayConfig(address notPauserOrGatewayConfig);
    error OwnableInvalidOwner(address owner);
    error OwnableUnauthorizedAccount(address account);
    error StartTimestampInFuture(uint256 currentTimestamp, uint256 startTimestamp);
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);
    error UnsupportedFHEType(FheType fheType);
    error UserAddressInContractAddresses(address userAddress, address[] contractAddresses);
    error UserDecryptionRequestExpired(uint256 currentTimestamp, IDecryption.RequestValidity requestValidity);

    event EIP712DomainChanged();
    event Initialized(uint64 version);
    event OwnershipTransferStarted(address indexed previousOwner, address indexed newOwner);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address account);
    event PublicDecryptionRequest(uint256 indexed decryptionId, SnsCiphertextMaterial[] snsCtMaterials, bytes extraData);
    event PublicDecryptionResponse(uint256 indexed decryptionId, bytes decryptedResult, bytes[] signatures, bytes extraData);
    event Unpaused(address account);
    event Upgraded(address indexed implementation);
    event UserDecryptionRequest(uint256 indexed decryptionId, SnsCiphertextMaterial[] snsCtMaterials, address userAddress, bytes publicKey, bytes extraData);
    event UserDecryptionResponse(uint256 indexed decryptionId, bytes[] userDecryptedShares, bytes[] signatures, bytes extraData);

    constructor();

    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function acceptOwnership() external;
    function checkDecryptionDone(uint256 decryptionId) external view;
    function checkDelegatedUserDecryptionReady(uint256 contractsChainId, DelegationAccounts memory delegationAccounts, CtHandleContractPair[] memory ctHandleContractPairs, address[] memory contractAddresses, bytes memory) external view;
    function checkPublicDecryptionReady(bytes32[] memory ctHandles, bytes memory) external view;
    function checkUserDecryptionReady(address userAddress, CtHandleContractPair[] memory ctHandleContractPairs, bytes memory) external view;
    function delegatedUserDecryptionRequest(CtHandleContractPair[] memory ctHandleContractPairs, IDecryption.RequestValidity memory requestValidity, DelegationAccounts memory delegationAccounts, IDecryption.ContractsInfo memory contractsInfo, bytes memory publicKey, bytes memory signature, bytes memory extraData) external;
    function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
    function getDecryptionConsensusTxSenders(uint256 decryptionId) external view returns (address[] memory);
    function getVersion() external pure returns (string memory);
    function initializeFromEmptyProxy() external;
    function owner() external view returns (address);
    function pause() external;
    function paused() external view returns (bool);
    function pendingOwner() external view returns (address);
    function proxiableUUID() external view returns (bytes32);
    function publicDecryptionRequest(bytes32[] memory ctHandles, bytes memory extraData) external;
    function publicDecryptionResponse(uint256 decryptionId, bytes memory decryptedResult, bytes memory signature, bytes memory extraData) external;
    function reinitializeV4() external;
    function renounceOwnership() external;
    function transferOwnership(address newOwner) external;
    function unpause() external;
    function upgradeToAndCall(address newImplementation, bytes memory data) external payable;
    function userDecryptionRequest(CtHandleContractPair[] memory ctHandleContractPairs, IDecryption.RequestValidity memory requestValidity, IDecryption.ContractsInfo memory contractsInfo, address userAddress, bytes memory publicKey, bytes memory signature, bytes memory extraData) external;
    function userDecryptionResponse(uint256 decryptionId, bytes memory userDecryptedShare, bytes memory signature, bytes memory extraData) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "UPGRADE_INTERFACE_VERSION",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "acceptOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "checkDecryptionDone",
    "inputs": [
      {
        "name": "decryptionId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "checkDelegatedUserDecryptionReady",
    "inputs": [
      {
        "name": "contractsChainId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "delegationAccounts",
        "type": "tuple",
        "internalType": "struct DelegationAccounts",
        "components": [
          {
            "name": "delegatorAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegatedAddress",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "ctHandleContractPairs",
        "type": "tuple[]",
        "internalType": "struct CtHandleContractPair[]",
        "components": [
          {
            "name": "ctHandle",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "contractAddress",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "contractAddresses",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "checkPublicDecryptionReady",
    "inputs": [
      {
        "name": "ctHandles",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      },
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "checkUserDecryptionReady",
    "inputs": [
      {
        "name": "userAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "ctHandleContractPairs",
        "type": "tuple[]",
        "internalType": "struct CtHandleContractPair[]",
        "components": [
          {
            "name": "ctHandle",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "contractAddress",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "delegatedUserDecryptionRequest",
    "inputs": [
      {
        "name": "ctHandleContractPairs",
        "type": "tuple[]",
        "internalType": "struct CtHandleContractPair[]",
        "components": [
          {
            "name": "ctHandle",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "contractAddress",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "requestValidity",
        "type": "tuple",
        "internalType": "struct IDecryption.RequestValidity",
        "components": [
          {
            "name": "startTimestamp",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "durationDays",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "delegationAccounts",
        "type": "tuple",
        "internalType": "struct DelegationAccounts",
        "components": [
          {
            "name": "delegatorAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegatedAddress",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "contractsInfo",
        "type": "tuple",
        "internalType": "struct IDecryption.ContractsInfo",
        "components": [
          {
            "name": "chainId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "addresses",
            "type": "address[]",
            "internalType": "address[]"
          }
        ]
      },
      {
        "name": "publicKey",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "signature",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "extraData",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "eip712Domain",
    "inputs": [],
    "outputs": [
      {
        "name": "fields",
        "type": "bytes1",
        "internalType": "bytes1"
      },
      {
        "name": "name",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "version",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "chainId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "verifyingContract",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "extensions",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getDecryptionConsensusTxSenders",
    "inputs": [
      {
        "name": "decryptionId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getVersion",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "initializeFromEmptyProxy",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "owner",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "pause",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "paused",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "pendingOwner",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "proxiableUUID",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "publicDecryptionRequest",
    "inputs": [
      {
        "name": "ctHandles",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      },
      {
        "name": "extraData",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "publicDecryptionResponse",
    "inputs": [
      {
        "name": "decryptionId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "decryptedResult",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "signature",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "extraData",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "reinitializeV4",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "renounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "transferOwnership",
    "inputs": [
      {
        "name": "newOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "unpause",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "upgradeToAndCall",
    "inputs": [
      {
        "name": "newImplementation",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "userDecryptionRequest",
    "inputs": [
      {
        "name": "ctHandleContractPairs",
        "type": "tuple[]",
        "internalType": "struct CtHandleContractPair[]",
        "components": [
          {
            "name": "ctHandle",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "contractAddress",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "requestValidity",
        "type": "tuple",
        "internalType": "struct IDecryption.RequestValidity",
        "components": [
          {
            "name": "startTimestamp",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "durationDays",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "contractsInfo",
        "type": "tuple",
        "internalType": "struct IDecryption.ContractsInfo",
        "components": [
          {
            "name": "chainId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "addresses",
            "type": "address[]",
            "internalType": "address[]"
          }
        ]
      },
      {
        "name": "userAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "publicKey",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "signature",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "extraData",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "userDecryptionResponse",
    "inputs": [
      {
        "name": "decryptionId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "userDecryptedShare",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "signature",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "extraData",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "EIP712DomainChanged",
    "inputs": [],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Initialized",
    "inputs": [
      {
        "name": "version",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferStarted",
    "inputs": [
      {
        "name": "previousOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "previousOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Paused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "PublicDecryptionRequest",
    "inputs": [
      {
        "name": "decryptionId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "snsCtMaterials",
        "type": "tuple[]",
        "indexed": false,
        "internalType": "struct SnsCiphertextMaterial[]",
        "components": [
          {
            "name": "ctHandle",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "keyId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "snsCiphertextDigest",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "coprocessorTxSenderAddresses",
            "type": "address[]",
            "internalType": "address[]"
          }
        ]
      },
      {
        "name": "extraData",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "PublicDecryptionResponse",
    "inputs": [
      {
        "name": "decryptionId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "decryptedResult",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      },
      {
        "name": "signatures",
        "type": "bytes[]",
        "indexed": false,
        "internalType": "bytes[]"
      },
      {
        "name": "extraData",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Unpaused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Upgraded",
    "inputs": [
      {
        "name": "implementation",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "UserDecryptionRequest",
    "inputs": [
      {
        "name": "decryptionId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "snsCtMaterials",
        "type": "tuple[]",
        "indexed": false,
        "internalType": "struct SnsCiphertextMaterial[]",
        "components": [
          {
            "name": "ctHandle",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "keyId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "snsCiphertextDigest",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "coprocessorTxSenderAddresses",
            "type": "address[]",
            "internalType": "address[]"
          }
        ]
      },
      {
        "name": "userAddress",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "publicKey",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      },
      {
        "name": "extraData",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "UserDecryptionResponse",
    "inputs": [
      {
        "name": "decryptionId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "userDecryptedShares",
        "type": "bytes[]",
        "indexed": false,
        "internalType": "bytes[]"
      },
      {
        "name": "signatures",
        "type": "bytes[]",
        "indexed": false,
        "internalType": "bytes[]"
      },
      {
        "name": "extraData",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "AddressEmptyCode",
    "inputs": [
      {
        "name": "target",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ContractAddressesMaxLengthExceeded",
    "inputs": [
      {
        "name": "maxLength",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "actualLength",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ContractNotInContractAddresses",
    "inputs": [
      {
        "name": "contractAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "contractAddresses",
        "type": "address[]",
        "internalType": "address[]"
      }
    ]
  },
  {
    "type": "error",
    "name": "DecryptionNotDone",
    "inputs": [
      {
        "name": "decryptionId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "DecryptionNotRequested",
    "inputs": [
      {
        "name": "decryptionId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "DelegatorAddressInContractAddresses",
    "inputs": [
      {
        "name": "delegatorAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "contractAddresses",
        "type": "address[]",
        "internalType": "address[]"
      }
    ]
  },
  {
    "type": "error",
    "name": "DifferentKeyIdsNotAllowed",
    "inputs": [
      {
        "name": "firstSnsCtMaterial",
        "type": "tuple",
        "internalType": "struct SnsCiphertextMaterial",
        "components": [
          {
            "name": "ctHandle",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "keyId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "snsCiphertextDigest",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "coprocessorTxSenderAddresses",
            "type": "address[]",
            "internalType": "address[]"
          }
        ]
      },
      {
        "name": "invalidSnsCtMaterial",
        "type": "tuple",
        "internalType": "struct SnsCiphertextMaterial",
        "components": [
          {
            "name": "ctHandle",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "keyId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "snsCiphertextDigest",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "coprocessorTxSenderAddresses",
            "type": "address[]",
            "internalType": "address[]"
          }
        ]
      }
    ]
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignatureLength",
    "inputs": [
      {
        "name": "length",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignatureS",
    "inputs": [
      {
        "name": "s",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC1967InvalidImplementation",
    "inputs": [
      {
        "name": "implementation",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC1967NonPayable",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EmptyContractAddresses",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EmptyCtHandleContractPairs",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EmptyCtHandles",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EnforcedPause",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ExpectedPause",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FailedCall",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidFHEType",
    "inputs": [
      {
        "name": "fheTypeUint8",
        "type": "uint8",
        "internalType": "uint8"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidInitialization",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidNullDurationDays",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidUserSignature",
    "inputs": [
      {
        "name": "signature",
        "type": "bytes",
        "internalType": "bytes"
      }
    ]
  },
  {
    "type": "error",
    "name": "KmsNodeAlreadySigned",
    "inputs": [
      {
        "name": "decryptionId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "signer",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "MaxDecryptionRequestBitSizeExceeded",
    "inputs": [
      {
        "name": "maxBitSize",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "totalBitSize",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "MaxDurationDaysExceeded",
    "inputs": [
      {
        "name": "maxValue",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "actualValue",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "NotGatewayOwner",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "NotInitializing",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotInitializingFromEmptyProxy",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotOwnerOrGatewayConfig",
    "inputs": [
      {
        "name": "notOwnerOrGatewayConfig",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "NotPauser",
    "inputs": [
      {
        "name": "notPauser",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "NotPauserOrGatewayConfig",
    "inputs": [
      {
        "name": "notPauserOrGatewayConfig",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "OwnableInvalidOwner",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "OwnableUnauthorizedAccount",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "StartTimestampInFuture",
    "inputs": [
      {
        "name": "currentTimestamp",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "startTimestamp",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "UUPSUnauthorizedCallContext",
    "inputs": []
  },
  {
    "type": "error",
    "name": "UUPSUnsupportedProxiableUUID",
    "inputs": [
      {
        "name": "slot",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "UnsupportedFHEType",
    "inputs": [
      {
        "name": "fheType",
        "type": "uint8",
        "internalType": "enum FheType"
      }
    ]
  },
  {
    "type": "error",
    "name": "UserAddressInContractAddresses",
    "inputs": [
      {
        "name": "userAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "contractAddresses",
        "type": "address[]",
        "internalType": "address[]"
      }
    ]
  },
  {
    "type": "error",
    "name": "UserDecryptionRequestExpired",
    "inputs": [
      {
        "name": "currentTimestamp",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "requestValidity",
        "type": "tuple",
        "internalType": "struct IDecryption.RequestValidity",
        "components": [
          {
            "name": "startTimestamp",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "durationDays",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ]
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod Decryption {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a06040523073ffffffffffffffffffffffffffffffffffffffff1660809073ffffffffffffffffffffffffffffffffffffffff1681525034801562000043575f80fd5b50620000546200005a60201b60201c565b620001c4565b5f6200006b6200015e60201b60201c565b9050805f0160089054906101000a900460ff1615620000b6576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b67ffffffffffffffff8016815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff16146200015b5767ffffffffffffffff815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d267ffffffffffffffff604051620001529190620001a9565b60405180910390a15b50565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b5f67ffffffffffffffff82169050919050565b620001a38162000185565b82525050565b5f602082019050620001be5f83018462000198565b92915050565b608051617b78620001eb5f395f818161306f015281816130c401526133660152617b785ff3fe608060405260043610610165575f3560e01c80638456cb59116100d0578063ad3cb1cc11610089578063e31ec19d11610063578063e31ec19d14610461578063f1b57adb14610489578063f2fde38b146104b1578063f4a6e2ce146104d957610165565b8063ad3cb1cc146103e5578063d8998f451461040f578063e30c39781461043757610165565b80638456cb59146102fd57806384b0196e146103135780638da5cb5b146103435780639f98d8991461036d5780639fad5a2f14610395578063a6090439146103bd57610165565b80634f1ef286116101225780634f1ef2861461023957806352d1902d146102555780635c975abb1461027f5780636f8913bc146102a9578063715018a6146102d157806379ba5097146102e757610165565b8063046f9eb3146101695780630900cc69146101915780630d8e6e2c146101cd578063123abb28146101f757806339f738101461020d5780633f4ba83a14610223575b5f80fd5b348015610174575f80fd5b5061018f600480360381019061018a91906152dd565b610501565b005b34801561019c575f80fd5b506101b760048036038101906101b291906153a1565b6109ba565b6040516101c491906154b3565b60405180910390f35b3480156101d8575f80fd5b506101e1610a8b565b6040516101ee919061555d565b60405180910390f35b348015610202575f80fd5b5061020b610b06565b005b348015610218575f80fd5b50610221610c93565b005b34801561022e575f80fd5b50610237610edb565b005b610253600480360381019061024e91906156cf565b611023565b005b348015610260575f80fd5b50610269611042565b6040516102769190615741565b60405180910390f35b34801561028a575f80fd5b50610293611073565b6040516102a09190615774565b60405180910390f35b3480156102b4575f80fd5b506102cf60048036038101906102ca91906152dd565b611095565b005b3480156102dc575f80fd5b506102e5611462565b005b3480156102f2575f80fd5b506102fb611475565b005b348015610308575f80fd5b50610311611503565b005b34801561031e575f80fd5b5061032761164b565b60405161033a979695949392919061589c565b60405180910390f35b34801561034e575f80fd5b50610357611754565b604051610364919061591e565b60405180910390f35b348015610378575f80fd5b50610393600480360381019061038e9190615a03565b611789565b005b3480156103a0575f80fd5b506103bb60048036038101906103b69190615b16565b611a28565b005b3480156103c8575f80fd5b506103e360048036038101906103de91906153a1565b611ff2565b005b3480156103f0575f80fd5b506103f9612062565b604051610406919061555d565b60405180910390f35b34801561041a575f80fd5b5061043560048036038101906104309190615ca6565b61209b565b005b348015610442575f80fd5b5061044b612259565b604051610458919061591e565b60405180910390f35b34801561046c575f80fd5b5061048760048036038101906104829190615ca6565b61228e565b005b348015610494575f80fd5b506104af60048036038101906104aa9190615d24565b6123d3565b005b3480156104bc575f80fd5b506104d760048036038101906104d29190615e5e565b612889565b005b3480156104e4575f80fd5b506104ff60048036038101906104fa9190615e89565b612942565b005b73c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff1663c6275258336040518263ffffffff1660e01b815260040161054e919061591e565b5f6040518083038186803b158015610564575f80fd5b505afa158015610576573d5f803e3d5ffd5b505050505f610583612b4e565b90505f8814806105b85750805f0154881180156105b7575060f8600260058111156105b1576105b0615f1a565b5b901b8811155b5b806105c6575080600c015488115b1561060857876040517fd48af9420000000000000000000000000000000000000000000000000000000081526004016105ff9190615f47565b60405180910390fd5b5f816006015f8a81526020019081526020015f206040518060400160405290815f8201805461063690615f8d565b80601f016020809104026020016040519081016040528092919081815260200182805461066290615f8d565b80156106ad5780601f10610684576101008083540402835291602001916106ad565b820191905f5260205f20905b81548152906001019060200180831161069057829003601f168201915b505050505081526020016001820180548060200260200160405190810160405280929190818152602001828054801561070357602002820191905f5260205f20905b8154815260200190600101908083116106ef575b50505050508152505090505f6040518060800160405280835f01518152602001836020015181526020018a8a8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050815260200186868080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081525090505f6107c982612b75565b90506107d78b828a8a612c3c565b5f846005015f8d81526020019081526020015f20905080898990918060018154018082558091505060019003905f5260205f20015f909192909192909192909192509182610826929190616164565b50846008015f8d81526020019081526020015f208b8b90918060018154018082558091505060019003905f5260205f20015f909192909192909192909192509182610872929190616164565b50846009015f8d81526020019081526020015f205f805f1b81526020019081526020015f2033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550846001015f8d81526020019081526020015f205f9054906101000a900460ff1615801561092b575061092a8180549050612e1d565b5b156109ac576001856001015f8e81526020019081526020015f205f6101000a81548160ff0219169083151502179055508b7f1cbf1c6e9ea3dbcfd49b9690d93cd4b93b3cb22913f99b71d3aecae60b25fc56866008015f8f81526020019081526020015f20838a8a6040516109a39493929190616455565b60405180910390a25b505050505050505050505050565b60605f6109c5612b4e565b90505f81600a015f8581526020019081526020015f20549050816009015f8581526020019081526020015f205f8281526020019081526020015f20805480602002602001604051908101604052809291908181526020018280548015610a7d57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311610a34575b505050505092505050919050565b60606040518060400160405280600a81526020017f44656372797074696f6e00000000000000000000000000000000000000000000815250610acc5f612eae565b610ad66004612eae565b610adf5f612eae565b604051602001610af2949392919061656f565b604051602081830303815290604052905090565b60065f610b11612f78565b9050805f0160089054906101000a900460ff1680610b5957508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15610b90576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055505f610bde612b4e565b905060f860016005811115610bf657610bf5615f1a565b5b901b815f0154610c0691906165fa565b81600b018190555060f860026005811115610c2457610c23615f1a565b5b901b815f0154610c3491906165fa565b81600c0181905550505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051610c87919061664f565b60405180910390a15050565b6001610c9d612f9f565b67ffffffffffffffff1614610cde576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60065f610ce9612f78565b9050805f0160089054906101000a900460ff1680610d3157508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15610d68576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff021916908315150217905550610e216040518060400160405280600a81526020017f44656372797074696f6e000000000000000000000000000000000000000000008152506040518060400160405280600181526020017f3100000000000000000000000000000000000000000000000000000000000000815250612fc3565b610e31610e2c611754565b612fd9565b610e39612fed565b5f610e42612b4e565b905060f860016005811115610e5a57610e59615f1a565b5b901b81600b018190555060f860026005811115610e7a57610e79615f1a565b5b901b81600c0181905550505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051610ecf919061664f565b60405180910390a15050565b73c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610f38573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f5c919061667c565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614158015610fd7575073c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614155b1561101957336040517fe19166ee000000000000000000000000000000000000000000000000000000008152600401611010919061591e565b60405180910390fd5b611021612fff565b565b61102b61306d565b61103482613153565b61103e8282613246565b5050565b5f61104b613364565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b5f8061107d6133eb565b9050805f015f9054906101000a900460ff1691505090565b73c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff1663c6275258336040518263ffffffff1660e01b81526004016110e2919061591e565b5f6040518083038186803b1580156110f8575f80fd5b505afa15801561110a573d5f803e3d5ffd5b505050505f611117612b4e565b90505f88148061114c5750805f01548811801561114b575060f86001600581111561114557611144615f1a565b5b901b8811155b5b8061115a575080600b015488115b1561119c57876040517fd48af9420000000000000000000000000000000000000000000000000000000081526004016111939190615f47565b60405180910390fd5b5f6040518060600160405280836004015f8c81526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561120357602002820191905f5260205f20905b8154815260200190600101908083116111ef575b5050505050815260200189898080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050815260200185858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081525090505f6112a982613412565b90506112b78a828989612c3c565b5f836003015f8c81526020019081526020015f205f8381526020019081526020015f20905080888890918060018154018082558091505060019003905f5260205f20015f909192909192909192909192509182611315929190616164565b50836009015f8c81526020019081526020015f205f8381526020019081526020015f2033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550836001015f8c81526020019081526020015f205f9054906101000a900460ff161580156113cc57506113cb81805490506134cc565b5b15611455576001846001015f8d81526020019081526020015f205f6101000a81548160ff0219169083151502179055508184600a015f8d81526020019081526020015f20819055508a7fd7e58a367a0a6c298e76ad5d240004e327aa1423cbe4bd7ff85d4c715ef8d15f8b8b848a8a60405161144c9594939291906166a7565b60405180910390a25b5050505050505050505050565b61146a61355d565b6114735f6135e4565b565b5f61147e613621565b90508073ffffffffffffffffffffffffffffffffffffffff1661149f612259565b73ffffffffffffffffffffffffffffffffffffffff16146114f757806040517f118cdaa70000000000000000000000000000000000000000000000000000000081526004016114ee919061591e565b60405180910390fd5b611500816135e4565b50565b73c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff16637008b5486040518163ffffffff1660e01b8152600401602060405180830381865afa158015611560573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611584919061667c565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16141580156115ff575073c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614155b1561164157336040517f388916bb000000000000000000000000000000000000000000000000000000008152600401611638919061591e565b60405180910390fd5b611649613628565b565b5f6060805f805f60605f61165d613697565b90505f801b815f015414801561167857505f801b8160010154145b6116b7576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016116ae9061673f565b60405180910390fd5b6116bf6136be565b6116c761375c565b46305f801b5f67ffffffffffffffff8111156116e6576116e56155ab565b5b6040519080825280602002602001820160405280156117145781602001602082028036833780820191505090505b507f0f0000000000000000000000000000000000000000000000000000000000000095949392919097509750975097509750975097505090919293949596565b5f8061175e6137fa565b9050805f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1691505090565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff166351c41d0e898987876040518563ffffffff1660e01b81526004016117dc9493929190616820565b5f6040518083038186803b1580156117f2575f80fd5b505afa158015611804573d5f803e3d5ffd5b505050505f5b86869050811015611a1d5773a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16633bce498d8888848181106118585761185761685e565b5b9050604002015f01358a5f0160208101906118739190615e5e565b6040518363ffffffff1660e01b815260040161189092919061688b565b5f6040518083038186803b1580156118a6575f80fd5b505afa1580156118b8573d5f803e3d5ffd5b5050505073a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16633bce498d8888848181106118ff576118fe61685e565b5b9050604002015f013589898581811061191b5761191a61685e565b5b90506040020160200160208101906119339190615e5e565b6040518363ffffffff1660e01b815260040161195092919061688b565b5f6040518083038186803b158015611966575f80fd5b505afa158015611978573d5f803e3d5ffd5b5050505073d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663d4476f638888848181106119bf576119be61685e565b5b9050604002015f01356040518263ffffffff1660e01b81526004016119e49190615741565b5f6040518083038186803b1580156119fa575f80fd5b505afa158015611a0c573d5f803e3d5ffd5b50505050808060010191505061180a565b505050505050505050565b611a30613821565b5f878060200190611a4191906168be565b905003611a7a576040517f57cfa21700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600a60ff16878060200190611a8f91906168be565b90501115611ae857600a878060200190611aa991906168be565b90506040517fc5ab467e000000000000000000000000000000000000000000000000000000008152600401611adf92919061693b565b60405180910390fd5b611b0189803603810190611afc91906169b7565b613862565b611b6a878060200190611b1491906168be565b808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f82011690508083019250505050505050895f016020810190611b659190615e5e565b6139ad565b15611bcf57875f016020810190611b819190615e5e565b878060200190611b9191906168be565b6040517fc3446ac7000000000000000000000000000000000000000000000000000000008152600401611bc6939291906169e2565b60405180910390fd5b5f611c3b8c8c8a8060200190611be591906168be565b808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f820116905080830192505050505050508c5f016020810190611c369190615e5e565b613a2b565b905073a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff166351c41d0e895f01358b8b8060200190611c8291906168be565b6040518563ffffffff1660e01b8152600401611ca19493929190616820565b5f6040518083038186803b158015611cb7575f80fd5b505afa158015611cc9573d5f803e3d5ffd5b505050505f6040518060e0016040528089898080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081526020018a8060200190611d3291906168be565b808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f8201169050808301925050505050505081526020018b5f016020810190611d889190615e5e565b73ffffffffffffffffffffffffffffffffffffffff1681526020018a5f013581526020018c5f013581526020018c60200135815260200185858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050508152509050611e26818b6020016020810190611e1f9190615e5e565b8888613d00565b5f73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663a14f8971846040518263ffffffff1660e01b8152600401611e749190616ac9565b5f60405180830381865afa158015611e8e573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f82011682018060405250810190611eb69190616d56565b9050611ec181613dd6565b5f611eca612b4e565b905080600c015f815480929190611ee090616d9d565b91905055505f81600c0154905060405180604001604052808c8c8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050815260200186815250826006015f8381526020019081526020015f205f820151815f019081611f6b9190616dee565b506020820151816001019080519060200190611f88929190615187565b50905050807ff9011bd6ba0da6049c520d70fe5971f17ed7ab795486052544b51019896c596b848f6020016020810190611fc29190615e5e565b8e8e8c8c604051611fd896959493929190617044565b60405180910390a250505050505050505050505050505050565b5f611ffb612b4e565b9050806001015f8381526020019081526020015f205f9054906101000a900460ff1661205e57816040517f0bf014060000000000000000000000000000000000000000000000000000000081526004016120559190615f47565b60405180910390fd5b5050565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b6120a3613821565b5f84849050036120df576040517f2de7543800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6121288484808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f82011690508083019250505050505050613ebc565b5f73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663a14f897186866040518363ffffffff1660e01b8152600401612178929190617108565b5f60405180830381865afa158015612192573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f820116820180604052508101906121ba9190616d56565b90506121c581613dd6565b5f6121ce612b4e565b905080600b015f8154809291906121e490616d9d565b91905055505f81600b015490508686836004015f8481526020019081526020015f2091906122139291906151d2565b50807f22db480a39bd72556438aadb4a32a3d2a6638b87c03bbec5fef6997e109587ff8487876040516122489392919061712a565b60405180910390a250505050505050565b5f80612263613fe4565b9050805f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1691505090565b5f5b848490508110156123cc5773a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663193f3f2c8686848181106122de576122dd61685e565b5b905060200201356040518263ffffffff1660e01b81526004016123019190615741565b5f6040518083038186803b158015612317575f80fd5b505afa158015612329573d5f803e3d5ffd5b5050505073d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663d4476f638686848181106123705761236f61685e565b5b905060200201356040518263ffffffff1660e01b81526004016123939190615741565b5f6040518083038186803b1580156123a9575f80fd5b505afa1580156123bb573d5f803e3d5ffd5b505050508080600101915050612290565b5050505050565b6123db613821565b5f8880602001906123ec91906168be565b905003612425576040517f57cfa21700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600a60ff1688806020019061243a91906168be565b9050111561249357600a88806020019061245491906168be565b90506040517fc5ab467e00000000000000000000000000000000000000000000000000000000815260040161248a92919061693b565b60405180910390fd5b6124ac898036038101906124a791906169b7565b613862565b6125048880602001906124bf91906168be565b808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f82011690508083019250505050505050886139ad565b15612558578688806020019061251a91906168be565b6040517fdc4d78b100000000000000000000000000000000000000000000000000000000815260040161254f939291906169e2565b60405180910390fd5b5f6125b38c8c8b806020019061256e91906168be565b808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f820116905080830192505050505050508b613a2b565b90505f6040518060c0016040528089898080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081526020018b806020019061261a91906168be565b808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f8201169050808301925050505050505081526020018b5f013581526020018c5f013581526020018c60200135815260200185858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081525090506126cf818a888861400b565b5f73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663a14f8971846040518263ffffffff1660e01b815260040161271d9190616ac9565b5f60405180830381865afa158015612737573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f8201168201806040525081019061275f9190616d56565b905061276a81613dd6565b5f612773612b4e565b905080600c015f81548092919061278990616d9d565b91905055505f81600c0154905060405180604001604052808c8c8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050815260200186815250826006015f8381526020019081526020015f205f820151815f0190816128149190616dee565b506020820151816001019080519060200190612831929190615187565b50905050807ff9011bd6ba0da6049c520d70fe5971f17ed7ab795486052544b51019896c596b848e8e8e8c8c60405161286f96959493929190617044565b60405180910390a250505050505050505050505050505050565b61289161355d565b5f61289a613fe4565b905081815f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508173ffffffffffffffffffffffffffffffffffffffff166128fc611754565b73ffffffffffffffffffffffffffffffffffffffff167f38d16b8cac22d99fc7c124b9cd0de2d3fa1faef420bfe791d8c362d765e2270060405160405180910390a35050565b5f5b84849050811015612b465773a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16633bce498d8686848181106129925761299161685e565b5b9050604002015f0135886040518363ffffffff1660e01b81526004016129b992919061688b565b5f6040518083038186803b1580156129cf575f80fd5b505afa1580156129e1573d5f803e3d5ffd5b5050505073a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16633bce498d868684818110612a2857612a2761685e565b5b9050604002015f0135878785818110612a4457612a4361685e565b5b9050604002016020016020810190612a5c9190615e5e565b6040518363ffffffff1660e01b8152600401612a7992919061688b565b5f6040518083038186803b158015612a8f575f80fd5b505afa158015612aa1573d5f803e3d5ffd5b5050505073d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663d4476f63868684818110612ae857612ae761685e565b5b9050604002015f01356040518263ffffffff1660e01b8152600401612b0d9190615741565b5f6040518083038186803b158015612b23575f80fd5b505afa158015612b35573d5f803e3d5ffd5b505050508080600101915050612944565b505050505050565b5f7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee700905090565b5f612c356040518060a00160405280606d8152602001617955606d913980519060200120835f0151805190602001208460200151604051602001612bb991906171ed565b604051602081830303815290604052805190602001208560400151805190602001208660600151604051602001612bf0919061723d565b60405160208183030381529060405280519060200120604051602001612c1a959493929190617253565b604051602081830303815290604052805190602001206140e1565b9050919050565b5f612c45612b4e565b90505f612c958585858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050506140fa565b905073c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff16636c88eb43826040518263ffffffff1660e01b8152600401612ce4919061591e565b5f6040518083038186803b158015612cfa575f80fd5b505afa158015612d0c573d5f803e3d5ffd5b50505050816002015f8781526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615612daf5785816040517f99ec48d9000000000000000000000000000000000000000000000000000000008152600401612da69291906172a4565b60405180910390fd5b6001826002015f8881526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550505050505050565b5f8073c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff1663c2b429866040518163ffffffff1660e01b8152600401602060405180830381865afa158015612e7c573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612ea091906172cb565b905080831015915050919050565b60605f6001612ebc84614124565b0190505f8167ffffffffffffffff811115612eda57612ed96155ab565b5b6040519080825280601f01601f191660200182016040528015612f0c5781602001600182028036833780820191505090505b5090505f82602001820190505b600115612f6d578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a8581612f6257612f616172f6565b5b0494505f8503612f19575b819350505050919050565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b5f612fa8612f78565b5f015f9054906101000a900467ffffffffffffffff16905090565b612fcb614275565b612fd582826142b5565b5050565b612fe1614275565b612fea81614306565b50565b612ff5614275565b612ffd61438a565b565b6130076143ba565b5f6130106133eb565b90505f815f015f6101000a81548160ff0219169083151502179055507f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa613055613621565b604051613062919061591e565b60405180910390a150565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff16148061311a57507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166131016143fa565b73ffffffffffffffffffffffffffffffffffffffff1614155b15613151576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b73c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156131b0573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906131d4919061667c565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461324357336040517f0e56cf3d00000000000000000000000000000000000000000000000000000000815260040161323a919061591e565b60405180910390fd5b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa9250505080156132ae57506040513d601f19601f820116820180604052508101906132ab9190617323565b60015b6132ef57816040517f4c9c8ce30000000000000000000000000000000000000000000000000000000081526004016132e6919061591e565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b811461335557806040517faa1d49a400000000000000000000000000000000000000000000000000000000815260040161334c9190615741565b60405180910390fd5b61335f838361444d565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff16146133e9576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300905090565b5f6134c56040518060800160405280605481526020016179c26054913980519060200120835f015160405160200161344a91906171ed565b604051602081830303815290604052805190602001208460200151805190602001208560400151604051602001613481919061723d565b604051602081830303815290604052805190602001206040516020016134aa949392919061734e565b604051602081830303815290604052805190602001206140e1565b9050919050565b5f8073c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff16632a3889986040518163ffffffff1660e01b8152600401602060405180830381865afa15801561352b573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061354f91906172cb565b905080831015915050919050565b613565613621565b73ffffffffffffffffffffffffffffffffffffffff16613583611754565b73ffffffffffffffffffffffffffffffffffffffff16146135e2576135a6613621565b6040517f118cdaa70000000000000000000000000000000000000000000000000000000081526004016135d9919061591e565b60405180910390fd5b565b5f6135ed613fe4565b9050805f015f6101000a81549073ffffffffffffffffffffffffffffffffffffffff021916905561361d826144bf565b5050565b5f33905090565b613630613821565b5f6136396133eb565b90506001815f015f6101000a81548160ff0219169083151502179055507f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a25861367f613621565b60405161368c919061591e565b60405180910390a150565b5f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100905090565b60605f6136c9613697565b90508060020180546136da90615f8d565b80601f016020809104026020016040519081016040528092919081815260200182805461370690615f8d565b80156137515780601f1061372857610100808354040283529160200191613751565b820191905f5260205f20905b81548152906001019060200180831161373457829003601f168201915b505050505091505090565b60605f613767613697565b905080600301805461377890615f8d565b80601f01602080910402602001604051908101604052809291908181526020018280546137a490615f8d565b80156137ef5780601f106137c6576101008083540402835291602001916137ef565b820191905f5260205f20905b8154815290600101906020018083116137d257829003601f168201915b505050505091505090565b5f7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300905090565b613829611073565b15613860576040517fd93c066500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f81602001510361389f576040517fde2859c100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61016d61ffff16816020015111156138f65761016d81602001516040517f329518630000000000000000000000000000000000000000000000000000000081526004016138ed9291906173ce565b60405180910390fd5b42815f015111156139435742815f01516040517ff24c088700000000000000000000000000000000000000000000000000000000815260040161393a9291906173f5565b60405180910390fd5b42620151808260200151613957919061741c565b825f015161396591906165fa565b10156139aa5742816040517f303480400000000000000000000000000000000000000000000000000000000081526004016139a192919061748a565b60405180910390fd5b50565b5f805f90505b8351811015613a20578273ffffffffffffffffffffffffffffffffffffffff168482815181106139e6576139e561685e565b5b602002602001015173ffffffffffffffffffffffffffffffffffffffff1603613a13576001915050613a25565b80806001019150506139b3565b505f90505b92915050565b60605f8585905003613a69576040517fa6a6cb2100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8484905067ffffffffffffffff811115613a8657613a856155ab565b5b604051908082528060200260200182016040528015613ab45781602001602082028036833780820191505090505b5090505f805b86869050811015613cab575f878783818110613ad957613ad861685e565b5b9050604002015f013590505f888884818110613af857613af761685e565b5b9050604002016020016020810190613b109190615e5e565b90505f613b1c83614590565b9050613b278161461a565b61ffff1685613b3691906165fa565b945073a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16633bce498d84896040518363ffffffff1660e01b8152600401613b8792919061688b565b5f6040518083038186803b158015613b9d575f80fd5b505afa158015613baf573d5f803e3d5ffd5b5050505073a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16633bce498d84846040518363ffffffff1660e01b8152600401613c0292919061688b565b5f6040518083038186803b158015613c18575f80fd5b505afa158015613c2a573d5f803e3d5ffd5b50505050613c3888836139ad565b613c7b5781886040517fa4c30391000000000000000000000000000000000000000000000000000000008152600401613c729291906174b1565b60405180910390fd5b82868581518110613c8f57613c8e61685e565b5b6020026020010181815250505050508080600101915050613aba565b50610800811115613cf757610800816040517fe7f4895d000000000000000000000000000000000000000000000000000000008152600401613cee9291906173f5565b60405180910390fd5b50949350505050565b5f613d0a856148a7565b90505f613d5a8285858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050506140fa565b90508473ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614613dce5783836040517f2a873d27000000000000000000000000000000000000000000000000000000008152600401613dc59291906174df565b60405180910390fd5b505050505050565b600181511115613eb9575f815f81518110613df457613df361685e565b5b60200260200101516020015190505f600190505b8251811015613eb65781838281518110613e2557613e2461685e565b5b60200260200101516020015114613ea957825f81518110613e4957613e4861685e565b5b6020026020010151838281518110613e6457613e6361685e565b5b60200260200101516040517fcfae921f000000000000000000000000000000000000000000000000000000008152600401613ea0929190617561565b60405180910390fd5b8080600101915050613e08565b50505b50565b5f805b8251811015613f94575f838281518110613edc57613edb61685e565b5b602002602001015190505f613ef082614590565b9050613efb8161461a565b61ffff1684613f0a91906165fa565b935073a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663193f3f2c836040518263ffffffff1660e01b8152600401613f599190615741565b5f6040518083038186803b158015613f6f575f80fd5b505afa158015613f81573d5f803e3d5ffd5b5050505050508080600101915050613ebf565b50610800811115613fe057610800816040517fe7f4895d000000000000000000000000000000000000000000000000000000008152600401613fd79291906173f5565b60405180910390fd5b5050565b5f7f237e158222e3e6968b72b9db0d8043aacf074ad9f650f0d1606b4d82ee432c00905090565b5f61401585614984565b90505f6140658285858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050506140fa565b90508473ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16146140d95783836040517f2a873d270000000000000000000000000000000000000000000000000000000081526004016140d09291906174df565b60405180910390fd5b505050505050565b5f6140f36140ed614a5a565b83614a68565b9050919050565b5f805f806141088686614aa8565b9250925092506141188282614afd565b82935050505092915050565b5f805f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f0100000000000000008310614180577a184f03e93ff9f4daa797ed6e38ed64bf6a1f0100000000000000008381614176576141756172f6565b5b0492506040810190505b6d04ee2d6d415b85acef810000000083106141bd576d04ee2d6d415b85acef810000000083816141b3576141b26172f6565b5b0492506020810190505b662386f26fc1000083106141ec57662386f26fc1000083816141e2576141e16172f6565b5b0492506010810190505b6305f5e1008310614215576305f5e100838161420b5761420a6172f6565b5b0492506008810190505b612710831061423a5761271083816142305761422f6172f6565b5b0492506004810190505b6064831061425d5760648381614253576142526172f6565b5b0492506002810190505b600a831061426c576001810190505b80915050919050565b61427d614c5f565b6142b3576040517fd7e6bcf800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b6142bd614275565b5f6142c6613697565b9050828160020190816142d991906175ee565b50818160030190816142eb91906175ee565b505f801b815f01819055505f801b8160010181905550505050565b61430e614275565b5f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff160361437e575f6040517f1e4fbdf7000000000000000000000000000000000000000000000000000000008152600401614375919061591e565b60405180910390fd5b614387816135e4565b50565b614392614275565b5f61439b6133eb565b90505f815f015f6101000a81548160ff02191690831515021790555050565b6143c2611073565b6143f8576040517f8dfc202b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f6144267f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b614c7d565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b61445682614c86565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f815111156144b2576144ac8282614d4f565b506144bb565b6144ba614dcf565b5b5050565b5f6144c86137fa565b90505f815f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905082825f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508273ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a3505050565b5f8060f860f084901b901c5f1c90506053808111156145b2576145b1615f1a565b5b60ff168160ff1611156145fc57806040517f641950d70000000000000000000000000000000000000000000000000000000081526004016145f391906176bd565b60405180910390fd5b8060ff16605381111561461257614611615f1a565b5b915050919050565b5f80605381111561462e5761462d615f1a565b5b82605381111561464157614640615f1a565b5b0361464f57600290506148a2565b6002605381111561466357614662615f1a565b5b82605381111561467657614675615f1a565b5b0361468457600890506148a2565b6003605381111561469857614697615f1a565b5b8260538111156146ab576146aa615f1a565b5b036146b957601090506148a2565b600460538111156146cd576146cc615f1a565b5b8260538111156146e0576146df615f1a565b5b036146ee57602090506148a2565b6005605381111561470257614701615f1a565b5b82605381111561471557614714615f1a565b5b0361472357604090506148a2565b6006605381111561473757614736615f1a565b5b82605381111561474a57614749615f1a565b5b0361475857608090506148a2565b6007605381111561476c5761476b615f1a565b5b82605381111561477f5761477e615f1a565b5b0361478d5760a090506148a2565b600860538111156147a1576147a0615f1a565b5b8260538111156147b4576147b3615f1a565b5b036147c35761010090506148a2565b600960538111156147d7576147d6615f1a565b5b8260538111156147ea576147e9615f1a565b5b036147f95761020090506148a2565b600a605381111561480d5761480c615f1a565b5b8260538111156148205761481f615f1a565b5b0361482f5761040090506148a2565b600b605381111561484357614842615f1a565b5b82605381111561485657614855615f1a565b5b036148655761080090506148a2565b816040517fbe7830b1000000000000000000000000000000000000000000000000000000008152600401614899919061771c565b60405180910390fd5b919050565b5f8060405180610100016040528060c28152602001617ab660c2913980519060200120835f01518051906020012084602001516040516020016148ea91906177c1565b604051602081830303815290604052805190602001208560400151866060015187608001518860a001518960c00151604051602001614929919061723d565b604051602081830303815290604052805190602001206040516020016149569897969594939291906177d7565b60405160208183030381529060405280519060200120905061497c836060015182614e0b565b915050919050565b5f806040518060c0016040528060a08152602001617a1660a0913980519060200120835f01518051906020012084602001516040516020016149c691906177c1565b604051602081830303815290604052805190602001208560400151866060015187608001518860a00151604051602001614a00919061723d565b60405160208183030381529060405280519060200120604051602001614a2c9796959493929190617853565b604051602081830303815290604052805190602001209050614a52836040015182614e0b565b915050919050565b5f614a63614e7f565b905090565b5f6040517f190100000000000000000000000000000000000000000000000000000000000081528360028201528260228201526042812091505092915050565b5f805f6041845103614ae8575f805f602087015192506040870151915060608701515f1a9050614ada88828585614ee2565b955095509550505050614af6565b5f600285515f1b9250925092505b9250925092565b5f6003811115614b1057614b0f615f1a565b5b826003811115614b2357614b22615f1a565b5b0315614c5b5760016003811115614b3d57614b3c615f1a565b5b826003811115614b5057614b4f615f1a565b5b03614b87576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60026003811115614b9b57614b9a615f1a565b5b826003811115614bae57614bad615f1a565b5b03614bf257805f1c6040517ffce698f7000000000000000000000000000000000000000000000000000000008152600401614be99190615f47565b60405180910390fd5b600380811115614c0557614c04615f1a565b5b826003811115614c1857614c17615f1a565b5b03614c5a57806040517fd78bce0c000000000000000000000000000000000000000000000000000000008152600401614c519190615741565b60405180910390fd5b5b5050565b5f614c68612f78565b5f0160089054906101000a900460ff16905090565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b03614ce157806040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401614cd8919061591e565b60405180910390fd5b80614d0d7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b614c7d565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f808473ffffffffffffffffffffffffffffffffffffffff1684604051614d78919061723d565b5f60405180830381855af49150503d805f8114614db0576040519150601f19603f3d011682016040523d82523d5f602084013e614db5565b606091505b5091509150614dc5858383614fc9565b9250505092915050565b5f341115614e09576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f807f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f614e36615056565b614e3e6150cc565b8630604051602001614e549594939291906178c0565b604051602081830303815290604052805190602001209050614e768184614a68565b91505092915050565b5f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f614ea9615056565b614eb16150cc565b4630604051602001614ec79594939291906178c0565b60405160208183030381529060405280519060200120905090565b5f805f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c1115614f1e575f600385925092509250614fbf565b5f6001888888886040515f8152602001604052604051614f419493929190617911565b6020604051602081039080840390855afa158015614f61573d5f803e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603614fb2575f60015f801b93509350935050614fbf565b805f805f1b935093509350505b9450945094915050565b606082614fde57614fd982615143565b61504e565b5f825114801561500457505f8473ffffffffffffffffffffffffffffffffffffffff163b145b1561504657836040517f9996b31500000000000000000000000000000000000000000000000000000000815260040161503d919061591e565b60405180910390fd5b81905061504f565b5b9392505050565b5f80615060613697565b90505f61506b6136be565b90505f81511115615087578080519060200120925050506150c9565b5f825f015490505f801b81146150a2578093505050506150c9565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f806150d6613697565b90505f6150e161375c565b90505f815111156150fd57808051906020012092505050615140565b5f826001015490505f801b811461511957809350505050615140565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f815111156151555780518082602001fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b828054828255905f5260205f209081019282156151c1579160200282015b828111156151c05782518255916020019190600101906151a5565b5b5090506151ce919061521d565b5090565b828054828255905f5260205f2090810192821561520c579160200282015b8281111561520b5782358255916020019190600101906151f0565b5b509050615219919061521d565b5090565b5b80821115615234575f815f90555060010161521e565b5090565b5f604051905090565b5f80fd5b5f80fd5b5f819050919050565b61525b81615249565b8114615265575f80fd5b50565b5f8135905061527681615252565b92915050565b5f80fd5b5f80fd5b5f80fd5b5f8083601f84011261529d5761529c61527c565b5b8235905067ffffffffffffffff8111156152ba576152b9615280565b5b6020830191508360018202830111156152d6576152d5615284565b5b9250929050565b5f805f805f805f6080888a0312156152f8576152f7615241565b5b5f6153058a828b01615268565b975050602088013567ffffffffffffffff81111561532657615325615245565b5b6153328a828b01615288565b9650965050604088013567ffffffffffffffff81111561535557615354615245565b5b6153618a828b01615288565b9450945050606088013567ffffffffffffffff81111561538457615383615245565b5b6153908a828b01615288565b925092505092959891949750929550565b5f602082840312156153b6576153b5615241565b5b5f6153c384828501615268565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61541e826153f5565b9050919050565b61542e81615414565b82525050565b5f61543f8383615425565b60208301905092915050565b5f602082019050919050565b5f615461826153cc565b61546b81856153d6565b9350615476836153e6565b805f5b838110156154a657815161548d8882615434565b97506154988361544b565b925050600181019050615479565b5085935050505092915050565b5f6020820190508181035f8301526154cb8184615457565b905092915050565b5f81519050919050565b5f82825260208201905092915050565b5f5b8381101561550a5780820151818401526020810190506154ef565b5f8484015250505050565b5f601f19601f8301169050919050565b5f61552f826154d3565b61553981856154dd565b93506155498185602086016154ed565b61555281615515565b840191505092915050565b5f6020820190508181035f8301526155758184615525565b905092915050565b61558681615414565b8114615590575f80fd5b50565b5f813590506155a18161557d565b92915050565b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6155e182615515565b810181811067ffffffffffffffff82111715615600576155ff6155ab565b5b80604052505050565b5f615612615238565b905061561e82826155d8565b919050565b5f67ffffffffffffffff82111561563d5761563c6155ab565b5b61564682615515565b9050602081019050919050565b828183375f83830152505050565b5f61567361566e84615623565b615609565b90508281526020810184848401111561568f5761568e6155a7565b5b61569a848285615653565b509392505050565b5f82601f8301126156b6576156b561527c565b5b81356156c6848260208601615661565b91505092915050565b5f80604083850312156156e5576156e4615241565b5b5f6156f285828601615593565b925050602083013567ffffffffffffffff81111561571357615712615245565b5b61571f858286016156a2565b9150509250929050565b5f819050919050565b61573b81615729565b82525050565b5f6020820190506157545f830184615732565b92915050565b5f8115159050919050565b61576e8161575a565b82525050565b5f6020820190506157875f830184615765565b92915050565b5f7fff0000000000000000000000000000000000000000000000000000000000000082169050919050565b6157c18161578d565b82525050565b6157d081615249565b82525050565b6157df81615414565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b61581781615249565b82525050565b5f615828838361580e565b60208301905092915050565b5f602082019050919050565b5f61584a826157e5565b61585481856157ef565b935061585f836157ff565b805f5b8381101561588f578151615876888261581d565b975061588183615834565b925050600181019050615862565b5085935050505092915050565b5f60e0820190506158af5f83018a6157b8565b81810360208301526158c18189615525565b905081810360408301526158d58188615525565b90506158e460608301876157c7565b6158f160808301866157d6565b6158fe60a0830185615732565b81810360c08301526159108184615840565b905098975050505050505050565b5f6020820190506159315f8301846157d6565b92915050565b5f80fd5b5f604082840312156159505761594f615937565b5b81905092915050565b5f8083601f84011261596e5761596d61527c565b5b8235905067ffffffffffffffff81111561598b5761598a615280565b5b6020830191508360408202830111156159a7576159a6615284565b5b9250929050565b5f8083601f8401126159c3576159c261527c565b5b8235905067ffffffffffffffff8111156159e0576159df615280565b5b6020830191508360208202830111156159fc576159fb615284565b5b9250929050565b5f805f805f805f8060c0898b031215615a1f57615a1e615241565b5b5f615a2c8b828c01615268565b9850506020615a3d8b828c0161593b565b975050606089013567ffffffffffffffff811115615a5e57615a5d615245565b5b615a6a8b828c01615959565b9650965050608089013567ffffffffffffffff811115615a8d57615a8c615245565b5b615a998b828c016159ae565b945094505060a089013567ffffffffffffffff811115615abc57615abb615245565b5b615ac88b828c01615288565b92509250509295985092959890939650565b5f60408284031215615aef57615aee615937565b5b81905092915050565b5f60408284031215615b0d57615b0c615937565b5b81905092915050565b5f805f805f805f805f805f6101208c8e031215615b3657615b35615241565b5b5f8c013567ffffffffffffffff811115615b5357615b52615245565b5b615b5f8e828f01615959565b9b509b50506020615b728e828f01615ada565b9950506060615b838e828f0161593b565b98505060a08c013567ffffffffffffffff811115615ba457615ba3615245565b5b615bb08e828f01615af8565b97505060c08c013567ffffffffffffffff811115615bd157615bd0615245565b5b615bdd8e828f01615288565b965096505060e08c013567ffffffffffffffff811115615c0057615bff615245565b5b615c0c8e828f01615288565b94509450506101008c013567ffffffffffffffff811115615c3057615c2f615245565b5b615c3c8e828f01615288565b92509250509295989b509295989b9093969950565b5f8083601f840112615c6657615c6561527c565b5b8235905067ffffffffffffffff811115615c8357615c82615280565b5b602083019150836020820283011115615c9f57615c9e615284565b5b9250929050565b5f805f8060408587031215615cbe57615cbd615241565b5b5f85013567ffffffffffffffff811115615cdb57615cda615245565b5b615ce787828801615c51565b9450945050602085013567ffffffffffffffff811115615d0a57615d09615245565b5b615d1687828801615288565b925092505092959194509250565b5f805f805f805f805f805f6101008c8e031215615d4457615d43615241565b5b5f8c013567ffffffffffffffff811115615d6157615d60615245565b5b615d6d8e828f01615959565b9b509b50506020615d808e828f01615ada565b99505060608c013567ffffffffffffffff811115615da157615da0615245565b5b615dad8e828f01615af8565b9850506080615dbe8e828f01615593565b97505060a08c013567ffffffffffffffff811115615ddf57615dde615245565b5b615deb8e828f01615288565b965096505060c08c013567ffffffffffffffff811115615e0e57615e0d615245565b5b615e1a8e828f01615288565b945094505060e08c013567ffffffffffffffff811115615e3d57615e3c615245565b5b615e498e828f01615288565b92509250509295989b509295989b9093969950565b5f60208284031215615e7357615e72615241565b5b5f615e8084828501615593565b91505092915050565b5f805f805f60608688031215615ea257615ea1615241565b5b5f615eaf88828901615593565b955050602086013567ffffffffffffffff811115615ed057615ecf615245565b5b615edc88828901615959565b9450945050604086013567ffffffffffffffff811115615eff57615efe615245565b5b615f0b88828901615288565b92509250509295509295909350565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b5f602082019050615f5a5f8301846157c7565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f6002820490506001821680615fa457607f821691505b602082108103615fb757615fb6615f60565b5b50919050565b5f82905092915050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f600883026160237fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82615fe8565b61602d8683615fe8565b95508019841693508086168417925050509392505050565b5f819050919050565b5f61606861606361605e84615249565b616045565b615249565b9050919050565b5f819050919050565b6160818361604e565b61609561608d8261606f565b848454615ff4565b825550505050565b5f90565b6160a961609d565b6160b4818484616078565b505050565b5b818110156160d7576160cc5f826160a1565b6001810190506160ba565b5050565b601f82111561611c576160ed81615fc7565b6160f684615fd9565b81016020851015616105578190505b61611961611185615fd9565b8301826160b9565b50505b505050565b5f82821c905092915050565b5f61613c5f1984600802616121565b1980831691505092915050565b5f616154838361612d565b9150826002028217905092915050565b61616e8383615fbd565b67ffffffffffffffff811115616187576161866155ab565b5b6161918254615f8d565b61619c8282856160db565b5f601f8311600181146161c9575f84156161b7578287013590505b6161c18582616149565b865550616228565b601f1984166161d786615fc7565b5f5b828110156161fe578489013582556001820191506020850194506020810190506161d9565b8683101561621b5784890135616217601f89168261612d565b8355505b6001600288020188555050505b50505050505050565b5f81549050919050565b5f82825260208201905092915050565b5f819050815f5260205f209050919050565b5f82825260208201905092915050565b5f815461627981615f8d565b616283818661625d565b9450600182165f811461629d57600181146162b3576162e5565b60ff1983168652811515602002860193506162e5565b6162bc85615fc7565b5f5b838110156162dd578154818901526001820191506020810190506162be565b808801955050505b50505092915050565b5f6162f9838361626d565b905092915050565b5f600182019050919050565b5f61631782616231565b616321818561623b565b9350836020820285016163338561624b565b805f5b8581101561636d5784840389528161634e85826162ee565b945061635983616301565b925060208a01995050600181019050616336565b50829750879550505050505092915050565b5f81549050919050565b5f819050815f5260205f209050919050565b5f600182019050919050565b5f6163b18261637f565b6163bb818561623b565b9350836020820285016163cd85616389565b805f5b85811015616407578484038952816163e885826162ee565b94506163f38361639b565b925060208a019950506001810190506163d0565b50829750879550505050505092915050565b5f82825260208201905092915050565b5f6164348385616419565b9350616441838584615653565b61644a83615515565b840190509392505050565b5f6060820190508181035f83015261646d818761630d565b9050818103602083015261648181866163a7565b90508181036040830152616496818486616429565b905095945050505050565b5f81905092915050565b5f6164b5826154d3565b6164bf81856164a1565b93506164cf8185602086016154ed565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f61650f6002836164a1565b915061651a826164db565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f6165596001836164a1565b915061656482616525565b600182019050919050565b5f61657a82876164ab565b915061658582616503565b915061659182866164ab565b915061659c8261654d565b91506165a882856164ab565b91506165b38261654d565b91506165bf82846164ab565b915081905095945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61660482615249565b915061660f83615249565b9250828201905080821115616627576166266165cd565b5b92915050565b5f67ffffffffffffffff82169050919050565b6166498161662d565b82525050565b5f6020820190506166625f830184616640565b92915050565b5f815190506166768161557d565b92915050565b5f6020828403121561669157616690615241565b5b5f61669e84828501616668565b91505092915050565b5f6060820190508181035f8301526166c0818789616429565b905081810360208301526166d481866163a7565b905081810360408301526166e9818486616429565b90509695505050505050565b7f4549503731323a20556e696e697469616c697a656400000000000000000000005f82015250565b5f6167296015836154dd565b9150616734826166f5565b602082019050919050565b5f6020820190508181035f8301526167568161671d565b9050919050565b5f61676b6020840184615593565b905092915050565b604082016167835f83018361675d565b61678f5f850182615425565b5061679d602083018361675d565b6167aa6020850182615425565b50505050565b5f819050919050565b5f602082019050919050565b5f6167d083856153d6565b93506167db826167b0565b805f5b85811015616813576167f0828461675d565b6167fa8882615434565b9750616805836167b9565b9250506001810190506167de565b5085925050509392505050565b5f6080820190506168335f8301876157c7565b6168406020830186616773565b81810360608301526168538184866167c5565b905095945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f60408201905061689e5f830185615732565b6168ab60208301846157d6565b9392505050565b5f80fd5b5f80fd5b5f80fd5b5f80833560016020038436030381126168da576168d96168b2565b5b80840192508235915067ffffffffffffffff8211156168fc576168fb6168b6565b5b602083019250602082023603831315616918576169176168ba565b5b509250929050565b5f60ff82169050919050565b61693581616920565b82525050565b5f60408201905061694e5f83018561692c565b61695b60208301846157c7565b9392505050565b5f80fd5b5f80fd5b5f6040828403121561697f5761697e616962565b5b6169896040615609565b90505f61699884828501615268565b5f8301525060206169ab84828501615268565b60208301525092915050565b5f604082840312156169cc576169cb615241565b5b5f6169d98482850161696a565b91505092915050565b5f6040820190506169f55f8301866157d6565b8181036020830152616a088184866167c5565b9050949350505050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b616a4481615729565b82525050565b5f616a558383616a3b565b60208301905092915050565b5f602082019050919050565b5f616a7782616a12565b616a818185616a1c565b9350616a8c83616a2c565b805f5b83811015616abc578151616aa38882616a4a565b9750616aae83616a61565b925050600181019050616a8f565b5085935050505092915050565b5f6020820190508181035f830152616ae18184616a6d565b905092915050565b5f67ffffffffffffffff821115616b0357616b026155ab565b5b602082029050602081019050919050565b616b1d81615729565b8114616b27575f80fd5b50565b5f81519050616b3881616b14565b92915050565b5f81519050616b4c81615252565b92915050565b5f67ffffffffffffffff821115616b6c57616b6b6155ab565b5b602082029050602081019050919050565b5f616b8f616b8a84616b52565b615609565b90508083825260208201905060208402830185811115616bb257616bb1615284565b5b835b81811015616bdb5780616bc78882616668565b845260208401935050602081019050616bb4565b5050509392505050565b5f82601f830112616bf957616bf861527c565b5b8151616c09848260208601616b7d565b91505092915050565b5f60808284031215616c2757616c26616962565b5b616c316080615609565b90505f616c4084828501616b2a565b5f830152506020616c5384828501616b3e565b6020830152506040616c6784828501616b2a565b604083015250606082015167ffffffffffffffff811115616c8b57616c8a616966565b5b616c9784828501616be5565b60608301525092915050565b5f616cb5616cb084616ae9565b615609565b90508083825260208201905060208402830185811115616cd857616cd7615284565b5b835b81811015616d1f57805167ffffffffffffffff811115616cfd57616cfc61527c565b5b808601616d0a8982616c12565b85526020850194505050602081019050616cda565b5050509392505050565b5f82601f830112616d3d57616d3c61527c565b5b8151616d4d848260208601616ca3565b91505092915050565b5f60208284031215616d6b57616d6a615241565b5b5f82015167ffffffffffffffff811115616d8857616d87615245565b5b616d9484828501616d29565b91505092915050565b5f616da782615249565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203616dd957616dd86165cd565b5b600182019050919050565b5f81519050919050565b616df782616de4565b67ffffffffffffffff811115616e1057616e0f6155ab565b5b616e1a8254615f8d565b616e258282856160db565b5f60209050601f831160018114616e56575f8415616e44578287015190505b616e4e8582616149565b865550616eb5565b601f198416616e6486615fc7565b5f5b82811015616e8b57848901518255600182019150602085019450602081019050616e66565b86831015616ea85784890151616ea4601f89168261612d565b8355505b6001600288020188555050505b505050505050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f82825260208201905092915050565b5f616f00826153cc565b616f0a8185616ee6565b9350616f15836153e6565b805f5b83811015616f45578151616f2c8882615434565b9750616f378361544b565b925050600181019050616f18565b5085935050505092915050565b5f608083015f830151616f675f860182616a3b565b506020830151616f7a602086018261580e565b506040830151616f8d6040860182616a3b565b5060608301518482036060860152616fa58282616ef6565b9150508091505092915050565b5f616fbd8383616f52565b905092915050565b5f602082019050919050565b5f616fdb82616ebd565b616fe58185616ec7565b935083602082028501616ff785616ed7565b805f5b8581101561703257848403895281516170138582616fb2565b945061701e83616fc5565b925060208a01995050600181019050616ffa565b50829750879550505050505092915050565b5f6080820190508181035f83015261705c8189616fd1565b905061706b60208301886157d6565b818103604083015261707e818688616429565b90508181036060830152617093818486616429565b9050979650505050505050565b5f80fd5b82818337505050565b5f6170b88385616a1c565b93507f07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8311156170eb576170ea6170a0565b5b6020830292506170fc8385846170a4565b82840190509392505050565b5f6020820190508181035f8301526171218184866170ad565b90509392505050565b5f6040820190508181035f8301526171428186616fd1565b90508181036020830152617157818486616429565b9050949350505050565b5f81905092915050565b61717481615729565b82525050565b5f617185838361716b565b60208301905092915050565b5f61719b82616a12565b6171a58185617161565b93506171b083616a2c565b805f5b838110156171e05781516171c7888261717a565b97506171d283616a61565b9250506001810190506171b3565b5085935050505092915050565b5f6171f88284617191565b915081905092915050565b5f81905092915050565b5f61721782616de4565b6172218185617203565b93506172318185602086016154ed565b80840191505092915050565b5f617248828461720d565b915081905092915050565b5f60a0820190506172665f830188615732565b6172736020830187615732565b6172806040830186615732565b61728d6060830185615732565b61729a6080830184615732565b9695505050505050565b5f6040820190506172b75f8301856157c7565b6172c460208301846157d6565b9392505050565b5f602082840312156172e0576172df615241565b5b5f6172ed84828501616b3e565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f6020828403121561733857617337615241565b5b5f61734584828501616b2a565b91505092915050565b5f6080820190506173615f830187615732565b61736e6020830186615732565b61737b6040830185615732565b6173886060830184615732565b95945050505050565b5f61ffff82169050919050565b5f6173b86173b36173ae84617391565b616045565b615249565b9050919050565b6173c88161739e565b82525050565b5f6040820190506173e15f8301856173bf565b6173ee60208301846157c7565b9392505050565b5f6040820190506174085f8301856157c7565b61741560208301846157c7565b9392505050565b5f61742682615249565b915061743183615249565b925082820261743f81615249565b91508282048414831517617456576174556165cd565b5b5092915050565b604082015f8201516174715f85018261580e565b506020820151617484602085018261580e565b50505050565b5f60608201905061749d5f8301856157c7565b6174aa602083018461745d565b9392505050565b5f6040820190506174c45f8301856157d6565b81810360208301526174d68184615457565b90509392505050565b5f6020820190508181035f8301526174f8818486616429565b90509392505050565b5f608083015f8301516175165f860182616a3b565b506020830151617529602086018261580e565b50604083015161753c6040860182616a3b565b50606083015184820360608601526175548282616ef6565b9150508091505092915050565b5f6040820190508181035f8301526175798185617501565b9050818103602083015261758d8184617501565b90509392505050565b5f819050815f5260205f209050919050565b601f8211156175e9576175ba81617596565b6175c384615fd9565b810160208510156175d2578190505b6175e66175de85615fd9565b8301826160b9565b50505b505050565b6175f7826154d3565b67ffffffffffffffff8111156176105761760f6155ab565b5b61761a8254615f8d565b6176258282856175a8565b5f60209050601f831160018114617656575f8415617644578287015190505b61764e8582616149565b8655506176b5565b601f19841661766486617596565b5f5b8281101561768b57848901518255600182019150602085019450602081019050617666565b868310156176a857848901516176a4601f89168261612d565b8355505b6001600288020188555050505b505050505050565b5f6020820190506176d05f83018461692c565b92915050565b605481106176e7576176e6615f1a565b5b50565b5f8190506176f7826176d6565b919050565b5f617706826176ea565b9050919050565b617716816176fc565b82525050565b5f60208201905061772f5f83018461770d565b92915050565b5f81905092915050565b61774881615414565b82525050565b5f617759838361773f565b60208301905092915050565b5f61776f826153cc565b6177798185617735565b9350617784836153e6565b805f5b838110156177b457815161779b888261774e565b97506177a68361544b565b925050600181019050617787565b5085935050505092915050565b5f6177cc8284617765565b915081905092915050565b5f610100820190506177eb5f83018b615732565b6177f8602083018a615732565b6178056040830189615732565b61781260608301886157d6565b61781f60808301876157c7565b61782c60a08301866157c7565b61783960c08301856157c7565b61784660e0830184615732565b9998505050505050505050565b5f60e0820190506178665f83018a615732565b6178736020830189615732565b6178806040830188615732565b61788d60608301876157c7565b61789a60808301866157c7565b6178a760a08301856157c7565b6178b460c0830184615732565b98975050505050505050565b5f60a0820190506178d35f830188615732565b6178e06020830187615732565b6178ed6040830186615732565b6178fa60608301856157c7565b61790760808301846157d6565b9695505050505050565b5f6080820190506179245f830187615732565b617931602083018661692c565b61793e6040830185615732565b61794b6060830184615732565b9594505050505056fe5573657244656372797074526573706f6e7365566572696669636174696f6e286279746573207075626c69634b65792c627974657333325b5d20637448616e646c65732c6279746573207573657244656372797074656453686172652c627974657320657874726144617461295075626c696344656372797074566572696669636174696f6e28627974657333325b5d20637448616e646c65732c627974657320646563727970746564526573756c742c62797465732065787472614461746129557365724465637279707452657175657374566572696669636174696f6e286279746573207075626c69634b65792c616464726573735b5d20636f6e74726163744164647265737365732c75696e7432353620636f6e747261637473436861696e49642c75696e7432353620737461727454696d657374616d702c75696e74323536206475726174696f6e446179732c6279746573206578747261446174612944656c656761746564557365724465637279707452657175657374566572696669636174696f6e286279746573207075626c69634b65792c616464726573735b5d20636f6e74726163744164647265737365732c616464726573732064656c656761746f72416464726573732c75696e7432353620636f6e747261637473436861696e49642c75696e7432353620737461727454696d657374616d702c75696e74323536206475726174696f6e446179732c62797465732065787472614461746129
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP4\x80\x15b\0\0CW_\x80\xFD[Pb\0\0Tb\0\0Z` \x1B` \x1CV[b\0\x01\xC4V[_b\0\0kb\0\x01^` \x1B` \x1CV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15b\0\0\xB6W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14b\0\x01[Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Qb\0\x01R\x91\x90b\0\x01\xA9V[`@Q\x80\x91\x03\x90\xA1[PV[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[b\0\x01\xA3\x81b\0\x01\x85V[\x82RPPV[_` \x82\x01\x90Pb\0\x01\xBE_\x83\x01\x84b\0\x01\x98V[\x92\x91PPV[`\x80Qa{xb\0\x01\xEB_9_\x81\x81a0o\x01R\x81\x81a0\xC4\x01Ra3f\x01Ra{x_\xF3\xFE`\x80`@R`\x046\x10a\x01eW_5`\xE0\x1C\x80c\x84V\xCBY\x11a\0\xD0W\x80c\xAD<\xB1\xCC\x11a\0\x89W\x80c\xE3\x1E\xC1\x9D\x11a\0cW\x80c\xE3\x1E\xC1\x9D\x14a\x04aW\x80c\xF1\xB5z\xDB\x14a\x04\x89W\x80c\xF2\xFD\xE3\x8B\x14a\x04\xB1W\x80c\xF4\xA6\xE2\xCE\x14a\x04\xD9Wa\x01eV[\x80c\xAD<\xB1\xCC\x14a\x03\xE5W\x80c\xD8\x99\x8FE\x14a\x04\x0FW\x80c\xE3\x0C9x\x14a\x047Wa\x01eV[\x80c\x84V\xCBY\x14a\x02\xFDW\x80c\x84\xB0\x19n\x14a\x03\x13W\x80c\x8D\xA5\xCB[\x14a\x03CW\x80c\x9F\x98\xD8\x99\x14a\x03mW\x80c\x9F\xADZ/\x14a\x03\x95W\x80c\xA6\t\x049\x14a\x03\xBDWa\x01eV[\x80cO\x1E\xF2\x86\x11a\x01\"W\x80cO\x1E\xF2\x86\x14a\x029W\x80cR\xD1\x90-\x14a\x02UW\x80c\\\x97Z\xBB\x14a\x02\x7FW\x80co\x89\x13\xBC\x14a\x02\xA9W\x80cqP\x18\xA6\x14a\x02\xD1W\x80cy\xBAP\x97\x14a\x02\xE7Wa\x01eV[\x80c\x04o\x9E\xB3\x14a\x01iW\x80c\t\0\xCCi\x14a\x01\x91W\x80c\r\x8En,\x14a\x01\xCDW\x80c\x12:\xBB(\x14a\x01\xF7W\x80c9\xF78\x10\x14a\x02\rW\x80c?K\xA8:\x14a\x02#W[_\x80\xFD[4\x80\x15a\x01tW_\x80\xFD[Pa\x01\x8F`\x04\x806\x03\x81\x01\x90a\x01\x8A\x91\x90aR\xDDV[a\x05\x01V[\0[4\x80\x15a\x01\x9CW_\x80\xFD[Pa\x01\xB7`\x04\x806\x03\x81\x01\x90a\x01\xB2\x91\x90aS\xA1V[a\t\xBAV[`@Qa\x01\xC4\x91\x90aT\xB3V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xD8W_\x80\xFD[Pa\x01\xE1a\n\x8BV[`@Qa\x01\xEE\x91\x90aU]V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x02W_\x80\xFD[Pa\x02\x0Ba\x0B\x06V[\0[4\x80\x15a\x02\x18W_\x80\xFD[Pa\x02!a\x0C\x93V[\0[4\x80\x15a\x02.W_\x80\xFD[Pa\x027a\x0E\xDBV[\0[a\x02S`\x04\x806\x03\x81\x01\x90a\x02N\x91\x90aV\xCFV[a\x10#V[\0[4\x80\x15a\x02`W_\x80\xFD[Pa\x02ia\x10BV[`@Qa\x02v\x91\x90aWAV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x8AW_\x80\xFD[Pa\x02\x93a\x10sV[`@Qa\x02\xA0\x91\x90aWtV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xB4W_\x80\xFD[Pa\x02\xCF`\x04\x806\x03\x81\x01\x90a\x02\xCA\x91\x90aR\xDDV[a\x10\x95V[\0[4\x80\x15a\x02\xDCW_\x80\xFD[Pa\x02\xE5a\x14bV[\0[4\x80\x15a\x02\xF2W_\x80\xFD[Pa\x02\xFBa\x14uV[\0[4\x80\x15a\x03\x08W_\x80\xFD[Pa\x03\x11a\x15\x03V[\0[4\x80\x15a\x03\x1EW_\x80\xFD[Pa\x03'a\x16KV[`@Qa\x03:\x97\x96\x95\x94\x93\x92\x91\x90aX\x9CV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03NW_\x80\xFD[Pa\x03Wa\x17TV[`@Qa\x03d\x91\x90aY\x1EV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03xW_\x80\xFD[Pa\x03\x93`\x04\x806\x03\x81\x01\x90a\x03\x8E\x91\x90aZ\x03V[a\x17\x89V[\0[4\x80\x15a\x03\xA0W_\x80\xFD[Pa\x03\xBB`\x04\x806\x03\x81\x01\x90a\x03\xB6\x91\x90a[\x16V[a\x1A(V[\0[4\x80\x15a\x03\xC8W_\x80\xFD[Pa\x03\xE3`\x04\x806\x03\x81\x01\x90a\x03\xDE\x91\x90aS\xA1V[a\x1F\xF2V[\0[4\x80\x15a\x03\xF0W_\x80\xFD[Pa\x03\xF9a bV[`@Qa\x04\x06\x91\x90aU]V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\x1AW_\x80\xFD[Pa\x045`\x04\x806\x03\x81\x01\x90a\x040\x91\x90a\\\xA6V[a \x9BV[\0[4\x80\x15a\x04BW_\x80\xFD[Pa\x04Ka\"YV[`@Qa\x04X\x91\x90aY\x1EV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04lW_\x80\xFD[Pa\x04\x87`\x04\x806\x03\x81\x01\x90a\x04\x82\x91\x90a\\\xA6V[a\"\x8EV[\0[4\x80\x15a\x04\x94W_\x80\xFD[Pa\x04\xAF`\x04\x806\x03\x81\x01\x90a\x04\xAA\x91\x90a]$V[a#\xD3V[\0[4\x80\x15a\x04\xBCW_\x80\xFD[Pa\x04\xD7`\x04\x806\x03\x81\x01\x90a\x04\xD2\x91\x90a^^V[a(\x89V[\0[4\x80\x15a\x04\xE4W_\x80\xFD[Pa\x04\xFF`\x04\x806\x03\x81\x01\x90a\x04\xFA\x91\x90a^\x89V[a)BV[\0[s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC6'RX3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05N\x91\x90aY\x1EV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x05dW_\x80\xFD[PZ\xFA\x15\x80\x15a\x05vW=_\x80>=_\xFD[PPPP_a\x05\x83a+NV[\x90P_\x88\x14\x80a\x05\xB8WP\x80_\x01T\x88\x11\x80\x15a\x05\xB7WP`\xF8`\x02`\x05\x81\x11\x15a\x05\xB1Wa\x05\xB0a_\x1AV[[\x90\x1B\x88\x11\x15[[\x80a\x05\xC6WP\x80`\x0C\x01T\x88\x11[\x15a\x06\x08W\x87`@Q\x7F\xD4\x8A\xF9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05\xFF\x91\x90a_GV[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x8A\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80Ta\x066\x90a_\x8DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06b\x90a_\x8DV[\x80\x15a\x06\xADW\x80`\x1F\x10a\x06\x84Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xADV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x90W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07\x03W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x06\xEFW[PPPPP\x81RPP\x90P_`@Q\x80`\x80\x01`@R\x80\x83_\x01Q\x81R` \x01\x83` \x01Q\x81R` \x01\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81RP\x90P_a\x07\xC9\x82a+uV[\x90Pa\x07\xD7\x8B\x82\x8A\x8Aa,<V[_\x84`\x05\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x90P\x80\x89\x89\x90\x91\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x92\x90\x91\x92\x90\x91\x92\x90\x91\x92P\x91\x82a\x08&\x92\x91\x90aadV[P\x84`\x08\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x8B\x8B\x90\x91\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x92\x90\x91\x92\x90\x91\x92\x90\x91\x92P\x91\x82a\x08r\x92\x91\x90aadV[P\x84`\t\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _\x80_\x1B\x81R` \x01\x90\x81R` \x01_ 3\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x84`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\t+WPa\t*\x81\x80T\x90Pa.\x1DV[[\x15a\t\xACW`\x01\x85`\x01\x01_\x8E\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x8B\x7F\x1C\xBF\x1Cn\x9E\xA3\xDB\xCF\xD4\x9B\x96\x90\xD9<\xD4\xB9;<\xB2)\x13\xF9\x9Bq\xD3\xAE\xCA\xE6\x0B%\xFCV\x86`\x08\x01_\x8F\x81R` \x01\x90\x81R` \x01_ \x83\x8A\x8A`@Qa\t\xA3\x94\x93\x92\x91\x90adUV[`@Q\x80\x91\x03\x90\xA2[PPPPPPPPPPPPV[``_a\t\xC5a+NV[\x90P_\x81`\n\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\t\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x82\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\n}W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\n4W[PPPPP\x92PPP\x91\x90PV[```@Q\x80`@\x01`@R\x80`\n\x81R` \x01\x7FDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\n\xCC_a.\xAEV[a\n\xD6`\x04a.\xAEV[a\n\xDF_a.\xAEV[`@Q` \x01a\n\xF2\x94\x93\x92\x91\x90aeoV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`\x06_a\x0B\x11a/xV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x0BYWP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x0B\x90W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_a\x0B\xDEa+NV[\x90P`\xF8`\x01`\x05\x81\x11\x15a\x0B\xF6Wa\x0B\xF5a_\x1AV[[\x90\x1B\x81_\x01Ta\x0C\x06\x91\x90ae\xFAV[\x81`\x0B\x01\x81\x90UP`\xF8`\x02`\x05\x81\x11\x15a\x0C$Wa\x0C#a_\x1AV[[\x90\x1B\x81_\x01Ta\x0C4\x91\x90ae\xFAV[\x81`\x0C\x01\x81\x90UPP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x0C\x87\x91\x90afOV[`@Q\x80\x91\x03\x90\xA1PPV[`\x01a\x0C\x9Da/\x9FV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0C\xDEW`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x06_a\x0C\xE9a/xV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\r1WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\rhW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x0E!`@Q\x80`@\x01`@R\x80`\n\x81R` \x01\x7FDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa/\xC3V[a\x0E1a\x0E,a\x17TV[a/\xD9V[a\x0E9a/\xEDV[_a\x0EBa+NV[\x90P`\xF8`\x01`\x05\x81\x11\x15a\x0EZWa\x0EYa_\x1AV[[\x90\x1B\x81`\x0B\x01\x81\x90UP`\xF8`\x02`\x05\x81\x11\x15a\x0EzWa\x0Eya_\x1AV[[\x90\x1B\x81`\x0C\x01\x81\x90UPP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x0E\xCF\x91\x90afOV[`@Q\x80\x91\x03\x90\xA1PPV[s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F8W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\\\x91\x90af|V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\x0F\xD7WPs\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a\x10\x19W3`@Q\x7F\xE1\x91f\xEE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\x10\x91\x90aY\x1EV[`@Q\x80\x91\x03\x90\xFD[a\x10!a/\xFFV[V[a\x10+a0mV[a\x104\x82a1SV[a\x10>\x82\x82a2FV[PPV[_a\x10Ka3dV[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[_\x80a\x10}a3\xEBV[\x90P\x80_\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x90V[s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC6'RX3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\xE2\x91\x90aY\x1EV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x10\xF8W_\x80\xFD[PZ\xFA\x15\x80\x15a\x11\nW=_\x80>=_\xFD[PPPP_a\x11\x17a+NV[\x90P_\x88\x14\x80a\x11LWP\x80_\x01T\x88\x11\x80\x15a\x11KWP`\xF8`\x01`\x05\x81\x11\x15a\x11EWa\x11Da_\x1AV[[\x90\x1B\x88\x11\x15[[\x80a\x11ZWP\x80`\x0B\x01T\x88\x11[\x15a\x11\x9CW\x87`@Q\x7F\xD4\x8A\xF9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\x93\x91\x90a_GV[`@Q\x80\x91\x03\x90\xFD[_`@Q\x80``\x01`@R\x80\x83`\x04\x01_\x8C\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x12\x03W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x11\xEFW[PPPPP\x81R` \x01\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81RP\x90P_a\x12\xA9\x82a4\x12V[\x90Pa\x12\xB7\x8A\x82\x89\x89a,<V[_\x83`\x03\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x90P\x80\x88\x88\x90\x91\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x92\x90\x91\x92\x90\x91\x92\x90\x91\x92P\x91\x82a\x13\x15\x92\x91\x90aadV[P\x83`\t\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ 3\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x83`\x01\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x13\xCCWPa\x13\xCB\x81\x80T\x90Pa4\xCCV[[\x15a\x14UW`\x01\x84`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x81\x84`\n\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x8A\x7F\xD7\xE5\x8A6z\nl)\x8Ev\xAD]$\0\x04\xE3'\xAA\x14#\xCB\xE4\xBD\x7F\xF8]Lq^\xF8\xD1_\x8B\x8B\x84\x8A\x8A`@Qa\x14L\x95\x94\x93\x92\x91\x90af\xA7V[`@Q\x80\x91\x03\x90\xA2[PPPPPPPPPPPV[a\x14ja5]V[a\x14s_a5\xE4V[V[_a\x14~a6!V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x14\x9Fa\"YV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x14\xF7W\x80`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14\xEE\x91\x90aY\x1EV[`@Q\x80\x91\x03\x90\xFD[a\x15\0\x81a5\xE4V[PV[s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cp\x08\xB5H`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15`W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x84\x91\x90af|V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\x15\xFFWPs\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a\x16AW3`@Q\x7F8\x89\x16\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x168\x91\x90aY\x1EV[`@Q\x80\x91\x03\x90\xFD[a\x16Ia6(V[V[_``\x80_\x80_``_a\x16]a6\x97V[\x90P_\x80\x1B\x81_\x01T\x14\x80\x15a\x16xWP_\x80\x1B\x81`\x01\x01T\x14[a\x16\xB7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16\xAE\x90ag?V[`@Q\x80\x91\x03\x90\xFD[a\x16\xBFa6\xBEV[a\x16\xC7a7\\V[F0_\x80\x1B_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\xE6Wa\x16\xE5aU\xABV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\x14W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94\x93\x92\x91\x90\x97P\x97P\x97P\x97P\x97P\x97P\x97PP\x90\x91\x92\x93\x94\x95\x96V[_\x80a\x17^a7\xFAV[\x90P\x80_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91PP\x90V[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cQ\xC4\x1D\x0E\x89\x89\x87\x87`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\xDC\x94\x93\x92\x91\x90ah V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x17\xF2W_\x80\xFD[PZ\xFA\x15\x80\x15a\x18\x04W=_\x80>=_\xFD[PPPP_[\x86\x86\x90P\x81\x10\x15a\x1A\x1DWs\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c;\xCEI\x8D\x88\x88\x84\x81\x81\x10a\x18XWa\x18Wah^V[[\x90P`@\x02\x01_\x015\x8A_\x01` \x81\x01\x90a\x18s\x91\x90a^^V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\x90\x92\x91\x90ah\x8BV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x18\xA6W_\x80\xFD[PZ\xFA\x15\x80\x15a\x18\xB8W=_\x80>=_\xFD[PPPPs\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c;\xCEI\x8D\x88\x88\x84\x81\x81\x10a\x18\xFFWa\x18\xFEah^V[[\x90P`@\x02\x01_\x015\x89\x89\x85\x81\x81\x10a\x19\x1BWa\x19\x1Aah^V[[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x193\x91\x90a^^V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19P\x92\x91\x90ah\x8BV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x19fW_\x80\xFD[PZ\xFA\x15\x80\x15a\x19xW=_\x80>=_\xFD[PPPPs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD4Goc\x88\x88\x84\x81\x81\x10a\x19\xBFWa\x19\xBEah^V[[\x90P`@\x02\x01_\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19\xE4\x91\x90aWAV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x19\xFAW_\x80\xFD[PZ\xFA\x15\x80\x15a\x1A\x0CW=_\x80>=_\xFD[PPPP\x80\x80`\x01\x01\x91PPa\x18\nV[PPPPPPPPPV[a\x1A0a8!V[_\x87\x80` \x01\x90a\x1AA\x91\x90ah\xBEV[\x90P\x03a\x1AzW`@Q\x7FW\xCF\xA2\x17\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\n`\xFF\x16\x87\x80` \x01\x90a\x1A\x8F\x91\x90ah\xBEV[\x90P\x11\x15a\x1A\xE8W`\n\x87\x80` \x01\x90a\x1A\xA9\x91\x90ah\xBEV[\x90P`@Q\x7F\xC5\xABF~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1A\xDF\x92\x91\x90ai;V[`@Q\x80\x91\x03\x90\xFD[a\x1B\x01\x89\x806\x03\x81\x01\x90a\x1A\xFC\x91\x90ai\xB7V[a8bV[a\x1Bj\x87\x80` \x01\x90a\x1B\x14\x91\x90ah\xBEV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x89_\x01` \x81\x01\x90a\x1Be\x91\x90a^^V[a9\xADV[\x15a\x1B\xCFW\x87_\x01` \x81\x01\x90a\x1B\x81\x91\x90a^^V[\x87\x80` \x01\x90a\x1B\x91\x91\x90ah\xBEV[`@Q\x7F\xC3Dj\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1B\xC6\x93\x92\x91\x90ai\xE2V[`@Q\x80\x91\x03\x90\xFD[_a\x1C;\x8C\x8C\x8A\x80` \x01\x90a\x1B\xE5\x91\x90ah\xBEV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x8C_\x01` \x81\x01\x90a\x1C6\x91\x90a^^V[a:+V[\x90Ps\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cQ\xC4\x1D\x0E\x89_\x015\x8B\x8B\x80` \x01\x90a\x1C\x82\x91\x90ah\xBEV[`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1C\xA1\x94\x93\x92\x91\x90ah V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1C\xB7W_\x80\xFD[PZ\xFA\x15\x80\x15a\x1C\xC9W=_\x80>=_\xFD[PPPP_`@Q\x80`\xE0\x01`@R\x80\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8A\x80` \x01\x90a\x1D2\x91\x90ah\xBEV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8B_\x01` \x81\x01\x90a\x1D\x88\x91\x90a^^V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x8A_\x015\x81R` \x01\x8C_\x015\x81R` \x01\x8C` \x015\x81R` \x01\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81RP\x90Pa\x1E&\x81\x8B` \x01` \x81\x01\x90a\x1E\x1F\x91\x90a^^V[\x88\x88a=\0V[_s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA1O\x89q\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1Et\x91\x90aj\xC9V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\x8EW=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xB6\x91\x90amVV[\x90Pa\x1E\xC1\x81a=\xD6V[_a\x1E\xCAa+NV[\x90P\x80`\x0C\x01_\x81T\x80\x92\x91\x90a\x1E\xE0\x90am\x9DV[\x91\x90PUP_\x81`\x0C\x01T\x90P`@Q\x80`@\x01`@R\x80\x8C\x8C\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x86\x81RP\x82`\x06\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01\x90\x81a\x1Fk\x91\x90am\xEEV[P` \x82\x01Q\x81`\x01\x01\x90\x80Q\x90` \x01\x90a\x1F\x88\x92\x91\x90aQ\x87V[P\x90PP\x80\x7F\xF9\x01\x1B\xD6\xBA\r\xA6\x04\x9CR\rp\xFEYq\xF1~\xD7\xAByT\x86\x05%D\xB5\x10\x19\x89lYk\x84\x8F` \x01` \x81\x01\x90a\x1F\xC2\x91\x90a^^V[\x8E\x8E\x8C\x8C`@Qa\x1F\xD8\x96\x95\x94\x93\x92\x91\x90apDV[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPPPPPV[_a\x1F\xFBa+NV[\x90P\x80`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a ^W\x81`@Q\x7F\x0B\xF0\x14\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a U\x91\x90a_GV[`@Q\x80\x91\x03\x90\xFD[PPV[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[a \xA3a8!V[_\x84\x84\x90P\x03a \xDFW`@Q\x7F-\xE7T8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a!(\x84\x84\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa>\xBCV[_s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA1O\x89q\x86\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a!x\x92\x91\x90aq\x08V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\x92W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xBA\x91\x90amVV[\x90Pa!\xC5\x81a=\xD6V[_a!\xCEa+NV[\x90P\x80`\x0B\x01_\x81T\x80\x92\x91\x90a!\xE4\x90am\x9DV[\x91\x90PUP_\x81`\x0B\x01T\x90P\x86\x86\x83`\x04\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x91\x90a\"\x13\x92\x91\x90aQ\xD2V[P\x80\x7F\"\xDBH\n9\xBDrUd8\xAA\xDBJ2\xA3\xD2\xA6c\x8B\x87\xC0;\xBE\xC5\xFE\xF6\x99~\x10\x95\x87\xFF\x84\x87\x87`@Qa\"H\x93\x92\x91\x90aq*V[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[_\x80a\"ca?\xE4V[\x90P\x80_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91PP\x90V[_[\x84\x84\x90P\x81\x10\x15a#\xCCWs\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x19??,\x86\x86\x84\x81\x81\x10a\"\xDEWa\"\xDDah^V[[\x90P` \x02\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a#\x01\x91\x90aWAV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a#\x17W_\x80\xFD[PZ\xFA\x15\x80\x15a#)W=_\x80>=_\xFD[PPPPs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD4Goc\x86\x86\x84\x81\x81\x10a#pWa#oah^V[[\x90P` \x02\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a#\x93\x91\x90aWAV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a#\xA9W_\x80\xFD[PZ\xFA\x15\x80\x15a#\xBBW=_\x80>=_\xFD[PPPP\x80\x80`\x01\x01\x91PPa\"\x90V[PPPPPV[a#\xDBa8!V[_\x88\x80` \x01\x90a#\xEC\x91\x90ah\xBEV[\x90P\x03a$%W`@Q\x7FW\xCF\xA2\x17\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\n`\xFF\x16\x88\x80` \x01\x90a$:\x91\x90ah\xBEV[\x90P\x11\x15a$\x93W`\n\x88\x80` \x01\x90a$T\x91\x90ah\xBEV[\x90P`@Q\x7F\xC5\xABF~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a$\x8A\x92\x91\x90ai;V[`@Q\x80\x91\x03\x90\xFD[a$\xAC\x89\x806\x03\x81\x01\x90a$\xA7\x91\x90ai\xB7V[a8bV[a%\x04\x88\x80` \x01\x90a$\xBF\x91\x90ah\xBEV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x88a9\xADV[\x15a%XW\x86\x88\x80` \x01\x90a%\x1A\x91\x90ah\xBEV[`@Q\x7F\xDCMx\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a%O\x93\x92\x91\x90ai\xE2V[`@Q\x80\x91\x03\x90\xFD[_a%\xB3\x8C\x8C\x8B\x80` \x01\x90a%n\x91\x90ah\xBEV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x8Ba:+V[\x90P_`@Q\x80`\xC0\x01`@R\x80\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8B\x80` \x01\x90a&\x1A\x91\x90ah\xBEV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8B_\x015\x81R` \x01\x8C_\x015\x81R` \x01\x8C` \x015\x81R` \x01\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81RP\x90Pa&\xCF\x81\x8A\x88\x88a@\x0BV[_s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA1O\x89q\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a'\x1D\x91\x90aj\xC9V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'7W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'_\x91\x90amVV[\x90Pa'j\x81a=\xD6V[_a'sa+NV[\x90P\x80`\x0C\x01_\x81T\x80\x92\x91\x90a'\x89\x90am\x9DV[\x91\x90PUP_\x81`\x0C\x01T\x90P`@Q\x80`@\x01`@R\x80\x8C\x8C\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x86\x81RP\x82`\x06\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01\x90\x81a(\x14\x91\x90am\xEEV[P` \x82\x01Q\x81`\x01\x01\x90\x80Q\x90` \x01\x90a(1\x92\x91\x90aQ\x87V[P\x90PP\x80\x7F\xF9\x01\x1B\xD6\xBA\r\xA6\x04\x9CR\rp\xFEYq\xF1~\xD7\xAByT\x86\x05%D\xB5\x10\x19\x89lYk\x84\x8E\x8E\x8E\x8C\x8C`@Qa(o\x96\x95\x94\x93\x92\x91\x90apDV[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPPPPPV[a(\x91a5]V[_a(\x9Aa?\xE4V[\x90P\x81\x81_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a(\xFCa\x17TV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F8\xD1k\x8C\xAC\"\xD9\x9F\xC7\xC1$\xB9\xCD\r\xE2\xD3\xFA\x1F\xAE\xF4 \xBF\xE7\x91\xD8\xC3b\xD7e\xE2'\0`@Q`@Q\x80\x91\x03\x90\xA3PPV[_[\x84\x84\x90P\x81\x10\x15a+FWs\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c;\xCEI\x8D\x86\x86\x84\x81\x81\x10a)\x92Wa)\x91ah^V[[\x90P`@\x02\x01_\x015\x88`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a)\xB9\x92\x91\x90ah\x8BV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a)\xCFW_\x80\xFD[PZ\xFA\x15\x80\x15a)\xE1W=_\x80>=_\xFD[PPPPs\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c;\xCEI\x8D\x86\x86\x84\x81\x81\x10a*(Wa*'ah^V[[\x90P`@\x02\x01_\x015\x87\x87\x85\x81\x81\x10a*DWa*Cah^V[[\x90P`@\x02\x01` \x01` \x81\x01\x90a*\\\x91\x90a^^V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a*y\x92\x91\x90ah\x8BV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a*\x8FW_\x80\xFD[PZ\xFA\x15\x80\x15a*\xA1W=_\x80>=_\xFD[PPPPs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD4Goc\x86\x86\x84\x81\x81\x10a*\xE8Wa*\xE7ah^V[[\x90P`@\x02\x01_\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a+\r\x91\x90aWAV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a+#W_\x80\xFD[PZ\xFA\x15\x80\x15a+5W=_\x80>=_\xFD[PPPP\x80\x80`\x01\x01\x91PPa)DV[PPPPPPV[_\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\0\x90P\x90V[_a,5`@Q\x80`\xA0\x01`@R\x80`m\x81R` \x01ayU`m\x919\x80Q\x90` \x01 \x83_\x01Q\x80Q\x90` \x01 \x84` \x01Q`@Q` \x01a+\xB9\x91\x90aq\xEDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85`@\x01Q\x80Q\x90` \x01 \x86``\x01Q`@Q` \x01a+\xF0\x91\x90ar=V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a,\x1A\x95\x94\x93\x92\x91\x90arSV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a@\xE1V[\x90P\x91\x90PV[_a,Ea+NV[\x90P_a,\x95\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa@\xFAV[\x90Ps\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cl\x88\xEBC\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\xE4\x91\x90aY\x1EV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a,\xFAW_\x80\xFD[PZ\xFA\x15\x80\x15a-\x0CW=_\x80>=_\xFD[PPPP\x81`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a-\xAFW\x85\x81`@Q\x7F\x99\xECH\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a-\xA6\x92\x91\x90ar\xA4V[`@Q\x80\x91\x03\x90\xFD[`\x01\x82`\x02\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPPPPPPV[_\x80s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC2\xB4)\x86`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.|W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\xA0\x91\x90ar\xCBV[\x90P\x80\x83\x10\x15\x91PP\x91\x90PV[``_`\x01a.\xBC\x84aA$V[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.\xDAWa.\xD9aU\xABV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a/\x0CW\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x01\x82\x01\x90P[`\x01\x15a/mW\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a/bWa/aar\xF6V[[\x04\x94P_\x85\x03a/\x19W[\x81\x93PPPP\x91\x90PV[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[_a/\xA8a/xV[_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a/\xCBaBuV[a/\xD5\x82\x82aB\xB5V[PPV[a/\xE1aBuV[a/\xEA\x81aC\x06V[PV[a/\xF5aBuV[a/\xFDaC\x8AV[V[a0\x07aC\xBAV[_a0\x10a3\xEBV[\x90P_\x81_\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAAa0Ua6!V[`@Qa0b\x91\x90aY\x1EV[`@Q\x80\x91\x03\x90\xA1PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a1\x1AWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a1\x01aC\xFAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a1QW`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1\xB0W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1\xD4\x91\x90af|V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a2CW3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a2:\x91\x90aY\x1EV[`@Q\x80\x91\x03\x90\xFD[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a2\xAEWP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2\xAB\x91\x90as#V[`\x01[a2\xEFW\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a2\xE6\x91\x90aY\x1EV[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14a3UW\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a3L\x91\x90aWAV[`@Q\x80\x91\x03\x90\xFD[a3_\x83\x83aDMV[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a3\xE9W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0\x90P\x90V[_a4\xC5`@Q\x80`\x80\x01`@R\x80`T\x81R` \x01ay\xC2`T\x919\x80Q\x90` \x01 \x83_\x01Q`@Q` \x01a4J\x91\x90aq\xEDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q\x80Q\x90` \x01 \x85`@\x01Q`@Q` \x01a4\x81\x91\x90ar=V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a4\xAA\x94\x93\x92\x91\x90asNV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a@\xE1V[\x90P\x91\x90PV[_\x80s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c*8\x89\x98`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5+W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5O\x91\x90ar\xCBV[\x90P\x80\x83\x10\x15\x91PP\x91\x90PV[a5ea6!V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a5\x83a\x17TV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a5\xE2Wa5\xA6a6!V[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a5\xD9\x91\x90aY\x1EV[`@Q\x80\x91\x03\x90\xFD[V[_a5\xEDa?\xE4V[\x90P\x80_\x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90Ua6\x1D\x82aD\xBFV[PPV[_3\x90P\x90V[a60a8!V[_a69a3\xEBV[\x90P`\x01\x81_\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2Xa6\x7Fa6!V[`@Qa6\x8C\x91\x90aY\x1EV[`@Q\x80\x91\x03\x90\xA1PV[_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x90P\x90V[``_a6\xC9a6\x97V[\x90P\x80`\x02\x01\x80Ta6\xDA\x90a_\x8DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta7\x06\x90a_\x8DV[\x80\x15a7QW\x80`\x1F\x10a7(Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a7QV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a74W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[``_a7ga6\x97V[\x90P\x80`\x03\x01\x80Ta7x\x90a_\x8DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta7\xA4\x90a_\x8DV[\x80\x15a7\xEFW\x80`\x1F\x10a7\xC6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a7\xEFV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a7\xD2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[_\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\x90P\x90V[a8)a\x10sV[\x15a8`W`@Q\x7F\xD9<\x06e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x81` \x01Q\x03a8\x9FW`@Q\x7F\xDE(Y\xC1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01ma\xFF\xFF\x16\x81` \x01Q\x11\x15a8\xF6Wa\x01m\x81` \x01Q`@Q\x7F2\x95\x18c\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a8\xED\x92\x91\x90as\xCEV[`@Q\x80\x91\x03\x90\xFD[B\x81_\x01Q\x11\x15a9CWB\x81_\x01Q`@Q\x7F\xF2L\x08\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a9:\x92\x91\x90as\xF5V[`@Q\x80\x91\x03\x90\xFD[Bb\x01Q\x80\x82` \x01Qa9W\x91\x90at\x1CV[\x82_\x01Qa9e\x91\x90ae\xFAV[\x10\x15a9\xAAWB\x81`@Q\x7F04\x80@\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a9\xA1\x92\x91\x90at\x8AV[`@Q\x80\x91\x03\x90\xFD[PV[_\x80_\x90P[\x83Q\x81\x10\x15a: W\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x82\x81Q\x81\x10a9\xE6Wa9\xE5ah^V[[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a:\x13W`\x01\x91PPa:%V[\x80\x80`\x01\x01\x91PPa9\xB3V[P_\x90P[\x92\x91PPV[``_\x85\x85\x90P\x03a:iW`@Q\x7F\xA6\xA6\xCB!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84\x84\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:\x86Wa:\x85aU\xABV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a:\xB4W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x80[\x86\x86\x90P\x81\x10\x15a<\xABW_\x87\x87\x83\x81\x81\x10a:\xD9Wa:\xD8ah^V[[\x90P`@\x02\x01_\x015\x90P_\x88\x88\x84\x81\x81\x10a:\xF8Wa:\xF7ah^V[[\x90P`@\x02\x01` \x01` \x81\x01\x90a;\x10\x91\x90a^^V[\x90P_a;\x1C\x83aE\x90V[\x90Pa;'\x81aF\x1AV[a\xFF\xFF\x16\x85a;6\x91\x90ae\xFAV[\x94Ps\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c;\xCEI\x8D\x84\x89`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a;\x87\x92\x91\x90ah\x8BV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a;\x9DW_\x80\xFD[PZ\xFA\x15\x80\x15a;\xAFW=_\x80>=_\xFD[PPPPs\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c;\xCEI\x8D\x84\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a<\x02\x92\x91\x90ah\x8BV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a<\x18W_\x80\xFD[PZ\xFA\x15\x80\x15a<*W=_\x80>=_\xFD[PPPPa<8\x88\x83a9\xADV[a<{W\x81\x88`@Q\x7F\xA4\xC3\x03\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a<r\x92\x91\x90at\xB1V[`@Q\x80\x91\x03\x90\xFD[\x82\x86\x85\x81Q\x81\x10a<\x8FWa<\x8Eah^V[[` \x02` \x01\x01\x81\x81RPPPPP\x80\x80`\x01\x01\x91PPa:\xBAV[Pa\x08\0\x81\x11\x15a<\xF7Wa\x08\0\x81`@Q\x7F\xE7\xF4\x89]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a<\xEE\x92\x91\x90as\xF5V[`@Q\x80\x91\x03\x90\xFD[P\x94\x93PPPPV[_a=\n\x85aH\xA7V[\x90P_a=Z\x82\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa@\xFAV[\x90P\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a=\xCEW\x83\x83`@Q\x7F*\x87='\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a=\xC5\x92\x91\x90at\xDFV[`@Q\x80\x91\x03\x90\xFD[PPPPPPV[`\x01\x81Q\x11\x15a>\xB9W_\x81_\x81Q\x81\x10a=\xF4Wa=\xF3ah^V[[` \x02` \x01\x01Q` \x01Q\x90P_`\x01\x90P[\x82Q\x81\x10\x15a>\xB6W\x81\x83\x82\x81Q\x81\x10a>%Wa>$ah^V[[` \x02` \x01\x01Q` \x01Q\x14a>\xA9W\x82_\x81Q\x81\x10a>IWa>Hah^V[[` \x02` \x01\x01Q\x83\x82\x81Q\x81\x10a>dWa>cah^V[[` \x02` \x01\x01Q`@Q\x7F\xCF\xAE\x92\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a>\xA0\x92\x91\x90auaV[`@Q\x80\x91\x03\x90\xFD[\x80\x80`\x01\x01\x91PPa>\x08V[PP[PV[_\x80[\x82Q\x81\x10\x15a?\x94W_\x83\x82\x81Q\x81\x10a>\xDCWa>\xDBah^V[[` \x02` \x01\x01Q\x90P_a>\xF0\x82aE\x90V[\x90Pa>\xFB\x81aF\x1AV[a\xFF\xFF\x16\x84a?\n\x91\x90ae\xFAV[\x93Ps\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x19??,\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a?Y\x91\x90aWAV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a?oW_\x80\xFD[PZ\xFA\x15\x80\x15a?\x81W=_\x80>=_\xFD[PPPPPP\x80\x80`\x01\x01\x91PPa>\xBFV[Pa\x08\0\x81\x11\x15a?\xE0Wa\x08\0\x81`@Q\x7F\xE7\xF4\x89]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a?\xD7\x92\x91\x90as\xF5V[`@Q\x80\x91\x03\x90\xFD[PPV[_\x7F#~\x15\x82\"\xE3\xE6\x96\x8Br\xB9\xDB\r\x80C\xAA\xCF\x07J\xD9\xF6P\xF0\xD1`kM\x82\xEEC,\0\x90P\x90V[_a@\x15\x85aI\x84V[\x90P_a@e\x82\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa@\xFAV[\x90P\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a@\xD9W\x83\x83`@Q\x7F*\x87='\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a@\xD0\x92\x91\x90at\xDFV[`@Q\x80\x91\x03\x90\xFD[PPPPPPV[_a@\xF3a@\xEDaJZV[\x83aJhV[\x90P\x91\x90PV[_\x80_\x80aA\x08\x86\x86aJ\xA8V[\x92P\x92P\x92PaA\x18\x82\x82aJ\xFDV[\x82\x93PPPP\x92\x91PPV[_\x80_\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10aA\x80Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81aAvWaAuar\xF6V[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10aA\xBDWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81aA\xB3WaA\xB2ar\xF6V[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10aA\xECWf#\x86\xF2o\xC1\0\0\x83\x81aA\xE2WaA\xE1ar\xF6V[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10aB\x15Wc\x05\xF5\xE1\0\x83\x81aB\x0BWaB\nar\xF6V[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10aB:Wa'\x10\x83\x81aB0WaB/ar\xF6V[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10aB]W`d\x83\x81aBSWaBRar\xF6V[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10aBlW`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[aB}aL_V[aB\xB3W`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[aB\xBDaBuV[_aB\xC6a6\x97V[\x90P\x82\x81`\x02\x01\x90\x81aB\xD9\x91\x90au\xEEV[P\x81\x81`\x03\x01\x90\x81aB\xEB\x91\x90au\xEEV[P_\x80\x1B\x81_\x01\x81\x90UP_\x80\x1B\x81`\x01\x01\x81\x90UPPPPV[aC\x0EaBuV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03aC~W_`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aCu\x91\x90aY\x1EV[`@Q\x80\x91\x03\x90\xFD[aC\x87\x81a5\xE4V[PV[aC\x92aBuV[_aC\x9Ba3\xEBV[\x90P_\x81_\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPV[aC\xC2a\x10sV[aC\xF8W`@Q\x7F\x8D\xFC +\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_aD&\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1BaL}V[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[aDV\x82aL\x86V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15aD\xB2WaD\xAC\x82\x82aMOV[PaD\xBBV[aD\xBAaM\xCFV[[PPV[_aD\xC8a7\xFAV[\x90P_\x81_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x82\x82_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPPV[_\x80`\xF8`\xF0\x84\x90\x1B\x90\x1C_\x1C\x90P`S\x80\x81\x11\x15aE\xB2WaE\xB1a_\x1AV[[`\xFF\x16\x81`\xFF\x16\x11\x15aE\xFCW\x80`@Q\x7Fd\x19P\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aE\xF3\x91\x90av\xBDV[`@Q\x80\x91\x03\x90\xFD[\x80`\xFF\x16`S\x81\x11\x15aF\x12WaF\x11a_\x1AV[[\x91PP\x91\x90PV[_\x80`S\x81\x11\x15aF.WaF-a_\x1AV[[\x82`S\x81\x11\x15aFAWaF@a_\x1AV[[\x03aFOW`\x02\x90PaH\xA2V[`\x02`S\x81\x11\x15aFcWaFba_\x1AV[[\x82`S\x81\x11\x15aFvWaFua_\x1AV[[\x03aF\x84W`\x08\x90PaH\xA2V[`\x03`S\x81\x11\x15aF\x98WaF\x97a_\x1AV[[\x82`S\x81\x11\x15aF\xABWaF\xAAa_\x1AV[[\x03aF\xB9W`\x10\x90PaH\xA2V[`\x04`S\x81\x11\x15aF\xCDWaF\xCCa_\x1AV[[\x82`S\x81\x11\x15aF\xE0WaF\xDFa_\x1AV[[\x03aF\xEEW` \x90PaH\xA2V[`\x05`S\x81\x11\x15aG\x02WaG\x01a_\x1AV[[\x82`S\x81\x11\x15aG\x15WaG\x14a_\x1AV[[\x03aG#W`@\x90PaH\xA2V[`\x06`S\x81\x11\x15aG7WaG6a_\x1AV[[\x82`S\x81\x11\x15aGJWaGIa_\x1AV[[\x03aGXW`\x80\x90PaH\xA2V[`\x07`S\x81\x11\x15aGlWaGka_\x1AV[[\x82`S\x81\x11\x15aG\x7FWaG~a_\x1AV[[\x03aG\x8DW`\xA0\x90PaH\xA2V[`\x08`S\x81\x11\x15aG\xA1WaG\xA0a_\x1AV[[\x82`S\x81\x11\x15aG\xB4WaG\xB3a_\x1AV[[\x03aG\xC3Wa\x01\0\x90PaH\xA2V[`\t`S\x81\x11\x15aG\xD7WaG\xD6a_\x1AV[[\x82`S\x81\x11\x15aG\xEAWaG\xE9a_\x1AV[[\x03aG\xF9Wa\x02\0\x90PaH\xA2V[`\n`S\x81\x11\x15aH\rWaH\x0Ca_\x1AV[[\x82`S\x81\x11\x15aH WaH\x1Fa_\x1AV[[\x03aH/Wa\x04\0\x90PaH\xA2V[`\x0B`S\x81\x11\x15aHCWaHBa_\x1AV[[\x82`S\x81\x11\x15aHVWaHUa_\x1AV[[\x03aHeWa\x08\0\x90PaH\xA2V[\x81`@Q\x7F\xBEx0\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aH\x99\x91\x90aw\x1CV[`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[_\x80`@Q\x80a\x01\0\x01`@R\x80`\xC2\x81R` \x01az\xB6`\xC2\x919\x80Q\x90` \x01 \x83_\x01Q\x80Q\x90` \x01 \x84` \x01Q`@Q` \x01aH\xEA\x91\x90aw\xC1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85`@\x01Q\x86``\x01Q\x87`\x80\x01Q\x88`\xA0\x01Q\x89`\xC0\x01Q`@Q` \x01aI)\x91\x90ar=V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aIV\x98\x97\x96\x95\x94\x93\x92\x91\x90aw\xD7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90PaI|\x83``\x01Q\x82aN\x0BV[\x91PP\x91\x90PV[_\x80`@Q\x80`\xC0\x01`@R\x80`\xA0\x81R` \x01az\x16`\xA0\x919\x80Q\x90` \x01 \x83_\x01Q\x80Q\x90` \x01 \x84` \x01Q`@Q` \x01aI\xC6\x91\x90aw\xC1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85`@\x01Q\x86``\x01Q\x87`\x80\x01Q\x88`\xA0\x01Q`@Q` \x01aJ\0\x91\x90ar=V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aJ,\x97\x96\x95\x94\x93\x92\x91\x90axSV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90PaJR\x83`@\x01Q\x82aN\x0BV[\x91PP\x91\x90PV[_aJcaN\x7FV[\x90P\x90V[_`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x02\x82\x01R\x82`\"\x82\x01R`B\x81 \x91PP\x92\x91PPV[_\x80_`A\x84Q\x03aJ\xE8W_\x80_` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90PaJ\xDA\x88\x82\x85\x85aN\xE2V[\x95P\x95P\x95PPPPaJ\xF6V[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15aK\x10WaK\x0Fa_\x1AV[[\x82`\x03\x81\x11\x15aK#WaK\"a_\x1AV[[\x03\x15aL[W`\x01`\x03\x81\x11\x15aK=WaK<a_\x1AV[[\x82`\x03\x81\x11\x15aKPWaKOa_\x1AV[[\x03aK\x87W`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15aK\x9BWaK\x9Aa_\x1AV[[\x82`\x03\x81\x11\x15aK\xAEWaK\xADa_\x1AV[[\x03aK\xF2W\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aK\xE9\x91\x90a_GV[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15aL\x05WaL\x04a_\x1AV[[\x82`\x03\x81\x11\x15aL\x18WaL\x17a_\x1AV[[\x03aLZW\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aLQ\x91\x90aWAV[`@Q\x80\x91\x03\x90\xFD[[PPV[_aLha/xV[_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03aL\xE1W\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aL\xD8\x91\x90aY\x1EV[`@Q\x80\x91\x03\x90\xFD[\x80aM\r\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1BaL}V[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@QaMx\x91\x90ar=V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14aM\xB0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>aM\xB5V[``\x91P[P\x91P\x91PaM\xC5\x85\x83\x83aO\xC9V[\x92PPP\x92\x91PPV[_4\x11\x15aN\tW`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x80\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0FaN6aPVV[aN>aP\xCCV[\x860`@Q` \x01aNT\x95\x94\x93\x92\x91\x90ax\xC0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90PaNv\x81\x84aJhV[\x91PP\x92\x91PPV[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0FaN\xA9aPVV[aN\xB1aP\xCCV[F0`@Q` \x01aN\xC7\x95\x94\x93\x92\x91\x90ax\xC0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x80_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15aO\x1EW_`\x03\x85\x92P\x92P\x92PaO\xBFV[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@QaOA\x94\x93\x92\x91\x90ay\x11V[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aOaW=_\x80>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03aO\xB2W_`\x01_\x80\x1B\x93P\x93P\x93PPaO\xBFV[\x80_\x80_\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[``\x82aO\xDEWaO\xD9\x82aQCV[aPNV[_\x82Q\x14\x80\x15aP\x04WP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15aPFW\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aP=\x91\x90aY\x1EV[`@Q\x80\x91\x03\x90\xFD[\x81\x90PaPOV[[\x93\x92PPPV[_\x80aP`a6\x97V[\x90P_aPka6\xBEV[\x90P_\x81Q\x11\x15aP\x87W\x80\x80Q\x90` \x01 \x92PPPaP\xC9V[_\x82_\x01T\x90P_\x80\x1B\x81\x14aP\xA2W\x80\x93PPPPaP\xC9V[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x80aP\xD6a6\x97V[\x90P_aP\xE1a7\\V[\x90P_\x81Q\x11\x15aP\xFDW\x80\x80Q\x90` \x01 \x92PPPaQ@V[_\x82`\x01\x01T\x90P_\x80\x1B\x81\x14aQ\x19W\x80\x93PPPPaQ@V[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x81Q\x11\x15aQUW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15aQ\xC1W\x91` \x02\x82\x01[\x82\x81\x11\x15aQ\xC0W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90aQ\xA5V[[P\x90PaQ\xCE\x91\x90aR\x1DV[P\x90V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15aR\x0CW\x91` \x02\x82\x01[\x82\x81\x11\x15aR\x0BW\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90aQ\xF0V[[P\x90PaR\x19\x91\x90aR\x1DV[P\x90V[[\x80\x82\x11\x15aR4W_\x81_\x90UP`\x01\x01aR\x1EV[P\x90V[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x81\x90P\x91\x90PV[aR[\x81aRIV[\x81\x14aReW_\x80\xFD[PV[_\x815\x90PaRv\x81aRRV[\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12aR\x9DWaR\x9CaR|V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aR\xBAWaR\xB9aR\x80V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15aR\xD6WaR\xD5aR\x84V[[\x92P\x92\x90PV[_\x80_\x80_\x80_`\x80\x88\x8A\x03\x12\x15aR\xF8WaR\xF7aRAV[[_aS\x05\x8A\x82\x8B\x01aRhV[\x97PP` \x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aS&WaS%aREV[[aS2\x8A\x82\x8B\x01aR\x88V[\x96P\x96PP`@\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aSUWaSTaREV[[aSa\x8A\x82\x8B\x01aR\x88V[\x94P\x94PP``\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aS\x84WaS\x83aREV[[aS\x90\x8A\x82\x8B\x01aR\x88V[\x92P\x92PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[_` \x82\x84\x03\x12\x15aS\xB6WaS\xB5aRAV[[_aS\xC3\x84\x82\x85\x01aRhV[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_aT\x1E\x82aS\xF5V[\x90P\x91\x90PV[aT.\x81aT\x14V[\x82RPPV[_aT?\x83\x83aT%V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aTa\x82aS\xCCV[aTk\x81\x85aS\xD6V[\x93PaTv\x83aS\xE6V[\x80_[\x83\x81\x10\x15aT\xA6W\x81QaT\x8D\x88\x82aT4V[\x97PaT\x98\x83aTKV[\x92PP`\x01\x81\x01\x90PaTyV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaT\xCB\x81\x84aTWV[\x90P\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15aU\nW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90PaT\xEFV[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_aU/\x82aT\xD3V[aU9\x81\x85aT\xDDV[\x93PaUI\x81\x85` \x86\x01aT\xEDV[aUR\x81aU\x15V[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaUu\x81\x84aU%V[\x90P\x92\x91PPV[aU\x86\x81aT\x14V[\x81\x14aU\x90W_\x80\xFD[PV[_\x815\x90PaU\xA1\x81aU}V[\x92\x91PPV[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[aU\xE1\x82aU\x15V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aV\0WaU\xFFaU\xABV[[\x80`@RPPPV[_aV\x12aR8V[\x90PaV\x1E\x82\x82aU\xD8V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aV=WaV<aU\xABV[[aVF\x82aU\x15V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_aVsaVn\x84aV#V[aV\tV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15aV\x8FWaV\x8EaU\xA7V[[aV\x9A\x84\x82\x85aVSV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aV\xB6WaV\xB5aR|V[[\x815aV\xC6\x84\x82` \x86\x01aVaV[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15aV\xE5WaV\xE4aRAV[[_aV\xF2\x85\x82\x86\x01aU\x93V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aW\x13WaW\x12aREV[[aW\x1F\x85\x82\x86\x01aV\xA2V[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[aW;\x81aW)V[\x82RPPV[_` \x82\x01\x90PaWT_\x83\x01\x84aW2V[\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[aWn\x81aWZV[\x82RPPV[_` \x82\x01\x90PaW\x87_\x83\x01\x84aWeV[\x92\x91PPV[_\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[aW\xC1\x81aW\x8DV[\x82RPPV[aW\xD0\x81aRIV[\x82RPPV[aW\xDF\x81aT\x14V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[aX\x17\x81aRIV[\x82RPPV[_aX(\x83\x83aX\x0EV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aXJ\x82aW\xE5V[aXT\x81\x85aW\xEFV[\x93PaX_\x83aW\xFFV[\x80_[\x83\x81\x10\x15aX\x8FW\x81QaXv\x88\x82aX\x1DV[\x97PaX\x81\x83aX4V[\x92PP`\x01\x81\x01\x90PaXbV[P\x85\x93PPPP\x92\x91PPV[_`\xE0\x82\x01\x90PaX\xAF_\x83\x01\x8AaW\xB8V[\x81\x81\x03` \x83\x01RaX\xC1\x81\x89aU%V[\x90P\x81\x81\x03`@\x83\x01RaX\xD5\x81\x88aU%V[\x90PaX\xE4``\x83\x01\x87aW\xC7V[aX\xF1`\x80\x83\x01\x86aW\xD6V[aX\xFE`\xA0\x83\x01\x85aW2V[\x81\x81\x03`\xC0\x83\x01RaY\x10\x81\x84aX@V[\x90P\x98\x97PPPPPPPPV[_` \x82\x01\x90PaY1_\x83\x01\x84aW\xD6V[\x92\x91PPV[_\x80\xFD[_`@\x82\x84\x03\x12\x15aYPWaYOaY7V[[\x81\x90P\x92\x91PPV[_\x80\x83`\x1F\x84\x01\x12aYnWaYmaR|V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aY\x8BWaY\x8AaR\x80V[[` \x83\x01\x91P\x83`@\x82\x02\x83\x01\x11\x15aY\xA7WaY\xA6aR\x84V[[\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12aY\xC3WaY\xC2aR|V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aY\xE0WaY\xDFaR\x80V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aY\xFCWaY\xFBaR\x84V[[\x92P\x92\x90PV[_\x80_\x80_\x80_\x80`\xC0\x89\x8B\x03\x12\x15aZ\x1FWaZ\x1EaRAV[[_aZ,\x8B\x82\x8C\x01aRhV[\x98PP` aZ=\x8B\x82\x8C\x01aY;V[\x97PP``\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZ^WaZ]aREV[[aZj\x8B\x82\x8C\x01aYYV[\x96P\x96PP`\x80\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZ\x8DWaZ\x8CaREV[[aZ\x99\x8B\x82\x8C\x01aY\xAEV[\x94P\x94PP`\xA0\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZ\xBCWaZ\xBBaREV[[aZ\xC8\x8B\x82\x8C\x01aR\x88V[\x92P\x92PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[_`@\x82\x84\x03\x12\x15aZ\xEFWaZ\xEEaY7V[[\x81\x90P\x92\x91PPV[_`@\x82\x84\x03\x12\x15a[\rWa[\x0CaY7V[[\x81\x90P\x92\x91PPV[_\x80_\x80_\x80_\x80_\x80_a\x01 \x8C\x8E\x03\x12\x15a[6Wa[5aRAV[[_\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a[SWa[RaREV[[a[_\x8E\x82\x8F\x01aYYV[\x9BP\x9BPP` a[r\x8E\x82\x8F\x01aZ\xDAV[\x99PP``a[\x83\x8E\x82\x8F\x01aY;V[\x98PP`\xA0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a[\xA4Wa[\xA3aREV[[a[\xB0\x8E\x82\x8F\x01aZ\xF8V[\x97PP`\xC0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a[\xD1Wa[\xD0aREV[[a[\xDD\x8E\x82\x8F\x01aR\x88V[\x96P\x96PP`\xE0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\\\0Wa[\xFFaREV[[a\\\x0C\x8E\x82\x8F\x01aR\x88V[\x94P\x94PPa\x01\0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\\0Wa\\/aREV[[a\\<\x8E\x82\x8F\x01aR\x88V[\x92P\x92PP\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[_\x80\x83`\x1F\x84\x01\x12a\\fWa\\eaR|V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\\\x83Wa\\\x82aR\x80V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a\\\x9FWa\\\x9EaR\x84V[[\x92P\x92\x90PV[_\x80_\x80`@\x85\x87\x03\x12\x15a\\\xBEWa\\\xBDaRAV[[_\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\\\xDBWa\\\xDAaREV[[a\\\xE7\x87\x82\x88\x01a\\QV[\x94P\x94PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a]\nWa]\taREV[[a]\x16\x87\x82\x88\x01aR\x88V[\x92P\x92PP\x92\x95\x91\x94P\x92PV[_\x80_\x80_\x80_\x80_\x80_a\x01\0\x8C\x8E\x03\x12\x15a]DWa]CaRAV[[_\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a]aWa]`aREV[[a]m\x8E\x82\x8F\x01aYYV[\x9BP\x9BPP` a]\x80\x8E\x82\x8F\x01aZ\xDAV[\x99PP``\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a]\xA1Wa]\xA0aREV[[a]\xAD\x8E\x82\x8F\x01aZ\xF8V[\x98PP`\x80a]\xBE\x8E\x82\x8F\x01aU\x93V[\x97PP`\xA0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a]\xDFWa]\xDEaREV[[a]\xEB\x8E\x82\x8F\x01aR\x88V[\x96P\x96PP`\xC0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a^\x0EWa^\raREV[[a^\x1A\x8E\x82\x8F\x01aR\x88V[\x94P\x94PP`\xE0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a^=Wa^<aREV[[a^I\x8E\x82\x8F\x01aR\x88V[\x92P\x92PP\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[_` \x82\x84\x03\x12\x15a^sWa^raRAV[[_a^\x80\x84\x82\x85\x01aU\x93V[\x91PP\x92\x91PPV[_\x80_\x80_``\x86\x88\x03\x12\x15a^\xA2Wa^\xA1aRAV[[_a^\xAF\x88\x82\x89\x01aU\x93V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a^\xD0Wa^\xCFaREV[[a^\xDC\x88\x82\x89\x01aYYV[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a^\xFFWa^\xFEaREV[[a_\x0B\x88\x82\x89\x01aR\x88V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[_` \x82\x01\x90Pa_Z_\x83\x01\x84aW\xC7V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a_\xA4W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a_\xB7Wa_\xB6a_`V[[P\x91\x90PV[_\x82\x90P\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02a`#\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a_\xE8V[a`-\x86\x83a_\xE8V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_a`ha`ca`^\x84aRIV[a`EV[aRIV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a`\x81\x83a`NV[a`\x95a`\x8D\x82a`oV[\x84\x84Ta_\xF4V[\x82UPPPPV[_\x90V[a`\xA9a`\x9DV[a`\xB4\x81\x84\x84a`xV[PPPV[[\x81\x81\x10\x15a`\xD7Wa`\xCC_\x82a`\xA1V[`\x01\x81\x01\x90Pa`\xBAV[PPV[`\x1F\x82\x11\x15aa\x1CWa`\xED\x81a_\xC7V[a`\xF6\x84a_\xD9V[\x81\x01` \x85\x10\x15aa\x05W\x81\x90P[aa\x19aa\x11\x85a_\xD9V[\x83\x01\x82a`\xB9V[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_aa<_\x19\x84`\x08\x02aa!V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_aaT\x83\x83aa-V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[aan\x83\x83a_\xBDV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aa\x87Waa\x86aU\xABV[[aa\x91\x82Ta_\x8DV[aa\x9C\x82\x82\x85a`\xDBV[_`\x1F\x83\x11`\x01\x81\x14aa\xC9W_\x84\x15aa\xB7W\x82\x87\x015\x90P[aa\xC1\x85\x82aaIV[\x86UPab(V[`\x1F\x19\x84\x16aa\xD7\x86a_\xC7V[_[\x82\x81\x10\x15aa\xFEW\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Paa\xD9V[\x86\x83\x10\x15ab\x1BW\x84\x89\x015ab\x17`\x1F\x89\x16\x82aa-V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[_\x81T\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81Taby\x81a_\x8DV[ab\x83\x81\x86ab]V[\x94P`\x01\x82\x16_\x81\x14ab\x9DW`\x01\x81\x14ab\xB3Wab\xE5V[`\xFF\x19\x83\x16\x86R\x81\x15\x15` \x02\x86\x01\x93Pab\xE5V[ab\xBC\x85a_\xC7V[_[\x83\x81\x10\x15ab\xDDW\x81T\x81\x89\x01R`\x01\x82\x01\x91P` \x81\x01\x90Pab\xBEV[\x80\x88\x01\x95PPP[PPP\x92\x91PPV[_ab\xF9\x83\x83abmV[\x90P\x92\x91PPV[_`\x01\x82\x01\x90P\x91\x90PV[_ac\x17\x82ab1V[ac!\x81\x85ab;V[\x93P\x83` \x82\x02\x85\x01ac3\x85abKV[\x80_[\x85\x81\x10\x15acmW\x84\x84\x03\x89R\x81acN\x85\x82ab\xEEV[\x94PacY\x83ac\x01V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pac6V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_\x81T\x90P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_`\x01\x82\x01\x90P\x91\x90PV[_ac\xB1\x82ac\x7FV[ac\xBB\x81\x85ab;V[\x93P\x83` \x82\x02\x85\x01ac\xCD\x85ac\x89V[\x80_[\x85\x81\x10\x15ad\x07W\x84\x84\x03\x89R\x81ac\xE8\x85\x82ab\xEEV[\x94Pac\xF3\x83ac\x9BV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pac\xD0V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_ad4\x83\x85ad\x19V[\x93PadA\x83\x85\x84aVSV[adJ\x83aU\x15V[\x84\x01\x90P\x93\x92PPPV[_``\x82\x01\x90P\x81\x81\x03_\x83\x01Radm\x81\x87ac\rV[\x90P\x81\x81\x03` \x83\x01Rad\x81\x81\x86ac\xA7V[\x90P\x81\x81\x03`@\x83\x01Rad\x96\x81\x84\x86ad)V[\x90P\x95\x94PPPPPV[_\x81\x90P\x92\x91PPV[_ad\xB5\x82aT\xD3V[ad\xBF\x81\x85ad\xA1V[\x93Pad\xCF\x81\x85` \x86\x01aT\xEDV[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_ae\x0F`\x02\x83ad\xA1V[\x91Pae\x1A\x82ad\xDBV[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aeY`\x01\x83ad\xA1V[\x91Paed\x82ae%V[`\x01\x82\x01\x90P\x91\x90PV[_aez\x82\x87ad\xABV[\x91Pae\x85\x82ae\x03V[\x91Pae\x91\x82\x86ad\xABV[\x91Pae\x9C\x82aeMV[\x91Pae\xA8\x82\x85ad\xABV[\x91Pae\xB3\x82aeMV[\x91Pae\xBF\x82\x84ad\xABV[\x91P\x81\x90P\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_af\x04\x82aRIV[\x91Paf\x0F\x83aRIV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15af'Waf&ae\xCDV[[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[afI\x81af-V[\x82RPPV[_` \x82\x01\x90Pafb_\x83\x01\x84af@V[\x92\x91PPV[_\x81Q\x90Pafv\x81aU}V[\x92\x91PPV[_` \x82\x84\x03\x12\x15af\x91Waf\x90aRAV[[_af\x9E\x84\x82\x85\x01afhV[\x91PP\x92\x91PPV[_``\x82\x01\x90P\x81\x81\x03_\x83\x01Raf\xC0\x81\x87\x89ad)V[\x90P\x81\x81\x03` \x83\x01Raf\xD4\x81\x86ac\xA7V[\x90P\x81\x81\x03`@\x83\x01Raf\xE9\x81\x84\x86ad)V[\x90P\x96\x95PPPPPPV[\x7FEIP712: Uninitialized\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_ag)`\x15\x83aT\xDDV[\x91Pag4\x82af\xF5V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RagV\x81ag\x1DV[\x90P\x91\x90PV[_agk` \x84\x01\x84aU\x93V[\x90P\x92\x91PPV[`@\x82\x01ag\x83_\x83\x01\x83ag]V[ag\x8F_\x85\x01\x82aT%V[Pag\x9D` \x83\x01\x83ag]V[ag\xAA` \x85\x01\x82aT%V[PPPPV[_\x81\x90P\x91\x90PV[_` \x82\x01\x90P\x91\x90PV[_ag\xD0\x83\x85aS\xD6V[\x93Pag\xDB\x82ag\xB0V[\x80_[\x85\x81\x10\x15ah\x13Wag\xF0\x82\x84ag]V[ag\xFA\x88\x82aT4V[\x97Pah\x05\x83ag\xB9V[\x92PP`\x01\x81\x01\x90Pag\xDEV[P\x85\x92PPP\x93\x92PPPV[_`\x80\x82\x01\x90Pah3_\x83\x01\x87aW\xC7V[ah@` \x83\x01\x86agsV[\x81\x81\x03``\x83\x01RahS\x81\x84\x86ag\xC5V[\x90P\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_`@\x82\x01\x90Pah\x9E_\x83\x01\x85aW2V[ah\xAB` \x83\x01\x84aW\xD6V[\x93\x92PPPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12ah\xDAWah\xD9ah\xB2V[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15ah\xFCWah\xFBah\xB6V[[` \x83\x01\x92P` \x82\x026\x03\x83\x13\x15ai\x18Wai\x17ah\xBAV[[P\x92P\x92\x90PV[_`\xFF\x82\x16\x90P\x91\x90PV[ai5\x81ai V[\x82RPPV[_`@\x82\x01\x90PaiN_\x83\x01\x85ai,V[ai[` \x83\x01\x84aW\xC7V[\x93\x92PPPV[_\x80\xFD[_\x80\xFD[_`@\x82\x84\x03\x12\x15ai\x7FWai~aibV[[ai\x89`@aV\tV[\x90P_ai\x98\x84\x82\x85\x01aRhV[_\x83\x01RP` ai\xAB\x84\x82\x85\x01aRhV[` \x83\x01RP\x92\x91PPV[_`@\x82\x84\x03\x12\x15ai\xCCWai\xCBaRAV[[_ai\xD9\x84\x82\x85\x01aijV[\x91PP\x92\x91PPV[_`@\x82\x01\x90Pai\xF5_\x83\x01\x86aW\xD6V[\x81\x81\x03` \x83\x01Raj\x08\x81\x84\x86ag\xC5V[\x90P\x94\x93PPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[ajD\x81aW)V[\x82RPPV[_ajU\x83\x83aj;V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_ajw\x82aj\x12V[aj\x81\x81\x85aj\x1CV[\x93Paj\x8C\x83aj,V[\x80_[\x83\x81\x10\x15aj\xBCW\x81Qaj\xA3\x88\x82ajJV[\x97Paj\xAE\x83ajaV[\x92PP`\x01\x81\x01\x90Paj\x8FV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Raj\xE1\x81\x84ajmV[\x90P\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15ak\x03Wak\x02aU\xABV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[ak\x1D\x81aW)V[\x81\x14ak'W_\x80\xFD[PV[_\x81Q\x90Pak8\x81ak\x14V[\x92\x91PPV[_\x81Q\x90PakL\x81aRRV[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aklWakkaU\xABV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_ak\x8Fak\x8A\x84akRV[aV\tV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15ak\xB2Wak\xB1aR\x84V[[\x83[\x81\x81\x10\x15ak\xDBW\x80ak\xC7\x88\x82afhV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pak\xB4V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12ak\xF9Wak\xF8aR|V[[\x81Qal\t\x84\x82` \x86\x01ak}V[\x91PP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15al'Wal&aibV[[al1`\x80aV\tV[\x90P_al@\x84\x82\x85\x01ak*V[_\x83\x01RP` alS\x84\x82\x85\x01ak>V[` \x83\x01RP`@alg\x84\x82\x85\x01ak*V[`@\x83\x01RP``\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15al\x8BWal\x8AaifV[[al\x97\x84\x82\x85\x01ak\xE5V[``\x83\x01RP\x92\x91PPV[_al\xB5al\xB0\x84aj\xE9V[aV\tV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15al\xD8Wal\xD7aR\x84V[[\x83[\x81\x81\x10\x15am\x1FW\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15al\xFDWal\xFCaR|V[[\x80\x86\x01am\n\x89\x82al\x12V[\x85R` \x85\x01\x94PPP` \x81\x01\x90Pal\xDAV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12am=Wam<aR|V[[\x81QamM\x84\x82` \x86\x01al\xA3V[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15amkWamjaRAV[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15am\x88Wam\x87aREV[[am\x94\x84\x82\x85\x01am)V[\x91PP\x92\x91PPV[_am\xA7\x82aRIV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03am\xD9Wam\xD8ae\xCDV[[`\x01\x82\x01\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[am\xF7\x82am\xE4V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15an\x10Wan\x0FaU\xABV[[an\x1A\x82Ta_\x8DV[an%\x82\x82\x85a`\xDBV[_` \x90P`\x1F\x83\x11`\x01\x81\x14anVW_\x84\x15anDW\x82\x87\x01Q\x90P[anN\x85\x82aaIV[\x86UPan\xB5V[`\x1F\x19\x84\x16and\x86a_\xC7V[_[\x82\x81\x10\x15an\x8BW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PanfV[\x86\x83\x10\x15an\xA8W\x84\x89\x01Qan\xA4`\x1F\x89\x16\x82aa-V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_ao\0\x82aS\xCCV[ao\n\x81\x85an\xE6V[\x93Pao\x15\x83aS\xE6V[\x80_[\x83\x81\x10\x15aoEW\x81Qao,\x88\x82aT4V[\x97Pao7\x83aTKV[\x92PP`\x01\x81\x01\x90Pao\x18V[P\x85\x93PPPP\x92\x91PPV[_`\x80\x83\x01_\x83\x01Qaog_\x86\x01\x82aj;V[P` \x83\x01Qaoz` \x86\x01\x82aX\x0EV[P`@\x83\x01Qao\x8D`@\x86\x01\x82aj;V[P``\x83\x01Q\x84\x82\x03``\x86\x01Rao\xA5\x82\x82an\xF6V[\x91PP\x80\x91PP\x92\x91PPV[_ao\xBD\x83\x83aoRV[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_ao\xDB\x82an\xBDV[ao\xE5\x81\x85an\xC7V[\x93P\x83` \x82\x02\x85\x01ao\xF7\x85an\xD7V[\x80_[\x85\x81\x10\x15ap2W\x84\x84\x03\x89R\x81Qap\x13\x85\x82ao\xB2V[\x94Pap\x1E\x83ao\xC5V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pao\xFAV[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_`\x80\x82\x01\x90P\x81\x81\x03_\x83\x01Rap\\\x81\x89ao\xD1V[\x90Papk` \x83\x01\x88aW\xD6V[\x81\x81\x03`@\x83\x01Rap~\x81\x86\x88ad)V[\x90P\x81\x81\x03``\x83\x01Rap\x93\x81\x84\x86ad)V[\x90P\x97\x96PPPPPPPV[_\x80\xFD[\x82\x81\x837PPPV[_ap\xB8\x83\x85aj\x1CV[\x93P\x7F\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15ap\xEBWap\xEAap\xA0V[[` \x83\x02\x92Pap\xFC\x83\x85\x84ap\xA4V[\x82\x84\x01\x90P\x93\x92PPPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Raq!\x81\x84\x86ap\xADV[\x90P\x93\x92PPPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01RaqB\x81\x86ao\xD1V[\x90P\x81\x81\x03` \x83\x01RaqW\x81\x84\x86ad)V[\x90P\x94\x93PPPPV[_\x81\x90P\x92\x91PPV[aqt\x81aW)V[\x82RPPV[_aq\x85\x83\x83aqkV[` \x83\x01\x90P\x92\x91PPV[_aq\x9B\x82aj\x12V[aq\xA5\x81\x85aqaV[\x93Paq\xB0\x83aj,V[\x80_[\x83\x81\x10\x15aq\xE0W\x81Qaq\xC7\x88\x82aqzV[\x97Paq\xD2\x83ajaV[\x92PP`\x01\x81\x01\x90Paq\xB3V[P\x85\x93PPPP\x92\x91PPV[_aq\xF8\x82\x84aq\x91V[\x91P\x81\x90P\x92\x91PPV[_\x81\x90P\x92\x91PPV[_ar\x17\x82am\xE4V[ar!\x81\x85ar\x03V[\x93Par1\x81\x85` \x86\x01aT\xEDV[\x80\x84\x01\x91PP\x92\x91PPV[_arH\x82\x84ar\rV[\x91P\x81\x90P\x92\x91PPV[_`\xA0\x82\x01\x90Parf_\x83\x01\x88aW2V[ars` \x83\x01\x87aW2V[ar\x80`@\x83\x01\x86aW2V[ar\x8D``\x83\x01\x85aW2V[ar\x9A`\x80\x83\x01\x84aW2V[\x96\x95PPPPPPV[_`@\x82\x01\x90Par\xB7_\x83\x01\x85aW\xC7V[ar\xC4` \x83\x01\x84aW\xD6V[\x93\x92PPPV[_` \x82\x84\x03\x12\x15ar\xE0War\xDFaRAV[[_ar\xED\x84\x82\x85\x01ak>V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15as8Was7aRAV[[_asE\x84\x82\x85\x01ak*V[\x91PP\x92\x91PPV[_`\x80\x82\x01\x90Pasa_\x83\x01\x87aW2V[asn` \x83\x01\x86aW2V[as{`@\x83\x01\x85aW2V[as\x88``\x83\x01\x84aW2V[\x95\x94PPPPPV[_a\xFF\xFF\x82\x16\x90P\x91\x90PV[_as\xB8as\xB3as\xAE\x84as\x91V[a`EV[aRIV[\x90P\x91\x90PV[as\xC8\x81as\x9EV[\x82RPPV[_`@\x82\x01\x90Pas\xE1_\x83\x01\x85as\xBFV[as\xEE` \x83\x01\x84aW\xC7V[\x93\x92PPPV[_`@\x82\x01\x90Pat\x08_\x83\x01\x85aW\xC7V[at\x15` \x83\x01\x84aW\xC7V[\x93\x92PPPV[_at&\x82aRIV[\x91Pat1\x83aRIV[\x92P\x82\x82\x02at?\x81aRIV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17atVWatUae\xCDV[[P\x92\x91PPV[`@\x82\x01_\x82\x01Qatq_\x85\x01\x82aX\x0EV[P` \x82\x01Qat\x84` \x85\x01\x82aX\x0EV[PPPPV[_``\x82\x01\x90Pat\x9D_\x83\x01\x85aW\xC7V[at\xAA` \x83\x01\x84at]V[\x93\x92PPPV[_`@\x82\x01\x90Pat\xC4_\x83\x01\x85aW\xD6V[\x81\x81\x03` \x83\x01Rat\xD6\x81\x84aTWV[\x90P\x93\x92PPPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Rat\xF8\x81\x84\x86ad)V[\x90P\x93\x92PPPV[_`\x80\x83\x01_\x83\x01Qau\x16_\x86\x01\x82aj;V[P` \x83\x01Qau)` \x86\x01\x82aX\x0EV[P`@\x83\x01Qau<`@\x86\x01\x82aj;V[P``\x83\x01Q\x84\x82\x03``\x86\x01RauT\x82\x82an\xF6V[\x91PP\x80\x91PP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Rauy\x81\x85au\x01V[\x90P\x81\x81\x03` \x83\x01Rau\x8D\x81\x84au\x01V[\x90P\x93\x92PPPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[`\x1F\x82\x11\x15au\xE9Wau\xBA\x81au\x96V[au\xC3\x84a_\xD9V[\x81\x01` \x85\x10\x15au\xD2W\x81\x90P[au\xE6au\xDE\x85a_\xD9V[\x83\x01\x82a`\xB9V[PP[PPPV[au\xF7\x82aT\xD3V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15av\x10Wav\x0FaU\xABV[[av\x1A\x82Ta_\x8DV[av%\x82\x82\x85au\xA8V[_` \x90P`\x1F\x83\x11`\x01\x81\x14avVW_\x84\x15avDW\x82\x87\x01Q\x90P[avN\x85\x82aaIV[\x86UPav\xB5V[`\x1F\x19\x84\x16avd\x86au\x96V[_[\x82\x81\x10\x15av\x8BW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PavfV[\x86\x83\x10\x15av\xA8W\x84\x89\x01Qav\xA4`\x1F\x89\x16\x82aa-V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_` \x82\x01\x90Pav\xD0_\x83\x01\x84ai,V[\x92\x91PPV[`T\x81\x10av\xE7Wav\xE6a_\x1AV[[PV[_\x81\x90Pav\xF7\x82av\xD6V[\x91\x90PV[_aw\x06\x82av\xEAV[\x90P\x91\x90PV[aw\x16\x81av\xFCV[\x82RPPV[_` \x82\x01\x90Paw/_\x83\x01\x84aw\rV[\x92\x91PPV[_\x81\x90P\x92\x91PPV[awH\x81aT\x14V[\x82RPPV[_awY\x83\x83aw?V[` \x83\x01\x90P\x92\x91PPV[_awo\x82aS\xCCV[awy\x81\x85aw5V[\x93Paw\x84\x83aS\xE6V[\x80_[\x83\x81\x10\x15aw\xB4W\x81Qaw\x9B\x88\x82awNV[\x97Paw\xA6\x83aTKV[\x92PP`\x01\x81\x01\x90Paw\x87V[P\x85\x93PPPP\x92\x91PPV[_aw\xCC\x82\x84aweV[\x91P\x81\x90P\x92\x91PPV[_a\x01\0\x82\x01\x90Paw\xEB_\x83\x01\x8BaW2V[aw\xF8` \x83\x01\x8AaW2V[ax\x05`@\x83\x01\x89aW2V[ax\x12``\x83\x01\x88aW\xD6V[ax\x1F`\x80\x83\x01\x87aW\xC7V[ax,`\xA0\x83\x01\x86aW\xC7V[ax9`\xC0\x83\x01\x85aW\xC7V[axF`\xE0\x83\x01\x84aW2V[\x99\x98PPPPPPPPPV[_`\xE0\x82\x01\x90Paxf_\x83\x01\x8AaW2V[axs` \x83\x01\x89aW2V[ax\x80`@\x83\x01\x88aW2V[ax\x8D``\x83\x01\x87aW\xC7V[ax\x9A`\x80\x83\x01\x86aW\xC7V[ax\xA7`\xA0\x83\x01\x85aW\xC7V[ax\xB4`\xC0\x83\x01\x84aW2V[\x98\x97PPPPPPPPV[_`\xA0\x82\x01\x90Pax\xD3_\x83\x01\x88aW2V[ax\xE0` \x83\x01\x87aW2V[ax\xED`@\x83\x01\x86aW2V[ax\xFA``\x83\x01\x85aW\xC7V[ay\x07`\x80\x83\x01\x84aW\xD6V[\x96\x95PPPPPPV[_`\x80\x82\x01\x90Pay$_\x83\x01\x87aW2V[ay1` \x83\x01\x86ai,V[ay>`@\x83\x01\x85aW2V[ayK``\x83\x01\x84aW2V[\x95\x94PPPPPV\xFEUserDecryptResponseVerification(bytes publicKey,bytes32[] ctHandles,bytes userDecryptedShare,bytes extraData)PublicDecryptVerification(bytes32[] ctHandles,bytes decryptedResult,bytes extraData)UserDecryptRequestVerification(bytes publicKey,address[] contractAddresses,uint256 contractsChainId,uint256 startTimestamp,uint256 durationDays,bytes extraData)DelegatedUserDecryptRequestVerification(bytes publicKey,address[] contractAddresses,address delegatorAddress,uint256 contractsChainId,uint256 startTimestamp,uint256 durationDays,bytes extraData)",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405260043610610165575f3560e01c80638456cb59116100d0578063ad3cb1cc11610089578063e31ec19d11610063578063e31ec19d14610461578063f1b57adb14610489578063f2fde38b146104b1578063f4a6e2ce146104d957610165565b8063ad3cb1cc146103e5578063d8998f451461040f578063e30c39781461043757610165565b80638456cb59146102fd57806384b0196e146103135780638da5cb5b146103435780639f98d8991461036d5780639fad5a2f14610395578063a6090439146103bd57610165565b80634f1ef286116101225780634f1ef2861461023957806352d1902d146102555780635c975abb1461027f5780636f8913bc146102a9578063715018a6146102d157806379ba5097146102e757610165565b8063046f9eb3146101695780630900cc69146101915780630d8e6e2c146101cd578063123abb28146101f757806339f738101461020d5780633f4ba83a14610223575b5f80fd5b348015610174575f80fd5b5061018f600480360381019061018a91906152dd565b610501565b005b34801561019c575f80fd5b506101b760048036038101906101b291906153a1565b6109ba565b6040516101c491906154b3565b60405180910390f35b3480156101d8575f80fd5b506101e1610a8b565b6040516101ee919061555d565b60405180910390f35b348015610202575f80fd5b5061020b610b06565b005b348015610218575f80fd5b50610221610c93565b005b34801561022e575f80fd5b50610237610edb565b005b610253600480360381019061024e91906156cf565b611023565b005b348015610260575f80fd5b50610269611042565b6040516102769190615741565b60405180910390f35b34801561028a575f80fd5b50610293611073565b6040516102a09190615774565b60405180910390f35b3480156102b4575f80fd5b506102cf60048036038101906102ca91906152dd565b611095565b005b3480156102dc575f80fd5b506102e5611462565b005b3480156102f2575f80fd5b506102fb611475565b005b348015610308575f80fd5b50610311611503565b005b34801561031e575f80fd5b5061032761164b565b60405161033a979695949392919061589c565b60405180910390f35b34801561034e575f80fd5b50610357611754565b604051610364919061591e565b60405180910390f35b348015610378575f80fd5b50610393600480360381019061038e9190615a03565b611789565b005b3480156103a0575f80fd5b506103bb60048036038101906103b69190615b16565b611a28565b005b3480156103c8575f80fd5b506103e360048036038101906103de91906153a1565b611ff2565b005b3480156103f0575f80fd5b506103f9612062565b604051610406919061555d565b60405180910390f35b34801561041a575f80fd5b5061043560048036038101906104309190615ca6565b61209b565b005b348015610442575f80fd5b5061044b612259565b604051610458919061591e565b60405180910390f35b34801561046c575f80fd5b5061048760048036038101906104829190615ca6565b61228e565b005b348015610494575f80fd5b506104af60048036038101906104aa9190615d24565b6123d3565b005b3480156104bc575f80fd5b506104d760048036038101906104d29190615e5e565b612889565b005b3480156104e4575f80fd5b506104ff60048036038101906104fa9190615e89565b612942565b005b73c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff1663c6275258336040518263ffffffff1660e01b815260040161054e919061591e565b5f6040518083038186803b158015610564575f80fd5b505afa158015610576573d5f803e3d5ffd5b505050505f610583612b4e565b90505f8814806105b85750805f0154881180156105b7575060f8600260058111156105b1576105b0615f1a565b5b901b8811155b5b806105c6575080600c015488115b1561060857876040517fd48af9420000000000000000000000000000000000000000000000000000000081526004016105ff9190615f47565b60405180910390fd5b5f816006015f8a81526020019081526020015f206040518060400160405290815f8201805461063690615f8d565b80601f016020809104026020016040519081016040528092919081815260200182805461066290615f8d565b80156106ad5780601f10610684576101008083540402835291602001916106ad565b820191905f5260205f20905b81548152906001019060200180831161069057829003601f168201915b505050505081526020016001820180548060200260200160405190810160405280929190818152602001828054801561070357602002820191905f5260205f20905b8154815260200190600101908083116106ef575b50505050508152505090505f6040518060800160405280835f01518152602001836020015181526020018a8a8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050815260200186868080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081525090505f6107c982612b75565b90506107d78b828a8a612c3c565b5f846005015f8d81526020019081526020015f20905080898990918060018154018082558091505060019003905f5260205f20015f909192909192909192909192509182610826929190616164565b50846008015f8d81526020019081526020015f208b8b90918060018154018082558091505060019003905f5260205f20015f909192909192909192909192509182610872929190616164565b50846009015f8d81526020019081526020015f205f805f1b81526020019081526020015f2033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550846001015f8d81526020019081526020015f205f9054906101000a900460ff1615801561092b575061092a8180549050612e1d565b5b156109ac576001856001015f8e81526020019081526020015f205f6101000a81548160ff0219169083151502179055508b7f1cbf1c6e9ea3dbcfd49b9690d93cd4b93b3cb22913f99b71d3aecae60b25fc56866008015f8f81526020019081526020015f20838a8a6040516109a39493929190616455565b60405180910390a25b505050505050505050505050565b60605f6109c5612b4e565b90505f81600a015f8581526020019081526020015f20549050816009015f8581526020019081526020015f205f8281526020019081526020015f20805480602002602001604051908101604052809291908181526020018280548015610a7d57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311610a34575b505050505092505050919050565b60606040518060400160405280600a81526020017f44656372797074696f6e00000000000000000000000000000000000000000000815250610acc5f612eae565b610ad66004612eae565b610adf5f612eae565b604051602001610af2949392919061656f565b604051602081830303815290604052905090565b60065f610b11612f78565b9050805f0160089054906101000a900460ff1680610b5957508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15610b90576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055505f610bde612b4e565b905060f860016005811115610bf657610bf5615f1a565b5b901b815f0154610c0691906165fa565b81600b018190555060f860026005811115610c2457610c23615f1a565b5b901b815f0154610c3491906165fa565b81600c0181905550505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051610c87919061664f565b60405180910390a15050565b6001610c9d612f9f565b67ffffffffffffffff1614610cde576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60065f610ce9612f78565b9050805f0160089054906101000a900460ff1680610d3157508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15610d68576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff021916908315150217905550610e216040518060400160405280600a81526020017f44656372797074696f6e000000000000000000000000000000000000000000008152506040518060400160405280600181526020017f3100000000000000000000000000000000000000000000000000000000000000815250612fc3565b610e31610e2c611754565b612fd9565b610e39612fed565b5f610e42612b4e565b905060f860016005811115610e5a57610e59615f1a565b5b901b81600b018190555060f860026005811115610e7a57610e79615f1a565b5b901b81600c0181905550505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051610ecf919061664f565b60405180910390a15050565b73c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610f38573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f5c919061667c565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614158015610fd7575073c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614155b1561101957336040517fe19166ee000000000000000000000000000000000000000000000000000000008152600401611010919061591e565b60405180910390fd5b611021612fff565b565b61102b61306d565b61103482613153565b61103e8282613246565b5050565b5f61104b613364565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b5f8061107d6133eb565b9050805f015f9054906101000a900460ff1691505090565b73c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff1663c6275258336040518263ffffffff1660e01b81526004016110e2919061591e565b5f6040518083038186803b1580156110f8575f80fd5b505afa15801561110a573d5f803e3d5ffd5b505050505f611117612b4e565b90505f88148061114c5750805f01548811801561114b575060f86001600581111561114557611144615f1a565b5b901b8811155b5b8061115a575080600b015488115b1561119c57876040517fd48af9420000000000000000000000000000000000000000000000000000000081526004016111939190615f47565b60405180910390fd5b5f6040518060600160405280836004015f8c81526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561120357602002820191905f5260205f20905b8154815260200190600101908083116111ef575b5050505050815260200189898080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050815260200185858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081525090505f6112a982613412565b90506112b78a828989612c3c565b5f836003015f8c81526020019081526020015f205f8381526020019081526020015f20905080888890918060018154018082558091505060019003905f5260205f20015f909192909192909192909192509182611315929190616164565b50836009015f8c81526020019081526020015f205f8381526020019081526020015f2033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550836001015f8c81526020019081526020015f205f9054906101000a900460ff161580156113cc57506113cb81805490506134cc565b5b15611455576001846001015f8d81526020019081526020015f205f6101000a81548160ff0219169083151502179055508184600a015f8d81526020019081526020015f20819055508a7fd7e58a367a0a6c298e76ad5d240004e327aa1423cbe4bd7ff85d4c715ef8d15f8b8b848a8a60405161144c9594939291906166a7565b60405180910390a25b5050505050505050505050565b61146a61355d565b6114735f6135e4565b565b5f61147e613621565b90508073ffffffffffffffffffffffffffffffffffffffff1661149f612259565b73ffffffffffffffffffffffffffffffffffffffff16146114f757806040517f118cdaa70000000000000000000000000000000000000000000000000000000081526004016114ee919061591e565b60405180910390fd5b611500816135e4565b50565b73c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff16637008b5486040518163ffffffff1660e01b8152600401602060405180830381865afa158015611560573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611584919061667c565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16141580156115ff575073c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614155b1561164157336040517f388916bb000000000000000000000000000000000000000000000000000000008152600401611638919061591e565b60405180910390fd5b611649613628565b565b5f6060805f805f60605f61165d613697565b90505f801b815f015414801561167857505f801b8160010154145b6116b7576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016116ae9061673f565b60405180910390fd5b6116bf6136be565b6116c761375c565b46305f801b5f67ffffffffffffffff8111156116e6576116e56155ab565b5b6040519080825280602002602001820160405280156117145781602001602082028036833780820191505090505b507f0f0000000000000000000000000000000000000000000000000000000000000095949392919097509750975097509750975097505090919293949596565b5f8061175e6137fa565b9050805f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1691505090565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff166351c41d0e898987876040518563ffffffff1660e01b81526004016117dc9493929190616820565b5f6040518083038186803b1580156117f2575f80fd5b505afa158015611804573d5f803e3d5ffd5b505050505f5b86869050811015611a1d5773a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16633bce498d8888848181106118585761185761685e565b5b9050604002015f01358a5f0160208101906118739190615e5e565b6040518363ffffffff1660e01b815260040161189092919061688b565b5f6040518083038186803b1580156118a6575f80fd5b505afa1580156118b8573d5f803e3d5ffd5b5050505073a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16633bce498d8888848181106118ff576118fe61685e565b5b9050604002015f013589898581811061191b5761191a61685e565b5b90506040020160200160208101906119339190615e5e565b6040518363ffffffff1660e01b815260040161195092919061688b565b5f6040518083038186803b158015611966575f80fd5b505afa158015611978573d5f803e3d5ffd5b5050505073d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663d4476f638888848181106119bf576119be61685e565b5b9050604002015f01356040518263ffffffff1660e01b81526004016119e49190615741565b5f6040518083038186803b1580156119fa575f80fd5b505afa158015611a0c573d5f803e3d5ffd5b50505050808060010191505061180a565b505050505050505050565b611a30613821565b5f878060200190611a4191906168be565b905003611a7a576040517f57cfa21700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600a60ff16878060200190611a8f91906168be565b90501115611ae857600a878060200190611aa991906168be565b90506040517fc5ab467e000000000000000000000000000000000000000000000000000000008152600401611adf92919061693b565b60405180910390fd5b611b0189803603810190611afc91906169b7565b613862565b611b6a878060200190611b1491906168be565b808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f82011690508083019250505050505050895f016020810190611b659190615e5e565b6139ad565b15611bcf57875f016020810190611b819190615e5e565b878060200190611b9191906168be565b6040517fc3446ac7000000000000000000000000000000000000000000000000000000008152600401611bc6939291906169e2565b60405180910390fd5b5f611c3b8c8c8a8060200190611be591906168be565b808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f820116905080830192505050505050508c5f016020810190611c369190615e5e565b613a2b565b905073a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff166351c41d0e895f01358b8b8060200190611c8291906168be565b6040518563ffffffff1660e01b8152600401611ca19493929190616820565b5f6040518083038186803b158015611cb7575f80fd5b505afa158015611cc9573d5f803e3d5ffd5b505050505f6040518060e0016040528089898080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081526020018a8060200190611d3291906168be565b808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f8201169050808301925050505050505081526020018b5f016020810190611d889190615e5e565b73ffffffffffffffffffffffffffffffffffffffff1681526020018a5f013581526020018c5f013581526020018c60200135815260200185858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050508152509050611e26818b6020016020810190611e1f9190615e5e565b8888613d00565b5f73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663a14f8971846040518263ffffffff1660e01b8152600401611e749190616ac9565b5f60405180830381865afa158015611e8e573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f82011682018060405250810190611eb69190616d56565b9050611ec181613dd6565b5f611eca612b4e565b905080600c015f815480929190611ee090616d9d565b91905055505f81600c0154905060405180604001604052808c8c8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050815260200186815250826006015f8381526020019081526020015f205f820151815f019081611f6b9190616dee565b506020820151816001019080519060200190611f88929190615187565b50905050807ff9011bd6ba0da6049c520d70fe5971f17ed7ab795486052544b51019896c596b848f6020016020810190611fc29190615e5e565b8e8e8c8c604051611fd896959493929190617044565b60405180910390a250505050505050505050505050505050565b5f611ffb612b4e565b9050806001015f8381526020019081526020015f205f9054906101000a900460ff1661205e57816040517f0bf014060000000000000000000000000000000000000000000000000000000081526004016120559190615f47565b60405180910390fd5b5050565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b6120a3613821565b5f84849050036120df576040517f2de7543800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6121288484808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f82011690508083019250505050505050613ebc565b5f73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663a14f897186866040518363ffffffff1660e01b8152600401612178929190617108565b5f60405180830381865afa158015612192573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f820116820180604052508101906121ba9190616d56565b90506121c581613dd6565b5f6121ce612b4e565b905080600b015f8154809291906121e490616d9d565b91905055505f81600b015490508686836004015f8481526020019081526020015f2091906122139291906151d2565b50807f22db480a39bd72556438aadb4a32a3d2a6638b87c03bbec5fef6997e109587ff8487876040516122489392919061712a565b60405180910390a250505050505050565b5f80612263613fe4565b9050805f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1691505090565b5f5b848490508110156123cc5773a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663193f3f2c8686848181106122de576122dd61685e565b5b905060200201356040518263ffffffff1660e01b81526004016123019190615741565b5f6040518083038186803b158015612317575f80fd5b505afa158015612329573d5f803e3d5ffd5b5050505073d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663d4476f638686848181106123705761236f61685e565b5b905060200201356040518263ffffffff1660e01b81526004016123939190615741565b5f6040518083038186803b1580156123a9575f80fd5b505afa1580156123bb573d5f803e3d5ffd5b505050508080600101915050612290565b5050505050565b6123db613821565b5f8880602001906123ec91906168be565b905003612425576040517f57cfa21700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600a60ff1688806020019061243a91906168be565b9050111561249357600a88806020019061245491906168be565b90506040517fc5ab467e00000000000000000000000000000000000000000000000000000000815260040161248a92919061693b565b60405180910390fd5b6124ac898036038101906124a791906169b7565b613862565b6125048880602001906124bf91906168be565b808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f82011690508083019250505050505050886139ad565b15612558578688806020019061251a91906168be565b6040517fdc4d78b100000000000000000000000000000000000000000000000000000000815260040161254f939291906169e2565b60405180910390fd5b5f6125b38c8c8b806020019061256e91906168be565b808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f820116905080830192505050505050508b613a2b565b90505f6040518060c0016040528089898080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081526020018b806020019061261a91906168be565b808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f8201169050808301925050505050505081526020018b5f013581526020018c5f013581526020018c60200135815260200185858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081525090506126cf818a888861400b565b5f73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663a14f8971846040518263ffffffff1660e01b815260040161271d9190616ac9565b5f60405180830381865afa158015612737573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f8201168201806040525081019061275f9190616d56565b905061276a81613dd6565b5f612773612b4e565b905080600c015f81548092919061278990616d9d565b91905055505f81600c0154905060405180604001604052808c8c8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050815260200186815250826006015f8381526020019081526020015f205f820151815f0190816128149190616dee565b506020820151816001019080519060200190612831929190615187565b50905050807ff9011bd6ba0da6049c520d70fe5971f17ed7ab795486052544b51019896c596b848e8e8e8c8c60405161286f96959493929190617044565b60405180910390a250505050505050505050505050505050565b61289161355d565b5f61289a613fe4565b905081815f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508173ffffffffffffffffffffffffffffffffffffffff166128fc611754565b73ffffffffffffffffffffffffffffffffffffffff167f38d16b8cac22d99fc7c124b9cd0de2d3fa1faef420bfe791d8c362d765e2270060405160405180910390a35050565b5f5b84849050811015612b465773a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16633bce498d8686848181106129925761299161685e565b5b9050604002015f0135886040518363ffffffff1660e01b81526004016129b992919061688b565b5f6040518083038186803b1580156129cf575f80fd5b505afa1580156129e1573d5f803e3d5ffd5b5050505073a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16633bce498d868684818110612a2857612a2761685e565b5b9050604002015f0135878785818110612a4457612a4361685e565b5b9050604002016020016020810190612a5c9190615e5e565b6040518363ffffffff1660e01b8152600401612a7992919061688b565b5f6040518083038186803b158015612a8f575f80fd5b505afa158015612aa1573d5f803e3d5ffd5b5050505073d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663d4476f63868684818110612ae857612ae761685e565b5b9050604002015f01356040518263ffffffff1660e01b8152600401612b0d9190615741565b5f6040518083038186803b158015612b23575f80fd5b505afa158015612b35573d5f803e3d5ffd5b505050508080600101915050612944565b505050505050565b5f7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee700905090565b5f612c356040518060a00160405280606d8152602001617955606d913980519060200120835f0151805190602001208460200151604051602001612bb991906171ed565b604051602081830303815290604052805190602001208560400151805190602001208660600151604051602001612bf0919061723d565b60405160208183030381529060405280519060200120604051602001612c1a959493929190617253565b604051602081830303815290604052805190602001206140e1565b9050919050565b5f612c45612b4e565b90505f612c958585858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050506140fa565b905073c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff16636c88eb43826040518263ffffffff1660e01b8152600401612ce4919061591e565b5f6040518083038186803b158015612cfa575f80fd5b505afa158015612d0c573d5f803e3d5ffd5b50505050816002015f8781526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615612daf5785816040517f99ec48d9000000000000000000000000000000000000000000000000000000008152600401612da69291906172a4565b60405180910390fd5b6001826002015f8881526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550505050505050565b5f8073c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff1663c2b429866040518163ffffffff1660e01b8152600401602060405180830381865afa158015612e7c573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612ea091906172cb565b905080831015915050919050565b60605f6001612ebc84614124565b0190505f8167ffffffffffffffff811115612eda57612ed96155ab565b5b6040519080825280601f01601f191660200182016040528015612f0c5781602001600182028036833780820191505090505b5090505f82602001820190505b600115612f6d578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a8581612f6257612f616172f6565b5b0494505f8503612f19575b819350505050919050565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b5f612fa8612f78565b5f015f9054906101000a900467ffffffffffffffff16905090565b612fcb614275565b612fd582826142b5565b5050565b612fe1614275565b612fea81614306565b50565b612ff5614275565b612ffd61438a565b565b6130076143ba565b5f6130106133eb565b90505f815f015f6101000a81548160ff0219169083151502179055507f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa613055613621565b604051613062919061591e565b60405180910390a150565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff16148061311a57507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166131016143fa565b73ffffffffffffffffffffffffffffffffffffffff1614155b15613151576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b73c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156131b0573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906131d4919061667c565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461324357336040517f0e56cf3d00000000000000000000000000000000000000000000000000000000815260040161323a919061591e565b60405180910390fd5b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa9250505080156132ae57506040513d601f19601f820116820180604052508101906132ab9190617323565b60015b6132ef57816040517f4c9c8ce30000000000000000000000000000000000000000000000000000000081526004016132e6919061591e565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b811461335557806040517faa1d49a400000000000000000000000000000000000000000000000000000000815260040161334c9190615741565b60405180910390fd5b61335f838361444d565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff16146133e9576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300905090565b5f6134c56040518060800160405280605481526020016179c26054913980519060200120835f015160405160200161344a91906171ed565b604051602081830303815290604052805190602001208460200151805190602001208560400151604051602001613481919061723d565b604051602081830303815290604052805190602001206040516020016134aa949392919061734e565b604051602081830303815290604052805190602001206140e1565b9050919050565b5f8073c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff16632a3889986040518163ffffffff1660e01b8152600401602060405180830381865afa15801561352b573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061354f91906172cb565b905080831015915050919050565b613565613621565b73ffffffffffffffffffffffffffffffffffffffff16613583611754565b73ffffffffffffffffffffffffffffffffffffffff16146135e2576135a6613621565b6040517f118cdaa70000000000000000000000000000000000000000000000000000000081526004016135d9919061591e565b60405180910390fd5b565b5f6135ed613fe4565b9050805f015f6101000a81549073ffffffffffffffffffffffffffffffffffffffff021916905561361d826144bf565b5050565b5f33905090565b613630613821565b5f6136396133eb565b90506001815f015f6101000a81548160ff0219169083151502179055507f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a25861367f613621565b60405161368c919061591e565b60405180910390a150565b5f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100905090565b60605f6136c9613697565b90508060020180546136da90615f8d565b80601f016020809104026020016040519081016040528092919081815260200182805461370690615f8d565b80156137515780601f1061372857610100808354040283529160200191613751565b820191905f5260205f20905b81548152906001019060200180831161373457829003601f168201915b505050505091505090565b60605f613767613697565b905080600301805461377890615f8d565b80601f01602080910402602001604051908101604052809291908181526020018280546137a490615f8d565b80156137ef5780601f106137c6576101008083540402835291602001916137ef565b820191905f5260205f20905b8154815290600101906020018083116137d257829003601f168201915b505050505091505090565b5f7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300905090565b613829611073565b15613860576040517fd93c066500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f81602001510361389f576040517fde2859c100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61016d61ffff16816020015111156138f65761016d81602001516040517f329518630000000000000000000000000000000000000000000000000000000081526004016138ed9291906173ce565b60405180910390fd5b42815f015111156139435742815f01516040517ff24c088700000000000000000000000000000000000000000000000000000000815260040161393a9291906173f5565b60405180910390fd5b42620151808260200151613957919061741c565b825f015161396591906165fa565b10156139aa5742816040517f303480400000000000000000000000000000000000000000000000000000000081526004016139a192919061748a565b60405180910390fd5b50565b5f805f90505b8351811015613a20578273ffffffffffffffffffffffffffffffffffffffff168482815181106139e6576139e561685e565b5b602002602001015173ffffffffffffffffffffffffffffffffffffffff1603613a13576001915050613a25565b80806001019150506139b3565b505f90505b92915050565b60605f8585905003613a69576040517fa6a6cb2100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8484905067ffffffffffffffff811115613a8657613a856155ab565b5b604051908082528060200260200182016040528015613ab45781602001602082028036833780820191505090505b5090505f805b86869050811015613cab575f878783818110613ad957613ad861685e565b5b9050604002015f013590505f888884818110613af857613af761685e565b5b9050604002016020016020810190613b109190615e5e565b90505f613b1c83614590565b9050613b278161461a565b61ffff1685613b3691906165fa565b945073a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16633bce498d84896040518363ffffffff1660e01b8152600401613b8792919061688b565b5f6040518083038186803b158015613b9d575f80fd5b505afa158015613baf573d5f803e3d5ffd5b5050505073a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16633bce498d84846040518363ffffffff1660e01b8152600401613c0292919061688b565b5f6040518083038186803b158015613c18575f80fd5b505afa158015613c2a573d5f803e3d5ffd5b50505050613c3888836139ad565b613c7b5781886040517fa4c30391000000000000000000000000000000000000000000000000000000008152600401613c729291906174b1565b60405180910390fd5b82868581518110613c8f57613c8e61685e565b5b6020026020010181815250505050508080600101915050613aba565b50610800811115613cf757610800816040517fe7f4895d000000000000000000000000000000000000000000000000000000008152600401613cee9291906173f5565b60405180910390fd5b50949350505050565b5f613d0a856148a7565b90505f613d5a8285858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050506140fa565b90508473ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614613dce5783836040517f2a873d27000000000000000000000000000000000000000000000000000000008152600401613dc59291906174df565b60405180910390fd5b505050505050565b600181511115613eb9575f815f81518110613df457613df361685e565b5b60200260200101516020015190505f600190505b8251811015613eb65781838281518110613e2557613e2461685e565b5b60200260200101516020015114613ea957825f81518110613e4957613e4861685e565b5b6020026020010151838281518110613e6457613e6361685e565b5b60200260200101516040517fcfae921f000000000000000000000000000000000000000000000000000000008152600401613ea0929190617561565b60405180910390fd5b8080600101915050613e08565b50505b50565b5f805b8251811015613f94575f838281518110613edc57613edb61685e565b5b602002602001015190505f613ef082614590565b9050613efb8161461a565b61ffff1684613f0a91906165fa565b935073a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663193f3f2c836040518263ffffffff1660e01b8152600401613f599190615741565b5f6040518083038186803b158015613f6f575f80fd5b505afa158015613f81573d5f803e3d5ffd5b5050505050508080600101915050613ebf565b50610800811115613fe057610800816040517fe7f4895d000000000000000000000000000000000000000000000000000000008152600401613fd79291906173f5565b60405180910390fd5b5050565b5f7f237e158222e3e6968b72b9db0d8043aacf074ad9f650f0d1606b4d82ee432c00905090565b5f61401585614984565b90505f6140658285858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050506140fa565b90508473ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16146140d95783836040517f2a873d270000000000000000000000000000000000000000000000000000000081526004016140d09291906174df565b60405180910390fd5b505050505050565b5f6140f36140ed614a5a565b83614a68565b9050919050565b5f805f806141088686614aa8565b9250925092506141188282614afd565b82935050505092915050565b5f805f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f0100000000000000008310614180577a184f03e93ff9f4daa797ed6e38ed64bf6a1f0100000000000000008381614176576141756172f6565b5b0492506040810190505b6d04ee2d6d415b85acef810000000083106141bd576d04ee2d6d415b85acef810000000083816141b3576141b26172f6565b5b0492506020810190505b662386f26fc1000083106141ec57662386f26fc1000083816141e2576141e16172f6565b5b0492506010810190505b6305f5e1008310614215576305f5e100838161420b5761420a6172f6565b5b0492506008810190505b612710831061423a5761271083816142305761422f6172f6565b5b0492506004810190505b6064831061425d5760648381614253576142526172f6565b5b0492506002810190505b600a831061426c576001810190505b80915050919050565b61427d614c5f565b6142b3576040517fd7e6bcf800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b6142bd614275565b5f6142c6613697565b9050828160020190816142d991906175ee565b50818160030190816142eb91906175ee565b505f801b815f01819055505f801b8160010181905550505050565b61430e614275565b5f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff160361437e575f6040517f1e4fbdf7000000000000000000000000000000000000000000000000000000008152600401614375919061591e565b60405180910390fd5b614387816135e4565b50565b614392614275565b5f61439b6133eb565b90505f815f015f6101000a81548160ff02191690831515021790555050565b6143c2611073565b6143f8576040517f8dfc202b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f6144267f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b614c7d565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b61445682614c86565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f815111156144b2576144ac8282614d4f565b506144bb565b6144ba614dcf565b5b5050565b5f6144c86137fa565b90505f815f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905082825f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508273ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a3505050565b5f8060f860f084901b901c5f1c90506053808111156145b2576145b1615f1a565b5b60ff168160ff1611156145fc57806040517f641950d70000000000000000000000000000000000000000000000000000000081526004016145f391906176bd565b60405180910390fd5b8060ff16605381111561461257614611615f1a565b5b915050919050565b5f80605381111561462e5761462d615f1a565b5b82605381111561464157614640615f1a565b5b0361464f57600290506148a2565b6002605381111561466357614662615f1a565b5b82605381111561467657614675615f1a565b5b0361468457600890506148a2565b6003605381111561469857614697615f1a565b5b8260538111156146ab576146aa615f1a565b5b036146b957601090506148a2565b600460538111156146cd576146cc615f1a565b5b8260538111156146e0576146df615f1a565b5b036146ee57602090506148a2565b6005605381111561470257614701615f1a565b5b82605381111561471557614714615f1a565b5b0361472357604090506148a2565b6006605381111561473757614736615f1a565b5b82605381111561474a57614749615f1a565b5b0361475857608090506148a2565b6007605381111561476c5761476b615f1a565b5b82605381111561477f5761477e615f1a565b5b0361478d5760a090506148a2565b600860538111156147a1576147a0615f1a565b5b8260538111156147b4576147b3615f1a565b5b036147c35761010090506148a2565b600960538111156147d7576147d6615f1a565b5b8260538111156147ea576147e9615f1a565b5b036147f95761020090506148a2565b600a605381111561480d5761480c615f1a565b5b8260538111156148205761481f615f1a565b5b0361482f5761040090506148a2565b600b605381111561484357614842615f1a565b5b82605381111561485657614855615f1a565b5b036148655761080090506148a2565b816040517fbe7830b1000000000000000000000000000000000000000000000000000000008152600401614899919061771c565b60405180910390fd5b919050565b5f8060405180610100016040528060c28152602001617ab660c2913980519060200120835f01518051906020012084602001516040516020016148ea91906177c1565b604051602081830303815290604052805190602001208560400151866060015187608001518860a001518960c00151604051602001614929919061723d565b604051602081830303815290604052805190602001206040516020016149569897969594939291906177d7565b60405160208183030381529060405280519060200120905061497c836060015182614e0b565b915050919050565b5f806040518060c0016040528060a08152602001617a1660a0913980519060200120835f01518051906020012084602001516040516020016149c691906177c1565b604051602081830303815290604052805190602001208560400151866060015187608001518860a00151604051602001614a00919061723d565b60405160208183030381529060405280519060200120604051602001614a2c9796959493929190617853565b604051602081830303815290604052805190602001209050614a52836040015182614e0b565b915050919050565b5f614a63614e7f565b905090565b5f6040517f190100000000000000000000000000000000000000000000000000000000000081528360028201528260228201526042812091505092915050565b5f805f6041845103614ae8575f805f602087015192506040870151915060608701515f1a9050614ada88828585614ee2565b955095509550505050614af6565b5f600285515f1b9250925092505b9250925092565b5f6003811115614b1057614b0f615f1a565b5b826003811115614b2357614b22615f1a565b5b0315614c5b5760016003811115614b3d57614b3c615f1a565b5b826003811115614b5057614b4f615f1a565b5b03614b87576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60026003811115614b9b57614b9a615f1a565b5b826003811115614bae57614bad615f1a565b5b03614bf257805f1c6040517ffce698f7000000000000000000000000000000000000000000000000000000008152600401614be99190615f47565b60405180910390fd5b600380811115614c0557614c04615f1a565b5b826003811115614c1857614c17615f1a565b5b03614c5a57806040517fd78bce0c000000000000000000000000000000000000000000000000000000008152600401614c519190615741565b60405180910390fd5b5b5050565b5f614c68612f78565b5f0160089054906101000a900460ff16905090565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b03614ce157806040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401614cd8919061591e565b60405180910390fd5b80614d0d7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b614c7d565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f808473ffffffffffffffffffffffffffffffffffffffff1684604051614d78919061723d565b5f60405180830381855af49150503d805f8114614db0576040519150601f19603f3d011682016040523d82523d5f602084013e614db5565b606091505b5091509150614dc5858383614fc9565b9250505092915050565b5f341115614e09576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f807f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f614e36615056565b614e3e6150cc565b8630604051602001614e549594939291906178c0565b604051602081830303815290604052805190602001209050614e768184614a68565b91505092915050565b5f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f614ea9615056565b614eb16150cc565b4630604051602001614ec79594939291906178c0565b60405160208183030381529060405280519060200120905090565b5f805f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c1115614f1e575f600385925092509250614fbf565b5f6001888888886040515f8152602001604052604051614f419493929190617911565b6020604051602081039080840390855afa158015614f61573d5f803e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603614fb2575f60015f801b93509350935050614fbf565b805f805f1b935093509350505b9450945094915050565b606082614fde57614fd982615143565b61504e565b5f825114801561500457505f8473ffffffffffffffffffffffffffffffffffffffff163b145b1561504657836040517f9996b31500000000000000000000000000000000000000000000000000000000815260040161503d919061591e565b60405180910390fd5b81905061504f565b5b9392505050565b5f80615060613697565b90505f61506b6136be565b90505f81511115615087578080519060200120925050506150c9565b5f825f015490505f801b81146150a2578093505050506150c9565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f806150d6613697565b90505f6150e161375c565b90505f815111156150fd57808051906020012092505050615140565b5f826001015490505f801b811461511957809350505050615140565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f815111156151555780518082602001fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b828054828255905f5260205f209081019282156151c1579160200282015b828111156151c05782518255916020019190600101906151a5565b5b5090506151ce919061521d565b5090565b828054828255905f5260205f2090810192821561520c579160200282015b8281111561520b5782358255916020019190600101906151f0565b5b509050615219919061521d565b5090565b5b80821115615234575f815f90555060010161521e565b5090565b5f604051905090565b5f80fd5b5f80fd5b5f819050919050565b61525b81615249565b8114615265575f80fd5b50565b5f8135905061527681615252565b92915050565b5f80fd5b5f80fd5b5f80fd5b5f8083601f84011261529d5761529c61527c565b5b8235905067ffffffffffffffff8111156152ba576152b9615280565b5b6020830191508360018202830111156152d6576152d5615284565b5b9250929050565b5f805f805f805f6080888a0312156152f8576152f7615241565b5b5f6153058a828b01615268565b975050602088013567ffffffffffffffff81111561532657615325615245565b5b6153328a828b01615288565b9650965050604088013567ffffffffffffffff81111561535557615354615245565b5b6153618a828b01615288565b9450945050606088013567ffffffffffffffff81111561538457615383615245565b5b6153908a828b01615288565b925092505092959891949750929550565b5f602082840312156153b6576153b5615241565b5b5f6153c384828501615268565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61541e826153f5565b9050919050565b61542e81615414565b82525050565b5f61543f8383615425565b60208301905092915050565b5f602082019050919050565b5f615461826153cc565b61546b81856153d6565b9350615476836153e6565b805f5b838110156154a657815161548d8882615434565b97506154988361544b565b925050600181019050615479565b5085935050505092915050565b5f6020820190508181035f8301526154cb8184615457565b905092915050565b5f81519050919050565b5f82825260208201905092915050565b5f5b8381101561550a5780820151818401526020810190506154ef565b5f8484015250505050565b5f601f19601f8301169050919050565b5f61552f826154d3565b61553981856154dd565b93506155498185602086016154ed565b61555281615515565b840191505092915050565b5f6020820190508181035f8301526155758184615525565b905092915050565b61558681615414565b8114615590575f80fd5b50565b5f813590506155a18161557d565b92915050565b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6155e182615515565b810181811067ffffffffffffffff82111715615600576155ff6155ab565b5b80604052505050565b5f615612615238565b905061561e82826155d8565b919050565b5f67ffffffffffffffff82111561563d5761563c6155ab565b5b61564682615515565b9050602081019050919050565b828183375f83830152505050565b5f61567361566e84615623565b615609565b90508281526020810184848401111561568f5761568e6155a7565b5b61569a848285615653565b509392505050565b5f82601f8301126156b6576156b561527c565b5b81356156c6848260208601615661565b91505092915050565b5f80604083850312156156e5576156e4615241565b5b5f6156f285828601615593565b925050602083013567ffffffffffffffff81111561571357615712615245565b5b61571f858286016156a2565b9150509250929050565b5f819050919050565b61573b81615729565b82525050565b5f6020820190506157545f830184615732565b92915050565b5f8115159050919050565b61576e8161575a565b82525050565b5f6020820190506157875f830184615765565b92915050565b5f7fff0000000000000000000000000000000000000000000000000000000000000082169050919050565b6157c18161578d565b82525050565b6157d081615249565b82525050565b6157df81615414565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b61581781615249565b82525050565b5f615828838361580e565b60208301905092915050565b5f602082019050919050565b5f61584a826157e5565b61585481856157ef565b935061585f836157ff565b805f5b8381101561588f578151615876888261581d565b975061588183615834565b925050600181019050615862565b5085935050505092915050565b5f60e0820190506158af5f83018a6157b8565b81810360208301526158c18189615525565b905081810360408301526158d58188615525565b90506158e460608301876157c7565b6158f160808301866157d6565b6158fe60a0830185615732565b81810360c08301526159108184615840565b905098975050505050505050565b5f6020820190506159315f8301846157d6565b92915050565b5f80fd5b5f604082840312156159505761594f615937565b5b81905092915050565b5f8083601f84011261596e5761596d61527c565b5b8235905067ffffffffffffffff81111561598b5761598a615280565b5b6020830191508360408202830111156159a7576159a6615284565b5b9250929050565b5f8083601f8401126159c3576159c261527c565b5b8235905067ffffffffffffffff8111156159e0576159df615280565b5b6020830191508360208202830111156159fc576159fb615284565b5b9250929050565b5f805f805f805f8060c0898b031215615a1f57615a1e615241565b5b5f615a2c8b828c01615268565b9850506020615a3d8b828c0161593b565b975050606089013567ffffffffffffffff811115615a5e57615a5d615245565b5b615a6a8b828c01615959565b9650965050608089013567ffffffffffffffff811115615a8d57615a8c615245565b5b615a998b828c016159ae565b945094505060a089013567ffffffffffffffff811115615abc57615abb615245565b5b615ac88b828c01615288565b92509250509295985092959890939650565b5f60408284031215615aef57615aee615937565b5b81905092915050565b5f60408284031215615b0d57615b0c615937565b5b81905092915050565b5f805f805f805f805f805f6101208c8e031215615b3657615b35615241565b5b5f8c013567ffffffffffffffff811115615b5357615b52615245565b5b615b5f8e828f01615959565b9b509b50506020615b728e828f01615ada565b9950506060615b838e828f0161593b565b98505060a08c013567ffffffffffffffff811115615ba457615ba3615245565b5b615bb08e828f01615af8565b97505060c08c013567ffffffffffffffff811115615bd157615bd0615245565b5b615bdd8e828f01615288565b965096505060e08c013567ffffffffffffffff811115615c0057615bff615245565b5b615c0c8e828f01615288565b94509450506101008c013567ffffffffffffffff811115615c3057615c2f615245565b5b615c3c8e828f01615288565b92509250509295989b509295989b9093969950565b5f8083601f840112615c6657615c6561527c565b5b8235905067ffffffffffffffff811115615c8357615c82615280565b5b602083019150836020820283011115615c9f57615c9e615284565b5b9250929050565b5f805f8060408587031215615cbe57615cbd615241565b5b5f85013567ffffffffffffffff811115615cdb57615cda615245565b5b615ce787828801615c51565b9450945050602085013567ffffffffffffffff811115615d0a57615d09615245565b5b615d1687828801615288565b925092505092959194509250565b5f805f805f805f805f805f6101008c8e031215615d4457615d43615241565b5b5f8c013567ffffffffffffffff811115615d6157615d60615245565b5b615d6d8e828f01615959565b9b509b50506020615d808e828f01615ada565b99505060608c013567ffffffffffffffff811115615da157615da0615245565b5b615dad8e828f01615af8565b9850506080615dbe8e828f01615593565b97505060a08c013567ffffffffffffffff811115615ddf57615dde615245565b5b615deb8e828f01615288565b965096505060c08c013567ffffffffffffffff811115615e0e57615e0d615245565b5b615e1a8e828f01615288565b945094505060e08c013567ffffffffffffffff811115615e3d57615e3c615245565b5b615e498e828f01615288565b92509250509295989b509295989b9093969950565b5f60208284031215615e7357615e72615241565b5b5f615e8084828501615593565b91505092915050565b5f805f805f60608688031215615ea257615ea1615241565b5b5f615eaf88828901615593565b955050602086013567ffffffffffffffff811115615ed057615ecf615245565b5b615edc88828901615959565b9450945050604086013567ffffffffffffffff811115615eff57615efe615245565b5b615f0b88828901615288565b92509250509295509295909350565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b5f602082019050615f5a5f8301846157c7565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f6002820490506001821680615fa457607f821691505b602082108103615fb757615fb6615f60565b5b50919050565b5f82905092915050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f600883026160237fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82615fe8565b61602d8683615fe8565b95508019841693508086168417925050509392505050565b5f819050919050565b5f61606861606361605e84615249565b616045565b615249565b9050919050565b5f819050919050565b6160818361604e565b61609561608d8261606f565b848454615ff4565b825550505050565b5f90565b6160a961609d565b6160b4818484616078565b505050565b5b818110156160d7576160cc5f826160a1565b6001810190506160ba565b5050565b601f82111561611c576160ed81615fc7565b6160f684615fd9565b81016020851015616105578190505b61611961611185615fd9565b8301826160b9565b50505b505050565b5f82821c905092915050565b5f61613c5f1984600802616121565b1980831691505092915050565b5f616154838361612d565b9150826002028217905092915050565b61616e8383615fbd565b67ffffffffffffffff811115616187576161866155ab565b5b6161918254615f8d565b61619c8282856160db565b5f601f8311600181146161c9575f84156161b7578287013590505b6161c18582616149565b865550616228565b601f1984166161d786615fc7565b5f5b828110156161fe578489013582556001820191506020850194506020810190506161d9565b8683101561621b5784890135616217601f89168261612d565b8355505b6001600288020188555050505b50505050505050565b5f81549050919050565b5f82825260208201905092915050565b5f819050815f5260205f209050919050565b5f82825260208201905092915050565b5f815461627981615f8d565b616283818661625d565b9450600182165f811461629d57600181146162b3576162e5565b60ff1983168652811515602002860193506162e5565b6162bc85615fc7565b5f5b838110156162dd578154818901526001820191506020810190506162be565b808801955050505b50505092915050565b5f6162f9838361626d565b905092915050565b5f600182019050919050565b5f61631782616231565b616321818561623b565b9350836020820285016163338561624b565b805f5b8581101561636d5784840389528161634e85826162ee565b945061635983616301565b925060208a01995050600181019050616336565b50829750879550505050505092915050565b5f81549050919050565b5f819050815f5260205f209050919050565b5f600182019050919050565b5f6163b18261637f565b6163bb818561623b565b9350836020820285016163cd85616389565b805f5b85811015616407578484038952816163e885826162ee565b94506163f38361639b565b925060208a019950506001810190506163d0565b50829750879550505050505092915050565b5f82825260208201905092915050565b5f6164348385616419565b9350616441838584615653565b61644a83615515565b840190509392505050565b5f6060820190508181035f83015261646d818761630d565b9050818103602083015261648181866163a7565b90508181036040830152616496818486616429565b905095945050505050565b5f81905092915050565b5f6164b5826154d3565b6164bf81856164a1565b93506164cf8185602086016154ed565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f61650f6002836164a1565b915061651a826164db565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f6165596001836164a1565b915061656482616525565b600182019050919050565b5f61657a82876164ab565b915061658582616503565b915061659182866164ab565b915061659c8261654d565b91506165a882856164ab565b91506165b38261654d565b91506165bf82846164ab565b915081905095945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61660482615249565b915061660f83615249565b9250828201905080821115616627576166266165cd565b5b92915050565b5f67ffffffffffffffff82169050919050565b6166498161662d565b82525050565b5f6020820190506166625f830184616640565b92915050565b5f815190506166768161557d565b92915050565b5f6020828403121561669157616690615241565b5b5f61669e84828501616668565b91505092915050565b5f6060820190508181035f8301526166c0818789616429565b905081810360208301526166d481866163a7565b905081810360408301526166e9818486616429565b90509695505050505050565b7f4549503731323a20556e696e697469616c697a656400000000000000000000005f82015250565b5f6167296015836154dd565b9150616734826166f5565b602082019050919050565b5f6020820190508181035f8301526167568161671d565b9050919050565b5f61676b6020840184615593565b905092915050565b604082016167835f83018361675d565b61678f5f850182615425565b5061679d602083018361675d565b6167aa6020850182615425565b50505050565b5f819050919050565b5f602082019050919050565b5f6167d083856153d6565b93506167db826167b0565b805f5b85811015616813576167f0828461675d565b6167fa8882615434565b9750616805836167b9565b9250506001810190506167de565b5085925050509392505050565b5f6080820190506168335f8301876157c7565b6168406020830186616773565b81810360608301526168538184866167c5565b905095945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f60408201905061689e5f830185615732565b6168ab60208301846157d6565b9392505050565b5f80fd5b5f80fd5b5f80fd5b5f80833560016020038436030381126168da576168d96168b2565b5b80840192508235915067ffffffffffffffff8211156168fc576168fb6168b6565b5b602083019250602082023603831315616918576169176168ba565b5b509250929050565b5f60ff82169050919050565b61693581616920565b82525050565b5f60408201905061694e5f83018561692c565b61695b60208301846157c7565b9392505050565b5f80fd5b5f80fd5b5f6040828403121561697f5761697e616962565b5b6169896040615609565b90505f61699884828501615268565b5f8301525060206169ab84828501615268565b60208301525092915050565b5f604082840312156169cc576169cb615241565b5b5f6169d98482850161696a565b91505092915050565b5f6040820190506169f55f8301866157d6565b8181036020830152616a088184866167c5565b9050949350505050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b616a4481615729565b82525050565b5f616a558383616a3b565b60208301905092915050565b5f602082019050919050565b5f616a7782616a12565b616a818185616a1c565b9350616a8c83616a2c565b805f5b83811015616abc578151616aa38882616a4a565b9750616aae83616a61565b925050600181019050616a8f565b5085935050505092915050565b5f6020820190508181035f830152616ae18184616a6d565b905092915050565b5f67ffffffffffffffff821115616b0357616b026155ab565b5b602082029050602081019050919050565b616b1d81615729565b8114616b27575f80fd5b50565b5f81519050616b3881616b14565b92915050565b5f81519050616b4c81615252565b92915050565b5f67ffffffffffffffff821115616b6c57616b6b6155ab565b5b602082029050602081019050919050565b5f616b8f616b8a84616b52565b615609565b90508083825260208201905060208402830185811115616bb257616bb1615284565b5b835b81811015616bdb5780616bc78882616668565b845260208401935050602081019050616bb4565b5050509392505050565b5f82601f830112616bf957616bf861527c565b5b8151616c09848260208601616b7d565b91505092915050565b5f60808284031215616c2757616c26616962565b5b616c316080615609565b90505f616c4084828501616b2a565b5f830152506020616c5384828501616b3e565b6020830152506040616c6784828501616b2a565b604083015250606082015167ffffffffffffffff811115616c8b57616c8a616966565b5b616c9784828501616be5565b60608301525092915050565b5f616cb5616cb084616ae9565b615609565b90508083825260208201905060208402830185811115616cd857616cd7615284565b5b835b81811015616d1f57805167ffffffffffffffff811115616cfd57616cfc61527c565b5b808601616d0a8982616c12565b85526020850194505050602081019050616cda565b5050509392505050565b5f82601f830112616d3d57616d3c61527c565b5b8151616d4d848260208601616ca3565b91505092915050565b5f60208284031215616d6b57616d6a615241565b5b5f82015167ffffffffffffffff811115616d8857616d87615245565b5b616d9484828501616d29565b91505092915050565b5f616da782615249565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203616dd957616dd86165cd565b5b600182019050919050565b5f81519050919050565b616df782616de4565b67ffffffffffffffff811115616e1057616e0f6155ab565b5b616e1a8254615f8d565b616e258282856160db565b5f60209050601f831160018114616e56575f8415616e44578287015190505b616e4e8582616149565b865550616eb5565b601f198416616e6486615fc7565b5f5b82811015616e8b57848901518255600182019150602085019450602081019050616e66565b86831015616ea85784890151616ea4601f89168261612d565b8355505b6001600288020188555050505b505050505050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f82825260208201905092915050565b5f616f00826153cc565b616f0a8185616ee6565b9350616f15836153e6565b805f5b83811015616f45578151616f2c8882615434565b9750616f378361544b565b925050600181019050616f18565b5085935050505092915050565b5f608083015f830151616f675f860182616a3b565b506020830151616f7a602086018261580e565b506040830151616f8d6040860182616a3b565b5060608301518482036060860152616fa58282616ef6565b9150508091505092915050565b5f616fbd8383616f52565b905092915050565b5f602082019050919050565b5f616fdb82616ebd565b616fe58185616ec7565b935083602082028501616ff785616ed7565b805f5b8581101561703257848403895281516170138582616fb2565b945061701e83616fc5565b925060208a01995050600181019050616ffa565b50829750879550505050505092915050565b5f6080820190508181035f83015261705c8189616fd1565b905061706b60208301886157d6565b818103604083015261707e818688616429565b90508181036060830152617093818486616429565b9050979650505050505050565b5f80fd5b82818337505050565b5f6170b88385616a1c565b93507f07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8311156170eb576170ea6170a0565b5b6020830292506170fc8385846170a4565b82840190509392505050565b5f6020820190508181035f8301526171218184866170ad565b90509392505050565b5f6040820190508181035f8301526171428186616fd1565b90508181036020830152617157818486616429565b9050949350505050565b5f81905092915050565b61717481615729565b82525050565b5f617185838361716b565b60208301905092915050565b5f61719b82616a12565b6171a58185617161565b93506171b083616a2c565b805f5b838110156171e05781516171c7888261717a565b97506171d283616a61565b9250506001810190506171b3565b5085935050505092915050565b5f6171f88284617191565b915081905092915050565b5f81905092915050565b5f61721782616de4565b6172218185617203565b93506172318185602086016154ed565b80840191505092915050565b5f617248828461720d565b915081905092915050565b5f60a0820190506172665f830188615732565b6172736020830187615732565b6172806040830186615732565b61728d6060830185615732565b61729a6080830184615732565b9695505050505050565b5f6040820190506172b75f8301856157c7565b6172c460208301846157d6565b9392505050565b5f602082840312156172e0576172df615241565b5b5f6172ed84828501616b3e565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f6020828403121561733857617337615241565b5b5f61734584828501616b2a565b91505092915050565b5f6080820190506173615f830187615732565b61736e6020830186615732565b61737b6040830185615732565b6173886060830184615732565b95945050505050565b5f61ffff82169050919050565b5f6173b86173b36173ae84617391565b616045565b615249565b9050919050565b6173c88161739e565b82525050565b5f6040820190506173e15f8301856173bf565b6173ee60208301846157c7565b9392505050565b5f6040820190506174085f8301856157c7565b61741560208301846157c7565b9392505050565b5f61742682615249565b915061743183615249565b925082820261743f81615249565b91508282048414831517617456576174556165cd565b5b5092915050565b604082015f8201516174715f85018261580e565b506020820151617484602085018261580e565b50505050565b5f60608201905061749d5f8301856157c7565b6174aa602083018461745d565b9392505050565b5f6040820190506174c45f8301856157d6565b81810360208301526174d68184615457565b90509392505050565b5f6020820190508181035f8301526174f8818486616429565b90509392505050565b5f608083015f8301516175165f860182616a3b565b506020830151617529602086018261580e565b50604083015161753c6040860182616a3b565b50606083015184820360608601526175548282616ef6565b9150508091505092915050565b5f6040820190508181035f8301526175798185617501565b9050818103602083015261758d8184617501565b90509392505050565b5f819050815f5260205f209050919050565b601f8211156175e9576175ba81617596565b6175c384615fd9565b810160208510156175d2578190505b6175e66175de85615fd9565b8301826160b9565b50505b505050565b6175f7826154d3565b67ffffffffffffffff8111156176105761760f6155ab565b5b61761a8254615f8d565b6176258282856175a8565b5f60209050601f831160018114617656575f8415617644578287015190505b61764e8582616149565b8655506176b5565b601f19841661766486617596565b5f5b8281101561768b57848901518255600182019150602085019450602081019050617666565b868310156176a857848901516176a4601f89168261612d565b8355505b6001600288020188555050505b505050505050565b5f6020820190506176d05f83018461692c565b92915050565b605481106176e7576176e6615f1a565b5b50565b5f8190506176f7826176d6565b919050565b5f617706826176ea565b9050919050565b617716816176fc565b82525050565b5f60208201905061772f5f83018461770d565b92915050565b5f81905092915050565b61774881615414565b82525050565b5f617759838361773f565b60208301905092915050565b5f61776f826153cc565b6177798185617735565b9350617784836153e6565b805f5b838110156177b457815161779b888261774e565b97506177a68361544b565b925050600181019050617787565b5085935050505092915050565b5f6177cc8284617765565b915081905092915050565b5f610100820190506177eb5f83018b615732565b6177f8602083018a615732565b6178056040830189615732565b61781260608301886157d6565b61781f60808301876157c7565b61782c60a08301866157c7565b61783960c08301856157c7565b61784660e0830184615732565b9998505050505050505050565b5f60e0820190506178665f83018a615732565b6178736020830189615732565b6178806040830188615732565b61788d60608301876157c7565b61789a60808301866157c7565b6178a760a08301856157c7565b6178b460c0830184615732565b98975050505050505050565b5f60a0820190506178d35f830188615732565b6178e06020830187615732565b6178ed6040830186615732565b6178fa60608301856157c7565b61790760808301846157d6565b9695505050505050565b5f6080820190506179245f830187615732565b617931602083018661692c565b61793e6040830185615732565b61794b6060830184615732565b9594505050505056fe5573657244656372797074526573706f6e7365566572696669636174696f6e286279746573207075626c69634b65792c627974657333325b5d20637448616e646c65732c6279746573207573657244656372797074656453686172652c627974657320657874726144617461295075626c696344656372797074566572696669636174696f6e28627974657333325b5d20637448616e646c65732c627974657320646563727970746564526573756c742c62797465732065787472614461746129557365724465637279707452657175657374566572696669636174696f6e286279746573207075626c69634b65792c616464726573735b5d20636f6e74726163744164647265737365732c75696e7432353620636f6e747261637473436861696e49642c75696e7432353620737461727454696d657374616d702c75696e74323536206475726174696f6e446179732c6279746573206578747261446174612944656c656761746564557365724465637279707452657175657374566572696669636174696f6e286279746573207075626c69634b65792c616464726573735b5d20636f6e74726163744164647265737365732c616464726573732064656c656761746f72416464726573732c75696e7432353620636f6e747261637473436861696e49642c75696e7432353620737461727454696d657374616d702c75696e74323536206475726174696f6e446179732c62797465732065787472614461746129
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01eW_5`\xE0\x1C\x80c\x84V\xCBY\x11a\0\xD0W\x80c\xAD<\xB1\xCC\x11a\0\x89W\x80c\xE3\x1E\xC1\x9D\x11a\0cW\x80c\xE3\x1E\xC1\x9D\x14a\x04aW\x80c\xF1\xB5z\xDB\x14a\x04\x89W\x80c\xF2\xFD\xE3\x8B\x14a\x04\xB1W\x80c\xF4\xA6\xE2\xCE\x14a\x04\xD9Wa\x01eV[\x80c\xAD<\xB1\xCC\x14a\x03\xE5W\x80c\xD8\x99\x8FE\x14a\x04\x0FW\x80c\xE3\x0C9x\x14a\x047Wa\x01eV[\x80c\x84V\xCBY\x14a\x02\xFDW\x80c\x84\xB0\x19n\x14a\x03\x13W\x80c\x8D\xA5\xCB[\x14a\x03CW\x80c\x9F\x98\xD8\x99\x14a\x03mW\x80c\x9F\xADZ/\x14a\x03\x95W\x80c\xA6\t\x049\x14a\x03\xBDWa\x01eV[\x80cO\x1E\xF2\x86\x11a\x01\"W\x80cO\x1E\xF2\x86\x14a\x029W\x80cR\xD1\x90-\x14a\x02UW\x80c\\\x97Z\xBB\x14a\x02\x7FW\x80co\x89\x13\xBC\x14a\x02\xA9W\x80cqP\x18\xA6\x14a\x02\xD1W\x80cy\xBAP\x97\x14a\x02\xE7Wa\x01eV[\x80c\x04o\x9E\xB3\x14a\x01iW\x80c\t\0\xCCi\x14a\x01\x91W\x80c\r\x8En,\x14a\x01\xCDW\x80c\x12:\xBB(\x14a\x01\xF7W\x80c9\xF78\x10\x14a\x02\rW\x80c?K\xA8:\x14a\x02#W[_\x80\xFD[4\x80\x15a\x01tW_\x80\xFD[Pa\x01\x8F`\x04\x806\x03\x81\x01\x90a\x01\x8A\x91\x90aR\xDDV[a\x05\x01V[\0[4\x80\x15a\x01\x9CW_\x80\xFD[Pa\x01\xB7`\x04\x806\x03\x81\x01\x90a\x01\xB2\x91\x90aS\xA1V[a\t\xBAV[`@Qa\x01\xC4\x91\x90aT\xB3V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xD8W_\x80\xFD[Pa\x01\xE1a\n\x8BV[`@Qa\x01\xEE\x91\x90aU]V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x02W_\x80\xFD[Pa\x02\x0Ba\x0B\x06V[\0[4\x80\x15a\x02\x18W_\x80\xFD[Pa\x02!a\x0C\x93V[\0[4\x80\x15a\x02.W_\x80\xFD[Pa\x027a\x0E\xDBV[\0[a\x02S`\x04\x806\x03\x81\x01\x90a\x02N\x91\x90aV\xCFV[a\x10#V[\0[4\x80\x15a\x02`W_\x80\xFD[Pa\x02ia\x10BV[`@Qa\x02v\x91\x90aWAV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x8AW_\x80\xFD[Pa\x02\x93a\x10sV[`@Qa\x02\xA0\x91\x90aWtV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xB4W_\x80\xFD[Pa\x02\xCF`\x04\x806\x03\x81\x01\x90a\x02\xCA\x91\x90aR\xDDV[a\x10\x95V[\0[4\x80\x15a\x02\xDCW_\x80\xFD[Pa\x02\xE5a\x14bV[\0[4\x80\x15a\x02\xF2W_\x80\xFD[Pa\x02\xFBa\x14uV[\0[4\x80\x15a\x03\x08W_\x80\xFD[Pa\x03\x11a\x15\x03V[\0[4\x80\x15a\x03\x1EW_\x80\xFD[Pa\x03'a\x16KV[`@Qa\x03:\x97\x96\x95\x94\x93\x92\x91\x90aX\x9CV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03NW_\x80\xFD[Pa\x03Wa\x17TV[`@Qa\x03d\x91\x90aY\x1EV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03xW_\x80\xFD[Pa\x03\x93`\x04\x806\x03\x81\x01\x90a\x03\x8E\x91\x90aZ\x03V[a\x17\x89V[\0[4\x80\x15a\x03\xA0W_\x80\xFD[Pa\x03\xBB`\x04\x806\x03\x81\x01\x90a\x03\xB6\x91\x90a[\x16V[a\x1A(V[\0[4\x80\x15a\x03\xC8W_\x80\xFD[Pa\x03\xE3`\x04\x806\x03\x81\x01\x90a\x03\xDE\x91\x90aS\xA1V[a\x1F\xF2V[\0[4\x80\x15a\x03\xF0W_\x80\xFD[Pa\x03\xF9a bV[`@Qa\x04\x06\x91\x90aU]V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\x1AW_\x80\xFD[Pa\x045`\x04\x806\x03\x81\x01\x90a\x040\x91\x90a\\\xA6V[a \x9BV[\0[4\x80\x15a\x04BW_\x80\xFD[Pa\x04Ka\"YV[`@Qa\x04X\x91\x90aY\x1EV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04lW_\x80\xFD[Pa\x04\x87`\x04\x806\x03\x81\x01\x90a\x04\x82\x91\x90a\\\xA6V[a\"\x8EV[\0[4\x80\x15a\x04\x94W_\x80\xFD[Pa\x04\xAF`\x04\x806\x03\x81\x01\x90a\x04\xAA\x91\x90a]$V[a#\xD3V[\0[4\x80\x15a\x04\xBCW_\x80\xFD[Pa\x04\xD7`\x04\x806\x03\x81\x01\x90a\x04\xD2\x91\x90a^^V[a(\x89V[\0[4\x80\x15a\x04\xE4W_\x80\xFD[Pa\x04\xFF`\x04\x806\x03\x81\x01\x90a\x04\xFA\x91\x90a^\x89V[a)BV[\0[s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC6'RX3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05N\x91\x90aY\x1EV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x05dW_\x80\xFD[PZ\xFA\x15\x80\x15a\x05vW=_\x80>=_\xFD[PPPP_a\x05\x83a+NV[\x90P_\x88\x14\x80a\x05\xB8WP\x80_\x01T\x88\x11\x80\x15a\x05\xB7WP`\xF8`\x02`\x05\x81\x11\x15a\x05\xB1Wa\x05\xB0a_\x1AV[[\x90\x1B\x88\x11\x15[[\x80a\x05\xC6WP\x80`\x0C\x01T\x88\x11[\x15a\x06\x08W\x87`@Q\x7F\xD4\x8A\xF9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05\xFF\x91\x90a_GV[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x8A\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80Ta\x066\x90a_\x8DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06b\x90a_\x8DV[\x80\x15a\x06\xADW\x80`\x1F\x10a\x06\x84Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xADV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x90W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07\x03W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x06\xEFW[PPPPP\x81RPP\x90P_`@Q\x80`\x80\x01`@R\x80\x83_\x01Q\x81R` \x01\x83` \x01Q\x81R` \x01\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81RP\x90P_a\x07\xC9\x82a+uV[\x90Pa\x07\xD7\x8B\x82\x8A\x8Aa,<V[_\x84`\x05\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x90P\x80\x89\x89\x90\x91\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x92\x90\x91\x92\x90\x91\x92\x90\x91\x92P\x91\x82a\x08&\x92\x91\x90aadV[P\x84`\x08\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x8B\x8B\x90\x91\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x92\x90\x91\x92\x90\x91\x92\x90\x91\x92P\x91\x82a\x08r\x92\x91\x90aadV[P\x84`\t\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _\x80_\x1B\x81R` \x01\x90\x81R` \x01_ 3\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x84`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\t+WPa\t*\x81\x80T\x90Pa.\x1DV[[\x15a\t\xACW`\x01\x85`\x01\x01_\x8E\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x8B\x7F\x1C\xBF\x1Cn\x9E\xA3\xDB\xCF\xD4\x9B\x96\x90\xD9<\xD4\xB9;<\xB2)\x13\xF9\x9Bq\xD3\xAE\xCA\xE6\x0B%\xFCV\x86`\x08\x01_\x8F\x81R` \x01\x90\x81R` \x01_ \x83\x8A\x8A`@Qa\t\xA3\x94\x93\x92\x91\x90adUV[`@Q\x80\x91\x03\x90\xA2[PPPPPPPPPPPPV[``_a\t\xC5a+NV[\x90P_\x81`\n\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\t\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x82\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\n}W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\n4W[PPPPP\x92PPP\x91\x90PV[```@Q\x80`@\x01`@R\x80`\n\x81R` \x01\x7FDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\n\xCC_a.\xAEV[a\n\xD6`\x04a.\xAEV[a\n\xDF_a.\xAEV[`@Q` \x01a\n\xF2\x94\x93\x92\x91\x90aeoV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`\x06_a\x0B\x11a/xV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x0BYWP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x0B\x90W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_a\x0B\xDEa+NV[\x90P`\xF8`\x01`\x05\x81\x11\x15a\x0B\xF6Wa\x0B\xF5a_\x1AV[[\x90\x1B\x81_\x01Ta\x0C\x06\x91\x90ae\xFAV[\x81`\x0B\x01\x81\x90UP`\xF8`\x02`\x05\x81\x11\x15a\x0C$Wa\x0C#a_\x1AV[[\x90\x1B\x81_\x01Ta\x0C4\x91\x90ae\xFAV[\x81`\x0C\x01\x81\x90UPP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x0C\x87\x91\x90afOV[`@Q\x80\x91\x03\x90\xA1PPV[`\x01a\x0C\x9Da/\x9FV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0C\xDEW`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x06_a\x0C\xE9a/xV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\r1WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\rhW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x0E!`@Q\x80`@\x01`@R\x80`\n\x81R` \x01\x7FDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa/\xC3V[a\x0E1a\x0E,a\x17TV[a/\xD9V[a\x0E9a/\xEDV[_a\x0EBa+NV[\x90P`\xF8`\x01`\x05\x81\x11\x15a\x0EZWa\x0EYa_\x1AV[[\x90\x1B\x81`\x0B\x01\x81\x90UP`\xF8`\x02`\x05\x81\x11\x15a\x0EzWa\x0Eya_\x1AV[[\x90\x1B\x81`\x0C\x01\x81\x90UPP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x0E\xCF\x91\x90afOV[`@Q\x80\x91\x03\x90\xA1PPV[s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F8W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\\\x91\x90af|V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\x0F\xD7WPs\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a\x10\x19W3`@Q\x7F\xE1\x91f\xEE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\x10\x91\x90aY\x1EV[`@Q\x80\x91\x03\x90\xFD[a\x10!a/\xFFV[V[a\x10+a0mV[a\x104\x82a1SV[a\x10>\x82\x82a2FV[PPV[_a\x10Ka3dV[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[_\x80a\x10}a3\xEBV[\x90P\x80_\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x90V[s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC6'RX3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\xE2\x91\x90aY\x1EV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x10\xF8W_\x80\xFD[PZ\xFA\x15\x80\x15a\x11\nW=_\x80>=_\xFD[PPPP_a\x11\x17a+NV[\x90P_\x88\x14\x80a\x11LWP\x80_\x01T\x88\x11\x80\x15a\x11KWP`\xF8`\x01`\x05\x81\x11\x15a\x11EWa\x11Da_\x1AV[[\x90\x1B\x88\x11\x15[[\x80a\x11ZWP\x80`\x0B\x01T\x88\x11[\x15a\x11\x9CW\x87`@Q\x7F\xD4\x8A\xF9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\x93\x91\x90a_GV[`@Q\x80\x91\x03\x90\xFD[_`@Q\x80``\x01`@R\x80\x83`\x04\x01_\x8C\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x12\x03W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x11\xEFW[PPPPP\x81R` \x01\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81RP\x90P_a\x12\xA9\x82a4\x12V[\x90Pa\x12\xB7\x8A\x82\x89\x89a,<V[_\x83`\x03\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x90P\x80\x88\x88\x90\x91\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x92\x90\x91\x92\x90\x91\x92\x90\x91\x92P\x91\x82a\x13\x15\x92\x91\x90aadV[P\x83`\t\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ 3\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x83`\x01\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x13\xCCWPa\x13\xCB\x81\x80T\x90Pa4\xCCV[[\x15a\x14UW`\x01\x84`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x81\x84`\n\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x8A\x7F\xD7\xE5\x8A6z\nl)\x8Ev\xAD]$\0\x04\xE3'\xAA\x14#\xCB\xE4\xBD\x7F\xF8]Lq^\xF8\xD1_\x8B\x8B\x84\x8A\x8A`@Qa\x14L\x95\x94\x93\x92\x91\x90af\xA7V[`@Q\x80\x91\x03\x90\xA2[PPPPPPPPPPPV[a\x14ja5]V[a\x14s_a5\xE4V[V[_a\x14~a6!V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x14\x9Fa\"YV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x14\xF7W\x80`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14\xEE\x91\x90aY\x1EV[`@Q\x80\x91\x03\x90\xFD[a\x15\0\x81a5\xE4V[PV[s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cp\x08\xB5H`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15`W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x84\x91\x90af|V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\x15\xFFWPs\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a\x16AW3`@Q\x7F8\x89\x16\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x168\x91\x90aY\x1EV[`@Q\x80\x91\x03\x90\xFD[a\x16Ia6(V[V[_``\x80_\x80_``_a\x16]a6\x97V[\x90P_\x80\x1B\x81_\x01T\x14\x80\x15a\x16xWP_\x80\x1B\x81`\x01\x01T\x14[a\x16\xB7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16\xAE\x90ag?V[`@Q\x80\x91\x03\x90\xFD[a\x16\xBFa6\xBEV[a\x16\xC7a7\\V[F0_\x80\x1B_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\xE6Wa\x16\xE5aU\xABV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\x14W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94\x93\x92\x91\x90\x97P\x97P\x97P\x97P\x97P\x97P\x97PP\x90\x91\x92\x93\x94\x95\x96V[_\x80a\x17^a7\xFAV[\x90P\x80_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91PP\x90V[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cQ\xC4\x1D\x0E\x89\x89\x87\x87`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\xDC\x94\x93\x92\x91\x90ah V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x17\xF2W_\x80\xFD[PZ\xFA\x15\x80\x15a\x18\x04W=_\x80>=_\xFD[PPPP_[\x86\x86\x90P\x81\x10\x15a\x1A\x1DWs\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c;\xCEI\x8D\x88\x88\x84\x81\x81\x10a\x18XWa\x18Wah^V[[\x90P`@\x02\x01_\x015\x8A_\x01` \x81\x01\x90a\x18s\x91\x90a^^V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\x90\x92\x91\x90ah\x8BV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x18\xA6W_\x80\xFD[PZ\xFA\x15\x80\x15a\x18\xB8W=_\x80>=_\xFD[PPPPs\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c;\xCEI\x8D\x88\x88\x84\x81\x81\x10a\x18\xFFWa\x18\xFEah^V[[\x90P`@\x02\x01_\x015\x89\x89\x85\x81\x81\x10a\x19\x1BWa\x19\x1Aah^V[[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x193\x91\x90a^^V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19P\x92\x91\x90ah\x8BV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x19fW_\x80\xFD[PZ\xFA\x15\x80\x15a\x19xW=_\x80>=_\xFD[PPPPs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD4Goc\x88\x88\x84\x81\x81\x10a\x19\xBFWa\x19\xBEah^V[[\x90P`@\x02\x01_\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19\xE4\x91\x90aWAV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x19\xFAW_\x80\xFD[PZ\xFA\x15\x80\x15a\x1A\x0CW=_\x80>=_\xFD[PPPP\x80\x80`\x01\x01\x91PPa\x18\nV[PPPPPPPPPV[a\x1A0a8!V[_\x87\x80` \x01\x90a\x1AA\x91\x90ah\xBEV[\x90P\x03a\x1AzW`@Q\x7FW\xCF\xA2\x17\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\n`\xFF\x16\x87\x80` \x01\x90a\x1A\x8F\x91\x90ah\xBEV[\x90P\x11\x15a\x1A\xE8W`\n\x87\x80` \x01\x90a\x1A\xA9\x91\x90ah\xBEV[\x90P`@Q\x7F\xC5\xABF~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1A\xDF\x92\x91\x90ai;V[`@Q\x80\x91\x03\x90\xFD[a\x1B\x01\x89\x806\x03\x81\x01\x90a\x1A\xFC\x91\x90ai\xB7V[a8bV[a\x1Bj\x87\x80` \x01\x90a\x1B\x14\x91\x90ah\xBEV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x89_\x01` \x81\x01\x90a\x1Be\x91\x90a^^V[a9\xADV[\x15a\x1B\xCFW\x87_\x01` \x81\x01\x90a\x1B\x81\x91\x90a^^V[\x87\x80` \x01\x90a\x1B\x91\x91\x90ah\xBEV[`@Q\x7F\xC3Dj\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1B\xC6\x93\x92\x91\x90ai\xE2V[`@Q\x80\x91\x03\x90\xFD[_a\x1C;\x8C\x8C\x8A\x80` \x01\x90a\x1B\xE5\x91\x90ah\xBEV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x8C_\x01` \x81\x01\x90a\x1C6\x91\x90a^^V[a:+V[\x90Ps\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cQ\xC4\x1D\x0E\x89_\x015\x8B\x8B\x80` \x01\x90a\x1C\x82\x91\x90ah\xBEV[`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1C\xA1\x94\x93\x92\x91\x90ah V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1C\xB7W_\x80\xFD[PZ\xFA\x15\x80\x15a\x1C\xC9W=_\x80>=_\xFD[PPPP_`@Q\x80`\xE0\x01`@R\x80\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8A\x80` \x01\x90a\x1D2\x91\x90ah\xBEV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8B_\x01` \x81\x01\x90a\x1D\x88\x91\x90a^^V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x8A_\x015\x81R` \x01\x8C_\x015\x81R` \x01\x8C` \x015\x81R` \x01\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81RP\x90Pa\x1E&\x81\x8B` \x01` \x81\x01\x90a\x1E\x1F\x91\x90a^^V[\x88\x88a=\0V[_s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA1O\x89q\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1Et\x91\x90aj\xC9V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\x8EW=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xB6\x91\x90amVV[\x90Pa\x1E\xC1\x81a=\xD6V[_a\x1E\xCAa+NV[\x90P\x80`\x0C\x01_\x81T\x80\x92\x91\x90a\x1E\xE0\x90am\x9DV[\x91\x90PUP_\x81`\x0C\x01T\x90P`@Q\x80`@\x01`@R\x80\x8C\x8C\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x86\x81RP\x82`\x06\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01\x90\x81a\x1Fk\x91\x90am\xEEV[P` \x82\x01Q\x81`\x01\x01\x90\x80Q\x90` \x01\x90a\x1F\x88\x92\x91\x90aQ\x87V[P\x90PP\x80\x7F\xF9\x01\x1B\xD6\xBA\r\xA6\x04\x9CR\rp\xFEYq\xF1~\xD7\xAByT\x86\x05%D\xB5\x10\x19\x89lYk\x84\x8F` \x01` \x81\x01\x90a\x1F\xC2\x91\x90a^^V[\x8E\x8E\x8C\x8C`@Qa\x1F\xD8\x96\x95\x94\x93\x92\x91\x90apDV[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPPPPPV[_a\x1F\xFBa+NV[\x90P\x80`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a ^W\x81`@Q\x7F\x0B\xF0\x14\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a U\x91\x90a_GV[`@Q\x80\x91\x03\x90\xFD[PPV[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[a \xA3a8!V[_\x84\x84\x90P\x03a \xDFW`@Q\x7F-\xE7T8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a!(\x84\x84\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa>\xBCV[_s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA1O\x89q\x86\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a!x\x92\x91\x90aq\x08V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\x92W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xBA\x91\x90amVV[\x90Pa!\xC5\x81a=\xD6V[_a!\xCEa+NV[\x90P\x80`\x0B\x01_\x81T\x80\x92\x91\x90a!\xE4\x90am\x9DV[\x91\x90PUP_\x81`\x0B\x01T\x90P\x86\x86\x83`\x04\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x91\x90a\"\x13\x92\x91\x90aQ\xD2V[P\x80\x7F\"\xDBH\n9\xBDrUd8\xAA\xDBJ2\xA3\xD2\xA6c\x8B\x87\xC0;\xBE\xC5\xFE\xF6\x99~\x10\x95\x87\xFF\x84\x87\x87`@Qa\"H\x93\x92\x91\x90aq*V[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[_\x80a\"ca?\xE4V[\x90P\x80_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91PP\x90V[_[\x84\x84\x90P\x81\x10\x15a#\xCCWs\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x19??,\x86\x86\x84\x81\x81\x10a\"\xDEWa\"\xDDah^V[[\x90P` \x02\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a#\x01\x91\x90aWAV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a#\x17W_\x80\xFD[PZ\xFA\x15\x80\x15a#)W=_\x80>=_\xFD[PPPPs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD4Goc\x86\x86\x84\x81\x81\x10a#pWa#oah^V[[\x90P` \x02\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a#\x93\x91\x90aWAV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a#\xA9W_\x80\xFD[PZ\xFA\x15\x80\x15a#\xBBW=_\x80>=_\xFD[PPPP\x80\x80`\x01\x01\x91PPa\"\x90V[PPPPPV[a#\xDBa8!V[_\x88\x80` \x01\x90a#\xEC\x91\x90ah\xBEV[\x90P\x03a$%W`@Q\x7FW\xCF\xA2\x17\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\n`\xFF\x16\x88\x80` \x01\x90a$:\x91\x90ah\xBEV[\x90P\x11\x15a$\x93W`\n\x88\x80` \x01\x90a$T\x91\x90ah\xBEV[\x90P`@Q\x7F\xC5\xABF~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a$\x8A\x92\x91\x90ai;V[`@Q\x80\x91\x03\x90\xFD[a$\xAC\x89\x806\x03\x81\x01\x90a$\xA7\x91\x90ai\xB7V[a8bV[a%\x04\x88\x80` \x01\x90a$\xBF\x91\x90ah\xBEV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x88a9\xADV[\x15a%XW\x86\x88\x80` \x01\x90a%\x1A\x91\x90ah\xBEV[`@Q\x7F\xDCMx\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a%O\x93\x92\x91\x90ai\xE2V[`@Q\x80\x91\x03\x90\xFD[_a%\xB3\x8C\x8C\x8B\x80` \x01\x90a%n\x91\x90ah\xBEV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x8Ba:+V[\x90P_`@Q\x80`\xC0\x01`@R\x80\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8B\x80` \x01\x90a&\x1A\x91\x90ah\xBEV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8B_\x015\x81R` \x01\x8C_\x015\x81R` \x01\x8C` \x015\x81R` \x01\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81RP\x90Pa&\xCF\x81\x8A\x88\x88a@\x0BV[_s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA1O\x89q\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a'\x1D\x91\x90aj\xC9V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'7W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'_\x91\x90amVV[\x90Pa'j\x81a=\xD6V[_a'sa+NV[\x90P\x80`\x0C\x01_\x81T\x80\x92\x91\x90a'\x89\x90am\x9DV[\x91\x90PUP_\x81`\x0C\x01T\x90P`@Q\x80`@\x01`@R\x80\x8C\x8C\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x86\x81RP\x82`\x06\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01\x90\x81a(\x14\x91\x90am\xEEV[P` \x82\x01Q\x81`\x01\x01\x90\x80Q\x90` \x01\x90a(1\x92\x91\x90aQ\x87V[P\x90PP\x80\x7F\xF9\x01\x1B\xD6\xBA\r\xA6\x04\x9CR\rp\xFEYq\xF1~\xD7\xAByT\x86\x05%D\xB5\x10\x19\x89lYk\x84\x8E\x8E\x8E\x8C\x8C`@Qa(o\x96\x95\x94\x93\x92\x91\x90apDV[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPPPPPV[a(\x91a5]V[_a(\x9Aa?\xE4V[\x90P\x81\x81_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a(\xFCa\x17TV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F8\xD1k\x8C\xAC\"\xD9\x9F\xC7\xC1$\xB9\xCD\r\xE2\xD3\xFA\x1F\xAE\xF4 \xBF\xE7\x91\xD8\xC3b\xD7e\xE2'\0`@Q`@Q\x80\x91\x03\x90\xA3PPV[_[\x84\x84\x90P\x81\x10\x15a+FWs\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c;\xCEI\x8D\x86\x86\x84\x81\x81\x10a)\x92Wa)\x91ah^V[[\x90P`@\x02\x01_\x015\x88`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a)\xB9\x92\x91\x90ah\x8BV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a)\xCFW_\x80\xFD[PZ\xFA\x15\x80\x15a)\xE1W=_\x80>=_\xFD[PPPPs\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c;\xCEI\x8D\x86\x86\x84\x81\x81\x10a*(Wa*'ah^V[[\x90P`@\x02\x01_\x015\x87\x87\x85\x81\x81\x10a*DWa*Cah^V[[\x90P`@\x02\x01` \x01` \x81\x01\x90a*\\\x91\x90a^^V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a*y\x92\x91\x90ah\x8BV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a*\x8FW_\x80\xFD[PZ\xFA\x15\x80\x15a*\xA1W=_\x80>=_\xFD[PPPPs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD4Goc\x86\x86\x84\x81\x81\x10a*\xE8Wa*\xE7ah^V[[\x90P`@\x02\x01_\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a+\r\x91\x90aWAV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a+#W_\x80\xFD[PZ\xFA\x15\x80\x15a+5W=_\x80>=_\xFD[PPPP\x80\x80`\x01\x01\x91PPa)DV[PPPPPPV[_\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\0\x90P\x90V[_a,5`@Q\x80`\xA0\x01`@R\x80`m\x81R` \x01ayU`m\x919\x80Q\x90` \x01 \x83_\x01Q\x80Q\x90` \x01 \x84` \x01Q`@Q` \x01a+\xB9\x91\x90aq\xEDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85`@\x01Q\x80Q\x90` \x01 \x86``\x01Q`@Q` \x01a+\xF0\x91\x90ar=V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a,\x1A\x95\x94\x93\x92\x91\x90arSV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a@\xE1V[\x90P\x91\x90PV[_a,Ea+NV[\x90P_a,\x95\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa@\xFAV[\x90Ps\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cl\x88\xEBC\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\xE4\x91\x90aY\x1EV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a,\xFAW_\x80\xFD[PZ\xFA\x15\x80\x15a-\x0CW=_\x80>=_\xFD[PPPP\x81`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a-\xAFW\x85\x81`@Q\x7F\x99\xECH\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a-\xA6\x92\x91\x90ar\xA4V[`@Q\x80\x91\x03\x90\xFD[`\x01\x82`\x02\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPPPPPPV[_\x80s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC2\xB4)\x86`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.|W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\xA0\x91\x90ar\xCBV[\x90P\x80\x83\x10\x15\x91PP\x91\x90PV[``_`\x01a.\xBC\x84aA$V[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.\xDAWa.\xD9aU\xABV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a/\x0CW\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x01\x82\x01\x90P[`\x01\x15a/mW\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a/bWa/aar\xF6V[[\x04\x94P_\x85\x03a/\x19W[\x81\x93PPPP\x91\x90PV[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[_a/\xA8a/xV[_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a/\xCBaBuV[a/\xD5\x82\x82aB\xB5V[PPV[a/\xE1aBuV[a/\xEA\x81aC\x06V[PV[a/\xF5aBuV[a/\xFDaC\x8AV[V[a0\x07aC\xBAV[_a0\x10a3\xEBV[\x90P_\x81_\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAAa0Ua6!V[`@Qa0b\x91\x90aY\x1EV[`@Q\x80\x91\x03\x90\xA1PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a1\x1AWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a1\x01aC\xFAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a1QW`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1\xB0W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1\xD4\x91\x90af|V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a2CW3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a2:\x91\x90aY\x1EV[`@Q\x80\x91\x03\x90\xFD[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a2\xAEWP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2\xAB\x91\x90as#V[`\x01[a2\xEFW\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a2\xE6\x91\x90aY\x1EV[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14a3UW\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a3L\x91\x90aWAV[`@Q\x80\x91\x03\x90\xFD[a3_\x83\x83aDMV[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a3\xE9W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0\x90P\x90V[_a4\xC5`@Q\x80`\x80\x01`@R\x80`T\x81R` \x01ay\xC2`T\x919\x80Q\x90` \x01 \x83_\x01Q`@Q` \x01a4J\x91\x90aq\xEDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q\x80Q\x90` \x01 \x85`@\x01Q`@Q` \x01a4\x81\x91\x90ar=V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a4\xAA\x94\x93\x92\x91\x90asNV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a@\xE1V[\x90P\x91\x90PV[_\x80s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c*8\x89\x98`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5+W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5O\x91\x90ar\xCBV[\x90P\x80\x83\x10\x15\x91PP\x91\x90PV[a5ea6!V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a5\x83a\x17TV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a5\xE2Wa5\xA6a6!V[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a5\xD9\x91\x90aY\x1EV[`@Q\x80\x91\x03\x90\xFD[V[_a5\xEDa?\xE4V[\x90P\x80_\x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90Ua6\x1D\x82aD\xBFV[PPV[_3\x90P\x90V[a60a8!V[_a69a3\xEBV[\x90P`\x01\x81_\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2Xa6\x7Fa6!V[`@Qa6\x8C\x91\x90aY\x1EV[`@Q\x80\x91\x03\x90\xA1PV[_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x90P\x90V[``_a6\xC9a6\x97V[\x90P\x80`\x02\x01\x80Ta6\xDA\x90a_\x8DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta7\x06\x90a_\x8DV[\x80\x15a7QW\x80`\x1F\x10a7(Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a7QV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a74W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[``_a7ga6\x97V[\x90P\x80`\x03\x01\x80Ta7x\x90a_\x8DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta7\xA4\x90a_\x8DV[\x80\x15a7\xEFW\x80`\x1F\x10a7\xC6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a7\xEFV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a7\xD2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[_\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\x90P\x90V[a8)a\x10sV[\x15a8`W`@Q\x7F\xD9<\x06e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x81` \x01Q\x03a8\x9FW`@Q\x7F\xDE(Y\xC1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01ma\xFF\xFF\x16\x81` \x01Q\x11\x15a8\xF6Wa\x01m\x81` \x01Q`@Q\x7F2\x95\x18c\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a8\xED\x92\x91\x90as\xCEV[`@Q\x80\x91\x03\x90\xFD[B\x81_\x01Q\x11\x15a9CWB\x81_\x01Q`@Q\x7F\xF2L\x08\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a9:\x92\x91\x90as\xF5V[`@Q\x80\x91\x03\x90\xFD[Bb\x01Q\x80\x82` \x01Qa9W\x91\x90at\x1CV[\x82_\x01Qa9e\x91\x90ae\xFAV[\x10\x15a9\xAAWB\x81`@Q\x7F04\x80@\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a9\xA1\x92\x91\x90at\x8AV[`@Q\x80\x91\x03\x90\xFD[PV[_\x80_\x90P[\x83Q\x81\x10\x15a: W\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x82\x81Q\x81\x10a9\xE6Wa9\xE5ah^V[[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a:\x13W`\x01\x91PPa:%V[\x80\x80`\x01\x01\x91PPa9\xB3V[P_\x90P[\x92\x91PPV[``_\x85\x85\x90P\x03a:iW`@Q\x7F\xA6\xA6\xCB!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84\x84\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:\x86Wa:\x85aU\xABV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a:\xB4W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x80[\x86\x86\x90P\x81\x10\x15a<\xABW_\x87\x87\x83\x81\x81\x10a:\xD9Wa:\xD8ah^V[[\x90P`@\x02\x01_\x015\x90P_\x88\x88\x84\x81\x81\x10a:\xF8Wa:\xF7ah^V[[\x90P`@\x02\x01` \x01` \x81\x01\x90a;\x10\x91\x90a^^V[\x90P_a;\x1C\x83aE\x90V[\x90Pa;'\x81aF\x1AV[a\xFF\xFF\x16\x85a;6\x91\x90ae\xFAV[\x94Ps\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c;\xCEI\x8D\x84\x89`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a;\x87\x92\x91\x90ah\x8BV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a;\x9DW_\x80\xFD[PZ\xFA\x15\x80\x15a;\xAFW=_\x80>=_\xFD[PPPPs\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c;\xCEI\x8D\x84\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a<\x02\x92\x91\x90ah\x8BV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a<\x18W_\x80\xFD[PZ\xFA\x15\x80\x15a<*W=_\x80>=_\xFD[PPPPa<8\x88\x83a9\xADV[a<{W\x81\x88`@Q\x7F\xA4\xC3\x03\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a<r\x92\x91\x90at\xB1V[`@Q\x80\x91\x03\x90\xFD[\x82\x86\x85\x81Q\x81\x10a<\x8FWa<\x8Eah^V[[` \x02` \x01\x01\x81\x81RPPPPP\x80\x80`\x01\x01\x91PPa:\xBAV[Pa\x08\0\x81\x11\x15a<\xF7Wa\x08\0\x81`@Q\x7F\xE7\xF4\x89]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a<\xEE\x92\x91\x90as\xF5V[`@Q\x80\x91\x03\x90\xFD[P\x94\x93PPPPV[_a=\n\x85aH\xA7V[\x90P_a=Z\x82\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa@\xFAV[\x90P\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a=\xCEW\x83\x83`@Q\x7F*\x87='\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a=\xC5\x92\x91\x90at\xDFV[`@Q\x80\x91\x03\x90\xFD[PPPPPPV[`\x01\x81Q\x11\x15a>\xB9W_\x81_\x81Q\x81\x10a=\xF4Wa=\xF3ah^V[[` \x02` \x01\x01Q` \x01Q\x90P_`\x01\x90P[\x82Q\x81\x10\x15a>\xB6W\x81\x83\x82\x81Q\x81\x10a>%Wa>$ah^V[[` \x02` \x01\x01Q` \x01Q\x14a>\xA9W\x82_\x81Q\x81\x10a>IWa>Hah^V[[` \x02` \x01\x01Q\x83\x82\x81Q\x81\x10a>dWa>cah^V[[` \x02` \x01\x01Q`@Q\x7F\xCF\xAE\x92\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a>\xA0\x92\x91\x90auaV[`@Q\x80\x91\x03\x90\xFD[\x80\x80`\x01\x01\x91PPa>\x08V[PP[PV[_\x80[\x82Q\x81\x10\x15a?\x94W_\x83\x82\x81Q\x81\x10a>\xDCWa>\xDBah^V[[` \x02` \x01\x01Q\x90P_a>\xF0\x82aE\x90V[\x90Pa>\xFB\x81aF\x1AV[a\xFF\xFF\x16\x84a?\n\x91\x90ae\xFAV[\x93Ps\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x19??,\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a?Y\x91\x90aWAV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a?oW_\x80\xFD[PZ\xFA\x15\x80\x15a?\x81W=_\x80>=_\xFD[PPPPPP\x80\x80`\x01\x01\x91PPa>\xBFV[Pa\x08\0\x81\x11\x15a?\xE0Wa\x08\0\x81`@Q\x7F\xE7\xF4\x89]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a?\xD7\x92\x91\x90as\xF5V[`@Q\x80\x91\x03\x90\xFD[PPV[_\x7F#~\x15\x82\"\xE3\xE6\x96\x8Br\xB9\xDB\r\x80C\xAA\xCF\x07J\xD9\xF6P\xF0\xD1`kM\x82\xEEC,\0\x90P\x90V[_a@\x15\x85aI\x84V[\x90P_a@e\x82\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa@\xFAV[\x90P\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a@\xD9W\x83\x83`@Q\x7F*\x87='\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a@\xD0\x92\x91\x90at\xDFV[`@Q\x80\x91\x03\x90\xFD[PPPPPPV[_a@\xF3a@\xEDaJZV[\x83aJhV[\x90P\x91\x90PV[_\x80_\x80aA\x08\x86\x86aJ\xA8V[\x92P\x92P\x92PaA\x18\x82\x82aJ\xFDV[\x82\x93PPPP\x92\x91PPV[_\x80_\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10aA\x80Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81aAvWaAuar\xF6V[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10aA\xBDWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81aA\xB3WaA\xB2ar\xF6V[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10aA\xECWf#\x86\xF2o\xC1\0\0\x83\x81aA\xE2WaA\xE1ar\xF6V[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10aB\x15Wc\x05\xF5\xE1\0\x83\x81aB\x0BWaB\nar\xF6V[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10aB:Wa'\x10\x83\x81aB0WaB/ar\xF6V[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10aB]W`d\x83\x81aBSWaBRar\xF6V[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10aBlW`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[aB}aL_V[aB\xB3W`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[aB\xBDaBuV[_aB\xC6a6\x97V[\x90P\x82\x81`\x02\x01\x90\x81aB\xD9\x91\x90au\xEEV[P\x81\x81`\x03\x01\x90\x81aB\xEB\x91\x90au\xEEV[P_\x80\x1B\x81_\x01\x81\x90UP_\x80\x1B\x81`\x01\x01\x81\x90UPPPPV[aC\x0EaBuV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03aC~W_`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aCu\x91\x90aY\x1EV[`@Q\x80\x91\x03\x90\xFD[aC\x87\x81a5\xE4V[PV[aC\x92aBuV[_aC\x9Ba3\xEBV[\x90P_\x81_\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPV[aC\xC2a\x10sV[aC\xF8W`@Q\x7F\x8D\xFC +\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_aD&\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1BaL}V[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[aDV\x82aL\x86V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15aD\xB2WaD\xAC\x82\x82aMOV[PaD\xBBV[aD\xBAaM\xCFV[[PPV[_aD\xC8a7\xFAV[\x90P_\x81_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x82\x82_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPPV[_\x80`\xF8`\xF0\x84\x90\x1B\x90\x1C_\x1C\x90P`S\x80\x81\x11\x15aE\xB2WaE\xB1a_\x1AV[[`\xFF\x16\x81`\xFF\x16\x11\x15aE\xFCW\x80`@Q\x7Fd\x19P\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aE\xF3\x91\x90av\xBDV[`@Q\x80\x91\x03\x90\xFD[\x80`\xFF\x16`S\x81\x11\x15aF\x12WaF\x11a_\x1AV[[\x91PP\x91\x90PV[_\x80`S\x81\x11\x15aF.WaF-a_\x1AV[[\x82`S\x81\x11\x15aFAWaF@a_\x1AV[[\x03aFOW`\x02\x90PaH\xA2V[`\x02`S\x81\x11\x15aFcWaFba_\x1AV[[\x82`S\x81\x11\x15aFvWaFua_\x1AV[[\x03aF\x84W`\x08\x90PaH\xA2V[`\x03`S\x81\x11\x15aF\x98WaF\x97a_\x1AV[[\x82`S\x81\x11\x15aF\xABWaF\xAAa_\x1AV[[\x03aF\xB9W`\x10\x90PaH\xA2V[`\x04`S\x81\x11\x15aF\xCDWaF\xCCa_\x1AV[[\x82`S\x81\x11\x15aF\xE0WaF\xDFa_\x1AV[[\x03aF\xEEW` \x90PaH\xA2V[`\x05`S\x81\x11\x15aG\x02WaG\x01a_\x1AV[[\x82`S\x81\x11\x15aG\x15WaG\x14a_\x1AV[[\x03aG#W`@\x90PaH\xA2V[`\x06`S\x81\x11\x15aG7WaG6a_\x1AV[[\x82`S\x81\x11\x15aGJWaGIa_\x1AV[[\x03aGXW`\x80\x90PaH\xA2V[`\x07`S\x81\x11\x15aGlWaGka_\x1AV[[\x82`S\x81\x11\x15aG\x7FWaG~a_\x1AV[[\x03aG\x8DW`\xA0\x90PaH\xA2V[`\x08`S\x81\x11\x15aG\xA1WaG\xA0a_\x1AV[[\x82`S\x81\x11\x15aG\xB4WaG\xB3a_\x1AV[[\x03aG\xC3Wa\x01\0\x90PaH\xA2V[`\t`S\x81\x11\x15aG\xD7WaG\xD6a_\x1AV[[\x82`S\x81\x11\x15aG\xEAWaG\xE9a_\x1AV[[\x03aG\xF9Wa\x02\0\x90PaH\xA2V[`\n`S\x81\x11\x15aH\rWaH\x0Ca_\x1AV[[\x82`S\x81\x11\x15aH WaH\x1Fa_\x1AV[[\x03aH/Wa\x04\0\x90PaH\xA2V[`\x0B`S\x81\x11\x15aHCWaHBa_\x1AV[[\x82`S\x81\x11\x15aHVWaHUa_\x1AV[[\x03aHeWa\x08\0\x90PaH\xA2V[\x81`@Q\x7F\xBEx0\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aH\x99\x91\x90aw\x1CV[`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[_\x80`@Q\x80a\x01\0\x01`@R\x80`\xC2\x81R` \x01az\xB6`\xC2\x919\x80Q\x90` \x01 \x83_\x01Q\x80Q\x90` \x01 \x84` \x01Q`@Q` \x01aH\xEA\x91\x90aw\xC1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85`@\x01Q\x86``\x01Q\x87`\x80\x01Q\x88`\xA0\x01Q\x89`\xC0\x01Q`@Q` \x01aI)\x91\x90ar=V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aIV\x98\x97\x96\x95\x94\x93\x92\x91\x90aw\xD7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90PaI|\x83``\x01Q\x82aN\x0BV[\x91PP\x91\x90PV[_\x80`@Q\x80`\xC0\x01`@R\x80`\xA0\x81R` \x01az\x16`\xA0\x919\x80Q\x90` \x01 \x83_\x01Q\x80Q\x90` \x01 \x84` \x01Q`@Q` \x01aI\xC6\x91\x90aw\xC1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85`@\x01Q\x86``\x01Q\x87`\x80\x01Q\x88`\xA0\x01Q`@Q` \x01aJ\0\x91\x90ar=V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aJ,\x97\x96\x95\x94\x93\x92\x91\x90axSV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90PaJR\x83`@\x01Q\x82aN\x0BV[\x91PP\x91\x90PV[_aJcaN\x7FV[\x90P\x90V[_`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x02\x82\x01R\x82`\"\x82\x01R`B\x81 \x91PP\x92\x91PPV[_\x80_`A\x84Q\x03aJ\xE8W_\x80_` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90PaJ\xDA\x88\x82\x85\x85aN\xE2V[\x95P\x95P\x95PPPPaJ\xF6V[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15aK\x10WaK\x0Fa_\x1AV[[\x82`\x03\x81\x11\x15aK#WaK\"a_\x1AV[[\x03\x15aL[W`\x01`\x03\x81\x11\x15aK=WaK<a_\x1AV[[\x82`\x03\x81\x11\x15aKPWaKOa_\x1AV[[\x03aK\x87W`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15aK\x9BWaK\x9Aa_\x1AV[[\x82`\x03\x81\x11\x15aK\xAEWaK\xADa_\x1AV[[\x03aK\xF2W\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aK\xE9\x91\x90a_GV[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15aL\x05WaL\x04a_\x1AV[[\x82`\x03\x81\x11\x15aL\x18WaL\x17a_\x1AV[[\x03aLZW\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aLQ\x91\x90aWAV[`@Q\x80\x91\x03\x90\xFD[[PPV[_aLha/xV[_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03aL\xE1W\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aL\xD8\x91\x90aY\x1EV[`@Q\x80\x91\x03\x90\xFD[\x80aM\r\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1BaL}V[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@QaMx\x91\x90ar=V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14aM\xB0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>aM\xB5V[``\x91P[P\x91P\x91PaM\xC5\x85\x83\x83aO\xC9V[\x92PPP\x92\x91PPV[_4\x11\x15aN\tW`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x80\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0FaN6aPVV[aN>aP\xCCV[\x860`@Q` \x01aNT\x95\x94\x93\x92\x91\x90ax\xC0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90PaNv\x81\x84aJhV[\x91PP\x92\x91PPV[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0FaN\xA9aPVV[aN\xB1aP\xCCV[F0`@Q` \x01aN\xC7\x95\x94\x93\x92\x91\x90ax\xC0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x80_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15aO\x1EW_`\x03\x85\x92P\x92P\x92PaO\xBFV[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@QaOA\x94\x93\x92\x91\x90ay\x11V[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aOaW=_\x80>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03aO\xB2W_`\x01_\x80\x1B\x93P\x93P\x93PPaO\xBFV[\x80_\x80_\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[``\x82aO\xDEWaO\xD9\x82aQCV[aPNV[_\x82Q\x14\x80\x15aP\x04WP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15aPFW\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aP=\x91\x90aY\x1EV[`@Q\x80\x91\x03\x90\xFD[\x81\x90PaPOV[[\x93\x92PPPV[_\x80aP`a6\x97V[\x90P_aPka6\xBEV[\x90P_\x81Q\x11\x15aP\x87W\x80\x80Q\x90` \x01 \x92PPPaP\xC9V[_\x82_\x01T\x90P_\x80\x1B\x81\x14aP\xA2W\x80\x93PPPPaP\xC9V[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x80aP\xD6a6\x97V[\x90P_aP\xE1a7\\V[\x90P_\x81Q\x11\x15aP\xFDW\x80\x80Q\x90` \x01 \x92PPPaQ@V[_\x82`\x01\x01T\x90P_\x80\x1B\x81\x14aQ\x19W\x80\x93PPPPaQ@V[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x81Q\x11\x15aQUW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15aQ\xC1W\x91` \x02\x82\x01[\x82\x81\x11\x15aQ\xC0W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90aQ\xA5V[[P\x90PaQ\xCE\x91\x90aR\x1DV[P\x90V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15aR\x0CW\x91` \x02\x82\x01[\x82\x81\x11\x15aR\x0BW\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90aQ\xF0V[[P\x90PaR\x19\x91\x90aR\x1DV[P\x90V[[\x80\x82\x11\x15aR4W_\x81_\x90UP`\x01\x01aR\x1EV[P\x90V[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x81\x90P\x91\x90PV[aR[\x81aRIV[\x81\x14aReW_\x80\xFD[PV[_\x815\x90PaRv\x81aRRV[\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12aR\x9DWaR\x9CaR|V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aR\xBAWaR\xB9aR\x80V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15aR\xD6WaR\xD5aR\x84V[[\x92P\x92\x90PV[_\x80_\x80_\x80_`\x80\x88\x8A\x03\x12\x15aR\xF8WaR\xF7aRAV[[_aS\x05\x8A\x82\x8B\x01aRhV[\x97PP` \x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aS&WaS%aREV[[aS2\x8A\x82\x8B\x01aR\x88V[\x96P\x96PP`@\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aSUWaSTaREV[[aSa\x8A\x82\x8B\x01aR\x88V[\x94P\x94PP``\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aS\x84WaS\x83aREV[[aS\x90\x8A\x82\x8B\x01aR\x88V[\x92P\x92PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[_` \x82\x84\x03\x12\x15aS\xB6WaS\xB5aRAV[[_aS\xC3\x84\x82\x85\x01aRhV[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_aT\x1E\x82aS\xF5V[\x90P\x91\x90PV[aT.\x81aT\x14V[\x82RPPV[_aT?\x83\x83aT%V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aTa\x82aS\xCCV[aTk\x81\x85aS\xD6V[\x93PaTv\x83aS\xE6V[\x80_[\x83\x81\x10\x15aT\xA6W\x81QaT\x8D\x88\x82aT4V[\x97PaT\x98\x83aTKV[\x92PP`\x01\x81\x01\x90PaTyV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaT\xCB\x81\x84aTWV[\x90P\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15aU\nW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90PaT\xEFV[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_aU/\x82aT\xD3V[aU9\x81\x85aT\xDDV[\x93PaUI\x81\x85` \x86\x01aT\xEDV[aUR\x81aU\x15V[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaUu\x81\x84aU%V[\x90P\x92\x91PPV[aU\x86\x81aT\x14V[\x81\x14aU\x90W_\x80\xFD[PV[_\x815\x90PaU\xA1\x81aU}V[\x92\x91PPV[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[aU\xE1\x82aU\x15V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aV\0WaU\xFFaU\xABV[[\x80`@RPPPV[_aV\x12aR8V[\x90PaV\x1E\x82\x82aU\xD8V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aV=WaV<aU\xABV[[aVF\x82aU\x15V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_aVsaVn\x84aV#V[aV\tV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15aV\x8FWaV\x8EaU\xA7V[[aV\x9A\x84\x82\x85aVSV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aV\xB6WaV\xB5aR|V[[\x815aV\xC6\x84\x82` \x86\x01aVaV[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15aV\xE5WaV\xE4aRAV[[_aV\xF2\x85\x82\x86\x01aU\x93V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aW\x13WaW\x12aREV[[aW\x1F\x85\x82\x86\x01aV\xA2V[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[aW;\x81aW)V[\x82RPPV[_` \x82\x01\x90PaWT_\x83\x01\x84aW2V[\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[aWn\x81aWZV[\x82RPPV[_` \x82\x01\x90PaW\x87_\x83\x01\x84aWeV[\x92\x91PPV[_\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[aW\xC1\x81aW\x8DV[\x82RPPV[aW\xD0\x81aRIV[\x82RPPV[aW\xDF\x81aT\x14V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[aX\x17\x81aRIV[\x82RPPV[_aX(\x83\x83aX\x0EV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aXJ\x82aW\xE5V[aXT\x81\x85aW\xEFV[\x93PaX_\x83aW\xFFV[\x80_[\x83\x81\x10\x15aX\x8FW\x81QaXv\x88\x82aX\x1DV[\x97PaX\x81\x83aX4V[\x92PP`\x01\x81\x01\x90PaXbV[P\x85\x93PPPP\x92\x91PPV[_`\xE0\x82\x01\x90PaX\xAF_\x83\x01\x8AaW\xB8V[\x81\x81\x03` \x83\x01RaX\xC1\x81\x89aU%V[\x90P\x81\x81\x03`@\x83\x01RaX\xD5\x81\x88aU%V[\x90PaX\xE4``\x83\x01\x87aW\xC7V[aX\xF1`\x80\x83\x01\x86aW\xD6V[aX\xFE`\xA0\x83\x01\x85aW2V[\x81\x81\x03`\xC0\x83\x01RaY\x10\x81\x84aX@V[\x90P\x98\x97PPPPPPPPV[_` \x82\x01\x90PaY1_\x83\x01\x84aW\xD6V[\x92\x91PPV[_\x80\xFD[_`@\x82\x84\x03\x12\x15aYPWaYOaY7V[[\x81\x90P\x92\x91PPV[_\x80\x83`\x1F\x84\x01\x12aYnWaYmaR|V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aY\x8BWaY\x8AaR\x80V[[` \x83\x01\x91P\x83`@\x82\x02\x83\x01\x11\x15aY\xA7WaY\xA6aR\x84V[[\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12aY\xC3WaY\xC2aR|V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aY\xE0WaY\xDFaR\x80V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aY\xFCWaY\xFBaR\x84V[[\x92P\x92\x90PV[_\x80_\x80_\x80_\x80`\xC0\x89\x8B\x03\x12\x15aZ\x1FWaZ\x1EaRAV[[_aZ,\x8B\x82\x8C\x01aRhV[\x98PP` aZ=\x8B\x82\x8C\x01aY;V[\x97PP``\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZ^WaZ]aREV[[aZj\x8B\x82\x8C\x01aYYV[\x96P\x96PP`\x80\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZ\x8DWaZ\x8CaREV[[aZ\x99\x8B\x82\x8C\x01aY\xAEV[\x94P\x94PP`\xA0\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZ\xBCWaZ\xBBaREV[[aZ\xC8\x8B\x82\x8C\x01aR\x88V[\x92P\x92PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[_`@\x82\x84\x03\x12\x15aZ\xEFWaZ\xEEaY7V[[\x81\x90P\x92\x91PPV[_`@\x82\x84\x03\x12\x15a[\rWa[\x0CaY7V[[\x81\x90P\x92\x91PPV[_\x80_\x80_\x80_\x80_\x80_a\x01 \x8C\x8E\x03\x12\x15a[6Wa[5aRAV[[_\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a[SWa[RaREV[[a[_\x8E\x82\x8F\x01aYYV[\x9BP\x9BPP` a[r\x8E\x82\x8F\x01aZ\xDAV[\x99PP``a[\x83\x8E\x82\x8F\x01aY;V[\x98PP`\xA0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a[\xA4Wa[\xA3aREV[[a[\xB0\x8E\x82\x8F\x01aZ\xF8V[\x97PP`\xC0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a[\xD1Wa[\xD0aREV[[a[\xDD\x8E\x82\x8F\x01aR\x88V[\x96P\x96PP`\xE0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\\\0Wa[\xFFaREV[[a\\\x0C\x8E\x82\x8F\x01aR\x88V[\x94P\x94PPa\x01\0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\\0Wa\\/aREV[[a\\<\x8E\x82\x8F\x01aR\x88V[\x92P\x92PP\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[_\x80\x83`\x1F\x84\x01\x12a\\fWa\\eaR|V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\\\x83Wa\\\x82aR\x80V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a\\\x9FWa\\\x9EaR\x84V[[\x92P\x92\x90PV[_\x80_\x80`@\x85\x87\x03\x12\x15a\\\xBEWa\\\xBDaRAV[[_\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\\\xDBWa\\\xDAaREV[[a\\\xE7\x87\x82\x88\x01a\\QV[\x94P\x94PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a]\nWa]\taREV[[a]\x16\x87\x82\x88\x01aR\x88V[\x92P\x92PP\x92\x95\x91\x94P\x92PV[_\x80_\x80_\x80_\x80_\x80_a\x01\0\x8C\x8E\x03\x12\x15a]DWa]CaRAV[[_\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a]aWa]`aREV[[a]m\x8E\x82\x8F\x01aYYV[\x9BP\x9BPP` a]\x80\x8E\x82\x8F\x01aZ\xDAV[\x99PP``\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a]\xA1Wa]\xA0aREV[[a]\xAD\x8E\x82\x8F\x01aZ\xF8V[\x98PP`\x80a]\xBE\x8E\x82\x8F\x01aU\x93V[\x97PP`\xA0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a]\xDFWa]\xDEaREV[[a]\xEB\x8E\x82\x8F\x01aR\x88V[\x96P\x96PP`\xC0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a^\x0EWa^\raREV[[a^\x1A\x8E\x82\x8F\x01aR\x88V[\x94P\x94PP`\xE0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a^=Wa^<aREV[[a^I\x8E\x82\x8F\x01aR\x88V[\x92P\x92PP\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[_` \x82\x84\x03\x12\x15a^sWa^raRAV[[_a^\x80\x84\x82\x85\x01aU\x93V[\x91PP\x92\x91PPV[_\x80_\x80_``\x86\x88\x03\x12\x15a^\xA2Wa^\xA1aRAV[[_a^\xAF\x88\x82\x89\x01aU\x93V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a^\xD0Wa^\xCFaREV[[a^\xDC\x88\x82\x89\x01aYYV[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a^\xFFWa^\xFEaREV[[a_\x0B\x88\x82\x89\x01aR\x88V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[_` \x82\x01\x90Pa_Z_\x83\x01\x84aW\xC7V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a_\xA4W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a_\xB7Wa_\xB6a_`V[[P\x91\x90PV[_\x82\x90P\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02a`#\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a_\xE8V[a`-\x86\x83a_\xE8V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_a`ha`ca`^\x84aRIV[a`EV[aRIV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a`\x81\x83a`NV[a`\x95a`\x8D\x82a`oV[\x84\x84Ta_\xF4V[\x82UPPPPV[_\x90V[a`\xA9a`\x9DV[a`\xB4\x81\x84\x84a`xV[PPPV[[\x81\x81\x10\x15a`\xD7Wa`\xCC_\x82a`\xA1V[`\x01\x81\x01\x90Pa`\xBAV[PPV[`\x1F\x82\x11\x15aa\x1CWa`\xED\x81a_\xC7V[a`\xF6\x84a_\xD9V[\x81\x01` \x85\x10\x15aa\x05W\x81\x90P[aa\x19aa\x11\x85a_\xD9V[\x83\x01\x82a`\xB9V[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_aa<_\x19\x84`\x08\x02aa!V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_aaT\x83\x83aa-V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[aan\x83\x83a_\xBDV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aa\x87Waa\x86aU\xABV[[aa\x91\x82Ta_\x8DV[aa\x9C\x82\x82\x85a`\xDBV[_`\x1F\x83\x11`\x01\x81\x14aa\xC9W_\x84\x15aa\xB7W\x82\x87\x015\x90P[aa\xC1\x85\x82aaIV[\x86UPab(V[`\x1F\x19\x84\x16aa\xD7\x86a_\xC7V[_[\x82\x81\x10\x15aa\xFEW\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Paa\xD9V[\x86\x83\x10\x15ab\x1BW\x84\x89\x015ab\x17`\x1F\x89\x16\x82aa-V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[_\x81T\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81Taby\x81a_\x8DV[ab\x83\x81\x86ab]V[\x94P`\x01\x82\x16_\x81\x14ab\x9DW`\x01\x81\x14ab\xB3Wab\xE5V[`\xFF\x19\x83\x16\x86R\x81\x15\x15` \x02\x86\x01\x93Pab\xE5V[ab\xBC\x85a_\xC7V[_[\x83\x81\x10\x15ab\xDDW\x81T\x81\x89\x01R`\x01\x82\x01\x91P` \x81\x01\x90Pab\xBEV[\x80\x88\x01\x95PPP[PPP\x92\x91PPV[_ab\xF9\x83\x83abmV[\x90P\x92\x91PPV[_`\x01\x82\x01\x90P\x91\x90PV[_ac\x17\x82ab1V[ac!\x81\x85ab;V[\x93P\x83` \x82\x02\x85\x01ac3\x85abKV[\x80_[\x85\x81\x10\x15acmW\x84\x84\x03\x89R\x81acN\x85\x82ab\xEEV[\x94PacY\x83ac\x01V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pac6V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_\x81T\x90P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_`\x01\x82\x01\x90P\x91\x90PV[_ac\xB1\x82ac\x7FV[ac\xBB\x81\x85ab;V[\x93P\x83` \x82\x02\x85\x01ac\xCD\x85ac\x89V[\x80_[\x85\x81\x10\x15ad\x07W\x84\x84\x03\x89R\x81ac\xE8\x85\x82ab\xEEV[\x94Pac\xF3\x83ac\x9BV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pac\xD0V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_ad4\x83\x85ad\x19V[\x93PadA\x83\x85\x84aVSV[adJ\x83aU\x15V[\x84\x01\x90P\x93\x92PPPV[_``\x82\x01\x90P\x81\x81\x03_\x83\x01Radm\x81\x87ac\rV[\x90P\x81\x81\x03` \x83\x01Rad\x81\x81\x86ac\xA7V[\x90P\x81\x81\x03`@\x83\x01Rad\x96\x81\x84\x86ad)V[\x90P\x95\x94PPPPPV[_\x81\x90P\x92\x91PPV[_ad\xB5\x82aT\xD3V[ad\xBF\x81\x85ad\xA1V[\x93Pad\xCF\x81\x85` \x86\x01aT\xEDV[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_ae\x0F`\x02\x83ad\xA1V[\x91Pae\x1A\x82ad\xDBV[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aeY`\x01\x83ad\xA1V[\x91Paed\x82ae%V[`\x01\x82\x01\x90P\x91\x90PV[_aez\x82\x87ad\xABV[\x91Pae\x85\x82ae\x03V[\x91Pae\x91\x82\x86ad\xABV[\x91Pae\x9C\x82aeMV[\x91Pae\xA8\x82\x85ad\xABV[\x91Pae\xB3\x82aeMV[\x91Pae\xBF\x82\x84ad\xABV[\x91P\x81\x90P\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_af\x04\x82aRIV[\x91Paf\x0F\x83aRIV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15af'Waf&ae\xCDV[[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[afI\x81af-V[\x82RPPV[_` \x82\x01\x90Pafb_\x83\x01\x84af@V[\x92\x91PPV[_\x81Q\x90Pafv\x81aU}V[\x92\x91PPV[_` \x82\x84\x03\x12\x15af\x91Waf\x90aRAV[[_af\x9E\x84\x82\x85\x01afhV[\x91PP\x92\x91PPV[_``\x82\x01\x90P\x81\x81\x03_\x83\x01Raf\xC0\x81\x87\x89ad)V[\x90P\x81\x81\x03` \x83\x01Raf\xD4\x81\x86ac\xA7V[\x90P\x81\x81\x03`@\x83\x01Raf\xE9\x81\x84\x86ad)V[\x90P\x96\x95PPPPPPV[\x7FEIP712: Uninitialized\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_ag)`\x15\x83aT\xDDV[\x91Pag4\x82af\xF5V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RagV\x81ag\x1DV[\x90P\x91\x90PV[_agk` \x84\x01\x84aU\x93V[\x90P\x92\x91PPV[`@\x82\x01ag\x83_\x83\x01\x83ag]V[ag\x8F_\x85\x01\x82aT%V[Pag\x9D` \x83\x01\x83ag]V[ag\xAA` \x85\x01\x82aT%V[PPPPV[_\x81\x90P\x91\x90PV[_` \x82\x01\x90P\x91\x90PV[_ag\xD0\x83\x85aS\xD6V[\x93Pag\xDB\x82ag\xB0V[\x80_[\x85\x81\x10\x15ah\x13Wag\xF0\x82\x84ag]V[ag\xFA\x88\x82aT4V[\x97Pah\x05\x83ag\xB9V[\x92PP`\x01\x81\x01\x90Pag\xDEV[P\x85\x92PPP\x93\x92PPPV[_`\x80\x82\x01\x90Pah3_\x83\x01\x87aW\xC7V[ah@` \x83\x01\x86agsV[\x81\x81\x03``\x83\x01RahS\x81\x84\x86ag\xC5V[\x90P\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_`@\x82\x01\x90Pah\x9E_\x83\x01\x85aW2V[ah\xAB` \x83\x01\x84aW\xD6V[\x93\x92PPPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12ah\xDAWah\xD9ah\xB2V[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15ah\xFCWah\xFBah\xB6V[[` \x83\x01\x92P` \x82\x026\x03\x83\x13\x15ai\x18Wai\x17ah\xBAV[[P\x92P\x92\x90PV[_`\xFF\x82\x16\x90P\x91\x90PV[ai5\x81ai V[\x82RPPV[_`@\x82\x01\x90PaiN_\x83\x01\x85ai,V[ai[` \x83\x01\x84aW\xC7V[\x93\x92PPPV[_\x80\xFD[_\x80\xFD[_`@\x82\x84\x03\x12\x15ai\x7FWai~aibV[[ai\x89`@aV\tV[\x90P_ai\x98\x84\x82\x85\x01aRhV[_\x83\x01RP` ai\xAB\x84\x82\x85\x01aRhV[` \x83\x01RP\x92\x91PPV[_`@\x82\x84\x03\x12\x15ai\xCCWai\xCBaRAV[[_ai\xD9\x84\x82\x85\x01aijV[\x91PP\x92\x91PPV[_`@\x82\x01\x90Pai\xF5_\x83\x01\x86aW\xD6V[\x81\x81\x03` \x83\x01Raj\x08\x81\x84\x86ag\xC5V[\x90P\x94\x93PPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[ajD\x81aW)V[\x82RPPV[_ajU\x83\x83aj;V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_ajw\x82aj\x12V[aj\x81\x81\x85aj\x1CV[\x93Paj\x8C\x83aj,V[\x80_[\x83\x81\x10\x15aj\xBCW\x81Qaj\xA3\x88\x82ajJV[\x97Paj\xAE\x83ajaV[\x92PP`\x01\x81\x01\x90Paj\x8FV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Raj\xE1\x81\x84ajmV[\x90P\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15ak\x03Wak\x02aU\xABV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[ak\x1D\x81aW)V[\x81\x14ak'W_\x80\xFD[PV[_\x81Q\x90Pak8\x81ak\x14V[\x92\x91PPV[_\x81Q\x90PakL\x81aRRV[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aklWakkaU\xABV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_ak\x8Fak\x8A\x84akRV[aV\tV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15ak\xB2Wak\xB1aR\x84V[[\x83[\x81\x81\x10\x15ak\xDBW\x80ak\xC7\x88\x82afhV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pak\xB4V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12ak\xF9Wak\xF8aR|V[[\x81Qal\t\x84\x82` \x86\x01ak}V[\x91PP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15al'Wal&aibV[[al1`\x80aV\tV[\x90P_al@\x84\x82\x85\x01ak*V[_\x83\x01RP` alS\x84\x82\x85\x01ak>V[` \x83\x01RP`@alg\x84\x82\x85\x01ak*V[`@\x83\x01RP``\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15al\x8BWal\x8AaifV[[al\x97\x84\x82\x85\x01ak\xE5V[``\x83\x01RP\x92\x91PPV[_al\xB5al\xB0\x84aj\xE9V[aV\tV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15al\xD8Wal\xD7aR\x84V[[\x83[\x81\x81\x10\x15am\x1FW\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15al\xFDWal\xFCaR|V[[\x80\x86\x01am\n\x89\x82al\x12V[\x85R` \x85\x01\x94PPP` \x81\x01\x90Pal\xDAV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12am=Wam<aR|V[[\x81QamM\x84\x82` \x86\x01al\xA3V[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15amkWamjaRAV[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15am\x88Wam\x87aREV[[am\x94\x84\x82\x85\x01am)V[\x91PP\x92\x91PPV[_am\xA7\x82aRIV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03am\xD9Wam\xD8ae\xCDV[[`\x01\x82\x01\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[am\xF7\x82am\xE4V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15an\x10Wan\x0FaU\xABV[[an\x1A\x82Ta_\x8DV[an%\x82\x82\x85a`\xDBV[_` \x90P`\x1F\x83\x11`\x01\x81\x14anVW_\x84\x15anDW\x82\x87\x01Q\x90P[anN\x85\x82aaIV[\x86UPan\xB5V[`\x1F\x19\x84\x16and\x86a_\xC7V[_[\x82\x81\x10\x15an\x8BW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PanfV[\x86\x83\x10\x15an\xA8W\x84\x89\x01Qan\xA4`\x1F\x89\x16\x82aa-V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_ao\0\x82aS\xCCV[ao\n\x81\x85an\xE6V[\x93Pao\x15\x83aS\xE6V[\x80_[\x83\x81\x10\x15aoEW\x81Qao,\x88\x82aT4V[\x97Pao7\x83aTKV[\x92PP`\x01\x81\x01\x90Pao\x18V[P\x85\x93PPPP\x92\x91PPV[_`\x80\x83\x01_\x83\x01Qaog_\x86\x01\x82aj;V[P` \x83\x01Qaoz` \x86\x01\x82aX\x0EV[P`@\x83\x01Qao\x8D`@\x86\x01\x82aj;V[P``\x83\x01Q\x84\x82\x03``\x86\x01Rao\xA5\x82\x82an\xF6V[\x91PP\x80\x91PP\x92\x91PPV[_ao\xBD\x83\x83aoRV[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_ao\xDB\x82an\xBDV[ao\xE5\x81\x85an\xC7V[\x93P\x83` \x82\x02\x85\x01ao\xF7\x85an\xD7V[\x80_[\x85\x81\x10\x15ap2W\x84\x84\x03\x89R\x81Qap\x13\x85\x82ao\xB2V[\x94Pap\x1E\x83ao\xC5V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pao\xFAV[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_`\x80\x82\x01\x90P\x81\x81\x03_\x83\x01Rap\\\x81\x89ao\xD1V[\x90Papk` \x83\x01\x88aW\xD6V[\x81\x81\x03`@\x83\x01Rap~\x81\x86\x88ad)V[\x90P\x81\x81\x03``\x83\x01Rap\x93\x81\x84\x86ad)V[\x90P\x97\x96PPPPPPPV[_\x80\xFD[\x82\x81\x837PPPV[_ap\xB8\x83\x85aj\x1CV[\x93P\x7F\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15ap\xEBWap\xEAap\xA0V[[` \x83\x02\x92Pap\xFC\x83\x85\x84ap\xA4V[\x82\x84\x01\x90P\x93\x92PPPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Raq!\x81\x84\x86ap\xADV[\x90P\x93\x92PPPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01RaqB\x81\x86ao\xD1V[\x90P\x81\x81\x03` \x83\x01RaqW\x81\x84\x86ad)V[\x90P\x94\x93PPPPV[_\x81\x90P\x92\x91PPV[aqt\x81aW)V[\x82RPPV[_aq\x85\x83\x83aqkV[` \x83\x01\x90P\x92\x91PPV[_aq\x9B\x82aj\x12V[aq\xA5\x81\x85aqaV[\x93Paq\xB0\x83aj,V[\x80_[\x83\x81\x10\x15aq\xE0W\x81Qaq\xC7\x88\x82aqzV[\x97Paq\xD2\x83ajaV[\x92PP`\x01\x81\x01\x90Paq\xB3V[P\x85\x93PPPP\x92\x91PPV[_aq\xF8\x82\x84aq\x91V[\x91P\x81\x90P\x92\x91PPV[_\x81\x90P\x92\x91PPV[_ar\x17\x82am\xE4V[ar!\x81\x85ar\x03V[\x93Par1\x81\x85` \x86\x01aT\xEDV[\x80\x84\x01\x91PP\x92\x91PPV[_arH\x82\x84ar\rV[\x91P\x81\x90P\x92\x91PPV[_`\xA0\x82\x01\x90Parf_\x83\x01\x88aW2V[ars` \x83\x01\x87aW2V[ar\x80`@\x83\x01\x86aW2V[ar\x8D``\x83\x01\x85aW2V[ar\x9A`\x80\x83\x01\x84aW2V[\x96\x95PPPPPPV[_`@\x82\x01\x90Par\xB7_\x83\x01\x85aW\xC7V[ar\xC4` \x83\x01\x84aW\xD6V[\x93\x92PPPV[_` \x82\x84\x03\x12\x15ar\xE0War\xDFaRAV[[_ar\xED\x84\x82\x85\x01ak>V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15as8Was7aRAV[[_asE\x84\x82\x85\x01ak*V[\x91PP\x92\x91PPV[_`\x80\x82\x01\x90Pasa_\x83\x01\x87aW2V[asn` \x83\x01\x86aW2V[as{`@\x83\x01\x85aW2V[as\x88``\x83\x01\x84aW2V[\x95\x94PPPPPV[_a\xFF\xFF\x82\x16\x90P\x91\x90PV[_as\xB8as\xB3as\xAE\x84as\x91V[a`EV[aRIV[\x90P\x91\x90PV[as\xC8\x81as\x9EV[\x82RPPV[_`@\x82\x01\x90Pas\xE1_\x83\x01\x85as\xBFV[as\xEE` \x83\x01\x84aW\xC7V[\x93\x92PPPV[_`@\x82\x01\x90Pat\x08_\x83\x01\x85aW\xC7V[at\x15` \x83\x01\x84aW\xC7V[\x93\x92PPPV[_at&\x82aRIV[\x91Pat1\x83aRIV[\x92P\x82\x82\x02at?\x81aRIV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17atVWatUae\xCDV[[P\x92\x91PPV[`@\x82\x01_\x82\x01Qatq_\x85\x01\x82aX\x0EV[P` \x82\x01Qat\x84` \x85\x01\x82aX\x0EV[PPPPV[_``\x82\x01\x90Pat\x9D_\x83\x01\x85aW\xC7V[at\xAA` \x83\x01\x84at]V[\x93\x92PPPV[_`@\x82\x01\x90Pat\xC4_\x83\x01\x85aW\xD6V[\x81\x81\x03` \x83\x01Rat\xD6\x81\x84aTWV[\x90P\x93\x92PPPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Rat\xF8\x81\x84\x86ad)V[\x90P\x93\x92PPPV[_`\x80\x83\x01_\x83\x01Qau\x16_\x86\x01\x82aj;V[P` \x83\x01Qau)` \x86\x01\x82aX\x0EV[P`@\x83\x01Qau<`@\x86\x01\x82aj;V[P``\x83\x01Q\x84\x82\x03``\x86\x01RauT\x82\x82an\xF6V[\x91PP\x80\x91PP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Rauy\x81\x85au\x01V[\x90P\x81\x81\x03` \x83\x01Rau\x8D\x81\x84au\x01V[\x90P\x93\x92PPPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[`\x1F\x82\x11\x15au\xE9Wau\xBA\x81au\x96V[au\xC3\x84a_\xD9V[\x81\x01` \x85\x10\x15au\xD2W\x81\x90P[au\xE6au\xDE\x85a_\xD9V[\x83\x01\x82a`\xB9V[PP[PPPV[au\xF7\x82aT\xD3V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15av\x10Wav\x0FaU\xABV[[av\x1A\x82Ta_\x8DV[av%\x82\x82\x85au\xA8V[_` \x90P`\x1F\x83\x11`\x01\x81\x14avVW_\x84\x15avDW\x82\x87\x01Q\x90P[avN\x85\x82aaIV[\x86UPav\xB5V[`\x1F\x19\x84\x16avd\x86au\x96V[_[\x82\x81\x10\x15av\x8BW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PavfV[\x86\x83\x10\x15av\xA8W\x84\x89\x01Qav\xA4`\x1F\x89\x16\x82aa-V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_` \x82\x01\x90Pav\xD0_\x83\x01\x84ai,V[\x92\x91PPV[`T\x81\x10av\xE7Wav\xE6a_\x1AV[[PV[_\x81\x90Pav\xF7\x82av\xD6V[\x91\x90PV[_aw\x06\x82av\xEAV[\x90P\x91\x90PV[aw\x16\x81av\xFCV[\x82RPPV[_` \x82\x01\x90Paw/_\x83\x01\x84aw\rV[\x92\x91PPV[_\x81\x90P\x92\x91PPV[awH\x81aT\x14V[\x82RPPV[_awY\x83\x83aw?V[` \x83\x01\x90P\x92\x91PPV[_awo\x82aS\xCCV[awy\x81\x85aw5V[\x93Paw\x84\x83aS\xE6V[\x80_[\x83\x81\x10\x15aw\xB4W\x81Qaw\x9B\x88\x82awNV[\x97Paw\xA6\x83aTKV[\x92PP`\x01\x81\x01\x90Paw\x87V[P\x85\x93PPPP\x92\x91PPV[_aw\xCC\x82\x84aweV[\x91P\x81\x90P\x92\x91PPV[_a\x01\0\x82\x01\x90Paw\xEB_\x83\x01\x8BaW2V[aw\xF8` \x83\x01\x8AaW2V[ax\x05`@\x83\x01\x89aW2V[ax\x12``\x83\x01\x88aW\xD6V[ax\x1F`\x80\x83\x01\x87aW\xC7V[ax,`\xA0\x83\x01\x86aW\xC7V[ax9`\xC0\x83\x01\x85aW\xC7V[axF`\xE0\x83\x01\x84aW2V[\x99\x98PPPPPPPPPV[_`\xE0\x82\x01\x90Paxf_\x83\x01\x8AaW2V[axs` \x83\x01\x89aW2V[ax\x80`@\x83\x01\x88aW2V[ax\x8D``\x83\x01\x87aW\xC7V[ax\x9A`\x80\x83\x01\x86aW\xC7V[ax\xA7`\xA0\x83\x01\x85aW\xC7V[ax\xB4`\xC0\x83\x01\x84aW2V[\x98\x97PPPPPPPPV[_`\xA0\x82\x01\x90Pax\xD3_\x83\x01\x88aW2V[ax\xE0` \x83\x01\x87aW2V[ax\xED`@\x83\x01\x86aW2V[ax\xFA``\x83\x01\x85aW\xC7V[ay\x07`\x80\x83\x01\x84aW\xD6V[\x96\x95PPPPPPV[_`\x80\x82\x01\x90Pay$_\x83\x01\x87aW2V[ay1` \x83\x01\x86ai,V[ay>`@\x83\x01\x85aW2V[ayK``\x83\x01\x84aW2V[\x95\x94PPPPPV\xFEUserDecryptResponseVerification(bytes publicKey,bytes32[] ctHandles,bytes userDecryptedShare,bytes extraData)PublicDecryptVerification(bytes32[] ctHandles,bytes decryptedResult,bytes extraData)UserDecryptRequestVerification(bytes publicKey,address[] contractAddresses,uint256 contractsChainId,uint256 startTimestamp,uint256 durationDays,bytes extraData)DelegatedUserDecryptRequestVerification(bytes publicKey,address[] contractAddresses,address delegatorAddress,uint256 contractsChainId,uint256 startTimestamp,uint256 durationDays,bytes extraData)",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FheType(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<FheType> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        #[automatically_derived]
        impl FheType {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u8 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl From<u8> for FheType {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<FheType> for u8 {
            fn from(value: FheType) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for FheType {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for FheType {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct CtHandleContractPair { bytes32 ctHandle; address contractAddress; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CtHandleContractPair {
        #[allow(missing_docs)]
        pub ctHandle: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub contractAddress: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::Address,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<CtHandleContractPair> for UnderlyingRustTuple<'_> {
            fn from(value: CtHandleContractPair) -> Self {
                (value.ctHandle, value.contractAddress)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CtHandleContractPair {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    ctHandle: tuple.0,
                    contractAddress: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for CtHandleContractPair {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for CtHandleContractPair {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.ctHandle),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.contractAddress,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for CtHandleContractPair {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for CtHandleContractPair {
            const NAME: &'static str = "CtHandleContractPair";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "CtHandleContractPair(bytes32 ctHandle,address contractAddress)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.ctHandle)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.contractAddress,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for CtHandleContractPair {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.ctHandle,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.contractAddress,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ctHandle,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.contractAddress,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct DelegationAccounts { address delegatorAddress; address delegatedAddress; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DelegationAccounts {
        #[allow(missing_docs)]
        pub delegatorAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub delegatedAddress: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<DelegationAccounts> for UnderlyingRustTuple<'_> {
            fn from(value: DelegationAccounts) -> Self {
                (value.delegatorAddress, value.delegatedAddress)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DelegationAccounts {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    delegatorAddress: tuple.0,
                    delegatedAddress: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for DelegationAccounts {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for DelegationAccounts {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.delegatorAddress,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.delegatedAddress,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for DelegationAccounts {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for DelegationAccounts {
            const NAME: &'static str = "DelegationAccounts";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "DelegationAccounts(address delegatorAddress,address delegatedAddress)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.delegatorAddress,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.delegatedAddress,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for DelegationAccounts {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.delegatorAddress,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.delegatedAddress,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.delegatorAddress,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.delegatedAddress,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct SnsCiphertextMaterial { bytes32 ctHandle; uint256 keyId; bytes32 snsCiphertextDigest; address[] coprocessorTxSenderAddresses; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SnsCiphertextMaterial {
        #[allow(missing_docs)]
        pub ctHandle: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub snsCiphertextDigest: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub coprocessorTxSenderAddresses: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SnsCiphertextMaterial> for UnderlyingRustTuple<'_> {
            fn from(value: SnsCiphertextMaterial) -> Self {
                (
                    value.ctHandle,
                    value.keyId,
                    value.snsCiphertextDigest,
                    value.coprocessorTxSenderAddresses,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SnsCiphertextMaterial {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    ctHandle: tuple.0,
                    keyId: tuple.1,
                    snsCiphertextDigest: tuple.2,
                    coprocessorTxSenderAddresses: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SnsCiphertextMaterial {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for SnsCiphertextMaterial {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.ctHandle),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.snsCiphertextDigest),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.coprocessorTxSenderAddresses,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for SnsCiphertextMaterial {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for SnsCiphertextMaterial {
            const NAME: &'static str = "SnsCiphertextMaterial";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SnsCiphertextMaterial(bytes32 ctHandle,uint256 keyId,bytes32 snsCiphertextDigest,address[] coprocessorTxSenderAddresses)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.ctHandle)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.keyId)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.snsCiphertextDigest,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.coprocessorTxSenderAddresses,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SnsCiphertextMaterial {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.ctHandle,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.keyId)
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.snsCiphertextDigest,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.coprocessorTxSenderAddresses,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ctHandle,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.keyId,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.snsCiphertextDigest,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.coprocessorTxSenderAddresses,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `AddressEmptyCode(address)` and selector `0x9996b315`.
```solidity
error AddressEmptyCode(address target);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AddressEmptyCode {
        #[allow(missing_docs)]
        pub target: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<AddressEmptyCode> for UnderlyingRustTuple<'_> {
            fn from(value: AddressEmptyCode) -> Self {
                (value.target,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AddressEmptyCode {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { target: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AddressEmptyCode {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AddressEmptyCode(address)";
            const SELECTOR: [u8; 4] = [153u8, 150u8, 179u8, 21u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.target,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ContractAddressesMaxLengthExceeded(uint8,uint256)` and selector `0xc5ab467e`.
```solidity
error ContractAddressesMaxLengthExceeded(uint8 maxLength, uint256 actualLength);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ContractAddressesMaxLengthExceeded {
        #[allow(missing_docs)]
        pub maxLength: u8,
        #[allow(missing_docs)]
        pub actualLength: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<8>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u8,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ContractAddressesMaxLengthExceeded>
        for UnderlyingRustTuple<'_> {
            fn from(value: ContractAddressesMaxLengthExceeded) -> Self {
                (value.maxLength, value.actualLength)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ContractAddressesMaxLengthExceeded {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    maxLength: tuple.0,
                    actualLength: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ContractAddressesMaxLengthExceeded {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ContractAddressesMaxLengthExceeded(uint8,uint256)";
            const SELECTOR: [u8; 4] = [197u8, 171u8, 70u8, 126u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxLength),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.actualLength),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ContractNotInContractAddresses(address,address[])` and selector `0xa4c30391`.
```solidity
error ContractNotInContractAddresses(address contractAddress, address[] contractAddresses);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ContractNotInContractAddresses {
        #[allow(missing_docs)]
        pub contractAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub contractAddresses: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ContractNotInContractAddresses>
        for UnderlyingRustTuple<'_> {
            fn from(value: ContractNotInContractAddresses) -> Self {
                (value.contractAddress, value.contractAddresses)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ContractNotInContractAddresses {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    contractAddress: tuple.0,
                    contractAddresses: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ContractNotInContractAddresses {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ContractNotInContractAddresses(address,address[])";
            const SELECTOR: [u8; 4] = [164u8, 195u8, 3u8, 145u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.contractAddress,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.contractAddresses),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `DecryptionNotDone(uint256)` and selector `0x0bf01406`.
```solidity
error DecryptionNotDone(uint256 decryptionId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DecryptionNotDone {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<DecryptionNotDone> for UnderlyingRustTuple<'_> {
            fn from(value: DecryptionNotDone) -> Self {
                (value.decryptionId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DecryptionNotDone {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { decryptionId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DecryptionNotDone {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DecryptionNotDone(uint256)";
            const SELECTOR: [u8; 4] = [11u8, 240u8, 20u8, 6u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.decryptionId),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `DecryptionNotRequested(uint256)` and selector `0xd48af942`.
```solidity
error DecryptionNotRequested(uint256 decryptionId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DecryptionNotRequested {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<DecryptionNotRequested> for UnderlyingRustTuple<'_> {
            fn from(value: DecryptionNotRequested) -> Self {
                (value.decryptionId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DecryptionNotRequested {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { decryptionId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DecryptionNotRequested {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DecryptionNotRequested(uint256)";
            const SELECTOR: [u8; 4] = [212u8, 138u8, 249u8, 66u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.decryptionId),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `DelegatorAddressInContractAddresses(address,address[])` and selector `0xc3446ac7`.
```solidity
error DelegatorAddressInContractAddresses(address delegatorAddress, address[] contractAddresses);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DelegatorAddressInContractAddresses {
        #[allow(missing_docs)]
        pub delegatorAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub contractAddresses: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<DelegatorAddressInContractAddresses>
        for UnderlyingRustTuple<'_> {
            fn from(value: DelegatorAddressInContractAddresses) -> Self {
                (value.delegatorAddress, value.contractAddresses)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for DelegatorAddressInContractAddresses {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    delegatorAddress: tuple.0,
                    contractAddresses: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DelegatorAddressInContractAddresses {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DelegatorAddressInContractAddresses(address,address[])";
            const SELECTOR: [u8; 4] = [195u8, 68u8, 106u8, 199u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.delegatorAddress,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.contractAddresses),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `DifferentKeyIdsNotAllowed((bytes32,uint256,bytes32,address[]),(bytes32,uint256,bytes32,address[]))` and selector `0xcfae921f`.
```solidity
error DifferentKeyIdsNotAllowed(SnsCiphertextMaterial firstSnsCtMaterial, SnsCiphertextMaterial invalidSnsCtMaterial);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DifferentKeyIdsNotAllowed {
        #[allow(missing_docs)]
        pub firstSnsCtMaterial: <SnsCiphertextMaterial as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub invalidSnsCtMaterial: <SnsCiphertextMaterial as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (SnsCiphertextMaterial, SnsCiphertextMaterial);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <SnsCiphertextMaterial as alloy::sol_types::SolType>::RustType,
            <SnsCiphertextMaterial as alloy::sol_types::SolType>::RustType,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<DifferentKeyIdsNotAllowed>
        for UnderlyingRustTuple<'_> {
            fn from(value: DifferentKeyIdsNotAllowed) -> Self {
                (value.firstSnsCtMaterial, value.invalidSnsCtMaterial)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for DifferentKeyIdsNotAllowed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    firstSnsCtMaterial: tuple.0,
                    invalidSnsCtMaterial: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DifferentKeyIdsNotAllowed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DifferentKeyIdsNotAllowed((bytes32,uint256,bytes32,address[]),(bytes32,uint256,bytes32,address[]))";
            const SELECTOR: [u8; 4] = [207u8, 174u8, 146u8, 31u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <SnsCiphertextMaterial as alloy_sol_types::SolType>::tokenize(
                        &self.firstSnsCtMaterial,
                    ),
                    <SnsCiphertextMaterial as alloy_sol_types::SolType>::tokenize(
                        &self.invalidSnsCtMaterial,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ECDSAInvalidSignature()` and selector `0xf645eedf`.
```solidity
error ECDSAInvalidSignature();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignature;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ECDSAInvalidSignature> for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSAInvalidSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignature()";
            const SELECTOR: [u8; 4] = [246u8, 69u8, 238u8, 223u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ECDSAInvalidSignatureLength(uint256)` and selector `0xfce698f7`.
```solidity
error ECDSAInvalidSignatureLength(uint256 length);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignatureLength {
        #[allow(missing_docs)]
        pub length: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ECDSAInvalidSignatureLength>
        for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignatureLength) -> Self {
                (value.length,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ECDSAInvalidSignatureLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { length: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignatureLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignatureLength(uint256)";
            const SELECTOR: [u8; 4] = [252u8, 230u8, 152u8, 247u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.length),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ECDSAInvalidSignatureS(bytes32)` and selector `0xd78bce0c`.
```solidity
error ECDSAInvalidSignatureS(bytes32 s);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignatureS {
        #[allow(missing_docs)]
        pub s: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ECDSAInvalidSignatureS> for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignatureS) -> Self {
                (value.s,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSAInvalidSignatureS {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { s: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignatureS {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignatureS(bytes32)";
            const SELECTOR: [u8; 4] = [215u8, 139u8, 206u8, 12u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.s),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ERC1967InvalidImplementation(address)` and selector `0x4c9c8ce3`.
```solidity
error ERC1967InvalidImplementation(address implementation);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC1967InvalidImplementation {
        #[allow(missing_docs)]
        pub implementation: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC1967InvalidImplementation>
        for UnderlyingRustTuple<'_> {
            fn from(value: ERC1967InvalidImplementation) -> Self {
                (value.implementation,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ERC1967InvalidImplementation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { implementation: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC1967InvalidImplementation {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC1967InvalidImplementation(address)";
            const SELECTOR: [u8; 4] = [76u8, 156u8, 140u8, 227u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.implementation,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ERC1967NonPayable()` and selector `0xb398979f`.
```solidity
error ERC1967NonPayable();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC1967NonPayable;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC1967NonPayable> for UnderlyingRustTuple<'_> {
            fn from(value: ERC1967NonPayable) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC1967NonPayable {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC1967NonPayable {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC1967NonPayable()";
            const SELECTOR: [u8; 4] = [179u8, 152u8, 151u8, 159u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `EmptyContractAddresses()` and selector `0x57cfa217`.
```solidity
error EmptyContractAddresses();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyContractAddresses;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<EmptyContractAddresses> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyContractAddresses) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyContractAddresses {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyContractAddresses {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyContractAddresses()";
            const SELECTOR: [u8; 4] = [87u8, 207u8, 162u8, 23u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `EmptyCtHandleContractPairs()` and selector `0xa6a6cb21`.
```solidity
error EmptyCtHandleContractPairs();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyCtHandleContractPairs;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<EmptyCtHandleContractPairs>
        for UnderlyingRustTuple<'_> {
            fn from(value: EmptyCtHandleContractPairs) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for EmptyCtHandleContractPairs {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyCtHandleContractPairs {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyCtHandleContractPairs()";
            const SELECTOR: [u8; 4] = [166u8, 166u8, 203u8, 33u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `EmptyCtHandles()` and selector `0x2de75438`.
```solidity
error EmptyCtHandles();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyCtHandles;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<EmptyCtHandles> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyCtHandles) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyCtHandles {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyCtHandles {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyCtHandles()";
            const SELECTOR: [u8; 4] = [45u8, 231u8, 84u8, 56u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `EnforcedPause()` and selector `0xd93c0665`.
```solidity
error EnforcedPause();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EnforcedPause;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<EnforcedPause> for UnderlyingRustTuple<'_> {
            fn from(value: EnforcedPause) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EnforcedPause {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EnforcedPause {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EnforcedPause()";
            const SELECTOR: [u8; 4] = [217u8, 60u8, 6u8, 101u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ExpectedPause()` and selector `0x8dfc202b`.
```solidity
error ExpectedPause();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ExpectedPause;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ExpectedPause> for UnderlyingRustTuple<'_> {
            fn from(value: ExpectedPause) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ExpectedPause {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ExpectedPause {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ExpectedPause()";
            const SELECTOR: [u8; 4] = [141u8, 252u8, 32u8, 43u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `FailedCall()` and selector `0xd6bda275`.
```solidity
error FailedCall();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FailedCall;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<FailedCall> for UnderlyingRustTuple<'_> {
            fn from(value: FailedCall) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FailedCall {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FailedCall {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FailedCall()";
            const SELECTOR: [u8; 4] = [214u8, 189u8, 162u8, 117u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidFHEType(uint8)` and selector `0x641950d7`.
```solidity
error InvalidFHEType(uint8 fheTypeUint8);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidFHEType {
        #[allow(missing_docs)]
        pub fheTypeUint8: u8,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u8,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidFHEType> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidFHEType) -> Self {
                (value.fheTypeUint8,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidFHEType {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { fheTypeUint8: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidFHEType {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidFHEType(uint8)";
            const SELECTOR: [u8; 4] = [100u8, 25u8, 80u8, 215u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.fheTypeUint8),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidInitialization()` and selector `0xf92ee8a9`.
```solidity
error InvalidInitialization();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidInitialization;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidInitialization> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidInitialization) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidInitialization {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidInitialization {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidInitialization()";
            const SELECTOR: [u8; 4] = [249u8, 46u8, 232u8, 169u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidNullDurationDays()` and selector `0xde2859c1`.
```solidity
error InvalidNullDurationDays();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidNullDurationDays;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidNullDurationDays> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidNullDurationDays) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidNullDurationDays {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidNullDurationDays {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidNullDurationDays()";
            const SELECTOR: [u8; 4] = [222u8, 40u8, 89u8, 193u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidUserSignature(bytes)` and selector `0x2a873d27`.
```solidity
error InvalidUserSignature(bytes signature);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidUserSignature {
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidUserSignature> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidUserSignature) -> Self {
                (value.signature,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidUserSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { signature: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidUserSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidUserSignature(bytes)";
            const SELECTOR: [u8; 4] = [42u8, 135u8, 61u8, 39u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `KmsNodeAlreadySigned(uint256,address)` and selector `0x99ec48d9`.
```solidity
error KmsNodeAlreadySigned(uint256 decryptionId, address signer);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsNodeAlreadySigned {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub signer: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<KmsNodeAlreadySigned> for UnderlyingRustTuple<'_> {
            fn from(value: KmsNodeAlreadySigned) -> Self {
                (value.decryptionId, value.signer)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KmsNodeAlreadySigned {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    decryptionId: tuple.0,
                    signer: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KmsNodeAlreadySigned {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KmsNodeAlreadySigned(uint256,address)";
            const SELECTOR: [u8; 4] = [153u8, 236u8, 72u8, 217u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.decryptionId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.signer,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `MaxDecryptionRequestBitSizeExceeded(uint256,uint256)` and selector `0xe7f4895d`.
```solidity
error MaxDecryptionRequestBitSizeExceeded(uint256 maxBitSize, uint256 totalBitSize);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MaxDecryptionRequestBitSizeExceeded {
        #[allow(missing_docs)]
        pub maxBitSize: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub totalBitSize: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<MaxDecryptionRequestBitSizeExceeded>
        for UnderlyingRustTuple<'_> {
            fn from(value: MaxDecryptionRequestBitSizeExceeded) -> Self {
                (value.maxBitSize, value.totalBitSize)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for MaxDecryptionRequestBitSizeExceeded {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    maxBitSize: tuple.0,
                    totalBitSize: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MaxDecryptionRequestBitSizeExceeded {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MaxDecryptionRequestBitSizeExceeded(uint256,uint256)";
            const SELECTOR: [u8; 4] = [231u8, 244u8, 137u8, 93u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxBitSize),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalBitSize),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `MaxDurationDaysExceeded(uint256,uint256)` and selector `0x32951863`.
```solidity
error MaxDurationDaysExceeded(uint256 maxValue, uint256 actualValue);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MaxDurationDaysExceeded {
        #[allow(missing_docs)]
        pub maxValue: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub actualValue: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<MaxDurationDaysExceeded> for UnderlyingRustTuple<'_> {
            fn from(value: MaxDurationDaysExceeded) -> Self {
                (value.maxValue, value.actualValue)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MaxDurationDaysExceeded {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    maxValue: tuple.0,
                    actualValue: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MaxDurationDaysExceeded {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MaxDurationDaysExceeded(uint256,uint256)";
            const SELECTOR: [u8; 4] = [50u8, 149u8, 24u8, 99u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxValue),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.actualValue),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NotGatewayOwner(address)` and selector `0x0e56cf3d`.
```solidity
error NotGatewayOwner(address sender);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotGatewayOwner {
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NotGatewayOwner> for UnderlyingRustTuple<'_> {
            fn from(value: NotGatewayOwner) -> Self {
                (value.sender,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotGatewayOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { sender: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotGatewayOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotGatewayOwner(address)";
            const SELECTOR: [u8; 4] = [14u8, 86u8, 207u8, 61u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sender,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NotInitializing()` and selector `0xd7e6bcf8`.
```solidity
error NotInitializing();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotInitializing;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NotInitializing> for UnderlyingRustTuple<'_> {
            fn from(value: NotInitializing) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotInitializing {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotInitializing {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotInitializing()";
            const SELECTOR: [u8; 4] = [215u8, 230u8, 188u8, 248u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NotInitializingFromEmptyProxy()` and selector `0x6f4f731f`.
```solidity
error NotInitializingFromEmptyProxy();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotInitializingFromEmptyProxy;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NotInitializingFromEmptyProxy>
        for UnderlyingRustTuple<'_> {
            fn from(value: NotInitializingFromEmptyProxy) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for NotInitializingFromEmptyProxy {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotInitializingFromEmptyProxy {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotInitializingFromEmptyProxy()";
            const SELECTOR: [u8; 4] = [111u8, 79u8, 115u8, 31u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NotOwnerOrGatewayConfig(address)` and selector `0xe19166ee`.
```solidity
error NotOwnerOrGatewayConfig(address notOwnerOrGatewayConfig);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotOwnerOrGatewayConfig {
        #[allow(missing_docs)]
        pub notOwnerOrGatewayConfig: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NotOwnerOrGatewayConfig> for UnderlyingRustTuple<'_> {
            fn from(value: NotOwnerOrGatewayConfig) -> Self {
                (value.notOwnerOrGatewayConfig,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotOwnerOrGatewayConfig {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    notOwnerOrGatewayConfig: tuple.0,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotOwnerOrGatewayConfig {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotOwnerOrGatewayConfig(address)";
            const SELECTOR: [u8; 4] = [225u8, 145u8, 102u8, 238u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.notOwnerOrGatewayConfig,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NotPauser(address)` and selector `0x206a346e`.
```solidity
error NotPauser(address notPauser);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotPauser {
        #[allow(missing_docs)]
        pub notPauser: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NotPauser> for UnderlyingRustTuple<'_> {
            fn from(value: NotPauser) -> Self {
                (value.notPauser,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotPauser {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { notPauser: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotPauser {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotPauser(address)";
            const SELECTOR: [u8; 4] = [32u8, 106u8, 52u8, 110u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.notPauser,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NotPauserOrGatewayConfig(address)` and selector `0x388916bb`.
```solidity
error NotPauserOrGatewayConfig(address notPauserOrGatewayConfig);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotPauserOrGatewayConfig {
        #[allow(missing_docs)]
        pub notPauserOrGatewayConfig: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NotPauserOrGatewayConfig>
        for UnderlyingRustTuple<'_> {
            fn from(value: NotPauserOrGatewayConfig) -> Self {
                (value.notPauserOrGatewayConfig,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for NotPauserOrGatewayConfig {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    notPauserOrGatewayConfig: tuple.0,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotPauserOrGatewayConfig {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotPauserOrGatewayConfig(address)";
            const SELECTOR: [u8; 4] = [56u8, 137u8, 22u8, 187u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.notPauserOrGatewayConfig,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `OwnableInvalidOwner(address)` and selector `0x1e4fbdf7`.
```solidity
error OwnableInvalidOwner(address owner);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OwnableInvalidOwner {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OwnableInvalidOwner> for UnderlyingRustTuple<'_> {
            fn from(value: OwnableInvalidOwner) -> Self {
                (value.owner,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OwnableInvalidOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { owner: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OwnableInvalidOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OwnableInvalidOwner(address)";
            const SELECTOR: [u8; 4] = [30u8, 79u8, 189u8, 247u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `OwnableUnauthorizedAccount(address)` and selector `0x118cdaa7`.
```solidity
error OwnableUnauthorizedAccount(address account);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OwnableUnauthorizedAccount {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OwnableUnauthorizedAccount>
        for UnderlyingRustTuple<'_> {
            fn from(value: OwnableUnauthorizedAccount) -> Self {
                (value.account,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OwnableUnauthorizedAccount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { account: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OwnableUnauthorizedAccount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OwnableUnauthorizedAccount(address)";
            const SELECTOR: [u8; 4] = [17u8, 140u8, 218u8, 167u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `StartTimestampInFuture(uint256,uint256)` and selector `0xf24c0887`.
```solidity
error StartTimestampInFuture(uint256 currentTimestamp, uint256 startTimestamp);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StartTimestampInFuture {
        #[allow(missing_docs)]
        pub currentTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub startTimestamp: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<StartTimestampInFuture> for UnderlyingRustTuple<'_> {
            fn from(value: StartTimestampInFuture) -> Self {
                (value.currentTimestamp, value.startTimestamp)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StartTimestampInFuture {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    currentTimestamp: tuple.0,
                    startTimestamp: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for StartTimestampInFuture {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "StartTimestampInFuture(uint256,uint256)";
            const SELECTOR: [u8; 4] = [242u8, 76u8, 8u8, 135u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.currentTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.startTimestamp),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `UUPSUnauthorizedCallContext()` and selector `0xe07c8dba`.
```solidity
error UUPSUnauthorizedCallContext();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UUPSUnauthorizedCallContext;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UUPSUnauthorizedCallContext>
        for UnderlyingRustTuple<'_> {
            fn from(value: UUPSUnauthorizedCallContext) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for UUPSUnauthorizedCallContext {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UUPSUnauthorizedCallContext {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UUPSUnauthorizedCallContext()";
            const SELECTOR: [u8; 4] = [224u8, 124u8, 141u8, 186u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `UUPSUnsupportedProxiableUUID(bytes32)` and selector `0xaa1d49a4`.
```solidity
error UUPSUnsupportedProxiableUUID(bytes32 slot);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UUPSUnsupportedProxiableUUID {
        #[allow(missing_docs)]
        pub slot: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UUPSUnsupportedProxiableUUID>
        for UnderlyingRustTuple<'_> {
            fn from(value: UUPSUnsupportedProxiableUUID) -> Self {
                (value.slot,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for UUPSUnsupportedProxiableUUID {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { slot: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UUPSUnsupportedProxiableUUID {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UUPSUnsupportedProxiableUUID(bytes32)";
            const SELECTOR: [u8; 4] = [170u8, 29u8, 73u8, 164u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.slot),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `UnsupportedFHEType(uint8)` and selector `0xbe7830b1`.
```solidity
error UnsupportedFHEType(FheType fheType);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnsupportedFHEType {
        #[allow(missing_docs)]
        pub fheType: <FheType as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (FheType,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <FheType as alloy::sol_types::SolType>::RustType,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnsupportedFHEType> for UnderlyingRustTuple<'_> {
            fn from(value: UnsupportedFHEType) -> Self {
                (value.fheType,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UnsupportedFHEType {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { fheType: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnsupportedFHEType {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnsupportedFHEType(uint8)";
            const SELECTOR: [u8; 4] = [190u8, 120u8, 48u8, 177u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<FheType as alloy_sol_types::SolType>::tokenize(&self.fheType),)
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `UserAddressInContractAddresses(address,address[])` and selector `0xdc4d78b1`.
```solidity
error UserAddressInContractAddresses(address userAddress, address[] contractAddresses);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UserAddressInContractAddresses {
        #[allow(missing_docs)]
        pub userAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub contractAddresses: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UserAddressInContractAddresses>
        for UnderlyingRustTuple<'_> {
            fn from(value: UserAddressInContractAddresses) -> Self {
                (value.userAddress, value.contractAddresses)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for UserAddressInContractAddresses {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    userAddress: tuple.0,
                    contractAddresses: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UserAddressInContractAddresses {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UserAddressInContractAddresses(address,address[])";
            const SELECTOR: [u8; 4] = [220u8, 77u8, 120u8, 177u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.userAddress,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.contractAddresses),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `UserDecryptionRequestExpired(uint256,(uint256,uint256))` and selector `0x30348040`.
```solidity
error UserDecryptionRequestExpired(uint256 currentTimestamp, IDecryption.RequestValidity requestValidity);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UserDecryptionRequestExpired {
        #[allow(missing_docs)]
        pub currentTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub requestValidity: <IDecryption::RequestValidity as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            IDecryption::RequestValidity,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            <IDecryption::RequestValidity as alloy::sol_types::SolType>::RustType,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UserDecryptionRequestExpired>
        for UnderlyingRustTuple<'_> {
            fn from(value: UserDecryptionRequestExpired) -> Self {
                (value.currentTimestamp, value.requestValidity)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for UserDecryptionRequestExpired {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    currentTimestamp: tuple.0,
                    requestValidity: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UserDecryptionRequestExpired {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UserDecryptionRequestExpired(uint256,(uint256,uint256))";
            const SELECTOR: [u8; 4] = [48u8, 52u8, 128u8, 64u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.currentTimestamp),
                    <IDecryption::RequestValidity as alloy_sol_types::SolType>::tokenize(
                        &self.requestValidity,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `EIP712DomainChanged()` and selector `0x0a6387c9ea3628b88a633bb4f3b151770f70085117a15f9bf3787cda53f13d31`.
```solidity
event EIP712DomainChanged();
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EIP712DomainChanged;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for EIP712DomainChanged {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "EIP712DomainChanged()";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                10u8, 99u8, 135u8, 201u8, 234u8, 54u8, 40u8, 184u8, 138u8, 99u8, 59u8,
                180u8, 243u8, 177u8, 81u8, 119u8, 15u8, 112u8, 8u8, 81u8, 23u8, 161u8,
                95u8, 155u8, 243u8, 120u8, 124u8, 218u8, 83u8, 241u8, 61u8, 49u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {}
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for EIP712DomainChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EIP712DomainChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EIP712DomainChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Initialized(uint64)` and selector `0xc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2`.
```solidity
event Initialized(uint64 version);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Initialized {
        #[allow(missing_docs)]
        pub version: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Initialized {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Initialized(uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                199u8, 245u8, 5u8, 178u8, 243u8, 113u8, 174u8, 33u8, 117u8, 238u8, 73u8,
                19u8, 244u8, 73u8, 158u8, 31u8, 38u8, 51u8, 167u8, 181u8, 147u8, 99u8,
                33u8, 238u8, 209u8, 205u8, 174u8, 182u8, 17u8, 81u8, 129u8, 210u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { version: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.version),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Initialized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Initialized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Initialized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OwnershipTransferStarted(address,address)` and selector `0x38d16b8cac22d99fc7c124b9cd0de2d3fa1faef420bfe791d8c362d765e22700`.
```solidity
event OwnershipTransferStarted(address indexed previousOwner, address indexed newOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferStarted {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OwnershipTransferStarted {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferStarted(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                56u8, 209u8, 107u8, 140u8, 172u8, 34u8, 217u8, 159u8, 199u8, 193u8, 36u8,
                185u8, 205u8, 13u8, 226u8, 211u8, 250u8, 31u8, 174u8, 244u8, 32u8, 191u8,
                231u8, 145u8, 216u8, 195u8, 98u8, 215u8, 101u8, 226u8, 39u8, 0u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousOwner: topics.1,
                    newOwner: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.previousOwner.clone(),
                    self.newOwner.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.previousOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferStarted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferStarted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &OwnershipTransferStarted,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
```solidity
event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousOwner: topics.1,
                    newOwner: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.previousOwner.clone(),
                    self.newOwner.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.previousOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Paused(address)` and selector `0x62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a258`.
```solidity
event Paused(address account);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Paused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Paused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Paused(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                98u8, 231u8, 140u8, 234u8, 1u8, 190u8, 227u8, 32u8, 205u8, 78u8, 66u8,
                2u8, 112u8, 181u8, 234u8, 116u8, 0u8, 13u8, 17u8, 176u8, 201u8, 247u8,
                71u8, 84u8, 235u8, 219u8, 252u8, 84u8, 75u8, 5u8, 162u8, 88u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { account: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Paused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Paused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Paused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `PublicDecryptionRequest(uint256,(bytes32,uint256,bytes32,address[])[],bytes)` and selector `0x22db480a39bd72556438aadb4a32a3d2a6638b87c03bbec5fef6997e109587ff`.
```solidity
event PublicDecryptionRequest(uint256 indexed decryptionId, SnsCiphertextMaterial[] snsCtMaterials, bytes extraData);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PublicDecryptionRequest {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub snsCtMaterials: alloy::sol_types::private::Vec<
            <SnsCiphertextMaterial as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub extraData: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for PublicDecryptionRequest {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<SnsCiphertextMaterial>,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "PublicDecryptionRequest(uint256,(bytes32,uint256,bytes32,address[])[],bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                34u8, 219u8, 72u8, 10u8, 57u8, 189u8, 114u8, 85u8, 100u8, 56u8, 170u8,
                219u8, 74u8, 50u8, 163u8, 210u8, 166u8, 99u8, 139u8, 135u8, 192u8, 59u8,
                190u8, 197u8, 254u8, 246u8, 153u8, 126u8, 16u8, 149u8, 135u8, 255u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    decryptionId: topics.1,
                    snsCtMaterials: data.0,
                    extraData: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        SnsCiphertextMaterial,
                    > as alloy_sol_types::SolType>::tokenize(&self.snsCtMaterials),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.decryptionId.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.decryptionId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for PublicDecryptionRequest {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PublicDecryptionRequest> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &PublicDecryptionRequest,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `PublicDecryptionResponse(uint256,bytes,bytes[],bytes)` and selector `0xd7e58a367a0a6c298e76ad5d240004e327aa1423cbe4bd7ff85d4c715ef8d15f`.
```solidity
event PublicDecryptionResponse(uint256 indexed decryptionId, bytes decryptedResult, bytes[] signatures, bytes extraData);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PublicDecryptionResponse {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub decryptedResult: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signatures: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
        #[allow(missing_docs)]
        pub extraData: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for PublicDecryptionResponse {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "PublicDecryptionResponse(uint256,bytes,bytes[],bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                215u8, 229u8, 138u8, 54u8, 122u8, 10u8, 108u8, 41u8, 142u8, 118u8, 173u8,
                93u8, 36u8, 0u8, 4u8, 227u8, 39u8, 170u8, 20u8, 35u8, 203u8, 228u8,
                189u8, 127u8, 248u8, 93u8, 76u8, 113u8, 94u8, 248u8, 209u8, 95u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    decryptionId: topics.1,
                    decryptedResult: data.0,
                    signatures: data.1,
                    extraData: data.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.decryptedResult,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(&self.signatures),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.decryptionId.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.decryptionId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for PublicDecryptionResponse {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PublicDecryptionResponse> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &PublicDecryptionResponse,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Unpaused(address)` and selector `0x5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa`.
```solidity
event Unpaused(address account);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Unpaused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Unpaused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Unpaused(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                93u8, 185u8, 238u8, 10u8, 73u8, 91u8, 242u8, 230u8, 255u8, 156u8, 145u8,
                167u8, 131u8, 76u8, 27u8, 164u8, 253u8, 210u8, 68u8, 165u8, 232u8, 170u8,
                78u8, 83u8, 123u8, 211u8, 138u8, 234u8, 228u8, 176u8, 115u8, 170u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { account: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Unpaused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Unpaused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Unpaused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Upgraded(address)` and selector `0xbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b`.
```solidity
event Upgraded(address indexed implementation);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Upgraded {
        #[allow(missing_docs)]
        pub implementation: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Upgraded {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Upgraded(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                188u8, 124u8, 215u8, 90u8, 32u8, 238u8, 39u8, 253u8, 154u8, 222u8, 186u8,
                179u8, 32u8, 65u8, 247u8, 85u8, 33u8, 77u8, 188u8, 107u8, 255u8, 169u8,
                12u8, 192u8, 34u8, 91u8, 57u8, 218u8, 46u8, 92u8, 45u8, 59u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { implementation: topics.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.implementation.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.implementation,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Upgraded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Upgraded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Upgraded) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `UserDecryptionRequest(uint256,(bytes32,uint256,bytes32,address[])[],address,bytes,bytes)` and selector `0xf9011bd6ba0da6049c520d70fe5971f17ed7ab795486052544b51019896c596b`.
```solidity
event UserDecryptionRequest(uint256 indexed decryptionId, SnsCiphertextMaterial[] snsCtMaterials, address userAddress, bytes publicKey, bytes extraData);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UserDecryptionRequest {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub snsCtMaterials: alloy::sol_types::private::Vec<
            <SnsCiphertextMaterial as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub userAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub publicKey: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub extraData: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for UserDecryptionRequest {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<SnsCiphertextMaterial>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "UserDecryptionRequest(uint256,(bytes32,uint256,bytes32,address[])[],address,bytes,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                249u8, 1u8, 27u8, 214u8, 186u8, 13u8, 166u8, 4u8, 156u8, 82u8, 13u8,
                112u8, 254u8, 89u8, 113u8, 241u8, 126u8, 215u8, 171u8, 121u8, 84u8,
                134u8, 5u8, 37u8, 68u8, 181u8, 16u8, 25u8, 137u8, 108u8, 89u8, 107u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    decryptionId: topics.1,
                    snsCtMaterials: data.0,
                    userAddress: data.1,
                    publicKey: data.2,
                    extraData: data.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        SnsCiphertextMaterial,
                    > as alloy_sol_types::SolType>::tokenize(&self.snsCtMaterials),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.userAddress,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.publicKey,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.decryptionId.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.decryptionId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for UserDecryptionRequest {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UserDecryptionRequest> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &UserDecryptionRequest) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `UserDecryptionResponse(uint256,bytes[],bytes[],bytes)` and selector `0x1cbf1c6e9ea3dbcfd49b9690d93cd4b93b3cb22913f99b71d3aecae60b25fc56`.
```solidity
event UserDecryptionResponse(uint256 indexed decryptionId, bytes[] userDecryptedShares, bytes[] signatures, bytes extraData);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UserDecryptionResponse {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub userDecryptedShares: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Bytes,
        >,
        #[allow(missing_docs)]
        pub signatures: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
        #[allow(missing_docs)]
        pub extraData: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for UserDecryptionResponse {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "UserDecryptionResponse(uint256,bytes[],bytes[],bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                28u8, 191u8, 28u8, 110u8, 158u8, 163u8, 219u8, 207u8, 212u8, 155u8,
                150u8, 144u8, 217u8, 60u8, 212u8, 185u8, 59u8, 60u8, 178u8, 41u8, 19u8,
                249u8, 155u8, 113u8, 211u8, 174u8, 202u8, 230u8, 11u8, 37u8, 252u8, 86u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    decryptionId: topics.1,
                    userDecryptedShares: data.0,
                    signatures: data.1,
                    extraData: data.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(&self.userDecryptedShares),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(&self.signatures),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.decryptionId.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.decryptionId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for UserDecryptionResponse {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UserDecryptionResponse> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &UserDecryptionResponse) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {}
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `UPGRADE_INTERFACE_VERSION()` and selector `0xad3cb1cc`.
```solidity
function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UPGRADE_INTERFACE_VERSIONCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`UPGRADE_INTERFACE_VERSION()`](UPGRADE_INTERFACE_VERSIONCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UPGRADE_INTERFACE_VERSIONReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UPGRADE_INTERFACE_VERSIONCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: UPGRADE_INTERFACE_VERSIONCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for UPGRADE_INTERFACE_VERSIONCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UPGRADE_INTERFACE_VERSIONReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: UPGRADE_INTERFACE_VERSIONReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for UPGRADE_INTERFACE_VERSIONReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for UPGRADE_INTERFACE_VERSIONCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::String;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UPGRADE_INTERFACE_VERSION()";
            const SELECTOR: [u8; 4] = [173u8, 60u8, 177u8, 204u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: UPGRADE_INTERFACE_VERSIONReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: UPGRADE_INTERFACE_VERSIONReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `acceptOwnership()` and selector `0x79ba5097`.
```solidity
function acceptOwnership() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptOwnershipCall;
    ///Container type for the return parameters of the [`acceptOwnership()`](acceptOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<acceptOwnershipCall> for UnderlyingRustTuple<'_> {
                fn from(value: acceptOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for acceptOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<acceptOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: acceptOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for acceptOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl acceptOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <acceptOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for acceptOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = acceptOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "acceptOwnership()";
            const SELECTOR: [u8; 4] = [121u8, 186u8, 80u8, 151u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                acceptOwnershipReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `checkDecryptionDone(uint256)` and selector `0xa6090439`.
```solidity
function checkDecryptionDone(uint256 decryptionId) external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkDecryptionDoneCall {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`checkDecryptionDone(uint256)`](checkDecryptionDoneCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkDecryptionDoneReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkDecryptionDoneCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkDecryptionDoneCall) -> Self {
                    (value.decryptionId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkDecryptionDoneCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { decryptionId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkDecryptionDoneReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkDecryptionDoneReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkDecryptionDoneReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl checkDecryptionDoneReturn {
            fn _tokenize(
                &self,
            ) -> <checkDecryptionDoneCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkDecryptionDoneCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkDecryptionDoneReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkDecryptionDone(uint256)";
            const SELECTOR: [u8; 4] = [166u8, 9u8, 4u8, 57u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.decryptionId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                checkDecryptionDoneReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `checkDelegatedUserDecryptionReady(uint256,(address,address),(bytes32,address)[],address[],bytes)` and selector `0x9f98d899`.
```solidity
function checkDelegatedUserDecryptionReady(uint256 contractsChainId, DelegationAccounts memory delegationAccounts, CtHandleContractPair[] memory ctHandleContractPairs, address[] memory contractAddresses, bytes memory) external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkDelegatedUserDecryptionReadyCall {
        #[allow(missing_docs)]
        pub contractsChainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub delegationAccounts: <DelegationAccounts as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub ctHandleContractPairs: alloy::sol_types::private::Vec<
            <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub contractAddresses: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub _4: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`checkDelegatedUserDecryptionReady(uint256,(address,address),(bytes32,address)[],address[],bytes)`](checkDelegatedUserDecryptionReadyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkDelegatedUserDecryptionReadyReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                DelegationAccounts,
                alloy::sol_types::sol_data::Array<CtHandleContractPair>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                <DelegationAccounts as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<
                    <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkDelegatedUserDecryptionReadyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkDelegatedUserDecryptionReadyCall) -> Self {
                    (
                        value.contractsChainId,
                        value.delegationAccounts,
                        value.ctHandleContractPairs,
                        value.contractAddresses,
                        value._4,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkDelegatedUserDecryptionReadyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        contractsChainId: tuple.0,
                        delegationAccounts: tuple.1,
                        ctHandleContractPairs: tuple.2,
                        contractAddresses: tuple.3,
                        _4: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkDelegatedUserDecryptionReadyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkDelegatedUserDecryptionReadyReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkDelegatedUserDecryptionReadyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl checkDelegatedUserDecryptionReadyReturn {
            fn _tokenize(
                &self,
            ) -> <checkDelegatedUserDecryptionReadyCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkDelegatedUserDecryptionReadyCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                DelegationAccounts,
                alloy::sol_types::sol_data::Array<CtHandleContractPair>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkDelegatedUserDecryptionReadyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkDelegatedUserDecryptionReady(uint256,(address,address),(bytes32,address)[],address[],bytes)";
            const SELECTOR: [u8; 4] = [159u8, 152u8, 216u8, 153u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.contractsChainId),
                    <DelegationAccounts as alloy_sol_types::SolType>::tokenize(
                        &self.delegationAccounts,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        CtHandleContractPair,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.ctHandleContractPairs,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.contractAddresses),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._4,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                checkDelegatedUserDecryptionReadyReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `checkPublicDecryptionReady(bytes32[],bytes)` and selector `0xe31ec19d`.
```solidity
function checkPublicDecryptionReady(bytes32[] memory ctHandles, bytes memory) external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkPublicDecryptionReadyCall {
        #[allow(missing_docs)]
        pub ctHandles: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`checkPublicDecryptionReady(bytes32[],bytes)`](checkPublicDecryptionReadyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkPublicDecryptionReadyReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
                >,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkPublicDecryptionReadyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkPublicDecryptionReadyCall) -> Self {
                    (value.ctHandles, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkPublicDecryptionReadyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        ctHandles: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkPublicDecryptionReadyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkPublicDecryptionReadyReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkPublicDecryptionReadyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl checkPublicDecryptionReadyReturn {
            fn _tokenize(
                &self,
            ) -> <checkPublicDecryptionReadyCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkPublicDecryptionReadyCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkPublicDecryptionReadyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkPublicDecryptionReady(bytes32[],bytes)";
            const SELECTOR: [u8; 4] = [227u8, 30u8, 193u8, 157u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.ctHandles),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._1,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                checkPublicDecryptionReadyReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `checkUserDecryptionReady(address,(bytes32,address)[],bytes)` and selector `0xf4a6e2ce`.
```solidity
function checkUserDecryptionReady(address userAddress, CtHandleContractPair[] memory ctHandleContractPairs, bytes memory) external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkUserDecryptionReadyCall {
        #[allow(missing_docs)]
        pub userAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub ctHandleContractPairs: alloy::sol_types::private::Vec<
            <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub _2: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`checkUserDecryptionReady(address,(bytes32,address)[],bytes)`](checkUserDecryptionReadyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkUserDecryptionReadyReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<CtHandleContractPair>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<
                    <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkUserDecryptionReadyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkUserDecryptionReadyCall) -> Self {
                    (value.userAddress, value.ctHandleContractPairs, value._2)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkUserDecryptionReadyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        userAddress: tuple.0,
                        ctHandleContractPairs: tuple.1,
                        _2: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkUserDecryptionReadyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkUserDecryptionReadyReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkUserDecryptionReadyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl checkUserDecryptionReadyReturn {
            fn _tokenize(
                &self,
            ) -> <checkUserDecryptionReadyCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkUserDecryptionReadyCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<CtHandleContractPair>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkUserDecryptionReadyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkUserDecryptionReady(address,(bytes32,address)[],bytes)";
            const SELECTOR: [u8; 4] = [244u8, 166u8, 226u8, 206u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.userAddress,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        CtHandleContractPair,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.ctHandleContractPairs,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._2,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                checkUserDecryptionReadyReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `delegatedUserDecryptionRequest((bytes32,address)[],(uint256,uint256),(address,address),(uint256,address[]),bytes,bytes,bytes)` and selector `0x9fad5a2f`.
```solidity
function delegatedUserDecryptionRequest(CtHandleContractPair[] memory ctHandleContractPairs, IDecryption.RequestValidity memory requestValidity, DelegationAccounts memory delegationAccounts, IDecryption.ContractsInfo memory contractsInfo, bytes memory publicKey, bytes memory signature, bytes memory extraData) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegatedUserDecryptionRequestCall {
        #[allow(missing_docs)]
        pub ctHandleContractPairs: alloy::sol_types::private::Vec<
            <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub requestValidity: <IDecryption::RequestValidity as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub delegationAccounts: <DelegationAccounts as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub contractsInfo: <IDecryption::ContractsInfo as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub publicKey: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub extraData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`delegatedUserDecryptionRequest((bytes32,address)[],(uint256,uint256),(address,address),(uint256,address[]),bytes,bytes,bytes)`](delegatedUserDecryptionRequestCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegatedUserDecryptionRequestReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<CtHandleContractPair>,
                IDecryption::RequestValidity,
                DelegationAccounts,
                IDecryption::ContractsInfo,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
                >,
                <IDecryption::RequestValidity as alloy::sol_types::SolType>::RustType,
                <DelegationAccounts as alloy::sol_types::SolType>::RustType,
                <IDecryption::ContractsInfo as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegatedUserDecryptionRequestCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: delegatedUserDecryptionRequestCall) -> Self {
                    (
                        value.ctHandleContractPairs,
                        value.requestValidity,
                        value.delegationAccounts,
                        value.contractsInfo,
                        value.publicKey,
                        value.signature,
                        value.extraData,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for delegatedUserDecryptionRequestCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        ctHandleContractPairs: tuple.0,
                        requestValidity: tuple.1,
                        delegationAccounts: tuple.2,
                        contractsInfo: tuple.3,
                        publicKey: tuple.4,
                        signature: tuple.5,
                        extraData: tuple.6,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegatedUserDecryptionRequestReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: delegatedUserDecryptionRequestReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for delegatedUserDecryptionRequestReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl delegatedUserDecryptionRequestReturn {
            fn _tokenize(
                &self,
            ) -> <delegatedUserDecryptionRequestCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegatedUserDecryptionRequestCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<CtHandleContractPair>,
                IDecryption::RequestValidity,
                DelegationAccounts,
                IDecryption::ContractsInfo,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegatedUserDecryptionRequestReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegatedUserDecryptionRequest((bytes32,address)[],(uint256,uint256),(address,address),(uint256,address[]),bytes,bytes,bytes)";
            const SELECTOR: [u8; 4] = [159u8, 173u8, 90u8, 47u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        CtHandleContractPair,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.ctHandleContractPairs,
                    ),
                    <IDecryption::RequestValidity as alloy_sol_types::SolType>::tokenize(
                        &self.requestValidity,
                    ),
                    <DelegationAccounts as alloy_sol_types::SolType>::tokenize(
                        &self.delegationAccounts,
                    ),
                    <IDecryption::ContractsInfo as alloy_sol_types::SolType>::tokenize(
                        &self.contractsInfo,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.publicKey,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                delegatedUserDecryptionRequestReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `eip712Domain()` and selector `0x84b0196e`.
```solidity
function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eip712DomainCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`eip712Domain()`](eip712DomainCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eip712DomainReturn {
        #[allow(missing_docs)]
        pub fields: alloy::sol_types::private::FixedBytes<1>,
        #[allow(missing_docs)]
        pub name: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub version: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub verifyingContract: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub extensions: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<eip712DomainCall> for UnderlyingRustTuple<'_> {
                fn from(value: eip712DomainCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eip712DomainCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<1>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<1>,
                alloy::sol_types::private::String,
                alloy::sol_types::private::String,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<eip712DomainReturn> for UnderlyingRustTuple<'_> {
                fn from(value: eip712DomainReturn) -> Self {
                    (
                        value.fields,
                        value.name,
                        value.version,
                        value.chainId,
                        value.verifyingContract,
                        value.salt,
                        value.extensions,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eip712DomainReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        fields: tuple.0,
                        name: tuple.1,
                        version: tuple.2,
                        chainId: tuple.3,
                        verifyingContract: tuple.4,
                        salt: tuple.5,
                        extensions: tuple.6,
                    }
                }
            }
        }
        impl eip712DomainReturn {
            fn _tokenize(
                &self,
            ) -> <eip712DomainCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        1,
                    > as alloy_sol_types::SolType>::tokenize(&self.fields),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.name,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.version,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.chainId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.verifyingContract,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.extensions),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eip712DomainCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = eip712DomainReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<1>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eip712Domain()";
            const SELECTOR: [u8; 4] = [132u8, 176u8, 25u8, 110u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                eip712DomainReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getDecryptionConsensusTxSenders(uint256)` and selector `0x0900cc69`.
```solidity
function getDecryptionConsensusTxSenders(uint256 decryptionId) external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDecryptionConsensusTxSendersCall {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getDecryptionConsensusTxSenders(uint256)`](getDecryptionConsensusTxSendersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDecryptionConsensusTxSendersReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getDecryptionConsensusTxSendersCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getDecryptionConsensusTxSendersCall) -> Self {
                    (value.decryptionId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getDecryptionConsensusTxSendersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { decryptionId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getDecryptionConsensusTxSendersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getDecryptionConsensusTxSendersReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getDecryptionConsensusTxSendersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getDecryptionConsensusTxSendersCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getDecryptionConsensusTxSenders(uint256)";
            const SELECTOR: [u8; 4] = [9u8, 0u8, 204u8, 105u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.decryptionId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getDecryptionConsensusTxSendersReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getDecryptionConsensusTxSendersReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getVersion()` and selector `0x0d8e6e2c`.
```solidity
function getVersion() external pure returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getVersionCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getVersion()`](getVersionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getVersionReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getVersionCall> for UnderlyingRustTuple<'_> {
                fn from(value: getVersionCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getVersionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getVersionReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getVersionReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getVersionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getVersionCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::String;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getVersion()";
            const SELECTOR: [u8; 4] = [13u8, 142u8, 110u8, 44u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getVersionReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getVersionReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `initializeFromEmptyProxy()` and selector `0x39f73810`.
```solidity
function initializeFromEmptyProxy() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeFromEmptyProxyCall;
    ///Container type for the return parameters of the [`initializeFromEmptyProxy()`](initializeFromEmptyProxyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeFromEmptyProxyReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeFromEmptyProxyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializeFromEmptyProxyCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeFromEmptyProxyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeFromEmptyProxyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializeFromEmptyProxyReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeFromEmptyProxyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl initializeFromEmptyProxyReturn {
            fn _tokenize(
                &self,
            ) -> <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeFromEmptyProxyCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeFromEmptyProxyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initializeFromEmptyProxy()";
            const SELECTOR: [u8; 4] = [57u8, 247u8, 56u8, 16u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                initializeFromEmptyProxyReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
```solidity
function owner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: ownerReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: ownerReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `pause()` and selector `0x8456cb59`.
```solidity
function pause() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseCall;
    ///Container type for the return parameters of the [`pause()`](pauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauseCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl pauseReturn {
            fn _tokenize(
                &self,
            ) -> <pauseCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauseCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pause()";
            const SELECTOR: [u8; 4] = [132u8, 86u8, 203u8, 89u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                pauseReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `paused()` and selector `0x5c975abb`.
```solidity
function paused() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pausedCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`paused()`](pausedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pausedReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pausedCall> for UnderlyingRustTuple<'_> {
                fn from(value: pausedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pausedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pausedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pausedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pausedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pausedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "paused()";
            const SELECTOR: [u8; 4] = [92u8, 151u8, 90u8, 187u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: pausedReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: pausedReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `pendingOwner()` and selector `0xe30c3978`.
```solidity
function pendingOwner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pendingOwnerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`pendingOwner()`](pendingOwnerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pendingOwnerReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pendingOwnerCall> for UnderlyingRustTuple<'_> {
                fn from(value: pendingOwnerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pendingOwnerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pendingOwnerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pendingOwnerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pendingOwnerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pendingOwnerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pendingOwner()";
            const SELECTOR: [u8; 4] = [227u8, 12u8, 57u8, 120u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: pendingOwnerReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: pendingOwnerReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `proxiableUUID()` and selector `0x52d1902d`.
```solidity
function proxiableUUID() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proxiableUUIDCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`proxiableUUID()`](proxiableUUIDCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proxiableUUIDReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proxiableUUIDCall> for UnderlyingRustTuple<'_> {
                fn from(value: proxiableUUIDCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proxiableUUIDCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proxiableUUIDReturn> for UnderlyingRustTuple<'_> {
                fn from(value: proxiableUUIDReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proxiableUUIDReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proxiableUUIDCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proxiableUUID()";
            const SELECTOR: [u8; 4] = [82u8, 209u8, 144u8, 45u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: proxiableUUIDReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: proxiableUUIDReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `publicDecryptionRequest(bytes32[],bytes)` and selector `0xd8998f45`.
```solidity
function publicDecryptionRequest(bytes32[] memory ctHandles, bytes memory extraData) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct publicDecryptionRequestCall {
        #[allow(missing_docs)]
        pub ctHandles: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
        #[allow(missing_docs)]
        pub extraData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`publicDecryptionRequest(bytes32[],bytes)`](publicDecryptionRequestCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct publicDecryptionRequestReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
                >,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<publicDecryptionRequestCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: publicDecryptionRequestCall) -> Self {
                    (value.ctHandles, value.extraData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for publicDecryptionRequestCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        ctHandles: tuple.0,
                        extraData: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<publicDecryptionRequestReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: publicDecryptionRequestReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for publicDecryptionRequestReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl publicDecryptionRequestReturn {
            fn _tokenize(
                &self,
            ) -> <publicDecryptionRequestCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for publicDecryptionRequestCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = publicDecryptionRequestReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "publicDecryptionRequest(bytes32[],bytes)";
            const SELECTOR: [u8; 4] = [216u8, 153u8, 143u8, 69u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.ctHandles),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                publicDecryptionRequestReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `publicDecryptionResponse(uint256,bytes,bytes,bytes)` and selector `0x6f8913bc`.
```solidity
function publicDecryptionResponse(uint256 decryptionId, bytes memory decryptedResult, bytes memory signature, bytes memory extraData) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct publicDecryptionResponseCall {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub decryptedResult: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub extraData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`publicDecryptionResponse(uint256,bytes,bytes,bytes)`](publicDecryptionResponseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct publicDecryptionResponseReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<publicDecryptionResponseCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: publicDecryptionResponseCall) -> Self {
                    (
                        value.decryptionId,
                        value.decryptedResult,
                        value.signature,
                        value.extraData,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for publicDecryptionResponseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        decryptionId: tuple.0,
                        decryptedResult: tuple.1,
                        signature: tuple.2,
                        extraData: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<publicDecryptionResponseReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: publicDecryptionResponseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for publicDecryptionResponseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl publicDecryptionResponseReturn {
            fn _tokenize(
                &self,
            ) -> <publicDecryptionResponseCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for publicDecryptionResponseCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = publicDecryptionResponseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "publicDecryptionResponse(uint256,bytes,bytes,bytes)";
            const SELECTOR: [u8; 4] = [111u8, 137u8, 19u8, 188u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.decryptionId),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.decryptedResult,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                publicDecryptionResponseReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `reinitializeV4()` and selector `0x123abb28`.
```solidity
function reinitializeV4() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reinitializeV4Call;
    ///Container type for the return parameters of the [`reinitializeV4()`](reinitializeV4Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reinitializeV4Return {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<reinitializeV4Call> for UnderlyingRustTuple<'_> {
                fn from(value: reinitializeV4Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for reinitializeV4Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<reinitializeV4Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: reinitializeV4Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for reinitializeV4Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl reinitializeV4Return {
            fn _tokenize(
                &self,
            ) -> <reinitializeV4Call as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for reinitializeV4Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = reinitializeV4Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "reinitializeV4()";
            const SELECTOR: [u8; 4] = [18u8, 58u8, 187u8, 40u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                reinitializeV4Return::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
```solidity
function renounceOwnership() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall;
    ///Container type for the return parameters of the [`renounceOwnership()`](renounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl renounceOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <renounceOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceOwnership()";
            const SELECTOR: [u8; 4] = [113u8, 80u8, 24u8, 166u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                renounceOwnershipReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
```solidity
function transferOwnership(address newOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newOwner: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl transferOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <transferOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnership(address)";
            const SELECTOR: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newOwner,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                transferOwnershipReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `unpause()` and selector `0x3f4ba83a`.
```solidity
function unpause() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseCall;
    ///Container type for the return parameters of the [`unpause()`](unpauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unpauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unpauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl unpauseReturn {
            fn _tokenize(
                &self,
            ) -> <unpauseCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for unpauseCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = unpauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unpause()";
            const SELECTOR: [u8; 4] = [63u8, 75u8, 168u8, 58u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                unpauseReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `upgradeToAndCall(address,bytes)` and selector `0x4f1ef286`.
```solidity
function upgradeToAndCall(address newImplementation, bytes memory data) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct upgradeToAndCallCall {
        #[allow(missing_docs)]
        pub newImplementation: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`upgradeToAndCall(address,bytes)`](upgradeToAndCallCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct upgradeToAndCallReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<upgradeToAndCallCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: upgradeToAndCallCall) -> Self {
                    (value.newImplementation, value.data)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for upgradeToAndCallCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newImplementation: tuple.0,
                        data: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<upgradeToAndCallReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: upgradeToAndCallReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for upgradeToAndCallReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl upgradeToAndCallReturn {
            fn _tokenize(
                &self,
            ) -> <upgradeToAndCallCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for upgradeToAndCallCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = upgradeToAndCallReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "upgradeToAndCall(address,bytes)";
            const SELECTOR: [u8; 4] = [79u8, 30u8, 242u8, 134u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newImplementation,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                upgradeToAndCallReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `userDecryptionRequest((bytes32,address)[],(uint256,uint256),(uint256,address[]),address,bytes,bytes,bytes)` and selector `0xf1b57adb`.
```solidity
function userDecryptionRequest(CtHandleContractPair[] memory ctHandleContractPairs, IDecryption.RequestValidity memory requestValidity, IDecryption.ContractsInfo memory contractsInfo, address userAddress, bytes memory publicKey, bytes memory signature, bytes memory extraData) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct userDecryptionRequestCall {
        #[allow(missing_docs)]
        pub ctHandleContractPairs: alloy::sol_types::private::Vec<
            <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub requestValidity: <IDecryption::RequestValidity as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub contractsInfo: <IDecryption::ContractsInfo as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub userAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub publicKey: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub extraData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`userDecryptionRequest((bytes32,address)[],(uint256,uint256),(uint256,address[]),address,bytes,bytes,bytes)`](userDecryptionRequestCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct userDecryptionRequestReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<CtHandleContractPair>,
                IDecryption::RequestValidity,
                IDecryption::ContractsInfo,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
                >,
                <IDecryption::RequestValidity as alloy::sol_types::SolType>::RustType,
                <IDecryption::ContractsInfo as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<userDecryptionRequestCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: userDecryptionRequestCall) -> Self {
                    (
                        value.ctHandleContractPairs,
                        value.requestValidity,
                        value.contractsInfo,
                        value.userAddress,
                        value.publicKey,
                        value.signature,
                        value.extraData,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for userDecryptionRequestCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        ctHandleContractPairs: tuple.0,
                        requestValidity: tuple.1,
                        contractsInfo: tuple.2,
                        userAddress: tuple.3,
                        publicKey: tuple.4,
                        signature: tuple.5,
                        extraData: tuple.6,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<userDecryptionRequestReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: userDecryptionRequestReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for userDecryptionRequestReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl userDecryptionRequestReturn {
            fn _tokenize(
                &self,
            ) -> <userDecryptionRequestCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for userDecryptionRequestCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<CtHandleContractPair>,
                IDecryption::RequestValidity,
                IDecryption::ContractsInfo,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = userDecryptionRequestReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "userDecryptionRequest((bytes32,address)[],(uint256,uint256),(uint256,address[]),address,bytes,bytes,bytes)";
            const SELECTOR: [u8; 4] = [241u8, 181u8, 122u8, 219u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        CtHandleContractPair,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.ctHandleContractPairs,
                    ),
                    <IDecryption::RequestValidity as alloy_sol_types::SolType>::tokenize(
                        &self.requestValidity,
                    ),
                    <IDecryption::ContractsInfo as alloy_sol_types::SolType>::tokenize(
                        &self.contractsInfo,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.userAddress,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.publicKey,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                userDecryptionRequestReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `userDecryptionResponse(uint256,bytes,bytes,bytes)` and selector `0x046f9eb3`.
```solidity
function userDecryptionResponse(uint256 decryptionId, bytes memory userDecryptedShare, bytes memory signature, bytes memory extraData) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct userDecryptionResponseCall {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub userDecryptedShare: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub extraData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`userDecryptionResponse(uint256,bytes,bytes,bytes)`](userDecryptionResponseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct userDecryptionResponseReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<userDecryptionResponseCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: userDecryptionResponseCall) -> Self {
                    (
                        value.decryptionId,
                        value.userDecryptedShare,
                        value.signature,
                        value.extraData,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for userDecryptionResponseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        decryptionId: tuple.0,
                        userDecryptedShare: tuple.1,
                        signature: tuple.2,
                        extraData: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<userDecryptionResponseReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: userDecryptionResponseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for userDecryptionResponseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl userDecryptionResponseReturn {
            fn _tokenize(
                &self,
            ) -> <userDecryptionResponseCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for userDecryptionResponseCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = userDecryptionResponseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "userDecryptionResponse(uint256,bytes,bytes,bytes)";
            const SELECTOR: [u8; 4] = [4u8, 111u8, 158u8, 179u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.decryptionId),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.userDecryptedShare,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                userDecryptionResponseReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`Decryption`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum DecryptionCalls {
        #[allow(missing_docs)]
        UPGRADE_INTERFACE_VERSION(UPGRADE_INTERFACE_VERSIONCall),
        #[allow(missing_docs)]
        acceptOwnership(acceptOwnershipCall),
        #[allow(missing_docs)]
        checkDecryptionDone(checkDecryptionDoneCall),
        #[allow(missing_docs)]
        checkDelegatedUserDecryptionReady(checkDelegatedUserDecryptionReadyCall),
        #[allow(missing_docs)]
        checkPublicDecryptionReady(checkPublicDecryptionReadyCall),
        #[allow(missing_docs)]
        checkUserDecryptionReady(checkUserDecryptionReadyCall),
        #[allow(missing_docs)]
        delegatedUserDecryptionRequest(delegatedUserDecryptionRequestCall),
        #[allow(missing_docs)]
        eip712Domain(eip712DomainCall),
        #[allow(missing_docs)]
        getDecryptionConsensusTxSenders(getDecryptionConsensusTxSendersCall),
        #[allow(missing_docs)]
        getVersion(getVersionCall),
        #[allow(missing_docs)]
        initializeFromEmptyProxy(initializeFromEmptyProxyCall),
        #[allow(missing_docs)]
        owner(ownerCall),
        #[allow(missing_docs)]
        pause(pauseCall),
        #[allow(missing_docs)]
        paused(pausedCall),
        #[allow(missing_docs)]
        pendingOwner(pendingOwnerCall),
        #[allow(missing_docs)]
        proxiableUUID(proxiableUUIDCall),
        #[allow(missing_docs)]
        publicDecryptionRequest(publicDecryptionRequestCall),
        #[allow(missing_docs)]
        publicDecryptionResponse(publicDecryptionResponseCall),
        #[allow(missing_docs)]
        reinitializeV4(reinitializeV4Call),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        unpause(unpauseCall),
        #[allow(missing_docs)]
        upgradeToAndCall(upgradeToAndCallCall),
        #[allow(missing_docs)]
        userDecryptionRequest(userDecryptionRequestCall),
        #[allow(missing_docs)]
        userDecryptionResponse(userDecryptionResponseCall),
    }
    #[automatically_derived]
    impl DecryptionCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [4u8, 111u8, 158u8, 179u8],
            [9u8, 0u8, 204u8, 105u8],
            [13u8, 142u8, 110u8, 44u8],
            [18u8, 58u8, 187u8, 40u8],
            [57u8, 247u8, 56u8, 16u8],
            [63u8, 75u8, 168u8, 58u8],
            [79u8, 30u8, 242u8, 134u8],
            [82u8, 209u8, 144u8, 45u8],
            [92u8, 151u8, 90u8, 187u8],
            [111u8, 137u8, 19u8, 188u8],
            [113u8, 80u8, 24u8, 166u8],
            [121u8, 186u8, 80u8, 151u8],
            [132u8, 86u8, 203u8, 89u8],
            [132u8, 176u8, 25u8, 110u8],
            [141u8, 165u8, 203u8, 91u8],
            [159u8, 152u8, 216u8, 153u8],
            [159u8, 173u8, 90u8, 47u8],
            [166u8, 9u8, 4u8, 57u8],
            [173u8, 60u8, 177u8, 204u8],
            [216u8, 153u8, 143u8, 69u8],
            [227u8, 12u8, 57u8, 120u8],
            [227u8, 30u8, 193u8, 157u8],
            [241u8, 181u8, 122u8, 219u8],
            [242u8, 253u8, 227u8, 139u8],
            [244u8, 166u8, 226u8, 206u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for DecryptionCalls {
        const NAME: &'static str = "DecryptionCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 25usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::UPGRADE_INTERFACE_VERSION(_) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::acceptOwnership(_) => {
                    <acceptOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkDecryptionDone(_) => {
                    <checkDecryptionDoneCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkDelegatedUserDecryptionReady(_) => {
                    <checkDelegatedUserDecryptionReadyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkPublicDecryptionReady(_) => {
                    <checkPublicDecryptionReadyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkUserDecryptionReady(_) => {
                    <checkUserDecryptionReadyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegatedUserDecryptionRequest(_) => {
                    <delegatedUserDecryptionRequestCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eip712Domain(_) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getDecryptionConsensusTxSenders(_) => {
                    <getDecryptionConsensusTxSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getVersion(_) => {
                    <getVersionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initializeFromEmptyProxy(_) => {
                    <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused(_) => <pausedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pendingOwner(_) => {
                    <pendingOwnerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proxiableUUID(_) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::publicDecryptionRequest(_) => {
                    <publicDecryptionRequestCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::publicDecryptionResponse(_) => {
                    <publicDecryptionResponseCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::reinitializeV4(_) => {
                    <reinitializeV4Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unpause(_) => <unpauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::upgradeToAndCall(_) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::userDecryptionRequest(_) => {
                    <userDecryptionRequestCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::userDecryptionResponse(_) => {
                    <userDecryptionResponseCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<DecryptionCalls>] = &[
                {
                    fn userDecryptionResponse(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <userDecryptionResponseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::userDecryptionResponse)
                    }
                    userDecryptionResponse
                },
                {
                    fn getDecryptionConsensusTxSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <getDecryptionConsensusTxSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::getDecryptionConsensusTxSenders)
                    }
                    getDecryptionConsensusTxSenders
                },
                {
                    fn getVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <getVersionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::getVersion)
                    }
                    getVersion
                },
                {
                    fn reinitializeV4(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <reinitializeV4Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::reinitializeV4)
                    }
                    reinitializeV4
                },
                {
                    fn initializeFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::initializeFromEmptyProxy)
                    }
                    initializeFromEmptyProxy
                },
                {
                    fn unpause(data: &[u8]) -> alloy_sol_types::Result<DecryptionCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(DecryptionCalls::unpause)
                    }
                    unpause
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn paused(data: &[u8]) -> alloy_sol_types::Result<DecryptionCalls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(DecryptionCalls::paused)
                    }
                    paused
                },
                {
                    fn publicDecryptionResponse(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <publicDecryptionResponseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::publicDecryptionResponse)
                    }
                    publicDecryptionResponse
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn acceptOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <acceptOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::acceptOwnership)
                    }
                    acceptOwnership
                },
                {
                    fn pause(data: &[u8]) -> alloy_sol_types::Result<DecryptionCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(DecryptionCalls::pause)
                    }
                    pause
                },
                {
                    fn eip712Domain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <eip712DomainCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::eip712Domain)
                    }
                    eip712Domain
                },
                {
                    fn owner(data: &[u8]) -> alloy_sol_types::Result<DecryptionCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(DecryptionCalls::owner)
                    }
                    owner
                },
                {
                    fn checkDelegatedUserDecryptionReady(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <checkDelegatedUserDecryptionReadyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::checkDelegatedUserDecryptionReady)
                    }
                    checkDelegatedUserDecryptionReady
                },
                {
                    fn delegatedUserDecryptionRequest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <delegatedUserDecryptionRequestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::delegatedUserDecryptionRequest)
                    }
                    delegatedUserDecryptionRequest
                },
                {
                    fn checkDecryptionDone(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <checkDecryptionDoneCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::checkDecryptionDone)
                    }
                    checkDecryptionDone
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn publicDecryptionRequest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <publicDecryptionRequestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::publicDecryptionRequest)
                    }
                    publicDecryptionRequest
                },
                {
                    fn pendingOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <pendingOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::pendingOwner)
                    }
                    pendingOwner
                },
                {
                    fn checkPublicDecryptionReady(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <checkPublicDecryptionReadyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::checkPublicDecryptionReady)
                    }
                    checkPublicDecryptionReady
                },
                {
                    fn userDecryptionRequest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <userDecryptionRequestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::userDecryptionRequest)
                    }
                    userDecryptionRequest
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn checkUserDecryptionReady(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <checkUserDecryptionReadyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::checkUserDecryptionReady)
                    }
                    checkUserDecryptionReady
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<DecryptionCalls>] = &[
                {
                    fn userDecryptionResponse(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <userDecryptionResponseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::userDecryptionResponse)
                    }
                    userDecryptionResponse
                },
                {
                    fn getDecryptionConsensusTxSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <getDecryptionConsensusTxSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::getDecryptionConsensusTxSenders)
                    }
                    getDecryptionConsensusTxSenders
                },
                {
                    fn getVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <getVersionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::getVersion)
                    }
                    getVersion
                },
                {
                    fn reinitializeV4(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <reinitializeV4Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::reinitializeV4)
                    }
                    reinitializeV4
                },
                {
                    fn initializeFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::initializeFromEmptyProxy)
                    }
                    initializeFromEmptyProxy
                },
                {
                    fn unpause(data: &[u8]) -> alloy_sol_types::Result<DecryptionCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::unpause)
                    }
                    unpause
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn paused(data: &[u8]) -> alloy_sol_types::Result<DecryptionCalls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::paused)
                    }
                    paused
                },
                {
                    fn publicDecryptionResponse(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <publicDecryptionResponseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::publicDecryptionResponse)
                    }
                    publicDecryptionResponse
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn acceptOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <acceptOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::acceptOwnership)
                    }
                    acceptOwnership
                },
                {
                    fn pause(data: &[u8]) -> alloy_sol_types::Result<DecryptionCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::pause)
                    }
                    pause
                },
                {
                    fn eip712Domain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <eip712DomainCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::eip712Domain)
                    }
                    eip712Domain
                },
                {
                    fn owner(data: &[u8]) -> alloy_sol_types::Result<DecryptionCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::owner)
                    }
                    owner
                },
                {
                    fn checkDelegatedUserDecryptionReady(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <checkDelegatedUserDecryptionReadyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::checkDelegatedUserDecryptionReady)
                    }
                    checkDelegatedUserDecryptionReady
                },
                {
                    fn delegatedUserDecryptionRequest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <delegatedUserDecryptionRequestCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::delegatedUserDecryptionRequest)
                    }
                    delegatedUserDecryptionRequest
                },
                {
                    fn checkDecryptionDone(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <checkDecryptionDoneCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::checkDecryptionDone)
                    }
                    checkDecryptionDone
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn publicDecryptionRequest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <publicDecryptionRequestCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::publicDecryptionRequest)
                    }
                    publicDecryptionRequest
                },
                {
                    fn pendingOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <pendingOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::pendingOwner)
                    }
                    pendingOwner
                },
                {
                    fn checkPublicDecryptionReady(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <checkPublicDecryptionReadyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::checkPublicDecryptionReady)
                    }
                    checkPublicDecryptionReady
                },
                {
                    fn userDecryptionRequest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <userDecryptionRequestCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::userDecryptionRequest)
                    }
                    userDecryptionRequest
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn checkUserDecryptionReady(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <checkUserDecryptionReadyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::checkUserDecryptionReady)
                    }
                    checkUserDecryptionReady
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::UPGRADE_INTERFACE_VERSION(inner) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::acceptOwnership(inner) => {
                    <acceptOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::checkDecryptionDone(inner) => {
                    <checkDecryptionDoneCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::checkDelegatedUserDecryptionReady(inner) => {
                    <checkDelegatedUserDecryptionReadyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::checkPublicDecryptionReady(inner) => {
                    <checkPublicDecryptionReadyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::checkUserDecryptionReady(inner) => {
                    <checkUserDecryptionReadyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegatedUserDecryptionRequest(inner) => {
                    <delegatedUserDecryptionRequestCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::eip712Domain(inner) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getDecryptionConsensusTxSenders(inner) => {
                    <getDecryptionConsensusTxSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getVersion(inner) => {
                    <getVersionCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::initializeFromEmptyProxy(inner) => {
                    <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::paused(inner) => {
                    <pausedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pendingOwner(inner) => {
                    <pendingOwnerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proxiableUUID(inner) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::publicDecryptionRequest(inner) => {
                    <publicDecryptionRequestCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::publicDecryptionResponse(inner) => {
                    <publicDecryptionResponseCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::reinitializeV4(inner) => {
                    <reinitializeV4Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::upgradeToAndCall(inner) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::userDecryptionRequest(inner) => {
                    <userDecryptionRequestCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::userDecryptionResponse(inner) => {
                    <userDecryptionResponseCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::UPGRADE_INTERFACE_VERSION(inner) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::acceptOwnership(inner) => {
                    <acceptOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::checkDecryptionDone(inner) => {
                    <checkDecryptionDoneCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::checkDelegatedUserDecryptionReady(inner) => {
                    <checkDelegatedUserDecryptionReadyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::checkPublicDecryptionReady(inner) => {
                    <checkPublicDecryptionReadyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::checkUserDecryptionReady(inner) => {
                    <checkUserDecryptionReadyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegatedUserDecryptionRequest(inner) => {
                    <delegatedUserDecryptionRequestCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::eip712Domain(inner) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getDecryptionConsensusTxSenders(inner) => {
                    <getDecryptionConsensusTxSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getVersion(inner) => {
                    <getVersionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initializeFromEmptyProxy(inner) => {
                    <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::paused(inner) => {
                    <pausedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pendingOwner(inner) => {
                    <pendingOwnerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proxiableUUID(inner) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::publicDecryptionRequest(inner) => {
                    <publicDecryptionRequestCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::publicDecryptionResponse(inner) => {
                    <publicDecryptionResponseCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::reinitializeV4(inner) => {
                    <reinitializeV4Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::upgradeToAndCall(inner) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::userDecryptionRequest(inner) => {
                    <userDecryptionRequestCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::userDecryptionResponse(inner) => {
                    <userDecryptionResponseCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`Decryption`](self) custom errors.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum DecryptionErrors {
        #[allow(missing_docs)]
        AddressEmptyCode(AddressEmptyCode),
        #[allow(missing_docs)]
        ContractAddressesMaxLengthExceeded(ContractAddressesMaxLengthExceeded),
        #[allow(missing_docs)]
        ContractNotInContractAddresses(ContractNotInContractAddresses),
        #[allow(missing_docs)]
        DecryptionNotDone(DecryptionNotDone),
        #[allow(missing_docs)]
        DecryptionNotRequested(DecryptionNotRequested),
        #[allow(missing_docs)]
        DelegatorAddressInContractAddresses(DelegatorAddressInContractAddresses),
        #[allow(missing_docs)]
        DifferentKeyIdsNotAllowed(DifferentKeyIdsNotAllowed),
        #[allow(missing_docs)]
        ECDSAInvalidSignature(ECDSAInvalidSignature),
        #[allow(missing_docs)]
        ECDSAInvalidSignatureLength(ECDSAInvalidSignatureLength),
        #[allow(missing_docs)]
        ECDSAInvalidSignatureS(ECDSAInvalidSignatureS),
        #[allow(missing_docs)]
        ERC1967InvalidImplementation(ERC1967InvalidImplementation),
        #[allow(missing_docs)]
        ERC1967NonPayable(ERC1967NonPayable),
        #[allow(missing_docs)]
        EmptyContractAddresses(EmptyContractAddresses),
        #[allow(missing_docs)]
        EmptyCtHandleContractPairs(EmptyCtHandleContractPairs),
        #[allow(missing_docs)]
        EmptyCtHandles(EmptyCtHandles),
        #[allow(missing_docs)]
        EnforcedPause(EnforcedPause),
        #[allow(missing_docs)]
        ExpectedPause(ExpectedPause),
        #[allow(missing_docs)]
        FailedCall(FailedCall),
        #[allow(missing_docs)]
        InvalidFHEType(InvalidFHEType),
        #[allow(missing_docs)]
        InvalidInitialization(InvalidInitialization),
        #[allow(missing_docs)]
        InvalidNullDurationDays(InvalidNullDurationDays),
        #[allow(missing_docs)]
        InvalidUserSignature(InvalidUserSignature),
        #[allow(missing_docs)]
        KmsNodeAlreadySigned(KmsNodeAlreadySigned),
        #[allow(missing_docs)]
        MaxDecryptionRequestBitSizeExceeded(MaxDecryptionRequestBitSizeExceeded),
        #[allow(missing_docs)]
        MaxDurationDaysExceeded(MaxDurationDaysExceeded),
        #[allow(missing_docs)]
        NotGatewayOwner(NotGatewayOwner),
        #[allow(missing_docs)]
        NotInitializing(NotInitializing),
        #[allow(missing_docs)]
        NotInitializingFromEmptyProxy(NotInitializingFromEmptyProxy),
        #[allow(missing_docs)]
        NotOwnerOrGatewayConfig(NotOwnerOrGatewayConfig),
        #[allow(missing_docs)]
        NotPauser(NotPauser),
        #[allow(missing_docs)]
        NotPauserOrGatewayConfig(NotPauserOrGatewayConfig),
        #[allow(missing_docs)]
        OwnableInvalidOwner(OwnableInvalidOwner),
        #[allow(missing_docs)]
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        #[allow(missing_docs)]
        StartTimestampInFuture(StartTimestampInFuture),
        #[allow(missing_docs)]
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        #[allow(missing_docs)]
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
        #[allow(missing_docs)]
        UnsupportedFHEType(UnsupportedFHEType),
        #[allow(missing_docs)]
        UserAddressInContractAddresses(UserAddressInContractAddresses),
        #[allow(missing_docs)]
        UserDecryptionRequestExpired(UserDecryptionRequestExpired),
    }
    #[automatically_derived]
    impl DecryptionErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [11u8, 240u8, 20u8, 6u8],
            [14u8, 86u8, 207u8, 61u8],
            [17u8, 140u8, 218u8, 167u8],
            [30u8, 79u8, 189u8, 247u8],
            [32u8, 106u8, 52u8, 110u8],
            [42u8, 135u8, 61u8, 39u8],
            [45u8, 231u8, 84u8, 56u8],
            [48u8, 52u8, 128u8, 64u8],
            [50u8, 149u8, 24u8, 99u8],
            [56u8, 137u8, 22u8, 187u8],
            [76u8, 156u8, 140u8, 227u8],
            [87u8, 207u8, 162u8, 23u8],
            [100u8, 25u8, 80u8, 215u8],
            [111u8, 79u8, 115u8, 31u8],
            [141u8, 252u8, 32u8, 43u8],
            [153u8, 150u8, 179u8, 21u8],
            [153u8, 236u8, 72u8, 217u8],
            [164u8, 195u8, 3u8, 145u8],
            [166u8, 166u8, 203u8, 33u8],
            [170u8, 29u8, 73u8, 164u8],
            [179u8, 152u8, 151u8, 159u8],
            [190u8, 120u8, 48u8, 177u8],
            [195u8, 68u8, 106u8, 199u8],
            [197u8, 171u8, 70u8, 126u8],
            [207u8, 174u8, 146u8, 31u8],
            [212u8, 138u8, 249u8, 66u8],
            [214u8, 189u8, 162u8, 117u8],
            [215u8, 139u8, 206u8, 12u8],
            [215u8, 230u8, 188u8, 248u8],
            [217u8, 60u8, 6u8, 101u8],
            [220u8, 77u8, 120u8, 177u8],
            [222u8, 40u8, 89u8, 193u8],
            [224u8, 124u8, 141u8, 186u8],
            [225u8, 145u8, 102u8, 238u8],
            [231u8, 244u8, 137u8, 93u8],
            [242u8, 76u8, 8u8, 135u8],
            [246u8, 69u8, 238u8, 223u8],
            [249u8, 46u8, 232u8, 169u8],
            [252u8, 230u8, 152u8, 247u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for DecryptionErrors {
        const NAME: &'static str = "DecryptionErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 39usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AddressEmptyCode(_) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ContractAddressesMaxLengthExceeded(_) => {
                    <ContractAddressesMaxLengthExceeded as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ContractNotInContractAddresses(_) => {
                    <ContractNotInContractAddresses as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DecryptionNotDone(_) => {
                    <DecryptionNotDone as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DecryptionNotRequested(_) => {
                    <DecryptionNotRequested as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DelegatorAddressInContractAddresses(_) => {
                    <DelegatorAddressInContractAddresses as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DifferentKeyIdsNotAllowed(_) => {
                    <DifferentKeyIdsNotAllowed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignature(_) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignatureLength(_) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignatureS(_) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC1967InvalidImplementation(_) => {
                    <ERC1967InvalidImplementation as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC1967NonPayable(_) => {
                    <ERC1967NonPayable as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyContractAddresses(_) => {
                    <EmptyContractAddresses as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyCtHandleContractPairs(_) => {
                    <EmptyCtHandleContractPairs as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyCtHandles(_) => {
                    <EmptyCtHandles as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EnforcedPause(_) => {
                    <EnforcedPause as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ExpectedPause(_) => {
                    <ExpectedPause as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FailedCall(_) => {
                    <FailedCall as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidFHEType(_) => {
                    <InvalidFHEType as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidInitialization(_) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidNullDurationDays(_) => {
                    <InvalidNullDurationDays as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidUserSignature(_) => {
                    <InvalidUserSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsNodeAlreadySigned(_) => {
                    <KmsNodeAlreadySigned as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MaxDecryptionRequestBitSizeExceeded(_) => {
                    <MaxDecryptionRequestBitSizeExceeded as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MaxDurationDaysExceeded(_) => {
                    <MaxDurationDaysExceeded as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotGatewayOwner(_) => {
                    <NotGatewayOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotInitializing(_) => {
                    <NotInitializing as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotInitializingFromEmptyProxy(_) => {
                    <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotOwnerOrGatewayConfig(_) => {
                    <NotOwnerOrGatewayConfig as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotPauser(_) => <NotPauser as alloy_sol_types::SolError>::SELECTOR,
                Self::NotPauserOrGatewayConfig(_) => {
                    <NotPauserOrGatewayConfig as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OwnableInvalidOwner(_) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OwnableUnauthorizedAccount(_) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::StartTimestampInFuture(_) => {
                    <StartTimestampInFuture as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UUPSUnauthorizedCallContext(_) => {
                    <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UUPSUnsupportedProxiableUUID(_) => {
                    <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnsupportedFHEType(_) => {
                    <UnsupportedFHEType as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UserAddressInContractAddresses(_) => {
                    <UserAddressInContractAddresses as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UserDecryptionRequestExpired(_) => {
                    <UserDecryptionRequestExpired as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<DecryptionErrors>] = &[
                {
                    fn DecryptionNotDone(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <DecryptionNotDone as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::DecryptionNotDone)
                    }
                    DecryptionNotDone
                },
                {
                    fn NotGatewayOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotGatewayOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::NotGatewayOwner)
                    }
                    NotGatewayOwner
                },
                {
                    fn OwnableUnauthorizedAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::OwnableUnauthorizedAccount)
                    }
                    OwnableUnauthorizedAccount
                },
                {
                    fn OwnableInvalidOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::OwnableInvalidOwner)
                    }
                    OwnableInvalidOwner
                },
                {
                    fn NotPauser(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotPauser as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(DecryptionErrors::NotPauser)
                    }
                    NotPauser
                },
                {
                    fn InvalidUserSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <InvalidUserSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::InvalidUserSignature)
                    }
                    InvalidUserSignature
                },
                {
                    fn EmptyCtHandles(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <EmptyCtHandles as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::EmptyCtHandles)
                    }
                    EmptyCtHandles
                },
                {
                    fn UserDecryptionRequestExpired(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UserDecryptionRequestExpired as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::UserDecryptionRequestExpired)
                    }
                    UserDecryptionRequestExpired
                },
                {
                    fn MaxDurationDaysExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <MaxDurationDaysExceeded as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::MaxDurationDaysExceeded)
                    }
                    MaxDurationDaysExceeded
                },
                {
                    fn NotPauserOrGatewayConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotPauserOrGatewayConfig as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::NotPauserOrGatewayConfig)
                    }
                    NotPauserOrGatewayConfig
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn EmptyContractAddresses(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <EmptyContractAddresses as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::EmptyContractAddresses)
                    }
                    EmptyContractAddresses
                },
                {
                    fn InvalidFHEType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <InvalidFHEType as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::InvalidFHEType)
                    }
                    InvalidFHEType
                },
                {
                    fn NotInitializingFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::NotInitializingFromEmptyProxy)
                    }
                    NotInitializingFromEmptyProxy
                },
                {
                    fn ExpectedPause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ExpectedPause as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::ExpectedPause)
                    }
                    ExpectedPause
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn KmsNodeAlreadySigned(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <KmsNodeAlreadySigned as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::KmsNodeAlreadySigned)
                    }
                    KmsNodeAlreadySigned
                },
                {
                    fn ContractNotInContractAddresses(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ContractNotInContractAddresses as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::ContractNotInContractAddresses)
                    }
                    ContractNotInContractAddresses
                },
                {
                    fn EmptyCtHandleContractPairs(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <EmptyCtHandleContractPairs as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::EmptyCtHandleContractPairs)
                    }
                    EmptyCtHandleContractPairs
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn UnsupportedFHEType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UnsupportedFHEType as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::UnsupportedFHEType)
                    }
                    UnsupportedFHEType
                },
                {
                    fn DelegatorAddressInContractAddresses(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <DelegatorAddressInContractAddresses as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::DelegatorAddressInContractAddresses)
                    }
                    DelegatorAddressInContractAddresses
                },
                {
                    fn ContractAddressesMaxLengthExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ContractAddressesMaxLengthExceeded as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::ContractAddressesMaxLengthExceeded)
                    }
                    ContractAddressesMaxLengthExceeded
                },
                {
                    fn DifferentKeyIdsNotAllowed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <DifferentKeyIdsNotAllowed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::DifferentKeyIdsNotAllowed)
                    }
                    DifferentKeyIdsNotAllowed
                },
                {
                    fn DecryptionNotRequested(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <DecryptionNotRequested as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::DecryptionNotRequested)
                    }
                    DecryptionNotRequested
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(DecryptionErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn EnforcedPause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <EnforcedPause as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::EnforcedPause)
                    }
                    EnforcedPause
                },
                {
                    fn UserAddressInContractAddresses(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UserAddressInContractAddresses as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::UserAddressInContractAddresses)
                    }
                    UserAddressInContractAddresses
                },
                {
                    fn InvalidNullDurationDays(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <InvalidNullDurationDays as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::InvalidNullDurationDays)
                    }
                    InvalidNullDurationDays
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn NotOwnerOrGatewayConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotOwnerOrGatewayConfig as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::NotOwnerOrGatewayConfig)
                    }
                    NotOwnerOrGatewayConfig
                },
                {
                    fn MaxDecryptionRequestBitSizeExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <MaxDecryptionRequestBitSizeExceeded as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::MaxDecryptionRequestBitSizeExceeded)
                    }
                    MaxDecryptionRequestBitSizeExceeded
                },
                {
                    fn StartTimestampInFuture(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <StartTimestampInFuture as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::StartTimestampInFuture)
                    }
                    StartTimestampInFuture
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::InvalidInitialization)
                    }
                    InvalidInitialization
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::ECDSAInvalidSignatureLength)
                    }
                    ECDSAInvalidSignatureLength
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<DecryptionErrors>] = &[
                {
                    fn DecryptionNotDone(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <DecryptionNotDone as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::DecryptionNotDone)
                    }
                    DecryptionNotDone
                },
                {
                    fn NotGatewayOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotGatewayOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::NotGatewayOwner)
                    }
                    NotGatewayOwner
                },
                {
                    fn OwnableUnauthorizedAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::OwnableUnauthorizedAccount)
                    }
                    OwnableUnauthorizedAccount
                },
                {
                    fn OwnableInvalidOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::OwnableInvalidOwner)
                    }
                    OwnableInvalidOwner
                },
                {
                    fn NotPauser(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotPauser as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::NotPauser)
                    }
                    NotPauser
                },
                {
                    fn InvalidUserSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <InvalidUserSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::InvalidUserSignature)
                    }
                    InvalidUserSignature
                },
                {
                    fn EmptyCtHandles(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <EmptyCtHandles as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::EmptyCtHandles)
                    }
                    EmptyCtHandles
                },
                {
                    fn UserDecryptionRequestExpired(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UserDecryptionRequestExpired as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::UserDecryptionRequestExpired)
                    }
                    UserDecryptionRequestExpired
                },
                {
                    fn MaxDurationDaysExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <MaxDurationDaysExceeded as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::MaxDurationDaysExceeded)
                    }
                    MaxDurationDaysExceeded
                },
                {
                    fn NotPauserOrGatewayConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotPauserOrGatewayConfig as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::NotPauserOrGatewayConfig)
                    }
                    NotPauserOrGatewayConfig
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn EmptyContractAddresses(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <EmptyContractAddresses as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::EmptyContractAddresses)
                    }
                    EmptyContractAddresses
                },
                {
                    fn InvalidFHEType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <InvalidFHEType as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::InvalidFHEType)
                    }
                    InvalidFHEType
                },
                {
                    fn NotInitializingFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::NotInitializingFromEmptyProxy)
                    }
                    NotInitializingFromEmptyProxy
                },
                {
                    fn ExpectedPause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ExpectedPause as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::ExpectedPause)
                    }
                    ExpectedPause
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn KmsNodeAlreadySigned(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <KmsNodeAlreadySigned as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::KmsNodeAlreadySigned)
                    }
                    KmsNodeAlreadySigned
                },
                {
                    fn ContractNotInContractAddresses(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ContractNotInContractAddresses as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::ContractNotInContractAddresses)
                    }
                    ContractNotInContractAddresses
                },
                {
                    fn EmptyCtHandleContractPairs(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <EmptyCtHandleContractPairs as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::EmptyCtHandleContractPairs)
                    }
                    EmptyCtHandleContractPairs
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn UnsupportedFHEType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UnsupportedFHEType as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::UnsupportedFHEType)
                    }
                    UnsupportedFHEType
                },
                {
                    fn DelegatorAddressInContractAddresses(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <DelegatorAddressInContractAddresses as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::DelegatorAddressInContractAddresses)
                    }
                    DelegatorAddressInContractAddresses
                },
                {
                    fn ContractAddressesMaxLengthExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ContractAddressesMaxLengthExceeded as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::ContractAddressesMaxLengthExceeded)
                    }
                    ContractAddressesMaxLengthExceeded
                },
                {
                    fn DifferentKeyIdsNotAllowed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <DifferentKeyIdsNotAllowed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::DifferentKeyIdsNotAllowed)
                    }
                    DifferentKeyIdsNotAllowed
                },
                {
                    fn DecryptionNotRequested(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <DecryptionNotRequested as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::DecryptionNotRequested)
                    }
                    DecryptionNotRequested
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn EnforcedPause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <EnforcedPause as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::EnforcedPause)
                    }
                    EnforcedPause
                },
                {
                    fn UserAddressInContractAddresses(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UserAddressInContractAddresses as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::UserAddressInContractAddresses)
                    }
                    UserAddressInContractAddresses
                },
                {
                    fn InvalidNullDurationDays(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <InvalidNullDurationDays as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::InvalidNullDurationDays)
                    }
                    InvalidNullDurationDays
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn NotOwnerOrGatewayConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotOwnerOrGatewayConfig as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::NotOwnerOrGatewayConfig)
                    }
                    NotOwnerOrGatewayConfig
                },
                {
                    fn MaxDecryptionRequestBitSizeExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <MaxDecryptionRequestBitSizeExceeded as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::MaxDecryptionRequestBitSizeExceeded)
                    }
                    MaxDecryptionRequestBitSizeExceeded
                },
                {
                    fn StartTimestampInFuture(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <StartTimestampInFuture as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::StartTimestampInFuture)
                    }
                    StartTimestampInFuture
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::InvalidInitialization)
                    }
                    InvalidInitialization
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::ECDSAInvalidSignatureLength)
                    }
                    ECDSAInvalidSignatureLength
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::AddressEmptyCode(inner) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ContractAddressesMaxLengthExceeded(inner) => {
                    <ContractAddressesMaxLengthExceeded as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ContractNotInContractAddresses(inner) => {
                    <ContractNotInContractAddresses as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DecryptionNotDone(inner) => {
                    <DecryptionNotDone as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DecryptionNotRequested(inner) => {
                    <DecryptionNotRequested as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DelegatorAddressInContractAddresses(inner) => {
                    <DelegatorAddressInContractAddresses as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DifferentKeyIdsNotAllowed(inner) => {
                    <DifferentKeyIdsNotAllowed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignature(inner) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignatureLength(inner) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignatureS(inner) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC1967InvalidImplementation(inner) => {
                    <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC1967NonPayable(inner) => {
                    <ERC1967NonPayable as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EmptyContractAddresses(inner) => {
                    <EmptyContractAddresses as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EmptyCtHandleContractPairs(inner) => {
                    <EmptyCtHandleContractPairs as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EmptyCtHandles(inner) => {
                    <EmptyCtHandles as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EnforcedPause(inner) => {
                    <EnforcedPause as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ExpectedPause(inner) => {
                    <ExpectedPause as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidFHEType(inner) => {
                    <InvalidFHEType as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidInitialization(inner) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidNullDurationDays(inner) => {
                    <InvalidNullDurationDays as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidUserSignature(inner) => {
                    <InvalidUserSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KmsNodeAlreadySigned(inner) => {
                    <KmsNodeAlreadySigned as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MaxDecryptionRequestBitSizeExceeded(inner) => {
                    <MaxDecryptionRequestBitSizeExceeded as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MaxDurationDaysExceeded(inner) => {
                    <MaxDurationDaysExceeded as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotGatewayOwner(inner) => {
                    <NotGatewayOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotInitializing(inner) => {
                    <NotInitializing as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotInitializingFromEmptyProxy(inner) => {
                    <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotOwnerOrGatewayConfig(inner) => {
                    <NotOwnerOrGatewayConfig as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotPauser(inner) => {
                    <NotPauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NotPauserOrGatewayConfig(inner) => {
                    <NotPauserOrGatewayConfig as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OwnableInvalidOwner(inner) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OwnableUnauthorizedAccount(inner) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::StartTimestampInFuture(inner) => {
                    <StartTimestampInFuture as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UUPSUnauthorizedCallContext(inner) => {
                    <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UUPSUnsupportedProxiableUUID(inner) => {
                    <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UnsupportedFHEType(inner) => {
                    <UnsupportedFHEType as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UserAddressInContractAddresses(inner) => {
                    <UserAddressInContractAddresses as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UserDecryptionRequestExpired(inner) => {
                    <UserDecryptionRequestExpired as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AddressEmptyCode(inner) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ContractAddressesMaxLengthExceeded(inner) => {
                    <ContractAddressesMaxLengthExceeded as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ContractNotInContractAddresses(inner) => {
                    <ContractNotInContractAddresses as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DecryptionNotDone(inner) => {
                    <DecryptionNotDone as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DecryptionNotRequested(inner) => {
                    <DecryptionNotRequested as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DelegatorAddressInContractAddresses(inner) => {
                    <DelegatorAddressInContractAddresses as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DifferentKeyIdsNotAllowed(inner) => {
                    <DifferentKeyIdsNotAllowed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignature(inner) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignatureLength(inner) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignatureS(inner) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC1967InvalidImplementation(inner) => {
                    <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC1967NonPayable(inner) => {
                    <ERC1967NonPayable as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EmptyContractAddresses(inner) => {
                    <EmptyContractAddresses as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EmptyCtHandleContractPairs(inner) => {
                    <EmptyCtHandleContractPairs as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EmptyCtHandles(inner) => {
                    <EmptyCtHandles as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EnforcedPause(inner) => {
                    <EnforcedPause as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ExpectedPause(inner) => {
                    <ExpectedPause as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidFHEType(inner) => {
                    <InvalidFHEType as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidInitialization(inner) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidNullDurationDays(inner) => {
                    <InvalidNullDurationDays as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidUserSignature(inner) => {
                    <InvalidUserSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KmsNodeAlreadySigned(inner) => {
                    <KmsNodeAlreadySigned as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MaxDecryptionRequestBitSizeExceeded(inner) => {
                    <MaxDecryptionRequestBitSizeExceeded as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MaxDurationDaysExceeded(inner) => {
                    <MaxDurationDaysExceeded as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotGatewayOwner(inner) => {
                    <NotGatewayOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotInitializing(inner) => {
                    <NotInitializing as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotInitializingFromEmptyProxy(inner) => {
                    <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotOwnerOrGatewayConfig(inner) => {
                    <NotOwnerOrGatewayConfig as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotPauser(inner) => {
                    <NotPauser as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::NotPauserOrGatewayConfig(inner) => {
                    <NotPauserOrGatewayConfig as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OwnableInvalidOwner(inner) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OwnableUnauthorizedAccount(inner) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::StartTimestampInFuture(inner) => {
                    <StartTimestampInFuture as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UUPSUnauthorizedCallContext(inner) => {
                    <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UUPSUnsupportedProxiableUUID(inner) => {
                    <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UnsupportedFHEType(inner) => {
                    <UnsupportedFHEType as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UserAddressInContractAddresses(inner) => {
                    <UserAddressInContractAddresses as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UserDecryptionRequestExpired(inner) => {
                    <UserDecryptionRequestExpired as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`Decryption`](self) events.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum DecryptionEvents {
        #[allow(missing_docs)]
        EIP712DomainChanged(EIP712DomainChanged),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        OwnershipTransferStarted(OwnershipTransferStarted),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        Paused(Paused),
        #[allow(missing_docs)]
        PublicDecryptionRequest(PublicDecryptionRequest),
        #[allow(missing_docs)]
        PublicDecryptionResponse(PublicDecryptionResponse),
        #[allow(missing_docs)]
        Unpaused(Unpaused),
        #[allow(missing_docs)]
        Upgraded(Upgraded),
        #[allow(missing_docs)]
        UserDecryptionRequest(UserDecryptionRequest),
        #[allow(missing_docs)]
        UserDecryptionResponse(UserDecryptionResponse),
    }
    #[automatically_derived]
    impl DecryptionEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                10u8, 99u8, 135u8, 201u8, 234u8, 54u8, 40u8, 184u8, 138u8, 99u8, 59u8,
                180u8, 243u8, 177u8, 81u8, 119u8, 15u8, 112u8, 8u8, 81u8, 23u8, 161u8,
                95u8, 155u8, 243u8, 120u8, 124u8, 218u8, 83u8, 241u8, 61u8, 49u8,
            ],
            [
                28u8, 191u8, 28u8, 110u8, 158u8, 163u8, 219u8, 207u8, 212u8, 155u8,
                150u8, 144u8, 217u8, 60u8, 212u8, 185u8, 59u8, 60u8, 178u8, 41u8, 19u8,
                249u8, 155u8, 113u8, 211u8, 174u8, 202u8, 230u8, 11u8, 37u8, 252u8, 86u8,
            ],
            [
                34u8, 219u8, 72u8, 10u8, 57u8, 189u8, 114u8, 85u8, 100u8, 56u8, 170u8,
                219u8, 74u8, 50u8, 163u8, 210u8, 166u8, 99u8, 139u8, 135u8, 192u8, 59u8,
                190u8, 197u8, 254u8, 246u8, 153u8, 126u8, 16u8, 149u8, 135u8, 255u8,
            ],
            [
                56u8, 209u8, 107u8, 140u8, 172u8, 34u8, 217u8, 159u8, 199u8, 193u8, 36u8,
                185u8, 205u8, 13u8, 226u8, 211u8, 250u8, 31u8, 174u8, 244u8, 32u8, 191u8,
                231u8, 145u8, 216u8, 195u8, 98u8, 215u8, 101u8, 226u8, 39u8, 0u8,
            ],
            [
                93u8, 185u8, 238u8, 10u8, 73u8, 91u8, 242u8, 230u8, 255u8, 156u8, 145u8,
                167u8, 131u8, 76u8, 27u8, 164u8, 253u8, 210u8, 68u8, 165u8, 232u8, 170u8,
                78u8, 83u8, 123u8, 211u8, 138u8, 234u8, 228u8, 176u8, 115u8, 170u8,
            ],
            [
                98u8, 231u8, 140u8, 234u8, 1u8, 190u8, 227u8, 32u8, 205u8, 78u8, 66u8,
                2u8, 112u8, 181u8, 234u8, 116u8, 0u8, 13u8, 17u8, 176u8, 201u8, 247u8,
                71u8, 84u8, 235u8, 219u8, 252u8, 84u8, 75u8, 5u8, 162u8, 88u8,
            ],
            [
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ],
            [
                188u8, 124u8, 215u8, 90u8, 32u8, 238u8, 39u8, 253u8, 154u8, 222u8, 186u8,
                179u8, 32u8, 65u8, 247u8, 85u8, 33u8, 77u8, 188u8, 107u8, 255u8, 169u8,
                12u8, 192u8, 34u8, 91u8, 57u8, 218u8, 46u8, 92u8, 45u8, 59u8,
            ],
            [
                199u8, 245u8, 5u8, 178u8, 243u8, 113u8, 174u8, 33u8, 117u8, 238u8, 73u8,
                19u8, 244u8, 73u8, 158u8, 31u8, 38u8, 51u8, 167u8, 181u8, 147u8, 99u8,
                33u8, 238u8, 209u8, 205u8, 174u8, 182u8, 17u8, 81u8, 129u8, 210u8,
            ],
            [
                215u8, 229u8, 138u8, 54u8, 122u8, 10u8, 108u8, 41u8, 142u8, 118u8, 173u8,
                93u8, 36u8, 0u8, 4u8, 227u8, 39u8, 170u8, 20u8, 35u8, 203u8, 228u8,
                189u8, 127u8, 248u8, 93u8, 76u8, 113u8, 94u8, 248u8, 209u8, 95u8,
            ],
            [
                249u8, 1u8, 27u8, 214u8, 186u8, 13u8, 166u8, 4u8, 156u8, 82u8, 13u8,
                112u8, 254u8, 89u8, 113u8, 241u8, 126u8, 215u8, 171u8, 121u8, 84u8,
                134u8, 5u8, 37u8, 68u8, 181u8, 16u8, 25u8, 137u8, 108u8, 89u8, 107u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for DecryptionEvents {
        const NAME: &'static str = "DecryptionEvents";
        const COUNT: usize = 11usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <EIP712DomainChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <EIP712DomainChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::EIP712DomainChanged)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::Initialized)
                }
                Some(
                    <OwnershipTransferStarted as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferStarted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OwnershipTransferStarted)
                }
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(<Paused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Paused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Paused)
                }
                Some(
                    <PublicDecryptionRequest as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PublicDecryptionRequest as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PublicDecryptionRequest)
                }
                Some(
                    <PublicDecryptionResponse as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PublicDecryptionResponse as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PublicDecryptionResponse)
                }
                Some(<Unpaused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Unpaused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Unpaused)
                }
                Some(<Upgraded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Upgraded as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Upgraded)
                }
                Some(
                    <UserDecryptionRequest as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UserDecryptionRequest as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::UserDecryptionRequest)
                }
                Some(
                    <UserDecryptionResponse as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UserDecryptionResponse as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::UserDecryptionResponse)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for DecryptionEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::EIP712DomainChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferStarted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PublicDecryptionRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PublicDecryptionResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UserDecryptionRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UserDecryptionResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::EIP712DomainChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferStarted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PublicDecryptionRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PublicDecryptionResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UserDecryptionRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UserDecryptionResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`Decryption`](self) contract instance.

See the [wrapper's documentation](`DecryptionInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> DecryptionInstance<P, N> {
        DecryptionInstance::<P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<DecryptionInstance<P, N>>,
    > {
        DecryptionInstance::<P, N>::deploy(provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(provider: P) -> alloy_contract::RawCallBuilder<P, N> {
        DecryptionInstance::<P, N>::deploy_builder(provider)
    }
    /**A [`Decryption`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`Decryption`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct DecryptionInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for DecryptionInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("DecryptionInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > DecryptionInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`Decryption`](self) contract instance.

See the [wrapper's documentation](`DecryptionInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
        ) -> alloy_contract::Result<DecryptionInstance<P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                ::core::clone::Clone::clone(&BYTECODE),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> DecryptionInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> DecryptionInstance<P, N> {
            DecryptionInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > DecryptionInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`UPGRADE_INTERFACE_VERSION`] function.
        pub fn UPGRADE_INTERFACE_VERSION(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, UPGRADE_INTERFACE_VERSIONCall, N> {
            self.call_builder(&UPGRADE_INTERFACE_VERSIONCall)
        }
        ///Creates a new call builder for the [`acceptOwnership`] function.
        pub fn acceptOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, acceptOwnershipCall, N> {
            self.call_builder(&acceptOwnershipCall)
        }
        ///Creates a new call builder for the [`checkDecryptionDone`] function.
        pub fn checkDecryptionDone(
            &self,
            decryptionId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, checkDecryptionDoneCall, N> {
            self.call_builder(
                &checkDecryptionDoneCall {
                    decryptionId,
                },
            )
        }
        ///Creates a new call builder for the [`checkDelegatedUserDecryptionReady`] function.
        pub fn checkDelegatedUserDecryptionReady(
            &self,
            contractsChainId: alloy::sol_types::private::primitives::aliases::U256,
            delegationAccounts: <DelegationAccounts as alloy::sol_types::SolType>::RustType,
            ctHandleContractPairs: alloy::sol_types::private::Vec<
                <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
            >,
            contractAddresses: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            _4: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            checkDelegatedUserDecryptionReadyCall,
            N,
        > {
            self.call_builder(
                &checkDelegatedUserDecryptionReadyCall {
                    contractsChainId,
                    delegationAccounts,
                    ctHandleContractPairs,
                    contractAddresses,
                    _4,
                },
            )
        }
        ///Creates a new call builder for the [`checkPublicDecryptionReady`] function.
        pub fn checkPublicDecryptionReady(
            &self,
            ctHandles: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
            _1: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, checkPublicDecryptionReadyCall, N> {
            self.call_builder(
                &checkPublicDecryptionReadyCall {
                    ctHandles,
                    _1,
                },
            )
        }
        ///Creates a new call builder for the [`checkUserDecryptionReady`] function.
        pub fn checkUserDecryptionReady(
            &self,
            userAddress: alloy::sol_types::private::Address,
            ctHandleContractPairs: alloy::sol_types::private::Vec<
                <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
            >,
            _2: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, checkUserDecryptionReadyCall, N> {
            self.call_builder(
                &checkUserDecryptionReadyCall {
                    userAddress,
                    ctHandleContractPairs,
                    _2,
                },
            )
        }
        ///Creates a new call builder for the [`delegatedUserDecryptionRequest`] function.
        pub fn delegatedUserDecryptionRequest(
            &self,
            ctHandleContractPairs: alloy::sol_types::private::Vec<
                <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
            >,
            requestValidity: <IDecryption::RequestValidity as alloy::sol_types::SolType>::RustType,
            delegationAccounts: <DelegationAccounts as alloy::sol_types::SolType>::RustType,
            contractsInfo: <IDecryption::ContractsInfo as alloy::sol_types::SolType>::RustType,
            publicKey: alloy::sol_types::private::Bytes,
            signature: alloy::sol_types::private::Bytes,
            extraData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, delegatedUserDecryptionRequestCall, N> {
            self.call_builder(
                &delegatedUserDecryptionRequestCall {
                    ctHandleContractPairs,
                    requestValidity,
                    delegationAccounts,
                    contractsInfo,
                    publicKey,
                    signature,
                    extraData,
                },
            )
        }
        ///Creates a new call builder for the [`eip712Domain`] function.
        pub fn eip712Domain(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, eip712DomainCall, N> {
            self.call_builder(&eip712DomainCall)
        }
        ///Creates a new call builder for the [`getDecryptionConsensusTxSenders`] function.
        pub fn getDecryptionConsensusTxSenders(
            &self,
            decryptionId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getDecryptionConsensusTxSendersCall, N> {
            self.call_builder(
                &getDecryptionConsensusTxSendersCall {
                    decryptionId,
                },
            )
        }
        ///Creates a new call builder for the [`getVersion`] function.
        pub fn getVersion(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getVersionCall, N> {
            self.call_builder(&getVersionCall)
        }
        ///Creates a new call builder for the [`initializeFromEmptyProxy`] function.
        pub fn initializeFromEmptyProxy(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, initializeFromEmptyProxyCall, N> {
            self.call_builder(&initializeFromEmptyProxyCall)
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<&P, ownerCall, N> {
            self.call_builder(&ownerCall)
        }
        ///Creates a new call builder for the [`pause`] function.
        pub fn pause(&self) -> alloy_contract::SolCallBuilder<&P, pauseCall, N> {
            self.call_builder(&pauseCall)
        }
        ///Creates a new call builder for the [`paused`] function.
        pub fn paused(&self) -> alloy_contract::SolCallBuilder<&P, pausedCall, N> {
            self.call_builder(&pausedCall)
        }
        ///Creates a new call builder for the [`pendingOwner`] function.
        pub fn pendingOwner(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, pendingOwnerCall, N> {
            self.call_builder(&pendingOwnerCall)
        }
        ///Creates a new call builder for the [`proxiableUUID`] function.
        pub fn proxiableUUID(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, proxiableUUIDCall, N> {
            self.call_builder(&proxiableUUIDCall)
        }
        ///Creates a new call builder for the [`publicDecryptionRequest`] function.
        pub fn publicDecryptionRequest(
            &self,
            ctHandles: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
            extraData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, publicDecryptionRequestCall, N> {
            self.call_builder(
                &publicDecryptionRequestCall {
                    ctHandles,
                    extraData,
                },
            )
        }
        ///Creates a new call builder for the [`publicDecryptionResponse`] function.
        pub fn publicDecryptionResponse(
            &self,
            decryptionId: alloy::sol_types::private::primitives::aliases::U256,
            decryptedResult: alloy::sol_types::private::Bytes,
            signature: alloy::sol_types::private::Bytes,
            extraData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, publicDecryptionResponseCall, N> {
            self.call_builder(
                &publicDecryptionResponseCall {
                    decryptionId,
                    decryptedResult,
                    signature,
                    extraData,
                },
            )
        }
        ///Creates a new call builder for the [`reinitializeV4`] function.
        pub fn reinitializeV4(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, reinitializeV4Call, N> {
            self.call_builder(&reinitializeV4Call)
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall)
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`unpause`] function.
        pub fn unpause(&self) -> alloy_contract::SolCallBuilder<&P, unpauseCall, N> {
            self.call_builder(&unpauseCall)
        }
        ///Creates a new call builder for the [`upgradeToAndCall`] function.
        pub fn upgradeToAndCall(
            &self,
            newImplementation: alloy::sol_types::private::Address,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, upgradeToAndCallCall, N> {
            self.call_builder(
                &upgradeToAndCallCall {
                    newImplementation,
                    data,
                },
            )
        }
        ///Creates a new call builder for the [`userDecryptionRequest`] function.
        pub fn userDecryptionRequest(
            &self,
            ctHandleContractPairs: alloy::sol_types::private::Vec<
                <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
            >,
            requestValidity: <IDecryption::RequestValidity as alloy::sol_types::SolType>::RustType,
            contractsInfo: <IDecryption::ContractsInfo as alloy::sol_types::SolType>::RustType,
            userAddress: alloy::sol_types::private::Address,
            publicKey: alloy::sol_types::private::Bytes,
            signature: alloy::sol_types::private::Bytes,
            extraData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, userDecryptionRequestCall, N> {
            self.call_builder(
                &userDecryptionRequestCall {
                    ctHandleContractPairs,
                    requestValidity,
                    contractsInfo,
                    userAddress,
                    publicKey,
                    signature,
                    extraData,
                },
            )
        }
        ///Creates a new call builder for the [`userDecryptionResponse`] function.
        pub fn userDecryptionResponse(
            &self,
            decryptionId: alloy::sol_types::private::primitives::aliases::U256,
            userDecryptedShare: alloy::sol_types::private::Bytes,
            signature: alloy::sol_types::private::Bytes,
            extraData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, userDecryptionResponseCall, N> {
            self.call_builder(
                &userDecryptionResponseCall {
                    decryptionId,
                    userDecryptedShare,
                    signature,
                    extraData,
                },
            )
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > DecryptionInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`EIP712DomainChanged`] event.
        pub fn EIP712DomainChanged_filter(
            &self,
        ) -> alloy_contract::Event<&P, EIP712DomainChanged, N> {
            self.event_filter::<EIP712DomainChanged>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<&P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`OwnershipTransferStarted`] event.
        pub fn OwnershipTransferStarted_filter(
            &self,
        ) -> alloy_contract::Event<&P, OwnershipTransferStarted, N> {
            self.event_filter::<OwnershipTransferStarted>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<&P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`Paused`] event.
        pub fn Paused_filter(&self) -> alloy_contract::Event<&P, Paused, N> {
            self.event_filter::<Paused>()
        }
        ///Creates a new event filter for the [`PublicDecryptionRequest`] event.
        pub fn PublicDecryptionRequest_filter(
            &self,
        ) -> alloy_contract::Event<&P, PublicDecryptionRequest, N> {
            self.event_filter::<PublicDecryptionRequest>()
        }
        ///Creates a new event filter for the [`PublicDecryptionResponse`] event.
        pub fn PublicDecryptionResponse_filter(
            &self,
        ) -> alloy_contract::Event<&P, PublicDecryptionResponse, N> {
            self.event_filter::<PublicDecryptionResponse>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<&P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
        ///Creates a new event filter for the [`Upgraded`] event.
        pub fn Upgraded_filter(&self) -> alloy_contract::Event<&P, Upgraded, N> {
            self.event_filter::<Upgraded>()
        }
        ///Creates a new event filter for the [`UserDecryptionRequest`] event.
        pub fn UserDecryptionRequest_filter(
            &self,
        ) -> alloy_contract::Event<&P, UserDecryptionRequest, N> {
            self.event_filter::<UserDecryptionRequest>()
        }
        ///Creates a new event filter for the [`UserDecryptionResponse`] event.
        pub fn UserDecryptionResponse_filter(
            &self,
        ) -> alloy_contract::Event<&P, UserDecryptionResponse, N> {
            self.event_filter::<UserDecryptionResponse>()
        }
    }
}
