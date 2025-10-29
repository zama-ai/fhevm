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
    struct SnsCiphertextMaterial {
        bytes32 ctHandle;
        uint256 keyId;
        bytes32 snsCiphertextDigest;
        address[] coprocessorTxSenderAddresses;
    }

    error AccountNotAllowedToUseCiphertext(bytes32 ctHandle, address accountAddress);
    error AddressEmptyCode(address target);
    error ContractAddressesMaxLengthExceeded(uint256 maxLength, uint256 actualLength);
    error ContractNotInContractAddresses(address contractAddress, address[] contractAddresses);
    error CoprocessorSignerDoesNotMatchTxSender(address signerAddress, address txSenderAddress);
    error DecryptionNotRequested(uint256 decryptionId);
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
    error HostChainNotRegistered(uint256 chainId);
    error InvalidFHEType(uint8 fheTypeUint8);
    error InvalidInitialization();
    error InvalidNullDurationDays();
    error InvalidUserSignature(bytes signature);
    error KmsNodeAlreadySigned(uint256 decryptionId, address signer);
    error KmsSignerDoesNotMatchTxSender(address signerAddress, address txSenderAddress);
    error MaxDecryptionRequestBitSizeExceeded(uint256 maxBitSize, uint256 totalBitSize);
    error MaxDurationDaysExceeded(uint256 maxValue, uint256 actualValue);
    error NotCoprocessorSigner(address signerAddress);
    error NotCoprocessorTxSender(address txSenderAddress);
    error NotCustodianSigner(address signerAddress);
    error NotCustodianTxSender(address txSenderAddress);
    error NotGatewayOwner(address sender);
    error NotInitializing();
    error NotInitializingFromEmptyProxy();
    error NotKmsSigner(address signerAddress);
    error NotKmsTxSender(address txSenderAddress);
    error NotOwnerOrGatewayConfig(address notOwnerOrGatewayConfig);
    error NotPauserOrGatewayConfig(address notPauserOrGatewayConfig);
    error PublicDecryptNotAllowed(bytes32 ctHandle);
    error StartTimestampInFuture(uint256 currentTimestamp, uint256 startTimestamp);
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);
    error UnsupportedFHEType(FheType fheType);
    error UserAddressInContractAddresses(address userAddress, address[] contractAddresses);
    error UserDecryptionRequestExpired(uint256 currentTimestamp, IDecryption.RequestValidity requestValidity);

    event EIP712DomainChanged();
    event Initialized(uint64 version);
    event Paused(address account);
    event PublicDecryptionRequest(uint256 indexed decryptionId, SnsCiphertextMaterial[] snsCtMaterials, bytes extraData);
    event PublicDecryptionResponse(uint256 indexed decryptionId, bytes decryptedResult, bytes[] signatures, bytes extraData);
    event PublicDecryptionResponseCall(uint256 indexed decryptionId, bytes decryptedResult, bytes signature, address kmsTxSender, bytes extraData);
    event Unpaused(address account);
    event Upgraded(address indexed implementation);
    event UserDecryptionRequest(uint256 indexed decryptionId, SnsCiphertextMaterial[] snsCtMaterials, address userAddress, bytes publicKey, bytes extraData);
    event UserDecryptionResponse(uint256 indexed decryptionId, uint256 indexShare, bytes userDecryptedShare, bytes signature, bytes extraData);
    event UserDecryptionResponseThresholdReached(uint256 indexed decryptionId);

    constructor();

    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
    function getDecryptionConsensusTxSenders(uint256 decryptionId) external view returns (address[] memory);
    function getVersion() external pure returns (string memory);
    function initializeFromEmptyProxy() external;
    function isDecryptionDone(uint256 decryptionId) external view returns (bool);
    function isPublicDecryptionReady(bytes32[] memory ctHandles, bytes memory) external view returns (bool);
    function isUserDecryptionReady(address userAddress, CtHandleContractPair[] memory ctHandleContractPairs, bytes memory) external view returns (bool);
    function pause() external;
    function paused() external view returns (bool);
    function proxiableUUID() external view returns (bytes32);
    function publicDecryptionRequest(bytes32[] memory ctHandles, bytes memory extraData) external;
    function publicDecryptionResponse(uint256 decryptionId, bytes memory decryptedResult, bytes memory signature, bytes memory extraData) external;
    function reinitializeV2() external;
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
    "name": "isDecryptionDone",
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
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isPublicDecryptionReady",
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
    "name": "isUserDecryptionReady",
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
    "name": "reinitializeV2",
    "inputs": [],
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
    "name": "PublicDecryptionResponseCall",
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
        "name": "signature",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      },
      {
        "name": "kmsTxSender",
        "type": "address",
        "indexed": false,
        "internalType": "address"
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
        "name": "indexShare",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "userDecryptedShare",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      },
      {
        "name": "signature",
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
    "name": "UserDecryptionResponseThresholdReached",
    "inputs": [
      {
        "name": "decryptionId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "AccountNotAllowedToUseCiphertext",
    "inputs": [
      {
        "name": "ctHandle",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "accountAddress",
        "type": "address",
        "internalType": "address"
      }
    ]
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
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "CoprocessorSignerDoesNotMatchTxSender",
    "inputs": [
      {
        "name": "signerAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "txSenderAddress",
        "type": "address",
        "internalType": "address"
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
    "name": "HostChainNotRegistered",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
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
    "name": "KmsSignerDoesNotMatchTxSender",
    "inputs": [
      {
        "name": "signerAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "txSenderAddress",
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
    "name": "NotCoprocessorSigner",
    "inputs": [
      {
        "name": "signerAddress",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "NotCoprocessorTxSender",
    "inputs": [
      {
        "name": "txSenderAddress",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "NotCustodianSigner",
    "inputs": [
      {
        "name": "signerAddress",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "NotCustodianTxSender",
    "inputs": [
      {
        "name": "txSenderAddress",
        "type": "address",
        "internalType": "address"
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
    "name": "NotKmsSigner",
    "inputs": [
      {
        "name": "signerAddress",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "NotKmsTxSender",
    "inputs": [
      {
        "name": "txSenderAddress",
        "type": "address",
        "internalType": "address"
      }
    ]
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
    "name": "PublicDecryptNotAllowed",
    "inputs": [
      {
        "name": "ctHandle",
        "type": "bytes32",
        "internalType": "bytes32"
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
    ///0x60a06040523073ffffffffffffffffffffffffffffffffffffffff1660809073ffffffffffffffffffffffffffffffffffffffff1681525034801562000043575f80fd5b50620000546200005a60201b60201c565b620001c4565b5f6200006b6200015e60201b60201c565b9050805f0160089054906101000a900460ff1615620000b6576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b67ffffffffffffffff8016815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff16146200015b5767ffffffffffffffff815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d267ffffffffffffffff604051620001529190620001a9565b60405180910390a15b50565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b5f67ffffffffffffffff82169050919050565b620001a38162000185565b82525050565b5f602082019050620001be5f83018462000198565b92915050565b608051616c2d620001eb5f395f81816124d70152818161252c01526127ce0152616c2d5ff3fe608060405260043610610108575f3560e01c80635c975abb11610094578063ad3cb1cc11610063578063ad3cb1cc1461031c578063c411587414610346578063d8998f451461035c578063f1b57adb14610384578063fbb83259146103ac57610108565b80635c975abb146102845780636f8913bc146102ae5780638456cb59146102d657806384b0196e146102ec57610108565b80633f4ba83a116100db5780633f4ba83a146101b05780634014c4cd146101c65780634f1ef2861461020257806352d1902d1461021e57806358f5b8ab1461024857610108565b8063046f9eb31461010c5780630900cc69146101345780630d8e6e2c1461017057806339f738101461019a575b5f80fd5b348015610117575f80fd5b50610132600480360381019061012d91906145b0565b6103e8565b005b34801561013f575f80fd5b5061015a60048036038101906101559190614674565b610874565b6040516101679190614786565b60405180910390f35b34801561017b575f80fd5b50610184610945565b6040516101919190614830565b60405180910390f35b3480156101a5575f80fd5b506101ae6109c0565b005b3480156101bb575f80fd5b506101c4610bf8565b005b3480156101d1575f80fd5b506101ec60048036038101906101e791906148a5565b610d40565b6040516101f9919061493d565b60405180910390f35b61021c60048036038101906102179190614aa8565b610ecd565b005b348015610229575f80fd5b50610232610eec565b60405161023f9190614b1a565b60405180910390f35b348015610253575f80fd5b5061026e60048036038101906102699190614674565b610f1d565b60405161027b919061493d565b60405180910390f35b34801561028f575f80fd5b50610298610f50565b6040516102a5919061493d565b60405180910390f35b3480156102b9575f80fd5b506102d460048036038101906102cf91906145b0565b610f72565b005b3480156102e1575f80fd5b506102ea6113bd565b005b3480156102f7575f80fd5b506103006114e2565b6040516103139796959493929190614c42565b60405180910390f35b348015610327575f80fd5b506103306115eb565b60405161033d9190614830565b60405180910390f35b348015610351575f80fd5b5061035a611624565b005b348015610367575f80fd5b50610382600480360381019061037d91906148a5565b611749565b005b34801561038f575f80fd5b506103aa60048036038101906103a59190614d59565b611910565b005b3480156103b7575f80fd5b506103d260048036038101906103cd9190614e93565b611dca565b6040516103df919061493d565b60405180910390f35b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663e5275eaf336040518263ffffffff1660e01b81526004016104359190614f24565b602060405180830381865afa158015610450573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906104749190614f67565b6104b557336040517faee863230000000000000000000000000000000000000000000000000000000081526004016104ac9190614f24565b60405180910390fd5b5f6104be612039565b905060f8600260068111156104d6576104d5614f92565b5b901b881115806104e95750806008015488115b1561052b57876040517fd48af9420000000000000000000000000000000000000000000000000000000081526004016105229190614fbf565b60405180910390fd5b5f816007015f8a81526020019081526020015f206040518060400160405290815f8201805461055990615005565b80601f016020809104026020016040519081016040528092919081815260200182805461058590615005565b80156105d05780601f106105a7576101008083540402835291602001916105d0565b820191905f5260205f20905b8154815290600101906020018083116105b357829003601f168201915b505050505081526020016001820180548060200260200160405190810160405280929190818152602001828054801561062657602002820191905f5260205f20905b815481526020019060010190808311610612575b50505050508152505090505f6040518060800160405280835f01518152602001836020015181526020018a8a8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050815260200186868080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081525090505f6106ec82612060565b90506106fa8b828a8a612127565b5f846002015f8d81526020019081526020015f205f805f1b81526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508b7f7fcdfb5381917f554a717d0a5470a33f5a49ba6445f05ec43c74c0bc2cc608b2600183805490506107b39190615062565b8d8d8d8d8d8d6040516107cc97969594939291906150d1565b60405180910390a2845f015f8d81526020019081526020015f205f9054906101000a900460ff1615801561080957506108088180549050612299565b5b15610866576001855f015f8e81526020019081526020015f205f6101000a81548160ff0219169083151502179055508b7fe89752be0ecdb68b2a6eb5ef1a891039e0e92ae3c8a62274c5881e48eea1ed2560405160405180910390a25b505050505050505050505050565b60605f61087f612039565b90505f816003015f8581526020019081526020015f20549050816002015f8581526020019081526020015f205f8281526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561093757602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116108ee575b505050505092505050919050565b60606040518060400160405280600a81526020017f44656372797074696f6e000000000000000000000000000000000000000000008152506109865f61232a565b610990600261232a565b6109995f61232a565b6040516020016109ac94939291906151fd565b604051602081830303815290604052905090565b60016109ca6123f4565b67ffffffffffffffff1614610a0b576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60035f610a16612418565b9050805f0160089054906101000a900460ff1680610a5e57508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15610a95576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff021916908315150217905550610b4e6040518060400160405280600a81526020017f44656372797074696f6e000000000000000000000000000000000000000000008152506040518060400160405280600181526020017f310000000000000000000000000000000000000000000000000000000000000081525061243f565b610b56612455565b5f610b5f612039565b905060f860016006811115610b7757610b76614f92565b5b901b816006018190555060f860026006811115610b9757610b96614f92565b5b901b8160080181905550505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051610bec919061527d565b60405180910390a15050565b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610c55573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c7991906152aa565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614158015610cf4575073d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614155b15610d3657336040517fe19166ee000000000000000000000000000000000000000000000000000000008152600401610d2d9190614f24565b60405180910390fd5b610d3e612467565b565b5f805f90505b85859050811015610ebf5773c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff16630620326d878784818110610d9457610d936152d5565b5b905060200201356040518263ffffffff1660e01b8152600401610db79190614b1a565b602060405180830381865afa158015610dd2573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610df69190614f67565b1580610ea4575073de409109e0fccaae7b87de518f61d617a3fda09473ffffffffffffffffffffffffffffffffffffffff16632ddc9a6f878784818110610e4057610e3f6152d5565b5b905060200201356040518263ffffffff1660e01b8152600401610e639190614b1a565b602060405180830381865afa158015610e7e573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ea29190614f67565b155b15610eb2575f915050610ec5565b8080600101915050610d46565b50600190505b949350505050565b610ed56124d5565b610ede826125bb565b610ee882826126ae565b5050565b5f610ef56127cc565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b5f80610f27612039565b9050805f015f8481526020019081526020015f205f9054906101000a900460ff16915050919050565b5f80610f5a612853565b9050805f015f9054906101000a900460ff1691505090565b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663e5275eaf336040518263ffffffff1660e01b8152600401610fbf9190614f24565b602060405180830381865afa158015610fda573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ffe9190614f67565b61103f57336040517faee863230000000000000000000000000000000000000000000000000000000081526004016110369190614f24565b60405180910390fd5b5f611048612039565b905060f8600160068111156110605761105f614f92565b5b901b881115806110735750806006015488115b156110b557876040517fd48af9420000000000000000000000000000000000000000000000000000000081526004016110ac9190614fbf565b60405180910390fd5b5f6040518060600160405280836005015f8c81526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561111c57602002820191905f5260205f20905b815481526020019060010190808311611108575b5050505050815260200189898080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050815260200185858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081525090505f6111c28261287a565b90506111d08a828989612127565b5f836004015f8c81526020019081526020015f205f8381526020019081526020015f20905080888890918060018154018082558091505060019003905f5260205f20015f90919290919290919290919250918261122e9291906154a9565b50836002015f8c81526020019081526020015f205f8381526020019081526020015f2033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508a7f4d7b1dba49e9e846215e1621f5737c81d8614c4f268494d8b787632c4e59f0e58b8b8b8b338c8c6040516112eb9796959493929190615576565b60405180910390a2835f015f8c81526020019081526020015f205f9054906101000a900460ff1615801561132857506113278180549050612934565b5b156113b0576001845f015f8d81526020019081526020015f205f6101000a81548160ff02191690831515021790555081846003015f8d81526020019081526020015f20819055508a7fd7e58a367a0a6c298e76ad5d240004e327aa1423cbe4bd7ff85d4c715ef8d15f8b8b848a8a6040516113a7959493929190615722565b60405180910390a25b5050505050505050505050565b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff166346fbf68e336040518263ffffffff1660e01b815260040161140a9190614f24565b602060405180830381865afa158015611425573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114499190614f67565b158015611496575073d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614155b156114d857336040517f388916bb0000000000000000000000000000000000000000000000000000000081526004016114cf9190614f24565b60405180910390fd5b6114e06129c5565b565b5f6060805f805f60605f6114f4612a34565b90505f801b815f015414801561150f57505f801b8160010154145b61154e576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611545906157ba565b60405180910390fd5b611556612a5b565b61155e612af9565b46305f801b5f67ffffffffffffffff81111561157d5761157c614984565b5b6040519080825280602002602001820160405280156115ab5781602001602082028036833780820191505090505b507f0f0000000000000000000000000000000000000000000000000000000000000095949392919097509750975097509750975097505090919293949596565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b60035f61162f612418565b9050805f0160089054906101000a900460ff168061167757508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b156116ae576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d28260405161173d919061527d565b60405180910390a15050565b611751612b97565b5f848490500361178d576040517f2de7543800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6117d68484808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f82011690508083019250505050505050612bd8565b5f73de409109e0fccaae7b87de518f61d617a3fda09473ffffffffffffffffffffffffffffffffffffffff1663a14f897186866040518363ffffffff1660e01b8152600401611826929190615850565b5f60405180830381865afa158015611840573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f820116820180604052508101906118689190615ae7565b905061187381612c90565b5f61187c612039565b9050806006015f81548092919061189290615b2e565b91905055505f816006015490508686836005015f8481526020019081526020015f2091906118c192919061445a565b506118cb33612d76565b807f22db480a39bd72556438aadb4a32a3d2a6638b87c03bbec5fef6997e109587ff8487876040516118ff93929190615d0b565b60405180910390a250505050505050565b611918612b97565b5f8880602001906119299190615d4e565b905003611962576040517f57cfa21700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600a60ff168880602001906119779190615d4e565b905011156119d057600a8880602001906119919190615d4e565b90506040517faf1f04950000000000000000000000000000000000000000000000000000000081526004016119c7929190615dec565b60405180910390fd5b6119e9898036038101906119e49190615e60565b612df3565b611a418880602001906119fc9190615d4e565b808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f8201169050808301925050505050505088612f3e565b15611a955786888060200190611a579190615d4e565b6040517fdc4d78b1000000000000000000000000000000000000000000000000000000008152600401611a8c93929190615f11565b60405180910390fd5b5f611af08c8c8b8060200190611aab9190615d4e565b808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f820116905080830192505050505050508b612fbc565b90505f6040518060a0016040528089898080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081526020018b8060200190611b579190615d4e565b808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f8201169050808301925050505050505081526020018c5f013581526020018c60200135815260200185858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050508152509050611c07818a88888e5f01356131af565b5f73de409109e0fccaae7b87de518f61d617a3fda09473ffffffffffffffffffffffffffffffffffffffff1663a14f8971846040518263ffffffff1660e01b8152600401611c559190615fd9565b5f60405180830381865afa158015611c6f573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f82011682018060405250810190611c979190615ae7565b9050611ca281612c90565b5f611cab612039565b9050806008015f815480929190611cc190615b2e565b91905055505f8160080154905060405180604001604052808c8c8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050815260200186815250826007015f8381526020019081526020015f205f820151815f019081611d4c9190616003565b506020820151816001019080519060200190611d699291906144a5565b50905050611d7633613287565b807ff9011bd6ba0da6049c520d70fe5971f17ed7ab795486052544b51019896c596b848e8e8e8c8c604051611db0969594939291906160d2565b60405180910390a250505050505050505050505050505050565b5f805f90505b8585905081101561202a5773c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff1663c6528f69878784818110611e1e57611e1d6152d5565b5b9050604002015f0135896040518363ffffffff1660e01b8152600401611e4592919061612e565b602060405180830381865afa158015611e60573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e849190614f67565b1580611f60575073c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff1663c6528f69878784818110611ece57611ecd6152d5565b5b9050604002015f0135888885818110611eea57611ee96152d5565b5b9050604002016020016020810190611f029190616155565b6040518363ffffffff1660e01b8152600401611f1f92919061612e565b602060405180830381865afa158015611f3a573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f5e9190614f67565b155b8061200f575073de409109e0fccaae7b87de518f61d617a3fda09473ffffffffffffffffffffffffffffffffffffffff16632ddc9a6f878784818110611fa957611fa86152d5565b5b9050604002015f01356040518263ffffffff1660e01b8152600401611fce9190614b1a565b602060405180830381865afa158015611fe9573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061200d9190614f67565b155b1561201d575f915050612030565b8080600101915050611dd0565b50600190505b95945050505050565b5f7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee700905090565b5f6121206040518060a00160405280606d8152602001616ae5606d913980519060200120835f01518051906020012084602001516040516020016120a4919061620c565b6040516020818303038152906040528051906020012085604001518051906020012086606001516040516020016120db919061625c565b60405160208183030381529060405280519060200120604051602001612105959493929190616272565b60405160208183030381529060405280519060200120613304565b9050919050565b5f612130612039565b90505f6121808585858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505061331d565b905061218c8133613347565b816001015f8781526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff161561222b5785816040517f99ec48d90000000000000000000000000000000000000000000000000000000081526004016122229291906162c3565b60405180910390fd5b6001826001015f8881526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550505050505050565b5f8073d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663c2b429866040518163ffffffff1660e01b8152600401602060405180830381865afa1580156122f8573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061231c91906162ea565b905080831015915050919050565b60605f600161233884613458565b0190505f8167ffffffffffffffff81111561235657612355614984565b5b6040519080825280601f01601f1916602001820160405280156123885781602001600182028036833780820191505090505b5090505f82602001820190505b6001156123e9578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a85816123de576123dd616315565b5b0494505f8503612395575b819350505050919050565b5f6123fd612418565b5f015f9054906101000a900467ffffffffffffffff16905090565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b6124476135a9565b61245182826135e9565b5050565b61245d6135a9565b61246561363a565b565b61246f61366a565b5f612478612853565b90505f815f015f6101000a81548160ff0219169083151502179055507f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa6124bd6136aa565b6040516124ca9190614f24565b60405180910390a150565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff16148061258257507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166125696136b1565b73ffffffffffffffffffffffffffffffffffffffff1614155b156125b9576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612618573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061263c91906152aa565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146126ab57336040517f0e56cf3d0000000000000000000000000000000000000000000000000000000081526004016126a29190614f24565b60405180910390fd5b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa92505050801561271657506040513d601f19601f820116820180604052508101906127139190616342565b60015b61275757816040517f4c9c8ce300000000000000000000000000000000000000000000000000000000815260040161274e9190614f24565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b81146127bd57806040517faa1d49a40000000000000000000000000000000000000000000000000000000081526004016127b49190614b1a565b60405180910390fd5b6127c78383613704565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff1614612851576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300905090565b5f61292d604051806080016040528060548152602001616b526054913980519060200120835f01516040516020016128b2919061620c565b6040516020818303038152906040528051906020012084602001518051906020012085604001516040516020016128e9919061625c565b60405160208183030381529060405280519060200120604051602001612912949392919061636d565b60405160208183030381529060405280519060200120613304565b9050919050565b5f8073d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff16632a3889986040518163ffffffff1660e01b8152600401602060405180830381865afa158015612993573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129b791906162ea565b905080831015915050919050565b6129cd612b97565b5f6129d6612853565b90506001815f015f6101000a81548160ff0219169083151502179055507f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a258612a1c6136aa565b604051612a299190614f24565b60405180910390a150565b5f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100905090565b60605f612a66612a34565b9050806002018054612a7790615005565b80601f0160208091040260200160405190810160405280929190818152602001828054612aa390615005565b8015612aee5780601f10612ac557610100808354040283529160200191612aee565b820191905f5260205f20905b815481529060010190602001808311612ad157829003601f168201915b505050505091505090565b60605f612b04612a34565b9050806003018054612b1590615005565b80601f0160208091040260200160405190810160405280929190818152602001828054612b4190615005565b8015612b8c5780601f10612b6357610100808354040283529160200191612b8c565b820191905f5260205f20905b815481529060010190602001808311612b6f57829003601f168201915b505050505091505090565b612b9f610f50565b15612bd6576040517fd93c066500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f805b8251811015612c40575f838281518110612bf857612bf76152d5565b5b602002602001015190505f612c0c82613776565b9050612c1781613800565b61ffff1684612c2691906163b0565b9350612c31826139eb565b50508080600101915050612bdb565b50610800811115612c8c57610800816040517fe7f4895d000000000000000000000000000000000000000000000000000000008152600401612c839291906163e3565b60405180910390fd5b5050565b600181511115612d73575f815f81518110612cae57612cad6152d5565b5b60200260200101516020015190505f600190505b8251811015612d705781838281518110612cdf57612cde6152d5565b5b60200260200101516020015114612d6357825f81518110612d0357612d026152d5565b5b6020026020010151838281518110612d1e57612d1d6152d5565b5b60200260200101516040517fcfae921f000000000000000000000000000000000000000000000000000000008152600401612d5a92919061646a565b60405180910390fd5b8080600101915050612cc2565b50505b50565b738733d4013efc4256977150f31a8ea1e9e4c1458873ffffffffffffffffffffffffffffffffffffffff166391eeb27c826040518263ffffffff1660e01b8152600401612dc39190614f24565b5f604051808303815f87803b158015612dda575f80fd5b505af1158015612dec573d5f803e3d5ffd5b5050505050565b5f816020015103612e30576040517fde2859c100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61016d61ffff1681602001511115612e875761016d81602001516040517f32951863000000000000000000000000000000000000000000000000000000008152600401612e7e9291906164dc565b60405180910390fd5b42815f01511115612ed45742815f01516040517ff24c0887000000000000000000000000000000000000000000000000000000008152600401612ecb9291906163e3565b60405180910390fd5b42620151808260200151612ee89190616503565b825f0151612ef691906163b0565b1015612f3b5742816040517f30348040000000000000000000000000000000000000000000000000000000008152600401612f32929190616571565b60405180910390fd5b50565b5f805f90505b8351811015612fb1578273ffffffffffffffffffffffffffffffffffffffff16848281518110612f7757612f766152d5565b5b602002602001015173ffffffffffffffffffffffffffffffffffffffff1603612fa4576001915050612fb6565b8080600101915050612f44565b505f90505b92915050565b60605f8585905003612ffa576040517fa6a6cb2100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8484905067ffffffffffffffff81111561301757613016614984565b5b6040519080825280602002602001820160405280156130455781602001602082028036833780820191505090505b5090505f805b8686905081101561315a575f87878381811061306a576130696152d5565b5b9050604002015f013590505f888884818110613089576130886152d5565b5b90506040020160200160208101906130a19190616155565b90505f6130ad83613776565b90506130b881613800565b61ffff16856130c791906163b0565b94506130d38388613abb565b6130dd8383613abb565b6130e78883612f3e565b61312a5781886040517fa4c30391000000000000000000000000000000000000000000000000000000008152600401613121929190616598565b60405180910390fd5b8286858151811061313e5761313d6152d5565b5b602002602001018181525050505050808060010191505061304b565b506108008111156131a657610800816040517fe7f4895d00000000000000000000000000000000000000000000000000000000815260040161319d9291906163e3565b60405180910390fd5b50949350505050565b5f6131ba8683613b90565b90505f61320a8286868080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505061331d565b90508573ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff161461327e5784846040517f2a873d270000000000000000000000000000000000000000000000000000000081526004016132759291906165c6565b60405180910390fd5b50505050505050565b738733d4013efc4256977150f31a8ea1e9e4c1458873ffffffffffffffffffffffffffffffffffffffff1663988a2d2d826040518263ffffffff1660e01b81526004016132d49190614f24565b5f604051808303815f87803b1580156132eb575f80fd5b505af11580156132fd573d5f803e3d5ffd5b5050505050565b5f613316613310613c5d565b83613c6b565b9050919050565b5f805f8061332b8686613cab565b92509250925061333b8282613d00565b82935050505092915050565b61335082613e62565b8173ffffffffffffffffffffffffffffffffffffffff1673d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663e3b2a874836040518263ffffffff1660e01b81526004016133b49190614f24565b5f60405180830381865afa1580156133ce573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f820116820180604052508101906133f69190616733565b6020015173ffffffffffffffffffffffffffffffffffffffff16146134545781816040517f0d86f52100000000000000000000000000000000000000000000000000000000815260040161344b92919061677a565b60405180910390fd5b5050565b5f805f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f01000000000000000083106134b4577a184f03e93ff9f4daa797ed6e38ed64bf6a1f01000000000000000083816134aa576134a9616315565b5b0492506040810190505b6d04ee2d6d415b85acef810000000083106134f1576d04ee2d6d415b85acef810000000083816134e7576134e6616315565b5b0492506020810190505b662386f26fc10000831061352057662386f26fc10000838161351657613515616315565b5b0492506010810190505b6305f5e1008310613549576305f5e100838161353f5761353e616315565b5b0492506008810190505b612710831061356e57612710838161356457613563616315565b5b0492506004810190505b60648310613591576064838161358757613586616315565b5b0492506002810190505b600a83106135a0576001810190505b80915050919050565b6135b1613f32565b6135e7576040517fd7e6bcf800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b6135f16135a9565b5f6135fa612a34565b90508281600201908161360d91906167f9565b508181600301908161361f91906167f9565b505f801b815f01819055505f801b8160010181905550505050565b6136426135a9565b5f61364b612853565b90505f815f015f6101000a81548160ff02191690831515021790555050565b613672610f50565b6136a8576040517f8dfc202b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f33905090565b5f6136dd7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b613f50565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b61370d82613f59565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f81511115613769576137638282614022565b50613772565b6137716140a2565b5b5050565b5f8060f860f084901b901c5f1c905060538081111561379857613797614f92565b5b60ff168160ff1611156137e257806040517f641950d70000000000000000000000000000000000000000000000000000000081526004016137d991906168d7565b60405180910390fd5b8060ff1660538111156137f8576137f7614f92565b5b915050919050565b5f80605381111561381457613813614f92565b5b82605381111561382757613826614f92565b5b0361383557600290506139e6565b6002605381111561384957613848614f92565b5b82605381111561385c5761385b614f92565b5b0361386a57600890506139e6565b6003605381111561387e5761387d614f92565b5b82605381111561389157613890614f92565b5b0361389f57601090506139e6565b600460538111156138b3576138b2614f92565b5b8260538111156138c6576138c5614f92565b5b036138d457602090506139e6565b600560538111156138e8576138e7614f92565b5b8260538111156138fb576138fa614f92565b5b0361390957604090506139e6565b6006605381111561391d5761391c614f92565b5b8260538111156139305761392f614f92565b5b0361393e57608090506139e6565b6007605381111561395257613951614f92565b5b82605381111561396557613964614f92565b5b036139735760a090506139e6565b6008605381111561398757613986614f92565b5b82605381111561399a57613999614f92565b5b036139a95761010090506139e6565b816040517fbe7830b10000000000000000000000000000000000000000000000000000000081526004016139dd9190616936565b60405180910390fd5b919050565b73c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff16630620326d826040518263ffffffff1660e01b8152600401613a389190614b1a565b602060405180830381865afa158015613a53573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613a779190614f67565b613ab857806040517f4331a85d000000000000000000000000000000000000000000000000000000008152600401613aaf9190614b1a565b60405180910390fd5b50565b73c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff1663c6528f6983836040518363ffffffff1660e01b8152600401613b0a92919061612e565b602060405180830381865afa158015613b25573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613b499190614f67565b613b8c5781816040517f160a2b4b000000000000000000000000000000000000000000000000000000008152600401613b8392919061612e565b60405180910390fd5b5050565b5f806040518060c0016040528060878152602001616ba66087913980519060200120845f0151805190602001208560200151604051602001613bd291906169db565b60405160208183030381529060405280519060200120866040015187606001518860800151604051602001613c07919061625c565b60405160208183030381529060405280519060200120604051602001613c32969594939291906169f1565b604051602081830303815290604052805190602001209050613c5483826140de565b91505092915050565b5f613c66614152565b905090565b5f6040517f190100000000000000000000000000000000000000000000000000000000000081528360028201528260228201526042812091505092915050565b5f805f6041845103613ceb575f805f602087015192506040870151915060608701515f1a9050613cdd888285856141b5565b955095509550505050613cf9565b5f600285515f1b9250925092505b9250925092565b5f6003811115613d1357613d12614f92565b5b826003811115613d2657613d25614f92565b5b0315613e5e5760016003811115613d4057613d3f614f92565b5b826003811115613d5357613d52614f92565b5b03613d8a576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60026003811115613d9e57613d9d614f92565b5b826003811115613db157613db0614f92565b5b03613df557805f1c6040517ffce698f7000000000000000000000000000000000000000000000000000000008152600401613dec9190614fbf565b60405180910390fd5b600380811115613e0857613e07614f92565b5b826003811115613e1b57613e1a614f92565b5b03613e5d57806040517fd78bce0c000000000000000000000000000000000000000000000000000000008152600401613e549190614b1a565b60405180910390fd5b5b5050565b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663203d0114826040518263ffffffff1660e01b8152600401613eaf9190614f24565b602060405180830381865afa158015613eca573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613eee9190614f67565b613f2f57806040517f2a7c6ef6000000000000000000000000000000000000000000000000000000008152600401613f269190614f24565b60405180910390fd5b50565b5f613f3b612418565b5f0160089054906101000a900460ff16905090565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b03613fb457806040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401613fab9190614f24565b60405180910390fd5b80613fe07f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b613f50565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f808473ffffffffffffffffffffffffffffffffffffffff168460405161404b919061625c565b5f60405180830381855af49150503d805f8114614083576040519150601f19603f3d011682016040523d82523d5f602084013e614088565b606091505b509150915061409885838361429c565b9250505092915050565b5f3411156140dc576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f807f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f614109614329565b61411161439f565b8630604051602001614127959493929190616a50565b6040516020818303038152906040528051906020012090506141498184613c6b565b91505092915050565b5f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f61417c614329565b61418461439f565b463060405160200161419a959493929190616a50565b60405160208183030381529060405280519060200120905090565b5f805f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c11156141f1575f600385925092509250614292565b5f6001888888886040515f81526020016040526040516142149493929190616aa1565b6020604051602081039080840390855afa158015614234573d5f803e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603614285575f60015f801b93509350935050614292565b805f805f1b935093509350505b9450945094915050565b6060826142b1576142ac82614416565b614321565b5f82511480156142d757505f8473ffffffffffffffffffffffffffffffffffffffff163b145b1561431957836040517f9996b3150000000000000000000000000000000000000000000000000000000081526004016143109190614f24565b60405180910390fd5b819050614322565b5b9392505050565b5f80614333612a34565b90505f61433e612a5b565b90505f8151111561435a5780805190602001209250505061439c565b5f825f015490505f801b81146143755780935050505061439c565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f806143a9612a34565b90505f6143b4612af9565b90505f815111156143d057808051906020012092505050614413565b5f826001015490505f801b81146143ec57809350505050614413565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f815111156144285780518082602001fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b828054828255905f5260205f20908101928215614494579160200282015b82811115614493578235825591602001919060010190614478565b5b5090506144a191906144f0565b5090565b828054828255905f5260205f209081019282156144df579160200282015b828111156144de5782518255916020019190600101906144c3565b5b5090506144ec91906144f0565b5090565b5b80821115614507575f815f9055506001016144f1565b5090565b5f604051905090565b5f80fd5b5f80fd5b5f819050919050565b61452e8161451c565b8114614538575f80fd5b50565b5f8135905061454981614525565b92915050565b5f80fd5b5f80fd5b5f80fd5b5f8083601f8401126145705761456f61454f565b5b8235905067ffffffffffffffff81111561458d5761458c614553565b5b6020830191508360018202830111156145a9576145a8614557565b5b9250929050565b5f805f805f805f6080888a0312156145cb576145ca614514565b5b5f6145d88a828b0161453b565b975050602088013567ffffffffffffffff8111156145f9576145f8614518565b5b6146058a828b0161455b565b9650965050604088013567ffffffffffffffff81111561462857614627614518565b5b6146348a828b0161455b565b9450945050606088013567ffffffffffffffff81111561465757614656614518565b5b6146638a828b0161455b565b925092505092959891949750929550565b5f6020828403121561468957614688614514565b5b5f6146968482850161453b565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6146f1826146c8565b9050919050565b614701816146e7565b82525050565b5f61471283836146f8565b60208301905092915050565b5f602082019050919050565b5f6147348261469f565b61473e81856146a9565b9350614749836146b9565b805f5b838110156147795781516147608882614707565b975061476b8361471e565b92505060018101905061474c565b5085935050505092915050565b5f6020820190508181035f83015261479e818461472a565b905092915050565b5f81519050919050565b5f82825260208201905092915050565b5f5b838110156147dd5780820151818401526020810190506147c2565b5f8484015250505050565b5f601f19601f8301169050919050565b5f614802826147a6565b61480c81856147b0565b935061481c8185602086016147c0565b614825816147e8565b840191505092915050565b5f6020820190508181035f83015261484881846147f8565b905092915050565b5f8083601f8401126148655761486461454f565b5b8235905067ffffffffffffffff81111561488257614881614553565b5b60208301915083602082028301111561489e5761489d614557565b5b9250929050565b5f805f80604085870312156148bd576148bc614514565b5b5f85013567ffffffffffffffff8111156148da576148d9614518565b5b6148e687828801614850565b9450945050602085013567ffffffffffffffff81111561490957614908614518565b5b6149158782880161455b565b925092505092959194509250565b5f8115159050919050565b61493781614923565b82525050565b5f6020820190506149505f83018461492e565b92915050565b61495f816146e7565b8114614969575f80fd5b50565b5f8135905061497a81614956565b92915050565b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6149ba826147e8565b810181811067ffffffffffffffff821117156149d9576149d8614984565b5b80604052505050565b5f6149eb61450b565b90506149f782826149b1565b919050565b5f67ffffffffffffffff821115614a1657614a15614984565b5b614a1f826147e8565b9050602081019050919050565b828183375f83830152505050565b5f614a4c614a47846149fc565b6149e2565b905082815260208101848484011115614a6857614a67614980565b5b614a73848285614a2c565b509392505050565b5f82601f830112614a8f57614a8e61454f565b5b8135614a9f848260208601614a3a565b91505092915050565b5f8060408385031215614abe57614abd614514565b5b5f614acb8582860161496c565b925050602083013567ffffffffffffffff811115614aec57614aeb614518565b5b614af885828601614a7b565b9150509250929050565b5f819050919050565b614b1481614b02565b82525050565b5f602082019050614b2d5f830184614b0b565b92915050565b5f7fff0000000000000000000000000000000000000000000000000000000000000082169050919050565b614b6781614b33565b82525050565b614b768161451c565b82525050565b614b85816146e7565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b614bbd8161451c565b82525050565b5f614bce8383614bb4565b60208301905092915050565b5f602082019050919050565b5f614bf082614b8b565b614bfa8185614b95565b9350614c0583614ba5565b805f5b83811015614c35578151614c1c8882614bc3565b9750614c2783614bda565b925050600181019050614c08565b5085935050505092915050565b5f60e082019050614c555f83018a614b5e565b8181036020830152614c6781896147f8565b90508181036040830152614c7b81886147f8565b9050614c8a6060830187614b6d565b614c976080830186614b7c565b614ca460a0830185614b0b565b81810360c0830152614cb68184614be6565b905098975050505050505050565b5f8083601f840112614cd957614cd861454f565b5b8235905067ffffffffffffffff811115614cf657614cf5614553565b5b602083019150836040820283011115614d1257614d11614557565b5b9250929050565b5f80fd5b5f60408284031215614d3257614d31614d19565b5b81905092915050565b5f60408284031215614d5057614d4f614d19565b5b81905092915050565b5f805f805f805f805f805f6101008c8e031215614d7957614d78614514565b5b5f8c013567ffffffffffffffff811115614d9657614d95614518565b5b614da28e828f01614cc4565b9b509b50506020614db58e828f01614d1d565b99505060608c013567ffffffffffffffff811115614dd657614dd5614518565b5b614de28e828f01614d3b565b9850506080614df38e828f0161496c565b97505060a08c013567ffffffffffffffff811115614e1457614e13614518565b5b614e208e828f0161455b565b965096505060c08c013567ffffffffffffffff811115614e4357614e42614518565b5b614e4f8e828f0161455b565b945094505060e08c013567ffffffffffffffff811115614e7257614e71614518565b5b614e7e8e828f0161455b565b92509250509295989b509295989b9093969950565b5f805f805f60608688031215614eac57614eab614514565b5b5f614eb98882890161496c565b955050602086013567ffffffffffffffff811115614eda57614ed9614518565b5b614ee688828901614cc4565b9450945050604086013567ffffffffffffffff811115614f0957614f08614518565b5b614f158882890161455b565b92509250509295509295909350565b5f602082019050614f375f830184614b7c565b92915050565b614f4681614923565b8114614f50575f80fd5b50565b5f81519050614f6181614f3d565b92915050565b5f60208284031215614f7c57614f7b614514565b5b5f614f8984828501614f53565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b5f602082019050614fd25f830184614b6d565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f600282049050600182168061501c57607f821691505b60208210810361502f5761502e614fd8565b5b50919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61506c8261451c565b91506150778361451c565b925082820390508181111561508f5761508e615035565b5b92915050565b5f82825260208201905092915050565b5f6150b08385615095565b93506150bd838584614a2c565b6150c6836147e8565b840190509392505050565b5f6080820190506150e45f83018a614b6d565b81810360208301526150f781888a6150a5565b9050818103604083015261510c8186886150a5565b905081810360608301526151218184866150a5565b905098975050505050505050565b5f81905092915050565b5f615143826147a6565b61514d818561512f565b935061515d8185602086016147c0565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f61519d60028361512f565b91506151a882615169565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f6151e760018361512f565b91506151f2826151b3565b600182019050919050565b5f6152088287615139565b915061521382615191565b915061521f8286615139565b915061522a826151db565b91506152368285615139565b9150615241826151db565b915061524d8284615139565b915081905095945050505050565b5f67ffffffffffffffff82169050919050565b6152778161525b565b82525050565b5f6020820190506152905f83018461526e565b92915050565b5f815190506152a481614956565b92915050565b5f602082840312156152bf576152be614514565b5b5f6152cc84828501615296565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f82905092915050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f600883026153687fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8261532d565b615372868361532d565b95508019841693508086168417925050509392505050565b5f819050919050565b5f6153ad6153a86153a38461451c565b61538a565b61451c565b9050919050565b5f819050919050565b6153c683615393565b6153da6153d2826153b4565b848454615339565b825550505050565b5f90565b6153ee6153e2565b6153f98184846153bd565b505050565b5b8181101561541c576154115f826153e6565b6001810190506153ff565b5050565b601f821115615461576154328161530c565b61543b8461531e565b8101602085101561544a578190505b61545e6154568561531e565b8301826153fe565b50505b505050565b5f82821c905092915050565b5f6154815f1984600802615466565b1980831691505092915050565b5f6154998383615472565b9150826002028217905092915050565b6154b38383615302565b67ffffffffffffffff8111156154cc576154cb614984565b5b6154d68254615005565b6154e1828285615420565b5f601f83116001811461550e575f84156154fc578287013590505b615506858261548e565b86555061556d565b601f19841661551c8661530c565b5f5b828110156155435784890135825560018201915060208501945060208101905061551e565b86831015615560578489013561555c601f891682615472565b8355505b6001600288020188555050505b50505050505050565b5f6080820190508181035f83015261558f81898b6150a5565b905081810360208301526155a48187896150a5565b90506155b36040830186614b7c565b81810360608301526155c68184866150a5565b905098975050505050505050565b5f81549050919050565b5f82825260208201905092915050565b5f819050815f5260205f209050919050565b5f82825260208201905092915050565b5f815461561c81615005565b6156268186615600565b9450600182165f8114615640576001811461565657615688565b60ff198316865281151560200286019350615688565b61565f8561530c565b5f5b8381101561568057815481890152600182019150602081019050615661565b808801955050505b50505092915050565b5f61569c8383615610565b905092915050565b5f600182019050919050565b5f6156ba826155d4565b6156c481856155de565b9350836020820285016156d6856155ee565b805f5b85811015615710578484038952816156f18582615691565b94506156fc836156a4565b925060208a019950506001810190506156d9565b50829750879550505050505092915050565b5f6060820190508181035f83015261573b8187896150a5565b9050818103602083015261574f81866156b0565b905081810360408301526157648184866150a5565b90509695505050505050565b7f4549503731323a20556e696e697469616c697a656400000000000000000000005f82015250565b5f6157a46015836147b0565b91506157af82615770565b602082019050919050565b5f6020820190508181035f8301526157d181615798565b9050919050565b5f82825260208201905092915050565b5f80fd5b82818337505050565b5f61580083856157d8565b93507f07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff831115615833576158326157e8565b5b6020830292506158448385846157ec565b82840190509392505050565b5f6020820190508181035f8301526158698184866157f5565b90509392505050565b5f67ffffffffffffffff82111561588c5761588b614984565b5b602082029050602081019050919050565b5f80fd5b5f80fd5b6158ae81614b02565b81146158b8575f80fd5b50565b5f815190506158c9816158a5565b92915050565b5f815190506158dd81614525565b92915050565b5f67ffffffffffffffff8211156158fd576158fc614984565b5b602082029050602081019050919050565b5f61592061591b846158e3565b6149e2565b9050808382526020820190506020840283018581111561594357615942614557565b5b835b8181101561596c57806159588882615296565b845260208401935050602081019050615945565b5050509392505050565b5f82601f83011261598a5761598961454f565b5b815161599a84826020860161590e565b91505092915050565b5f608082840312156159b8576159b761589d565b5b6159c260806149e2565b90505f6159d1848285016158bb565b5f8301525060206159e4848285016158cf565b60208301525060406159f8848285016158bb565b604083015250606082015167ffffffffffffffff811115615a1c57615a1b6158a1565b5b615a2884828501615976565b60608301525092915050565b5f615a46615a4184615872565b6149e2565b90508083825260208201905060208402830185811115615a6957615a68614557565b5b835b81811015615ab057805167ffffffffffffffff811115615a8e57615a8d61454f565b5b808601615a9b89826159a3565b85526020850194505050602081019050615a6b565b5050509392505050565b5f82601f830112615ace57615acd61454f565b5b8151615ade848260208601615a34565b91505092915050565b5f60208284031215615afc57615afb614514565b5b5f82015167ffffffffffffffff811115615b1957615b18614518565b5b615b2584828501615aba565b91505092915050565b5f615b388261451c565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203615b6a57615b69615035565b5b600182019050919050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b615ba781614b02565b82525050565b5f82825260208201905092915050565b5f615bc78261469f565b615bd18185615bad565b9350615bdc836146b9565b805f5b83811015615c0c578151615bf38882614707565b9750615bfe8361471e565b925050600181019050615bdf565b5085935050505092915050565b5f608083015f830151615c2e5f860182615b9e565b506020830151615c416020860182614bb4565b506040830151615c546040860182615b9e565b5060608301518482036060860152615c6c8282615bbd565b9150508091505092915050565b5f615c848383615c19565b905092915050565b5f602082019050919050565b5f615ca282615b75565b615cac8185615b7f565b935083602082028501615cbe85615b8f565b805f5b85811015615cf95784840389528151615cda8582615c79565b9450615ce583615c8c565b925060208a01995050600181019050615cc1565b50829750879550505050505092915050565b5f6040820190508181035f830152615d238186615c98565b90508181036020830152615d388184866150a5565b9050949350505050565b5f80fd5b5f80fd5b5f80fd5b5f8083356001602003843603038112615d6a57615d69615d42565b5b80840192508235915067ffffffffffffffff821115615d8c57615d8b615d46565b5b602083019250602082023603831315615da857615da7615d4a565b5b509250929050565b5f60ff82169050919050565b5f615dd6615dd1615dcc84615db0565b61538a565b61451c565b9050919050565b615de681615dbc565b82525050565b5f604082019050615dff5f830185615ddd565b615e0c6020830184614b6d565b9392505050565b5f60408284031215615e2857615e2761589d565b5b615e3260406149e2565b90505f615e418482850161453b565b5f830152506020615e548482850161453b565b60208301525092915050565b5f60408284031215615e7557615e74614514565b5b5f615e8284828501615e13565b91505092915050565b5f819050919050565b5f615ea2602084018461496c565b905092915050565b5f602082019050919050565b5f615ec183856146a9565b9350615ecc82615e8b565b805f5b85811015615f0457615ee18284615e94565b615eeb8882614707565b9750615ef683615eaa565b925050600181019050615ecf565b5085925050509392505050565b5f604082019050615f245f830186614b7c565b8181036020830152615f37818486615eb6565b9050949350505050565b5f81519050919050565b5f819050602082019050919050565b5f615f658383615b9e565b60208301905092915050565b5f602082019050919050565b5f615f8782615f41565b615f9181856157d8565b9350615f9c83615f4b565b805f5b83811015615fcc578151615fb38882615f5a565b9750615fbe83615f71565b925050600181019050615f9f565b5085935050505092915050565b5f6020820190508181035f830152615ff18184615f7d565b905092915050565b5f81519050919050565b61600c82615ff9565b67ffffffffffffffff81111561602557616024614984565b5b61602f8254615005565b61603a828285615420565b5f60209050601f83116001811461606b575f8415616059578287015190505b616063858261548e565b8655506160ca565b601f1984166160798661530c565b5f5b828110156160a05784890151825560018201915060208501945060208101905061607b565b868310156160bd57848901516160b9601f891682615472565b8355505b6001600288020188555050505b505050505050565b5f6080820190508181035f8301526160ea8189615c98565b90506160f96020830188614b7c565b818103604083015261610c8186886150a5565b905081810360608301526161218184866150a5565b9050979650505050505050565b5f6040820190506161415f830185614b0b565b61614e6020830184614b7c565b9392505050565b5f6020828403121561616a57616169614514565b5b5f6161778482850161496c565b91505092915050565b5f81905092915050565b61619381614b02565b82525050565b5f6161a4838361618a565b60208301905092915050565b5f6161ba82615f41565b6161c48185616180565b93506161cf83615f4b565b805f5b838110156161ff5781516161e68882616199565b97506161f183615f71565b9250506001810190506161d2565b5085935050505092915050565b5f61621782846161b0565b915081905092915050565b5f81905092915050565b5f61623682615ff9565b6162408185616222565b93506162508185602086016147c0565b80840191505092915050565b5f616267828461622c565b915081905092915050565b5f60a0820190506162855f830188614b0b565b6162926020830187614b0b565b61629f6040830186614b0b565b6162ac6060830185614b0b565b6162b96080830184614b0b565b9695505050505050565b5f6040820190506162d65f830185614b6d565b6162e36020830184614b7c565b9392505050565b5f602082840312156162ff576162fe614514565b5b5f61630c848285016158cf565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f6020828403121561635757616356614514565b5b5f616364848285016158bb565b91505092915050565b5f6080820190506163805f830187614b0b565b61638d6020830186614b0b565b61639a6040830185614b0b565b6163a76060830184614b0b565b95945050505050565b5f6163ba8261451c565b91506163c58361451c565b92508282019050808211156163dd576163dc615035565b5b92915050565b5f6040820190506163f65f830185614b6d565b6164036020830184614b6d565b9392505050565b5f608083015f83015161641f5f860182615b9e565b5060208301516164326020860182614bb4565b5060408301516164456040860182615b9e565b506060830151848203606086015261645d8282615bbd565b9150508091505092915050565b5f6040820190508181035f830152616482818561640a565b90508181036020830152616496818461640a565b90509392505050565b5f61ffff82169050919050565b5f6164c66164c16164bc8461649f565b61538a565b61451c565b9050919050565b6164d6816164ac565b82525050565b5f6040820190506164ef5f8301856164cd565b6164fc6020830184614b6d565b9392505050565b5f61650d8261451c565b91506165188361451c565b92508282026165268161451c565b9150828204841483151761653d5761653c615035565b5b5092915050565b604082015f8201516165585f850182614bb4565b50602082015161656b6020850182614bb4565b50505050565b5f6060820190506165845f830185614b6d565b6165916020830184616544565b9392505050565b5f6040820190506165ab5f830185614b7c565b81810360208301526165bd818461472a565b90509392505050565b5f6020820190508181035f8301526165df8184866150a5565b90509392505050565b5f67ffffffffffffffff82111561660257616601614984565b5b61660b826147e8565b9050602081019050919050565b5f61662a616625846165e8565b6149e2565b90508281526020810184848401111561664657616645614980565b5b6166518482856147c0565b509392505050565b5f82601f83011261666d5761666c61454f565b5b815161667d848260208601616618565b91505092915050565b5f6080828403121561669b5761669a61589d565b5b6166a560806149e2565b90505f6166b484828501615296565b5f8301525060206166c784828501615296565b602083015250604082015167ffffffffffffffff8111156166eb576166ea6158a1565b5b6166f784828501616659565b604083015250606082015167ffffffffffffffff81111561671b5761671a6158a1565b5b61672784828501616659565b60608301525092915050565b5f6020828403121561674857616747614514565b5b5f82015167ffffffffffffffff81111561676557616764614518565b5b61677184828501616686565b91505092915050565b5f60408201905061678d5f830185614b7c565b61679a6020830184614b7c565b9392505050565b5f819050815f5260205f209050919050565b601f8211156167f4576167c5816167a1565b6167ce8461531e565b810160208510156167dd578190505b6167f16167e98561531e565b8301826153fe565b50505b505050565b616802826147a6565b67ffffffffffffffff81111561681b5761681a614984565b5b6168258254615005565b6168308282856167b3565b5f60209050601f831160018114616861575f841561684f578287015190505b616859858261548e565b8655506168c0565b601f19841661686f866167a1565b5f5b8281101561689657848901518255600182019150602085019450602081019050616871565b868310156168b357848901516168af601f891682615472565b8355505b6001600288020188555050505b505050505050565b6168d181615db0565b82525050565b5f6020820190506168ea5f8301846168c8565b92915050565b6054811061690157616900614f92565b5b50565b5f819050616911826168f0565b919050565b5f61692082616904565b9050919050565b61693081616916565b82525050565b5f6020820190506169495f830184616927565b92915050565b5f81905092915050565b616962816146e7565b82525050565b5f6169738383616959565b60208301905092915050565b5f6169898261469f565b616993818561694f565b935061699e836146b9565b805f5b838110156169ce5781516169b58882616968565b97506169c08361471e565b9250506001810190506169a1565b5085935050505092915050565b5f6169e6828461697f565b915081905092915050565b5f60c082019050616a045f830189614b0b565b616a116020830188614b0b565b616a1e6040830187614b0b565b616a2b6060830186614b6d565b616a386080830185614b6d565b616a4560a0830184614b0b565b979650505050505050565b5f60a082019050616a635f830188614b0b565b616a706020830187614b0b565b616a7d6040830186614b0b565b616a8a6060830185614b6d565b616a976080830184614b7c565b9695505050505050565b5f608082019050616ab45f830187614b0b565b616ac160208301866168c8565b616ace6040830185614b0b565b616adb6060830184614b0b565b9594505050505056fe5573657244656372797074526573706f6e7365566572696669636174696f6e286279746573207075626c69634b65792c627974657333325b5d20637448616e646c65732c6279746573207573657244656372797074656453686172652c627974657320657874726144617461295075626c696344656372797074566572696669636174696f6e28627974657333325b5d20637448616e646c65732c627974657320646563727970746564526573756c742c62797465732065787472614461746129557365724465637279707452657175657374566572696669636174696f6e286279746573207075626c69634b65792c616464726573735b5d20636f6e74726163744164647265737365732c75696e7432353620737461727454696d657374616d702c75696e74323536206475726174696f6e446179732c62797465732065787472614461746129
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP4\x80\x15b\0\0CW_\x80\xFD[Pb\0\0Tb\0\0Z` \x1B` \x1CV[b\0\x01\xC4V[_b\0\0kb\0\x01^` \x1B` \x1CV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15b\0\0\xB6W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14b\0\x01[Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Qb\0\x01R\x91\x90b\0\x01\xA9V[`@Q\x80\x91\x03\x90\xA1[PV[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[b\0\x01\xA3\x81b\0\x01\x85V[\x82RPPV[_` \x82\x01\x90Pb\0\x01\xBE_\x83\x01\x84b\0\x01\x98V[\x92\x91PPV[`\x80Qal-b\0\x01\xEB_9_\x81\x81a$\xD7\x01R\x81\x81a%,\x01Ra'\xCE\x01Ral-_\xF3\xFE`\x80`@R`\x046\x10a\x01\x08W_5`\xE0\x1C\x80c\\\x97Z\xBB\x11a\0\x94W\x80c\xAD<\xB1\xCC\x11a\0cW\x80c\xAD<\xB1\xCC\x14a\x03\x1CW\x80c\xC4\x11Xt\x14a\x03FW\x80c\xD8\x99\x8FE\x14a\x03\\W\x80c\xF1\xB5z\xDB\x14a\x03\x84W\x80c\xFB\xB82Y\x14a\x03\xACWa\x01\x08V[\x80c\\\x97Z\xBB\x14a\x02\x84W\x80co\x89\x13\xBC\x14a\x02\xAEW\x80c\x84V\xCBY\x14a\x02\xD6W\x80c\x84\xB0\x19n\x14a\x02\xECWa\x01\x08V[\x80c?K\xA8:\x11a\0\xDBW\x80c?K\xA8:\x14a\x01\xB0W\x80c@\x14\xC4\xCD\x14a\x01\xC6W\x80cO\x1E\xF2\x86\x14a\x02\x02W\x80cR\xD1\x90-\x14a\x02\x1EW\x80cX\xF5\xB8\xAB\x14a\x02HWa\x01\x08V[\x80c\x04o\x9E\xB3\x14a\x01\x0CW\x80c\t\0\xCCi\x14a\x014W\x80c\r\x8En,\x14a\x01pW\x80c9\xF78\x10\x14a\x01\x9AW[_\x80\xFD[4\x80\x15a\x01\x17W_\x80\xFD[Pa\x012`\x04\x806\x03\x81\x01\x90a\x01-\x91\x90aE\xB0V[a\x03\xE8V[\0[4\x80\x15a\x01?W_\x80\xFD[Pa\x01Z`\x04\x806\x03\x81\x01\x90a\x01U\x91\x90aFtV[a\x08tV[`@Qa\x01g\x91\x90aG\x86V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01{W_\x80\xFD[Pa\x01\x84a\tEV[`@Qa\x01\x91\x91\x90aH0V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xA5W_\x80\xFD[Pa\x01\xAEa\t\xC0V[\0[4\x80\x15a\x01\xBBW_\x80\xFD[Pa\x01\xC4a\x0B\xF8V[\0[4\x80\x15a\x01\xD1W_\x80\xFD[Pa\x01\xEC`\x04\x806\x03\x81\x01\x90a\x01\xE7\x91\x90aH\xA5V[a\r@V[`@Qa\x01\xF9\x91\x90aI=V[`@Q\x80\x91\x03\x90\xF3[a\x02\x1C`\x04\x806\x03\x81\x01\x90a\x02\x17\x91\x90aJ\xA8V[a\x0E\xCDV[\0[4\x80\x15a\x02)W_\x80\xFD[Pa\x022a\x0E\xECV[`@Qa\x02?\x91\x90aK\x1AV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02SW_\x80\xFD[Pa\x02n`\x04\x806\x03\x81\x01\x90a\x02i\x91\x90aFtV[a\x0F\x1DV[`@Qa\x02{\x91\x90aI=V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x8FW_\x80\xFD[Pa\x02\x98a\x0FPV[`@Qa\x02\xA5\x91\x90aI=V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xB9W_\x80\xFD[Pa\x02\xD4`\x04\x806\x03\x81\x01\x90a\x02\xCF\x91\x90aE\xB0V[a\x0FrV[\0[4\x80\x15a\x02\xE1W_\x80\xFD[Pa\x02\xEAa\x13\xBDV[\0[4\x80\x15a\x02\xF7W_\x80\xFD[Pa\x03\0a\x14\xE2V[`@Qa\x03\x13\x97\x96\x95\x94\x93\x92\x91\x90aLBV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03'W_\x80\xFD[Pa\x030a\x15\xEBV[`@Qa\x03=\x91\x90aH0V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03QW_\x80\xFD[Pa\x03Za\x16$V[\0[4\x80\x15a\x03gW_\x80\xFD[Pa\x03\x82`\x04\x806\x03\x81\x01\x90a\x03}\x91\x90aH\xA5V[a\x17IV[\0[4\x80\x15a\x03\x8FW_\x80\xFD[Pa\x03\xAA`\x04\x806\x03\x81\x01\x90a\x03\xA5\x91\x90aMYV[a\x19\x10V[\0[4\x80\x15a\x03\xB7W_\x80\xFD[Pa\x03\xD2`\x04\x806\x03\x81\x01\x90a\x03\xCD\x91\x90aN\x93V[a\x1D\xCAV[`@Qa\x03\xDF\x91\x90aI=V[`@Q\x80\x91\x03\x90\xF3[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE5'^\xAF3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x045\x91\x90aO$V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04PW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04t\x91\x90aOgV[a\x04\xB5W3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04\xAC\x91\x90aO$V[`@Q\x80\x91\x03\x90\xFD[_a\x04\xBEa 9V[\x90P`\xF8`\x02`\x06\x81\x11\x15a\x04\xD6Wa\x04\xD5aO\x92V[[\x90\x1B\x88\x11\x15\x80a\x04\xE9WP\x80`\x08\x01T\x88\x11[\x15a\x05+W\x87`@Q\x7F\xD4\x8A\xF9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05\"\x91\x90aO\xBFV[`@Q\x80\x91\x03\x90\xFD[_\x81`\x07\x01_\x8A\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80Ta\x05Y\x90aP\x05V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x85\x90aP\x05V[\x80\x15a\x05\xD0W\x80`\x1F\x10a\x05\xA7Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xD0V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xB3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x06&W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x06\x12W[PPPPP\x81RPP\x90P_`@Q\x80`\x80\x01`@R\x80\x83_\x01Q\x81R` \x01\x83` \x01Q\x81R` \x01\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81RP\x90P_a\x06\xEC\x82a `V[\x90Pa\x06\xFA\x8B\x82\x8A\x8Aa!'V[_\x84`\x02\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _\x80_\x1B\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x8B\x7F\x7F\xCD\xFBS\x81\x91\x7FUJq}\nTp\xA3?ZI\xBAdE\xF0^\xC4<t\xC0\xBC,\xC6\x08\xB2`\x01\x83\x80T\x90Pa\x07\xB3\x91\x90aPbV[\x8D\x8D\x8D\x8D\x8D\x8D`@Qa\x07\xCC\x97\x96\x95\x94\x93\x92\x91\x90aP\xD1V[`@Q\x80\x91\x03\x90\xA2\x84_\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x08\tWPa\x08\x08\x81\x80T\x90Pa\"\x99V[[\x15a\x08fW`\x01\x85_\x01_\x8E\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x8B\x7F\xE8\x97R\xBE\x0E\xCD\xB6\x8B*n\xB5\xEF\x1A\x89\x109\xE0\xE9*\xE3\xC8\xA6\"t\xC5\x88\x1EH\xEE\xA1\xED%`@Q`@Q\x80\x91\x03\x90\xA2[PPPPPPPPPPPPV[``_a\x08\x7Fa 9V[\x90P_\x81`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\x02\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x82\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\t7W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x08\xEEW[PPPPP\x92PPP\x91\x90PV[```@Q\x80`@\x01`@R\x80`\n\x81R` \x01\x7FDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\t\x86_a#*V[a\t\x90`\x02a#*V[a\t\x99_a#*V[`@Q` \x01a\t\xAC\x94\x93\x92\x91\x90aQ\xFDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`\x01a\t\xCAa#\xF4V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\n\x0BW`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03_a\n\x16a$\x18V[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\n^WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\n\x95W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x0BN`@Q\x80`@\x01`@R\x80`\n\x81R` \x01\x7FDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa$?V[a\x0BVa$UV[_a\x0B_a 9V[\x90P`\xF8`\x01`\x06\x81\x11\x15a\x0BwWa\x0BvaO\x92V[[\x90\x1B\x81`\x06\x01\x81\x90UP`\xF8`\x02`\x06\x81\x11\x15a\x0B\x97Wa\x0B\x96aO\x92V[[\x90\x1B\x81`\x08\x01\x81\x90UPP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x0B\xEC\x91\x90aR}V[`@Q\x80\x91\x03\x90\xA1PPV[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CUW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Cy\x91\x90aR\xAAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\x0C\xF4WPs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a\r6W3`@Q\x7F\xE1\x91f\xEE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r-\x91\x90aO$V[`@Q\x80\x91\x03\x90\xFD[a\r>a$gV[V[_\x80_\x90P[\x85\x85\x90P\x81\x10\x15a\x0E\xBFWs\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x06 2m\x87\x87\x84\x81\x81\x10a\r\x94Wa\r\x93aR\xD5V[[\x90P` \x02\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\xB7\x91\x90aK\x1AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xD2W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xF6\x91\x90aOgV[\x15\x80a\x0E\xA4WPs\xDE@\x91\t\xE0\xFC\xCA\xAE{\x87\xDEQ\x8Fa\xD6\x17\xA3\xFD\xA0\x94s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c-\xDC\x9Ao\x87\x87\x84\x81\x81\x10a\x0E@Wa\x0E?aR\xD5V[[\x90P` \x02\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0Ec\x91\x90aK\x1AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E~W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xA2\x91\x90aOgV[\x15[\x15a\x0E\xB2W_\x91PPa\x0E\xC5V[\x80\x80`\x01\x01\x91PPa\rFV[P`\x01\x90P[\x94\x93PPPPV[a\x0E\xD5a$\xD5V[a\x0E\xDE\x82a%\xBBV[a\x0E\xE8\x82\x82a&\xAEV[PPV[_a\x0E\xF5a'\xCCV[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[_\x80a\x0F'a 9V[\x90P\x80_\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[_\x80a\x0FZa(SV[\x90P\x80_\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x90V[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE5'^\xAF3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\xBF\x91\x90aO$V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xDAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xFE\x91\x90aOgV[a\x10?W3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x106\x91\x90aO$V[`@Q\x80\x91\x03\x90\xFD[_a\x10Ha 9V[\x90P`\xF8`\x01`\x06\x81\x11\x15a\x10`Wa\x10_aO\x92V[[\x90\x1B\x88\x11\x15\x80a\x10sWP\x80`\x06\x01T\x88\x11[\x15a\x10\xB5W\x87`@Q\x7F\xD4\x8A\xF9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\xAC\x91\x90aO\xBFV[`@Q\x80\x91\x03\x90\xFD[_`@Q\x80``\x01`@R\x80\x83`\x05\x01_\x8C\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x11\x1CW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x11\x08W[PPPPP\x81R` \x01\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81RP\x90P_a\x11\xC2\x82a(zV[\x90Pa\x11\xD0\x8A\x82\x89\x89a!'V[_\x83`\x04\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x90P\x80\x88\x88\x90\x91\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x92\x90\x91\x92\x90\x91\x92\x90\x91\x92P\x91\x82a\x12.\x92\x91\x90aT\xA9V[P\x83`\x02\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ 3\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x8A\x7FM{\x1D\xBAI\xE9\xE8F!^\x16!\xF5s|\x81\xD8aLO&\x84\x94\xD8\xB7\x87c,NY\xF0\xE5\x8B\x8B\x8B\x8B3\x8C\x8C`@Qa\x12\xEB\x97\x96\x95\x94\x93\x92\x91\x90aUvV[`@Q\x80\x91\x03\x90\xA2\x83_\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x13(WPa\x13'\x81\x80T\x90Pa)4V[[\x15a\x13\xB0W`\x01\x84_\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x81\x84`\x03\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x8A\x7F\xD7\xE5\x8A6z\nl)\x8Ev\xAD]$\0\x04\xE3'\xAA\x14#\xCB\xE4\xBD\x7F\xF8]Lq^\xF8\xD1_\x8B\x8B\x84\x8A\x8A`@Qa\x13\xA7\x95\x94\x93\x92\x91\x90aW\"V[`@Q\x80\x91\x03\x90\xA2[PPPPPPPPPPPV[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cF\xFB\xF6\x8E3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\n\x91\x90aO$V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14%W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14I\x91\x90aOgV[\x15\x80\x15a\x14\x96WPs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a\x14\xD8W3`@Q\x7F8\x89\x16\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14\xCF\x91\x90aO$V[`@Q\x80\x91\x03\x90\xFD[a\x14\xE0a)\xC5V[V[_``\x80_\x80_``_a\x14\xF4a*4V[\x90P_\x80\x1B\x81_\x01T\x14\x80\x15a\x15\x0FWP_\x80\x1B\x81`\x01\x01T\x14[a\x15NW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15E\x90aW\xBAV[`@Q\x80\x91\x03\x90\xFD[a\x15Va*[V[a\x15^a*\xF9V[F0_\x80\x1B_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15}Wa\x15|aI\x84V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x15\xABW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94\x93\x92\x91\x90\x97P\x97P\x97P\x97P\x97P\x97P\x97PP\x90\x91\x92\x93\x94\x95\x96V[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`\x03_a\x16/a$\x18V[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x16wWP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x16\xAEW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x17=\x91\x90aR}V[`@Q\x80\x91\x03\x90\xA1PPV[a\x17Qa+\x97V[_\x84\x84\x90P\x03a\x17\x8DW`@Q\x7F-\xE7T8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x17\xD6\x84\x84\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa+\xD8V[_s\xDE@\x91\t\xE0\xFC\xCA\xAE{\x87\xDEQ\x8Fa\xD6\x17\xA3\xFD\xA0\x94s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA1O\x89q\x86\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18&\x92\x91\x90aXPV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18@W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18h\x91\x90aZ\xE7V[\x90Pa\x18s\x81a,\x90V[_a\x18|a 9V[\x90P\x80`\x06\x01_\x81T\x80\x92\x91\x90a\x18\x92\x90a[.V[\x91\x90PUP_\x81`\x06\x01T\x90P\x86\x86\x83`\x05\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x91\x90a\x18\xC1\x92\x91\x90aDZV[Pa\x18\xCB3a-vV[\x80\x7F\"\xDBH\n9\xBDrUd8\xAA\xDBJ2\xA3\xD2\xA6c\x8B\x87\xC0;\xBE\xC5\xFE\xF6\x99~\x10\x95\x87\xFF\x84\x87\x87`@Qa\x18\xFF\x93\x92\x91\x90a]\x0BV[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[a\x19\x18a+\x97V[_\x88\x80` \x01\x90a\x19)\x91\x90a]NV[\x90P\x03a\x19bW`@Q\x7FW\xCF\xA2\x17\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\n`\xFF\x16\x88\x80` \x01\x90a\x19w\x91\x90a]NV[\x90P\x11\x15a\x19\xD0W`\n\x88\x80` \x01\x90a\x19\x91\x91\x90a]NV[\x90P`@Q\x7F\xAF\x1F\x04\x95\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19\xC7\x92\x91\x90a]\xECV[`@Q\x80\x91\x03\x90\xFD[a\x19\xE9\x89\x806\x03\x81\x01\x90a\x19\xE4\x91\x90a^`V[a-\xF3V[a\x1AA\x88\x80` \x01\x90a\x19\xFC\x91\x90a]NV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x88a/>V[\x15a\x1A\x95W\x86\x88\x80` \x01\x90a\x1AW\x91\x90a]NV[`@Q\x7F\xDCMx\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1A\x8C\x93\x92\x91\x90a_\x11V[`@Q\x80\x91\x03\x90\xFD[_a\x1A\xF0\x8C\x8C\x8B\x80` \x01\x90a\x1A\xAB\x91\x90a]NV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x8Ba/\xBCV[\x90P_`@Q\x80`\xA0\x01`@R\x80\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8B\x80` \x01\x90a\x1BW\x91\x90a]NV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8C_\x015\x81R` \x01\x8C` \x015\x81R` \x01\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81RP\x90Pa\x1C\x07\x81\x8A\x88\x88\x8E_\x015a1\xAFV[_s\xDE@\x91\t\xE0\xFC\xCA\xAE{\x87\xDEQ\x8Fa\xD6\x17\xA3\xFD\xA0\x94s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA1O\x89q\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1CU\x91\x90a_\xD9V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CoW=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x97\x91\x90aZ\xE7V[\x90Pa\x1C\xA2\x81a,\x90V[_a\x1C\xABa 9V[\x90P\x80`\x08\x01_\x81T\x80\x92\x91\x90a\x1C\xC1\x90a[.V[\x91\x90PUP_\x81`\x08\x01T\x90P`@Q\x80`@\x01`@R\x80\x8C\x8C\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x86\x81RP\x82`\x07\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01\x90\x81a\x1DL\x91\x90a`\x03V[P` \x82\x01Q\x81`\x01\x01\x90\x80Q\x90` \x01\x90a\x1Di\x92\x91\x90aD\xA5V[P\x90PPa\x1Dv3a2\x87V[\x80\x7F\xF9\x01\x1B\xD6\xBA\r\xA6\x04\x9CR\rp\xFEYq\xF1~\xD7\xAByT\x86\x05%D\xB5\x10\x19\x89lYk\x84\x8E\x8E\x8E\x8C\x8C`@Qa\x1D\xB0\x96\x95\x94\x93\x92\x91\x90a`\xD2V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPPPPPV[_\x80_\x90P[\x85\x85\x90P\x81\x10\x15a *Ws\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC6R\x8Fi\x87\x87\x84\x81\x81\x10a\x1E\x1EWa\x1E\x1DaR\xD5V[[\x90P`@\x02\x01_\x015\x89`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1EE\x92\x91\x90aa.V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E`W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x84\x91\x90aOgV[\x15\x80a\x1F`WPs\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC6R\x8Fi\x87\x87\x84\x81\x81\x10a\x1E\xCEWa\x1E\xCDaR\xD5V[[\x90P`@\x02\x01_\x015\x88\x88\x85\x81\x81\x10a\x1E\xEAWa\x1E\xE9aR\xD5V[[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x1F\x02\x91\x90aaUV[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1F\x1F\x92\x91\x90aa.V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F:W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F^\x91\x90aOgV[\x15[\x80a \x0FWPs\xDE@\x91\t\xE0\xFC\xCA\xAE{\x87\xDEQ\x8Fa\xD6\x17\xA3\xFD\xA0\x94s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c-\xDC\x9Ao\x87\x87\x84\x81\x81\x10a\x1F\xA9Wa\x1F\xA8aR\xD5V[[\x90P`@\x02\x01_\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1F\xCE\x91\x90aK\x1AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\xE9W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \r\x91\x90aOgV[\x15[\x15a \x1DW_\x91PPa 0V[\x80\x80`\x01\x01\x91PPa\x1D\xD0V[P`\x01\x90P[\x95\x94PPPPPV[_\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\0\x90P\x90V[_a! `@Q\x80`\xA0\x01`@R\x80`m\x81R` \x01aj\xE5`m\x919\x80Q\x90` \x01 \x83_\x01Q\x80Q\x90` \x01 \x84` \x01Q`@Q` \x01a \xA4\x91\x90ab\x0CV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85`@\x01Q\x80Q\x90` \x01 \x86``\x01Q`@Q` \x01a \xDB\x91\x90ab\\V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a!\x05\x95\x94\x93\x92\x91\x90abrV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a3\x04V[\x90P\x91\x90PV[_a!0a 9V[\x90P_a!\x80\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa3\x1DV[\x90Pa!\x8C\x813a3GV[\x81`\x01\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\"+W\x85\x81`@Q\x7F\x99\xECH\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\"\"\x92\x91\x90ab\xC3V[`@Q\x80\x91\x03\x90\xFD[`\x01\x82`\x01\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPPPPPPV[_\x80s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC2\xB4)\x86`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\xF8W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\x1C\x91\x90ab\xEAV[\x90P\x80\x83\x10\x15\x91PP\x91\x90PV[``_`\x01a#8\x84a4XV[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#VWa#UaI\x84V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a#\x88W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x01\x82\x01\x90P[`\x01\x15a#\xE9W\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a#\xDEWa#\xDDac\x15V[[\x04\x94P_\x85\x03a#\x95W[\x81\x93PPPP\x91\x90PV[_a#\xFDa$\x18V[_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[a$Ga5\xA9V[a$Q\x82\x82a5\xE9V[PPV[a$]a5\xA9V[a$ea6:V[V[a$oa6jV[_a$xa(SV[\x90P_\x81_\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAAa$\xBDa6\xAAV[`@Qa$\xCA\x91\x90aO$V[`@Q\x80\x91\x03\x90\xA1PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a%\x82WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a%ia6\xB1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a%\xB9W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\x18W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&<\x91\x90aR\xAAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a&\xABW3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&\xA2\x91\x90aO$V[`@Q\x80\x91\x03\x90\xFD[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a'\x16WP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\x13\x91\x90acBV[`\x01[a'WW\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a'N\x91\x90aO$V[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14a'\xBDW\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a'\xB4\x91\x90aK\x1AV[`@Q\x80\x91\x03\x90\xFD[a'\xC7\x83\x83a7\x04V[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a(QW`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0\x90P\x90V[_a)-`@Q\x80`\x80\x01`@R\x80`T\x81R` \x01akR`T\x919\x80Q\x90` \x01 \x83_\x01Q`@Q` \x01a(\xB2\x91\x90ab\x0CV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q\x80Q\x90` \x01 \x85`@\x01Q`@Q` \x01a(\xE9\x91\x90ab\\V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a)\x12\x94\x93\x92\x91\x90acmV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a3\x04V[\x90P\x91\x90PV[_\x80s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c*8\x89\x98`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)\x93W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\xB7\x91\x90ab\xEAV[\x90P\x80\x83\x10\x15\x91PP\x91\x90PV[a)\xCDa+\x97V[_a)\xD6a(SV[\x90P`\x01\x81_\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2Xa*\x1Ca6\xAAV[`@Qa*)\x91\x90aO$V[`@Q\x80\x91\x03\x90\xA1PV[_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x90P\x90V[``_a*fa*4V[\x90P\x80`\x02\x01\x80Ta*w\x90aP\x05V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta*\xA3\x90aP\x05V[\x80\x15a*\xEEW\x80`\x1F\x10a*\xC5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a*\xEEV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a*\xD1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[``_a+\x04a*4V[\x90P\x80`\x03\x01\x80Ta+\x15\x90aP\x05V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta+A\x90aP\x05V[\x80\x15a+\x8CW\x80`\x1F\x10a+cWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a+\x8CV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a+oW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[a+\x9Fa\x0FPV[\x15a+\xD6W`@Q\x7F\xD9<\x06e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x80[\x82Q\x81\x10\x15a,@W_\x83\x82\x81Q\x81\x10a+\xF8Wa+\xF7aR\xD5V[[` \x02` \x01\x01Q\x90P_a,\x0C\x82a7vV[\x90Pa,\x17\x81a8\0V[a\xFF\xFF\x16\x84a,&\x91\x90ac\xB0V[\x93Pa,1\x82a9\xEBV[PP\x80\x80`\x01\x01\x91PPa+\xDBV[Pa\x08\0\x81\x11\x15a,\x8CWa\x08\0\x81`@Q\x7F\xE7\xF4\x89]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a,\x83\x92\x91\x90ac\xE3V[`@Q\x80\x91\x03\x90\xFD[PPV[`\x01\x81Q\x11\x15a-sW_\x81_\x81Q\x81\x10a,\xAEWa,\xADaR\xD5V[[` \x02` \x01\x01Q` \x01Q\x90P_`\x01\x90P[\x82Q\x81\x10\x15a-pW\x81\x83\x82\x81Q\x81\x10a,\xDFWa,\xDEaR\xD5V[[` \x02` \x01\x01Q` \x01Q\x14a-cW\x82_\x81Q\x81\x10a-\x03Wa-\x02aR\xD5V[[` \x02` \x01\x01Q\x83\x82\x81Q\x81\x10a-\x1EWa-\x1DaR\xD5V[[` \x02` \x01\x01Q`@Q\x7F\xCF\xAE\x92\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a-Z\x92\x91\x90adjV[`@Q\x80\x91\x03\x90\xFD[\x80\x80`\x01\x01\x91PPa,\xC2V[PP[PV[s\x873\xD4\x01>\xFCBV\x97qP\xF3\x1A\x8E\xA1\xE9\xE4\xC1E\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x91\xEE\xB2|\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a-\xC3\x91\x90aO$V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a-\xDAW_\x80\xFD[PZ\xF1\x15\x80\x15a-\xECW=_\x80>=_\xFD[PPPPPV[_\x81` \x01Q\x03a.0W`@Q\x7F\xDE(Y\xC1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01ma\xFF\xFF\x16\x81` \x01Q\x11\x15a.\x87Wa\x01m\x81` \x01Q`@Q\x7F2\x95\x18c\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.~\x92\x91\x90ad\xDCV[`@Q\x80\x91\x03\x90\xFD[B\x81_\x01Q\x11\x15a.\xD4WB\x81_\x01Q`@Q\x7F\xF2L\x08\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.\xCB\x92\x91\x90ac\xE3V[`@Q\x80\x91\x03\x90\xFD[Bb\x01Q\x80\x82` \x01Qa.\xE8\x91\x90ae\x03V[\x82_\x01Qa.\xF6\x91\x90ac\xB0V[\x10\x15a/;WB\x81`@Q\x7F04\x80@\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a/2\x92\x91\x90aeqV[`@Q\x80\x91\x03\x90\xFD[PV[_\x80_\x90P[\x83Q\x81\x10\x15a/\xB1W\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x82\x81Q\x81\x10a/wWa/vaR\xD5V[[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a/\xA4W`\x01\x91PPa/\xB6V[\x80\x80`\x01\x01\x91PPa/DV[P_\x90P[\x92\x91PPV[``_\x85\x85\x90P\x03a/\xFAW`@Q\x7F\xA6\xA6\xCB!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84\x84\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0\x17Wa0\x16aI\x84V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a0EW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x80[\x86\x86\x90P\x81\x10\x15a1ZW_\x87\x87\x83\x81\x81\x10a0jWa0iaR\xD5V[[\x90P`@\x02\x01_\x015\x90P_\x88\x88\x84\x81\x81\x10a0\x89Wa0\x88aR\xD5V[[\x90P`@\x02\x01` \x01` \x81\x01\x90a0\xA1\x91\x90aaUV[\x90P_a0\xAD\x83a7vV[\x90Pa0\xB8\x81a8\0V[a\xFF\xFF\x16\x85a0\xC7\x91\x90ac\xB0V[\x94Pa0\xD3\x83\x88a:\xBBV[a0\xDD\x83\x83a:\xBBV[a0\xE7\x88\x83a/>V[a1*W\x81\x88`@Q\x7F\xA4\xC3\x03\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a1!\x92\x91\x90ae\x98V[`@Q\x80\x91\x03\x90\xFD[\x82\x86\x85\x81Q\x81\x10a1>Wa1=aR\xD5V[[` \x02` \x01\x01\x81\x81RPPPPP\x80\x80`\x01\x01\x91PPa0KV[Pa\x08\0\x81\x11\x15a1\xA6Wa\x08\0\x81`@Q\x7F\xE7\xF4\x89]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a1\x9D\x92\x91\x90ac\xE3V[`@Q\x80\x91\x03\x90\xFD[P\x94\x93PPPPV[_a1\xBA\x86\x83a;\x90V[\x90P_a2\n\x82\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa3\x1DV[\x90P\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a2~W\x84\x84`@Q\x7F*\x87='\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a2u\x92\x91\x90ae\xC6V[`@Q\x80\x91\x03\x90\xFD[PPPPPPPV[s\x873\xD4\x01>\xFCBV\x97qP\xF3\x1A\x8E\xA1\xE9\xE4\xC1E\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x98\x8A--\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a2\xD4\x91\x90aO$V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a2\xEBW_\x80\xFD[PZ\xF1\x15\x80\x15a2\xFDW=_\x80>=_\xFD[PPPPPV[_a3\x16a3\x10a<]V[\x83a<kV[\x90P\x91\x90PV[_\x80_\x80a3+\x86\x86a<\xABV[\x92P\x92P\x92Pa3;\x82\x82a=\0V[\x82\x93PPPP\x92\x91PPV[a3P\x82a>bV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3\xB2\xA8t\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a3\xB4\x91\x90aO$V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3\xCEW=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\xF6\x91\x90ag3V[` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a4TW\x81\x81`@Q\x7F\r\x86\xF5!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a4K\x92\x91\x90agzV[`@Q\x80\x91\x03\x90\xFD[PPV[_\x80_\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a4\xB4Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81a4\xAAWa4\xA9ac\x15V[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a4\xF1Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81a4\xE7Wa4\xE6ac\x15V[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10a5 Wf#\x86\xF2o\xC1\0\0\x83\x81a5\x16Wa5\x15ac\x15V[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10a5IWc\x05\xF5\xE1\0\x83\x81a5?Wa5>ac\x15V[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10a5nWa'\x10\x83\x81a5dWa5cac\x15V[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10a5\x91W`d\x83\x81a5\x87Wa5\x86ac\x15V[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10a5\xA0W`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[a5\xB1a?2V[a5\xE7W`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a5\xF1a5\xA9V[_a5\xFAa*4V[\x90P\x82\x81`\x02\x01\x90\x81a6\r\x91\x90ag\xF9V[P\x81\x81`\x03\x01\x90\x81a6\x1F\x91\x90ag\xF9V[P_\x80\x1B\x81_\x01\x81\x90UP_\x80\x1B\x81`\x01\x01\x81\x90UPPPPV[a6Ba5\xA9V[_a6Ka(SV[\x90P_\x81_\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPV[a6ra\x0FPV[a6\xA8W`@Q\x7F\x8D\xFC +\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_3\x90P\x90V[_a6\xDD\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba?PV[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a7\r\x82a?YV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15a7iWa7c\x82\x82a@\"V[Pa7rV[a7qa@\xA2V[[PPV[_\x80`\xF8`\xF0\x84\x90\x1B\x90\x1C_\x1C\x90P`S\x80\x81\x11\x15a7\x98Wa7\x97aO\x92V[[`\xFF\x16\x81`\xFF\x16\x11\x15a7\xE2W\x80`@Q\x7Fd\x19P\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a7\xD9\x91\x90ah\xD7V[`@Q\x80\x91\x03\x90\xFD[\x80`\xFF\x16`S\x81\x11\x15a7\xF8Wa7\xF7aO\x92V[[\x91PP\x91\x90PV[_\x80`S\x81\x11\x15a8\x14Wa8\x13aO\x92V[[\x82`S\x81\x11\x15a8'Wa8&aO\x92V[[\x03a85W`\x02\x90Pa9\xE6V[`\x02`S\x81\x11\x15a8IWa8HaO\x92V[[\x82`S\x81\x11\x15a8\\Wa8[aO\x92V[[\x03a8jW`\x08\x90Pa9\xE6V[`\x03`S\x81\x11\x15a8~Wa8}aO\x92V[[\x82`S\x81\x11\x15a8\x91Wa8\x90aO\x92V[[\x03a8\x9FW`\x10\x90Pa9\xE6V[`\x04`S\x81\x11\x15a8\xB3Wa8\xB2aO\x92V[[\x82`S\x81\x11\x15a8\xC6Wa8\xC5aO\x92V[[\x03a8\xD4W` \x90Pa9\xE6V[`\x05`S\x81\x11\x15a8\xE8Wa8\xE7aO\x92V[[\x82`S\x81\x11\x15a8\xFBWa8\xFAaO\x92V[[\x03a9\tW`@\x90Pa9\xE6V[`\x06`S\x81\x11\x15a9\x1DWa9\x1CaO\x92V[[\x82`S\x81\x11\x15a90Wa9/aO\x92V[[\x03a9>W`\x80\x90Pa9\xE6V[`\x07`S\x81\x11\x15a9RWa9QaO\x92V[[\x82`S\x81\x11\x15a9eWa9daO\x92V[[\x03a9sW`\xA0\x90Pa9\xE6V[`\x08`S\x81\x11\x15a9\x87Wa9\x86aO\x92V[[\x82`S\x81\x11\x15a9\x9AWa9\x99aO\x92V[[\x03a9\xA9Wa\x01\0\x90Pa9\xE6V[\x81`@Q\x7F\xBEx0\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a9\xDD\x91\x90ai6V[`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x06 2m\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a:8\x91\x90aK\x1AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:SW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:w\x91\x90aOgV[a:\xB8W\x80`@Q\x7FC1\xA8]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a:\xAF\x91\x90aK\x1AV[`@Q\x80\x91\x03\x90\xFD[PV[s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC6R\x8Fi\x83\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a;\n\x92\x91\x90aa.V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;%W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;I\x91\x90aOgV[a;\x8CW\x81\x81`@Q\x7F\x16\n+K\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a;\x83\x92\x91\x90aa.V[`@Q\x80\x91\x03\x90\xFD[PPV[_\x80`@Q\x80`\xC0\x01`@R\x80`\x87\x81R` \x01ak\xA6`\x87\x919\x80Q\x90` \x01 \x84_\x01Q\x80Q\x90` \x01 \x85` \x01Q`@Q` \x01a;\xD2\x91\x90ai\xDBV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x86`@\x01Q\x87``\x01Q\x88`\x80\x01Q`@Q` \x01a<\x07\x91\x90ab\\V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a<2\x96\x95\x94\x93\x92\x91\x90ai\xF1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa<T\x83\x82a@\xDEV[\x91PP\x92\x91PPV[_a<faARV[\x90P\x90V[_`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x02\x82\x01R\x82`\"\x82\x01R`B\x81 \x91PP\x92\x91PPV[_\x80_`A\x84Q\x03a<\xEBW_\x80_` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90Pa<\xDD\x88\x82\x85\x85aA\xB5V[\x95P\x95P\x95PPPPa<\xF9V[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15a=\x13Wa=\x12aO\x92V[[\x82`\x03\x81\x11\x15a=&Wa=%aO\x92V[[\x03\x15a>^W`\x01`\x03\x81\x11\x15a=@Wa=?aO\x92V[[\x82`\x03\x81\x11\x15a=SWa=RaO\x92V[[\x03a=\x8AW`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15a=\x9EWa=\x9DaO\x92V[[\x82`\x03\x81\x11\x15a=\xB1Wa=\xB0aO\x92V[[\x03a=\xF5W\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a=\xEC\x91\x90aO\xBFV[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15a>\x08Wa>\x07aO\x92V[[\x82`\x03\x81\x11\x15a>\x1BWa>\x1AaO\x92V[[\x03a>]W\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a>T\x91\x90aK\x1AV[`@Q\x80\x91\x03\x90\xFD[[PPV[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c =\x01\x14\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a>\xAF\x91\x90aO$V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>\xCAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>\xEE\x91\x90aOgV[a?/W\x80`@Q\x7F*|n\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a?&\x91\x90aO$V[`@Q\x80\x91\x03\x90\xFD[PV[_a?;a$\x18V[_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a?\xB4W\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a?\xAB\x91\x90aO$V[`@Q\x80\x91\x03\x90\xFD[\x80a?\xE0\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba?PV[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qa@K\x91\x90ab\\V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a@\x83W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a@\x88V[``\x91P[P\x91P\x91Pa@\x98\x85\x83\x83aB\x9CV[\x92PPP\x92\x91PPV[_4\x11\x15a@\xDCW`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x80\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0FaA\taC)V[aA\x11aC\x9FV[\x860`@Q` \x01aA'\x95\x94\x93\x92\x91\x90ajPV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90PaAI\x81\x84a<kV[\x91PP\x92\x91PPV[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0FaA|aC)V[aA\x84aC\x9FV[F0`@Q` \x01aA\x9A\x95\x94\x93\x92\x91\x90ajPV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x80_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15aA\xF1W_`\x03\x85\x92P\x92P\x92PaB\x92V[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@QaB\x14\x94\x93\x92\x91\x90aj\xA1V[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aB4W=_\x80>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03aB\x85W_`\x01_\x80\x1B\x93P\x93P\x93PPaB\x92V[\x80_\x80_\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[``\x82aB\xB1WaB\xAC\x82aD\x16V[aC!V[_\x82Q\x14\x80\x15aB\xD7WP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15aC\x19W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aC\x10\x91\x90aO$V[`@Q\x80\x91\x03\x90\xFD[\x81\x90PaC\"V[[\x93\x92PPPV[_\x80aC3a*4V[\x90P_aC>a*[V[\x90P_\x81Q\x11\x15aCZW\x80\x80Q\x90` \x01 \x92PPPaC\x9CV[_\x82_\x01T\x90P_\x80\x1B\x81\x14aCuW\x80\x93PPPPaC\x9CV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x80aC\xA9a*4V[\x90P_aC\xB4a*\xF9V[\x90P_\x81Q\x11\x15aC\xD0W\x80\x80Q\x90` \x01 \x92PPPaD\x13V[_\x82`\x01\x01T\x90P_\x80\x1B\x81\x14aC\xECW\x80\x93PPPPaD\x13V[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x81Q\x11\x15aD(W\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15aD\x94W\x91` \x02\x82\x01[\x82\x81\x11\x15aD\x93W\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90aDxV[[P\x90PaD\xA1\x91\x90aD\xF0V[P\x90V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15aD\xDFW\x91` \x02\x82\x01[\x82\x81\x11\x15aD\xDEW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90aD\xC3V[[P\x90PaD\xEC\x91\x90aD\xF0V[P\x90V[[\x80\x82\x11\x15aE\x07W_\x81_\x90UP`\x01\x01aD\xF1V[P\x90V[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x81\x90P\x91\x90PV[aE.\x81aE\x1CV[\x81\x14aE8W_\x80\xFD[PV[_\x815\x90PaEI\x81aE%V[\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12aEpWaEoaEOV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aE\x8DWaE\x8CaESV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15aE\xA9WaE\xA8aEWV[[\x92P\x92\x90PV[_\x80_\x80_\x80_`\x80\x88\x8A\x03\x12\x15aE\xCBWaE\xCAaE\x14V[[_aE\xD8\x8A\x82\x8B\x01aE;V[\x97PP` \x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aE\xF9WaE\xF8aE\x18V[[aF\x05\x8A\x82\x8B\x01aE[V[\x96P\x96PP`@\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aF(WaF'aE\x18V[[aF4\x8A\x82\x8B\x01aE[V[\x94P\x94PP``\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aFWWaFVaE\x18V[[aFc\x8A\x82\x8B\x01aE[V[\x92P\x92PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[_` \x82\x84\x03\x12\x15aF\x89WaF\x88aE\x14V[[_aF\x96\x84\x82\x85\x01aE;V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_aF\xF1\x82aF\xC8V[\x90P\x91\x90PV[aG\x01\x81aF\xE7V[\x82RPPV[_aG\x12\x83\x83aF\xF8V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aG4\x82aF\x9FV[aG>\x81\x85aF\xA9V[\x93PaGI\x83aF\xB9V[\x80_[\x83\x81\x10\x15aGyW\x81QaG`\x88\x82aG\x07V[\x97PaGk\x83aG\x1EV[\x92PP`\x01\x81\x01\x90PaGLV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaG\x9E\x81\x84aG*V[\x90P\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15aG\xDDW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90PaG\xC2V[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_aH\x02\x82aG\xA6V[aH\x0C\x81\x85aG\xB0V[\x93PaH\x1C\x81\x85` \x86\x01aG\xC0V[aH%\x81aG\xE8V[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaHH\x81\x84aG\xF8V[\x90P\x92\x91PPV[_\x80\x83`\x1F\x84\x01\x12aHeWaHdaEOV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aH\x82WaH\x81aESV[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aH\x9EWaH\x9DaEWV[[\x92P\x92\x90PV[_\x80_\x80`@\x85\x87\x03\x12\x15aH\xBDWaH\xBCaE\x14V[[_\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aH\xDAWaH\xD9aE\x18V[[aH\xE6\x87\x82\x88\x01aHPV[\x94P\x94PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aI\tWaI\x08aE\x18V[[aI\x15\x87\x82\x88\x01aE[V[\x92P\x92PP\x92\x95\x91\x94P\x92PV[_\x81\x15\x15\x90P\x91\x90PV[aI7\x81aI#V[\x82RPPV[_` \x82\x01\x90PaIP_\x83\x01\x84aI.V[\x92\x91PPV[aI_\x81aF\xE7V[\x81\x14aIiW_\x80\xFD[PV[_\x815\x90PaIz\x81aIVV[\x92\x91PPV[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[aI\xBA\x82aG\xE8V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aI\xD9WaI\xD8aI\x84V[[\x80`@RPPPV[_aI\xEBaE\x0BV[\x90PaI\xF7\x82\x82aI\xB1V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aJ\x16WaJ\x15aI\x84V[[aJ\x1F\x82aG\xE8V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_aJLaJG\x84aI\xFCV[aI\xE2V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15aJhWaJgaI\x80V[[aJs\x84\x82\x85aJ,V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aJ\x8FWaJ\x8EaEOV[[\x815aJ\x9F\x84\x82` \x86\x01aJ:V[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15aJ\xBEWaJ\xBDaE\x14V[[_aJ\xCB\x85\x82\x86\x01aIlV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aJ\xECWaJ\xEBaE\x18V[[aJ\xF8\x85\x82\x86\x01aJ{V[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[aK\x14\x81aK\x02V[\x82RPPV[_` \x82\x01\x90PaK-_\x83\x01\x84aK\x0BV[\x92\x91PPV[_\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[aKg\x81aK3V[\x82RPPV[aKv\x81aE\x1CV[\x82RPPV[aK\x85\x81aF\xE7V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[aK\xBD\x81aE\x1CV[\x82RPPV[_aK\xCE\x83\x83aK\xB4V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aK\xF0\x82aK\x8BV[aK\xFA\x81\x85aK\x95V[\x93PaL\x05\x83aK\xA5V[\x80_[\x83\x81\x10\x15aL5W\x81QaL\x1C\x88\x82aK\xC3V[\x97PaL'\x83aK\xDAV[\x92PP`\x01\x81\x01\x90PaL\x08V[P\x85\x93PPPP\x92\x91PPV[_`\xE0\x82\x01\x90PaLU_\x83\x01\x8AaK^V[\x81\x81\x03` \x83\x01RaLg\x81\x89aG\xF8V[\x90P\x81\x81\x03`@\x83\x01RaL{\x81\x88aG\xF8V[\x90PaL\x8A``\x83\x01\x87aKmV[aL\x97`\x80\x83\x01\x86aK|V[aL\xA4`\xA0\x83\x01\x85aK\x0BV[\x81\x81\x03`\xC0\x83\x01RaL\xB6\x81\x84aK\xE6V[\x90P\x98\x97PPPPPPPPV[_\x80\x83`\x1F\x84\x01\x12aL\xD9WaL\xD8aEOV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aL\xF6WaL\xF5aESV[[` \x83\x01\x91P\x83`@\x82\x02\x83\x01\x11\x15aM\x12WaM\x11aEWV[[\x92P\x92\x90PV[_\x80\xFD[_`@\x82\x84\x03\x12\x15aM2WaM1aM\x19V[[\x81\x90P\x92\x91PPV[_`@\x82\x84\x03\x12\x15aMPWaMOaM\x19V[[\x81\x90P\x92\x91PPV[_\x80_\x80_\x80_\x80_\x80_a\x01\0\x8C\x8E\x03\x12\x15aMyWaMxaE\x14V[[_\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aM\x96WaM\x95aE\x18V[[aM\xA2\x8E\x82\x8F\x01aL\xC4V[\x9BP\x9BPP` aM\xB5\x8E\x82\x8F\x01aM\x1DV[\x99PP``\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aM\xD6WaM\xD5aE\x18V[[aM\xE2\x8E\x82\x8F\x01aM;V[\x98PP`\x80aM\xF3\x8E\x82\x8F\x01aIlV[\x97PP`\xA0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aN\x14WaN\x13aE\x18V[[aN \x8E\x82\x8F\x01aE[V[\x96P\x96PP`\xC0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aNCWaNBaE\x18V[[aNO\x8E\x82\x8F\x01aE[V[\x94P\x94PP`\xE0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aNrWaNqaE\x18V[[aN~\x8E\x82\x8F\x01aE[V[\x92P\x92PP\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[_\x80_\x80_``\x86\x88\x03\x12\x15aN\xACWaN\xABaE\x14V[[_aN\xB9\x88\x82\x89\x01aIlV[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aN\xDAWaN\xD9aE\x18V[[aN\xE6\x88\x82\x89\x01aL\xC4V[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aO\tWaO\x08aE\x18V[[aO\x15\x88\x82\x89\x01aE[V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[_` \x82\x01\x90PaO7_\x83\x01\x84aK|V[\x92\x91PPV[aOF\x81aI#V[\x81\x14aOPW_\x80\xFD[PV[_\x81Q\x90PaOa\x81aO=V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aO|WaO{aE\x14V[[_aO\x89\x84\x82\x85\x01aOSV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[_` \x82\x01\x90PaO\xD2_\x83\x01\x84aKmV[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80aP\x1CW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aP/WaP.aO\xD8V[[P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_aPl\x82aE\x1CV[\x91PaPw\x83aE\x1CV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15aP\x8FWaP\x8EaP5V[[\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aP\xB0\x83\x85aP\x95V[\x93PaP\xBD\x83\x85\x84aJ,V[aP\xC6\x83aG\xE8V[\x84\x01\x90P\x93\x92PPPV[_`\x80\x82\x01\x90PaP\xE4_\x83\x01\x8AaKmV[\x81\x81\x03` \x83\x01RaP\xF7\x81\x88\x8AaP\xA5V[\x90P\x81\x81\x03`@\x83\x01RaQ\x0C\x81\x86\x88aP\xA5V[\x90P\x81\x81\x03``\x83\x01RaQ!\x81\x84\x86aP\xA5V[\x90P\x98\x97PPPPPPPPV[_\x81\x90P\x92\x91PPV[_aQC\x82aG\xA6V[aQM\x81\x85aQ/V[\x93PaQ]\x81\x85` \x86\x01aG\xC0V[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aQ\x9D`\x02\x83aQ/V[\x91PaQ\xA8\x82aQiV[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aQ\xE7`\x01\x83aQ/V[\x91PaQ\xF2\x82aQ\xB3V[`\x01\x82\x01\x90P\x91\x90PV[_aR\x08\x82\x87aQ9V[\x91PaR\x13\x82aQ\x91V[\x91PaR\x1F\x82\x86aQ9V[\x91PaR*\x82aQ\xDBV[\x91PaR6\x82\x85aQ9V[\x91PaRA\x82aQ\xDBV[\x91PaRM\x82\x84aQ9V[\x91P\x81\x90P\x95\x94PPPPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[aRw\x81aR[V[\x82RPPV[_` \x82\x01\x90PaR\x90_\x83\x01\x84aRnV[\x92\x91PPV[_\x81Q\x90PaR\xA4\x81aIVV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aR\xBFWaR\xBEaE\x14V[[_aR\xCC\x84\x82\x85\x01aR\x96V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x82\x90P\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02aSh\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82aS-V[aSr\x86\x83aS-V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_aS\xADaS\xA8aS\xA3\x84aE\x1CV[aS\x8AV[aE\x1CV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[aS\xC6\x83aS\x93V[aS\xDAaS\xD2\x82aS\xB4V[\x84\x84TaS9V[\x82UPPPPV[_\x90V[aS\xEEaS\xE2V[aS\xF9\x81\x84\x84aS\xBDV[PPPV[[\x81\x81\x10\x15aT\x1CWaT\x11_\x82aS\xE6V[`\x01\x81\x01\x90PaS\xFFV[PPV[`\x1F\x82\x11\x15aTaWaT2\x81aS\x0CV[aT;\x84aS\x1EV[\x81\x01` \x85\x10\x15aTJW\x81\x90P[aT^aTV\x85aS\x1EV[\x83\x01\x82aS\xFEV[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_aT\x81_\x19\x84`\x08\x02aTfV[\x19\x80\x83\x16\x91PP\x92\x91PPV[_aT\x99\x83\x83aTrV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[aT\xB3\x83\x83aS\x02V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aT\xCCWaT\xCBaI\x84V[[aT\xD6\x82TaP\x05V[aT\xE1\x82\x82\x85aT V[_`\x1F\x83\x11`\x01\x81\x14aU\x0EW_\x84\x15aT\xFCW\x82\x87\x015\x90P[aU\x06\x85\x82aT\x8EV[\x86UPaUmV[`\x1F\x19\x84\x16aU\x1C\x86aS\x0CV[_[\x82\x81\x10\x15aUCW\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PaU\x1EV[\x86\x83\x10\x15aU`W\x84\x89\x015aU\\`\x1F\x89\x16\x82aTrV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[_`\x80\x82\x01\x90P\x81\x81\x03_\x83\x01RaU\x8F\x81\x89\x8BaP\xA5V[\x90P\x81\x81\x03` \x83\x01RaU\xA4\x81\x87\x89aP\xA5V[\x90PaU\xB3`@\x83\x01\x86aK|V[\x81\x81\x03``\x83\x01RaU\xC6\x81\x84\x86aP\xA5V[\x90P\x98\x97PPPPPPPPV[_\x81T\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81TaV\x1C\x81aP\x05V[aV&\x81\x86aV\0V[\x94P`\x01\x82\x16_\x81\x14aV@W`\x01\x81\x14aVVWaV\x88V[`\xFF\x19\x83\x16\x86R\x81\x15\x15` \x02\x86\x01\x93PaV\x88V[aV_\x85aS\x0CV[_[\x83\x81\x10\x15aV\x80W\x81T\x81\x89\x01R`\x01\x82\x01\x91P` \x81\x01\x90PaVaV[\x80\x88\x01\x95PPP[PPP\x92\x91PPV[_aV\x9C\x83\x83aV\x10V[\x90P\x92\x91PPV[_`\x01\x82\x01\x90P\x91\x90PV[_aV\xBA\x82aU\xD4V[aV\xC4\x81\x85aU\xDEV[\x93P\x83` \x82\x02\x85\x01aV\xD6\x85aU\xEEV[\x80_[\x85\x81\x10\x15aW\x10W\x84\x84\x03\x89R\x81aV\xF1\x85\x82aV\x91V[\x94PaV\xFC\x83aV\xA4V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaV\xD9V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_``\x82\x01\x90P\x81\x81\x03_\x83\x01RaW;\x81\x87\x89aP\xA5V[\x90P\x81\x81\x03` \x83\x01RaWO\x81\x86aV\xB0V[\x90P\x81\x81\x03`@\x83\x01RaWd\x81\x84\x86aP\xA5V[\x90P\x96\x95PPPPPPV[\x7FEIP712: Uninitialized\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aW\xA4`\x15\x83aG\xB0V[\x91PaW\xAF\x82aWpV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaW\xD1\x81aW\x98V[\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x80\xFD[\x82\x81\x837PPPV[_aX\0\x83\x85aW\xD8V[\x93P\x7F\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15aX3WaX2aW\xE8V[[` \x83\x02\x92PaXD\x83\x85\x84aW\xECV[\x82\x84\x01\x90P\x93\x92PPPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaXi\x81\x84\x86aW\xF5V[\x90P\x93\x92PPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aX\x8CWaX\x8BaI\x84V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x80\xFD[_\x80\xFD[aX\xAE\x81aK\x02V[\x81\x14aX\xB8W_\x80\xFD[PV[_\x81Q\x90PaX\xC9\x81aX\xA5V[\x92\x91PPV[_\x81Q\x90PaX\xDD\x81aE%V[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aX\xFDWaX\xFCaI\x84V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_aY aY\x1B\x84aX\xE3V[aI\xE2V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15aYCWaYBaEWV[[\x83[\x81\x81\x10\x15aYlW\x80aYX\x88\x82aR\x96V[\x84R` \x84\x01\x93PP` \x81\x01\x90PaYEV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aY\x8AWaY\x89aEOV[[\x81QaY\x9A\x84\x82` \x86\x01aY\x0EV[\x91PP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15aY\xB8WaY\xB7aX\x9DV[[aY\xC2`\x80aI\xE2V[\x90P_aY\xD1\x84\x82\x85\x01aX\xBBV[_\x83\x01RP` aY\xE4\x84\x82\x85\x01aX\xCFV[` \x83\x01RP`@aY\xF8\x84\x82\x85\x01aX\xBBV[`@\x83\x01RP``\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZ\x1CWaZ\x1BaX\xA1V[[aZ(\x84\x82\x85\x01aYvV[``\x83\x01RP\x92\x91PPV[_aZFaZA\x84aXrV[aI\xE2V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15aZiWaZhaEWV[[\x83[\x81\x81\x10\x15aZ\xB0W\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZ\x8EWaZ\x8DaEOV[[\x80\x86\x01aZ\x9B\x89\x82aY\xA3V[\x85R` \x85\x01\x94PPP` \x81\x01\x90PaZkV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aZ\xCEWaZ\xCDaEOV[[\x81QaZ\xDE\x84\x82` \x86\x01aZ4V[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15aZ\xFCWaZ\xFBaE\x14V[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a[\x19Wa[\x18aE\x18V[[a[%\x84\x82\x85\x01aZ\xBAV[\x91PP\x92\x91PPV[_a[8\x82aE\x1CV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a[jWa[iaP5V[[`\x01\x82\x01\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a[\xA7\x81aK\x02V[\x82RPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a[\xC7\x82aF\x9FV[a[\xD1\x81\x85a[\xADV[\x93Pa[\xDC\x83aF\xB9V[\x80_[\x83\x81\x10\x15a\\\x0CW\x81Qa[\xF3\x88\x82aG\x07V[\x97Pa[\xFE\x83aG\x1EV[\x92PP`\x01\x81\x01\x90Pa[\xDFV[P\x85\x93PPPP\x92\x91PPV[_`\x80\x83\x01_\x83\x01Qa\\._\x86\x01\x82a[\x9EV[P` \x83\x01Qa\\A` \x86\x01\x82aK\xB4V[P`@\x83\x01Qa\\T`@\x86\x01\x82a[\x9EV[P``\x83\x01Q\x84\x82\x03``\x86\x01Ra\\l\x82\x82a[\xBDV[\x91PP\x80\x91PP\x92\x91PPV[_a\\\x84\x83\x83a\\\x19V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\\\xA2\x82a[uV[a\\\xAC\x81\x85a[\x7FV[\x93P\x83` \x82\x02\x85\x01a\\\xBE\x85a[\x8FV[\x80_[\x85\x81\x10\x15a\\\xF9W\x84\x84\x03\x89R\x81Qa\\\xDA\x85\x82a\\yV[\x94Pa\\\xE5\x83a\\\x8CV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa\\\xC1V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra]#\x81\x86a\\\x98V[\x90P\x81\x81\x03` \x83\x01Ra]8\x81\x84\x86aP\xA5V[\x90P\x94\x93PPPPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12a]jWa]ia]BV[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a]\x8CWa]\x8Ba]FV[[` \x83\x01\x92P` \x82\x026\x03\x83\x13\x15a]\xA8Wa]\xA7a]JV[[P\x92P\x92\x90PV[_`\xFF\x82\x16\x90P\x91\x90PV[_a]\xD6a]\xD1a]\xCC\x84a]\xB0V[aS\x8AV[aE\x1CV[\x90P\x91\x90PV[a]\xE6\x81a]\xBCV[\x82RPPV[_`@\x82\x01\x90Pa]\xFF_\x83\x01\x85a]\xDDV[a^\x0C` \x83\x01\x84aKmV[\x93\x92PPPV[_`@\x82\x84\x03\x12\x15a^(Wa^'aX\x9DV[[a^2`@aI\xE2V[\x90P_a^A\x84\x82\x85\x01aE;V[_\x83\x01RP` a^T\x84\x82\x85\x01aE;V[` \x83\x01RP\x92\x91PPV[_`@\x82\x84\x03\x12\x15a^uWa^taE\x14V[[_a^\x82\x84\x82\x85\x01a^\x13V[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[_a^\xA2` \x84\x01\x84aIlV[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a^\xC1\x83\x85aF\xA9V[\x93Pa^\xCC\x82a^\x8BV[\x80_[\x85\x81\x10\x15a_\x04Wa^\xE1\x82\x84a^\x94V[a^\xEB\x88\x82aG\x07V[\x97Pa^\xF6\x83a^\xAAV[\x92PP`\x01\x81\x01\x90Pa^\xCFV[P\x85\x92PPP\x93\x92PPPV[_`@\x82\x01\x90Pa_$_\x83\x01\x86aK|V[\x81\x81\x03` \x83\x01Ra_7\x81\x84\x86a^\xB6V[\x90P\x94\x93PPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_a_e\x83\x83a[\x9EV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a_\x87\x82a_AV[a_\x91\x81\x85aW\xD8V[\x93Pa_\x9C\x83a_KV[\x80_[\x83\x81\x10\x15a_\xCCW\x81Qa_\xB3\x88\x82a_ZV[\x97Pa_\xBE\x83a_qV[\x92PP`\x01\x81\x01\x90Pa_\x9FV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra_\xF1\x81\x84a_}V[\x90P\x92\x91PPV[_\x81Q\x90P\x91\x90PV[a`\x0C\x82a_\xF9V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a`%Wa`$aI\x84V[[a`/\x82TaP\x05V[a`:\x82\x82\x85aT V[_` \x90P`\x1F\x83\x11`\x01\x81\x14a`kW_\x84\x15a`YW\x82\x87\x01Q\x90P[a`c\x85\x82aT\x8EV[\x86UPa`\xCAV[`\x1F\x19\x84\x16a`y\x86aS\x0CV[_[\x82\x81\x10\x15a`\xA0W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa`{V[\x86\x83\x10\x15a`\xBDW\x84\x89\x01Qa`\xB9`\x1F\x89\x16\x82aTrV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_`\x80\x82\x01\x90P\x81\x81\x03_\x83\x01Ra`\xEA\x81\x89a\\\x98V[\x90Pa`\xF9` \x83\x01\x88aK|V[\x81\x81\x03`@\x83\x01Raa\x0C\x81\x86\x88aP\xA5V[\x90P\x81\x81\x03``\x83\x01Raa!\x81\x84\x86aP\xA5V[\x90P\x97\x96PPPPPPPV[_`@\x82\x01\x90PaaA_\x83\x01\x85aK\x0BV[aaN` \x83\x01\x84aK|V[\x93\x92PPPV[_` \x82\x84\x03\x12\x15aajWaaiaE\x14V[[_aaw\x84\x82\x85\x01aIlV[\x91PP\x92\x91PPV[_\x81\x90P\x92\x91PPV[aa\x93\x81aK\x02V[\x82RPPV[_aa\xA4\x83\x83aa\x8AV[` \x83\x01\x90P\x92\x91PPV[_aa\xBA\x82a_AV[aa\xC4\x81\x85aa\x80V[\x93Paa\xCF\x83a_KV[\x80_[\x83\x81\x10\x15aa\xFFW\x81Qaa\xE6\x88\x82aa\x99V[\x97Paa\xF1\x83a_qV[\x92PP`\x01\x81\x01\x90Paa\xD2V[P\x85\x93PPPP\x92\x91PPV[_ab\x17\x82\x84aa\xB0V[\x91P\x81\x90P\x92\x91PPV[_\x81\x90P\x92\x91PPV[_ab6\x82a_\xF9V[ab@\x81\x85ab\"V[\x93PabP\x81\x85` \x86\x01aG\xC0V[\x80\x84\x01\x91PP\x92\x91PPV[_abg\x82\x84ab,V[\x91P\x81\x90P\x92\x91PPV[_`\xA0\x82\x01\x90Pab\x85_\x83\x01\x88aK\x0BV[ab\x92` \x83\x01\x87aK\x0BV[ab\x9F`@\x83\x01\x86aK\x0BV[ab\xAC``\x83\x01\x85aK\x0BV[ab\xB9`\x80\x83\x01\x84aK\x0BV[\x96\x95PPPPPPV[_`@\x82\x01\x90Pab\xD6_\x83\x01\x85aKmV[ab\xE3` \x83\x01\x84aK|V[\x93\x92PPPV[_` \x82\x84\x03\x12\x15ab\xFFWab\xFEaE\x14V[[_ac\x0C\x84\x82\x85\x01aX\xCFV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15acWWacVaE\x14V[[_acd\x84\x82\x85\x01aX\xBBV[\x91PP\x92\x91PPV[_`\x80\x82\x01\x90Pac\x80_\x83\x01\x87aK\x0BV[ac\x8D` \x83\x01\x86aK\x0BV[ac\x9A`@\x83\x01\x85aK\x0BV[ac\xA7``\x83\x01\x84aK\x0BV[\x95\x94PPPPPV[_ac\xBA\x82aE\x1CV[\x91Pac\xC5\x83aE\x1CV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15ac\xDDWac\xDCaP5V[[\x92\x91PPV[_`@\x82\x01\x90Pac\xF6_\x83\x01\x85aKmV[ad\x03` \x83\x01\x84aKmV[\x93\x92PPPV[_`\x80\x83\x01_\x83\x01Qad\x1F_\x86\x01\x82a[\x9EV[P` \x83\x01Qad2` \x86\x01\x82aK\xB4V[P`@\x83\x01QadE`@\x86\x01\x82a[\x9EV[P``\x83\x01Q\x84\x82\x03``\x86\x01Rad]\x82\x82a[\xBDV[\x91PP\x80\x91PP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Rad\x82\x81\x85ad\nV[\x90P\x81\x81\x03` \x83\x01Rad\x96\x81\x84ad\nV[\x90P\x93\x92PPPV[_a\xFF\xFF\x82\x16\x90P\x91\x90PV[_ad\xC6ad\xC1ad\xBC\x84ad\x9FV[aS\x8AV[aE\x1CV[\x90P\x91\x90PV[ad\xD6\x81ad\xACV[\x82RPPV[_`@\x82\x01\x90Pad\xEF_\x83\x01\x85ad\xCDV[ad\xFC` \x83\x01\x84aKmV[\x93\x92PPPV[_ae\r\x82aE\x1CV[\x91Pae\x18\x83aE\x1CV[\x92P\x82\x82\x02ae&\x81aE\x1CV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17ae=Wae<aP5V[[P\x92\x91PPV[`@\x82\x01_\x82\x01QaeX_\x85\x01\x82aK\xB4V[P` \x82\x01Qaek` \x85\x01\x82aK\xB4V[PPPPV[_``\x82\x01\x90Pae\x84_\x83\x01\x85aKmV[ae\x91` \x83\x01\x84aeDV[\x93\x92PPPV[_`@\x82\x01\x90Pae\xAB_\x83\x01\x85aK|V[\x81\x81\x03` \x83\x01Rae\xBD\x81\x84aG*V[\x90P\x93\x92PPPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Rae\xDF\x81\x84\x86aP\xA5V[\x90P\x93\x92PPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15af\x02Waf\x01aI\x84V[[af\x0B\x82aG\xE8V[\x90P` \x81\x01\x90P\x91\x90PV[_af*af%\x84ae\xE8V[aI\xE2V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15afFWafEaI\x80V[[afQ\x84\x82\x85aG\xC0V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12afmWaflaEOV[[\x81Qaf}\x84\x82` \x86\x01af\x18V[\x91PP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15af\x9BWaf\x9AaX\x9DV[[af\xA5`\x80aI\xE2V[\x90P_af\xB4\x84\x82\x85\x01aR\x96V[_\x83\x01RP` af\xC7\x84\x82\x85\x01aR\x96V[` \x83\x01RP`@\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15af\xEBWaf\xEAaX\xA1V[[af\xF7\x84\x82\x85\x01afYV[`@\x83\x01RP``\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ag\x1BWag\x1AaX\xA1V[[ag'\x84\x82\x85\x01afYV[``\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15agHWagGaE\x14V[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ageWagdaE\x18V[[agq\x84\x82\x85\x01af\x86V[\x91PP\x92\x91PPV[_`@\x82\x01\x90Pag\x8D_\x83\x01\x85aK|V[ag\x9A` \x83\x01\x84aK|V[\x93\x92PPPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[`\x1F\x82\x11\x15ag\xF4Wag\xC5\x81ag\xA1V[ag\xCE\x84aS\x1EV[\x81\x01` \x85\x10\x15ag\xDDW\x81\x90P[ag\xF1ag\xE9\x85aS\x1EV[\x83\x01\x82aS\xFEV[PP[PPPV[ah\x02\x82aG\xA6V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ah\x1BWah\x1AaI\x84V[[ah%\x82TaP\x05V[ah0\x82\x82\x85ag\xB3V[_` \x90P`\x1F\x83\x11`\x01\x81\x14ahaW_\x84\x15ahOW\x82\x87\x01Q\x90P[ahY\x85\x82aT\x8EV[\x86UPah\xC0V[`\x1F\x19\x84\x16aho\x86ag\xA1V[_[\x82\x81\x10\x15ah\x96W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PahqV[\x86\x83\x10\x15ah\xB3W\x84\x89\x01Qah\xAF`\x1F\x89\x16\x82aTrV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[ah\xD1\x81a]\xB0V[\x82RPPV[_` \x82\x01\x90Pah\xEA_\x83\x01\x84ah\xC8V[\x92\x91PPV[`T\x81\x10ai\x01Wai\0aO\x92V[[PV[_\x81\x90Pai\x11\x82ah\xF0V[\x91\x90PV[_ai \x82ai\x04V[\x90P\x91\x90PV[ai0\x81ai\x16V[\x82RPPV[_` \x82\x01\x90PaiI_\x83\x01\x84ai'V[\x92\x91PPV[_\x81\x90P\x92\x91PPV[aib\x81aF\xE7V[\x82RPPV[_ais\x83\x83aiYV[` \x83\x01\x90P\x92\x91PPV[_ai\x89\x82aF\x9FV[ai\x93\x81\x85aiOV[\x93Pai\x9E\x83aF\xB9V[\x80_[\x83\x81\x10\x15ai\xCEW\x81Qai\xB5\x88\x82aihV[\x97Pai\xC0\x83aG\x1EV[\x92PP`\x01\x81\x01\x90Pai\xA1V[P\x85\x93PPPP\x92\x91PPV[_ai\xE6\x82\x84ai\x7FV[\x91P\x81\x90P\x92\x91PPV[_`\xC0\x82\x01\x90Paj\x04_\x83\x01\x89aK\x0BV[aj\x11` \x83\x01\x88aK\x0BV[aj\x1E`@\x83\x01\x87aK\x0BV[aj+``\x83\x01\x86aKmV[aj8`\x80\x83\x01\x85aKmV[ajE`\xA0\x83\x01\x84aK\x0BV[\x97\x96PPPPPPPV[_`\xA0\x82\x01\x90Pajc_\x83\x01\x88aK\x0BV[ajp` \x83\x01\x87aK\x0BV[aj}`@\x83\x01\x86aK\x0BV[aj\x8A``\x83\x01\x85aKmV[aj\x97`\x80\x83\x01\x84aK|V[\x96\x95PPPPPPV[_`\x80\x82\x01\x90Paj\xB4_\x83\x01\x87aK\x0BV[aj\xC1` \x83\x01\x86ah\xC8V[aj\xCE`@\x83\x01\x85aK\x0BV[aj\xDB``\x83\x01\x84aK\x0BV[\x95\x94PPPPPV\xFEUserDecryptResponseVerification(bytes publicKey,bytes32[] ctHandles,bytes userDecryptedShare,bytes extraData)PublicDecryptVerification(bytes32[] ctHandles,bytes decryptedResult,bytes extraData)UserDecryptRequestVerification(bytes publicKey,address[] contractAddresses,uint256 startTimestamp,uint256 durationDays,bytes extraData)",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405260043610610108575f3560e01c80635c975abb11610094578063ad3cb1cc11610063578063ad3cb1cc1461031c578063c411587414610346578063d8998f451461035c578063f1b57adb14610384578063fbb83259146103ac57610108565b80635c975abb146102845780636f8913bc146102ae5780638456cb59146102d657806384b0196e146102ec57610108565b80633f4ba83a116100db5780633f4ba83a146101b05780634014c4cd146101c65780634f1ef2861461020257806352d1902d1461021e57806358f5b8ab1461024857610108565b8063046f9eb31461010c5780630900cc69146101345780630d8e6e2c1461017057806339f738101461019a575b5f80fd5b348015610117575f80fd5b50610132600480360381019061012d91906145b0565b6103e8565b005b34801561013f575f80fd5b5061015a60048036038101906101559190614674565b610874565b6040516101679190614786565b60405180910390f35b34801561017b575f80fd5b50610184610945565b6040516101919190614830565b60405180910390f35b3480156101a5575f80fd5b506101ae6109c0565b005b3480156101bb575f80fd5b506101c4610bf8565b005b3480156101d1575f80fd5b506101ec60048036038101906101e791906148a5565b610d40565b6040516101f9919061493d565b60405180910390f35b61021c60048036038101906102179190614aa8565b610ecd565b005b348015610229575f80fd5b50610232610eec565b60405161023f9190614b1a565b60405180910390f35b348015610253575f80fd5b5061026e60048036038101906102699190614674565b610f1d565b60405161027b919061493d565b60405180910390f35b34801561028f575f80fd5b50610298610f50565b6040516102a5919061493d565b60405180910390f35b3480156102b9575f80fd5b506102d460048036038101906102cf91906145b0565b610f72565b005b3480156102e1575f80fd5b506102ea6113bd565b005b3480156102f7575f80fd5b506103006114e2565b6040516103139796959493929190614c42565b60405180910390f35b348015610327575f80fd5b506103306115eb565b60405161033d9190614830565b60405180910390f35b348015610351575f80fd5b5061035a611624565b005b348015610367575f80fd5b50610382600480360381019061037d91906148a5565b611749565b005b34801561038f575f80fd5b506103aa60048036038101906103a59190614d59565b611910565b005b3480156103b7575f80fd5b506103d260048036038101906103cd9190614e93565b611dca565b6040516103df919061493d565b60405180910390f35b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663e5275eaf336040518263ffffffff1660e01b81526004016104359190614f24565b602060405180830381865afa158015610450573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906104749190614f67565b6104b557336040517faee863230000000000000000000000000000000000000000000000000000000081526004016104ac9190614f24565b60405180910390fd5b5f6104be612039565b905060f8600260068111156104d6576104d5614f92565b5b901b881115806104e95750806008015488115b1561052b57876040517fd48af9420000000000000000000000000000000000000000000000000000000081526004016105229190614fbf565b60405180910390fd5b5f816007015f8a81526020019081526020015f206040518060400160405290815f8201805461055990615005565b80601f016020809104026020016040519081016040528092919081815260200182805461058590615005565b80156105d05780601f106105a7576101008083540402835291602001916105d0565b820191905f5260205f20905b8154815290600101906020018083116105b357829003601f168201915b505050505081526020016001820180548060200260200160405190810160405280929190818152602001828054801561062657602002820191905f5260205f20905b815481526020019060010190808311610612575b50505050508152505090505f6040518060800160405280835f01518152602001836020015181526020018a8a8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050815260200186868080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081525090505f6106ec82612060565b90506106fa8b828a8a612127565b5f846002015f8d81526020019081526020015f205f805f1b81526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508b7f7fcdfb5381917f554a717d0a5470a33f5a49ba6445f05ec43c74c0bc2cc608b2600183805490506107b39190615062565b8d8d8d8d8d8d6040516107cc97969594939291906150d1565b60405180910390a2845f015f8d81526020019081526020015f205f9054906101000a900460ff1615801561080957506108088180549050612299565b5b15610866576001855f015f8e81526020019081526020015f205f6101000a81548160ff0219169083151502179055508b7fe89752be0ecdb68b2a6eb5ef1a891039e0e92ae3c8a62274c5881e48eea1ed2560405160405180910390a25b505050505050505050505050565b60605f61087f612039565b90505f816003015f8581526020019081526020015f20549050816002015f8581526020019081526020015f205f8281526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561093757602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116108ee575b505050505092505050919050565b60606040518060400160405280600a81526020017f44656372797074696f6e000000000000000000000000000000000000000000008152506109865f61232a565b610990600261232a565b6109995f61232a565b6040516020016109ac94939291906151fd565b604051602081830303815290604052905090565b60016109ca6123f4565b67ffffffffffffffff1614610a0b576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60035f610a16612418565b9050805f0160089054906101000a900460ff1680610a5e57508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15610a95576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff021916908315150217905550610b4e6040518060400160405280600a81526020017f44656372797074696f6e000000000000000000000000000000000000000000008152506040518060400160405280600181526020017f310000000000000000000000000000000000000000000000000000000000000081525061243f565b610b56612455565b5f610b5f612039565b905060f860016006811115610b7757610b76614f92565b5b901b816006018190555060f860026006811115610b9757610b96614f92565b5b901b8160080181905550505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051610bec919061527d565b60405180910390a15050565b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610c55573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c7991906152aa565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614158015610cf4575073d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614155b15610d3657336040517fe19166ee000000000000000000000000000000000000000000000000000000008152600401610d2d9190614f24565b60405180910390fd5b610d3e612467565b565b5f805f90505b85859050811015610ebf5773c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff16630620326d878784818110610d9457610d936152d5565b5b905060200201356040518263ffffffff1660e01b8152600401610db79190614b1a565b602060405180830381865afa158015610dd2573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610df69190614f67565b1580610ea4575073de409109e0fccaae7b87de518f61d617a3fda09473ffffffffffffffffffffffffffffffffffffffff16632ddc9a6f878784818110610e4057610e3f6152d5565b5b905060200201356040518263ffffffff1660e01b8152600401610e639190614b1a565b602060405180830381865afa158015610e7e573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ea29190614f67565b155b15610eb2575f915050610ec5565b8080600101915050610d46565b50600190505b949350505050565b610ed56124d5565b610ede826125bb565b610ee882826126ae565b5050565b5f610ef56127cc565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b5f80610f27612039565b9050805f015f8481526020019081526020015f205f9054906101000a900460ff16915050919050565b5f80610f5a612853565b9050805f015f9054906101000a900460ff1691505090565b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663e5275eaf336040518263ffffffff1660e01b8152600401610fbf9190614f24565b602060405180830381865afa158015610fda573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ffe9190614f67565b61103f57336040517faee863230000000000000000000000000000000000000000000000000000000081526004016110369190614f24565b60405180910390fd5b5f611048612039565b905060f8600160068111156110605761105f614f92565b5b901b881115806110735750806006015488115b156110b557876040517fd48af9420000000000000000000000000000000000000000000000000000000081526004016110ac9190614fbf565b60405180910390fd5b5f6040518060600160405280836005015f8c81526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561111c57602002820191905f5260205f20905b815481526020019060010190808311611108575b5050505050815260200189898080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050815260200185858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081525090505f6111c28261287a565b90506111d08a828989612127565b5f836004015f8c81526020019081526020015f205f8381526020019081526020015f20905080888890918060018154018082558091505060019003905f5260205f20015f90919290919290919290919250918261122e9291906154a9565b50836002015f8c81526020019081526020015f205f8381526020019081526020015f2033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508a7f4d7b1dba49e9e846215e1621f5737c81d8614c4f268494d8b787632c4e59f0e58b8b8b8b338c8c6040516112eb9796959493929190615576565b60405180910390a2835f015f8c81526020019081526020015f205f9054906101000a900460ff1615801561132857506113278180549050612934565b5b156113b0576001845f015f8d81526020019081526020015f205f6101000a81548160ff02191690831515021790555081846003015f8d81526020019081526020015f20819055508a7fd7e58a367a0a6c298e76ad5d240004e327aa1423cbe4bd7ff85d4c715ef8d15f8b8b848a8a6040516113a7959493929190615722565b60405180910390a25b5050505050505050505050565b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff166346fbf68e336040518263ffffffff1660e01b815260040161140a9190614f24565b602060405180830381865afa158015611425573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114499190614f67565b158015611496575073d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614155b156114d857336040517f388916bb0000000000000000000000000000000000000000000000000000000081526004016114cf9190614f24565b60405180910390fd5b6114e06129c5565b565b5f6060805f805f60605f6114f4612a34565b90505f801b815f015414801561150f57505f801b8160010154145b61154e576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611545906157ba565b60405180910390fd5b611556612a5b565b61155e612af9565b46305f801b5f67ffffffffffffffff81111561157d5761157c614984565b5b6040519080825280602002602001820160405280156115ab5781602001602082028036833780820191505090505b507f0f0000000000000000000000000000000000000000000000000000000000000095949392919097509750975097509750975097505090919293949596565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b60035f61162f612418565b9050805f0160089054906101000a900460ff168061167757508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b156116ae576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d28260405161173d919061527d565b60405180910390a15050565b611751612b97565b5f848490500361178d576040517f2de7543800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6117d68484808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f82011690508083019250505050505050612bd8565b5f73de409109e0fccaae7b87de518f61d617a3fda09473ffffffffffffffffffffffffffffffffffffffff1663a14f897186866040518363ffffffff1660e01b8152600401611826929190615850565b5f60405180830381865afa158015611840573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f820116820180604052508101906118689190615ae7565b905061187381612c90565b5f61187c612039565b9050806006015f81548092919061189290615b2e565b91905055505f816006015490508686836005015f8481526020019081526020015f2091906118c192919061445a565b506118cb33612d76565b807f22db480a39bd72556438aadb4a32a3d2a6638b87c03bbec5fef6997e109587ff8487876040516118ff93929190615d0b565b60405180910390a250505050505050565b611918612b97565b5f8880602001906119299190615d4e565b905003611962576040517f57cfa21700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600a60ff168880602001906119779190615d4e565b905011156119d057600a8880602001906119919190615d4e565b90506040517faf1f04950000000000000000000000000000000000000000000000000000000081526004016119c7929190615dec565b60405180910390fd5b6119e9898036038101906119e49190615e60565b612df3565b611a418880602001906119fc9190615d4e565b808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f8201169050808301925050505050505088612f3e565b15611a955786888060200190611a579190615d4e565b6040517fdc4d78b1000000000000000000000000000000000000000000000000000000008152600401611a8c93929190615f11565b60405180910390fd5b5f611af08c8c8b8060200190611aab9190615d4e565b808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f820116905080830192505050505050508b612fbc565b90505f6040518060a0016040528089898080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081526020018b8060200190611b579190615d4e565b808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f8201169050808301925050505050505081526020018c5f013581526020018c60200135815260200185858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050508152509050611c07818a88888e5f01356131af565b5f73de409109e0fccaae7b87de518f61d617a3fda09473ffffffffffffffffffffffffffffffffffffffff1663a14f8971846040518263ffffffff1660e01b8152600401611c559190615fd9565b5f60405180830381865afa158015611c6f573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f82011682018060405250810190611c979190615ae7565b9050611ca281612c90565b5f611cab612039565b9050806008015f815480929190611cc190615b2e565b91905055505f8160080154905060405180604001604052808c8c8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050815260200186815250826007015f8381526020019081526020015f205f820151815f019081611d4c9190616003565b506020820151816001019080519060200190611d699291906144a5565b50905050611d7633613287565b807ff9011bd6ba0da6049c520d70fe5971f17ed7ab795486052544b51019896c596b848e8e8e8c8c604051611db0969594939291906160d2565b60405180910390a250505050505050505050505050505050565b5f805f90505b8585905081101561202a5773c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff1663c6528f69878784818110611e1e57611e1d6152d5565b5b9050604002015f0135896040518363ffffffff1660e01b8152600401611e4592919061612e565b602060405180830381865afa158015611e60573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e849190614f67565b1580611f60575073c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff1663c6528f69878784818110611ece57611ecd6152d5565b5b9050604002015f0135888885818110611eea57611ee96152d5565b5b9050604002016020016020810190611f029190616155565b6040518363ffffffff1660e01b8152600401611f1f92919061612e565b602060405180830381865afa158015611f3a573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f5e9190614f67565b155b8061200f575073de409109e0fccaae7b87de518f61d617a3fda09473ffffffffffffffffffffffffffffffffffffffff16632ddc9a6f878784818110611fa957611fa86152d5565b5b9050604002015f01356040518263ffffffff1660e01b8152600401611fce9190614b1a565b602060405180830381865afa158015611fe9573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061200d9190614f67565b155b1561201d575f915050612030565b8080600101915050611dd0565b50600190505b95945050505050565b5f7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee700905090565b5f6121206040518060a00160405280606d8152602001616ae5606d913980519060200120835f01518051906020012084602001516040516020016120a4919061620c565b6040516020818303038152906040528051906020012085604001518051906020012086606001516040516020016120db919061625c565b60405160208183030381529060405280519060200120604051602001612105959493929190616272565b60405160208183030381529060405280519060200120613304565b9050919050565b5f612130612039565b90505f6121808585858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505061331d565b905061218c8133613347565b816001015f8781526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff161561222b5785816040517f99ec48d90000000000000000000000000000000000000000000000000000000081526004016122229291906162c3565b60405180910390fd5b6001826001015f8881526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550505050505050565b5f8073d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663c2b429866040518163ffffffff1660e01b8152600401602060405180830381865afa1580156122f8573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061231c91906162ea565b905080831015915050919050565b60605f600161233884613458565b0190505f8167ffffffffffffffff81111561235657612355614984565b5b6040519080825280601f01601f1916602001820160405280156123885781602001600182028036833780820191505090505b5090505f82602001820190505b6001156123e9578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a85816123de576123dd616315565b5b0494505f8503612395575b819350505050919050565b5f6123fd612418565b5f015f9054906101000a900467ffffffffffffffff16905090565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b6124476135a9565b61245182826135e9565b5050565b61245d6135a9565b61246561363a565b565b61246f61366a565b5f612478612853565b90505f815f015f6101000a81548160ff0219169083151502179055507f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa6124bd6136aa565b6040516124ca9190614f24565b60405180910390a150565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff16148061258257507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166125696136b1565b73ffffffffffffffffffffffffffffffffffffffff1614155b156125b9576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612618573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061263c91906152aa565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146126ab57336040517f0e56cf3d0000000000000000000000000000000000000000000000000000000081526004016126a29190614f24565b60405180910390fd5b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa92505050801561271657506040513d601f19601f820116820180604052508101906127139190616342565b60015b61275757816040517f4c9c8ce300000000000000000000000000000000000000000000000000000000815260040161274e9190614f24565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b81146127bd57806040517faa1d49a40000000000000000000000000000000000000000000000000000000081526004016127b49190614b1a565b60405180910390fd5b6127c78383613704565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff1614612851576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300905090565b5f61292d604051806080016040528060548152602001616b526054913980519060200120835f01516040516020016128b2919061620c565b6040516020818303038152906040528051906020012084602001518051906020012085604001516040516020016128e9919061625c565b60405160208183030381529060405280519060200120604051602001612912949392919061636d565b60405160208183030381529060405280519060200120613304565b9050919050565b5f8073d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff16632a3889986040518163ffffffff1660e01b8152600401602060405180830381865afa158015612993573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129b791906162ea565b905080831015915050919050565b6129cd612b97565b5f6129d6612853565b90506001815f015f6101000a81548160ff0219169083151502179055507f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a258612a1c6136aa565b604051612a299190614f24565b60405180910390a150565b5f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100905090565b60605f612a66612a34565b9050806002018054612a7790615005565b80601f0160208091040260200160405190810160405280929190818152602001828054612aa390615005565b8015612aee5780601f10612ac557610100808354040283529160200191612aee565b820191905f5260205f20905b815481529060010190602001808311612ad157829003601f168201915b505050505091505090565b60605f612b04612a34565b9050806003018054612b1590615005565b80601f0160208091040260200160405190810160405280929190818152602001828054612b4190615005565b8015612b8c5780601f10612b6357610100808354040283529160200191612b8c565b820191905f5260205f20905b815481529060010190602001808311612b6f57829003601f168201915b505050505091505090565b612b9f610f50565b15612bd6576040517fd93c066500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f805b8251811015612c40575f838281518110612bf857612bf76152d5565b5b602002602001015190505f612c0c82613776565b9050612c1781613800565b61ffff1684612c2691906163b0565b9350612c31826139eb565b50508080600101915050612bdb565b50610800811115612c8c57610800816040517fe7f4895d000000000000000000000000000000000000000000000000000000008152600401612c839291906163e3565b60405180910390fd5b5050565b600181511115612d73575f815f81518110612cae57612cad6152d5565b5b60200260200101516020015190505f600190505b8251811015612d705781838281518110612cdf57612cde6152d5565b5b60200260200101516020015114612d6357825f81518110612d0357612d026152d5565b5b6020026020010151838281518110612d1e57612d1d6152d5565b5b60200260200101516040517fcfae921f000000000000000000000000000000000000000000000000000000008152600401612d5a92919061646a565b60405180910390fd5b8080600101915050612cc2565b50505b50565b738733d4013efc4256977150f31a8ea1e9e4c1458873ffffffffffffffffffffffffffffffffffffffff166391eeb27c826040518263ffffffff1660e01b8152600401612dc39190614f24565b5f604051808303815f87803b158015612dda575f80fd5b505af1158015612dec573d5f803e3d5ffd5b5050505050565b5f816020015103612e30576040517fde2859c100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61016d61ffff1681602001511115612e875761016d81602001516040517f32951863000000000000000000000000000000000000000000000000000000008152600401612e7e9291906164dc565b60405180910390fd5b42815f01511115612ed45742815f01516040517ff24c0887000000000000000000000000000000000000000000000000000000008152600401612ecb9291906163e3565b60405180910390fd5b42620151808260200151612ee89190616503565b825f0151612ef691906163b0565b1015612f3b5742816040517f30348040000000000000000000000000000000000000000000000000000000008152600401612f32929190616571565b60405180910390fd5b50565b5f805f90505b8351811015612fb1578273ffffffffffffffffffffffffffffffffffffffff16848281518110612f7757612f766152d5565b5b602002602001015173ffffffffffffffffffffffffffffffffffffffff1603612fa4576001915050612fb6565b8080600101915050612f44565b505f90505b92915050565b60605f8585905003612ffa576040517fa6a6cb2100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8484905067ffffffffffffffff81111561301757613016614984565b5b6040519080825280602002602001820160405280156130455781602001602082028036833780820191505090505b5090505f805b8686905081101561315a575f87878381811061306a576130696152d5565b5b9050604002015f013590505f888884818110613089576130886152d5565b5b90506040020160200160208101906130a19190616155565b90505f6130ad83613776565b90506130b881613800565b61ffff16856130c791906163b0565b94506130d38388613abb565b6130dd8383613abb565b6130e78883612f3e565b61312a5781886040517fa4c30391000000000000000000000000000000000000000000000000000000008152600401613121929190616598565b60405180910390fd5b8286858151811061313e5761313d6152d5565b5b602002602001018181525050505050808060010191505061304b565b506108008111156131a657610800816040517fe7f4895d00000000000000000000000000000000000000000000000000000000815260040161319d9291906163e3565b60405180910390fd5b50949350505050565b5f6131ba8683613b90565b90505f61320a8286868080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505061331d565b90508573ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff161461327e5784846040517f2a873d270000000000000000000000000000000000000000000000000000000081526004016132759291906165c6565b60405180910390fd5b50505050505050565b738733d4013efc4256977150f31a8ea1e9e4c1458873ffffffffffffffffffffffffffffffffffffffff1663988a2d2d826040518263ffffffff1660e01b81526004016132d49190614f24565b5f604051808303815f87803b1580156132eb575f80fd5b505af11580156132fd573d5f803e3d5ffd5b5050505050565b5f613316613310613c5d565b83613c6b565b9050919050565b5f805f8061332b8686613cab565b92509250925061333b8282613d00565b82935050505092915050565b61335082613e62565b8173ffffffffffffffffffffffffffffffffffffffff1673d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663e3b2a874836040518263ffffffff1660e01b81526004016133b49190614f24565b5f60405180830381865afa1580156133ce573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f820116820180604052508101906133f69190616733565b6020015173ffffffffffffffffffffffffffffffffffffffff16146134545781816040517f0d86f52100000000000000000000000000000000000000000000000000000000815260040161344b92919061677a565b60405180910390fd5b5050565b5f805f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f01000000000000000083106134b4577a184f03e93ff9f4daa797ed6e38ed64bf6a1f01000000000000000083816134aa576134a9616315565b5b0492506040810190505b6d04ee2d6d415b85acef810000000083106134f1576d04ee2d6d415b85acef810000000083816134e7576134e6616315565b5b0492506020810190505b662386f26fc10000831061352057662386f26fc10000838161351657613515616315565b5b0492506010810190505b6305f5e1008310613549576305f5e100838161353f5761353e616315565b5b0492506008810190505b612710831061356e57612710838161356457613563616315565b5b0492506004810190505b60648310613591576064838161358757613586616315565b5b0492506002810190505b600a83106135a0576001810190505b80915050919050565b6135b1613f32565b6135e7576040517fd7e6bcf800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b6135f16135a9565b5f6135fa612a34565b90508281600201908161360d91906167f9565b508181600301908161361f91906167f9565b505f801b815f01819055505f801b8160010181905550505050565b6136426135a9565b5f61364b612853565b90505f815f015f6101000a81548160ff02191690831515021790555050565b613672610f50565b6136a8576040517f8dfc202b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f33905090565b5f6136dd7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b613f50565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b61370d82613f59565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f81511115613769576137638282614022565b50613772565b6137716140a2565b5b5050565b5f8060f860f084901b901c5f1c905060538081111561379857613797614f92565b5b60ff168160ff1611156137e257806040517f641950d70000000000000000000000000000000000000000000000000000000081526004016137d991906168d7565b60405180910390fd5b8060ff1660538111156137f8576137f7614f92565b5b915050919050565b5f80605381111561381457613813614f92565b5b82605381111561382757613826614f92565b5b0361383557600290506139e6565b6002605381111561384957613848614f92565b5b82605381111561385c5761385b614f92565b5b0361386a57600890506139e6565b6003605381111561387e5761387d614f92565b5b82605381111561389157613890614f92565b5b0361389f57601090506139e6565b600460538111156138b3576138b2614f92565b5b8260538111156138c6576138c5614f92565b5b036138d457602090506139e6565b600560538111156138e8576138e7614f92565b5b8260538111156138fb576138fa614f92565b5b0361390957604090506139e6565b6006605381111561391d5761391c614f92565b5b8260538111156139305761392f614f92565b5b0361393e57608090506139e6565b6007605381111561395257613951614f92565b5b82605381111561396557613964614f92565b5b036139735760a090506139e6565b6008605381111561398757613986614f92565b5b82605381111561399a57613999614f92565b5b036139a95761010090506139e6565b816040517fbe7830b10000000000000000000000000000000000000000000000000000000081526004016139dd9190616936565b60405180910390fd5b919050565b73c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff16630620326d826040518263ffffffff1660e01b8152600401613a389190614b1a565b602060405180830381865afa158015613a53573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613a779190614f67565b613ab857806040517f4331a85d000000000000000000000000000000000000000000000000000000008152600401613aaf9190614b1a565b60405180910390fd5b50565b73c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff1663c6528f6983836040518363ffffffff1660e01b8152600401613b0a92919061612e565b602060405180830381865afa158015613b25573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613b499190614f67565b613b8c5781816040517f160a2b4b000000000000000000000000000000000000000000000000000000008152600401613b8392919061612e565b60405180910390fd5b5050565b5f806040518060c0016040528060878152602001616ba66087913980519060200120845f0151805190602001208560200151604051602001613bd291906169db565b60405160208183030381529060405280519060200120866040015187606001518860800151604051602001613c07919061625c565b60405160208183030381529060405280519060200120604051602001613c32969594939291906169f1565b604051602081830303815290604052805190602001209050613c5483826140de565b91505092915050565b5f613c66614152565b905090565b5f6040517f190100000000000000000000000000000000000000000000000000000000000081528360028201528260228201526042812091505092915050565b5f805f6041845103613ceb575f805f602087015192506040870151915060608701515f1a9050613cdd888285856141b5565b955095509550505050613cf9565b5f600285515f1b9250925092505b9250925092565b5f6003811115613d1357613d12614f92565b5b826003811115613d2657613d25614f92565b5b0315613e5e5760016003811115613d4057613d3f614f92565b5b826003811115613d5357613d52614f92565b5b03613d8a576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60026003811115613d9e57613d9d614f92565b5b826003811115613db157613db0614f92565b5b03613df557805f1c6040517ffce698f7000000000000000000000000000000000000000000000000000000008152600401613dec9190614fbf565b60405180910390fd5b600380811115613e0857613e07614f92565b5b826003811115613e1b57613e1a614f92565b5b03613e5d57806040517fd78bce0c000000000000000000000000000000000000000000000000000000008152600401613e549190614b1a565b60405180910390fd5b5b5050565b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663203d0114826040518263ffffffff1660e01b8152600401613eaf9190614f24565b602060405180830381865afa158015613eca573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613eee9190614f67565b613f2f57806040517f2a7c6ef6000000000000000000000000000000000000000000000000000000008152600401613f269190614f24565b60405180910390fd5b50565b5f613f3b612418565b5f0160089054906101000a900460ff16905090565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b03613fb457806040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401613fab9190614f24565b60405180910390fd5b80613fe07f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b613f50565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f808473ffffffffffffffffffffffffffffffffffffffff168460405161404b919061625c565b5f60405180830381855af49150503d805f8114614083576040519150601f19603f3d011682016040523d82523d5f602084013e614088565b606091505b509150915061409885838361429c565b9250505092915050565b5f3411156140dc576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f807f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f614109614329565b61411161439f565b8630604051602001614127959493929190616a50565b6040516020818303038152906040528051906020012090506141498184613c6b565b91505092915050565b5f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f61417c614329565b61418461439f565b463060405160200161419a959493929190616a50565b60405160208183030381529060405280519060200120905090565b5f805f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c11156141f1575f600385925092509250614292565b5f6001888888886040515f81526020016040526040516142149493929190616aa1565b6020604051602081039080840390855afa158015614234573d5f803e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603614285575f60015f801b93509350935050614292565b805f805f1b935093509350505b9450945094915050565b6060826142b1576142ac82614416565b614321565b5f82511480156142d757505f8473ffffffffffffffffffffffffffffffffffffffff163b145b1561431957836040517f9996b3150000000000000000000000000000000000000000000000000000000081526004016143109190614f24565b60405180910390fd5b819050614322565b5b9392505050565b5f80614333612a34565b90505f61433e612a5b565b90505f8151111561435a5780805190602001209250505061439c565b5f825f015490505f801b81146143755780935050505061439c565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f806143a9612a34565b90505f6143b4612af9565b90505f815111156143d057808051906020012092505050614413565b5f826001015490505f801b81146143ec57809350505050614413565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f815111156144285780518082602001fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b828054828255905f5260205f20908101928215614494579160200282015b82811115614493578235825591602001919060010190614478565b5b5090506144a191906144f0565b5090565b828054828255905f5260205f209081019282156144df579160200282015b828111156144de5782518255916020019190600101906144c3565b5b5090506144ec91906144f0565b5090565b5b80821115614507575f815f9055506001016144f1565b5090565b5f604051905090565b5f80fd5b5f80fd5b5f819050919050565b61452e8161451c565b8114614538575f80fd5b50565b5f8135905061454981614525565b92915050565b5f80fd5b5f80fd5b5f80fd5b5f8083601f8401126145705761456f61454f565b5b8235905067ffffffffffffffff81111561458d5761458c614553565b5b6020830191508360018202830111156145a9576145a8614557565b5b9250929050565b5f805f805f805f6080888a0312156145cb576145ca614514565b5b5f6145d88a828b0161453b565b975050602088013567ffffffffffffffff8111156145f9576145f8614518565b5b6146058a828b0161455b565b9650965050604088013567ffffffffffffffff81111561462857614627614518565b5b6146348a828b0161455b565b9450945050606088013567ffffffffffffffff81111561465757614656614518565b5b6146638a828b0161455b565b925092505092959891949750929550565b5f6020828403121561468957614688614514565b5b5f6146968482850161453b565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6146f1826146c8565b9050919050565b614701816146e7565b82525050565b5f61471283836146f8565b60208301905092915050565b5f602082019050919050565b5f6147348261469f565b61473e81856146a9565b9350614749836146b9565b805f5b838110156147795781516147608882614707565b975061476b8361471e565b92505060018101905061474c565b5085935050505092915050565b5f6020820190508181035f83015261479e818461472a565b905092915050565b5f81519050919050565b5f82825260208201905092915050565b5f5b838110156147dd5780820151818401526020810190506147c2565b5f8484015250505050565b5f601f19601f8301169050919050565b5f614802826147a6565b61480c81856147b0565b935061481c8185602086016147c0565b614825816147e8565b840191505092915050565b5f6020820190508181035f83015261484881846147f8565b905092915050565b5f8083601f8401126148655761486461454f565b5b8235905067ffffffffffffffff81111561488257614881614553565b5b60208301915083602082028301111561489e5761489d614557565b5b9250929050565b5f805f80604085870312156148bd576148bc614514565b5b5f85013567ffffffffffffffff8111156148da576148d9614518565b5b6148e687828801614850565b9450945050602085013567ffffffffffffffff81111561490957614908614518565b5b6149158782880161455b565b925092505092959194509250565b5f8115159050919050565b61493781614923565b82525050565b5f6020820190506149505f83018461492e565b92915050565b61495f816146e7565b8114614969575f80fd5b50565b5f8135905061497a81614956565b92915050565b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6149ba826147e8565b810181811067ffffffffffffffff821117156149d9576149d8614984565b5b80604052505050565b5f6149eb61450b565b90506149f782826149b1565b919050565b5f67ffffffffffffffff821115614a1657614a15614984565b5b614a1f826147e8565b9050602081019050919050565b828183375f83830152505050565b5f614a4c614a47846149fc565b6149e2565b905082815260208101848484011115614a6857614a67614980565b5b614a73848285614a2c565b509392505050565b5f82601f830112614a8f57614a8e61454f565b5b8135614a9f848260208601614a3a565b91505092915050565b5f8060408385031215614abe57614abd614514565b5b5f614acb8582860161496c565b925050602083013567ffffffffffffffff811115614aec57614aeb614518565b5b614af885828601614a7b565b9150509250929050565b5f819050919050565b614b1481614b02565b82525050565b5f602082019050614b2d5f830184614b0b565b92915050565b5f7fff0000000000000000000000000000000000000000000000000000000000000082169050919050565b614b6781614b33565b82525050565b614b768161451c565b82525050565b614b85816146e7565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b614bbd8161451c565b82525050565b5f614bce8383614bb4565b60208301905092915050565b5f602082019050919050565b5f614bf082614b8b565b614bfa8185614b95565b9350614c0583614ba5565b805f5b83811015614c35578151614c1c8882614bc3565b9750614c2783614bda565b925050600181019050614c08565b5085935050505092915050565b5f60e082019050614c555f83018a614b5e565b8181036020830152614c6781896147f8565b90508181036040830152614c7b81886147f8565b9050614c8a6060830187614b6d565b614c976080830186614b7c565b614ca460a0830185614b0b565b81810360c0830152614cb68184614be6565b905098975050505050505050565b5f8083601f840112614cd957614cd861454f565b5b8235905067ffffffffffffffff811115614cf657614cf5614553565b5b602083019150836040820283011115614d1257614d11614557565b5b9250929050565b5f80fd5b5f60408284031215614d3257614d31614d19565b5b81905092915050565b5f60408284031215614d5057614d4f614d19565b5b81905092915050565b5f805f805f805f805f805f6101008c8e031215614d7957614d78614514565b5b5f8c013567ffffffffffffffff811115614d9657614d95614518565b5b614da28e828f01614cc4565b9b509b50506020614db58e828f01614d1d565b99505060608c013567ffffffffffffffff811115614dd657614dd5614518565b5b614de28e828f01614d3b565b9850506080614df38e828f0161496c565b97505060a08c013567ffffffffffffffff811115614e1457614e13614518565b5b614e208e828f0161455b565b965096505060c08c013567ffffffffffffffff811115614e4357614e42614518565b5b614e4f8e828f0161455b565b945094505060e08c013567ffffffffffffffff811115614e7257614e71614518565b5b614e7e8e828f0161455b565b92509250509295989b509295989b9093969950565b5f805f805f60608688031215614eac57614eab614514565b5b5f614eb98882890161496c565b955050602086013567ffffffffffffffff811115614eda57614ed9614518565b5b614ee688828901614cc4565b9450945050604086013567ffffffffffffffff811115614f0957614f08614518565b5b614f158882890161455b565b92509250509295509295909350565b5f602082019050614f375f830184614b7c565b92915050565b614f4681614923565b8114614f50575f80fd5b50565b5f81519050614f6181614f3d565b92915050565b5f60208284031215614f7c57614f7b614514565b5b5f614f8984828501614f53565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b5f602082019050614fd25f830184614b6d565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f600282049050600182168061501c57607f821691505b60208210810361502f5761502e614fd8565b5b50919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61506c8261451c565b91506150778361451c565b925082820390508181111561508f5761508e615035565b5b92915050565b5f82825260208201905092915050565b5f6150b08385615095565b93506150bd838584614a2c565b6150c6836147e8565b840190509392505050565b5f6080820190506150e45f83018a614b6d565b81810360208301526150f781888a6150a5565b9050818103604083015261510c8186886150a5565b905081810360608301526151218184866150a5565b905098975050505050505050565b5f81905092915050565b5f615143826147a6565b61514d818561512f565b935061515d8185602086016147c0565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f61519d60028361512f565b91506151a882615169565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f6151e760018361512f565b91506151f2826151b3565b600182019050919050565b5f6152088287615139565b915061521382615191565b915061521f8286615139565b915061522a826151db565b91506152368285615139565b9150615241826151db565b915061524d8284615139565b915081905095945050505050565b5f67ffffffffffffffff82169050919050565b6152778161525b565b82525050565b5f6020820190506152905f83018461526e565b92915050565b5f815190506152a481614956565b92915050565b5f602082840312156152bf576152be614514565b5b5f6152cc84828501615296565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f82905092915050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f600883026153687fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8261532d565b615372868361532d565b95508019841693508086168417925050509392505050565b5f819050919050565b5f6153ad6153a86153a38461451c565b61538a565b61451c565b9050919050565b5f819050919050565b6153c683615393565b6153da6153d2826153b4565b848454615339565b825550505050565b5f90565b6153ee6153e2565b6153f98184846153bd565b505050565b5b8181101561541c576154115f826153e6565b6001810190506153ff565b5050565b601f821115615461576154328161530c565b61543b8461531e565b8101602085101561544a578190505b61545e6154568561531e565b8301826153fe565b50505b505050565b5f82821c905092915050565b5f6154815f1984600802615466565b1980831691505092915050565b5f6154998383615472565b9150826002028217905092915050565b6154b38383615302565b67ffffffffffffffff8111156154cc576154cb614984565b5b6154d68254615005565b6154e1828285615420565b5f601f83116001811461550e575f84156154fc578287013590505b615506858261548e565b86555061556d565b601f19841661551c8661530c565b5f5b828110156155435784890135825560018201915060208501945060208101905061551e565b86831015615560578489013561555c601f891682615472565b8355505b6001600288020188555050505b50505050505050565b5f6080820190508181035f83015261558f81898b6150a5565b905081810360208301526155a48187896150a5565b90506155b36040830186614b7c565b81810360608301526155c68184866150a5565b905098975050505050505050565b5f81549050919050565b5f82825260208201905092915050565b5f819050815f5260205f209050919050565b5f82825260208201905092915050565b5f815461561c81615005565b6156268186615600565b9450600182165f8114615640576001811461565657615688565b60ff198316865281151560200286019350615688565b61565f8561530c565b5f5b8381101561568057815481890152600182019150602081019050615661565b808801955050505b50505092915050565b5f61569c8383615610565b905092915050565b5f600182019050919050565b5f6156ba826155d4565b6156c481856155de565b9350836020820285016156d6856155ee565b805f5b85811015615710578484038952816156f18582615691565b94506156fc836156a4565b925060208a019950506001810190506156d9565b50829750879550505050505092915050565b5f6060820190508181035f83015261573b8187896150a5565b9050818103602083015261574f81866156b0565b905081810360408301526157648184866150a5565b90509695505050505050565b7f4549503731323a20556e696e697469616c697a656400000000000000000000005f82015250565b5f6157a46015836147b0565b91506157af82615770565b602082019050919050565b5f6020820190508181035f8301526157d181615798565b9050919050565b5f82825260208201905092915050565b5f80fd5b82818337505050565b5f61580083856157d8565b93507f07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff831115615833576158326157e8565b5b6020830292506158448385846157ec565b82840190509392505050565b5f6020820190508181035f8301526158698184866157f5565b90509392505050565b5f67ffffffffffffffff82111561588c5761588b614984565b5b602082029050602081019050919050565b5f80fd5b5f80fd5b6158ae81614b02565b81146158b8575f80fd5b50565b5f815190506158c9816158a5565b92915050565b5f815190506158dd81614525565b92915050565b5f67ffffffffffffffff8211156158fd576158fc614984565b5b602082029050602081019050919050565b5f61592061591b846158e3565b6149e2565b9050808382526020820190506020840283018581111561594357615942614557565b5b835b8181101561596c57806159588882615296565b845260208401935050602081019050615945565b5050509392505050565b5f82601f83011261598a5761598961454f565b5b815161599a84826020860161590e565b91505092915050565b5f608082840312156159b8576159b761589d565b5b6159c260806149e2565b90505f6159d1848285016158bb565b5f8301525060206159e4848285016158cf565b60208301525060406159f8848285016158bb565b604083015250606082015167ffffffffffffffff811115615a1c57615a1b6158a1565b5b615a2884828501615976565b60608301525092915050565b5f615a46615a4184615872565b6149e2565b90508083825260208201905060208402830185811115615a6957615a68614557565b5b835b81811015615ab057805167ffffffffffffffff811115615a8e57615a8d61454f565b5b808601615a9b89826159a3565b85526020850194505050602081019050615a6b565b5050509392505050565b5f82601f830112615ace57615acd61454f565b5b8151615ade848260208601615a34565b91505092915050565b5f60208284031215615afc57615afb614514565b5b5f82015167ffffffffffffffff811115615b1957615b18614518565b5b615b2584828501615aba565b91505092915050565b5f615b388261451c565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203615b6a57615b69615035565b5b600182019050919050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b615ba781614b02565b82525050565b5f82825260208201905092915050565b5f615bc78261469f565b615bd18185615bad565b9350615bdc836146b9565b805f5b83811015615c0c578151615bf38882614707565b9750615bfe8361471e565b925050600181019050615bdf565b5085935050505092915050565b5f608083015f830151615c2e5f860182615b9e565b506020830151615c416020860182614bb4565b506040830151615c546040860182615b9e565b5060608301518482036060860152615c6c8282615bbd565b9150508091505092915050565b5f615c848383615c19565b905092915050565b5f602082019050919050565b5f615ca282615b75565b615cac8185615b7f565b935083602082028501615cbe85615b8f565b805f5b85811015615cf95784840389528151615cda8582615c79565b9450615ce583615c8c565b925060208a01995050600181019050615cc1565b50829750879550505050505092915050565b5f6040820190508181035f830152615d238186615c98565b90508181036020830152615d388184866150a5565b9050949350505050565b5f80fd5b5f80fd5b5f80fd5b5f8083356001602003843603038112615d6a57615d69615d42565b5b80840192508235915067ffffffffffffffff821115615d8c57615d8b615d46565b5b602083019250602082023603831315615da857615da7615d4a565b5b509250929050565b5f60ff82169050919050565b5f615dd6615dd1615dcc84615db0565b61538a565b61451c565b9050919050565b615de681615dbc565b82525050565b5f604082019050615dff5f830185615ddd565b615e0c6020830184614b6d565b9392505050565b5f60408284031215615e2857615e2761589d565b5b615e3260406149e2565b90505f615e418482850161453b565b5f830152506020615e548482850161453b565b60208301525092915050565b5f60408284031215615e7557615e74614514565b5b5f615e8284828501615e13565b91505092915050565b5f819050919050565b5f615ea2602084018461496c565b905092915050565b5f602082019050919050565b5f615ec183856146a9565b9350615ecc82615e8b565b805f5b85811015615f0457615ee18284615e94565b615eeb8882614707565b9750615ef683615eaa565b925050600181019050615ecf565b5085925050509392505050565b5f604082019050615f245f830186614b7c565b8181036020830152615f37818486615eb6565b9050949350505050565b5f81519050919050565b5f819050602082019050919050565b5f615f658383615b9e565b60208301905092915050565b5f602082019050919050565b5f615f8782615f41565b615f9181856157d8565b9350615f9c83615f4b565b805f5b83811015615fcc578151615fb38882615f5a565b9750615fbe83615f71565b925050600181019050615f9f565b5085935050505092915050565b5f6020820190508181035f830152615ff18184615f7d565b905092915050565b5f81519050919050565b61600c82615ff9565b67ffffffffffffffff81111561602557616024614984565b5b61602f8254615005565b61603a828285615420565b5f60209050601f83116001811461606b575f8415616059578287015190505b616063858261548e565b8655506160ca565b601f1984166160798661530c565b5f5b828110156160a05784890151825560018201915060208501945060208101905061607b565b868310156160bd57848901516160b9601f891682615472565b8355505b6001600288020188555050505b505050505050565b5f6080820190508181035f8301526160ea8189615c98565b90506160f96020830188614b7c565b818103604083015261610c8186886150a5565b905081810360608301526161218184866150a5565b9050979650505050505050565b5f6040820190506161415f830185614b0b565b61614e6020830184614b7c565b9392505050565b5f6020828403121561616a57616169614514565b5b5f6161778482850161496c565b91505092915050565b5f81905092915050565b61619381614b02565b82525050565b5f6161a4838361618a565b60208301905092915050565b5f6161ba82615f41565b6161c48185616180565b93506161cf83615f4b565b805f5b838110156161ff5781516161e68882616199565b97506161f183615f71565b9250506001810190506161d2565b5085935050505092915050565b5f61621782846161b0565b915081905092915050565b5f81905092915050565b5f61623682615ff9565b6162408185616222565b93506162508185602086016147c0565b80840191505092915050565b5f616267828461622c565b915081905092915050565b5f60a0820190506162855f830188614b0b565b6162926020830187614b0b565b61629f6040830186614b0b565b6162ac6060830185614b0b565b6162b96080830184614b0b565b9695505050505050565b5f6040820190506162d65f830185614b6d565b6162e36020830184614b7c565b9392505050565b5f602082840312156162ff576162fe614514565b5b5f61630c848285016158cf565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f6020828403121561635757616356614514565b5b5f616364848285016158bb565b91505092915050565b5f6080820190506163805f830187614b0b565b61638d6020830186614b0b565b61639a6040830185614b0b565b6163a76060830184614b0b565b95945050505050565b5f6163ba8261451c565b91506163c58361451c565b92508282019050808211156163dd576163dc615035565b5b92915050565b5f6040820190506163f65f830185614b6d565b6164036020830184614b6d565b9392505050565b5f608083015f83015161641f5f860182615b9e565b5060208301516164326020860182614bb4565b5060408301516164456040860182615b9e565b506060830151848203606086015261645d8282615bbd565b9150508091505092915050565b5f6040820190508181035f830152616482818561640a565b90508181036020830152616496818461640a565b90509392505050565b5f61ffff82169050919050565b5f6164c66164c16164bc8461649f565b61538a565b61451c565b9050919050565b6164d6816164ac565b82525050565b5f6040820190506164ef5f8301856164cd565b6164fc6020830184614b6d565b9392505050565b5f61650d8261451c565b91506165188361451c565b92508282026165268161451c565b9150828204841483151761653d5761653c615035565b5b5092915050565b604082015f8201516165585f850182614bb4565b50602082015161656b6020850182614bb4565b50505050565b5f6060820190506165845f830185614b6d565b6165916020830184616544565b9392505050565b5f6040820190506165ab5f830185614b7c565b81810360208301526165bd818461472a565b90509392505050565b5f6020820190508181035f8301526165df8184866150a5565b90509392505050565b5f67ffffffffffffffff82111561660257616601614984565b5b61660b826147e8565b9050602081019050919050565b5f61662a616625846165e8565b6149e2565b90508281526020810184848401111561664657616645614980565b5b6166518482856147c0565b509392505050565b5f82601f83011261666d5761666c61454f565b5b815161667d848260208601616618565b91505092915050565b5f6080828403121561669b5761669a61589d565b5b6166a560806149e2565b90505f6166b484828501615296565b5f8301525060206166c784828501615296565b602083015250604082015167ffffffffffffffff8111156166eb576166ea6158a1565b5b6166f784828501616659565b604083015250606082015167ffffffffffffffff81111561671b5761671a6158a1565b5b61672784828501616659565b60608301525092915050565b5f6020828403121561674857616747614514565b5b5f82015167ffffffffffffffff81111561676557616764614518565b5b61677184828501616686565b91505092915050565b5f60408201905061678d5f830185614b7c565b61679a6020830184614b7c565b9392505050565b5f819050815f5260205f209050919050565b601f8211156167f4576167c5816167a1565b6167ce8461531e565b810160208510156167dd578190505b6167f16167e98561531e565b8301826153fe565b50505b505050565b616802826147a6565b67ffffffffffffffff81111561681b5761681a614984565b5b6168258254615005565b6168308282856167b3565b5f60209050601f831160018114616861575f841561684f578287015190505b616859858261548e565b8655506168c0565b601f19841661686f866167a1565b5f5b8281101561689657848901518255600182019150602085019450602081019050616871565b868310156168b357848901516168af601f891682615472565b8355505b6001600288020188555050505b505050505050565b6168d181615db0565b82525050565b5f6020820190506168ea5f8301846168c8565b92915050565b6054811061690157616900614f92565b5b50565b5f819050616911826168f0565b919050565b5f61692082616904565b9050919050565b61693081616916565b82525050565b5f6020820190506169495f830184616927565b92915050565b5f81905092915050565b616962816146e7565b82525050565b5f6169738383616959565b60208301905092915050565b5f6169898261469f565b616993818561694f565b935061699e836146b9565b805f5b838110156169ce5781516169b58882616968565b97506169c08361471e565b9250506001810190506169a1565b5085935050505092915050565b5f6169e6828461697f565b915081905092915050565b5f60c082019050616a045f830189614b0b565b616a116020830188614b0b565b616a1e6040830187614b0b565b616a2b6060830186614b6d565b616a386080830185614b6d565b616a4560a0830184614b0b565b979650505050505050565b5f60a082019050616a635f830188614b0b565b616a706020830187614b0b565b616a7d6040830186614b0b565b616a8a6060830185614b6d565b616a976080830184614b7c565b9695505050505050565b5f608082019050616ab45f830187614b0b565b616ac160208301866168c8565b616ace6040830185614b0b565b616adb6060830184614b0b565b9594505050505056fe5573657244656372797074526573706f6e7365566572696669636174696f6e286279746573207075626c69634b65792c627974657333325b5d20637448616e646c65732c6279746573207573657244656372797074656453686172652c627974657320657874726144617461295075626c696344656372797074566572696669636174696f6e28627974657333325b5d20637448616e646c65732c627974657320646563727970746564526573756c742c62797465732065787472614461746129557365724465637279707452657175657374566572696669636174696f6e286279746573207075626c69634b65792c616464726573735b5d20636f6e74726163744164647265737365732c75696e7432353620737461727454696d657374616d702c75696e74323536206475726174696f6e446179732c62797465732065787472614461746129
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01\x08W_5`\xE0\x1C\x80c\\\x97Z\xBB\x11a\0\x94W\x80c\xAD<\xB1\xCC\x11a\0cW\x80c\xAD<\xB1\xCC\x14a\x03\x1CW\x80c\xC4\x11Xt\x14a\x03FW\x80c\xD8\x99\x8FE\x14a\x03\\W\x80c\xF1\xB5z\xDB\x14a\x03\x84W\x80c\xFB\xB82Y\x14a\x03\xACWa\x01\x08V[\x80c\\\x97Z\xBB\x14a\x02\x84W\x80co\x89\x13\xBC\x14a\x02\xAEW\x80c\x84V\xCBY\x14a\x02\xD6W\x80c\x84\xB0\x19n\x14a\x02\xECWa\x01\x08V[\x80c?K\xA8:\x11a\0\xDBW\x80c?K\xA8:\x14a\x01\xB0W\x80c@\x14\xC4\xCD\x14a\x01\xC6W\x80cO\x1E\xF2\x86\x14a\x02\x02W\x80cR\xD1\x90-\x14a\x02\x1EW\x80cX\xF5\xB8\xAB\x14a\x02HWa\x01\x08V[\x80c\x04o\x9E\xB3\x14a\x01\x0CW\x80c\t\0\xCCi\x14a\x014W\x80c\r\x8En,\x14a\x01pW\x80c9\xF78\x10\x14a\x01\x9AW[_\x80\xFD[4\x80\x15a\x01\x17W_\x80\xFD[Pa\x012`\x04\x806\x03\x81\x01\x90a\x01-\x91\x90aE\xB0V[a\x03\xE8V[\0[4\x80\x15a\x01?W_\x80\xFD[Pa\x01Z`\x04\x806\x03\x81\x01\x90a\x01U\x91\x90aFtV[a\x08tV[`@Qa\x01g\x91\x90aG\x86V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01{W_\x80\xFD[Pa\x01\x84a\tEV[`@Qa\x01\x91\x91\x90aH0V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xA5W_\x80\xFD[Pa\x01\xAEa\t\xC0V[\0[4\x80\x15a\x01\xBBW_\x80\xFD[Pa\x01\xC4a\x0B\xF8V[\0[4\x80\x15a\x01\xD1W_\x80\xFD[Pa\x01\xEC`\x04\x806\x03\x81\x01\x90a\x01\xE7\x91\x90aH\xA5V[a\r@V[`@Qa\x01\xF9\x91\x90aI=V[`@Q\x80\x91\x03\x90\xF3[a\x02\x1C`\x04\x806\x03\x81\x01\x90a\x02\x17\x91\x90aJ\xA8V[a\x0E\xCDV[\0[4\x80\x15a\x02)W_\x80\xFD[Pa\x022a\x0E\xECV[`@Qa\x02?\x91\x90aK\x1AV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02SW_\x80\xFD[Pa\x02n`\x04\x806\x03\x81\x01\x90a\x02i\x91\x90aFtV[a\x0F\x1DV[`@Qa\x02{\x91\x90aI=V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x8FW_\x80\xFD[Pa\x02\x98a\x0FPV[`@Qa\x02\xA5\x91\x90aI=V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xB9W_\x80\xFD[Pa\x02\xD4`\x04\x806\x03\x81\x01\x90a\x02\xCF\x91\x90aE\xB0V[a\x0FrV[\0[4\x80\x15a\x02\xE1W_\x80\xFD[Pa\x02\xEAa\x13\xBDV[\0[4\x80\x15a\x02\xF7W_\x80\xFD[Pa\x03\0a\x14\xE2V[`@Qa\x03\x13\x97\x96\x95\x94\x93\x92\x91\x90aLBV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03'W_\x80\xFD[Pa\x030a\x15\xEBV[`@Qa\x03=\x91\x90aH0V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03QW_\x80\xFD[Pa\x03Za\x16$V[\0[4\x80\x15a\x03gW_\x80\xFD[Pa\x03\x82`\x04\x806\x03\x81\x01\x90a\x03}\x91\x90aH\xA5V[a\x17IV[\0[4\x80\x15a\x03\x8FW_\x80\xFD[Pa\x03\xAA`\x04\x806\x03\x81\x01\x90a\x03\xA5\x91\x90aMYV[a\x19\x10V[\0[4\x80\x15a\x03\xB7W_\x80\xFD[Pa\x03\xD2`\x04\x806\x03\x81\x01\x90a\x03\xCD\x91\x90aN\x93V[a\x1D\xCAV[`@Qa\x03\xDF\x91\x90aI=V[`@Q\x80\x91\x03\x90\xF3[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE5'^\xAF3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x045\x91\x90aO$V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04PW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04t\x91\x90aOgV[a\x04\xB5W3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04\xAC\x91\x90aO$V[`@Q\x80\x91\x03\x90\xFD[_a\x04\xBEa 9V[\x90P`\xF8`\x02`\x06\x81\x11\x15a\x04\xD6Wa\x04\xD5aO\x92V[[\x90\x1B\x88\x11\x15\x80a\x04\xE9WP\x80`\x08\x01T\x88\x11[\x15a\x05+W\x87`@Q\x7F\xD4\x8A\xF9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05\"\x91\x90aO\xBFV[`@Q\x80\x91\x03\x90\xFD[_\x81`\x07\x01_\x8A\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80Ta\x05Y\x90aP\x05V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x85\x90aP\x05V[\x80\x15a\x05\xD0W\x80`\x1F\x10a\x05\xA7Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xD0V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xB3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x06&W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x06\x12W[PPPPP\x81RPP\x90P_`@Q\x80`\x80\x01`@R\x80\x83_\x01Q\x81R` \x01\x83` \x01Q\x81R` \x01\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81RP\x90P_a\x06\xEC\x82a `V[\x90Pa\x06\xFA\x8B\x82\x8A\x8Aa!'V[_\x84`\x02\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _\x80_\x1B\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x8B\x7F\x7F\xCD\xFBS\x81\x91\x7FUJq}\nTp\xA3?ZI\xBAdE\xF0^\xC4<t\xC0\xBC,\xC6\x08\xB2`\x01\x83\x80T\x90Pa\x07\xB3\x91\x90aPbV[\x8D\x8D\x8D\x8D\x8D\x8D`@Qa\x07\xCC\x97\x96\x95\x94\x93\x92\x91\x90aP\xD1V[`@Q\x80\x91\x03\x90\xA2\x84_\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x08\tWPa\x08\x08\x81\x80T\x90Pa\"\x99V[[\x15a\x08fW`\x01\x85_\x01_\x8E\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x8B\x7F\xE8\x97R\xBE\x0E\xCD\xB6\x8B*n\xB5\xEF\x1A\x89\x109\xE0\xE9*\xE3\xC8\xA6\"t\xC5\x88\x1EH\xEE\xA1\xED%`@Q`@Q\x80\x91\x03\x90\xA2[PPPPPPPPPPPPV[``_a\x08\x7Fa 9V[\x90P_\x81`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\x02\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x82\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\t7W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x08\xEEW[PPPPP\x92PPP\x91\x90PV[```@Q\x80`@\x01`@R\x80`\n\x81R` \x01\x7FDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\t\x86_a#*V[a\t\x90`\x02a#*V[a\t\x99_a#*V[`@Q` \x01a\t\xAC\x94\x93\x92\x91\x90aQ\xFDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`\x01a\t\xCAa#\xF4V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\n\x0BW`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03_a\n\x16a$\x18V[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\n^WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\n\x95W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x0BN`@Q\x80`@\x01`@R\x80`\n\x81R` \x01\x7FDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa$?V[a\x0BVa$UV[_a\x0B_a 9V[\x90P`\xF8`\x01`\x06\x81\x11\x15a\x0BwWa\x0BvaO\x92V[[\x90\x1B\x81`\x06\x01\x81\x90UP`\xF8`\x02`\x06\x81\x11\x15a\x0B\x97Wa\x0B\x96aO\x92V[[\x90\x1B\x81`\x08\x01\x81\x90UPP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x0B\xEC\x91\x90aR}V[`@Q\x80\x91\x03\x90\xA1PPV[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CUW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Cy\x91\x90aR\xAAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\x0C\xF4WPs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a\r6W3`@Q\x7F\xE1\x91f\xEE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r-\x91\x90aO$V[`@Q\x80\x91\x03\x90\xFD[a\r>a$gV[V[_\x80_\x90P[\x85\x85\x90P\x81\x10\x15a\x0E\xBFWs\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x06 2m\x87\x87\x84\x81\x81\x10a\r\x94Wa\r\x93aR\xD5V[[\x90P` \x02\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\xB7\x91\x90aK\x1AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xD2W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xF6\x91\x90aOgV[\x15\x80a\x0E\xA4WPs\xDE@\x91\t\xE0\xFC\xCA\xAE{\x87\xDEQ\x8Fa\xD6\x17\xA3\xFD\xA0\x94s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c-\xDC\x9Ao\x87\x87\x84\x81\x81\x10a\x0E@Wa\x0E?aR\xD5V[[\x90P` \x02\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0Ec\x91\x90aK\x1AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E~W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xA2\x91\x90aOgV[\x15[\x15a\x0E\xB2W_\x91PPa\x0E\xC5V[\x80\x80`\x01\x01\x91PPa\rFV[P`\x01\x90P[\x94\x93PPPPV[a\x0E\xD5a$\xD5V[a\x0E\xDE\x82a%\xBBV[a\x0E\xE8\x82\x82a&\xAEV[PPV[_a\x0E\xF5a'\xCCV[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[_\x80a\x0F'a 9V[\x90P\x80_\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[_\x80a\x0FZa(SV[\x90P\x80_\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x90V[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE5'^\xAF3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\xBF\x91\x90aO$V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xDAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xFE\x91\x90aOgV[a\x10?W3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x106\x91\x90aO$V[`@Q\x80\x91\x03\x90\xFD[_a\x10Ha 9V[\x90P`\xF8`\x01`\x06\x81\x11\x15a\x10`Wa\x10_aO\x92V[[\x90\x1B\x88\x11\x15\x80a\x10sWP\x80`\x06\x01T\x88\x11[\x15a\x10\xB5W\x87`@Q\x7F\xD4\x8A\xF9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\xAC\x91\x90aO\xBFV[`@Q\x80\x91\x03\x90\xFD[_`@Q\x80``\x01`@R\x80\x83`\x05\x01_\x8C\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x11\x1CW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x11\x08W[PPPPP\x81R` \x01\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81RP\x90P_a\x11\xC2\x82a(zV[\x90Pa\x11\xD0\x8A\x82\x89\x89a!'V[_\x83`\x04\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x90P\x80\x88\x88\x90\x91\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x92\x90\x91\x92\x90\x91\x92\x90\x91\x92P\x91\x82a\x12.\x92\x91\x90aT\xA9V[P\x83`\x02\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ 3\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x8A\x7FM{\x1D\xBAI\xE9\xE8F!^\x16!\xF5s|\x81\xD8aLO&\x84\x94\xD8\xB7\x87c,NY\xF0\xE5\x8B\x8B\x8B\x8B3\x8C\x8C`@Qa\x12\xEB\x97\x96\x95\x94\x93\x92\x91\x90aUvV[`@Q\x80\x91\x03\x90\xA2\x83_\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x13(WPa\x13'\x81\x80T\x90Pa)4V[[\x15a\x13\xB0W`\x01\x84_\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x81\x84`\x03\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x8A\x7F\xD7\xE5\x8A6z\nl)\x8Ev\xAD]$\0\x04\xE3'\xAA\x14#\xCB\xE4\xBD\x7F\xF8]Lq^\xF8\xD1_\x8B\x8B\x84\x8A\x8A`@Qa\x13\xA7\x95\x94\x93\x92\x91\x90aW\"V[`@Q\x80\x91\x03\x90\xA2[PPPPPPPPPPPV[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cF\xFB\xF6\x8E3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\n\x91\x90aO$V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14%W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14I\x91\x90aOgV[\x15\x80\x15a\x14\x96WPs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a\x14\xD8W3`@Q\x7F8\x89\x16\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14\xCF\x91\x90aO$V[`@Q\x80\x91\x03\x90\xFD[a\x14\xE0a)\xC5V[V[_``\x80_\x80_``_a\x14\xF4a*4V[\x90P_\x80\x1B\x81_\x01T\x14\x80\x15a\x15\x0FWP_\x80\x1B\x81`\x01\x01T\x14[a\x15NW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15E\x90aW\xBAV[`@Q\x80\x91\x03\x90\xFD[a\x15Va*[V[a\x15^a*\xF9V[F0_\x80\x1B_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15}Wa\x15|aI\x84V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x15\xABW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94\x93\x92\x91\x90\x97P\x97P\x97P\x97P\x97P\x97P\x97PP\x90\x91\x92\x93\x94\x95\x96V[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`\x03_a\x16/a$\x18V[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x16wWP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x16\xAEW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x17=\x91\x90aR}V[`@Q\x80\x91\x03\x90\xA1PPV[a\x17Qa+\x97V[_\x84\x84\x90P\x03a\x17\x8DW`@Q\x7F-\xE7T8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x17\xD6\x84\x84\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa+\xD8V[_s\xDE@\x91\t\xE0\xFC\xCA\xAE{\x87\xDEQ\x8Fa\xD6\x17\xA3\xFD\xA0\x94s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA1O\x89q\x86\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18&\x92\x91\x90aXPV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18@W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18h\x91\x90aZ\xE7V[\x90Pa\x18s\x81a,\x90V[_a\x18|a 9V[\x90P\x80`\x06\x01_\x81T\x80\x92\x91\x90a\x18\x92\x90a[.V[\x91\x90PUP_\x81`\x06\x01T\x90P\x86\x86\x83`\x05\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x91\x90a\x18\xC1\x92\x91\x90aDZV[Pa\x18\xCB3a-vV[\x80\x7F\"\xDBH\n9\xBDrUd8\xAA\xDBJ2\xA3\xD2\xA6c\x8B\x87\xC0;\xBE\xC5\xFE\xF6\x99~\x10\x95\x87\xFF\x84\x87\x87`@Qa\x18\xFF\x93\x92\x91\x90a]\x0BV[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[a\x19\x18a+\x97V[_\x88\x80` \x01\x90a\x19)\x91\x90a]NV[\x90P\x03a\x19bW`@Q\x7FW\xCF\xA2\x17\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\n`\xFF\x16\x88\x80` \x01\x90a\x19w\x91\x90a]NV[\x90P\x11\x15a\x19\xD0W`\n\x88\x80` \x01\x90a\x19\x91\x91\x90a]NV[\x90P`@Q\x7F\xAF\x1F\x04\x95\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19\xC7\x92\x91\x90a]\xECV[`@Q\x80\x91\x03\x90\xFD[a\x19\xE9\x89\x806\x03\x81\x01\x90a\x19\xE4\x91\x90a^`V[a-\xF3V[a\x1AA\x88\x80` \x01\x90a\x19\xFC\x91\x90a]NV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x88a/>V[\x15a\x1A\x95W\x86\x88\x80` \x01\x90a\x1AW\x91\x90a]NV[`@Q\x7F\xDCMx\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1A\x8C\x93\x92\x91\x90a_\x11V[`@Q\x80\x91\x03\x90\xFD[_a\x1A\xF0\x8C\x8C\x8B\x80` \x01\x90a\x1A\xAB\x91\x90a]NV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x8Ba/\xBCV[\x90P_`@Q\x80`\xA0\x01`@R\x80\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8B\x80` \x01\x90a\x1BW\x91\x90a]NV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8C_\x015\x81R` \x01\x8C` \x015\x81R` \x01\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81RP\x90Pa\x1C\x07\x81\x8A\x88\x88\x8E_\x015a1\xAFV[_s\xDE@\x91\t\xE0\xFC\xCA\xAE{\x87\xDEQ\x8Fa\xD6\x17\xA3\xFD\xA0\x94s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA1O\x89q\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1CU\x91\x90a_\xD9V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CoW=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x97\x91\x90aZ\xE7V[\x90Pa\x1C\xA2\x81a,\x90V[_a\x1C\xABa 9V[\x90P\x80`\x08\x01_\x81T\x80\x92\x91\x90a\x1C\xC1\x90a[.V[\x91\x90PUP_\x81`\x08\x01T\x90P`@Q\x80`@\x01`@R\x80\x8C\x8C\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x86\x81RP\x82`\x07\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01\x90\x81a\x1DL\x91\x90a`\x03V[P` \x82\x01Q\x81`\x01\x01\x90\x80Q\x90` \x01\x90a\x1Di\x92\x91\x90aD\xA5V[P\x90PPa\x1Dv3a2\x87V[\x80\x7F\xF9\x01\x1B\xD6\xBA\r\xA6\x04\x9CR\rp\xFEYq\xF1~\xD7\xAByT\x86\x05%D\xB5\x10\x19\x89lYk\x84\x8E\x8E\x8E\x8C\x8C`@Qa\x1D\xB0\x96\x95\x94\x93\x92\x91\x90a`\xD2V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPPPPPV[_\x80_\x90P[\x85\x85\x90P\x81\x10\x15a *Ws\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC6R\x8Fi\x87\x87\x84\x81\x81\x10a\x1E\x1EWa\x1E\x1DaR\xD5V[[\x90P`@\x02\x01_\x015\x89`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1EE\x92\x91\x90aa.V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E`W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x84\x91\x90aOgV[\x15\x80a\x1F`WPs\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC6R\x8Fi\x87\x87\x84\x81\x81\x10a\x1E\xCEWa\x1E\xCDaR\xD5V[[\x90P`@\x02\x01_\x015\x88\x88\x85\x81\x81\x10a\x1E\xEAWa\x1E\xE9aR\xD5V[[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x1F\x02\x91\x90aaUV[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1F\x1F\x92\x91\x90aa.V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F:W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F^\x91\x90aOgV[\x15[\x80a \x0FWPs\xDE@\x91\t\xE0\xFC\xCA\xAE{\x87\xDEQ\x8Fa\xD6\x17\xA3\xFD\xA0\x94s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c-\xDC\x9Ao\x87\x87\x84\x81\x81\x10a\x1F\xA9Wa\x1F\xA8aR\xD5V[[\x90P`@\x02\x01_\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1F\xCE\x91\x90aK\x1AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\xE9W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \r\x91\x90aOgV[\x15[\x15a \x1DW_\x91PPa 0V[\x80\x80`\x01\x01\x91PPa\x1D\xD0V[P`\x01\x90P[\x95\x94PPPPPV[_\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\0\x90P\x90V[_a! `@Q\x80`\xA0\x01`@R\x80`m\x81R` \x01aj\xE5`m\x919\x80Q\x90` \x01 \x83_\x01Q\x80Q\x90` \x01 \x84` \x01Q`@Q` \x01a \xA4\x91\x90ab\x0CV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85`@\x01Q\x80Q\x90` \x01 \x86``\x01Q`@Q` \x01a \xDB\x91\x90ab\\V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a!\x05\x95\x94\x93\x92\x91\x90abrV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a3\x04V[\x90P\x91\x90PV[_a!0a 9V[\x90P_a!\x80\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa3\x1DV[\x90Pa!\x8C\x813a3GV[\x81`\x01\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\"+W\x85\x81`@Q\x7F\x99\xECH\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\"\"\x92\x91\x90ab\xC3V[`@Q\x80\x91\x03\x90\xFD[`\x01\x82`\x01\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPPPPPPV[_\x80s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC2\xB4)\x86`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\xF8W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\x1C\x91\x90ab\xEAV[\x90P\x80\x83\x10\x15\x91PP\x91\x90PV[``_`\x01a#8\x84a4XV[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#VWa#UaI\x84V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a#\x88W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x01\x82\x01\x90P[`\x01\x15a#\xE9W\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a#\xDEWa#\xDDac\x15V[[\x04\x94P_\x85\x03a#\x95W[\x81\x93PPPP\x91\x90PV[_a#\xFDa$\x18V[_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[a$Ga5\xA9V[a$Q\x82\x82a5\xE9V[PPV[a$]a5\xA9V[a$ea6:V[V[a$oa6jV[_a$xa(SV[\x90P_\x81_\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAAa$\xBDa6\xAAV[`@Qa$\xCA\x91\x90aO$V[`@Q\x80\x91\x03\x90\xA1PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a%\x82WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a%ia6\xB1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a%\xB9W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\x18W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&<\x91\x90aR\xAAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a&\xABW3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&\xA2\x91\x90aO$V[`@Q\x80\x91\x03\x90\xFD[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a'\x16WP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\x13\x91\x90acBV[`\x01[a'WW\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a'N\x91\x90aO$V[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14a'\xBDW\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a'\xB4\x91\x90aK\x1AV[`@Q\x80\x91\x03\x90\xFD[a'\xC7\x83\x83a7\x04V[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a(QW`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0\x90P\x90V[_a)-`@Q\x80`\x80\x01`@R\x80`T\x81R` \x01akR`T\x919\x80Q\x90` \x01 \x83_\x01Q`@Q` \x01a(\xB2\x91\x90ab\x0CV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q\x80Q\x90` \x01 \x85`@\x01Q`@Q` \x01a(\xE9\x91\x90ab\\V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a)\x12\x94\x93\x92\x91\x90acmV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a3\x04V[\x90P\x91\x90PV[_\x80s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c*8\x89\x98`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)\x93W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\xB7\x91\x90ab\xEAV[\x90P\x80\x83\x10\x15\x91PP\x91\x90PV[a)\xCDa+\x97V[_a)\xD6a(SV[\x90P`\x01\x81_\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2Xa*\x1Ca6\xAAV[`@Qa*)\x91\x90aO$V[`@Q\x80\x91\x03\x90\xA1PV[_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x90P\x90V[``_a*fa*4V[\x90P\x80`\x02\x01\x80Ta*w\x90aP\x05V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta*\xA3\x90aP\x05V[\x80\x15a*\xEEW\x80`\x1F\x10a*\xC5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a*\xEEV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a*\xD1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[``_a+\x04a*4V[\x90P\x80`\x03\x01\x80Ta+\x15\x90aP\x05V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta+A\x90aP\x05V[\x80\x15a+\x8CW\x80`\x1F\x10a+cWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a+\x8CV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a+oW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[a+\x9Fa\x0FPV[\x15a+\xD6W`@Q\x7F\xD9<\x06e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x80[\x82Q\x81\x10\x15a,@W_\x83\x82\x81Q\x81\x10a+\xF8Wa+\xF7aR\xD5V[[` \x02` \x01\x01Q\x90P_a,\x0C\x82a7vV[\x90Pa,\x17\x81a8\0V[a\xFF\xFF\x16\x84a,&\x91\x90ac\xB0V[\x93Pa,1\x82a9\xEBV[PP\x80\x80`\x01\x01\x91PPa+\xDBV[Pa\x08\0\x81\x11\x15a,\x8CWa\x08\0\x81`@Q\x7F\xE7\xF4\x89]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a,\x83\x92\x91\x90ac\xE3V[`@Q\x80\x91\x03\x90\xFD[PPV[`\x01\x81Q\x11\x15a-sW_\x81_\x81Q\x81\x10a,\xAEWa,\xADaR\xD5V[[` \x02` \x01\x01Q` \x01Q\x90P_`\x01\x90P[\x82Q\x81\x10\x15a-pW\x81\x83\x82\x81Q\x81\x10a,\xDFWa,\xDEaR\xD5V[[` \x02` \x01\x01Q` \x01Q\x14a-cW\x82_\x81Q\x81\x10a-\x03Wa-\x02aR\xD5V[[` \x02` \x01\x01Q\x83\x82\x81Q\x81\x10a-\x1EWa-\x1DaR\xD5V[[` \x02` \x01\x01Q`@Q\x7F\xCF\xAE\x92\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a-Z\x92\x91\x90adjV[`@Q\x80\x91\x03\x90\xFD[\x80\x80`\x01\x01\x91PPa,\xC2V[PP[PV[s\x873\xD4\x01>\xFCBV\x97qP\xF3\x1A\x8E\xA1\xE9\xE4\xC1E\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x91\xEE\xB2|\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a-\xC3\x91\x90aO$V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a-\xDAW_\x80\xFD[PZ\xF1\x15\x80\x15a-\xECW=_\x80>=_\xFD[PPPPPV[_\x81` \x01Q\x03a.0W`@Q\x7F\xDE(Y\xC1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01ma\xFF\xFF\x16\x81` \x01Q\x11\x15a.\x87Wa\x01m\x81` \x01Q`@Q\x7F2\x95\x18c\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.~\x92\x91\x90ad\xDCV[`@Q\x80\x91\x03\x90\xFD[B\x81_\x01Q\x11\x15a.\xD4WB\x81_\x01Q`@Q\x7F\xF2L\x08\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.\xCB\x92\x91\x90ac\xE3V[`@Q\x80\x91\x03\x90\xFD[Bb\x01Q\x80\x82` \x01Qa.\xE8\x91\x90ae\x03V[\x82_\x01Qa.\xF6\x91\x90ac\xB0V[\x10\x15a/;WB\x81`@Q\x7F04\x80@\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a/2\x92\x91\x90aeqV[`@Q\x80\x91\x03\x90\xFD[PV[_\x80_\x90P[\x83Q\x81\x10\x15a/\xB1W\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x82\x81Q\x81\x10a/wWa/vaR\xD5V[[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a/\xA4W`\x01\x91PPa/\xB6V[\x80\x80`\x01\x01\x91PPa/DV[P_\x90P[\x92\x91PPV[``_\x85\x85\x90P\x03a/\xFAW`@Q\x7F\xA6\xA6\xCB!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84\x84\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0\x17Wa0\x16aI\x84V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a0EW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x80[\x86\x86\x90P\x81\x10\x15a1ZW_\x87\x87\x83\x81\x81\x10a0jWa0iaR\xD5V[[\x90P`@\x02\x01_\x015\x90P_\x88\x88\x84\x81\x81\x10a0\x89Wa0\x88aR\xD5V[[\x90P`@\x02\x01` \x01` \x81\x01\x90a0\xA1\x91\x90aaUV[\x90P_a0\xAD\x83a7vV[\x90Pa0\xB8\x81a8\0V[a\xFF\xFF\x16\x85a0\xC7\x91\x90ac\xB0V[\x94Pa0\xD3\x83\x88a:\xBBV[a0\xDD\x83\x83a:\xBBV[a0\xE7\x88\x83a/>V[a1*W\x81\x88`@Q\x7F\xA4\xC3\x03\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a1!\x92\x91\x90ae\x98V[`@Q\x80\x91\x03\x90\xFD[\x82\x86\x85\x81Q\x81\x10a1>Wa1=aR\xD5V[[` \x02` \x01\x01\x81\x81RPPPPP\x80\x80`\x01\x01\x91PPa0KV[Pa\x08\0\x81\x11\x15a1\xA6Wa\x08\0\x81`@Q\x7F\xE7\xF4\x89]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a1\x9D\x92\x91\x90ac\xE3V[`@Q\x80\x91\x03\x90\xFD[P\x94\x93PPPPV[_a1\xBA\x86\x83a;\x90V[\x90P_a2\n\x82\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa3\x1DV[\x90P\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a2~W\x84\x84`@Q\x7F*\x87='\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a2u\x92\x91\x90ae\xC6V[`@Q\x80\x91\x03\x90\xFD[PPPPPPPV[s\x873\xD4\x01>\xFCBV\x97qP\xF3\x1A\x8E\xA1\xE9\xE4\xC1E\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x98\x8A--\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a2\xD4\x91\x90aO$V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a2\xEBW_\x80\xFD[PZ\xF1\x15\x80\x15a2\xFDW=_\x80>=_\xFD[PPPPPV[_a3\x16a3\x10a<]V[\x83a<kV[\x90P\x91\x90PV[_\x80_\x80a3+\x86\x86a<\xABV[\x92P\x92P\x92Pa3;\x82\x82a=\0V[\x82\x93PPPP\x92\x91PPV[a3P\x82a>bV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3\xB2\xA8t\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a3\xB4\x91\x90aO$V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3\xCEW=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\xF6\x91\x90ag3V[` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a4TW\x81\x81`@Q\x7F\r\x86\xF5!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a4K\x92\x91\x90agzV[`@Q\x80\x91\x03\x90\xFD[PPV[_\x80_\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a4\xB4Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81a4\xAAWa4\xA9ac\x15V[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a4\xF1Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81a4\xE7Wa4\xE6ac\x15V[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10a5 Wf#\x86\xF2o\xC1\0\0\x83\x81a5\x16Wa5\x15ac\x15V[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10a5IWc\x05\xF5\xE1\0\x83\x81a5?Wa5>ac\x15V[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10a5nWa'\x10\x83\x81a5dWa5cac\x15V[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10a5\x91W`d\x83\x81a5\x87Wa5\x86ac\x15V[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10a5\xA0W`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[a5\xB1a?2V[a5\xE7W`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a5\xF1a5\xA9V[_a5\xFAa*4V[\x90P\x82\x81`\x02\x01\x90\x81a6\r\x91\x90ag\xF9V[P\x81\x81`\x03\x01\x90\x81a6\x1F\x91\x90ag\xF9V[P_\x80\x1B\x81_\x01\x81\x90UP_\x80\x1B\x81`\x01\x01\x81\x90UPPPPV[a6Ba5\xA9V[_a6Ka(SV[\x90P_\x81_\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPV[a6ra\x0FPV[a6\xA8W`@Q\x7F\x8D\xFC +\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_3\x90P\x90V[_a6\xDD\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba?PV[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a7\r\x82a?YV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15a7iWa7c\x82\x82a@\"V[Pa7rV[a7qa@\xA2V[[PPV[_\x80`\xF8`\xF0\x84\x90\x1B\x90\x1C_\x1C\x90P`S\x80\x81\x11\x15a7\x98Wa7\x97aO\x92V[[`\xFF\x16\x81`\xFF\x16\x11\x15a7\xE2W\x80`@Q\x7Fd\x19P\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a7\xD9\x91\x90ah\xD7V[`@Q\x80\x91\x03\x90\xFD[\x80`\xFF\x16`S\x81\x11\x15a7\xF8Wa7\xF7aO\x92V[[\x91PP\x91\x90PV[_\x80`S\x81\x11\x15a8\x14Wa8\x13aO\x92V[[\x82`S\x81\x11\x15a8'Wa8&aO\x92V[[\x03a85W`\x02\x90Pa9\xE6V[`\x02`S\x81\x11\x15a8IWa8HaO\x92V[[\x82`S\x81\x11\x15a8\\Wa8[aO\x92V[[\x03a8jW`\x08\x90Pa9\xE6V[`\x03`S\x81\x11\x15a8~Wa8}aO\x92V[[\x82`S\x81\x11\x15a8\x91Wa8\x90aO\x92V[[\x03a8\x9FW`\x10\x90Pa9\xE6V[`\x04`S\x81\x11\x15a8\xB3Wa8\xB2aO\x92V[[\x82`S\x81\x11\x15a8\xC6Wa8\xC5aO\x92V[[\x03a8\xD4W` \x90Pa9\xE6V[`\x05`S\x81\x11\x15a8\xE8Wa8\xE7aO\x92V[[\x82`S\x81\x11\x15a8\xFBWa8\xFAaO\x92V[[\x03a9\tW`@\x90Pa9\xE6V[`\x06`S\x81\x11\x15a9\x1DWa9\x1CaO\x92V[[\x82`S\x81\x11\x15a90Wa9/aO\x92V[[\x03a9>W`\x80\x90Pa9\xE6V[`\x07`S\x81\x11\x15a9RWa9QaO\x92V[[\x82`S\x81\x11\x15a9eWa9daO\x92V[[\x03a9sW`\xA0\x90Pa9\xE6V[`\x08`S\x81\x11\x15a9\x87Wa9\x86aO\x92V[[\x82`S\x81\x11\x15a9\x9AWa9\x99aO\x92V[[\x03a9\xA9Wa\x01\0\x90Pa9\xE6V[\x81`@Q\x7F\xBEx0\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a9\xDD\x91\x90ai6V[`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x06 2m\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a:8\x91\x90aK\x1AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:SW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:w\x91\x90aOgV[a:\xB8W\x80`@Q\x7FC1\xA8]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a:\xAF\x91\x90aK\x1AV[`@Q\x80\x91\x03\x90\xFD[PV[s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC6R\x8Fi\x83\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a;\n\x92\x91\x90aa.V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;%W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;I\x91\x90aOgV[a;\x8CW\x81\x81`@Q\x7F\x16\n+K\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a;\x83\x92\x91\x90aa.V[`@Q\x80\x91\x03\x90\xFD[PPV[_\x80`@Q\x80`\xC0\x01`@R\x80`\x87\x81R` \x01ak\xA6`\x87\x919\x80Q\x90` \x01 \x84_\x01Q\x80Q\x90` \x01 \x85` \x01Q`@Q` \x01a;\xD2\x91\x90ai\xDBV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x86`@\x01Q\x87``\x01Q\x88`\x80\x01Q`@Q` \x01a<\x07\x91\x90ab\\V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a<2\x96\x95\x94\x93\x92\x91\x90ai\xF1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa<T\x83\x82a@\xDEV[\x91PP\x92\x91PPV[_a<faARV[\x90P\x90V[_`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x02\x82\x01R\x82`\"\x82\x01R`B\x81 \x91PP\x92\x91PPV[_\x80_`A\x84Q\x03a<\xEBW_\x80_` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90Pa<\xDD\x88\x82\x85\x85aA\xB5V[\x95P\x95P\x95PPPPa<\xF9V[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15a=\x13Wa=\x12aO\x92V[[\x82`\x03\x81\x11\x15a=&Wa=%aO\x92V[[\x03\x15a>^W`\x01`\x03\x81\x11\x15a=@Wa=?aO\x92V[[\x82`\x03\x81\x11\x15a=SWa=RaO\x92V[[\x03a=\x8AW`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15a=\x9EWa=\x9DaO\x92V[[\x82`\x03\x81\x11\x15a=\xB1Wa=\xB0aO\x92V[[\x03a=\xF5W\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a=\xEC\x91\x90aO\xBFV[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15a>\x08Wa>\x07aO\x92V[[\x82`\x03\x81\x11\x15a>\x1BWa>\x1AaO\x92V[[\x03a>]W\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a>T\x91\x90aK\x1AV[`@Q\x80\x91\x03\x90\xFD[[PPV[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c =\x01\x14\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a>\xAF\x91\x90aO$V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>\xCAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>\xEE\x91\x90aOgV[a?/W\x80`@Q\x7F*|n\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a?&\x91\x90aO$V[`@Q\x80\x91\x03\x90\xFD[PV[_a?;a$\x18V[_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a?\xB4W\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a?\xAB\x91\x90aO$V[`@Q\x80\x91\x03\x90\xFD[\x80a?\xE0\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba?PV[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qa@K\x91\x90ab\\V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a@\x83W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a@\x88V[``\x91P[P\x91P\x91Pa@\x98\x85\x83\x83aB\x9CV[\x92PPP\x92\x91PPV[_4\x11\x15a@\xDCW`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x80\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0FaA\taC)V[aA\x11aC\x9FV[\x860`@Q` \x01aA'\x95\x94\x93\x92\x91\x90ajPV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90PaAI\x81\x84a<kV[\x91PP\x92\x91PPV[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0FaA|aC)V[aA\x84aC\x9FV[F0`@Q` \x01aA\x9A\x95\x94\x93\x92\x91\x90ajPV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x80_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15aA\xF1W_`\x03\x85\x92P\x92P\x92PaB\x92V[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@QaB\x14\x94\x93\x92\x91\x90aj\xA1V[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aB4W=_\x80>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03aB\x85W_`\x01_\x80\x1B\x93P\x93P\x93PPaB\x92V[\x80_\x80_\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[``\x82aB\xB1WaB\xAC\x82aD\x16V[aC!V[_\x82Q\x14\x80\x15aB\xD7WP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15aC\x19W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aC\x10\x91\x90aO$V[`@Q\x80\x91\x03\x90\xFD[\x81\x90PaC\"V[[\x93\x92PPPV[_\x80aC3a*4V[\x90P_aC>a*[V[\x90P_\x81Q\x11\x15aCZW\x80\x80Q\x90` \x01 \x92PPPaC\x9CV[_\x82_\x01T\x90P_\x80\x1B\x81\x14aCuW\x80\x93PPPPaC\x9CV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x80aC\xA9a*4V[\x90P_aC\xB4a*\xF9V[\x90P_\x81Q\x11\x15aC\xD0W\x80\x80Q\x90` \x01 \x92PPPaD\x13V[_\x82`\x01\x01T\x90P_\x80\x1B\x81\x14aC\xECW\x80\x93PPPPaD\x13V[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x81Q\x11\x15aD(W\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15aD\x94W\x91` \x02\x82\x01[\x82\x81\x11\x15aD\x93W\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90aDxV[[P\x90PaD\xA1\x91\x90aD\xF0V[P\x90V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15aD\xDFW\x91` \x02\x82\x01[\x82\x81\x11\x15aD\xDEW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90aD\xC3V[[P\x90PaD\xEC\x91\x90aD\xF0V[P\x90V[[\x80\x82\x11\x15aE\x07W_\x81_\x90UP`\x01\x01aD\xF1V[P\x90V[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x81\x90P\x91\x90PV[aE.\x81aE\x1CV[\x81\x14aE8W_\x80\xFD[PV[_\x815\x90PaEI\x81aE%V[\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12aEpWaEoaEOV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aE\x8DWaE\x8CaESV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15aE\xA9WaE\xA8aEWV[[\x92P\x92\x90PV[_\x80_\x80_\x80_`\x80\x88\x8A\x03\x12\x15aE\xCBWaE\xCAaE\x14V[[_aE\xD8\x8A\x82\x8B\x01aE;V[\x97PP` \x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aE\xF9WaE\xF8aE\x18V[[aF\x05\x8A\x82\x8B\x01aE[V[\x96P\x96PP`@\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aF(WaF'aE\x18V[[aF4\x8A\x82\x8B\x01aE[V[\x94P\x94PP``\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aFWWaFVaE\x18V[[aFc\x8A\x82\x8B\x01aE[V[\x92P\x92PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[_` \x82\x84\x03\x12\x15aF\x89WaF\x88aE\x14V[[_aF\x96\x84\x82\x85\x01aE;V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_aF\xF1\x82aF\xC8V[\x90P\x91\x90PV[aG\x01\x81aF\xE7V[\x82RPPV[_aG\x12\x83\x83aF\xF8V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aG4\x82aF\x9FV[aG>\x81\x85aF\xA9V[\x93PaGI\x83aF\xB9V[\x80_[\x83\x81\x10\x15aGyW\x81QaG`\x88\x82aG\x07V[\x97PaGk\x83aG\x1EV[\x92PP`\x01\x81\x01\x90PaGLV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaG\x9E\x81\x84aG*V[\x90P\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15aG\xDDW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90PaG\xC2V[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_aH\x02\x82aG\xA6V[aH\x0C\x81\x85aG\xB0V[\x93PaH\x1C\x81\x85` \x86\x01aG\xC0V[aH%\x81aG\xE8V[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaHH\x81\x84aG\xF8V[\x90P\x92\x91PPV[_\x80\x83`\x1F\x84\x01\x12aHeWaHdaEOV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aH\x82WaH\x81aESV[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aH\x9EWaH\x9DaEWV[[\x92P\x92\x90PV[_\x80_\x80`@\x85\x87\x03\x12\x15aH\xBDWaH\xBCaE\x14V[[_\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aH\xDAWaH\xD9aE\x18V[[aH\xE6\x87\x82\x88\x01aHPV[\x94P\x94PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aI\tWaI\x08aE\x18V[[aI\x15\x87\x82\x88\x01aE[V[\x92P\x92PP\x92\x95\x91\x94P\x92PV[_\x81\x15\x15\x90P\x91\x90PV[aI7\x81aI#V[\x82RPPV[_` \x82\x01\x90PaIP_\x83\x01\x84aI.V[\x92\x91PPV[aI_\x81aF\xE7V[\x81\x14aIiW_\x80\xFD[PV[_\x815\x90PaIz\x81aIVV[\x92\x91PPV[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[aI\xBA\x82aG\xE8V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aI\xD9WaI\xD8aI\x84V[[\x80`@RPPPV[_aI\xEBaE\x0BV[\x90PaI\xF7\x82\x82aI\xB1V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aJ\x16WaJ\x15aI\x84V[[aJ\x1F\x82aG\xE8V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_aJLaJG\x84aI\xFCV[aI\xE2V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15aJhWaJgaI\x80V[[aJs\x84\x82\x85aJ,V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aJ\x8FWaJ\x8EaEOV[[\x815aJ\x9F\x84\x82` \x86\x01aJ:V[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15aJ\xBEWaJ\xBDaE\x14V[[_aJ\xCB\x85\x82\x86\x01aIlV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aJ\xECWaJ\xEBaE\x18V[[aJ\xF8\x85\x82\x86\x01aJ{V[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[aK\x14\x81aK\x02V[\x82RPPV[_` \x82\x01\x90PaK-_\x83\x01\x84aK\x0BV[\x92\x91PPV[_\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[aKg\x81aK3V[\x82RPPV[aKv\x81aE\x1CV[\x82RPPV[aK\x85\x81aF\xE7V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[aK\xBD\x81aE\x1CV[\x82RPPV[_aK\xCE\x83\x83aK\xB4V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aK\xF0\x82aK\x8BV[aK\xFA\x81\x85aK\x95V[\x93PaL\x05\x83aK\xA5V[\x80_[\x83\x81\x10\x15aL5W\x81QaL\x1C\x88\x82aK\xC3V[\x97PaL'\x83aK\xDAV[\x92PP`\x01\x81\x01\x90PaL\x08V[P\x85\x93PPPP\x92\x91PPV[_`\xE0\x82\x01\x90PaLU_\x83\x01\x8AaK^V[\x81\x81\x03` \x83\x01RaLg\x81\x89aG\xF8V[\x90P\x81\x81\x03`@\x83\x01RaL{\x81\x88aG\xF8V[\x90PaL\x8A``\x83\x01\x87aKmV[aL\x97`\x80\x83\x01\x86aK|V[aL\xA4`\xA0\x83\x01\x85aK\x0BV[\x81\x81\x03`\xC0\x83\x01RaL\xB6\x81\x84aK\xE6V[\x90P\x98\x97PPPPPPPPV[_\x80\x83`\x1F\x84\x01\x12aL\xD9WaL\xD8aEOV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aL\xF6WaL\xF5aESV[[` \x83\x01\x91P\x83`@\x82\x02\x83\x01\x11\x15aM\x12WaM\x11aEWV[[\x92P\x92\x90PV[_\x80\xFD[_`@\x82\x84\x03\x12\x15aM2WaM1aM\x19V[[\x81\x90P\x92\x91PPV[_`@\x82\x84\x03\x12\x15aMPWaMOaM\x19V[[\x81\x90P\x92\x91PPV[_\x80_\x80_\x80_\x80_\x80_a\x01\0\x8C\x8E\x03\x12\x15aMyWaMxaE\x14V[[_\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aM\x96WaM\x95aE\x18V[[aM\xA2\x8E\x82\x8F\x01aL\xC4V[\x9BP\x9BPP` aM\xB5\x8E\x82\x8F\x01aM\x1DV[\x99PP``\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aM\xD6WaM\xD5aE\x18V[[aM\xE2\x8E\x82\x8F\x01aM;V[\x98PP`\x80aM\xF3\x8E\x82\x8F\x01aIlV[\x97PP`\xA0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aN\x14WaN\x13aE\x18V[[aN \x8E\x82\x8F\x01aE[V[\x96P\x96PP`\xC0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aNCWaNBaE\x18V[[aNO\x8E\x82\x8F\x01aE[V[\x94P\x94PP`\xE0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aNrWaNqaE\x18V[[aN~\x8E\x82\x8F\x01aE[V[\x92P\x92PP\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[_\x80_\x80_``\x86\x88\x03\x12\x15aN\xACWaN\xABaE\x14V[[_aN\xB9\x88\x82\x89\x01aIlV[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aN\xDAWaN\xD9aE\x18V[[aN\xE6\x88\x82\x89\x01aL\xC4V[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aO\tWaO\x08aE\x18V[[aO\x15\x88\x82\x89\x01aE[V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[_` \x82\x01\x90PaO7_\x83\x01\x84aK|V[\x92\x91PPV[aOF\x81aI#V[\x81\x14aOPW_\x80\xFD[PV[_\x81Q\x90PaOa\x81aO=V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aO|WaO{aE\x14V[[_aO\x89\x84\x82\x85\x01aOSV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[_` \x82\x01\x90PaO\xD2_\x83\x01\x84aKmV[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80aP\x1CW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aP/WaP.aO\xD8V[[P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_aPl\x82aE\x1CV[\x91PaPw\x83aE\x1CV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15aP\x8FWaP\x8EaP5V[[\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aP\xB0\x83\x85aP\x95V[\x93PaP\xBD\x83\x85\x84aJ,V[aP\xC6\x83aG\xE8V[\x84\x01\x90P\x93\x92PPPV[_`\x80\x82\x01\x90PaP\xE4_\x83\x01\x8AaKmV[\x81\x81\x03` \x83\x01RaP\xF7\x81\x88\x8AaP\xA5V[\x90P\x81\x81\x03`@\x83\x01RaQ\x0C\x81\x86\x88aP\xA5V[\x90P\x81\x81\x03``\x83\x01RaQ!\x81\x84\x86aP\xA5V[\x90P\x98\x97PPPPPPPPV[_\x81\x90P\x92\x91PPV[_aQC\x82aG\xA6V[aQM\x81\x85aQ/V[\x93PaQ]\x81\x85` \x86\x01aG\xC0V[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aQ\x9D`\x02\x83aQ/V[\x91PaQ\xA8\x82aQiV[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aQ\xE7`\x01\x83aQ/V[\x91PaQ\xF2\x82aQ\xB3V[`\x01\x82\x01\x90P\x91\x90PV[_aR\x08\x82\x87aQ9V[\x91PaR\x13\x82aQ\x91V[\x91PaR\x1F\x82\x86aQ9V[\x91PaR*\x82aQ\xDBV[\x91PaR6\x82\x85aQ9V[\x91PaRA\x82aQ\xDBV[\x91PaRM\x82\x84aQ9V[\x91P\x81\x90P\x95\x94PPPPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[aRw\x81aR[V[\x82RPPV[_` \x82\x01\x90PaR\x90_\x83\x01\x84aRnV[\x92\x91PPV[_\x81Q\x90PaR\xA4\x81aIVV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aR\xBFWaR\xBEaE\x14V[[_aR\xCC\x84\x82\x85\x01aR\x96V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x82\x90P\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02aSh\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82aS-V[aSr\x86\x83aS-V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_aS\xADaS\xA8aS\xA3\x84aE\x1CV[aS\x8AV[aE\x1CV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[aS\xC6\x83aS\x93V[aS\xDAaS\xD2\x82aS\xB4V[\x84\x84TaS9V[\x82UPPPPV[_\x90V[aS\xEEaS\xE2V[aS\xF9\x81\x84\x84aS\xBDV[PPPV[[\x81\x81\x10\x15aT\x1CWaT\x11_\x82aS\xE6V[`\x01\x81\x01\x90PaS\xFFV[PPV[`\x1F\x82\x11\x15aTaWaT2\x81aS\x0CV[aT;\x84aS\x1EV[\x81\x01` \x85\x10\x15aTJW\x81\x90P[aT^aTV\x85aS\x1EV[\x83\x01\x82aS\xFEV[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_aT\x81_\x19\x84`\x08\x02aTfV[\x19\x80\x83\x16\x91PP\x92\x91PPV[_aT\x99\x83\x83aTrV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[aT\xB3\x83\x83aS\x02V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aT\xCCWaT\xCBaI\x84V[[aT\xD6\x82TaP\x05V[aT\xE1\x82\x82\x85aT V[_`\x1F\x83\x11`\x01\x81\x14aU\x0EW_\x84\x15aT\xFCW\x82\x87\x015\x90P[aU\x06\x85\x82aT\x8EV[\x86UPaUmV[`\x1F\x19\x84\x16aU\x1C\x86aS\x0CV[_[\x82\x81\x10\x15aUCW\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PaU\x1EV[\x86\x83\x10\x15aU`W\x84\x89\x015aU\\`\x1F\x89\x16\x82aTrV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[_`\x80\x82\x01\x90P\x81\x81\x03_\x83\x01RaU\x8F\x81\x89\x8BaP\xA5V[\x90P\x81\x81\x03` \x83\x01RaU\xA4\x81\x87\x89aP\xA5V[\x90PaU\xB3`@\x83\x01\x86aK|V[\x81\x81\x03``\x83\x01RaU\xC6\x81\x84\x86aP\xA5V[\x90P\x98\x97PPPPPPPPV[_\x81T\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81TaV\x1C\x81aP\x05V[aV&\x81\x86aV\0V[\x94P`\x01\x82\x16_\x81\x14aV@W`\x01\x81\x14aVVWaV\x88V[`\xFF\x19\x83\x16\x86R\x81\x15\x15` \x02\x86\x01\x93PaV\x88V[aV_\x85aS\x0CV[_[\x83\x81\x10\x15aV\x80W\x81T\x81\x89\x01R`\x01\x82\x01\x91P` \x81\x01\x90PaVaV[\x80\x88\x01\x95PPP[PPP\x92\x91PPV[_aV\x9C\x83\x83aV\x10V[\x90P\x92\x91PPV[_`\x01\x82\x01\x90P\x91\x90PV[_aV\xBA\x82aU\xD4V[aV\xC4\x81\x85aU\xDEV[\x93P\x83` \x82\x02\x85\x01aV\xD6\x85aU\xEEV[\x80_[\x85\x81\x10\x15aW\x10W\x84\x84\x03\x89R\x81aV\xF1\x85\x82aV\x91V[\x94PaV\xFC\x83aV\xA4V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaV\xD9V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_``\x82\x01\x90P\x81\x81\x03_\x83\x01RaW;\x81\x87\x89aP\xA5V[\x90P\x81\x81\x03` \x83\x01RaWO\x81\x86aV\xB0V[\x90P\x81\x81\x03`@\x83\x01RaWd\x81\x84\x86aP\xA5V[\x90P\x96\x95PPPPPPV[\x7FEIP712: Uninitialized\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aW\xA4`\x15\x83aG\xB0V[\x91PaW\xAF\x82aWpV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaW\xD1\x81aW\x98V[\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x80\xFD[\x82\x81\x837PPPV[_aX\0\x83\x85aW\xD8V[\x93P\x7F\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15aX3WaX2aW\xE8V[[` \x83\x02\x92PaXD\x83\x85\x84aW\xECV[\x82\x84\x01\x90P\x93\x92PPPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaXi\x81\x84\x86aW\xF5V[\x90P\x93\x92PPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aX\x8CWaX\x8BaI\x84V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x80\xFD[_\x80\xFD[aX\xAE\x81aK\x02V[\x81\x14aX\xB8W_\x80\xFD[PV[_\x81Q\x90PaX\xC9\x81aX\xA5V[\x92\x91PPV[_\x81Q\x90PaX\xDD\x81aE%V[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aX\xFDWaX\xFCaI\x84V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_aY aY\x1B\x84aX\xE3V[aI\xE2V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15aYCWaYBaEWV[[\x83[\x81\x81\x10\x15aYlW\x80aYX\x88\x82aR\x96V[\x84R` \x84\x01\x93PP` \x81\x01\x90PaYEV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aY\x8AWaY\x89aEOV[[\x81QaY\x9A\x84\x82` \x86\x01aY\x0EV[\x91PP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15aY\xB8WaY\xB7aX\x9DV[[aY\xC2`\x80aI\xE2V[\x90P_aY\xD1\x84\x82\x85\x01aX\xBBV[_\x83\x01RP` aY\xE4\x84\x82\x85\x01aX\xCFV[` \x83\x01RP`@aY\xF8\x84\x82\x85\x01aX\xBBV[`@\x83\x01RP``\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZ\x1CWaZ\x1BaX\xA1V[[aZ(\x84\x82\x85\x01aYvV[``\x83\x01RP\x92\x91PPV[_aZFaZA\x84aXrV[aI\xE2V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15aZiWaZhaEWV[[\x83[\x81\x81\x10\x15aZ\xB0W\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZ\x8EWaZ\x8DaEOV[[\x80\x86\x01aZ\x9B\x89\x82aY\xA3V[\x85R` \x85\x01\x94PPP` \x81\x01\x90PaZkV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aZ\xCEWaZ\xCDaEOV[[\x81QaZ\xDE\x84\x82` \x86\x01aZ4V[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15aZ\xFCWaZ\xFBaE\x14V[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a[\x19Wa[\x18aE\x18V[[a[%\x84\x82\x85\x01aZ\xBAV[\x91PP\x92\x91PPV[_a[8\x82aE\x1CV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a[jWa[iaP5V[[`\x01\x82\x01\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a[\xA7\x81aK\x02V[\x82RPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a[\xC7\x82aF\x9FV[a[\xD1\x81\x85a[\xADV[\x93Pa[\xDC\x83aF\xB9V[\x80_[\x83\x81\x10\x15a\\\x0CW\x81Qa[\xF3\x88\x82aG\x07V[\x97Pa[\xFE\x83aG\x1EV[\x92PP`\x01\x81\x01\x90Pa[\xDFV[P\x85\x93PPPP\x92\x91PPV[_`\x80\x83\x01_\x83\x01Qa\\._\x86\x01\x82a[\x9EV[P` \x83\x01Qa\\A` \x86\x01\x82aK\xB4V[P`@\x83\x01Qa\\T`@\x86\x01\x82a[\x9EV[P``\x83\x01Q\x84\x82\x03``\x86\x01Ra\\l\x82\x82a[\xBDV[\x91PP\x80\x91PP\x92\x91PPV[_a\\\x84\x83\x83a\\\x19V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\\\xA2\x82a[uV[a\\\xAC\x81\x85a[\x7FV[\x93P\x83` \x82\x02\x85\x01a\\\xBE\x85a[\x8FV[\x80_[\x85\x81\x10\x15a\\\xF9W\x84\x84\x03\x89R\x81Qa\\\xDA\x85\x82a\\yV[\x94Pa\\\xE5\x83a\\\x8CV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa\\\xC1V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra]#\x81\x86a\\\x98V[\x90P\x81\x81\x03` \x83\x01Ra]8\x81\x84\x86aP\xA5V[\x90P\x94\x93PPPPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12a]jWa]ia]BV[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a]\x8CWa]\x8Ba]FV[[` \x83\x01\x92P` \x82\x026\x03\x83\x13\x15a]\xA8Wa]\xA7a]JV[[P\x92P\x92\x90PV[_`\xFF\x82\x16\x90P\x91\x90PV[_a]\xD6a]\xD1a]\xCC\x84a]\xB0V[aS\x8AV[aE\x1CV[\x90P\x91\x90PV[a]\xE6\x81a]\xBCV[\x82RPPV[_`@\x82\x01\x90Pa]\xFF_\x83\x01\x85a]\xDDV[a^\x0C` \x83\x01\x84aKmV[\x93\x92PPPV[_`@\x82\x84\x03\x12\x15a^(Wa^'aX\x9DV[[a^2`@aI\xE2V[\x90P_a^A\x84\x82\x85\x01aE;V[_\x83\x01RP` a^T\x84\x82\x85\x01aE;V[` \x83\x01RP\x92\x91PPV[_`@\x82\x84\x03\x12\x15a^uWa^taE\x14V[[_a^\x82\x84\x82\x85\x01a^\x13V[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[_a^\xA2` \x84\x01\x84aIlV[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a^\xC1\x83\x85aF\xA9V[\x93Pa^\xCC\x82a^\x8BV[\x80_[\x85\x81\x10\x15a_\x04Wa^\xE1\x82\x84a^\x94V[a^\xEB\x88\x82aG\x07V[\x97Pa^\xF6\x83a^\xAAV[\x92PP`\x01\x81\x01\x90Pa^\xCFV[P\x85\x92PPP\x93\x92PPPV[_`@\x82\x01\x90Pa_$_\x83\x01\x86aK|V[\x81\x81\x03` \x83\x01Ra_7\x81\x84\x86a^\xB6V[\x90P\x94\x93PPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_a_e\x83\x83a[\x9EV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a_\x87\x82a_AV[a_\x91\x81\x85aW\xD8V[\x93Pa_\x9C\x83a_KV[\x80_[\x83\x81\x10\x15a_\xCCW\x81Qa_\xB3\x88\x82a_ZV[\x97Pa_\xBE\x83a_qV[\x92PP`\x01\x81\x01\x90Pa_\x9FV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra_\xF1\x81\x84a_}V[\x90P\x92\x91PPV[_\x81Q\x90P\x91\x90PV[a`\x0C\x82a_\xF9V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a`%Wa`$aI\x84V[[a`/\x82TaP\x05V[a`:\x82\x82\x85aT V[_` \x90P`\x1F\x83\x11`\x01\x81\x14a`kW_\x84\x15a`YW\x82\x87\x01Q\x90P[a`c\x85\x82aT\x8EV[\x86UPa`\xCAV[`\x1F\x19\x84\x16a`y\x86aS\x0CV[_[\x82\x81\x10\x15a`\xA0W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa`{V[\x86\x83\x10\x15a`\xBDW\x84\x89\x01Qa`\xB9`\x1F\x89\x16\x82aTrV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_`\x80\x82\x01\x90P\x81\x81\x03_\x83\x01Ra`\xEA\x81\x89a\\\x98V[\x90Pa`\xF9` \x83\x01\x88aK|V[\x81\x81\x03`@\x83\x01Raa\x0C\x81\x86\x88aP\xA5V[\x90P\x81\x81\x03``\x83\x01Raa!\x81\x84\x86aP\xA5V[\x90P\x97\x96PPPPPPPV[_`@\x82\x01\x90PaaA_\x83\x01\x85aK\x0BV[aaN` \x83\x01\x84aK|V[\x93\x92PPPV[_` \x82\x84\x03\x12\x15aajWaaiaE\x14V[[_aaw\x84\x82\x85\x01aIlV[\x91PP\x92\x91PPV[_\x81\x90P\x92\x91PPV[aa\x93\x81aK\x02V[\x82RPPV[_aa\xA4\x83\x83aa\x8AV[` \x83\x01\x90P\x92\x91PPV[_aa\xBA\x82a_AV[aa\xC4\x81\x85aa\x80V[\x93Paa\xCF\x83a_KV[\x80_[\x83\x81\x10\x15aa\xFFW\x81Qaa\xE6\x88\x82aa\x99V[\x97Paa\xF1\x83a_qV[\x92PP`\x01\x81\x01\x90Paa\xD2V[P\x85\x93PPPP\x92\x91PPV[_ab\x17\x82\x84aa\xB0V[\x91P\x81\x90P\x92\x91PPV[_\x81\x90P\x92\x91PPV[_ab6\x82a_\xF9V[ab@\x81\x85ab\"V[\x93PabP\x81\x85` \x86\x01aG\xC0V[\x80\x84\x01\x91PP\x92\x91PPV[_abg\x82\x84ab,V[\x91P\x81\x90P\x92\x91PPV[_`\xA0\x82\x01\x90Pab\x85_\x83\x01\x88aK\x0BV[ab\x92` \x83\x01\x87aK\x0BV[ab\x9F`@\x83\x01\x86aK\x0BV[ab\xAC``\x83\x01\x85aK\x0BV[ab\xB9`\x80\x83\x01\x84aK\x0BV[\x96\x95PPPPPPV[_`@\x82\x01\x90Pab\xD6_\x83\x01\x85aKmV[ab\xE3` \x83\x01\x84aK|V[\x93\x92PPPV[_` \x82\x84\x03\x12\x15ab\xFFWab\xFEaE\x14V[[_ac\x0C\x84\x82\x85\x01aX\xCFV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15acWWacVaE\x14V[[_acd\x84\x82\x85\x01aX\xBBV[\x91PP\x92\x91PPV[_`\x80\x82\x01\x90Pac\x80_\x83\x01\x87aK\x0BV[ac\x8D` \x83\x01\x86aK\x0BV[ac\x9A`@\x83\x01\x85aK\x0BV[ac\xA7``\x83\x01\x84aK\x0BV[\x95\x94PPPPPV[_ac\xBA\x82aE\x1CV[\x91Pac\xC5\x83aE\x1CV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15ac\xDDWac\xDCaP5V[[\x92\x91PPV[_`@\x82\x01\x90Pac\xF6_\x83\x01\x85aKmV[ad\x03` \x83\x01\x84aKmV[\x93\x92PPPV[_`\x80\x83\x01_\x83\x01Qad\x1F_\x86\x01\x82a[\x9EV[P` \x83\x01Qad2` \x86\x01\x82aK\xB4V[P`@\x83\x01QadE`@\x86\x01\x82a[\x9EV[P``\x83\x01Q\x84\x82\x03``\x86\x01Rad]\x82\x82a[\xBDV[\x91PP\x80\x91PP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Rad\x82\x81\x85ad\nV[\x90P\x81\x81\x03` \x83\x01Rad\x96\x81\x84ad\nV[\x90P\x93\x92PPPV[_a\xFF\xFF\x82\x16\x90P\x91\x90PV[_ad\xC6ad\xC1ad\xBC\x84ad\x9FV[aS\x8AV[aE\x1CV[\x90P\x91\x90PV[ad\xD6\x81ad\xACV[\x82RPPV[_`@\x82\x01\x90Pad\xEF_\x83\x01\x85ad\xCDV[ad\xFC` \x83\x01\x84aKmV[\x93\x92PPPV[_ae\r\x82aE\x1CV[\x91Pae\x18\x83aE\x1CV[\x92P\x82\x82\x02ae&\x81aE\x1CV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17ae=Wae<aP5V[[P\x92\x91PPV[`@\x82\x01_\x82\x01QaeX_\x85\x01\x82aK\xB4V[P` \x82\x01Qaek` \x85\x01\x82aK\xB4V[PPPPV[_``\x82\x01\x90Pae\x84_\x83\x01\x85aKmV[ae\x91` \x83\x01\x84aeDV[\x93\x92PPPV[_`@\x82\x01\x90Pae\xAB_\x83\x01\x85aK|V[\x81\x81\x03` \x83\x01Rae\xBD\x81\x84aG*V[\x90P\x93\x92PPPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Rae\xDF\x81\x84\x86aP\xA5V[\x90P\x93\x92PPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15af\x02Waf\x01aI\x84V[[af\x0B\x82aG\xE8V[\x90P` \x81\x01\x90P\x91\x90PV[_af*af%\x84ae\xE8V[aI\xE2V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15afFWafEaI\x80V[[afQ\x84\x82\x85aG\xC0V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12afmWaflaEOV[[\x81Qaf}\x84\x82` \x86\x01af\x18V[\x91PP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15af\x9BWaf\x9AaX\x9DV[[af\xA5`\x80aI\xE2V[\x90P_af\xB4\x84\x82\x85\x01aR\x96V[_\x83\x01RP` af\xC7\x84\x82\x85\x01aR\x96V[` \x83\x01RP`@\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15af\xEBWaf\xEAaX\xA1V[[af\xF7\x84\x82\x85\x01afYV[`@\x83\x01RP``\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ag\x1BWag\x1AaX\xA1V[[ag'\x84\x82\x85\x01afYV[``\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15agHWagGaE\x14V[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ageWagdaE\x18V[[agq\x84\x82\x85\x01af\x86V[\x91PP\x92\x91PPV[_`@\x82\x01\x90Pag\x8D_\x83\x01\x85aK|V[ag\x9A` \x83\x01\x84aK|V[\x93\x92PPPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[`\x1F\x82\x11\x15ag\xF4Wag\xC5\x81ag\xA1V[ag\xCE\x84aS\x1EV[\x81\x01` \x85\x10\x15ag\xDDW\x81\x90P[ag\xF1ag\xE9\x85aS\x1EV[\x83\x01\x82aS\xFEV[PP[PPPV[ah\x02\x82aG\xA6V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ah\x1BWah\x1AaI\x84V[[ah%\x82TaP\x05V[ah0\x82\x82\x85ag\xB3V[_` \x90P`\x1F\x83\x11`\x01\x81\x14ahaW_\x84\x15ahOW\x82\x87\x01Q\x90P[ahY\x85\x82aT\x8EV[\x86UPah\xC0V[`\x1F\x19\x84\x16aho\x86ag\xA1V[_[\x82\x81\x10\x15ah\x96W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PahqV[\x86\x83\x10\x15ah\xB3W\x84\x89\x01Qah\xAF`\x1F\x89\x16\x82aTrV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[ah\xD1\x81a]\xB0V[\x82RPPV[_` \x82\x01\x90Pah\xEA_\x83\x01\x84ah\xC8V[\x92\x91PPV[`T\x81\x10ai\x01Wai\0aO\x92V[[PV[_\x81\x90Pai\x11\x82ah\xF0V[\x91\x90PV[_ai \x82ai\x04V[\x90P\x91\x90PV[ai0\x81ai\x16V[\x82RPPV[_` \x82\x01\x90PaiI_\x83\x01\x84ai'V[\x92\x91PPV[_\x81\x90P\x92\x91PPV[aib\x81aF\xE7V[\x82RPPV[_ais\x83\x83aiYV[` \x83\x01\x90P\x92\x91PPV[_ai\x89\x82aF\x9FV[ai\x93\x81\x85aiOV[\x93Pai\x9E\x83aF\xB9V[\x80_[\x83\x81\x10\x15ai\xCEW\x81Qai\xB5\x88\x82aihV[\x97Pai\xC0\x83aG\x1EV[\x92PP`\x01\x81\x01\x90Pai\xA1V[P\x85\x93PPPP\x92\x91PPV[_ai\xE6\x82\x84ai\x7FV[\x91P\x81\x90P\x92\x91PPV[_`\xC0\x82\x01\x90Paj\x04_\x83\x01\x89aK\x0BV[aj\x11` \x83\x01\x88aK\x0BV[aj\x1E`@\x83\x01\x87aK\x0BV[aj+``\x83\x01\x86aKmV[aj8`\x80\x83\x01\x85aKmV[ajE`\xA0\x83\x01\x84aK\x0BV[\x97\x96PPPPPPPV[_`\xA0\x82\x01\x90Pajc_\x83\x01\x88aK\x0BV[ajp` \x83\x01\x87aK\x0BV[aj}`@\x83\x01\x86aK\x0BV[aj\x8A``\x83\x01\x85aKmV[aj\x97`\x80\x83\x01\x84aK|V[\x96\x95PPPPPPV[_`\x80\x82\x01\x90Paj\xB4_\x83\x01\x87aK\x0BV[aj\xC1` \x83\x01\x86ah\xC8V[aj\xCE`@\x83\x01\x85aK\x0BV[aj\xDB``\x83\x01\x84aK\x0BV[\x95\x94PPPPPV\xFEUserDecryptResponseVerification(bytes publicKey,bytes32[] ctHandles,bytes userDecryptedShare,bytes extraData)PublicDecryptVerification(bytes32[] ctHandles,bytes decryptedResult,bytes extraData)UserDecryptRequestVerification(bytes publicKey,address[] contractAddresses,uint256 startTimestamp,uint256 durationDays,bytes extraData)",
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
    /**Custom error with signature `AccountNotAllowedToUseCiphertext(bytes32,address)` and selector `0x160a2b4b`.
```solidity
error AccountNotAllowedToUseCiphertext(bytes32 ctHandle, address accountAddress);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AccountNotAllowedToUseCiphertext {
        #[allow(missing_docs)]
        pub ctHandle: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub accountAddress: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<AccountNotAllowedToUseCiphertext>
        for UnderlyingRustTuple<'_> {
            fn from(value: AccountNotAllowedToUseCiphertext) -> Self {
                (value.ctHandle, value.accountAddress)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for AccountNotAllowedToUseCiphertext {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    ctHandle: tuple.0,
                    accountAddress: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AccountNotAllowedToUseCiphertext {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AccountNotAllowedToUseCiphertext(bytes32,address)";
            const SELECTOR: [u8; 4] = [22u8, 10u8, 43u8, 75u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.ctHandle),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.accountAddress,
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
    /**Custom error with signature `ContractAddressesMaxLengthExceeded(uint256,uint256)` and selector `0xaf1f0495`.
```solidity
error ContractAddressesMaxLengthExceeded(uint256 maxLength, uint256 actualLength);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ContractAddressesMaxLengthExceeded {
        #[allow(missing_docs)]
        pub maxLength: alloy::sol_types::private::primitives::aliases::U256,
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
            const SIGNATURE: &'static str = "ContractAddressesMaxLengthExceeded(uint256,uint256)";
            const SELECTOR: [u8; 4] = [175u8, 31u8, 4u8, 149u8];
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
    /**Custom error with signature `CoprocessorSignerDoesNotMatchTxSender(address,address)` and selector `0xe134bf62`.
```solidity
error CoprocessorSignerDoesNotMatchTxSender(address signerAddress, address txSenderAddress);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CoprocessorSignerDoesNotMatchTxSender {
        #[allow(missing_docs)]
        pub signerAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub txSenderAddress: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<CoprocessorSignerDoesNotMatchTxSender>
        for UnderlyingRustTuple<'_> {
            fn from(value: CoprocessorSignerDoesNotMatchTxSender) -> Self {
                (value.signerAddress, value.txSenderAddress)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for CoprocessorSignerDoesNotMatchTxSender {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signerAddress: tuple.0,
                    txSenderAddress: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CoprocessorSignerDoesNotMatchTxSender {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CoprocessorSignerDoesNotMatchTxSender(address,address)";
            const SELECTOR: [u8; 4] = [225u8, 52u8, 191u8, 98u8];
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
                        &self.signerAddress,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.txSenderAddress,
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
    /**Custom error with signature `HostChainNotRegistered(uint256)` and selector `0xb6679c3b`.
```solidity
error HostChainNotRegistered(uint256 chainId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct HostChainNotRegistered {
        #[allow(missing_docs)]
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<HostChainNotRegistered> for UnderlyingRustTuple<'_> {
            fn from(value: HostChainNotRegistered) -> Self {
                (value.chainId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for HostChainNotRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { chainId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for HostChainNotRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "HostChainNotRegistered(uint256)";
            const SELECTOR: [u8; 4] = [182u8, 103u8, 156u8, 59u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.chainId),
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
    /**Custom error with signature `KmsSignerDoesNotMatchTxSender(address,address)` and selector `0x0d86f521`.
```solidity
error KmsSignerDoesNotMatchTxSender(address signerAddress, address txSenderAddress);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsSignerDoesNotMatchTxSender {
        #[allow(missing_docs)]
        pub signerAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub txSenderAddress: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<KmsSignerDoesNotMatchTxSender>
        for UnderlyingRustTuple<'_> {
            fn from(value: KmsSignerDoesNotMatchTxSender) -> Self {
                (value.signerAddress, value.txSenderAddress)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for KmsSignerDoesNotMatchTxSender {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signerAddress: tuple.0,
                    txSenderAddress: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KmsSignerDoesNotMatchTxSender {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KmsSignerDoesNotMatchTxSender(address,address)";
            const SELECTOR: [u8; 4] = [13u8, 134u8, 245u8, 33u8];
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
                        &self.signerAddress,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.txSenderAddress,
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
    /**Custom error with signature `NotCoprocessorSigner(address)` and selector `0x26cd75dc`.
```solidity
error NotCoprocessorSigner(address signerAddress);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotCoprocessorSigner {
        #[allow(missing_docs)]
        pub signerAddress: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<NotCoprocessorSigner> for UnderlyingRustTuple<'_> {
            fn from(value: NotCoprocessorSigner) -> Self {
                (value.signerAddress,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotCoprocessorSigner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { signerAddress: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotCoprocessorSigner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotCoprocessorSigner(address)";
            const SELECTOR: [u8; 4] = [38u8, 205u8, 117u8, 220u8];
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
                        &self.signerAddress,
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
    /**Custom error with signature `NotCoprocessorTxSender(address)` and selector `0x52d725f5`.
```solidity
error NotCoprocessorTxSender(address txSenderAddress);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotCoprocessorTxSender {
        #[allow(missing_docs)]
        pub txSenderAddress: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<NotCoprocessorTxSender> for UnderlyingRustTuple<'_> {
            fn from(value: NotCoprocessorTxSender) -> Self {
                (value.txSenderAddress,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotCoprocessorTxSender {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { txSenderAddress: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotCoprocessorTxSender {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotCoprocessorTxSender(address)";
            const SELECTOR: [u8; 4] = [82u8, 215u8, 37u8, 245u8];
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
                        &self.txSenderAddress,
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
    /**Custom error with signature `NotCustodianSigner(address)` and selector `0x391672a7`.
```solidity
error NotCustodianSigner(address signerAddress);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotCustodianSigner {
        #[allow(missing_docs)]
        pub signerAddress: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<NotCustodianSigner> for UnderlyingRustTuple<'_> {
            fn from(value: NotCustodianSigner) -> Self {
                (value.signerAddress,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotCustodianSigner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { signerAddress: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotCustodianSigner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotCustodianSigner(address)";
            const SELECTOR: [u8; 4] = [57u8, 22u8, 114u8, 167u8];
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
                        &self.signerAddress,
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
    /**Custom error with signature `NotCustodianTxSender(address)` and selector `0xf924a0cf`.
```solidity
error NotCustodianTxSender(address txSenderAddress);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotCustodianTxSender {
        #[allow(missing_docs)]
        pub txSenderAddress: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<NotCustodianTxSender> for UnderlyingRustTuple<'_> {
            fn from(value: NotCustodianTxSender) -> Self {
                (value.txSenderAddress,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotCustodianTxSender {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { txSenderAddress: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotCustodianTxSender {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotCustodianTxSender(address)";
            const SELECTOR: [u8; 4] = [249u8, 36u8, 160u8, 207u8];
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
                        &self.txSenderAddress,
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
    /**Custom error with signature `NotKmsSigner(address)` and selector `0x2a7c6ef6`.
```solidity
error NotKmsSigner(address signerAddress);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotKmsSigner {
        #[allow(missing_docs)]
        pub signerAddress: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<NotKmsSigner> for UnderlyingRustTuple<'_> {
            fn from(value: NotKmsSigner) -> Self {
                (value.signerAddress,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotKmsSigner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { signerAddress: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotKmsSigner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotKmsSigner(address)";
            const SELECTOR: [u8; 4] = [42u8, 124u8, 110u8, 246u8];
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
                        &self.signerAddress,
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
    /**Custom error with signature `NotKmsTxSender(address)` and selector `0xaee86323`.
```solidity
error NotKmsTxSender(address txSenderAddress);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotKmsTxSender {
        #[allow(missing_docs)]
        pub txSenderAddress: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<NotKmsTxSender> for UnderlyingRustTuple<'_> {
            fn from(value: NotKmsTxSender) -> Self {
                (value.txSenderAddress,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotKmsTxSender {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { txSenderAddress: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotKmsTxSender {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotKmsTxSender(address)";
            const SELECTOR: [u8; 4] = [174u8, 232u8, 99u8, 35u8];
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
                        &self.txSenderAddress,
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
    /**Custom error with signature `PublicDecryptNotAllowed(bytes32)` and selector `0x4331a85d`.
```solidity
error PublicDecryptNotAllowed(bytes32 ctHandle);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PublicDecryptNotAllowed {
        #[allow(missing_docs)]
        pub ctHandle: alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<PublicDecryptNotAllowed> for UnderlyingRustTuple<'_> {
            fn from(value: PublicDecryptNotAllowed) -> Self {
                (value.ctHandle,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PublicDecryptNotAllowed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { ctHandle: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for PublicDecryptNotAllowed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PublicDecryptNotAllowed(bytes32)";
            const SELECTOR: [u8; 4] = [67u8, 49u8, 168u8, 93u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.ctHandle),
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
    /**Event with signature `PublicDecryptionResponseCall(uint256,bytes,bytes,address,bytes)` and selector `0x4d7b1dba49e9e846215e1621f5737c81d8614c4f268494d8b787632c4e59f0e5`.
```solidity
event PublicDecryptionResponseCall(uint256 indexed decryptionId, bytes decryptedResult, bytes signature, address kmsTxSender, bytes extraData);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PublicDecryptionResponseCall {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub decryptedResult: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub kmsTxSender: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for PublicDecryptionResponseCall {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "PublicDecryptionResponseCall(uint256,bytes,bytes,address,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                77u8, 123u8, 29u8, 186u8, 73u8, 233u8, 232u8, 70u8, 33u8, 94u8, 22u8,
                33u8, 245u8, 115u8, 124u8, 129u8, 216u8, 97u8, 76u8, 79u8, 38u8, 132u8,
                148u8, 216u8, 183u8, 135u8, 99u8, 44u8, 78u8, 89u8, 240u8, 229u8,
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
                    signature: data.1,
                    kmsTxSender: data.2,
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.decryptedResult,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.kmsTxSender,
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
        impl alloy_sol_types::private::IntoLogData for PublicDecryptionResponseCall {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PublicDecryptionResponseCall> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &PublicDecryptionResponseCall,
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
    /**Event with signature `UserDecryptionResponse(uint256,uint256,bytes,bytes,bytes)` and selector `0x7fcdfb5381917f554a717d0a5470a33f5a49ba6445f05ec43c74c0bc2cc608b2`.
```solidity
event UserDecryptionResponse(uint256 indexed decryptionId, uint256 indexShare, bytes userDecryptedShare, bytes signature, bytes extraData);
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
        pub indexShare: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub userDecryptedShare: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
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
            const SIGNATURE: &'static str = "UserDecryptionResponse(uint256,uint256,bytes,bytes,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                127u8, 205u8, 251u8, 83u8, 129u8, 145u8, 127u8, 85u8, 74u8, 113u8, 125u8,
                10u8, 84u8, 112u8, 163u8, 63u8, 90u8, 73u8, 186u8, 100u8, 69u8, 240u8,
                94u8, 196u8, 60u8, 116u8, 192u8, 188u8, 44u8, 198u8, 8u8, 178u8,
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
                    indexShare: data.0,
                    userDecryptedShare: data.1,
                    signature: data.2,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.indexShare),
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `UserDecryptionResponseThresholdReached(uint256)` and selector `0xe89752be0ecdb68b2a6eb5ef1a891039e0e92ae3c8a62274c5881e48eea1ed25`.
```solidity
event UserDecryptionResponseThresholdReached(uint256 indexed decryptionId);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UserDecryptionResponseThresholdReached {
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for UserDecryptionResponseThresholdReached {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "UserDecryptionResponseThresholdReached(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                232u8, 151u8, 82u8, 190u8, 14u8, 205u8, 182u8, 139u8, 42u8, 110u8, 181u8,
                239u8, 26u8, 137u8, 16u8, 57u8, 224u8, 233u8, 42u8, 227u8, 200u8, 166u8,
                34u8, 116u8, 197u8, 136u8, 30u8, 72u8, 238u8, 161u8, 237u8, 37u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { decryptionId: topics.1 }
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
        impl alloy_sol_types::private::IntoLogData
        for UserDecryptionResponseThresholdReached {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UserDecryptionResponseThresholdReached>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &UserDecryptionResponseThresholdReached,
            ) -> alloy_sol_types::private::LogData {
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
    /**Function with signature `isDecryptionDone(uint256)` and selector `0x58f5b8ab`.
```solidity
function isDecryptionDone(uint256 decryptionId) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isDecryptionDoneCall {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isDecryptionDone(uint256)`](isDecryptionDoneCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isDecryptionDoneReturn {
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
            impl ::core::convert::From<isDecryptionDoneCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isDecryptionDoneCall) -> Self {
                    (value.decryptionId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isDecryptionDoneCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { decryptionId: tuple.0 }
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
            impl ::core::convert::From<isDecryptionDoneReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isDecryptionDoneReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isDecryptionDoneReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isDecryptionDoneCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isDecryptionDone(uint256)";
            const SELECTOR: [u8; 4] = [88u8, 245u8, 184u8, 171u8];
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
                        let r: isDecryptionDoneReturn = r.into();
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
                        let r: isDecryptionDoneReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isPublicDecryptionReady(bytes32[],bytes)` and selector `0x4014c4cd`.
```solidity
function isPublicDecryptionReady(bytes32[] memory ctHandles, bytes memory) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isPublicDecryptionReadyCall {
        #[allow(missing_docs)]
        pub ctHandles: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isPublicDecryptionReady(bytes32[],bytes)`](isPublicDecryptionReadyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isPublicDecryptionReadyReturn {
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
            impl ::core::convert::From<isPublicDecryptionReadyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isPublicDecryptionReadyCall) -> Self {
                    (value.ctHandles, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isPublicDecryptionReadyCall {
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
            impl ::core::convert::From<isPublicDecryptionReadyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isPublicDecryptionReadyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isPublicDecryptionReadyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isPublicDecryptionReadyCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isPublicDecryptionReady(bytes32[],bytes)";
            const SELECTOR: [u8; 4] = [64u8, 20u8, 196u8, 205u8];
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
                        let r: isPublicDecryptionReadyReturn = r.into();
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
                        let r: isPublicDecryptionReadyReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isUserDecryptionReady(address,(bytes32,address)[],bytes)` and selector `0xfbb83259`.
```solidity
function isUserDecryptionReady(address userAddress, CtHandleContractPair[] memory ctHandleContractPairs, bytes memory) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isUserDecryptionReadyCall {
        #[allow(missing_docs)]
        pub userAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub ctHandleContractPairs: alloy::sol_types::private::Vec<
            <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub _2: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isUserDecryptionReady(address,(bytes32,address)[],bytes)`](isUserDecryptionReadyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isUserDecryptionReadyReturn {
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
            impl ::core::convert::From<isUserDecryptionReadyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isUserDecryptionReadyCall) -> Self {
                    (value.userAddress, value.ctHandleContractPairs, value._2)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isUserDecryptionReadyCall {
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
            impl ::core::convert::From<isUserDecryptionReadyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isUserDecryptionReadyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isUserDecryptionReadyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isUserDecryptionReadyCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<CtHandleContractPair>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isUserDecryptionReady(address,(bytes32,address)[],bytes)";
            const SELECTOR: [u8; 4] = [251u8, 184u8, 50u8, 89u8];
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
                        let r: isUserDecryptionReadyReturn = r.into();
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
                        let r: isUserDecryptionReadyReturn = r.into();
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
    /**Function with signature `reinitializeV2()` and selector `0xc4115874`.
```solidity
function reinitializeV2() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reinitializeV2Call;
    ///Container type for the return parameters of the [`reinitializeV2()`](reinitializeV2Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reinitializeV2Return {}
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
            impl ::core::convert::From<reinitializeV2Call> for UnderlyingRustTuple<'_> {
                fn from(value: reinitializeV2Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for reinitializeV2Call {
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
            impl ::core::convert::From<reinitializeV2Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: reinitializeV2Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for reinitializeV2Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl reinitializeV2Return {
            fn _tokenize(
                &self,
            ) -> <reinitializeV2Call as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for reinitializeV2Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = reinitializeV2Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "reinitializeV2()";
            const SELECTOR: [u8; 4] = [196u8, 17u8, 88u8, 116u8];
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
                reinitializeV2Return::_tokenize(ret)
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
        eip712Domain(eip712DomainCall),
        #[allow(missing_docs)]
        getDecryptionConsensusTxSenders(getDecryptionConsensusTxSendersCall),
        #[allow(missing_docs)]
        getVersion(getVersionCall),
        #[allow(missing_docs)]
        initializeFromEmptyProxy(initializeFromEmptyProxyCall),
        #[allow(missing_docs)]
        isDecryptionDone(isDecryptionDoneCall),
        #[allow(missing_docs)]
        isPublicDecryptionReady(isPublicDecryptionReadyCall),
        #[allow(missing_docs)]
        isUserDecryptionReady(isUserDecryptionReadyCall),
        #[allow(missing_docs)]
        pause(pauseCall),
        #[allow(missing_docs)]
        paused(pausedCall),
        #[allow(missing_docs)]
        proxiableUUID(proxiableUUIDCall),
        #[allow(missing_docs)]
        publicDecryptionRequest(publicDecryptionRequestCall),
        #[allow(missing_docs)]
        publicDecryptionResponse(publicDecryptionResponseCall),
        #[allow(missing_docs)]
        reinitializeV2(reinitializeV2Call),
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
            [57u8, 247u8, 56u8, 16u8],
            [63u8, 75u8, 168u8, 58u8],
            [64u8, 20u8, 196u8, 205u8],
            [79u8, 30u8, 242u8, 134u8],
            [82u8, 209u8, 144u8, 45u8],
            [88u8, 245u8, 184u8, 171u8],
            [92u8, 151u8, 90u8, 187u8],
            [111u8, 137u8, 19u8, 188u8],
            [132u8, 86u8, 203u8, 89u8],
            [132u8, 176u8, 25u8, 110u8],
            [173u8, 60u8, 177u8, 204u8],
            [196u8, 17u8, 88u8, 116u8],
            [216u8, 153u8, 143u8, 69u8],
            [241u8, 181u8, 122u8, 219u8],
            [251u8, 184u8, 50u8, 89u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for DecryptionCalls {
        const NAME: &'static str = "DecryptionCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 18usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::UPGRADE_INTERFACE_VERSION(_) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::isDecryptionDone(_) => {
                    <isDecryptionDoneCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isPublicDecryptionReady(_) => {
                    <isPublicDecryptionReadyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isUserDecryptionReady(_) => {
                    <isUserDecryptionReadyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused(_) => <pausedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::proxiableUUID(_) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::publicDecryptionRequest(_) => {
                    <publicDecryptionRequestCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::publicDecryptionResponse(_) => {
                    <publicDecryptionResponseCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::reinitializeV2(_) => {
                    <reinitializeV2Call as alloy_sol_types::SolCall>::SELECTOR
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
                    fn isPublicDecryptionReady(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <isPublicDecryptionReadyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::isPublicDecryptionReady)
                    }
                    isPublicDecryptionReady
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
                    fn isDecryptionDone(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <isDecryptionDoneCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::isDecryptionDone)
                    }
                    isDecryptionDone
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
                    fn reinitializeV2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <reinitializeV2Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::reinitializeV2)
                    }
                    reinitializeV2
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
                    fn isUserDecryptionReady(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <isUserDecryptionReadyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::isUserDecryptionReady)
                    }
                    isUserDecryptionReady
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
                    fn isPublicDecryptionReady(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <isPublicDecryptionReadyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::isPublicDecryptionReady)
                    }
                    isPublicDecryptionReady
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
                    fn isDecryptionDone(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <isDecryptionDoneCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::isDecryptionDone)
                    }
                    isDecryptionDone
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
                    fn reinitializeV2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <reinitializeV2Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::reinitializeV2)
                    }
                    reinitializeV2
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
                    fn isUserDecryptionReady(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <isUserDecryptionReadyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::isUserDecryptionReady)
                    }
                    isUserDecryptionReady
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
                Self::isDecryptionDone(inner) => {
                    <isDecryptionDoneCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isPublicDecryptionReady(inner) => {
                    <isPublicDecryptionReadyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isUserDecryptionReady(inner) => {
                    <isUserDecryptionReadyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::paused(inner) => {
                    <pausedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::reinitializeV2(inner) => {
                    <reinitializeV2Call as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::isDecryptionDone(inner) => {
                    <isDecryptionDoneCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isPublicDecryptionReady(inner) => {
                    <isPublicDecryptionReadyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isUserDecryptionReady(inner) => {
                    <isUserDecryptionReadyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::paused(inner) => {
                    <pausedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                Self::reinitializeV2(inner) => {
                    <reinitializeV2Call as alloy_sol_types::SolCall>::abi_encode_raw(
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
        AccountNotAllowedToUseCiphertext(AccountNotAllowedToUseCiphertext),
        #[allow(missing_docs)]
        AddressEmptyCode(AddressEmptyCode),
        #[allow(missing_docs)]
        ContractAddressesMaxLengthExceeded(ContractAddressesMaxLengthExceeded),
        #[allow(missing_docs)]
        ContractNotInContractAddresses(ContractNotInContractAddresses),
        #[allow(missing_docs)]
        CoprocessorSignerDoesNotMatchTxSender(CoprocessorSignerDoesNotMatchTxSender),
        #[allow(missing_docs)]
        DecryptionNotRequested(DecryptionNotRequested),
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
        HostChainNotRegistered(HostChainNotRegistered),
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
        KmsSignerDoesNotMatchTxSender(KmsSignerDoesNotMatchTxSender),
        #[allow(missing_docs)]
        MaxDecryptionRequestBitSizeExceeded(MaxDecryptionRequestBitSizeExceeded),
        #[allow(missing_docs)]
        MaxDurationDaysExceeded(MaxDurationDaysExceeded),
        #[allow(missing_docs)]
        NotCoprocessorSigner(NotCoprocessorSigner),
        #[allow(missing_docs)]
        NotCoprocessorTxSender(NotCoprocessorTxSender),
        #[allow(missing_docs)]
        NotCustodianSigner(NotCustodianSigner),
        #[allow(missing_docs)]
        NotCustodianTxSender(NotCustodianTxSender),
        #[allow(missing_docs)]
        NotGatewayOwner(NotGatewayOwner),
        #[allow(missing_docs)]
        NotInitializing(NotInitializing),
        #[allow(missing_docs)]
        NotInitializingFromEmptyProxy(NotInitializingFromEmptyProxy),
        #[allow(missing_docs)]
        NotKmsSigner(NotKmsSigner),
        #[allow(missing_docs)]
        NotKmsTxSender(NotKmsTxSender),
        #[allow(missing_docs)]
        NotOwnerOrGatewayConfig(NotOwnerOrGatewayConfig),
        #[allow(missing_docs)]
        NotPauserOrGatewayConfig(NotPauserOrGatewayConfig),
        #[allow(missing_docs)]
        PublicDecryptNotAllowed(PublicDecryptNotAllowed),
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
            [13u8, 134u8, 245u8, 33u8],
            [14u8, 86u8, 207u8, 61u8],
            [22u8, 10u8, 43u8, 75u8],
            [38u8, 205u8, 117u8, 220u8],
            [42u8, 124u8, 110u8, 246u8],
            [42u8, 135u8, 61u8, 39u8],
            [45u8, 231u8, 84u8, 56u8],
            [48u8, 52u8, 128u8, 64u8],
            [50u8, 149u8, 24u8, 99u8],
            [56u8, 137u8, 22u8, 187u8],
            [57u8, 22u8, 114u8, 167u8],
            [67u8, 49u8, 168u8, 93u8],
            [76u8, 156u8, 140u8, 227u8],
            [82u8, 215u8, 37u8, 245u8],
            [87u8, 207u8, 162u8, 23u8],
            [100u8, 25u8, 80u8, 215u8],
            [111u8, 79u8, 115u8, 31u8],
            [141u8, 252u8, 32u8, 43u8],
            [153u8, 150u8, 179u8, 21u8],
            [153u8, 236u8, 72u8, 217u8],
            [164u8, 195u8, 3u8, 145u8],
            [166u8, 166u8, 203u8, 33u8],
            [170u8, 29u8, 73u8, 164u8],
            [174u8, 232u8, 99u8, 35u8],
            [175u8, 31u8, 4u8, 149u8],
            [179u8, 152u8, 151u8, 159u8],
            [182u8, 103u8, 156u8, 59u8],
            [190u8, 120u8, 48u8, 177u8],
            [207u8, 174u8, 146u8, 31u8],
            [212u8, 138u8, 249u8, 66u8],
            [214u8, 189u8, 162u8, 117u8],
            [215u8, 139u8, 206u8, 12u8],
            [215u8, 230u8, 188u8, 248u8],
            [217u8, 60u8, 6u8, 101u8],
            [220u8, 77u8, 120u8, 177u8],
            [222u8, 40u8, 89u8, 193u8],
            [224u8, 124u8, 141u8, 186u8],
            [225u8, 52u8, 191u8, 98u8],
            [225u8, 145u8, 102u8, 238u8],
            [231u8, 244u8, 137u8, 93u8],
            [242u8, 76u8, 8u8, 135u8],
            [246u8, 69u8, 238u8, 223u8],
            [249u8, 36u8, 160u8, 207u8],
            [249u8, 46u8, 232u8, 169u8],
            [252u8, 230u8, 152u8, 247u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for DecryptionErrors {
        const NAME: &'static str = "DecryptionErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 45usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AccountNotAllowedToUseCiphertext(_) => {
                    <AccountNotAllowedToUseCiphertext as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AddressEmptyCode(_) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ContractAddressesMaxLengthExceeded(_) => {
                    <ContractAddressesMaxLengthExceeded as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ContractNotInContractAddresses(_) => {
                    <ContractNotInContractAddresses as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CoprocessorSignerDoesNotMatchTxSender(_) => {
                    <CoprocessorSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DecryptionNotRequested(_) => {
                    <DecryptionNotRequested as alloy_sol_types::SolError>::SELECTOR
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
                Self::HostChainNotRegistered(_) => {
                    <HostChainNotRegistered as alloy_sol_types::SolError>::SELECTOR
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
                Self::KmsSignerDoesNotMatchTxSender(_) => {
                    <KmsSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MaxDecryptionRequestBitSizeExceeded(_) => {
                    <MaxDecryptionRequestBitSizeExceeded as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MaxDurationDaysExceeded(_) => {
                    <MaxDurationDaysExceeded as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotCoprocessorSigner(_) => {
                    <NotCoprocessorSigner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotCoprocessorTxSender(_) => {
                    <NotCoprocessorTxSender as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotCustodianSigner(_) => {
                    <NotCustodianSigner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotCustodianTxSender(_) => {
                    <NotCustodianTxSender as alloy_sol_types::SolError>::SELECTOR
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
                Self::NotKmsSigner(_) => {
                    <NotKmsSigner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotKmsTxSender(_) => {
                    <NotKmsTxSender as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotOwnerOrGatewayConfig(_) => {
                    <NotOwnerOrGatewayConfig as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotPauserOrGatewayConfig(_) => {
                    <NotPauserOrGatewayConfig as alloy_sol_types::SolError>::SELECTOR
                }
                Self::PublicDecryptNotAllowed(_) => {
                    <PublicDecryptNotAllowed as alloy_sol_types::SolError>::SELECTOR
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
                    fn KmsSignerDoesNotMatchTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <KmsSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::KmsSignerDoesNotMatchTxSender)
                    }
                    KmsSignerDoesNotMatchTxSender
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
                    fn AccountNotAllowedToUseCiphertext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <AccountNotAllowedToUseCiphertext as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::AccountNotAllowedToUseCiphertext)
                    }
                    AccountNotAllowedToUseCiphertext
                },
                {
                    fn NotCoprocessorSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotCoprocessorSigner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::NotCoprocessorSigner)
                    }
                    NotCoprocessorSigner
                },
                {
                    fn NotKmsSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotKmsSigner as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(DecryptionErrors::NotKmsSigner)
                    }
                    NotKmsSigner
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
                    fn NotCustodianSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotCustodianSigner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::NotCustodianSigner)
                    }
                    NotCustodianSigner
                },
                {
                    fn PublicDecryptNotAllowed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <PublicDecryptNotAllowed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::PublicDecryptNotAllowed)
                    }
                    PublicDecryptNotAllowed
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
                    fn NotCoprocessorTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotCoprocessorTxSender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::NotCoprocessorTxSender)
                    }
                    NotCoprocessorTxSender
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
                    fn NotKmsTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotKmsTxSender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::NotKmsTxSender)
                    }
                    NotKmsTxSender
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
                    fn HostChainNotRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <HostChainNotRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::HostChainNotRegistered)
                    }
                    HostChainNotRegistered
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
                    fn CoprocessorSignerDoesNotMatchTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <CoprocessorSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::CoprocessorSignerDoesNotMatchTxSender)
                    }
                    CoprocessorSignerDoesNotMatchTxSender
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
                    fn NotCustodianTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotCustodianTxSender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::NotCustodianTxSender)
                    }
                    NotCustodianTxSender
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
                    fn KmsSignerDoesNotMatchTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <KmsSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::KmsSignerDoesNotMatchTxSender)
                    }
                    KmsSignerDoesNotMatchTxSender
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
                    fn AccountNotAllowedToUseCiphertext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <AccountNotAllowedToUseCiphertext as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::AccountNotAllowedToUseCiphertext)
                    }
                    AccountNotAllowedToUseCiphertext
                },
                {
                    fn NotCoprocessorSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotCoprocessorSigner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::NotCoprocessorSigner)
                    }
                    NotCoprocessorSigner
                },
                {
                    fn NotKmsSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotKmsSigner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::NotKmsSigner)
                    }
                    NotKmsSigner
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
                    fn NotCustodianSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotCustodianSigner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::NotCustodianSigner)
                    }
                    NotCustodianSigner
                },
                {
                    fn PublicDecryptNotAllowed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <PublicDecryptNotAllowed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::PublicDecryptNotAllowed)
                    }
                    PublicDecryptNotAllowed
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
                    fn NotCoprocessorTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotCoprocessorTxSender as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::NotCoprocessorTxSender)
                    }
                    NotCoprocessorTxSender
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
                    fn NotKmsTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotKmsTxSender as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::NotKmsTxSender)
                    }
                    NotKmsTxSender
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
                    fn HostChainNotRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <HostChainNotRegistered as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::HostChainNotRegistered)
                    }
                    HostChainNotRegistered
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
                    fn CoprocessorSignerDoesNotMatchTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <CoprocessorSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::CoprocessorSignerDoesNotMatchTxSender)
                    }
                    CoprocessorSignerDoesNotMatchTxSender
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
                    fn NotCustodianTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotCustodianTxSender as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::NotCustodianTxSender)
                    }
                    NotCustodianTxSender
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
                Self::AccountNotAllowedToUseCiphertext(inner) => {
                    <AccountNotAllowedToUseCiphertext as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
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
                Self::CoprocessorSignerDoesNotMatchTxSender(inner) => {
                    <CoprocessorSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DecryptionNotRequested(inner) => {
                    <DecryptionNotRequested as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::HostChainNotRegistered(inner) => {
                    <HostChainNotRegistered as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
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
                Self::KmsSignerDoesNotMatchTxSender(inner) => {
                    <KmsSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::NotCoprocessorSigner(inner) => {
                    <NotCoprocessorSigner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotCoprocessorTxSender(inner) => {
                    <NotCoprocessorTxSender as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotCustodianSigner(inner) => {
                    <NotCustodianSigner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotCustodianTxSender(inner) => {
                    <NotCustodianTxSender as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::NotKmsSigner(inner) => {
                    <NotKmsSigner as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NotKmsTxSender(inner) => {
                    <NotKmsTxSender as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotOwnerOrGatewayConfig(inner) => {
                    <NotOwnerOrGatewayConfig as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotPauserOrGatewayConfig(inner) => {
                    <NotPauserOrGatewayConfig as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PublicDecryptNotAllowed(inner) => {
                    <PublicDecryptNotAllowed as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::AccountNotAllowedToUseCiphertext(inner) => {
                    <AccountNotAllowedToUseCiphertext as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
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
                Self::CoprocessorSignerDoesNotMatchTxSender(inner) => {
                    <CoprocessorSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::HostChainNotRegistered(inner) => {
                    <HostChainNotRegistered as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
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
                Self::KmsSignerDoesNotMatchTxSender(inner) => {
                    <KmsSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::NotCoprocessorSigner(inner) => {
                    <NotCoprocessorSigner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotCoprocessorTxSender(inner) => {
                    <NotCoprocessorTxSender as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotCustodianSigner(inner) => {
                    <NotCustodianSigner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotCustodianTxSender(inner) => {
                    <NotCustodianTxSender as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::NotKmsSigner(inner) => {
                    <NotKmsSigner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotKmsTxSender(inner) => {
                    <NotKmsTxSender as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::NotPauserOrGatewayConfig(inner) => {
                    <NotPauserOrGatewayConfig as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PublicDecryptNotAllowed(inner) => {
                    <PublicDecryptNotAllowed as alloy_sol_types::SolError>::abi_encode_raw(
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
        Paused(Paused),
        #[allow(missing_docs)]
        PublicDecryptionRequest(PublicDecryptionRequest),
        #[allow(missing_docs)]
        PublicDecryptionResponse(PublicDecryptionResponse),
        #[allow(missing_docs)]
        PublicDecryptionResponseCall(PublicDecryptionResponseCall),
        #[allow(missing_docs)]
        Unpaused(Unpaused),
        #[allow(missing_docs)]
        Upgraded(Upgraded),
        #[allow(missing_docs)]
        UserDecryptionRequest(UserDecryptionRequest),
        #[allow(missing_docs)]
        UserDecryptionResponse(UserDecryptionResponse),
        #[allow(missing_docs)]
        UserDecryptionResponseThresholdReached(UserDecryptionResponseThresholdReached),
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
                34u8, 219u8, 72u8, 10u8, 57u8, 189u8, 114u8, 85u8, 100u8, 56u8, 170u8,
                219u8, 74u8, 50u8, 163u8, 210u8, 166u8, 99u8, 139u8, 135u8, 192u8, 59u8,
                190u8, 197u8, 254u8, 246u8, 153u8, 126u8, 16u8, 149u8, 135u8, 255u8,
            ],
            [
                77u8, 123u8, 29u8, 186u8, 73u8, 233u8, 232u8, 70u8, 33u8, 94u8, 22u8,
                33u8, 245u8, 115u8, 124u8, 129u8, 216u8, 97u8, 76u8, 79u8, 38u8, 132u8,
                148u8, 216u8, 183u8, 135u8, 99u8, 44u8, 78u8, 89u8, 240u8, 229u8,
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
                127u8, 205u8, 251u8, 83u8, 129u8, 145u8, 127u8, 85u8, 74u8, 113u8, 125u8,
                10u8, 84u8, 112u8, 163u8, 63u8, 90u8, 73u8, 186u8, 100u8, 69u8, 240u8,
                94u8, 196u8, 60u8, 116u8, 192u8, 188u8, 44u8, 198u8, 8u8, 178u8,
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
                232u8, 151u8, 82u8, 190u8, 14u8, 205u8, 182u8, 139u8, 42u8, 110u8, 181u8,
                239u8, 26u8, 137u8, 16u8, 57u8, 224u8, 233u8, 42u8, 227u8, 200u8, 166u8,
                34u8, 116u8, 197u8, 136u8, 30u8, 72u8, 238u8, 161u8, 237u8, 37u8,
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
                Some(
                    <PublicDecryptionResponseCall as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PublicDecryptionResponseCall as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PublicDecryptionResponseCall)
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
                Some(
                    <UserDecryptionResponseThresholdReached as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UserDecryptionResponseThresholdReached as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::UserDecryptionResponseThresholdReached)
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
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PublicDecryptionRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PublicDecryptionResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PublicDecryptionResponseCall(inner) => {
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
                Self::UserDecryptionResponseThresholdReached(inner) => {
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
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PublicDecryptionRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PublicDecryptionResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PublicDecryptionResponseCall(inner) => {
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
                Self::UserDecryptionResponseThresholdReached(inner) => {
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
        ///Creates a new call builder for the [`isDecryptionDone`] function.
        pub fn isDecryptionDone(
            &self,
            decryptionId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, isDecryptionDoneCall, N> {
            self.call_builder(
                &isDecryptionDoneCall {
                    decryptionId,
                },
            )
        }
        ///Creates a new call builder for the [`isPublicDecryptionReady`] function.
        pub fn isPublicDecryptionReady(
            &self,
            ctHandles: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
            _1: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, isPublicDecryptionReadyCall, N> {
            self.call_builder(
                &isPublicDecryptionReadyCall {
                    ctHandles,
                    _1,
                },
            )
        }
        ///Creates a new call builder for the [`isUserDecryptionReady`] function.
        pub fn isUserDecryptionReady(
            &self,
            userAddress: alloy::sol_types::private::Address,
            ctHandleContractPairs: alloy::sol_types::private::Vec<
                <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
            >,
            _2: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, isUserDecryptionReadyCall, N> {
            self.call_builder(
                &isUserDecryptionReadyCall {
                    userAddress,
                    ctHandleContractPairs,
                    _2,
                },
            )
        }
        ///Creates a new call builder for the [`pause`] function.
        pub fn pause(&self) -> alloy_contract::SolCallBuilder<&P, pauseCall, N> {
            self.call_builder(&pauseCall)
        }
        ///Creates a new call builder for the [`paused`] function.
        pub fn paused(&self) -> alloy_contract::SolCallBuilder<&P, pausedCall, N> {
            self.call_builder(&pausedCall)
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
        ///Creates a new call builder for the [`reinitializeV2`] function.
        pub fn reinitializeV2(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, reinitializeV2Call, N> {
            self.call_builder(&reinitializeV2Call)
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
        ///Creates a new event filter for the [`PublicDecryptionResponseCall`] event.
        pub fn PublicDecryptionResponseCall_filter(
            &self,
        ) -> alloy_contract::Event<&P, PublicDecryptionResponseCall, N> {
            self.event_filter::<PublicDecryptionResponseCall>()
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
        ///Creates a new event filter for the [`UserDecryptionResponseThresholdReached`] event.
        pub fn UserDecryptionResponseThresholdReached_filter(
            &self,
        ) -> alloy_contract::Event<&P, UserDecryptionResponseThresholdReached, N> {
            self.event_filter::<UserDecryptionResponseThresholdReached>()
        }
    }
}
