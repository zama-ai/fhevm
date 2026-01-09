///Module containing a contract's types and functions.
/**

```solidity
library IKMSGeneration {
    type KeyType is uint8;
    type ParamsType is uint8;
    struct KeyDigest { KeyType keyType; bytes digest; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IKMSGeneration {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KeyType(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<KeyType> for u8 {
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
        impl KeyType {
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
        impl From<u8> for KeyType {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<KeyType> for u8 {
            fn from(value: KeyType) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for KeyType {
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
        impl alloy_sol_types::EventTopic for KeyType {
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
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ParamsType(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<ParamsType> for u8 {
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
        impl ParamsType {
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
        impl From<u8> for ParamsType {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<ParamsType> for u8 {
            fn from(value: ParamsType) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for ParamsType {
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
        impl alloy_sol_types::EventTopic for ParamsType {
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
struct KeyDigest { KeyType keyType; bytes digest; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KeyDigest {
        #[allow(missing_docs)]
        pub keyType: <KeyType as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub digest: alloy::sol_types::private::Bytes,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (KeyType, alloy::sol_types::sol_data::Bytes);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <KeyType as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<KeyDigest> for UnderlyingRustTuple<'_> {
            fn from(value: KeyDigest) -> Self {
                (value.keyType, value.digest)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KeyDigest {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    keyType: tuple.0,
                    digest: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for KeyDigest {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for KeyDigest {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <KeyType as alloy_sol_types::SolType>::tokenize(&self.keyType),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.digest,
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
        impl alloy_sol_types::SolType for KeyDigest {
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
        impl alloy_sol_types::SolStruct for KeyDigest {
            const NAME: &'static str = "KeyDigest";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "KeyDigest(uint8 keyType,bytes digest)",
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
                    <KeyType as alloy_sol_types::SolType>::eip712_data_word(
                            &self.keyType,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.digest,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for KeyDigest {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <KeyType as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.keyType,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.digest,
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
                <KeyType as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.keyType,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.digest,
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
    /**Creates a new wrapper around an on-chain [`IKMSGeneration`](self) contract instance.

See the [wrapper's documentation](`IKMSGenerationInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> IKMSGenerationInstance<P, N> {
        IKMSGenerationInstance::<P, N>::new(address, __provider)
    }
    /**A [`IKMSGeneration`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IKMSGeneration`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IKMSGenerationInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IKMSGenerationInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IKMSGenerationInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IKMSGenerationInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`IKMSGeneration`](self) contract instance.

See the [wrapper's documentation](`IKMSGenerationInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
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
    impl<P: ::core::clone::Clone, N> IKMSGenerationInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IKMSGenerationInstance<P, N> {
            IKMSGenerationInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IKMSGenerationInstance<P, N> {
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
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IKMSGenerationInstance<P, N> {
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
library IKMSGeneration {
    type KeyType is uint8;
    type ParamsType is uint8;
    struct KeyDigest {
        KeyType keyType;
        bytes digest;
    }
}

interface KMSGeneration {
    error AddressEmptyCode(address target);
    error CoprocessorSignerDoesNotMatchTxSender(address signerAddress, address txSenderAddress);
    error CrsNotGenerated(uint256 crsId);
    error CrsgenNotRequested(uint256 crsId);
    error CrsgenOngoing(uint256 crsId);
    error ECDSAInvalidSignature();
    error ECDSAInvalidSignatureLength(uint256 length);
    error ECDSAInvalidSignatureS(bytes32 s);
    error ERC1967InvalidImplementation(address implementation);
    error ERC1967NonPayable();
    error EmptyKeyDigests(uint256 keyId);
    error FailedCall();
    error HostChainNotRegistered(uint256 chainId);
    error InvalidInitialization();
    error KeyNotGenerated(uint256 keyId);
    error KeygenNotRequested(uint256 keyId);
    error KeygenOngoing(uint256 keyId);
    error KmsAlreadySignedForCrsgen(uint256 crsId, address kmsSigner);
    error KmsAlreadySignedForKeygen(uint256 keyId, address kmsSigner);
    error KmsAlreadySignedForPrepKeygen(uint256 prepKeygenId, address kmsSigner);
    error KmsSignerDoesNotMatchTxSender(address signerAddress, address txSenderAddress);
    error NotCoprocessorSigner(address signerAddress);
    error NotCoprocessorTxSender(address txSenderAddress);
    error NotCustodianSigner(address signerAddress);
    error NotCustodianTxSender(address txSenderAddress);
    error NotGatewayOwner(address sender);
    error NotInitializing();
    error NotInitializingFromEmptyProxy();
    error NotKmsSigner(address signerAddress);
    error NotKmsTxSender(address txSenderAddress);
    error PrepKeygenNotRequested(uint256 prepKeygenId);
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);

    event ActivateCrs(uint256 crsId, string[] kmsNodeStorageUrls, bytes crsDigest);
    event ActivateKey(uint256 keyId, string[] kmsNodeStorageUrls, IKMSGeneration.KeyDigest[] keyDigests);
    event CrsgenRequest(uint256 crsId, uint256 maxBitLength, IKMSGeneration.ParamsType paramsType);
    event CrsgenResponse(uint256 crsId, bytes crsDigest, bytes signature, address kmsTxSender);
    event EIP712DomainChanged();
    event Initialized(uint64 version);
    event KeyReshareSameSet(uint256 prepKeygenId, uint256 keyId, uint256 keyReshareId, IKMSGeneration.ParamsType paramsType);
    event KeygenRequest(uint256 prepKeygenId, uint256 keyId);
    event KeygenResponse(uint256 keyId, IKMSGeneration.KeyDigest[] keyDigests, bytes signature, address kmsTxSender);
    event PRSSInit();
    event PrepKeygenRequest(uint256 prepKeygenId, uint256 epochId, IKMSGeneration.ParamsType paramsType);
    event PrepKeygenResponse(uint256 prepKeygenId, bytes signature, address kmsTxSender);
    event Upgraded(address indexed implementation);

    constructor();

    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function crsgenRequest(uint256 maxBitLength, IKMSGeneration.ParamsType paramsType) external;
    function crsgenResponse(uint256 crsId, bytes memory crsDigest, bytes memory signature) external;
    function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
    function getActiveCrsId() external view returns (uint256);
    function getActiveKeyId() external view returns (uint256);
    function getConsensusTxSenders(uint256 requestId) external view returns (address[] memory);
    function getCrsMaterials(uint256 crsId) external view returns (string[] memory, bytes memory);
    function getCrsParamsType(uint256 crsId) external view returns (IKMSGeneration.ParamsType);
    function getKeyMaterials(uint256 keyId) external view returns (string[] memory, IKMSGeneration.KeyDigest[] memory);
    function getKeyParamsType(uint256 keyId) external view returns (IKMSGeneration.ParamsType);
    function getVersion() external pure returns (string memory);
    function initializeFromEmptyProxy() external;
    function keyReshareSameSet(uint256 keyId) external;
    function keygen(IKMSGeneration.ParamsType paramsType) external;
    function keygenResponse(uint256 keyId, IKMSGeneration.KeyDigest[] memory keyDigests, bytes memory signature) external;
    function prepKeygenResponse(uint256 prepKeygenId, bytes memory signature) external;
    function proxiableUUID() external view returns (bytes32);
    function prssInit() external;
    function reinitializeV4() external;
    function upgradeToAndCall(address newImplementation, bytes memory data) external payable;
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
    "name": "crsgenRequest",
    "inputs": [
      {
        "name": "maxBitLength",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "paramsType",
        "type": "uint8",
        "internalType": "enum IKMSGeneration.ParamsType"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "crsgenResponse",
    "inputs": [
      {
        "name": "crsId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "crsDigest",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "signature",
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
    "name": "getActiveCrsId",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getActiveKeyId",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getConsensusTxSenders",
    "inputs": [
      {
        "name": "requestId",
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
    "name": "getCrsMaterials",
    "inputs": [
      {
        "name": "crsId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "string[]",
        "internalType": "string[]"
      },
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCrsParamsType",
    "inputs": [
      {
        "name": "crsId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "enum IKMSGeneration.ParamsType"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getKeyMaterials",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "string[]",
        "internalType": "string[]"
      },
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct IKMSGeneration.KeyDigest[]",
        "components": [
          {
            "name": "keyType",
            "type": "uint8",
            "internalType": "enum IKMSGeneration.KeyType"
          },
          {
            "name": "digest",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getKeyParamsType",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "enum IKMSGeneration.ParamsType"
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
    "name": "keyReshareSameSet",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "keygen",
    "inputs": [
      {
        "name": "paramsType",
        "type": "uint8",
        "internalType": "enum IKMSGeneration.ParamsType"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "keygenResponse",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "keyDigests",
        "type": "tuple[]",
        "internalType": "struct IKMSGeneration.KeyDigest[]",
        "components": [
          {
            "name": "keyType",
            "type": "uint8",
            "internalType": "enum IKMSGeneration.KeyType"
          },
          {
            "name": "digest",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "signature",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "prepKeygenResponse",
    "inputs": [
      {
        "name": "prepKeygenId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "signature",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "name": "prssInit",
    "inputs": [],
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
    "type": "event",
    "name": "ActivateCrs",
    "inputs": [
      {
        "name": "crsId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "kmsNodeStorageUrls",
        "type": "string[]",
        "indexed": false,
        "internalType": "string[]"
      },
      {
        "name": "crsDigest",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ActivateKey",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "kmsNodeStorageUrls",
        "type": "string[]",
        "indexed": false,
        "internalType": "string[]"
      },
      {
        "name": "keyDigests",
        "type": "tuple[]",
        "indexed": false,
        "internalType": "struct IKMSGeneration.KeyDigest[]",
        "components": [
          {
            "name": "keyType",
            "type": "uint8",
            "internalType": "enum IKMSGeneration.KeyType"
          },
          {
            "name": "digest",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "CrsgenRequest",
    "inputs": [
      {
        "name": "crsId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "maxBitLength",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "paramsType",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum IKMSGeneration.ParamsType"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "CrsgenResponse",
    "inputs": [
      {
        "name": "crsId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "crsDigest",
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
      }
    ],
    "anonymous": false
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
    "name": "KeyReshareSameSet",
    "inputs": [
      {
        "name": "prepKeygenId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "keyId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "keyReshareId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "paramsType",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum IKMSGeneration.ParamsType"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "KeygenRequest",
    "inputs": [
      {
        "name": "prepKeygenId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "keyId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "KeygenResponse",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "keyDigests",
        "type": "tuple[]",
        "indexed": false,
        "internalType": "struct IKMSGeneration.KeyDigest[]",
        "components": [
          {
            "name": "keyType",
            "type": "uint8",
            "internalType": "enum IKMSGeneration.KeyType"
          },
          {
            "name": "digest",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
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
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "PRSSInit",
    "inputs": [],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "PrepKeygenRequest",
    "inputs": [
      {
        "name": "prepKeygenId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "epochId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "paramsType",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum IKMSGeneration.ParamsType"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "PrepKeygenResponse",
    "inputs": [
      {
        "name": "prepKeygenId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
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
    "name": "CrsNotGenerated",
    "inputs": [
      {
        "name": "crsId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "CrsgenNotRequested",
    "inputs": [
      {
        "name": "crsId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "CrsgenOngoing",
    "inputs": [
      {
        "name": "crsId",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "EmptyKeyDigests",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
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
    "name": "InvalidInitialization",
    "inputs": []
  },
  {
    "type": "error",
    "name": "KeyNotGenerated",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "KeygenNotRequested",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "KeygenOngoing",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "KmsAlreadySignedForCrsgen",
    "inputs": [
      {
        "name": "crsId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "kmsSigner",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "KmsAlreadySignedForKeygen",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "kmsSigner",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "KmsAlreadySignedForPrepKeygen",
    "inputs": [
      {
        "name": "prepKeygenId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "kmsSigner",
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
    "name": "PrepKeygenNotRequested",
    "inputs": [
      {
        "name": "prepKeygenId",
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
pub mod KMSGeneration {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a06040523073ffffffffffffffffffffffffffffffffffffffff1660809073ffffffffffffffffffffffffffffffffffffffff1681525034801562000043575f80fd5b50620000546200005a60201b60201c565b620001e1565b5f6200006b6200015e60201b60201c565b9050805f0160089054906101000a900460ff1615620000b6576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b67ffffffffffffffff8016815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff16146200015b5767ffffffffffffffff815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d267ffffffffffffffff604051620001529190620001c6565b60405180910390a15b50565b5f80620001706200017960201b60201c565b90508091505090565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005f1b905090565b5f67ffffffffffffffff82169050919050565b620001c081620001a2565b82525050565b5f602082019050620001db5f830184620001b5565b92915050565b608051615fac620002085f395f8181612e9b01528181612ef001526131920152615fac5ff3fe608060405260043610610129575f3560e01c8063589adb0e116100aa578063ad3cb1cc1161006e578063ad3cb1cc146103a0578063baff211e146103ca578063c55b8724146103f4578063caa367db14610431578063d52f10eb14610459578063d65d83731461048357610129565b8063589adb0e146102cd57806362978787146102f55780637514a2ac1461031d57806384b0196e14610333578063936608ae1461036357610129565b80633c02f834116100f15780633c02f834146101fb57806345af261b146102235780634610ffe81461025f5780634f1ef2861461028757806352d1902d146102a357610129565b80630d8e6e2c1461012d578063123abb281461015757806316c713d91461016d57806319f4f632146101a957806339f73810146101e5575b5f80fd5b348015610138575f80fd5b506101416104ab565b60405161014e919061408f565b60405180910390f35b348015610162575f80fd5b5061016b610526565b005b348015610178575f80fd5b50610193600480360381019061018e91906140f3565b61064b565b6040516101a09190614205565b60405180910390f35b3480156101b4575f80fd5b506101cf60048036038101906101ca91906140f3565b61071c565b6040516101dc9190614298565b60405180910390f35b3480156101f0575f80fd5b506101f96107c9565b005b348015610206575f80fd5b50610221600480360381019061021c91906142d4565b610a38565b005b34801561022e575f80fd5b50610249600480360381019061024491906140f3565b610c76565b6040516102569190614298565b60405180910390f35b34801561026a575f80fd5b50610285600480360381019061028091906143c8565b610d0b565b005b6102a1600480360381019061029c91906145ab565b61132b565b005b3480156102ae575f80fd5b506102b761134a565b6040516102c4919061461d565b60405180910390f35b3480156102d8575f80fd5b506102f360048036038101906102ee9190614636565b61137b565b005b348015610300575f80fd5b5061031b60048036038101906103169190614693565b61175d565b005b348015610328575f80fd5b50610331611cd2565b005b34801561033e575f80fd5b50610347611df0565b60405161035a9796959493929190614833565b60405180910390f35b34801561036e575f80fd5b50610389600480360381019061038491906140f3565b611ef9565b604051610397929190614b45565b60405180910390f35b3480156103ab575f80fd5b506103b46122a0565b6040516103c1919061408f565b60405180910390f35b3480156103d5575f80fd5b506103de6122d9565b6040516103eb9190614b7a565b60405180910390f35b3480156103ff575f80fd5b5061041a600480360381019061041591906140f3565b6122f0565b604051610428929190614bdb565b60405180910390f35b34801561043c575f80fd5b5061045760048036038101906104529190614c10565b612602565b005b348015610464575f80fd5b5061046d61287b565b60405161047a9190614b7a565b60405180910390f35b34801561048e575f80fd5b506104a960048036038101906104a491906140f3565b612892565b005b60606040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506104ec5f612a8d565b6104f66004612a8d565b6104ff5f612a8d565b6040516020016105129493929190614d09565b604051602081830303815290604052905090565b60055f610531612b57565b9050805f0160089054906101000a900460ff168061057957508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b156105b0576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d28260405161063f9190614d89565b60405180910390a15050565b60605f610656612b6a565b90505f816003015f8581526020019081526020015f20549050816002015f8581526020019081526020015f205f8281526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561070e57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116106c5575b505050505092505050919050565b5f80610726612b6a565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff1661078957826040517f84de13310000000000000000000000000000000000000000000000000000000081526004016107809190614b7a565b60405180910390fd5b5f816006015f8581526020019081526020015f2054905081600d015f8281526020019081526020015f205f9054906101000a900460ff1692505050919050565b60016107d3612b91565b67ffffffffffffffff1614610814576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60055f61081f612b57565b9050805f0160089054906101000a900460ff168061086757508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b1561089e576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055506109576040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506040518060400160405280600181526020017f3100000000000000000000000000000000000000000000000000000000000000815250612bb5565b5f610960612b6a565b905060f86003600681111561097857610977614225565b5b901b816004018190555060f86004600681111561099857610997614225565b5b901b816005018190555060f8600560068111156109b8576109b7614225565b5b901b816009018190555060f86006808111156109d7576109d6614225565b5b901b81600e0181905550505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051610a2c9190614d89565b60405180910390a15050565b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610a95573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ab99190614db6565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610b2857336040517f0e56cf3d000000000000000000000000000000000000000000000000000000008152600401610b1f9190614de1565b60405180910390fd5b5f610b31612b6a565b90505f8160090154905060f860056006811115610b5157610b50614225565b5b901b8114158015610b7f5750816001015f8281526020019081526020015f205f9054906101000a900460ff16155b15610bc157806040517f061ac61d000000000000000000000000000000000000000000000000000000008152600401610bb89190614b7a565b60405180910390fd5b816009015f815480929190610bd590614e27565b91905055505f826009015490508483600a015f8381526020019081526020015f20819055508383600d015f8381526020019081526020015f205f6101000a81548160ff02191690836001811115610c2f57610c2e614225565b5b02179055507f3f038f6f88cb3031b7718588403a2ec220576a868be07dde4c02b846ca352ef5818686604051610c6793929190614e6e565b60405180910390a15050505050565b5f80610c80612b6a565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff16610ce357826040517fda32d00f000000000000000000000000000000000000000000000000000000008152600401610cda9190614b7a565b60405180910390fd5b80600d015f8481526020019081526020015f205f9054906101000a900460ff16915050919050565b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663e5275eaf336040518263ffffffff1660e01b8152600401610d589190614de1565b602060405180830381865afa158015610d73573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d979190614ed8565b610dd857336040517faee86323000000000000000000000000000000000000000000000000000000008152600401610dcf9190614de1565b60405180910390fd5b5f610de1612b6a565b90508060050154861180610df457505f86145b15610e3657856040517fadfab904000000000000000000000000000000000000000000000000000000008152600401610e2d9190614b7a565b60405180910390fd5b5f8585905003610e7d57856040517fe6f9083b000000000000000000000000000000000000000000000000000000008152600401610e749190614b7a565b60405180910390fd5b5f816006015f8881526020019081526020015f205490505f610ea182898989612bcb565b90505f610eaf828787612da2565b9050835f015f8a81526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615610f4f5788816040517f98fb957d000000000000000000000000000000000000000000000000000000008152600401610f46929190614f03565b60405180910390fd5b6001845f015f8b81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f846002015f8b81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505f818054905090507f2afe64fb3afde8e2678aea84cf36223f330e2fb1286d37aed573ab9cd1db47c78b8b8b8b8b3360405161107996959493929190615136565b60405180910390a1856001015f8c81526020019081526020015f205f9054906101000a900460ff161580156110b357506110b281612e08565b5b1561131e576001866001015f8d81526020019081526020015f205f6101000a81548160ff0219169083151502179055505f5b8a8a905081101561116957866007015f8d81526020019081526020015f208b8b838181106111165761111561518b565b5b905060200281019061112891906151c4565b908060018154018082558091505060019003905f5260205f2090600202015f90919091909150818161115a91906155f1565b505080806001019150506110e5565b5083866003015f8d81526020019081526020015f20819055508a86600801819055505f8167ffffffffffffffff8111156111a6576111a5614487565b5b6040519080825280602002602001820160405280156111d957816020015b60608152602001906001900390816111c45790505b5090505f5b828110156112de5773d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663e3b2a8748583815481106112295761122861518b565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b815260040161126d9190614de1565b5f60405180830381865afa158015611287573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f820116820180604052508101906112af9190615782565b606001518282815181106112c6576112c561518b565b5b602002602001018190525080806001019150506111de565b507feb85c26dbcad46b80a68a0f24cce7c2c90f0a1faded84184138839fc9e80a25b8c828d8d60405161131494939291906157c9565b60405180910390a1505b5050505050505050505050565b611333612e99565b61133c82612f7f565b6113468282613072565b5050565b5f611353613190565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663e5275eaf336040518263ffffffff1660e01b81526004016113c89190614de1565b602060405180830381865afa1580156113e3573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114079190614ed8565b61144857336040517faee8632300000000000000000000000000000000000000000000000000000000815260040161143f9190614de1565b60405180910390fd5b5f611451612b6a565b9050806004015484118061146457505f84145b156114a657836040517f0ab7f68700000000000000000000000000000000000000000000000000000000815260040161149d9190614b7a565b60405180910390fd5b5f6114b085613217565b90505f6114be828686612da2565b9050825f015f8781526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff161561155e5785816040517f33ca1fe3000000000000000000000000000000000000000000000000000000008152600401611555929190614f03565b60405180910390fd5b6001835f015f8881526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f836002015f8881526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507f4c715c5734ce5c18c9c12e8496e53d2a65f1ec381d476957f0f596b364a59b0c8787873360405161167c949392919061580e565b60405180910390a1836001015f8881526020019081526020015f205f9054906101000a900460ff161580156116ba57506116b98180549050612e08565b5b15611754576001846001015f8981526020019081526020015f205f6101000a81548160ff02191690831515021790555082846003015f8981526020019081526020015f20819055505f846006015f8981526020019081526020015f205490507f78b179176d1f19d7c28e80823deba2624da2ca2ec64b1701f3632a87c9aedc92888260405161174a92919061584c565b60405180910390a1505b50505050505050565b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663e5275eaf336040518263ffffffff1660e01b81526004016117aa9190614de1565b602060405180830381865afa1580156117c5573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117e99190614ed8565b61182a57336040517faee863230000000000000000000000000000000000000000000000000000000081526004016118219190614de1565b60405180910390fd5b5f611833612b6a565b9050806009015486118061184657505f86145b1561188857856040517f8d8c940a00000000000000000000000000000000000000000000000000000000815260040161187f9190614b7a565b60405180910390fd5b5f81600a015f8881526020019081526020015f205490505f6118ac8883898961326f565b90505f6118ba828787612da2565b9050835f015f8a81526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff161561195a5788816040517ffcf5a6e9000000000000000000000000000000000000000000000000000000008152600401611951929190614f03565b60405180910390fd5b6001845f015f8b81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f846002015f8b81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505f818054905090507f7bf1b42c10e9497c879620c5b7afced10bda17d8c90b22f0e3bc6b2fd6ced0bd8b8b8b8b8b33604051611a8496959493929190615873565b60405180910390a1856001015f8c81526020019081526020015f205f9054906101000a900460ff16158015611abe5750611abd81612e08565b5b15611cc5576001866001015f8d81526020019081526020015f205f6101000a81548160ff021916908315150217905550898987600b015f8e81526020019081526020015f209182611b109291906154d0565b5083866003015f8d81526020019081526020015f20819055508a86600c01819055505f8167ffffffffffffffff811115611b4d57611b4c614487565b5b604051908082528060200260200182016040528015611b8057816020015b6060815260200190600190039081611b6b5790505b5090505f5b82811015611c855773d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663e3b2a874858381548110611bd057611bcf61518b565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b8152600401611c149190614de1565b5f60405180830381865afa158015611c2e573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f82011682018060405250810190611c569190615782565b60600151828281518110611c6d57611c6c61518b565b5b60200260200101819052508080600101915050611b85565b507f2258b73faed33fb2e2ea454403bef974920caf682ab3a723484fcf67553b16a28c828d8d604051611cbb94939291906158c8565b60405180910390a1505b5050505050505050505050565b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611d2f573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d539190614db6565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614611dc257336040517f0e56cf3d000000000000000000000000000000000000000000000000000000008152600401611db99190614de1565b60405180910390fd5b7f11db42c1878f2e2819241f5250984563f06cf22818e7adb86a66921d15d59d3f60405160405180910390a1565b5f6060805f805f60605f611e026132f6565b90505f801b815f0154148015611e1d57505f801b8160010154145b611e5c576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611e5390615957565b60405180910390fd5b611e6461331d565b611e6c6133bb565b46305f801b5f67ffffffffffffffff811115611e8b57611e8a614487565b5b604051908082528060200260200182016040528015611eb95781602001602082028036833780820191505090505b507f0f0000000000000000000000000000000000000000000000000000000000000095949392919097509750975097509750975097505090919293949596565b6060805f611f05612b6a565b9050806001015f8581526020019081526020015f205f9054906101000a900460ff16611f6857836040517f84de1331000000000000000000000000000000000000000000000000000000008152600401611f5f9190614b7a565b60405180910390fd5b5f816003015f8681526020019081526020015f205490505f826002015f8781526020019081526020015f205f8381526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561201f57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311611fd6575b505050505090505f815190505f8167ffffffffffffffff81111561204657612045614487565b5b60405190808252806020026020018201604052801561207957816020015b60608152602001906001900390816120645790505b5090505f5b8281101561215e5773d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663e3b2a8748583815181106120c9576120c861518b565b5b60200260200101516040518263ffffffff1660e01b81526004016120ed9190614de1565b5f60405180830381865afa158015612107573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f8201168201806040525081019061212f9190615782565b606001518282815181106121465761214561518b565b5b6020026020010181905250808060010191505061207e565b5080856007015f8a81526020019081526020015f2080805480602002602001604051908101604052809291908181526020015f905b8282101561228c578382905f5260205f2090600202016040518060400160405290815f82015f9054906101000a900460ff1660018111156121d7576121d6614225565b5b60018111156121e9576121e8614225565b5b81526020016001820180546121fd90615303565b80601f016020809104026020016040519081016040528092919081815260200182805461222990615303565b80156122745780601f1061224b57610100808354040283529160200191612274565b820191905f5260205f20905b81548152906001019060200180831161225757829003601f168201915b50505050508152505081526020019060010190612193565b505050509050965096505050505050915091565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b5f806122e3612b6a565b905080600c015491505090565b6060805f6122fc612b6a565b9050806001015f8581526020019081526020015f205f9054906101000a900460ff1661235f57836040517fda32d00f0000000000000000000000000000000000000000000000000000000081526004016123569190614b7a565b60405180910390fd5b5f816003015f8681526020019081526020015f205490505f826002015f8781526020019081526020015f205f8381526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561241657602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116123cd575b505050505090505f815190505f8167ffffffffffffffff81111561243d5761243c614487565b5b60405190808252806020026020018201604052801561247057816020015b606081526020019060019003908161245b5790505b5090505f5b828110156125555773d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663e3b2a8748583815181106124c0576124bf61518b565b5b60200260200101516040518263ffffffff1660e01b81526004016124e49190614de1565b5f60405180830381865afa1580156124fe573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f820116820180604052508101906125269190615782565b6060015182828151811061253d5761253c61518b565b5b60200260200101819052508080600101915050612475565b508085600b015f8a81526020019081526020015f2080805461257690615303565b80601f01602080910402602001604051908101604052809291908181526020018280546125a290615303565b80156125ed5780601f106125c4576101008083540402835291602001916125ed565b820191905f5260205f20905b8154815290600101906020018083116125d057829003601f168201915b50505050509050965096505050505050915091565b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561265f573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906126839190614db6565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146126f257336040517f0e56cf3d0000000000000000000000000000000000000000000000000000000081526004016126e99190614de1565b60405180910390fd5b5f6126fb612b6a565b90505f8160050154905060f86004600681111561271b5761271a614225565b5b901b81141580156127495750816001015f8281526020019081526020015f205f9054906101000a900460ff16155b1561278b57806040517f3b853da80000000000000000000000000000000000000000000000000000000081526004016127829190614b7a565b60405180910390fd5b816004015f81548092919061279f90614e27565b91905055505f82600401549050826005015f8154809291906127c090614e27565b91905055505f8360050154905080846006015f8481526020019081526020015f208190555081846006015f8381526020019081526020015f20819055505f8585600d015f8581526020019081526020015f205f6101000a81548160ff0219169083600181111561283357612832614225565b5b02179055507f02024007d96574dbc9d11328bfee9893e7c7bb4ef4aa806df33bfdf454eb5e6083828860405161286b93929190614e6e565b60405180910390a1505050505050565b5f80612885612b6a565b9050806008015491505090565b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156128ef573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129139190614db6565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461298257336040517f0e56cf3d0000000000000000000000000000000000000000000000000000000081526004016129799190614de1565b60405180910390fd5b5f61298b612b6a565b9050806001015f8381526020019081526020015f205f9054906101000a900460ff166129ee57816040517f84de13310000000000000000000000000000000000000000000000000000000081526004016129e59190614b7a565b60405180910390fd5b5f816006015f8481526020019081526020015f205490505f82600d015f8381526020019081526020015f205f9054906101000a900460ff16905082600e015f815480929190612a3c90614e27565b91905055505f83600e015490507f1ccb5545c4c8db50a0f5b416499526929f68534ed47f6cfd4c9f069075e60b4583868385604051612a7e9493929190615975565b60405180910390a15050505050565b60605f6001612a9b84613459565b0190505f8167ffffffffffffffff811115612ab957612ab8614487565b5b6040519080825280601f01601f191660200182016040528015612aeb5781602001600182028036833780820191505090505b5090505f82602083010190505b600115612b4c578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a8581612b4157612b406159b8565b5b0494505f8503612af8575b819350505050919050565b5f80612b616135aa565b90508091505090565b5f7f0b8fdb1f0a6356dd20a6cbc6f9668fac23b85f96575d10e333e603faa794ac00905090565b5f612b9a612b57565b5f015f9054906101000a900467ffffffffffffffff16905090565b612bbd6135d3565b612bc78282613613565b5050565b5f808383905067ffffffffffffffff811115612bea57612be9614487565b5b604051908082528060200260200182016040528015612c185781602001602082028036833780820191505090505b5090505f5b84849050811015612d1c57604051806060016040528060258152602001615f876025913980519060200120858583818110612c5b57612c5a61518b565b5b9050602002810190612c6d91906151c4565b5f016020810190612c7e91906159e5565b868684818110612c9157612c9061518b565b5b9050602002810190612ca391906151c4565b8060200190612cb2919061526a565b604051612cc0929190615a3e565b6040518091039020604051602001612cda93929190615a65565b60405160208183030381529060405280519060200120828281518110612d0357612d0261518b565b5b6020026020010181815250508080600101915050612c1d565b50612d976040518060a0016040528060728152602001615f156072913980519060200120878784604051602001612d539190615b4b565b60405160208183030381529060405280519060200120604051602001612d7c9493929190615b61565b60405160208183030381529060405280519060200120613664565b915050949350505050565b5f80612df18585858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505061367d565b9050612dfd81336136a7565b809150509392505050565b5f8073d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663b4722bc46040518163ffffffff1660e01b8152600401602060405180830381865afa158015612e67573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612e8b9190615bb8565b905080831015915050919050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff161480612f4657507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16612f2d6137b8565b73ffffffffffffffffffffffffffffffffffffffff1614155b15612f7d576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612fdc573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906130009190614db6565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461306f57336040517f0e56cf3d0000000000000000000000000000000000000000000000000000000081526004016130669190614de1565b60405180910390fd5b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa9250505080156130da57506040513d601f19601f820116820180604052508101906130d79190615c0d565b60015b61311b57816040517f4c9c8ce30000000000000000000000000000000000000000000000000000000081526004016131129190614de1565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b811461318157806040517faa1d49a4000000000000000000000000000000000000000000000000000000008152600401613178919061461d565b60405180910390fd5b61318b838361380b565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff1614613215576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f6132686040518060600160405280602c8152602001615ee9602c9139805190602001208360405160200161324d929190615c38565b60405160208183030381529060405280519060200120613664565b9050919050565b5f6132ec604051806080016040528060468152602001615ea36046913980519060200120868686866040516020016132a8929190615a3e565b604051602081830303815290604052805190602001206040516020016132d19493929190615b61565b60405160208183030381529060405280519060200120613664565b9050949350505050565b5f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100905090565b60605f6133286132f6565b905080600201805461333990615303565b80601f016020809104026020016040519081016040528092919081815260200182805461336590615303565b80156133b05780601f10613387576101008083540402835291602001916133b0565b820191905f5260205f20905b81548152906001019060200180831161339357829003601f168201915b505050505091505090565b60605f6133c66132f6565b90508060030180546133d790615303565b80601f016020809104026020016040519081016040528092919081815260200182805461340390615303565b801561344e5780601f106134255761010080835404028352916020019161344e565b820191905f5260205f20905b81548152906001019060200180831161343157829003601f168201915b505050505091505090565b5f805f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f01000000000000000083106134b5577a184f03e93ff9f4daa797ed6e38ed64bf6a1f01000000000000000083816134ab576134aa6159b8565b5b0492506040810190505b6d04ee2d6d415b85acef810000000083106134f2576d04ee2d6d415b85acef810000000083816134e8576134e76159b8565b5b0492506020810190505b662386f26fc10000831061352157662386f26fc100008381613517576135166159b8565b5b0492506010810190505b6305f5e100831061354a576305f5e10083816135405761353f6159b8565b5b0492506008810190505b612710831061356f576127108381613565576135646159b8565b5b0492506004810190505b606483106135925760648381613588576135876159b8565b5b0492506002810190505b600a83106135a1576001810190505b80915050919050565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005f1b905090565b6135db61387d565b613611576040517fd7e6bcf800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b61361b6135d3565b5f6136246132f6565b9050828160020190816136379190615cb7565b50818160030190816136499190615cb7565b505f801b815f01819055505f801b8160010181905550505050565b5f61367661367061389b565b836138a9565b9050919050565b5f805f8061368b86866138e9565b92509250925061369b828261393e565b82935050505092915050565b6136b082613aa0565b8173ffffffffffffffffffffffffffffffffffffffff1673d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663e3b2a874836040518263ffffffff1660e01b81526004016137149190614de1565b5f60405180830381865afa15801561372e573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f820116820180604052508101906137569190615782565b6020015173ffffffffffffffffffffffffffffffffffffffff16146137b45781816040517f0d86f5210000000000000000000000000000000000000000000000000000000081526004016137ab929190615d86565b60405180910390fd5b5050565b5f6137e47f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b613b70565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b61381482613b79565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f815111156138705761386a8282613c42565b50613879565b613878613cc2565b5b5050565b5f613886612b57565b5f0160089054906101000a900460ff16905090565b5f6138a4613cfe565b905090565b5f6040517f190100000000000000000000000000000000000000000000000000000000000081528360028201528260228201526042812091505092915050565b5f805f6041845103613929575f805f602087015192506040870151915060608701515f1a905061391b88828585613d61565b955095509550505050613937565b5f600285515f1b9250925092505b9250925092565b5f600381111561395157613950614225565b5b82600381111561396457613963614225565b5b0315613a9c576001600381111561397e5761397d614225565b5b82600381111561399157613990614225565b5b036139c8576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600260038111156139dc576139db614225565b5b8260038111156139ef576139ee614225565b5b03613a3357805f1c6040517ffce698f7000000000000000000000000000000000000000000000000000000008152600401613a2a9190614b7a565b60405180910390fd5b600380811115613a4657613a45614225565b5b826003811115613a5957613a58614225565b5b03613a9b57806040517fd78bce0c000000000000000000000000000000000000000000000000000000008152600401613a92919061461d565b60405180910390fd5b5b5050565b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663203d0114826040518263ffffffff1660e01b8152600401613aed9190614de1565b602060405180830381865afa158015613b08573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613b2c9190614ed8565b613b6d57806040517f2a7c6ef6000000000000000000000000000000000000000000000000000000008152600401613b649190614de1565b60405180910390fd5b50565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b03613bd457806040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401613bcb9190614de1565b60405180910390fd5b80613c007f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b613b70565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f808473ffffffffffffffffffffffffffffffffffffffff1684604051613c6b9190615ddd565b5f60405180830381855af49150503d805f8114613ca3576040519150601f19603f3d011682016040523d82523d5f602084013e613ca8565b606091505b5091509150613cb8858383613e48565b9250505092915050565b5f341115613cfc576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f613d28613ed5565b613d30613f4b565b4630604051602001613d46959493929190615df3565b60405160208183030381529060405280519060200120905090565b5f805f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c1115613d9d575f600385925092509250613e3e565b5f6001888888886040515f8152602001604052604051613dc09493929190615e5f565b6020604051602081039080840390855afa158015613de0573d5f803e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603613e31575f60015f801b93509350935050613e3e565b805f805f1b935093509350505b9450945094915050565b606082613e5d57613e5882613fc2565b613ecd565b5f8251148015613e8357505f8473ffffffffffffffffffffffffffffffffffffffff163b145b15613ec557836040517f9996b315000000000000000000000000000000000000000000000000000000008152600401613ebc9190614de1565b60405180910390fd5b819050613ece565b5b9392505050565b5f80613edf6132f6565b90505f613eea61331d565b90505f81511115613f0657808051906020012092505050613f48565b5f825f015490505f801b8114613f2157809350505050613f48565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f80613f556132f6565b90505f613f606133bb565b90505f81511115613f7c57808051906020012092505050613fbf565b5f826001015490505f801b8114613f9857809350505050613fbf565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f81511115613fd357805160208201fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f81519050919050565b5f82825260208201905092915050565b5f5b8381101561403c578082015181840152602081019050614021565b5f8484015250505050565b5f601f19601f8301169050919050565b5f61406182614005565b61406b818561400f565b935061407b81856020860161401f565b61408481614047565b840191505092915050565b5f6020820190508181035f8301526140a78184614057565b905092915050565b5f604051905090565b5f80fd5b5f80fd5b5f819050919050565b6140d2816140c0565b81146140dc575f80fd5b50565b5f813590506140ed816140c9565b92915050565b5f60208284031215614108576141076140b8565b5b5f614115848285016140df565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61417082614147565b9050919050565b61418081614166565b82525050565b5f6141918383614177565b60208301905092915050565b5f602082019050919050565b5f6141b38261411e565b6141bd8185614128565b93506141c883614138565b805f5b838110156141f85781516141df8882614186565b97506141ea8361419d565b9250506001810190506141cb565b5085935050505092915050565b5f6020820190508181035f83015261421d81846141a9565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b6002811061426357614262614225565b5b50565b5f81905061427382614252565b919050565b5f61428282614266565b9050919050565b61429281614278565b82525050565b5f6020820190506142ab5f830184614289565b92915050565b600281106142bd575f80fd5b50565b5f813590506142ce816142b1565b92915050565b5f80604083850312156142ea576142e96140b8565b5b5f6142f7858286016140df565b9250506020614308858286016142c0565b9150509250929050565b5f80fd5b5f80fd5b5f80fd5b5f8083601f84011261433357614332614312565b5b8235905067ffffffffffffffff8111156143505761434f614316565b5b60208301915083602082028301111561436c5761436b61431a565b5b9250929050565b5f8083601f84011261438857614387614312565b5b8235905067ffffffffffffffff8111156143a5576143a4614316565b5b6020830191508360018202830111156143c1576143c061431a565b5b9250929050565b5f805f805f606086880312156143e1576143e06140b8565b5b5f6143ee888289016140df565b955050602086013567ffffffffffffffff81111561440f5761440e6140bc565b5b61441b8882890161431e565b9450945050604086013567ffffffffffffffff81111561443e5761443d6140bc565b5b61444a88828901614373565b92509250509295509295909350565b61446281614166565b811461446c575f80fd5b50565b5f8135905061447d81614459565b92915050565b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6144bd82614047565b810181811067ffffffffffffffff821117156144dc576144db614487565b5b80604052505050565b5f6144ee6140af565b90506144fa82826144b4565b919050565b5f67ffffffffffffffff82111561451957614518614487565b5b61452282614047565b9050602081019050919050565b828183375f83830152505050565b5f61454f61454a846144ff565b6144e5565b90508281526020810184848401111561456b5761456a614483565b5b61457684828561452f565b509392505050565b5f82601f83011261459257614591614312565b5b81356145a284826020860161453d565b91505092915050565b5f80604083850312156145c1576145c06140b8565b5b5f6145ce8582860161446f565b925050602083013567ffffffffffffffff8111156145ef576145ee6140bc565b5b6145fb8582860161457e565b9150509250929050565b5f819050919050565b61461781614605565b82525050565b5f6020820190506146305f83018461460e565b92915050565b5f805f6040848603121561464d5761464c6140b8565b5b5f61465a868287016140df565b935050602084013567ffffffffffffffff81111561467b5761467a6140bc565b5b61468786828701614373565b92509250509250925092565b5f805f805f606086880312156146ac576146ab6140b8565b5b5f6146b9888289016140df565b955050602086013567ffffffffffffffff8111156146da576146d96140bc565b5b6146e688828901614373565b9450945050604086013567ffffffffffffffff811115614709576147086140bc565b5b61471588828901614373565b92509250509295509295909350565b5f7fff0000000000000000000000000000000000000000000000000000000000000082169050919050565b61475881614724565b82525050565b614767816140c0565b82525050565b61477681614166565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b6147ae816140c0565b82525050565b5f6147bf83836147a5565b60208301905092915050565b5f602082019050919050565b5f6147e18261477c565b6147eb8185614786565b93506147f683614796565b805f5b8381101561482657815161480d88826147b4565b9750614818836147cb565b9250506001810190506147f9565b5085935050505092915050565b5f60e0820190506148465f83018a61474f565b81810360208301526148588189614057565b9050818103604083015261486c8188614057565b905061487b606083018761475e565b614888608083018661476d565b61489560a083018561460e565b81810360c08301526148a781846147d7565b905098975050505050505050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f82825260208201905092915050565b5f6148f882614005565b61490281856148de565b935061491281856020860161401f565b61491b81614047565b840191505092915050565b5f61493183836148ee565b905092915050565b5f602082019050919050565b5f61494f826148b5565b61495981856148bf565b93508360208202850161496b856148cf565b805f5b858110156149a657848403895281516149878582614926565b945061499283614939565b925060208a0199505060018101905061496e565b50829750879550505050505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b600281106149f2576149f1614225565b5b50565b5f819050614a02826149e1565b919050565b5f614a11826149f5565b9050919050565b614a2181614a07565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f614a4b82614a27565b614a558185614a31565b9350614a6581856020860161401f565b614a6e81614047565b840191505092915050565b5f604083015f830151614a8e5f860182614a18565b5060208301518482036020860152614aa68282614a41565b9150508091505092915050565b5f614abe8383614a79565b905092915050565b5f602082019050919050565b5f614adc826149b8565b614ae681856149c2565b935083602082028501614af8856149d2565b805f5b85811015614b335784840389528151614b148582614ab3565b9450614b1f83614ac6565b925060208a01995050600181019050614afb565b50829750879550505050505092915050565b5f6040820190508181035f830152614b5d8185614945565b90508181036020830152614b718184614ad2565b90509392505050565b5f602082019050614b8d5f83018461475e565b92915050565b5f82825260208201905092915050565b5f614bad82614a27565b614bb78185614b93565b9350614bc781856020860161401f565b614bd081614047565b840191505092915050565b5f6040820190508181035f830152614bf38185614945565b90508181036020830152614c078184614ba3565b90509392505050565b5f60208284031215614c2557614c246140b8565b5b5f614c32848285016142c0565b91505092915050565b5f81905092915050565b5f614c4f82614005565b614c598185614c3b565b9350614c6981856020860161401f565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f614ca9600283614c3b565b9150614cb482614c75565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f614cf3600183614c3b565b9150614cfe82614cbf565b600182019050919050565b5f614d148287614c45565b9150614d1f82614c9d565b9150614d2b8286614c45565b9150614d3682614ce7565b9150614d428285614c45565b9150614d4d82614ce7565b9150614d598284614c45565b915081905095945050505050565b5f67ffffffffffffffff82169050919050565b614d8381614d67565b82525050565b5f602082019050614d9c5f830184614d7a565b92915050565b5f81519050614db081614459565b92915050565b5f60208284031215614dcb57614dca6140b8565b5b5f614dd884828501614da2565b91505092915050565b5f602082019050614df45f83018461476d565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f614e31826140c0565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203614e6357614e62614dfa565b5b600182019050919050565b5f606082019050614e815f83018661475e565b614e8e602083018561475e565b614e9b6040830184614289565b949350505050565b5f8115159050919050565b614eb781614ea3565b8114614ec1575f80fd5b50565b5f81519050614ed281614eae565b92915050565b5f60208284031215614eed57614eec6140b8565b5b5f614efa84828501614ec4565b91505092915050565b5f604082019050614f165f83018561475e565b614f23602083018461476d565b9392505050565b5f819050919050565b60028110614f3f575f80fd5b50565b5f81359050614f5081614f33565b92915050565b5f614f646020840184614f42565b905092915050565b5f80fd5b5f80fd5b5f80fd5b5f8083356001602003843603038112614f9457614f93614f74565b5b83810192508235915060208301925067ffffffffffffffff821115614fbc57614fbb614f6c565b5b600182023603831315614fd257614fd1614f70565b5b509250929050565b5f614fe58385614a31565b9350614ff283858461452f565b614ffb83614047565b840190509392505050565b5f604083016150175f840184614f56565b6150235f860182614a18565b506150316020840184614f78565b8583036020870152615044838284614fda565b925050508091505092915050565b5f61505d8383615006565b905092915050565b5f823560016040038336030381126150805761507f614f74565b5b82810191505092915050565b5f602082019050919050565b5f6150a383856149c2565b9350836020840285016150b584614f2a565b805f5b878110156150f85784840389526150cf8284615065565b6150d98582615052565b94506150e48361508c565b925060208a019950506001810190506150b8565b50829750879450505050509392505050565b5f6151158385614b93565b935061512283858461452f565b61512b83614047565b840190509392505050565b5f6080820190506151495f83018961475e565b818103602083015261515c818789615098565b9050818103604083015261517181858761510a565b9050615180606083018461476d565b979650505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b5f823560016040038336030381126151df576151de6151b8565b5b80830191505092915050565b5f81356151f781614f33565b80915050919050565b5f815f1b9050919050565b5f60ff61521784615200565b9350801983169250808416831791505092915050565b5f615237826149f5565b9050919050565b5f819050919050565b6152508261522d565b61526361525c8261523e565b835461520b565b8255505050565b5f8083356001602003843603038112615286576152856151b8565b5b80840192508235915067ffffffffffffffff8211156152a8576152a76151bc565b5b6020830192506001820236038313156152c4576152c36151c0565b5b509250929050565b5f82905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f600282049050600182168061531a57607f821691505b60208210810361532d5761532c6152d6565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f6008830261538f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82615354565b6153998683615354565b95508019841693508086168417925050509392505050565b5f819050919050565b5f6153d46153cf6153ca846140c0565b6153b1565b6140c0565b9050919050565b5f819050919050565b6153ed836153ba565b6154016153f9826153db565b848454615360565b825550505050565b5f90565b615415615409565b6154208184846153e4565b505050565b5b81811015615443576154385f8261540d565b600181019050615426565b5050565b601f8211156154885761545981615333565b61546284615345565b81016020851015615471578190505b61548561547d85615345565b830182615425565b50505b505050565b5f82821c905092915050565b5f6154a85f198460080261548d565b1980831691505092915050565b5f6154c08383615499565b9150826002028217905092915050565b6154da83836152cc565b67ffffffffffffffff8111156154f3576154f2614487565b5b6154fd8254615303565b615508828285615447565b5f601f831160018114615535575f8415615523578287013590505b61552d85826154b5565b865550615594565b601f19841661554386615333565b5f5b8281101561556a57848901358255600182019150602085019450602081019050615545565b868310156155875784890135615583601f891682615499565b8355505b6001600288020188555050505b50505050505050565b6155a88383836154d0565b505050565b5f81015f8301806155bd816151eb565b90506155c98184615247565b50505060018101602083016155de818561526a565b6155e981838661559d565b505050505050565b6155fb82826155ad565b5050565b5f80fd5b5f80fd5b5f67ffffffffffffffff82111561562157615620614487565b5b61562a82614047565b9050602081019050919050565b5f61564961564484615607565b6144e5565b90508281526020810184848401111561566557615664614483565b5b61567084828561401f565b509392505050565b5f82601f83011261568c5761568b614312565b5b815161569c848260208601615637565b91505092915050565b5f60a082840312156156ba576156b96155ff565b5b6156c460a06144e5565b90505f6156d384828501614da2565b5f8301525060206156e684828501614da2565b602083015250604082015167ffffffffffffffff81111561570a57615709615603565b5b61571684828501615678565b604083015250606082015167ffffffffffffffff81111561573a57615739615603565b5b61574684828501615678565b606083015250608082015167ffffffffffffffff81111561576a57615769615603565b5b61577684828501615678565b60808301525092915050565b5f60208284031215615797576157966140b8565b5b5f82015167ffffffffffffffff8111156157b4576157b36140bc565b5b6157c0848285016156a5565b91505092915050565b5f6060820190506157dc5f83018761475e565b81810360208301526157ee8186614945565b90508181036040830152615803818486615098565b905095945050505050565b5f6060820190506158215f83018761475e565b818103602083015261583481858761510a565b9050615843604083018461476d565b95945050505050565b5f60408201905061585f5f83018561475e565b61586c602083018461475e565b9392505050565b5f6080820190506158865f83018961475e565b818103602083015261589981878961510a565b905081810360408301526158ae81858761510a565b90506158bd606083018461476d565b979650505050505050565b5f6060820190506158db5f83018761475e565b81810360208301526158ed8186614945565b9050818103604083015261590281848661510a565b905095945050505050565b7f4549503731323a20556e696e697469616c697a656400000000000000000000005f82015250565b5f61594160158361400f565b915061594c8261590d565b602082019050919050565b5f6020820190508181035f83015261596e81615935565b9050919050565b5f6080820190506159885f83018761475e565b615995602083018661475e565b6159a2604083018561475e565b6159af6060830184614289565b95945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f602082840312156159fa576159f96140b8565b5b5f615a0784828501614f42565b91505092915050565b5f81905092915050565b5f615a258385615a10565b9350615a3283858461452f565b82840190509392505050565b5f615a4a828486615a1a565b91508190509392505050565b615a5f81614a07565b82525050565b5f606082019050615a785f83018661460e565b615a856020830185615a56565b615a92604083018461460e565b949350505050565b5f81519050919050565b5f81905092915050565b5f819050602082019050919050565b615ac681614605565b82525050565b5f615ad78383615abd565b60208301905092915050565b5f602082019050919050565b5f615af982615a9a565b615b038185615aa4565b9350615b0e83615aae565b805f5b83811015615b3e578151615b258882615acc565b9750615b3083615ae3565b925050600181019050615b11565b5085935050505092915050565b5f615b568284615aef565b915081905092915050565b5f608082019050615b745f83018761460e565b615b81602083018661475e565b615b8e604083018561475e565b615b9b606083018461460e565b95945050505050565b5f81519050615bb2816140c9565b92915050565b5f60208284031215615bcd57615bcc6140b8565b5b5f615bda84828501615ba4565b91505092915050565b615bec81614605565b8114615bf6575f80fd5b50565b5f81519050615c0781615be3565b92915050565b5f60208284031215615c2257615c216140b8565b5b5f615c2f84828501615bf9565b91505092915050565b5f604082019050615c4b5f83018561460e565b615c58602083018461475e565b9392505050565b5f819050815f5260205f209050919050565b601f821115615cb257615c8381615c5f565b615c8c84615345565b81016020851015615c9b578190505b615caf615ca785615345565b830182615425565b50505b505050565b615cc082614005565b67ffffffffffffffff811115615cd957615cd8614487565b5b615ce38254615303565b615cee828285615c71565b5f60209050601f831160018114615d1f575f8415615d0d578287015190505b615d1785826154b5565b865550615d7e565b601f198416615d2d86615c5f565b5f5b82811015615d5457848901518255600182019150602085019450602081019050615d2f565b86831015615d715784890151615d6d601f891682615499565b8355505b6001600288020188555050505b505050505050565b5f604082019050615d995f83018561476d565b615da6602083018461476d565b9392505050565b5f615db782614a27565b615dc18185615a10565b9350615dd181856020860161401f565b80840191505092915050565b5f615de88284615dad565b915081905092915050565b5f60a082019050615e065f83018861460e565b615e13602083018761460e565b615e20604083018661460e565b615e2d606083018561475e565b615e3a608083018461476d565b9695505050505050565b5f60ff82169050919050565b615e5981615e44565b82525050565b5f608082019050615e725f83018761460e565b615e7f6020830186615e50565b615e8c604083018561460e565b615e99606083018461460e565b9594505050505056fe43727367656e566572696669636174696f6e2875696e743235362063727349642c75696e74323536206d61784269744c656e6774682c62797465732063727344696765737429507265704b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e4964294b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e49642c75696e74323536206b657949642c4b65794469676573745b5d206b657944696765737473294b65794469676573742875696e7438206b6579547970652c627974657320646967657374294b65794469676573742875696e7438206b6579547970652c62797465732064696765737429
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP4\x80\x15b\0\0CW_\x80\xFD[Pb\0\0Tb\0\0Z` \x1B` \x1CV[b\0\x01\xE1V[_b\0\0kb\0\x01^` \x1B` \x1CV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15b\0\0\xB6W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14b\0\x01[Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Qb\0\x01R\x91\x90b\0\x01\xC6V[`@Q\x80\x91\x03\x90\xA1[PV[_\x80b\0\x01pb\0\x01y` \x1B` \x1CV[\x90P\x80\x91PP\x90V[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0_\x1B\x90P\x90V[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[b\0\x01\xC0\x81b\0\x01\xA2V[\x82RPPV[_` \x82\x01\x90Pb\0\x01\xDB_\x83\x01\x84b\0\x01\xB5V[\x92\x91PPV[`\x80Qa_\xACb\0\x02\x08_9_\x81\x81a.\x9B\x01R\x81\x81a.\xF0\x01Ra1\x92\x01Ra_\xAC_\xF3\xFE`\x80`@R`\x046\x10a\x01)W_5`\xE0\x1C\x80cX\x9A\xDB\x0E\x11a\0\xAAW\x80c\xAD<\xB1\xCC\x11a\0nW\x80c\xAD<\xB1\xCC\x14a\x03\xA0W\x80c\xBA\xFF!\x1E\x14a\x03\xCAW\x80c\xC5[\x87$\x14a\x03\xF4W\x80c\xCA\xA3g\xDB\x14a\x041W\x80c\xD5/\x10\xEB\x14a\x04YW\x80c\xD6]\x83s\x14a\x04\x83Wa\x01)V[\x80cX\x9A\xDB\x0E\x14a\x02\xCDW\x80cb\x97\x87\x87\x14a\x02\xF5W\x80cu\x14\xA2\xAC\x14a\x03\x1DW\x80c\x84\xB0\x19n\x14a\x033W\x80c\x93f\x08\xAE\x14a\x03cWa\x01)V[\x80c<\x02\xF84\x11a\0\xF1W\x80c<\x02\xF84\x14a\x01\xFBW\x80cE\xAF&\x1B\x14a\x02#W\x80cF\x10\xFF\xE8\x14a\x02_W\x80cO\x1E\xF2\x86\x14a\x02\x87W\x80cR\xD1\x90-\x14a\x02\xA3Wa\x01)V[\x80c\r\x8En,\x14a\x01-W\x80c\x12:\xBB(\x14a\x01WW\x80c\x16\xC7\x13\xD9\x14a\x01mW\x80c\x19\xF4\xF62\x14a\x01\xA9W\x80c9\xF78\x10\x14a\x01\xE5W[_\x80\xFD[4\x80\x15a\x018W_\x80\xFD[Pa\x01Aa\x04\xABV[`@Qa\x01N\x91\x90a@\x8FV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01bW_\x80\xFD[Pa\x01ka\x05&V[\0[4\x80\x15a\x01xW_\x80\xFD[Pa\x01\x93`\x04\x806\x03\x81\x01\x90a\x01\x8E\x91\x90a@\xF3V[a\x06KV[`@Qa\x01\xA0\x91\x90aB\x05V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xB4W_\x80\xFD[Pa\x01\xCF`\x04\x806\x03\x81\x01\x90a\x01\xCA\x91\x90a@\xF3V[a\x07\x1CV[`@Qa\x01\xDC\x91\x90aB\x98V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xF0W_\x80\xFD[Pa\x01\xF9a\x07\xC9V[\0[4\x80\x15a\x02\x06W_\x80\xFD[Pa\x02!`\x04\x806\x03\x81\x01\x90a\x02\x1C\x91\x90aB\xD4V[a\n8V[\0[4\x80\x15a\x02.W_\x80\xFD[Pa\x02I`\x04\x806\x03\x81\x01\x90a\x02D\x91\x90a@\xF3V[a\x0CvV[`@Qa\x02V\x91\x90aB\x98V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02jW_\x80\xFD[Pa\x02\x85`\x04\x806\x03\x81\x01\x90a\x02\x80\x91\x90aC\xC8V[a\r\x0BV[\0[a\x02\xA1`\x04\x806\x03\x81\x01\x90a\x02\x9C\x91\x90aE\xABV[a\x13+V[\0[4\x80\x15a\x02\xAEW_\x80\xFD[Pa\x02\xB7a\x13JV[`@Qa\x02\xC4\x91\x90aF\x1DV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xD8W_\x80\xFD[Pa\x02\xF3`\x04\x806\x03\x81\x01\x90a\x02\xEE\x91\x90aF6V[a\x13{V[\0[4\x80\x15a\x03\0W_\x80\xFD[Pa\x03\x1B`\x04\x806\x03\x81\x01\x90a\x03\x16\x91\x90aF\x93V[a\x17]V[\0[4\x80\x15a\x03(W_\x80\xFD[Pa\x031a\x1C\xD2V[\0[4\x80\x15a\x03>W_\x80\xFD[Pa\x03Ga\x1D\xF0V[`@Qa\x03Z\x97\x96\x95\x94\x93\x92\x91\x90aH3V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03nW_\x80\xFD[Pa\x03\x89`\x04\x806\x03\x81\x01\x90a\x03\x84\x91\x90a@\xF3V[a\x1E\xF9V[`@Qa\x03\x97\x92\x91\x90aKEV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xABW_\x80\xFD[Pa\x03\xB4a\"\xA0V[`@Qa\x03\xC1\x91\x90a@\x8FV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xD5W_\x80\xFD[Pa\x03\xDEa\"\xD9V[`@Qa\x03\xEB\x91\x90aKzV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xFFW_\x80\xFD[Pa\x04\x1A`\x04\x806\x03\x81\x01\x90a\x04\x15\x91\x90a@\xF3V[a\"\xF0V[`@Qa\x04(\x92\x91\x90aK\xDBV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04<W_\x80\xFD[Pa\x04W`\x04\x806\x03\x81\x01\x90a\x04R\x91\x90aL\x10V[a&\x02V[\0[4\x80\x15a\x04dW_\x80\xFD[Pa\x04ma({V[`@Qa\x04z\x91\x90aKzV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\x8EW_\x80\xFD[Pa\x04\xA9`\x04\x806\x03\x81\x01\x90a\x04\xA4\x91\x90a@\xF3V[a(\x92V[\0[```@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x04\xEC_a*\x8DV[a\x04\xF6`\x04a*\x8DV[a\x04\xFF_a*\x8DV[`@Q` \x01a\x05\x12\x94\x93\x92\x91\x90aM\tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`\x05_a\x051a+WV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x05yWP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x05\xB0W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x06?\x91\x90aM\x89V[`@Q\x80\x91\x03\x90\xA1PPV[``_a\x06Va+jV[\x90P_\x81`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\x02\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x82\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07\x0EW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x06\xC5W[PPPPP\x92PPP\x91\x90PV[_\x80a\x07&a+jV[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x07\x89W\x82`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\x80\x91\x90aKzV[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\r\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x92PPP\x91\x90PV[`\x01a\x07\xD3a+\x91V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x08\x14W`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05_a\x08\x1Fa+WV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x08gWP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x08\x9EW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\tW`@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa+\xB5V[_a\t`a+jV[\x90P`\xF8`\x03`\x06\x81\x11\x15a\txWa\twaB%V[[\x90\x1B\x81`\x04\x01\x81\x90UP`\xF8`\x04`\x06\x81\x11\x15a\t\x98Wa\t\x97aB%V[[\x90\x1B\x81`\x05\x01\x81\x90UP`\xF8`\x05`\x06\x81\x11\x15a\t\xB8Wa\t\xB7aB%V[[\x90\x1B\x81`\t\x01\x81\x90UP`\xF8`\x06\x80\x81\x11\x15a\t\xD7Wa\t\xD6aB%V[[\x90\x1B\x81`\x0E\x01\x81\x90UPP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\n,\x91\x90aM\x89V[`@Q\x80\x91\x03\x90\xA1PPV[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x95W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xB9\x91\x90aM\xB6V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0B(W3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B\x1F\x91\x90aM\xE1V[`@Q\x80\x91\x03\x90\xFD[_a\x0B1a+jV[\x90P_\x81`\t\x01T\x90P`\xF8`\x05`\x06\x81\x11\x15a\x0BQWa\x0BPaB%V[[\x90\x1B\x81\x14\x15\x80\x15a\x0B\x7FWP\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15a\x0B\xC1W\x80`@Q\x7F\x06\x1A\xC6\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B\xB8\x91\x90aKzV[`@Q\x80\x91\x03\x90\xFD[\x81`\t\x01_\x81T\x80\x92\x91\x90a\x0B\xD5\x90aN'V[\x91\x90PUP_\x82`\t\x01T\x90P\x84\x83`\n\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x83\x83`\r\x01_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a\x0C/Wa\x0C.aB%V[[\x02\x17\x90UP\x7F?\x03\x8Fo\x88\xCB01\xB7q\x85\x88@:.\xC2 Wj\x86\x8B\xE0}\xDEL\x02\xB8F\xCA5.\xF5\x81\x86\x86`@Qa\x0Cg\x93\x92\x91\x90aNnV[`@Q\x80\x91\x03\x90\xA1PPPPPV[_\x80a\x0C\x80a+jV[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x0C\xE3W\x82`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\xDA\x91\x90aKzV[`@Q\x80\x91\x03\x90\xFD[\x80`\r\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE5'^\xAF3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\rX\x91\x90aM\xE1V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rsW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x97\x91\x90aN\xD8V[a\r\xD8W3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\xCF\x91\x90aM\xE1V[`@Q\x80\x91\x03\x90\xFD[_a\r\xE1a+jV[\x90P\x80`\x05\x01T\x86\x11\x80a\r\xF4WP_\x86\x14[\x15a\x0E6W\x85`@Q\x7F\xAD\xFA\xB9\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E-\x91\x90aKzV[`@Q\x80\x91\x03\x90\xFD[_\x85\x85\x90P\x03a\x0E}W\x85`@Q\x7F\xE6\xF9\x08;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0Et\x91\x90aKzV[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x88\x81R` \x01\x90\x81R` \x01_ T\x90P_a\x0E\xA1\x82\x89\x89\x89a+\xCBV[\x90P_a\x0E\xAF\x82\x87\x87a-\xA2V[\x90P\x83_\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x0FOW\x88\x81`@Q\x7F\x98\xFB\x95}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0FF\x92\x91\x90aO\x03V[`@Q\x80\x91\x03\x90\xFD[`\x01\x84_\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x84`\x02\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_\x81\x80T\x90P\x90P\x7F*\xFEd\xFB:\xFD\xE8\xE2g\x8A\xEA\x84\xCF6\"?3\x0E/\xB1(m7\xAE\xD5s\xAB\x9C\xD1\xDBG\xC7\x8B\x8B\x8B\x8B\x8B3`@Qa\x10y\x96\x95\x94\x93\x92\x91\x90aQ6V[`@Q\x80\x91\x03\x90\xA1\x85`\x01\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x10\xB3WPa\x10\xB2\x81a.\x08V[[\x15a\x13\x1EW`\x01\x86`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_[\x8A\x8A\x90P\x81\x10\x15a\x11iW\x86`\x07\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x8B\x8B\x83\x81\x81\x10a\x11\x16Wa\x11\x15aQ\x8BV[[\x90P` \x02\x81\x01\x90a\x11(\x91\x90aQ\xC4V[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P\x81\x81a\x11Z\x91\x90aU\xF1V[PP\x80\x80`\x01\x01\x91PPa\x10\xE5V[P\x83\x86`\x03\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x8A\x86`\x08\x01\x81\x90UP_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xA6Wa\x11\xA5aD\x87V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11\xD9W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x11\xC4W\x90P[P\x90P_[\x82\x81\x10\x15a\x12\xDEWs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3\xB2\xA8t\x85\x83\x81T\x81\x10a\x12)Wa\x12(aQ\x8BV[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12m\x91\x90aM\xE1V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\x87W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xAF\x91\x90aW\x82V[``\x01Q\x82\x82\x81Q\x81\x10a\x12\xC6Wa\x12\xC5aQ\x8BV[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa\x11\xDEV[P\x7F\xEB\x85\xC2m\xBC\xADF\xB8\nh\xA0\xF2L\xCE|,\x90\xF0\xA1\xFA\xDE\xD8A\x84\x13\x889\xFC\x9E\x80\xA2[\x8C\x82\x8D\x8D`@Qa\x13\x14\x94\x93\x92\x91\x90aW\xC9V[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPPPPPV[a\x133a.\x99V[a\x13<\x82a/\x7FV[a\x13F\x82\x82a0rV[PPV[_a\x13Sa1\x90V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE5'^\xAF3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\xC8\x91\x90aM\xE1V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xE3W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x07\x91\x90aN\xD8V[a\x14HW3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14?\x91\x90aM\xE1V[`@Q\x80\x91\x03\x90\xFD[_a\x14Qa+jV[\x90P\x80`\x04\x01T\x84\x11\x80a\x14dWP_\x84\x14[\x15a\x14\xA6W\x83`@Q\x7F\n\xB7\xF6\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14\x9D\x91\x90aKzV[`@Q\x80\x91\x03\x90\xFD[_a\x14\xB0\x85a2\x17V[\x90P_a\x14\xBE\x82\x86\x86a-\xA2V[\x90P\x82_\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x15^W\x85\x81`@Q\x7F3\xCA\x1F\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15U\x92\x91\x90aO\x03V[`@Q\x80\x91\x03\x90\xFD[`\x01\x83_\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x83`\x02\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7FLq\\W4\xCE\\\x18\xC9\xC1.\x84\x96\xE5=*e\xF1\xEC8\x1DGiW\xF0\xF5\x96\xB3d\xA5\x9B\x0C\x87\x87\x873`@Qa\x16|\x94\x93\x92\x91\x90aX\x0EV[`@Q\x80\x91\x03\x90\xA1\x83`\x01\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x16\xBAWPa\x16\xB9\x81\x80T\x90Pa.\x08V[[\x15a\x17TW`\x01\x84`\x01\x01_\x89\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x82\x84`\x03\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x81\x90UP_\x84`\x06\x01_\x89\x81R` \x01\x90\x81R` \x01_ T\x90P\x7Fx\xB1y\x17m\x1F\x19\xD7\xC2\x8E\x80\x82=\xEB\xA2bM\xA2\xCA.\xC6K\x17\x01\xF3c*\x87\xC9\xAE\xDC\x92\x88\x82`@Qa\x17J\x92\x91\x90aXLV[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPV[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE5'^\xAF3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\xAA\x91\x90aM\xE1V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xC5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xE9\x91\x90aN\xD8V[a\x18*W3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18!\x91\x90aM\xE1V[`@Q\x80\x91\x03\x90\xFD[_a\x183a+jV[\x90P\x80`\t\x01T\x86\x11\x80a\x18FWP_\x86\x14[\x15a\x18\x88W\x85`@Q\x7F\x8D\x8C\x94\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18\x7F\x91\x90aKzV[`@Q\x80\x91\x03\x90\xFD[_\x81`\n\x01_\x88\x81R` \x01\x90\x81R` \x01_ T\x90P_a\x18\xAC\x88\x83\x89\x89a2oV[\x90P_a\x18\xBA\x82\x87\x87a-\xA2V[\x90P\x83_\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x19ZW\x88\x81`@Q\x7F\xFC\xF5\xA6\xE9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19Q\x92\x91\x90aO\x03V[`@Q\x80\x91\x03\x90\xFD[`\x01\x84_\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x84`\x02\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_\x81\x80T\x90P\x90P\x7F{\xF1\xB4,\x10\xE9I|\x87\x96 \xC5\xB7\xAF\xCE\xD1\x0B\xDA\x17\xD8\xC9\x0B\"\xF0\xE3\xBCk/\xD6\xCE\xD0\xBD\x8B\x8B\x8B\x8B\x8B3`@Qa\x1A\x84\x96\x95\x94\x93\x92\x91\x90aXsV[`@Q\x80\x91\x03\x90\xA1\x85`\x01\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x1A\xBEWPa\x1A\xBD\x81a.\x08V[[\x15a\x1C\xC5W`\x01\x86`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x89\x89\x87`\x0B\x01_\x8E\x81R` \x01\x90\x81R` \x01_ \x91\x82a\x1B\x10\x92\x91\x90aT\xD0V[P\x83\x86`\x03\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x8A\x86`\x0C\x01\x81\x90UP_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1BMWa\x1BLaD\x87V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1B\x80W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1BkW\x90P[P\x90P_[\x82\x81\x10\x15a\x1C\x85Ws\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3\xB2\xA8t\x85\x83\x81T\x81\x10a\x1B\xD0Wa\x1B\xCFaQ\x8BV[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1C\x14\x91\x90aM\xE1V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C.W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1CV\x91\x90aW\x82V[``\x01Q\x82\x82\x81Q\x81\x10a\x1CmWa\x1ClaQ\x8BV[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa\x1B\x85V[P\x7F\"X\xB7?\xAE\xD3?\xB2\xE2\xEAED\x03\xBE\xF9t\x92\x0C\xAFh*\xB3\xA7#HO\xCFgU;\x16\xA2\x8C\x82\x8D\x8D`@Qa\x1C\xBB\x94\x93\x92\x91\x90aX\xC8V[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPPPPPV[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D/W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1DS\x91\x90aM\xB6V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x1D\xC2W3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1D\xB9\x91\x90aM\xE1V[`@Q\x80\x91\x03\x90\xFD[\x7F\x11\xDBB\xC1\x87\x8F.(\x19$\x1FRP\x98Ec\xF0l\xF2(\x18\xE7\xAD\xB8jf\x92\x1D\x15\xD5\x9D?`@Q`@Q\x80\x91\x03\x90\xA1V[_``\x80_\x80_``_a\x1E\x02a2\xF6V[\x90P_\x80\x1B\x81_\x01T\x14\x80\x15a\x1E\x1DWP_\x80\x1B\x81`\x01\x01T\x14[a\x1E\\W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1ES\x90aYWV[`@Q\x80\x91\x03\x90\xFD[a\x1Eda3\x1DV[a\x1Ela3\xBBV[F0_\x80\x1B_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\x8BWa\x1E\x8AaD\x87V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E\xB9W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94\x93\x92\x91\x90\x97P\x97P\x97P\x97P\x97P\x97P\x97PP\x90\x91\x92\x93\x94\x95\x96V[``\x80_a\x1F\x05a+jV[\x90P\x80`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x1FhW\x83`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1F_\x91\x90aKzV[`@Q\x80\x91\x03\x90\xFD[_\x81`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x82`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a \x1FW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x1F\xD6W[PPPPP\x90P_\x81Q\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a FWa EaD\x87V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a yW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a dW\x90P[P\x90P_[\x82\x81\x10\x15a!^Ws\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3\xB2\xA8t\x85\x83\x81Q\x81\x10a \xC9Wa \xC8aQ\x8BV[[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a \xED\x91\x90aM\xE1V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\x07W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!/\x91\x90aW\x82V[``\x01Q\x82\x82\x81Q\x81\x10a!FWa!EaQ\x8BV[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa ~V[P\x80\x85`\x07\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\"\x8CW\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x01\x81\x11\x15a!\xD7Wa!\xD6aB%V[[`\x01\x81\x11\x15a!\xE9Wa!\xE8aB%V[[\x81R` \x01`\x01\x82\x01\x80Ta!\xFD\x90aS\x03V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\")\x90aS\x03V[\x80\x15a\"tW\x80`\x1F\x10a\"KWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\"tV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\"WW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a!\x93V[PPPP\x90P\x96P\x96PPPPPP\x91P\x91V[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[_\x80a\"\xE3a+jV[\x90P\x80`\x0C\x01T\x91PP\x90V[``\x80_a\"\xFCa+jV[\x90P\x80`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a#_W\x83`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a#V\x91\x90aKzV[`@Q\x80\x91\x03\x90\xFD[_\x81`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x82`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a$\x16W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a#\xCDW[PPPPP\x90P_\x81Q\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$=Wa$<aD\x87V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a$pW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a$[W\x90P[P\x90P_[\x82\x81\x10\x15a%UWs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3\xB2\xA8t\x85\x83\x81Q\x81\x10a$\xC0Wa$\xBFaQ\x8BV[[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a$\xE4\x91\x90aM\xE1V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\xFEW=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%&\x91\x90aW\x82V[``\x01Q\x82\x82\x81Q\x81\x10a%=Wa%<aQ\x8BV[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa$uV[P\x80\x85`\x0B\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80\x80Ta%v\x90aS\x03V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta%\xA2\x90aS\x03V[\x80\x15a%\xEDW\x80`\x1F\x10a%\xC4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a%\xEDV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a%\xD0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x96P\x96PPPPPP\x91P\x91V[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&_W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\x83\x91\x90aM\xB6V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a&\xF2W3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&\xE9\x91\x90aM\xE1V[`@Q\x80\x91\x03\x90\xFD[_a&\xFBa+jV[\x90P_\x81`\x05\x01T\x90P`\xF8`\x04`\x06\x81\x11\x15a'\x1BWa'\x1AaB%V[[\x90\x1B\x81\x14\x15\x80\x15a'IWP\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15a'\x8BW\x80`@Q\x7F;\x85=\xA8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a'\x82\x91\x90aKzV[`@Q\x80\x91\x03\x90\xFD[\x81`\x04\x01_\x81T\x80\x92\x91\x90a'\x9F\x90aN'V[\x91\x90PUP_\x82`\x04\x01T\x90P\x82`\x05\x01_\x81T\x80\x92\x91\x90a'\xC0\x90aN'V[\x91\x90PUP_\x83`\x05\x01T\x90P\x80\x84`\x06\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x81\x84`\x06\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UP_\x85\x85`\r\x01_\x85\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a(3Wa(2aB%V[[\x02\x17\x90UP\x7F\x02\x02@\x07\xD9et\xDB\xC9\xD1\x13(\xBF\xEE\x98\x93\xE7\xC7\xBBN\xF4\xAA\x80m\xF3;\xFD\xF4T\xEB^`\x83\x82\x88`@Qa(k\x93\x92\x91\x90aNnV[`@Q\x80\x91\x03\x90\xA1PPPPPPV[_\x80a(\x85a+jV[\x90P\x80`\x08\x01T\x91PP\x90V[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(\xEFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\x13\x91\x90aM\xB6V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a)\x82W3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a)y\x91\x90aM\xE1V[`@Q\x80\x91\x03\x90\xFD[_a)\x8Ba+jV[\x90P\x80`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a)\xEEW\x81`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a)\xE5\x91\x90aKzV[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P_\x82`\r\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x82`\x0E\x01_\x81T\x80\x92\x91\x90a*<\x90aN'V[\x91\x90PUP_\x83`\x0E\x01T\x90P\x7F\x1C\xCBUE\xC4\xC8\xDBP\xA0\xF5\xB4\x16I\x95&\x92\x9FhSN\xD4\x7Fl\xFDL\x9F\x06\x90u\xE6\x0BE\x83\x86\x83\x85`@Qa*~\x94\x93\x92\x91\x90aYuV[`@Q\x80\x91\x03\x90\xA1PPPPPV[``_`\x01a*\x9B\x84a4YV[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xB9Wa*\xB8aD\x87V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a*\xEBW\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x83\x01\x01\x90P[`\x01\x15a+LW\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a+AWa+@aY\xB8V[[\x04\x94P_\x85\x03a*\xF8W[\x81\x93PPPP\x91\x90PV[_\x80a+aa5\xAAV[\x90P\x80\x91PP\x90V[_\x7F\x0B\x8F\xDB\x1F\ncV\xDD \xA6\xCB\xC6\xF9f\x8F\xAC#\xB8_\x96W]\x10\xE33\xE6\x03\xFA\xA7\x94\xAC\0\x90P\x90V[_a+\x9Aa+WV[_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a+\xBDa5\xD3V[a+\xC7\x82\x82a6\x13V[PPV[_\x80\x83\x83\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\xEAWa+\xE9aD\x87V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a,\x18W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_[\x84\x84\x90P\x81\x10\x15a-\x1CW`@Q\x80``\x01`@R\x80`%\x81R` \x01a_\x87`%\x919\x80Q\x90` \x01 \x85\x85\x83\x81\x81\x10a,[Wa,ZaQ\x8BV[[\x90P` \x02\x81\x01\x90a,m\x91\x90aQ\xC4V[_\x01` \x81\x01\x90a,~\x91\x90aY\xE5V[\x86\x86\x84\x81\x81\x10a,\x91Wa,\x90aQ\x8BV[[\x90P` \x02\x81\x01\x90a,\xA3\x91\x90aQ\xC4V[\x80` \x01\x90a,\xB2\x91\x90aRjV[`@Qa,\xC0\x92\x91\x90aZ>V[`@Q\x80\x91\x03\x90 `@Q` \x01a,\xDA\x93\x92\x91\x90aZeV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x82\x81Q\x81\x10a-\x03Wa-\x02aQ\x8BV[[` \x02` \x01\x01\x81\x81RPP\x80\x80`\x01\x01\x91PPa,\x1DV[Pa-\x97`@Q\x80`\xA0\x01`@R\x80`r\x81R` \x01a_\x15`r\x919\x80Q\x90` \x01 \x87\x87\x84`@Q` \x01a-S\x91\x90a[KV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a-|\x94\x93\x92\x91\x90a[aV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a6dV[\x91PP\x94\x93PPPPV[_\x80a-\xF1\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa6}V[\x90Pa-\xFD\x813a6\xA7V[\x80\x91PP\x93\x92PPPV[_\x80s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xB4r+\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.gW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\x8B\x91\x90a[\xB8V[\x90P\x80\x83\x10\x15\x91PP\x91\x90PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a/FWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a/-a7\xB8V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a/}W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/\xDCW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\0\x91\x90aM\xB6V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a0oW3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a0f\x91\x90aM\xE1V[`@Q\x80\x91\x03\x90\xFD[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a0\xDAWP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\xD7\x91\x90a\\\rV[`\x01[a1\x1BW\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a1\x12\x91\x90aM\xE1V[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14a1\x81W\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a1x\x91\x90aF\x1DV[`@Q\x80\x91\x03\x90\xFD[a1\x8B\x83\x83a8\x0BV[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a2\x15W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_a2h`@Q\x80``\x01`@R\x80`,\x81R` \x01a^\xE9`,\x919\x80Q\x90` \x01 \x83`@Q` \x01a2M\x92\x91\x90a\\8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a6dV[\x90P\x91\x90PV[_a2\xEC`@Q\x80`\x80\x01`@R\x80`F\x81R` \x01a^\xA3`F\x919\x80Q\x90` \x01 \x86\x86\x86\x86`@Q` \x01a2\xA8\x92\x91\x90aZ>V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a2\xD1\x94\x93\x92\x91\x90a[aV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a6dV[\x90P\x94\x93PPPPV[_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x90P\x90V[``_a3(a2\xF6V[\x90P\x80`\x02\x01\x80Ta39\x90aS\x03V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta3e\x90aS\x03V[\x80\x15a3\xB0W\x80`\x1F\x10a3\x87Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a3\xB0V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a3\x93W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[``_a3\xC6a2\xF6V[\x90P\x80`\x03\x01\x80Ta3\xD7\x90aS\x03V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta4\x03\x90aS\x03V[\x80\x15a4NW\x80`\x1F\x10a4%Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a4NV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a41W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[_\x80_\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a4\xB5Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81a4\xABWa4\xAAaY\xB8V[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a4\xF2Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81a4\xE8Wa4\xE7aY\xB8V[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10a5!Wf#\x86\xF2o\xC1\0\0\x83\x81a5\x17Wa5\x16aY\xB8V[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10a5JWc\x05\xF5\xE1\0\x83\x81a5@Wa5?aY\xB8V[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10a5oWa'\x10\x83\x81a5eWa5daY\xB8V[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10a5\x92W`d\x83\x81a5\x88Wa5\x87aY\xB8V[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10a5\xA1W`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0_\x1B\x90P\x90V[a5\xDBa8}V[a6\x11W`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a6\x1Ba5\xD3V[_a6$a2\xF6V[\x90P\x82\x81`\x02\x01\x90\x81a67\x91\x90a\\\xB7V[P\x81\x81`\x03\x01\x90\x81a6I\x91\x90a\\\xB7V[P_\x80\x1B\x81_\x01\x81\x90UP_\x80\x1B\x81`\x01\x01\x81\x90UPPPPV[_a6va6pa8\x9BV[\x83a8\xA9V[\x90P\x91\x90PV[_\x80_\x80a6\x8B\x86\x86a8\xE9V[\x92P\x92P\x92Pa6\x9B\x82\x82a9>V[\x82\x93PPPP\x92\x91PPV[a6\xB0\x82a:\xA0V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3\xB2\xA8t\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a7\x14\x91\x90aM\xE1V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7.W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7V\x91\x90aW\x82V[` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a7\xB4W\x81\x81`@Q\x7F\r\x86\xF5!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a7\xAB\x92\x91\x90a]\x86V[`@Q\x80\x91\x03\x90\xFD[PPV[_a7\xE4\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba;pV[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a8\x14\x82a;yV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15a8pWa8j\x82\x82a<BV[Pa8yV[a8xa<\xC2V[[PPV[_a8\x86a+WV[_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[_a8\xA4a<\xFEV[\x90P\x90V[_`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x02\x82\x01R\x82`\"\x82\x01R`B\x81 \x91PP\x92\x91PPV[_\x80_`A\x84Q\x03a9)W_\x80_` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90Pa9\x1B\x88\x82\x85\x85a=aV[\x95P\x95P\x95PPPPa97V[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15a9QWa9PaB%V[[\x82`\x03\x81\x11\x15a9dWa9caB%V[[\x03\x15a:\x9CW`\x01`\x03\x81\x11\x15a9~Wa9}aB%V[[\x82`\x03\x81\x11\x15a9\x91Wa9\x90aB%V[[\x03a9\xC8W`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15a9\xDCWa9\xDBaB%V[[\x82`\x03\x81\x11\x15a9\xEFWa9\xEEaB%V[[\x03a:3W\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a:*\x91\x90aKzV[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15a:FWa:EaB%V[[\x82`\x03\x81\x11\x15a:YWa:XaB%V[[\x03a:\x9BW\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a:\x92\x91\x90aF\x1DV[`@Q\x80\x91\x03\x90\xFD[[PPV[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c =\x01\x14\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a:\xED\x91\x90aM\xE1V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;\x08W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;,\x91\x90aN\xD8V[a;mW\x80`@Q\x7F*|n\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a;d\x91\x90aM\xE1V[`@Q\x80\x91\x03\x90\xFD[PV[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a;\xD4W\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a;\xCB\x91\x90aM\xE1V[`@Q\x80\x91\x03\x90\xFD[\x80a<\0\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba;pV[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qa<k\x91\x90a]\xDDV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a<\xA3W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a<\xA8V[``\x91P[P\x91P\x91Pa<\xB8\x85\x83\x83a>HV[\x92PPP\x92\x91PPV[_4\x11\x15a<\xFCW`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa=(a>\xD5V[a=0a?KV[F0`@Q` \x01a=F\x95\x94\x93\x92\x91\x90a]\xF3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x80_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15a=\x9DW_`\x03\x85\x92P\x92P\x92Pa>>V[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@Qa=\xC0\x94\x93\x92\x91\x90a^_V[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a=\xE0W=_\x80>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a>1W_`\x01_\x80\x1B\x93P\x93P\x93PPa>>V[\x80_\x80_\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[``\x82a>]Wa>X\x82a?\xC2V[a>\xCDV[_\x82Q\x14\x80\x15a>\x83WP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a>\xC5W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a>\xBC\x91\x90aM\xE1V[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa>\xCEV[[\x93\x92PPPV[_\x80a>\xDFa2\xF6V[\x90P_a>\xEAa3\x1DV[\x90P_\x81Q\x11\x15a?\x06W\x80\x80Q\x90` \x01 \x92PPPa?HV[_\x82_\x01T\x90P_\x80\x1B\x81\x14a?!W\x80\x93PPPPa?HV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x80a?Ua2\xF6V[\x90P_a?`a3\xBBV[\x90P_\x81Q\x11\x15a?|W\x80\x80Q\x90` \x01 \x92PPPa?\xBFV[_\x82`\x01\x01T\x90P_\x80\x1B\x81\x14a?\x98W\x80\x93PPPPa?\xBFV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x81Q\x11\x15a?\xD3W\x80Q` \x82\x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15a@<W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa@!V[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a@a\x82a@\x05V[a@k\x81\x85a@\x0FV[\x93Pa@{\x81\x85` \x86\x01a@\x1FV[a@\x84\x81a@GV[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra@\xA7\x81\x84a@WV[\x90P\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x81\x90P\x91\x90PV[a@\xD2\x81a@\xC0V[\x81\x14a@\xDCW_\x80\xFD[PV[_\x815\x90Pa@\xED\x81a@\xC9V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aA\x08WaA\x07a@\xB8V[[_aA\x15\x84\x82\x85\x01a@\xDFV[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_aAp\x82aAGV[\x90P\x91\x90PV[aA\x80\x81aAfV[\x82RPPV[_aA\x91\x83\x83aAwV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aA\xB3\x82aA\x1EV[aA\xBD\x81\x85aA(V[\x93PaA\xC8\x83aA8V[\x80_[\x83\x81\x10\x15aA\xF8W\x81QaA\xDF\x88\x82aA\x86V[\x97PaA\xEA\x83aA\x9DV[\x92PP`\x01\x81\x01\x90PaA\xCBV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaB\x1D\x81\x84aA\xA9V[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x02\x81\x10aBcWaBbaB%V[[PV[_\x81\x90PaBs\x82aBRV[\x91\x90PV[_aB\x82\x82aBfV[\x90P\x91\x90PV[aB\x92\x81aBxV[\x82RPPV[_` \x82\x01\x90PaB\xAB_\x83\x01\x84aB\x89V[\x92\x91PPV[`\x02\x81\x10aB\xBDW_\x80\xFD[PV[_\x815\x90PaB\xCE\x81aB\xB1V[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15aB\xEAWaB\xE9a@\xB8V[[_aB\xF7\x85\x82\x86\x01a@\xDFV[\x92PP` aC\x08\x85\x82\x86\x01aB\xC0V[\x91PP\x92P\x92\x90PV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12aC3WaC2aC\x12V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aCPWaCOaC\x16V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aClWaCkaC\x1AV[[\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12aC\x88WaC\x87aC\x12V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aC\xA5WaC\xA4aC\x16V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15aC\xC1WaC\xC0aC\x1AV[[\x92P\x92\x90PV[_\x80_\x80_``\x86\x88\x03\x12\x15aC\xE1WaC\xE0a@\xB8V[[_aC\xEE\x88\x82\x89\x01a@\xDFV[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aD\x0FWaD\x0Ea@\xBCV[[aD\x1B\x88\x82\x89\x01aC\x1EV[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aD>WaD=a@\xBCV[[aDJ\x88\x82\x89\x01aCsV[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[aDb\x81aAfV[\x81\x14aDlW_\x80\xFD[PV[_\x815\x90PaD}\x81aDYV[\x92\x91PPV[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[aD\xBD\x82a@GV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aD\xDCWaD\xDBaD\x87V[[\x80`@RPPPV[_aD\xEEa@\xAFV[\x90PaD\xFA\x82\x82aD\xB4V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aE\x19WaE\x18aD\x87V[[aE\"\x82a@GV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_aEOaEJ\x84aD\xFFV[aD\xE5V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15aEkWaEjaD\x83V[[aEv\x84\x82\x85aE/V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aE\x92WaE\x91aC\x12V[[\x815aE\xA2\x84\x82` \x86\x01aE=V[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15aE\xC1WaE\xC0a@\xB8V[[_aE\xCE\x85\x82\x86\x01aDoV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aE\xEFWaE\xEEa@\xBCV[[aE\xFB\x85\x82\x86\x01aE~V[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[aF\x17\x81aF\x05V[\x82RPPV[_` \x82\x01\x90PaF0_\x83\x01\x84aF\x0EV[\x92\x91PPV[_\x80_`@\x84\x86\x03\x12\x15aFMWaFLa@\xB8V[[_aFZ\x86\x82\x87\x01a@\xDFV[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aF{WaFza@\xBCV[[aF\x87\x86\x82\x87\x01aCsV[\x92P\x92PP\x92P\x92P\x92V[_\x80_\x80_``\x86\x88\x03\x12\x15aF\xACWaF\xABa@\xB8V[[_aF\xB9\x88\x82\x89\x01a@\xDFV[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aF\xDAWaF\xD9a@\xBCV[[aF\xE6\x88\x82\x89\x01aCsV[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aG\tWaG\x08a@\xBCV[[aG\x15\x88\x82\x89\x01aCsV[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[_\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[aGX\x81aG$V[\x82RPPV[aGg\x81a@\xC0V[\x82RPPV[aGv\x81aAfV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[aG\xAE\x81a@\xC0V[\x82RPPV[_aG\xBF\x83\x83aG\xA5V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aG\xE1\x82aG|V[aG\xEB\x81\x85aG\x86V[\x93PaG\xF6\x83aG\x96V[\x80_[\x83\x81\x10\x15aH&W\x81QaH\r\x88\x82aG\xB4V[\x97PaH\x18\x83aG\xCBV[\x92PP`\x01\x81\x01\x90PaG\xF9V[P\x85\x93PPPP\x92\x91PPV[_`\xE0\x82\x01\x90PaHF_\x83\x01\x8AaGOV[\x81\x81\x03` \x83\x01RaHX\x81\x89a@WV[\x90P\x81\x81\x03`@\x83\x01RaHl\x81\x88a@WV[\x90PaH{``\x83\x01\x87aG^V[aH\x88`\x80\x83\x01\x86aGmV[aH\x95`\xA0\x83\x01\x85aF\x0EV[\x81\x81\x03`\xC0\x83\x01RaH\xA7\x81\x84aG\xD7V[\x90P\x98\x97PPPPPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aH\xF8\x82a@\x05V[aI\x02\x81\x85aH\xDEV[\x93PaI\x12\x81\x85` \x86\x01a@\x1FV[aI\x1B\x81a@GV[\x84\x01\x91PP\x92\x91PPV[_aI1\x83\x83aH\xEEV[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aIO\x82aH\xB5V[aIY\x81\x85aH\xBFV[\x93P\x83` \x82\x02\x85\x01aIk\x85aH\xCFV[\x80_[\x85\x81\x10\x15aI\xA6W\x84\x84\x03\x89R\x81QaI\x87\x85\x82aI&V[\x94PaI\x92\x83aI9V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaInV[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[`\x02\x81\x10aI\xF2WaI\xF1aB%V[[PV[_\x81\x90PaJ\x02\x82aI\xE1V[\x91\x90PV[_aJ\x11\x82aI\xF5V[\x90P\x91\x90PV[aJ!\x81aJ\x07V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aJK\x82aJ'V[aJU\x81\x85aJ1V[\x93PaJe\x81\x85` \x86\x01a@\x1FV[aJn\x81a@GV[\x84\x01\x91PP\x92\x91PPV[_`@\x83\x01_\x83\x01QaJ\x8E_\x86\x01\x82aJ\x18V[P` \x83\x01Q\x84\x82\x03` \x86\x01RaJ\xA6\x82\x82aJAV[\x91PP\x80\x91PP\x92\x91PPV[_aJ\xBE\x83\x83aJyV[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aJ\xDC\x82aI\xB8V[aJ\xE6\x81\x85aI\xC2V[\x93P\x83` \x82\x02\x85\x01aJ\xF8\x85aI\xD2V[\x80_[\x85\x81\x10\x15aK3W\x84\x84\x03\x89R\x81QaK\x14\x85\x82aJ\xB3V[\x94PaK\x1F\x83aJ\xC6V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaJ\xFBV[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01RaK]\x81\x85aIEV[\x90P\x81\x81\x03` \x83\x01RaKq\x81\x84aJ\xD2V[\x90P\x93\x92PPPV[_` \x82\x01\x90PaK\x8D_\x83\x01\x84aG^V[\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aK\xAD\x82aJ'V[aK\xB7\x81\x85aK\x93V[\x93PaK\xC7\x81\x85` \x86\x01a@\x1FV[aK\xD0\x81a@GV[\x84\x01\x91PP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01RaK\xF3\x81\x85aIEV[\x90P\x81\x81\x03` \x83\x01RaL\x07\x81\x84aK\xA3V[\x90P\x93\x92PPPV[_` \x82\x84\x03\x12\x15aL%WaL$a@\xB8V[[_aL2\x84\x82\x85\x01aB\xC0V[\x91PP\x92\x91PPV[_\x81\x90P\x92\x91PPV[_aLO\x82a@\x05V[aLY\x81\x85aL;V[\x93PaLi\x81\x85` \x86\x01a@\x1FV[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aL\xA9`\x02\x83aL;V[\x91PaL\xB4\x82aLuV[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aL\xF3`\x01\x83aL;V[\x91PaL\xFE\x82aL\xBFV[`\x01\x82\x01\x90P\x91\x90PV[_aM\x14\x82\x87aLEV[\x91PaM\x1F\x82aL\x9DV[\x91PaM+\x82\x86aLEV[\x91PaM6\x82aL\xE7V[\x91PaMB\x82\x85aLEV[\x91PaMM\x82aL\xE7V[\x91PaMY\x82\x84aLEV[\x91P\x81\x90P\x95\x94PPPPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[aM\x83\x81aMgV[\x82RPPV[_` \x82\x01\x90PaM\x9C_\x83\x01\x84aMzV[\x92\x91PPV[_\x81Q\x90PaM\xB0\x81aDYV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aM\xCBWaM\xCAa@\xB8V[[_aM\xD8\x84\x82\x85\x01aM\xA2V[\x91PP\x92\x91PPV[_` \x82\x01\x90PaM\xF4_\x83\x01\x84aGmV[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_aN1\x82a@\xC0V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03aNcWaNbaM\xFAV[[`\x01\x82\x01\x90P\x91\x90PV[_``\x82\x01\x90PaN\x81_\x83\x01\x86aG^V[aN\x8E` \x83\x01\x85aG^V[aN\x9B`@\x83\x01\x84aB\x89V[\x94\x93PPPPV[_\x81\x15\x15\x90P\x91\x90PV[aN\xB7\x81aN\xA3V[\x81\x14aN\xC1W_\x80\xFD[PV[_\x81Q\x90PaN\xD2\x81aN\xAEV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aN\xEDWaN\xECa@\xB8V[[_aN\xFA\x84\x82\x85\x01aN\xC4V[\x91PP\x92\x91PPV[_`@\x82\x01\x90PaO\x16_\x83\x01\x85aG^V[aO#` \x83\x01\x84aGmV[\x93\x92PPPV[_\x81\x90P\x91\x90PV[`\x02\x81\x10aO?W_\x80\xFD[PV[_\x815\x90PaOP\x81aO3V[\x92\x91PPV[_aOd` \x84\x01\x84aOBV[\x90P\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aO\x94WaO\x93aOtV[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aO\xBCWaO\xBBaOlV[[`\x01\x82\x026\x03\x83\x13\x15aO\xD2WaO\xD1aOpV[[P\x92P\x92\x90PV[_aO\xE5\x83\x85aJ1V[\x93PaO\xF2\x83\x85\x84aE/V[aO\xFB\x83a@GV[\x84\x01\x90P\x93\x92PPPV[_`@\x83\x01aP\x17_\x84\x01\x84aOVV[aP#_\x86\x01\x82aJ\x18V[PaP1` \x84\x01\x84aOxV[\x85\x83\x03` \x87\x01RaPD\x83\x82\x84aO\xDAV[\x92PPP\x80\x91PP\x92\x91PPV[_aP]\x83\x83aP\x06V[\x90P\x92\x91PPV[_\x825`\x01`@\x03\x836\x03\x03\x81\x12aP\x80WaP\x7FaOtV[[\x82\x81\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aP\xA3\x83\x85aI\xC2V[\x93P\x83` \x84\x02\x85\x01aP\xB5\x84aO*V[\x80_[\x87\x81\x10\x15aP\xF8W\x84\x84\x03\x89RaP\xCF\x82\x84aPeV[aP\xD9\x85\x82aPRV[\x94PaP\xE4\x83aP\x8CV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaP\xB8V[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_aQ\x15\x83\x85aK\x93V[\x93PaQ\"\x83\x85\x84aE/V[aQ+\x83a@GV[\x84\x01\x90P\x93\x92PPPV[_`\x80\x82\x01\x90PaQI_\x83\x01\x89aG^V[\x81\x81\x03` \x83\x01RaQ\\\x81\x87\x89aP\x98V[\x90P\x81\x81\x03`@\x83\x01RaQq\x81\x85\x87aQ\nV[\x90PaQ\x80``\x83\x01\x84aGmV[\x97\x96PPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x825`\x01`@\x03\x836\x03\x03\x81\x12aQ\xDFWaQ\xDEaQ\xB8V[[\x80\x83\x01\x91PP\x92\x91PPV[_\x815aQ\xF7\x81aO3V[\x80\x91PP\x91\x90PV[_\x81_\x1B\x90P\x91\x90PV[_`\xFFaR\x17\x84aR\0V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_aR7\x82aI\xF5V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[aRP\x82aR-V[aRcaR\\\x82aR>V[\x83TaR\x0BV[\x82UPPPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aR\x86WaR\x85aQ\xB8V[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aR\xA8WaR\xA7aQ\xBCV[[` \x83\x01\x92P`\x01\x82\x026\x03\x83\x13\x15aR\xC4WaR\xC3aQ\xC0V[[P\x92P\x92\x90PV[_\x82\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80aS\x1AW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aS-WaS,aR\xD6V[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02aS\x8F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82aSTV[aS\x99\x86\x83aSTV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_aS\xD4aS\xCFaS\xCA\x84a@\xC0V[aS\xB1V[a@\xC0V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[aS\xED\x83aS\xBAV[aT\x01aS\xF9\x82aS\xDBV[\x84\x84TaS`V[\x82UPPPPV[_\x90V[aT\x15aT\tV[aT \x81\x84\x84aS\xE4V[PPPV[[\x81\x81\x10\x15aTCWaT8_\x82aT\rV[`\x01\x81\x01\x90PaT&V[PPV[`\x1F\x82\x11\x15aT\x88WaTY\x81aS3V[aTb\x84aSEV[\x81\x01` \x85\x10\x15aTqW\x81\x90P[aT\x85aT}\x85aSEV[\x83\x01\x82aT%V[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_aT\xA8_\x19\x84`\x08\x02aT\x8DV[\x19\x80\x83\x16\x91PP\x92\x91PPV[_aT\xC0\x83\x83aT\x99V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[aT\xDA\x83\x83aR\xCCV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aT\xF3WaT\xF2aD\x87V[[aT\xFD\x82TaS\x03V[aU\x08\x82\x82\x85aTGV[_`\x1F\x83\x11`\x01\x81\x14aU5W_\x84\x15aU#W\x82\x87\x015\x90P[aU-\x85\x82aT\xB5V[\x86UPaU\x94V[`\x1F\x19\x84\x16aUC\x86aS3V[_[\x82\x81\x10\x15aUjW\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PaUEV[\x86\x83\x10\x15aU\x87W\x84\x89\x015aU\x83`\x1F\x89\x16\x82aT\x99V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[aU\xA8\x83\x83\x83aT\xD0V[PPPV[_\x81\x01_\x83\x01\x80aU\xBD\x81aQ\xEBV[\x90PaU\xC9\x81\x84aRGV[PPP`\x01\x81\x01` \x83\x01aU\xDE\x81\x85aRjV[aU\xE9\x81\x83\x86aU\x9DV[PPPPPPV[aU\xFB\x82\x82aU\xADV[PPV[_\x80\xFD[_\x80\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aV!WaV aD\x87V[[aV*\x82a@GV[\x90P` \x81\x01\x90P\x91\x90PV[_aVIaVD\x84aV\x07V[aD\xE5V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15aVeWaVdaD\x83V[[aVp\x84\x82\x85a@\x1FV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aV\x8CWaV\x8BaC\x12V[[\x81QaV\x9C\x84\x82` \x86\x01aV7V[\x91PP\x92\x91PPV[_`\xA0\x82\x84\x03\x12\x15aV\xBAWaV\xB9aU\xFFV[[aV\xC4`\xA0aD\xE5V[\x90P_aV\xD3\x84\x82\x85\x01aM\xA2V[_\x83\x01RP` aV\xE6\x84\x82\x85\x01aM\xA2V[` \x83\x01RP`@\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aW\nWaW\taV\x03V[[aW\x16\x84\x82\x85\x01aVxV[`@\x83\x01RP``\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aW:WaW9aV\x03V[[aWF\x84\x82\x85\x01aVxV[``\x83\x01RP`\x80\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aWjWaWiaV\x03V[[aWv\x84\x82\x85\x01aVxV[`\x80\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15aW\x97WaW\x96a@\xB8V[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aW\xB4WaW\xB3a@\xBCV[[aW\xC0\x84\x82\x85\x01aV\xA5V[\x91PP\x92\x91PPV[_``\x82\x01\x90PaW\xDC_\x83\x01\x87aG^V[\x81\x81\x03` \x83\x01RaW\xEE\x81\x86aIEV[\x90P\x81\x81\x03`@\x83\x01RaX\x03\x81\x84\x86aP\x98V[\x90P\x95\x94PPPPPV[_``\x82\x01\x90PaX!_\x83\x01\x87aG^V[\x81\x81\x03` \x83\x01RaX4\x81\x85\x87aQ\nV[\x90PaXC`@\x83\x01\x84aGmV[\x95\x94PPPPPV[_`@\x82\x01\x90PaX__\x83\x01\x85aG^V[aXl` \x83\x01\x84aG^V[\x93\x92PPPV[_`\x80\x82\x01\x90PaX\x86_\x83\x01\x89aG^V[\x81\x81\x03` \x83\x01RaX\x99\x81\x87\x89aQ\nV[\x90P\x81\x81\x03`@\x83\x01RaX\xAE\x81\x85\x87aQ\nV[\x90PaX\xBD``\x83\x01\x84aGmV[\x97\x96PPPPPPPV[_``\x82\x01\x90PaX\xDB_\x83\x01\x87aG^V[\x81\x81\x03` \x83\x01RaX\xED\x81\x86aIEV[\x90P\x81\x81\x03`@\x83\x01RaY\x02\x81\x84\x86aQ\nV[\x90P\x95\x94PPPPPV[\x7FEIP712: Uninitialized\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aYA`\x15\x83a@\x0FV[\x91PaYL\x82aY\rV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaYn\x81aY5V[\x90P\x91\x90PV[_`\x80\x82\x01\x90PaY\x88_\x83\x01\x87aG^V[aY\x95` \x83\x01\x86aG^V[aY\xA2`@\x83\x01\x85aG^V[aY\xAF``\x83\x01\x84aB\x89V[\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aY\xFAWaY\xF9a@\xB8V[[_aZ\x07\x84\x82\x85\x01aOBV[\x91PP\x92\x91PPV[_\x81\x90P\x92\x91PPV[_aZ%\x83\x85aZ\x10V[\x93PaZ2\x83\x85\x84aE/V[\x82\x84\x01\x90P\x93\x92PPPV[_aZJ\x82\x84\x86aZ\x1AV[\x91P\x81\x90P\x93\x92PPPV[aZ_\x81aJ\x07V[\x82RPPV[_``\x82\x01\x90PaZx_\x83\x01\x86aF\x0EV[aZ\x85` \x83\x01\x85aZVV[aZ\x92`@\x83\x01\x84aF\x0EV[\x94\x93PPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[aZ\xC6\x81aF\x05V[\x82RPPV[_aZ\xD7\x83\x83aZ\xBDV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aZ\xF9\x82aZ\x9AV[a[\x03\x81\x85aZ\xA4V[\x93Pa[\x0E\x83aZ\xAEV[\x80_[\x83\x81\x10\x15a[>W\x81Qa[%\x88\x82aZ\xCCV[\x97Pa[0\x83aZ\xE3V[\x92PP`\x01\x81\x01\x90Pa[\x11V[P\x85\x93PPPP\x92\x91PPV[_a[V\x82\x84aZ\xEFV[\x91P\x81\x90P\x92\x91PPV[_`\x80\x82\x01\x90Pa[t_\x83\x01\x87aF\x0EV[a[\x81` \x83\x01\x86aG^V[a[\x8E`@\x83\x01\x85aG^V[a[\x9B``\x83\x01\x84aF\x0EV[\x95\x94PPPPPV[_\x81Q\x90Pa[\xB2\x81a@\xC9V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a[\xCDWa[\xCCa@\xB8V[[_a[\xDA\x84\x82\x85\x01a[\xA4V[\x91PP\x92\x91PPV[a[\xEC\x81aF\x05V[\x81\x14a[\xF6W_\x80\xFD[PV[_\x81Q\x90Pa\\\x07\x81a[\xE3V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\\\"Wa\\!a@\xB8V[[_a\\/\x84\x82\x85\x01a[\xF9V[\x91PP\x92\x91PPV[_`@\x82\x01\x90Pa\\K_\x83\x01\x85aF\x0EV[a\\X` \x83\x01\x84aG^V[\x93\x92PPPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[`\x1F\x82\x11\x15a\\\xB2Wa\\\x83\x81a\\_V[a\\\x8C\x84aSEV[\x81\x01` \x85\x10\x15a\\\x9BW\x81\x90P[a\\\xAFa\\\xA7\x85aSEV[\x83\x01\x82aT%V[PP[PPPV[a\\\xC0\x82a@\x05V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\\\xD9Wa\\\xD8aD\x87V[[a\\\xE3\x82TaS\x03V[a\\\xEE\x82\x82\x85a\\qV[_` \x90P`\x1F\x83\x11`\x01\x81\x14a]\x1FW_\x84\x15a]\rW\x82\x87\x01Q\x90P[a]\x17\x85\x82aT\xB5V[\x86UPa]~V[`\x1F\x19\x84\x16a]-\x86a\\_V[_[\x82\x81\x10\x15a]TW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa]/V[\x86\x83\x10\x15a]qW\x84\x89\x01Qa]m`\x1F\x89\x16\x82aT\x99V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_`@\x82\x01\x90Pa]\x99_\x83\x01\x85aGmV[a]\xA6` \x83\x01\x84aGmV[\x93\x92PPPV[_a]\xB7\x82aJ'V[a]\xC1\x81\x85aZ\x10V[\x93Pa]\xD1\x81\x85` \x86\x01a@\x1FV[\x80\x84\x01\x91PP\x92\x91PPV[_a]\xE8\x82\x84a]\xADV[\x91P\x81\x90P\x92\x91PPV[_`\xA0\x82\x01\x90Pa^\x06_\x83\x01\x88aF\x0EV[a^\x13` \x83\x01\x87aF\x0EV[a^ `@\x83\x01\x86aF\x0EV[a^-``\x83\x01\x85aG^V[a^:`\x80\x83\x01\x84aGmV[\x96\x95PPPPPPV[_`\xFF\x82\x16\x90P\x91\x90PV[a^Y\x81a^DV[\x82RPPV[_`\x80\x82\x01\x90Pa^r_\x83\x01\x87aF\x0EV[a^\x7F` \x83\x01\x86a^PV[a^\x8C`@\x83\x01\x85aF\x0EV[a^\x99``\x83\x01\x84aF\x0EV[\x95\x94PPPPPV\xFECrsgenVerification(uint256 crsId,uint256 maxBitLength,bytes crsDigest)PrepKeygenVerification(uint256 prepKeygenId)KeygenVerification(uint256 prepKeygenId,uint256 keyId,KeyDigest[] keyDigests)KeyDigest(uint8 keyType,bytes digest)KeyDigest(uint8 keyType,bytes digest)",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405260043610610129575f3560e01c8063589adb0e116100aa578063ad3cb1cc1161006e578063ad3cb1cc146103a0578063baff211e146103ca578063c55b8724146103f4578063caa367db14610431578063d52f10eb14610459578063d65d83731461048357610129565b8063589adb0e146102cd57806362978787146102f55780637514a2ac1461031d57806384b0196e14610333578063936608ae1461036357610129565b80633c02f834116100f15780633c02f834146101fb57806345af261b146102235780634610ffe81461025f5780634f1ef2861461028757806352d1902d146102a357610129565b80630d8e6e2c1461012d578063123abb281461015757806316c713d91461016d57806319f4f632146101a957806339f73810146101e5575b5f80fd5b348015610138575f80fd5b506101416104ab565b60405161014e919061408f565b60405180910390f35b348015610162575f80fd5b5061016b610526565b005b348015610178575f80fd5b50610193600480360381019061018e91906140f3565b61064b565b6040516101a09190614205565b60405180910390f35b3480156101b4575f80fd5b506101cf60048036038101906101ca91906140f3565b61071c565b6040516101dc9190614298565b60405180910390f35b3480156101f0575f80fd5b506101f96107c9565b005b348015610206575f80fd5b50610221600480360381019061021c91906142d4565b610a38565b005b34801561022e575f80fd5b50610249600480360381019061024491906140f3565b610c76565b6040516102569190614298565b60405180910390f35b34801561026a575f80fd5b50610285600480360381019061028091906143c8565b610d0b565b005b6102a1600480360381019061029c91906145ab565b61132b565b005b3480156102ae575f80fd5b506102b761134a565b6040516102c4919061461d565b60405180910390f35b3480156102d8575f80fd5b506102f360048036038101906102ee9190614636565b61137b565b005b348015610300575f80fd5b5061031b60048036038101906103169190614693565b61175d565b005b348015610328575f80fd5b50610331611cd2565b005b34801561033e575f80fd5b50610347611df0565b60405161035a9796959493929190614833565b60405180910390f35b34801561036e575f80fd5b50610389600480360381019061038491906140f3565b611ef9565b604051610397929190614b45565b60405180910390f35b3480156103ab575f80fd5b506103b46122a0565b6040516103c1919061408f565b60405180910390f35b3480156103d5575f80fd5b506103de6122d9565b6040516103eb9190614b7a565b60405180910390f35b3480156103ff575f80fd5b5061041a600480360381019061041591906140f3565b6122f0565b604051610428929190614bdb565b60405180910390f35b34801561043c575f80fd5b5061045760048036038101906104529190614c10565b612602565b005b348015610464575f80fd5b5061046d61287b565b60405161047a9190614b7a565b60405180910390f35b34801561048e575f80fd5b506104a960048036038101906104a491906140f3565b612892565b005b60606040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506104ec5f612a8d565b6104f66004612a8d565b6104ff5f612a8d565b6040516020016105129493929190614d09565b604051602081830303815290604052905090565b60055f610531612b57565b9050805f0160089054906101000a900460ff168061057957508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b156105b0576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d28260405161063f9190614d89565b60405180910390a15050565b60605f610656612b6a565b90505f816003015f8581526020019081526020015f20549050816002015f8581526020019081526020015f205f8281526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561070e57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116106c5575b505050505092505050919050565b5f80610726612b6a565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff1661078957826040517f84de13310000000000000000000000000000000000000000000000000000000081526004016107809190614b7a565b60405180910390fd5b5f816006015f8581526020019081526020015f2054905081600d015f8281526020019081526020015f205f9054906101000a900460ff1692505050919050565b60016107d3612b91565b67ffffffffffffffff1614610814576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60055f61081f612b57565b9050805f0160089054906101000a900460ff168061086757508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b1561089e576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055506109576040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506040518060400160405280600181526020017f3100000000000000000000000000000000000000000000000000000000000000815250612bb5565b5f610960612b6a565b905060f86003600681111561097857610977614225565b5b901b816004018190555060f86004600681111561099857610997614225565b5b901b816005018190555060f8600560068111156109b8576109b7614225565b5b901b816009018190555060f86006808111156109d7576109d6614225565b5b901b81600e0181905550505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051610a2c9190614d89565b60405180910390a15050565b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610a95573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ab99190614db6565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610b2857336040517f0e56cf3d000000000000000000000000000000000000000000000000000000008152600401610b1f9190614de1565b60405180910390fd5b5f610b31612b6a565b90505f8160090154905060f860056006811115610b5157610b50614225565b5b901b8114158015610b7f5750816001015f8281526020019081526020015f205f9054906101000a900460ff16155b15610bc157806040517f061ac61d000000000000000000000000000000000000000000000000000000008152600401610bb89190614b7a565b60405180910390fd5b816009015f815480929190610bd590614e27565b91905055505f826009015490508483600a015f8381526020019081526020015f20819055508383600d015f8381526020019081526020015f205f6101000a81548160ff02191690836001811115610c2f57610c2e614225565b5b02179055507f3f038f6f88cb3031b7718588403a2ec220576a868be07dde4c02b846ca352ef5818686604051610c6793929190614e6e565b60405180910390a15050505050565b5f80610c80612b6a565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff16610ce357826040517fda32d00f000000000000000000000000000000000000000000000000000000008152600401610cda9190614b7a565b60405180910390fd5b80600d015f8481526020019081526020015f205f9054906101000a900460ff16915050919050565b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663e5275eaf336040518263ffffffff1660e01b8152600401610d589190614de1565b602060405180830381865afa158015610d73573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d979190614ed8565b610dd857336040517faee86323000000000000000000000000000000000000000000000000000000008152600401610dcf9190614de1565b60405180910390fd5b5f610de1612b6a565b90508060050154861180610df457505f86145b15610e3657856040517fadfab904000000000000000000000000000000000000000000000000000000008152600401610e2d9190614b7a565b60405180910390fd5b5f8585905003610e7d57856040517fe6f9083b000000000000000000000000000000000000000000000000000000008152600401610e749190614b7a565b60405180910390fd5b5f816006015f8881526020019081526020015f205490505f610ea182898989612bcb565b90505f610eaf828787612da2565b9050835f015f8a81526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615610f4f5788816040517f98fb957d000000000000000000000000000000000000000000000000000000008152600401610f46929190614f03565b60405180910390fd5b6001845f015f8b81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f846002015f8b81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505f818054905090507f2afe64fb3afde8e2678aea84cf36223f330e2fb1286d37aed573ab9cd1db47c78b8b8b8b8b3360405161107996959493929190615136565b60405180910390a1856001015f8c81526020019081526020015f205f9054906101000a900460ff161580156110b357506110b281612e08565b5b1561131e576001866001015f8d81526020019081526020015f205f6101000a81548160ff0219169083151502179055505f5b8a8a905081101561116957866007015f8d81526020019081526020015f208b8b838181106111165761111561518b565b5b905060200281019061112891906151c4565b908060018154018082558091505060019003905f5260205f2090600202015f90919091909150818161115a91906155f1565b505080806001019150506110e5565b5083866003015f8d81526020019081526020015f20819055508a86600801819055505f8167ffffffffffffffff8111156111a6576111a5614487565b5b6040519080825280602002602001820160405280156111d957816020015b60608152602001906001900390816111c45790505b5090505f5b828110156112de5773d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663e3b2a8748583815481106112295761122861518b565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b815260040161126d9190614de1565b5f60405180830381865afa158015611287573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f820116820180604052508101906112af9190615782565b606001518282815181106112c6576112c561518b565b5b602002602001018190525080806001019150506111de565b507feb85c26dbcad46b80a68a0f24cce7c2c90f0a1faded84184138839fc9e80a25b8c828d8d60405161131494939291906157c9565b60405180910390a1505b5050505050505050505050565b611333612e99565b61133c82612f7f565b6113468282613072565b5050565b5f611353613190565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663e5275eaf336040518263ffffffff1660e01b81526004016113c89190614de1565b602060405180830381865afa1580156113e3573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114079190614ed8565b61144857336040517faee8632300000000000000000000000000000000000000000000000000000000815260040161143f9190614de1565b60405180910390fd5b5f611451612b6a565b9050806004015484118061146457505f84145b156114a657836040517f0ab7f68700000000000000000000000000000000000000000000000000000000815260040161149d9190614b7a565b60405180910390fd5b5f6114b085613217565b90505f6114be828686612da2565b9050825f015f8781526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff161561155e5785816040517f33ca1fe3000000000000000000000000000000000000000000000000000000008152600401611555929190614f03565b60405180910390fd5b6001835f015f8881526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f836002015f8881526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507f4c715c5734ce5c18c9c12e8496e53d2a65f1ec381d476957f0f596b364a59b0c8787873360405161167c949392919061580e565b60405180910390a1836001015f8881526020019081526020015f205f9054906101000a900460ff161580156116ba57506116b98180549050612e08565b5b15611754576001846001015f8981526020019081526020015f205f6101000a81548160ff02191690831515021790555082846003015f8981526020019081526020015f20819055505f846006015f8981526020019081526020015f205490507f78b179176d1f19d7c28e80823deba2624da2ca2ec64b1701f3632a87c9aedc92888260405161174a92919061584c565b60405180910390a1505b50505050505050565b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663e5275eaf336040518263ffffffff1660e01b81526004016117aa9190614de1565b602060405180830381865afa1580156117c5573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117e99190614ed8565b61182a57336040517faee863230000000000000000000000000000000000000000000000000000000081526004016118219190614de1565b60405180910390fd5b5f611833612b6a565b9050806009015486118061184657505f86145b1561188857856040517f8d8c940a00000000000000000000000000000000000000000000000000000000815260040161187f9190614b7a565b60405180910390fd5b5f81600a015f8881526020019081526020015f205490505f6118ac8883898961326f565b90505f6118ba828787612da2565b9050835f015f8a81526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff161561195a5788816040517ffcf5a6e9000000000000000000000000000000000000000000000000000000008152600401611951929190614f03565b60405180910390fd5b6001845f015f8b81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f846002015f8b81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505f818054905090507f7bf1b42c10e9497c879620c5b7afced10bda17d8c90b22f0e3bc6b2fd6ced0bd8b8b8b8b8b33604051611a8496959493929190615873565b60405180910390a1856001015f8c81526020019081526020015f205f9054906101000a900460ff16158015611abe5750611abd81612e08565b5b15611cc5576001866001015f8d81526020019081526020015f205f6101000a81548160ff021916908315150217905550898987600b015f8e81526020019081526020015f209182611b109291906154d0565b5083866003015f8d81526020019081526020015f20819055508a86600c01819055505f8167ffffffffffffffff811115611b4d57611b4c614487565b5b604051908082528060200260200182016040528015611b8057816020015b6060815260200190600190039081611b6b5790505b5090505f5b82811015611c855773d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663e3b2a874858381548110611bd057611bcf61518b565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b8152600401611c149190614de1565b5f60405180830381865afa158015611c2e573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f82011682018060405250810190611c569190615782565b60600151828281518110611c6d57611c6c61518b565b5b60200260200101819052508080600101915050611b85565b507f2258b73faed33fb2e2ea454403bef974920caf682ab3a723484fcf67553b16a28c828d8d604051611cbb94939291906158c8565b60405180910390a1505b5050505050505050505050565b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611d2f573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d539190614db6565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614611dc257336040517f0e56cf3d000000000000000000000000000000000000000000000000000000008152600401611db99190614de1565b60405180910390fd5b7f11db42c1878f2e2819241f5250984563f06cf22818e7adb86a66921d15d59d3f60405160405180910390a1565b5f6060805f805f60605f611e026132f6565b90505f801b815f0154148015611e1d57505f801b8160010154145b611e5c576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611e5390615957565b60405180910390fd5b611e6461331d565b611e6c6133bb565b46305f801b5f67ffffffffffffffff811115611e8b57611e8a614487565b5b604051908082528060200260200182016040528015611eb95781602001602082028036833780820191505090505b507f0f0000000000000000000000000000000000000000000000000000000000000095949392919097509750975097509750975097505090919293949596565b6060805f611f05612b6a565b9050806001015f8581526020019081526020015f205f9054906101000a900460ff16611f6857836040517f84de1331000000000000000000000000000000000000000000000000000000008152600401611f5f9190614b7a565b60405180910390fd5b5f816003015f8681526020019081526020015f205490505f826002015f8781526020019081526020015f205f8381526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561201f57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311611fd6575b505050505090505f815190505f8167ffffffffffffffff81111561204657612045614487565b5b60405190808252806020026020018201604052801561207957816020015b60608152602001906001900390816120645790505b5090505f5b8281101561215e5773d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663e3b2a8748583815181106120c9576120c861518b565b5b60200260200101516040518263ffffffff1660e01b81526004016120ed9190614de1565b5f60405180830381865afa158015612107573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f8201168201806040525081019061212f9190615782565b606001518282815181106121465761214561518b565b5b6020026020010181905250808060010191505061207e565b5080856007015f8a81526020019081526020015f2080805480602002602001604051908101604052809291908181526020015f905b8282101561228c578382905f5260205f2090600202016040518060400160405290815f82015f9054906101000a900460ff1660018111156121d7576121d6614225565b5b60018111156121e9576121e8614225565b5b81526020016001820180546121fd90615303565b80601f016020809104026020016040519081016040528092919081815260200182805461222990615303565b80156122745780601f1061224b57610100808354040283529160200191612274565b820191905f5260205f20905b81548152906001019060200180831161225757829003601f168201915b50505050508152505081526020019060010190612193565b505050509050965096505050505050915091565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b5f806122e3612b6a565b905080600c015491505090565b6060805f6122fc612b6a565b9050806001015f8581526020019081526020015f205f9054906101000a900460ff1661235f57836040517fda32d00f0000000000000000000000000000000000000000000000000000000081526004016123569190614b7a565b60405180910390fd5b5f816003015f8681526020019081526020015f205490505f826002015f8781526020019081526020015f205f8381526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561241657602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116123cd575b505050505090505f815190505f8167ffffffffffffffff81111561243d5761243c614487565b5b60405190808252806020026020018201604052801561247057816020015b606081526020019060019003908161245b5790505b5090505f5b828110156125555773d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663e3b2a8748583815181106124c0576124bf61518b565b5b60200260200101516040518263ffffffff1660e01b81526004016124e49190614de1565b5f60405180830381865afa1580156124fe573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f820116820180604052508101906125269190615782565b6060015182828151811061253d5761253c61518b565b5b60200260200101819052508080600101915050612475565b508085600b015f8a81526020019081526020015f2080805461257690615303565b80601f01602080910402602001604051908101604052809291908181526020018280546125a290615303565b80156125ed5780601f106125c4576101008083540402835291602001916125ed565b820191905f5260205f20905b8154815290600101906020018083116125d057829003601f168201915b50505050509050965096505050505050915091565b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561265f573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906126839190614db6565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146126f257336040517f0e56cf3d0000000000000000000000000000000000000000000000000000000081526004016126e99190614de1565b60405180910390fd5b5f6126fb612b6a565b90505f8160050154905060f86004600681111561271b5761271a614225565b5b901b81141580156127495750816001015f8281526020019081526020015f205f9054906101000a900460ff16155b1561278b57806040517f3b853da80000000000000000000000000000000000000000000000000000000081526004016127829190614b7a565b60405180910390fd5b816004015f81548092919061279f90614e27565b91905055505f82600401549050826005015f8154809291906127c090614e27565b91905055505f8360050154905080846006015f8481526020019081526020015f208190555081846006015f8381526020019081526020015f20819055505f8585600d015f8581526020019081526020015f205f6101000a81548160ff0219169083600181111561283357612832614225565b5b02179055507f02024007d96574dbc9d11328bfee9893e7c7bb4ef4aa806df33bfdf454eb5e6083828860405161286b93929190614e6e565b60405180910390a1505050505050565b5f80612885612b6a565b9050806008015491505090565b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156128ef573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129139190614db6565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461298257336040517f0e56cf3d0000000000000000000000000000000000000000000000000000000081526004016129799190614de1565b60405180910390fd5b5f61298b612b6a565b9050806001015f8381526020019081526020015f205f9054906101000a900460ff166129ee57816040517f84de13310000000000000000000000000000000000000000000000000000000081526004016129e59190614b7a565b60405180910390fd5b5f816006015f8481526020019081526020015f205490505f82600d015f8381526020019081526020015f205f9054906101000a900460ff16905082600e015f815480929190612a3c90614e27565b91905055505f83600e015490507f1ccb5545c4c8db50a0f5b416499526929f68534ed47f6cfd4c9f069075e60b4583868385604051612a7e9493929190615975565b60405180910390a15050505050565b60605f6001612a9b84613459565b0190505f8167ffffffffffffffff811115612ab957612ab8614487565b5b6040519080825280601f01601f191660200182016040528015612aeb5781602001600182028036833780820191505090505b5090505f82602083010190505b600115612b4c578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a8581612b4157612b406159b8565b5b0494505f8503612af8575b819350505050919050565b5f80612b616135aa565b90508091505090565b5f7f0b8fdb1f0a6356dd20a6cbc6f9668fac23b85f96575d10e333e603faa794ac00905090565b5f612b9a612b57565b5f015f9054906101000a900467ffffffffffffffff16905090565b612bbd6135d3565b612bc78282613613565b5050565b5f808383905067ffffffffffffffff811115612bea57612be9614487565b5b604051908082528060200260200182016040528015612c185781602001602082028036833780820191505090505b5090505f5b84849050811015612d1c57604051806060016040528060258152602001615f876025913980519060200120858583818110612c5b57612c5a61518b565b5b9050602002810190612c6d91906151c4565b5f016020810190612c7e91906159e5565b868684818110612c9157612c9061518b565b5b9050602002810190612ca391906151c4565b8060200190612cb2919061526a565b604051612cc0929190615a3e565b6040518091039020604051602001612cda93929190615a65565b60405160208183030381529060405280519060200120828281518110612d0357612d0261518b565b5b6020026020010181815250508080600101915050612c1d565b50612d976040518060a0016040528060728152602001615f156072913980519060200120878784604051602001612d539190615b4b565b60405160208183030381529060405280519060200120604051602001612d7c9493929190615b61565b60405160208183030381529060405280519060200120613664565b915050949350505050565b5f80612df18585858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505061367d565b9050612dfd81336136a7565b809150509392505050565b5f8073d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663b4722bc46040518163ffffffff1660e01b8152600401602060405180830381865afa158015612e67573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612e8b9190615bb8565b905080831015915050919050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff161480612f4657507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16612f2d6137b8565b73ffffffffffffffffffffffffffffffffffffffff1614155b15612f7d576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612fdc573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906130009190614db6565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461306f57336040517f0e56cf3d0000000000000000000000000000000000000000000000000000000081526004016130669190614de1565b60405180910390fd5b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa9250505080156130da57506040513d601f19601f820116820180604052508101906130d79190615c0d565b60015b61311b57816040517f4c9c8ce30000000000000000000000000000000000000000000000000000000081526004016131129190614de1565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b811461318157806040517faa1d49a4000000000000000000000000000000000000000000000000000000008152600401613178919061461d565b60405180910390fd5b61318b838361380b565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff1614613215576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f6132686040518060600160405280602c8152602001615ee9602c9139805190602001208360405160200161324d929190615c38565b60405160208183030381529060405280519060200120613664565b9050919050565b5f6132ec604051806080016040528060468152602001615ea36046913980519060200120868686866040516020016132a8929190615a3e565b604051602081830303815290604052805190602001206040516020016132d19493929190615b61565b60405160208183030381529060405280519060200120613664565b9050949350505050565b5f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100905090565b60605f6133286132f6565b905080600201805461333990615303565b80601f016020809104026020016040519081016040528092919081815260200182805461336590615303565b80156133b05780601f10613387576101008083540402835291602001916133b0565b820191905f5260205f20905b81548152906001019060200180831161339357829003601f168201915b505050505091505090565b60605f6133c66132f6565b90508060030180546133d790615303565b80601f016020809104026020016040519081016040528092919081815260200182805461340390615303565b801561344e5780601f106134255761010080835404028352916020019161344e565b820191905f5260205f20905b81548152906001019060200180831161343157829003601f168201915b505050505091505090565b5f805f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f01000000000000000083106134b5577a184f03e93ff9f4daa797ed6e38ed64bf6a1f01000000000000000083816134ab576134aa6159b8565b5b0492506040810190505b6d04ee2d6d415b85acef810000000083106134f2576d04ee2d6d415b85acef810000000083816134e8576134e76159b8565b5b0492506020810190505b662386f26fc10000831061352157662386f26fc100008381613517576135166159b8565b5b0492506010810190505b6305f5e100831061354a576305f5e10083816135405761353f6159b8565b5b0492506008810190505b612710831061356f576127108381613565576135646159b8565b5b0492506004810190505b606483106135925760648381613588576135876159b8565b5b0492506002810190505b600a83106135a1576001810190505b80915050919050565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005f1b905090565b6135db61387d565b613611576040517fd7e6bcf800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b61361b6135d3565b5f6136246132f6565b9050828160020190816136379190615cb7565b50818160030190816136499190615cb7565b505f801b815f01819055505f801b8160010181905550505050565b5f61367661367061389b565b836138a9565b9050919050565b5f805f8061368b86866138e9565b92509250925061369b828261393e565b82935050505092915050565b6136b082613aa0565b8173ffffffffffffffffffffffffffffffffffffffff1673d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663e3b2a874836040518263ffffffff1660e01b81526004016137149190614de1565b5f60405180830381865afa15801561372e573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f820116820180604052508101906137569190615782565b6020015173ffffffffffffffffffffffffffffffffffffffff16146137b45781816040517f0d86f5210000000000000000000000000000000000000000000000000000000081526004016137ab929190615d86565b60405180910390fd5b5050565b5f6137e47f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b613b70565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b61381482613b79565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f815111156138705761386a8282613c42565b50613879565b613878613cc2565b5b5050565b5f613886612b57565b5f0160089054906101000a900460ff16905090565b5f6138a4613cfe565b905090565b5f6040517f190100000000000000000000000000000000000000000000000000000000000081528360028201528260228201526042812091505092915050565b5f805f6041845103613929575f805f602087015192506040870151915060608701515f1a905061391b88828585613d61565b955095509550505050613937565b5f600285515f1b9250925092505b9250925092565b5f600381111561395157613950614225565b5b82600381111561396457613963614225565b5b0315613a9c576001600381111561397e5761397d614225565b5b82600381111561399157613990614225565b5b036139c8576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600260038111156139dc576139db614225565b5b8260038111156139ef576139ee614225565b5b03613a3357805f1c6040517ffce698f7000000000000000000000000000000000000000000000000000000008152600401613a2a9190614b7a565b60405180910390fd5b600380811115613a4657613a45614225565b5b826003811115613a5957613a58614225565b5b03613a9b57806040517fd78bce0c000000000000000000000000000000000000000000000000000000008152600401613a92919061461d565b60405180910390fd5b5b5050565b73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663203d0114826040518263ffffffff1660e01b8152600401613aed9190614de1565b602060405180830381865afa158015613b08573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613b2c9190614ed8565b613b6d57806040517f2a7c6ef6000000000000000000000000000000000000000000000000000000008152600401613b649190614de1565b60405180910390fd5b50565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b03613bd457806040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401613bcb9190614de1565b60405180910390fd5b80613c007f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b613b70565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f808473ffffffffffffffffffffffffffffffffffffffff1684604051613c6b9190615ddd565b5f60405180830381855af49150503d805f8114613ca3576040519150601f19603f3d011682016040523d82523d5f602084013e613ca8565b606091505b5091509150613cb8858383613e48565b9250505092915050565b5f341115613cfc576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f613d28613ed5565b613d30613f4b565b4630604051602001613d46959493929190615df3565b60405160208183030381529060405280519060200120905090565b5f805f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c1115613d9d575f600385925092509250613e3e565b5f6001888888886040515f8152602001604052604051613dc09493929190615e5f565b6020604051602081039080840390855afa158015613de0573d5f803e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603613e31575f60015f801b93509350935050613e3e565b805f805f1b935093509350505b9450945094915050565b606082613e5d57613e5882613fc2565b613ecd565b5f8251148015613e8357505f8473ffffffffffffffffffffffffffffffffffffffff163b145b15613ec557836040517f9996b315000000000000000000000000000000000000000000000000000000008152600401613ebc9190614de1565b60405180910390fd5b819050613ece565b5b9392505050565b5f80613edf6132f6565b90505f613eea61331d565b90505f81511115613f0657808051906020012092505050613f48565b5f825f015490505f801b8114613f2157809350505050613f48565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f80613f556132f6565b90505f613f606133bb565b90505f81511115613f7c57808051906020012092505050613fbf565b5f826001015490505f801b8114613f9857809350505050613fbf565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f81511115613fd357805160208201fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f81519050919050565b5f82825260208201905092915050565b5f5b8381101561403c578082015181840152602081019050614021565b5f8484015250505050565b5f601f19601f8301169050919050565b5f61406182614005565b61406b818561400f565b935061407b81856020860161401f565b61408481614047565b840191505092915050565b5f6020820190508181035f8301526140a78184614057565b905092915050565b5f604051905090565b5f80fd5b5f80fd5b5f819050919050565b6140d2816140c0565b81146140dc575f80fd5b50565b5f813590506140ed816140c9565b92915050565b5f60208284031215614108576141076140b8565b5b5f614115848285016140df565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61417082614147565b9050919050565b61418081614166565b82525050565b5f6141918383614177565b60208301905092915050565b5f602082019050919050565b5f6141b38261411e565b6141bd8185614128565b93506141c883614138565b805f5b838110156141f85781516141df8882614186565b97506141ea8361419d565b9250506001810190506141cb565b5085935050505092915050565b5f6020820190508181035f83015261421d81846141a9565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b6002811061426357614262614225565b5b50565b5f81905061427382614252565b919050565b5f61428282614266565b9050919050565b61429281614278565b82525050565b5f6020820190506142ab5f830184614289565b92915050565b600281106142bd575f80fd5b50565b5f813590506142ce816142b1565b92915050565b5f80604083850312156142ea576142e96140b8565b5b5f6142f7858286016140df565b9250506020614308858286016142c0565b9150509250929050565b5f80fd5b5f80fd5b5f80fd5b5f8083601f84011261433357614332614312565b5b8235905067ffffffffffffffff8111156143505761434f614316565b5b60208301915083602082028301111561436c5761436b61431a565b5b9250929050565b5f8083601f84011261438857614387614312565b5b8235905067ffffffffffffffff8111156143a5576143a4614316565b5b6020830191508360018202830111156143c1576143c061431a565b5b9250929050565b5f805f805f606086880312156143e1576143e06140b8565b5b5f6143ee888289016140df565b955050602086013567ffffffffffffffff81111561440f5761440e6140bc565b5b61441b8882890161431e565b9450945050604086013567ffffffffffffffff81111561443e5761443d6140bc565b5b61444a88828901614373565b92509250509295509295909350565b61446281614166565b811461446c575f80fd5b50565b5f8135905061447d81614459565b92915050565b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6144bd82614047565b810181811067ffffffffffffffff821117156144dc576144db614487565b5b80604052505050565b5f6144ee6140af565b90506144fa82826144b4565b919050565b5f67ffffffffffffffff82111561451957614518614487565b5b61452282614047565b9050602081019050919050565b828183375f83830152505050565b5f61454f61454a846144ff565b6144e5565b90508281526020810184848401111561456b5761456a614483565b5b61457684828561452f565b509392505050565b5f82601f83011261459257614591614312565b5b81356145a284826020860161453d565b91505092915050565b5f80604083850312156145c1576145c06140b8565b5b5f6145ce8582860161446f565b925050602083013567ffffffffffffffff8111156145ef576145ee6140bc565b5b6145fb8582860161457e565b9150509250929050565b5f819050919050565b61461781614605565b82525050565b5f6020820190506146305f83018461460e565b92915050565b5f805f6040848603121561464d5761464c6140b8565b5b5f61465a868287016140df565b935050602084013567ffffffffffffffff81111561467b5761467a6140bc565b5b61468786828701614373565b92509250509250925092565b5f805f805f606086880312156146ac576146ab6140b8565b5b5f6146b9888289016140df565b955050602086013567ffffffffffffffff8111156146da576146d96140bc565b5b6146e688828901614373565b9450945050604086013567ffffffffffffffff811115614709576147086140bc565b5b61471588828901614373565b92509250509295509295909350565b5f7fff0000000000000000000000000000000000000000000000000000000000000082169050919050565b61475881614724565b82525050565b614767816140c0565b82525050565b61477681614166565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b6147ae816140c0565b82525050565b5f6147bf83836147a5565b60208301905092915050565b5f602082019050919050565b5f6147e18261477c565b6147eb8185614786565b93506147f683614796565b805f5b8381101561482657815161480d88826147b4565b9750614818836147cb565b9250506001810190506147f9565b5085935050505092915050565b5f60e0820190506148465f83018a61474f565b81810360208301526148588189614057565b9050818103604083015261486c8188614057565b905061487b606083018761475e565b614888608083018661476d565b61489560a083018561460e565b81810360c08301526148a781846147d7565b905098975050505050505050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f82825260208201905092915050565b5f6148f882614005565b61490281856148de565b935061491281856020860161401f565b61491b81614047565b840191505092915050565b5f61493183836148ee565b905092915050565b5f602082019050919050565b5f61494f826148b5565b61495981856148bf565b93508360208202850161496b856148cf565b805f5b858110156149a657848403895281516149878582614926565b945061499283614939565b925060208a0199505060018101905061496e565b50829750879550505050505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b600281106149f2576149f1614225565b5b50565b5f819050614a02826149e1565b919050565b5f614a11826149f5565b9050919050565b614a2181614a07565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f614a4b82614a27565b614a558185614a31565b9350614a6581856020860161401f565b614a6e81614047565b840191505092915050565b5f604083015f830151614a8e5f860182614a18565b5060208301518482036020860152614aa68282614a41565b9150508091505092915050565b5f614abe8383614a79565b905092915050565b5f602082019050919050565b5f614adc826149b8565b614ae681856149c2565b935083602082028501614af8856149d2565b805f5b85811015614b335784840389528151614b148582614ab3565b9450614b1f83614ac6565b925060208a01995050600181019050614afb565b50829750879550505050505092915050565b5f6040820190508181035f830152614b5d8185614945565b90508181036020830152614b718184614ad2565b90509392505050565b5f602082019050614b8d5f83018461475e565b92915050565b5f82825260208201905092915050565b5f614bad82614a27565b614bb78185614b93565b9350614bc781856020860161401f565b614bd081614047565b840191505092915050565b5f6040820190508181035f830152614bf38185614945565b90508181036020830152614c078184614ba3565b90509392505050565b5f60208284031215614c2557614c246140b8565b5b5f614c32848285016142c0565b91505092915050565b5f81905092915050565b5f614c4f82614005565b614c598185614c3b565b9350614c6981856020860161401f565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f614ca9600283614c3b565b9150614cb482614c75565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f614cf3600183614c3b565b9150614cfe82614cbf565b600182019050919050565b5f614d148287614c45565b9150614d1f82614c9d565b9150614d2b8286614c45565b9150614d3682614ce7565b9150614d428285614c45565b9150614d4d82614ce7565b9150614d598284614c45565b915081905095945050505050565b5f67ffffffffffffffff82169050919050565b614d8381614d67565b82525050565b5f602082019050614d9c5f830184614d7a565b92915050565b5f81519050614db081614459565b92915050565b5f60208284031215614dcb57614dca6140b8565b5b5f614dd884828501614da2565b91505092915050565b5f602082019050614df45f83018461476d565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f614e31826140c0565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203614e6357614e62614dfa565b5b600182019050919050565b5f606082019050614e815f83018661475e565b614e8e602083018561475e565b614e9b6040830184614289565b949350505050565b5f8115159050919050565b614eb781614ea3565b8114614ec1575f80fd5b50565b5f81519050614ed281614eae565b92915050565b5f60208284031215614eed57614eec6140b8565b5b5f614efa84828501614ec4565b91505092915050565b5f604082019050614f165f83018561475e565b614f23602083018461476d565b9392505050565b5f819050919050565b60028110614f3f575f80fd5b50565b5f81359050614f5081614f33565b92915050565b5f614f646020840184614f42565b905092915050565b5f80fd5b5f80fd5b5f80fd5b5f8083356001602003843603038112614f9457614f93614f74565b5b83810192508235915060208301925067ffffffffffffffff821115614fbc57614fbb614f6c565b5b600182023603831315614fd257614fd1614f70565b5b509250929050565b5f614fe58385614a31565b9350614ff283858461452f565b614ffb83614047565b840190509392505050565b5f604083016150175f840184614f56565b6150235f860182614a18565b506150316020840184614f78565b8583036020870152615044838284614fda565b925050508091505092915050565b5f61505d8383615006565b905092915050565b5f823560016040038336030381126150805761507f614f74565b5b82810191505092915050565b5f602082019050919050565b5f6150a383856149c2565b9350836020840285016150b584614f2a565b805f5b878110156150f85784840389526150cf8284615065565b6150d98582615052565b94506150e48361508c565b925060208a019950506001810190506150b8565b50829750879450505050509392505050565b5f6151158385614b93565b935061512283858461452f565b61512b83614047565b840190509392505050565b5f6080820190506151495f83018961475e565b818103602083015261515c818789615098565b9050818103604083015261517181858761510a565b9050615180606083018461476d565b979650505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b5f823560016040038336030381126151df576151de6151b8565b5b80830191505092915050565b5f81356151f781614f33565b80915050919050565b5f815f1b9050919050565b5f60ff61521784615200565b9350801983169250808416831791505092915050565b5f615237826149f5565b9050919050565b5f819050919050565b6152508261522d565b61526361525c8261523e565b835461520b565b8255505050565b5f8083356001602003843603038112615286576152856151b8565b5b80840192508235915067ffffffffffffffff8211156152a8576152a76151bc565b5b6020830192506001820236038313156152c4576152c36151c0565b5b509250929050565b5f82905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f600282049050600182168061531a57607f821691505b60208210810361532d5761532c6152d6565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f6008830261538f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82615354565b6153998683615354565b95508019841693508086168417925050509392505050565b5f819050919050565b5f6153d46153cf6153ca846140c0565b6153b1565b6140c0565b9050919050565b5f819050919050565b6153ed836153ba565b6154016153f9826153db565b848454615360565b825550505050565b5f90565b615415615409565b6154208184846153e4565b505050565b5b81811015615443576154385f8261540d565b600181019050615426565b5050565b601f8211156154885761545981615333565b61546284615345565b81016020851015615471578190505b61548561547d85615345565b830182615425565b50505b505050565b5f82821c905092915050565b5f6154a85f198460080261548d565b1980831691505092915050565b5f6154c08383615499565b9150826002028217905092915050565b6154da83836152cc565b67ffffffffffffffff8111156154f3576154f2614487565b5b6154fd8254615303565b615508828285615447565b5f601f831160018114615535575f8415615523578287013590505b61552d85826154b5565b865550615594565b601f19841661554386615333565b5f5b8281101561556a57848901358255600182019150602085019450602081019050615545565b868310156155875784890135615583601f891682615499565b8355505b6001600288020188555050505b50505050505050565b6155a88383836154d0565b505050565b5f81015f8301806155bd816151eb565b90506155c98184615247565b50505060018101602083016155de818561526a565b6155e981838661559d565b505050505050565b6155fb82826155ad565b5050565b5f80fd5b5f80fd5b5f67ffffffffffffffff82111561562157615620614487565b5b61562a82614047565b9050602081019050919050565b5f61564961564484615607565b6144e5565b90508281526020810184848401111561566557615664614483565b5b61567084828561401f565b509392505050565b5f82601f83011261568c5761568b614312565b5b815161569c848260208601615637565b91505092915050565b5f60a082840312156156ba576156b96155ff565b5b6156c460a06144e5565b90505f6156d384828501614da2565b5f8301525060206156e684828501614da2565b602083015250604082015167ffffffffffffffff81111561570a57615709615603565b5b61571684828501615678565b604083015250606082015167ffffffffffffffff81111561573a57615739615603565b5b61574684828501615678565b606083015250608082015167ffffffffffffffff81111561576a57615769615603565b5b61577684828501615678565b60808301525092915050565b5f60208284031215615797576157966140b8565b5b5f82015167ffffffffffffffff8111156157b4576157b36140bc565b5b6157c0848285016156a5565b91505092915050565b5f6060820190506157dc5f83018761475e565b81810360208301526157ee8186614945565b90508181036040830152615803818486615098565b905095945050505050565b5f6060820190506158215f83018761475e565b818103602083015261583481858761510a565b9050615843604083018461476d565b95945050505050565b5f60408201905061585f5f83018561475e565b61586c602083018461475e565b9392505050565b5f6080820190506158865f83018961475e565b818103602083015261589981878961510a565b905081810360408301526158ae81858761510a565b90506158bd606083018461476d565b979650505050505050565b5f6060820190506158db5f83018761475e565b81810360208301526158ed8186614945565b9050818103604083015261590281848661510a565b905095945050505050565b7f4549503731323a20556e696e697469616c697a656400000000000000000000005f82015250565b5f61594160158361400f565b915061594c8261590d565b602082019050919050565b5f6020820190508181035f83015261596e81615935565b9050919050565b5f6080820190506159885f83018761475e565b615995602083018661475e565b6159a2604083018561475e565b6159af6060830184614289565b95945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f602082840312156159fa576159f96140b8565b5b5f615a0784828501614f42565b91505092915050565b5f81905092915050565b5f615a258385615a10565b9350615a3283858461452f565b82840190509392505050565b5f615a4a828486615a1a565b91508190509392505050565b615a5f81614a07565b82525050565b5f606082019050615a785f83018661460e565b615a856020830185615a56565b615a92604083018461460e565b949350505050565b5f81519050919050565b5f81905092915050565b5f819050602082019050919050565b615ac681614605565b82525050565b5f615ad78383615abd565b60208301905092915050565b5f602082019050919050565b5f615af982615a9a565b615b038185615aa4565b9350615b0e83615aae565b805f5b83811015615b3e578151615b258882615acc565b9750615b3083615ae3565b925050600181019050615b11565b5085935050505092915050565b5f615b568284615aef565b915081905092915050565b5f608082019050615b745f83018761460e565b615b81602083018661475e565b615b8e604083018561475e565b615b9b606083018461460e565b95945050505050565b5f81519050615bb2816140c9565b92915050565b5f60208284031215615bcd57615bcc6140b8565b5b5f615bda84828501615ba4565b91505092915050565b615bec81614605565b8114615bf6575f80fd5b50565b5f81519050615c0781615be3565b92915050565b5f60208284031215615c2257615c216140b8565b5b5f615c2f84828501615bf9565b91505092915050565b5f604082019050615c4b5f83018561460e565b615c58602083018461475e565b9392505050565b5f819050815f5260205f209050919050565b601f821115615cb257615c8381615c5f565b615c8c84615345565b81016020851015615c9b578190505b615caf615ca785615345565b830182615425565b50505b505050565b615cc082614005565b67ffffffffffffffff811115615cd957615cd8614487565b5b615ce38254615303565b615cee828285615c71565b5f60209050601f831160018114615d1f575f8415615d0d578287015190505b615d1785826154b5565b865550615d7e565b601f198416615d2d86615c5f565b5f5b82811015615d5457848901518255600182019150602085019450602081019050615d2f565b86831015615d715784890151615d6d601f891682615499565b8355505b6001600288020188555050505b505050505050565b5f604082019050615d995f83018561476d565b615da6602083018461476d565b9392505050565b5f615db782614a27565b615dc18185615a10565b9350615dd181856020860161401f565b80840191505092915050565b5f615de88284615dad565b915081905092915050565b5f60a082019050615e065f83018861460e565b615e13602083018761460e565b615e20604083018661460e565b615e2d606083018561475e565b615e3a608083018461476d565b9695505050505050565b5f60ff82169050919050565b615e5981615e44565b82525050565b5f608082019050615e725f83018761460e565b615e7f6020830186615e50565b615e8c604083018561460e565b615e99606083018461460e565b9594505050505056fe43727367656e566572696669636174696f6e2875696e743235362063727349642c75696e74323536206d61784269744c656e6774682c62797465732063727344696765737429507265704b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e4964294b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e49642c75696e74323536206b657949642c4b65794469676573745b5d206b657944696765737473294b65794469676573742875696e7438206b6579547970652c627974657320646967657374294b65794469676573742875696e7438206b6579547970652c62797465732064696765737429
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01)W_5`\xE0\x1C\x80cX\x9A\xDB\x0E\x11a\0\xAAW\x80c\xAD<\xB1\xCC\x11a\0nW\x80c\xAD<\xB1\xCC\x14a\x03\xA0W\x80c\xBA\xFF!\x1E\x14a\x03\xCAW\x80c\xC5[\x87$\x14a\x03\xF4W\x80c\xCA\xA3g\xDB\x14a\x041W\x80c\xD5/\x10\xEB\x14a\x04YW\x80c\xD6]\x83s\x14a\x04\x83Wa\x01)V[\x80cX\x9A\xDB\x0E\x14a\x02\xCDW\x80cb\x97\x87\x87\x14a\x02\xF5W\x80cu\x14\xA2\xAC\x14a\x03\x1DW\x80c\x84\xB0\x19n\x14a\x033W\x80c\x93f\x08\xAE\x14a\x03cWa\x01)V[\x80c<\x02\xF84\x11a\0\xF1W\x80c<\x02\xF84\x14a\x01\xFBW\x80cE\xAF&\x1B\x14a\x02#W\x80cF\x10\xFF\xE8\x14a\x02_W\x80cO\x1E\xF2\x86\x14a\x02\x87W\x80cR\xD1\x90-\x14a\x02\xA3Wa\x01)V[\x80c\r\x8En,\x14a\x01-W\x80c\x12:\xBB(\x14a\x01WW\x80c\x16\xC7\x13\xD9\x14a\x01mW\x80c\x19\xF4\xF62\x14a\x01\xA9W\x80c9\xF78\x10\x14a\x01\xE5W[_\x80\xFD[4\x80\x15a\x018W_\x80\xFD[Pa\x01Aa\x04\xABV[`@Qa\x01N\x91\x90a@\x8FV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01bW_\x80\xFD[Pa\x01ka\x05&V[\0[4\x80\x15a\x01xW_\x80\xFD[Pa\x01\x93`\x04\x806\x03\x81\x01\x90a\x01\x8E\x91\x90a@\xF3V[a\x06KV[`@Qa\x01\xA0\x91\x90aB\x05V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xB4W_\x80\xFD[Pa\x01\xCF`\x04\x806\x03\x81\x01\x90a\x01\xCA\x91\x90a@\xF3V[a\x07\x1CV[`@Qa\x01\xDC\x91\x90aB\x98V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xF0W_\x80\xFD[Pa\x01\xF9a\x07\xC9V[\0[4\x80\x15a\x02\x06W_\x80\xFD[Pa\x02!`\x04\x806\x03\x81\x01\x90a\x02\x1C\x91\x90aB\xD4V[a\n8V[\0[4\x80\x15a\x02.W_\x80\xFD[Pa\x02I`\x04\x806\x03\x81\x01\x90a\x02D\x91\x90a@\xF3V[a\x0CvV[`@Qa\x02V\x91\x90aB\x98V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02jW_\x80\xFD[Pa\x02\x85`\x04\x806\x03\x81\x01\x90a\x02\x80\x91\x90aC\xC8V[a\r\x0BV[\0[a\x02\xA1`\x04\x806\x03\x81\x01\x90a\x02\x9C\x91\x90aE\xABV[a\x13+V[\0[4\x80\x15a\x02\xAEW_\x80\xFD[Pa\x02\xB7a\x13JV[`@Qa\x02\xC4\x91\x90aF\x1DV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xD8W_\x80\xFD[Pa\x02\xF3`\x04\x806\x03\x81\x01\x90a\x02\xEE\x91\x90aF6V[a\x13{V[\0[4\x80\x15a\x03\0W_\x80\xFD[Pa\x03\x1B`\x04\x806\x03\x81\x01\x90a\x03\x16\x91\x90aF\x93V[a\x17]V[\0[4\x80\x15a\x03(W_\x80\xFD[Pa\x031a\x1C\xD2V[\0[4\x80\x15a\x03>W_\x80\xFD[Pa\x03Ga\x1D\xF0V[`@Qa\x03Z\x97\x96\x95\x94\x93\x92\x91\x90aH3V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03nW_\x80\xFD[Pa\x03\x89`\x04\x806\x03\x81\x01\x90a\x03\x84\x91\x90a@\xF3V[a\x1E\xF9V[`@Qa\x03\x97\x92\x91\x90aKEV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xABW_\x80\xFD[Pa\x03\xB4a\"\xA0V[`@Qa\x03\xC1\x91\x90a@\x8FV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xD5W_\x80\xFD[Pa\x03\xDEa\"\xD9V[`@Qa\x03\xEB\x91\x90aKzV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xFFW_\x80\xFD[Pa\x04\x1A`\x04\x806\x03\x81\x01\x90a\x04\x15\x91\x90a@\xF3V[a\"\xF0V[`@Qa\x04(\x92\x91\x90aK\xDBV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04<W_\x80\xFD[Pa\x04W`\x04\x806\x03\x81\x01\x90a\x04R\x91\x90aL\x10V[a&\x02V[\0[4\x80\x15a\x04dW_\x80\xFD[Pa\x04ma({V[`@Qa\x04z\x91\x90aKzV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\x8EW_\x80\xFD[Pa\x04\xA9`\x04\x806\x03\x81\x01\x90a\x04\xA4\x91\x90a@\xF3V[a(\x92V[\0[```@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x04\xEC_a*\x8DV[a\x04\xF6`\x04a*\x8DV[a\x04\xFF_a*\x8DV[`@Q` \x01a\x05\x12\x94\x93\x92\x91\x90aM\tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`\x05_a\x051a+WV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x05yWP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x05\xB0W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x06?\x91\x90aM\x89V[`@Q\x80\x91\x03\x90\xA1PPV[``_a\x06Va+jV[\x90P_\x81`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\x02\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x82\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07\x0EW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x06\xC5W[PPPPP\x92PPP\x91\x90PV[_\x80a\x07&a+jV[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x07\x89W\x82`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\x80\x91\x90aKzV[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\r\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x92PPP\x91\x90PV[`\x01a\x07\xD3a+\x91V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x08\x14W`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05_a\x08\x1Fa+WV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x08gWP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x08\x9EW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\tW`@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa+\xB5V[_a\t`a+jV[\x90P`\xF8`\x03`\x06\x81\x11\x15a\txWa\twaB%V[[\x90\x1B\x81`\x04\x01\x81\x90UP`\xF8`\x04`\x06\x81\x11\x15a\t\x98Wa\t\x97aB%V[[\x90\x1B\x81`\x05\x01\x81\x90UP`\xF8`\x05`\x06\x81\x11\x15a\t\xB8Wa\t\xB7aB%V[[\x90\x1B\x81`\t\x01\x81\x90UP`\xF8`\x06\x80\x81\x11\x15a\t\xD7Wa\t\xD6aB%V[[\x90\x1B\x81`\x0E\x01\x81\x90UPP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\n,\x91\x90aM\x89V[`@Q\x80\x91\x03\x90\xA1PPV[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x95W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xB9\x91\x90aM\xB6V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0B(W3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B\x1F\x91\x90aM\xE1V[`@Q\x80\x91\x03\x90\xFD[_a\x0B1a+jV[\x90P_\x81`\t\x01T\x90P`\xF8`\x05`\x06\x81\x11\x15a\x0BQWa\x0BPaB%V[[\x90\x1B\x81\x14\x15\x80\x15a\x0B\x7FWP\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15a\x0B\xC1W\x80`@Q\x7F\x06\x1A\xC6\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B\xB8\x91\x90aKzV[`@Q\x80\x91\x03\x90\xFD[\x81`\t\x01_\x81T\x80\x92\x91\x90a\x0B\xD5\x90aN'V[\x91\x90PUP_\x82`\t\x01T\x90P\x84\x83`\n\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x83\x83`\r\x01_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a\x0C/Wa\x0C.aB%V[[\x02\x17\x90UP\x7F?\x03\x8Fo\x88\xCB01\xB7q\x85\x88@:.\xC2 Wj\x86\x8B\xE0}\xDEL\x02\xB8F\xCA5.\xF5\x81\x86\x86`@Qa\x0Cg\x93\x92\x91\x90aNnV[`@Q\x80\x91\x03\x90\xA1PPPPPV[_\x80a\x0C\x80a+jV[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x0C\xE3W\x82`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\xDA\x91\x90aKzV[`@Q\x80\x91\x03\x90\xFD[\x80`\r\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE5'^\xAF3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\rX\x91\x90aM\xE1V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rsW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x97\x91\x90aN\xD8V[a\r\xD8W3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\xCF\x91\x90aM\xE1V[`@Q\x80\x91\x03\x90\xFD[_a\r\xE1a+jV[\x90P\x80`\x05\x01T\x86\x11\x80a\r\xF4WP_\x86\x14[\x15a\x0E6W\x85`@Q\x7F\xAD\xFA\xB9\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E-\x91\x90aKzV[`@Q\x80\x91\x03\x90\xFD[_\x85\x85\x90P\x03a\x0E}W\x85`@Q\x7F\xE6\xF9\x08;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0Et\x91\x90aKzV[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x88\x81R` \x01\x90\x81R` \x01_ T\x90P_a\x0E\xA1\x82\x89\x89\x89a+\xCBV[\x90P_a\x0E\xAF\x82\x87\x87a-\xA2V[\x90P\x83_\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x0FOW\x88\x81`@Q\x7F\x98\xFB\x95}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0FF\x92\x91\x90aO\x03V[`@Q\x80\x91\x03\x90\xFD[`\x01\x84_\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x84`\x02\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_\x81\x80T\x90P\x90P\x7F*\xFEd\xFB:\xFD\xE8\xE2g\x8A\xEA\x84\xCF6\"?3\x0E/\xB1(m7\xAE\xD5s\xAB\x9C\xD1\xDBG\xC7\x8B\x8B\x8B\x8B\x8B3`@Qa\x10y\x96\x95\x94\x93\x92\x91\x90aQ6V[`@Q\x80\x91\x03\x90\xA1\x85`\x01\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x10\xB3WPa\x10\xB2\x81a.\x08V[[\x15a\x13\x1EW`\x01\x86`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_[\x8A\x8A\x90P\x81\x10\x15a\x11iW\x86`\x07\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x8B\x8B\x83\x81\x81\x10a\x11\x16Wa\x11\x15aQ\x8BV[[\x90P` \x02\x81\x01\x90a\x11(\x91\x90aQ\xC4V[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P\x81\x81a\x11Z\x91\x90aU\xF1V[PP\x80\x80`\x01\x01\x91PPa\x10\xE5V[P\x83\x86`\x03\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x8A\x86`\x08\x01\x81\x90UP_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xA6Wa\x11\xA5aD\x87V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11\xD9W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x11\xC4W\x90P[P\x90P_[\x82\x81\x10\x15a\x12\xDEWs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3\xB2\xA8t\x85\x83\x81T\x81\x10a\x12)Wa\x12(aQ\x8BV[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12m\x91\x90aM\xE1V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\x87W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xAF\x91\x90aW\x82V[``\x01Q\x82\x82\x81Q\x81\x10a\x12\xC6Wa\x12\xC5aQ\x8BV[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa\x11\xDEV[P\x7F\xEB\x85\xC2m\xBC\xADF\xB8\nh\xA0\xF2L\xCE|,\x90\xF0\xA1\xFA\xDE\xD8A\x84\x13\x889\xFC\x9E\x80\xA2[\x8C\x82\x8D\x8D`@Qa\x13\x14\x94\x93\x92\x91\x90aW\xC9V[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPPPPPV[a\x133a.\x99V[a\x13<\x82a/\x7FV[a\x13F\x82\x82a0rV[PPV[_a\x13Sa1\x90V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE5'^\xAF3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\xC8\x91\x90aM\xE1V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xE3W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x07\x91\x90aN\xD8V[a\x14HW3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14?\x91\x90aM\xE1V[`@Q\x80\x91\x03\x90\xFD[_a\x14Qa+jV[\x90P\x80`\x04\x01T\x84\x11\x80a\x14dWP_\x84\x14[\x15a\x14\xA6W\x83`@Q\x7F\n\xB7\xF6\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14\x9D\x91\x90aKzV[`@Q\x80\x91\x03\x90\xFD[_a\x14\xB0\x85a2\x17V[\x90P_a\x14\xBE\x82\x86\x86a-\xA2V[\x90P\x82_\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x15^W\x85\x81`@Q\x7F3\xCA\x1F\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15U\x92\x91\x90aO\x03V[`@Q\x80\x91\x03\x90\xFD[`\x01\x83_\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x83`\x02\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7FLq\\W4\xCE\\\x18\xC9\xC1.\x84\x96\xE5=*e\xF1\xEC8\x1DGiW\xF0\xF5\x96\xB3d\xA5\x9B\x0C\x87\x87\x873`@Qa\x16|\x94\x93\x92\x91\x90aX\x0EV[`@Q\x80\x91\x03\x90\xA1\x83`\x01\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x16\xBAWPa\x16\xB9\x81\x80T\x90Pa.\x08V[[\x15a\x17TW`\x01\x84`\x01\x01_\x89\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x82\x84`\x03\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x81\x90UP_\x84`\x06\x01_\x89\x81R` \x01\x90\x81R` \x01_ T\x90P\x7Fx\xB1y\x17m\x1F\x19\xD7\xC2\x8E\x80\x82=\xEB\xA2bM\xA2\xCA.\xC6K\x17\x01\xF3c*\x87\xC9\xAE\xDC\x92\x88\x82`@Qa\x17J\x92\x91\x90aXLV[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPV[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE5'^\xAF3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\xAA\x91\x90aM\xE1V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xC5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xE9\x91\x90aN\xD8V[a\x18*W3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18!\x91\x90aM\xE1V[`@Q\x80\x91\x03\x90\xFD[_a\x183a+jV[\x90P\x80`\t\x01T\x86\x11\x80a\x18FWP_\x86\x14[\x15a\x18\x88W\x85`@Q\x7F\x8D\x8C\x94\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18\x7F\x91\x90aKzV[`@Q\x80\x91\x03\x90\xFD[_\x81`\n\x01_\x88\x81R` \x01\x90\x81R` \x01_ T\x90P_a\x18\xAC\x88\x83\x89\x89a2oV[\x90P_a\x18\xBA\x82\x87\x87a-\xA2V[\x90P\x83_\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x19ZW\x88\x81`@Q\x7F\xFC\xF5\xA6\xE9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19Q\x92\x91\x90aO\x03V[`@Q\x80\x91\x03\x90\xFD[`\x01\x84_\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x84`\x02\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_\x81\x80T\x90P\x90P\x7F{\xF1\xB4,\x10\xE9I|\x87\x96 \xC5\xB7\xAF\xCE\xD1\x0B\xDA\x17\xD8\xC9\x0B\"\xF0\xE3\xBCk/\xD6\xCE\xD0\xBD\x8B\x8B\x8B\x8B\x8B3`@Qa\x1A\x84\x96\x95\x94\x93\x92\x91\x90aXsV[`@Q\x80\x91\x03\x90\xA1\x85`\x01\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x1A\xBEWPa\x1A\xBD\x81a.\x08V[[\x15a\x1C\xC5W`\x01\x86`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x89\x89\x87`\x0B\x01_\x8E\x81R` \x01\x90\x81R` \x01_ \x91\x82a\x1B\x10\x92\x91\x90aT\xD0V[P\x83\x86`\x03\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x8A\x86`\x0C\x01\x81\x90UP_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1BMWa\x1BLaD\x87V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1B\x80W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1BkW\x90P[P\x90P_[\x82\x81\x10\x15a\x1C\x85Ws\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3\xB2\xA8t\x85\x83\x81T\x81\x10a\x1B\xD0Wa\x1B\xCFaQ\x8BV[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1C\x14\x91\x90aM\xE1V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C.W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1CV\x91\x90aW\x82V[``\x01Q\x82\x82\x81Q\x81\x10a\x1CmWa\x1ClaQ\x8BV[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa\x1B\x85V[P\x7F\"X\xB7?\xAE\xD3?\xB2\xE2\xEAED\x03\xBE\xF9t\x92\x0C\xAFh*\xB3\xA7#HO\xCFgU;\x16\xA2\x8C\x82\x8D\x8D`@Qa\x1C\xBB\x94\x93\x92\x91\x90aX\xC8V[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPPPPPV[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D/W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1DS\x91\x90aM\xB6V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x1D\xC2W3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1D\xB9\x91\x90aM\xE1V[`@Q\x80\x91\x03\x90\xFD[\x7F\x11\xDBB\xC1\x87\x8F.(\x19$\x1FRP\x98Ec\xF0l\xF2(\x18\xE7\xAD\xB8jf\x92\x1D\x15\xD5\x9D?`@Q`@Q\x80\x91\x03\x90\xA1V[_``\x80_\x80_``_a\x1E\x02a2\xF6V[\x90P_\x80\x1B\x81_\x01T\x14\x80\x15a\x1E\x1DWP_\x80\x1B\x81`\x01\x01T\x14[a\x1E\\W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1ES\x90aYWV[`@Q\x80\x91\x03\x90\xFD[a\x1Eda3\x1DV[a\x1Ela3\xBBV[F0_\x80\x1B_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\x8BWa\x1E\x8AaD\x87V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E\xB9W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94\x93\x92\x91\x90\x97P\x97P\x97P\x97P\x97P\x97P\x97PP\x90\x91\x92\x93\x94\x95\x96V[``\x80_a\x1F\x05a+jV[\x90P\x80`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x1FhW\x83`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1F_\x91\x90aKzV[`@Q\x80\x91\x03\x90\xFD[_\x81`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x82`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a \x1FW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x1F\xD6W[PPPPP\x90P_\x81Q\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a FWa EaD\x87V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a yW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a dW\x90P[P\x90P_[\x82\x81\x10\x15a!^Ws\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3\xB2\xA8t\x85\x83\x81Q\x81\x10a \xC9Wa \xC8aQ\x8BV[[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a \xED\x91\x90aM\xE1V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\x07W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!/\x91\x90aW\x82V[``\x01Q\x82\x82\x81Q\x81\x10a!FWa!EaQ\x8BV[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa ~V[P\x80\x85`\x07\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\"\x8CW\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x01\x81\x11\x15a!\xD7Wa!\xD6aB%V[[`\x01\x81\x11\x15a!\xE9Wa!\xE8aB%V[[\x81R` \x01`\x01\x82\x01\x80Ta!\xFD\x90aS\x03V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\")\x90aS\x03V[\x80\x15a\"tW\x80`\x1F\x10a\"KWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\"tV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\"WW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a!\x93V[PPPP\x90P\x96P\x96PPPPPP\x91P\x91V[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[_\x80a\"\xE3a+jV[\x90P\x80`\x0C\x01T\x91PP\x90V[``\x80_a\"\xFCa+jV[\x90P\x80`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a#_W\x83`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a#V\x91\x90aKzV[`@Q\x80\x91\x03\x90\xFD[_\x81`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x82`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a$\x16W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a#\xCDW[PPPPP\x90P_\x81Q\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$=Wa$<aD\x87V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a$pW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a$[W\x90P[P\x90P_[\x82\x81\x10\x15a%UWs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3\xB2\xA8t\x85\x83\x81Q\x81\x10a$\xC0Wa$\xBFaQ\x8BV[[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a$\xE4\x91\x90aM\xE1V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\xFEW=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%&\x91\x90aW\x82V[``\x01Q\x82\x82\x81Q\x81\x10a%=Wa%<aQ\x8BV[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa$uV[P\x80\x85`\x0B\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80\x80Ta%v\x90aS\x03V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta%\xA2\x90aS\x03V[\x80\x15a%\xEDW\x80`\x1F\x10a%\xC4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a%\xEDV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a%\xD0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x96P\x96PPPPPP\x91P\x91V[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&_W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\x83\x91\x90aM\xB6V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a&\xF2W3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&\xE9\x91\x90aM\xE1V[`@Q\x80\x91\x03\x90\xFD[_a&\xFBa+jV[\x90P_\x81`\x05\x01T\x90P`\xF8`\x04`\x06\x81\x11\x15a'\x1BWa'\x1AaB%V[[\x90\x1B\x81\x14\x15\x80\x15a'IWP\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15a'\x8BW\x80`@Q\x7F;\x85=\xA8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a'\x82\x91\x90aKzV[`@Q\x80\x91\x03\x90\xFD[\x81`\x04\x01_\x81T\x80\x92\x91\x90a'\x9F\x90aN'V[\x91\x90PUP_\x82`\x04\x01T\x90P\x82`\x05\x01_\x81T\x80\x92\x91\x90a'\xC0\x90aN'V[\x91\x90PUP_\x83`\x05\x01T\x90P\x80\x84`\x06\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x81\x84`\x06\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UP_\x85\x85`\r\x01_\x85\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a(3Wa(2aB%V[[\x02\x17\x90UP\x7F\x02\x02@\x07\xD9et\xDB\xC9\xD1\x13(\xBF\xEE\x98\x93\xE7\xC7\xBBN\xF4\xAA\x80m\xF3;\xFD\xF4T\xEB^`\x83\x82\x88`@Qa(k\x93\x92\x91\x90aNnV[`@Q\x80\x91\x03\x90\xA1PPPPPPV[_\x80a(\x85a+jV[\x90P\x80`\x08\x01T\x91PP\x90V[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(\xEFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\x13\x91\x90aM\xB6V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a)\x82W3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a)y\x91\x90aM\xE1V[`@Q\x80\x91\x03\x90\xFD[_a)\x8Ba+jV[\x90P\x80`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a)\xEEW\x81`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a)\xE5\x91\x90aKzV[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P_\x82`\r\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x82`\x0E\x01_\x81T\x80\x92\x91\x90a*<\x90aN'V[\x91\x90PUP_\x83`\x0E\x01T\x90P\x7F\x1C\xCBUE\xC4\xC8\xDBP\xA0\xF5\xB4\x16I\x95&\x92\x9FhSN\xD4\x7Fl\xFDL\x9F\x06\x90u\xE6\x0BE\x83\x86\x83\x85`@Qa*~\x94\x93\x92\x91\x90aYuV[`@Q\x80\x91\x03\x90\xA1PPPPPV[``_`\x01a*\x9B\x84a4YV[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xB9Wa*\xB8aD\x87V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a*\xEBW\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x83\x01\x01\x90P[`\x01\x15a+LW\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a+AWa+@aY\xB8V[[\x04\x94P_\x85\x03a*\xF8W[\x81\x93PPPP\x91\x90PV[_\x80a+aa5\xAAV[\x90P\x80\x91PP\x90V[_\x7F\x0B\x8F\xDB\x1F\ncV\xDD \xA6\xCB\xC6\xF9f\x8F\xAC#\xB8_\x96W]\x10\xE33\xE6\x03\xFA\xA7\x94\xAC\0\x90P\x90V[_a+\x9Aa+WV[_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a+\xBDa5\xD3V[a+\xC7\x82\x82a6\x13V[PPV[_\x80\x83\x83\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\xEAWa+\xE9aD\x87V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a,\x18W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_[\x84\x84\x90P\x81\x10\x15a-\x1CW`@Q\x80``\x01`@R\x80`%\x81R` \x01a_\x87`%\x919\x80Q\x90` \x01 \x85\x85\x83\x81\x81\x10a,[Wa,ZaQ\x8BV[[\x90P` \x02\x81\x01\x90a,m\x91\x90aQ\xC4V[_\x01` \x81\x01\x90a,~\x91\x90aY\xE5V[\x86\x86\x84\x81\x81\x10a,\x91Wa,\x90aQ\x8BV[[\x90P` \x02\x81\x01\x90a,\xA3\x91\x90aQ\xC4V[\x80` \x01\x90a,\xB2\x91\x90aRjV[`@Qa,\xC0\x92\x91\x90aZ>V[`@Q\x80\x91\x03\x90 `@Q` \x01a,\xDA\x93\x92\x91\x90aZeV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x82\x81Q\x81\x10a-\x03Wa-\x02aQ\x8BV[[` \x02` \x01\x01\x81\x81RPP\x80\x80`\x01\x01\x91PPa,\x1DV[Pa-\x97`@Q\x80`\xA0\x01`@R\x80`r\x81R` \x01a_\x15`r\x919\x80Q\x90` \x01 \x87\x87\x84`@Q` \x01a-S\x91\x90a[KV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a-|\x94\x93\x92\x91\x90a[aV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a6dV[\x91PP\x94\x93PPPPV[_\x80a-\xF1\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa6}V[\x90Pa-\xFD\x813a6\xA7V[\x80\x91PP\x93\x92PPPV[_\x80s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xB4r+\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.gW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\x8B\x91\x90a[\xB8V[\x90P\x80\x83\x10\x15\x91PP\x91\x90PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a/FWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a/-a7\xB8V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a/}W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/\xDCW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\0\x91\x90aM\xB6V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a0oW3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a0f\x91\x90aM\xE1V[`@Q\x80\x91\x03\x90\xFD[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a0\xDAWP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\xD7\x91\x90a\\\rV[`\x01[a1\x1BW\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a1\x12\x91\x90aM\xE1V[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14a1\x81W\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a1x\x91\x90aF\x1DV[`@Q\x80\x91\x03\x90\xFD[a1\x8B\x83\x83a8\x0BV[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a2\x15W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_a2h`@Q\x80``\x01`@R\x80`,\x81R` \x01a^\xE9`,\x919\x80Q\x90` \x01 \x83`@Q` \x01a2M\x92\x91\x90a\\8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a6dV[\x90P\x91\x90PV[_a2\xEC`@Q\x80`\x80\x01`@R\x80`F\x81R` \x01a^\xA3`F\x919\x80Q\x90` \x01 \x86\x86\x86\x86`@Q` \x01a2\xA8\x92\x91\x90aZ>V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a2\xD1\x94\x93\x92\x91\x90a[aV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a6dV[\x90P\x94\x93PPPPV[_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x90P\x90V[``_a3(a2\xF6V[\x90P\x80`\x02\x01\x80Ta39\x90aS\x03V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta3e\x90aS\x03V[\x80\x15a3\xB0W\x80`\x1F\x10a3\x87Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a3\xB0V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a3\x93W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[``_a3\xC6a2\xF6V[\x90P\x80`\x03\x01\x80Ta3\xD7\x90aS\x03V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta4\x03\x90aS\x03V[\x80\x15a4NW\x80`\x1F\x10a4%Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a4NV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a41W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[_\x80_\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a4\xB5Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81a4\xABWa4\xAAaY\xB8V[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a4\xF2Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81a4\xE8Wa4\xE7aY\xB8V[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10a5!Wf#\x86\xF2o\xC1\0\0\x83\x81a5\x17Wa5\x16aY\xB8V[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10a5JWc\x05\xF5\xE1\0\x83\x81a5@Wa5?aY\xB8V[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10a5oWa'\x10\x83\x81a5eWa5daY\xB8V[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10a5\x92W`d\x83\x81a5\x88Wa5\x87aY\xB8V[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10a5\xA1W`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0_\x1B\x90P\x90V[a5\xDBa8}V[a6\x11W`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a6\x1Ba5\xD3V[_a6$a2\xF6V[\x90P\x82\x81`\x02\x01\x90\x81a67\x91\x90a\\\xB7V[P\x81\x81`\x03\x01\x90\x81a6I\x91\x90a\\\xB7V[P_\x80\x1B\x81_\x01\x81\x90UP_\x80\x1B\x81`\x01\x01\x81\x90UPPPPV[_a6va6pa8\x9BV[\x83a8\xA9V[\x90P\x91\x90PV[_\x80_\x80a6\x8B\x86\x86a8\xE9V[\x92P\x92P\x92Pa6\x9B\x82\x82a9>V[\x82\x93PPPP\x92\x91PPV[a6\xB0\x82a:\xA0V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3\xB2\xA8t\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a7\x14\x91\x90aM\xE1V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7.W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7V\x91\x90aW\x82V[` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a7\xB4W\x81\x81`@Q\x7F\r\x86\xF5!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a7\xAB\x92\x91\x90a]\x86V[`@Q\x80\x91\x03\x90\xFD[PPV[_a7\xE4\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba;pV[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a8\x14\x82a;yV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15a8pWa8j\x82\x82a<BV[Pa8yV[a8xa<\xC2V[[PPV[_a8\x86a+WV[_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[_a8\xA4a<\xFEV[\x90P\x90V[_`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x02\x82\x01R\x82`\"\x82\x01R`B\x81 \x91PP\x92\x91PPV[_\x80_`A\x84Q\x03a9)W_\x80_` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90Pa9\x1B\x88\x82\x85\x85a=aV[\x95P\x95P\x95PPPPa97V[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15a9QWa9PaB%V[[\x82`\x03\x81\x11\x15a9dWa9caB%V[[\x03\x15a:\x9CW`\x01`\x03\x81\x11\x15a9~Wa9}aB%V[[\x82`\x03\x81\x11\x15a9\x91Wa9\x90aB%V[[\x03a9\xC8W`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15a9\xDCWa9\xDBaB%V[[\x82`\x03\x81\x11\x15a9\xEFWa9\xEEaB%V[[\x03a:3W\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a:*\x91\x90aKzV[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15a:FWa:EaB%V[[\x82`\x03\x81\x11\x15a:YWa:XaB%V[[\x03a:\x9BW\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a:\x92\x91\x90aF\x1DV[`@Q\x80\x91\x03\x90\xFD[[PPV[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c =\x01\x14\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a:\xED\x91\x90aM\xE1V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;\x08W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;,\x91\x90aN\xD8V[a;mW\x80`@Q\x7F*|n\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a;d\x91\x90aM\xE1V[`@Q\x80\x91\x03\x90\xFD[PV[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a;\xD4W\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a;\xCB\x91\x90aM\xE1V[`@Q\x80\x91\x03\x90\xFD[\x80a<\0\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba;pV[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qa<k\x91\x90a]\xDDV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a<\xA3W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a<\xA8V[``\x91P[P\x91P\x91Pa<\xB8\x85\x83\x83a>HV[\x92PPP\x92\x91PPV[_4\x11\x15a<\xFCW`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa=(a>\xD5V[a=0a?KV[F0`@Q` \x01a=F\x95\x94\x93\x92\x91\x90a]\xF3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x80_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15a=\x9DW_`\x03\x85\x92P\x92P\x92Pa>>V[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@Qa=\xC0\x94\x93\x92\x91\x90a^_V[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a=\xE0W=_\x80>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a>1W_`\x01_\x80\x1B\x93P\x93P\x93PPa>>V[\x80_\x80_\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[``\x82a>]Wa>X\x82a?\xC2V[a>\xCDV[_\x82Q\x14\x80\x15a>\x83WP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a>\xC5W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a>\xBC\x91\x90aM\xE1V[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa>\xCEV[[\x93\x92PPPV[_\x80a>\xDFa2\xF6V[\x90P_a>\xEAa3\x1DV[\x90P_\x81Q\x11\x15a?\x06W\x80\x80Q\x90` \x01 \x92PPPa?HV[_\x82_\x01T\x90P_\x80\x1B\x81\x14a?!W\x80\x93PPPPa?HV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x80a?Ua2\xF6V[\x90P_a?`a3\xBBV[\x90P_\x81Q\x11\x15a?|W\x80\x80Q\x90` \x01 \x92PPPa?\xBFV[_\x82`\x01\x01T\x90P_\x80\x1B\x81\x14a?\x98W\x80\x93PPPPa?\xBFV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x81Q\x11\x15a?\xD3W\x80Q` \x82\x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15a@<W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa@!V[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a@a\x82a@\x05V[a@k\x81\x85a@\x0FV[\x93Pa@{\x81\x85` \x86\x01a@\x1FV[a@\x84\x81a@GV[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra@\xA7\x81\x84a@WV[\x90P\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x81\x90P\x91\x90PV[a@\xD2\x81a@\xC0V[\x81\x14a@\xDCW_\x80\xFD[PV[_\x815\x90Pa@\xED\x81a@\xC9V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aA\x08WaA\x07a@\xB8V[[_aA\x15\x84\x82\x85\x01a@\xDFV[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_aAp\x82aAGV[\x90P\x91\x90PV[aA\x80\x81aAfV[\x82RPPV[_aA\x91\x83\x83aAwV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aA\xB3\x82aA\x1EV[aA\xBD\x81\x85aA(V[\x93PaA\xC8\x83aA8V[\x80_[\x83\x81\x10\x15aA\xF8W\x81QaA\xDF\x88\x82aA\x86V[\x97PaA\xEA\x83aA\x9DV[\x92PP`\x01\x81\x01\x90PaA\xCBV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaB\x1D\x81\x84aA\xA9V[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x02\x81\x10aBcWaBbaB%V[[PV[_\x81\x90PaBs\x82aBRV[\x91\x90PV[_aB\x82\x82aBfV[\x90P\x91\x90PV[aB\x92\x81aBxV[\x82RPPV[_` \x82\x01\x90PaB\xAB_\x83\x01\x84aB\x89V[\x92\x91PPV[`\x02\x81\x10aB\xBDW_\x80\xFD[PV[_\x815\x90PaB\xCE\x81aB\xB1V[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15aB\xEAWaB\xE9a@\xB8V[[_aB\xF7\x85\x82\x86\x01a@\xDFV[\x92PP` aC\x08\x85\x82\x86\x01aB\xC0V[\x91PP\x92P\x92\x90PV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12aC3WaC2aC\x12V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aCPWaCOaC\x16V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aClWaCkaC\x1AV[[\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12aC\x88WaC\x87aC\x12V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aC\xA5WaC\xA4aC\x16V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15aC\xC1WaC\xC0aC\x1AV[[\x92P\x92\x90PV[_\x80_\x80_``\x86\x88\x03\x12\x15aC\xE1WaC\xE0a@\xB8V[[_aC\xEE\x88\x82\x89\x01a@\xDFV[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aD\x0FWaD\x0Ea@\xBCV[[aD\x1B\x88\x82\x89\x01aC\x1EV[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aD>WaD=a@\xBCV[[aDJ\x88\x82\x89\x01aCsV[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[aDb\x81aAfV[\x81\x14aDlW_\x80\xFD[PV[_\x815\x90PaD}\x81aDYV[\x92\x91PPV[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[aD\xBD\x82a@GV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aD\xDCWaD\xDBaD\x87V[[\x80`@RPPPV[_aD\xEEa@\xAFV[\x90PaD\xFA\x82\x82aD\xB4V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aE\x19WaE\x18aD\x87V[[aE\"\x82a@GV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_aEOaEJ\x84aD\xFFV[aD\xE5V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15aEkWaEjaD\x83V[[aEv\x84\x82\x85aE/V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aE\x92WaE\x91aC\x12V[[\x815aE\xA2\x84\x82` \x86\x01aE=V[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15aE\xC1WaE\xC0a@\xB8V[[_aE\xCE\x85\x82\x86\x01aDoV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aE\xEFWaE\xEEa@\xBCV[[aE\xFB\x85\x82\x86\x01aE~V[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[aF\x17\x81aF\x05V[\x82RPPV[_` \x82\x01\x90PaF0_\x83\x01\x84aF\x0EV[\x92\x91PPV[_\x80_`@\x84\x86\x03\x12\x15aFMWaFLa@\xB8V[[_aFZ\x86\x82\x87\x01a@\xDFV[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aF{WaFza@\xBCV[[aF\x87\x86\x82\x87\x01aCsV[\x92P\x92PP\x92P\x92P\x92V[_\x80_\x80_``\x86\x88\x03\x12\x15aF\xACWaF\xABa@\xB8V[[_aF\xB9\x88\x82\x89\x01a@\xDFV[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aF\xDAWaF\xD9a@\xBCV[[aF\xE6\x88\x82\x89\x01aCsV[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aG\tWaG\x08a@\xBCV[[aG\x15\x88\x82\x89\x01aCsV[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[_\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[aGX\x81aG$V[\x82RPPV[aGg\x81a@\xC0V[\x82RPPV[aGv\x81aAfV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[aG\xAE\x81a@\xC0V[\x82RPPV[_aG\xBF\x83\x83aG\xA5V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aG\xE1\x82aG|V[aG\xEB\x81\x85aG\x86V[\x93PaG\xF6\x83aG\x96V[\x80_[\x83\x81\x10\x15aH&W\x81QaH\r\x88\x82aG\xB4V[\x97PaH\x18\x83aG\xCBV[\x92PP`\x01\x81\x01\x90PaG\xF9V[P\x85\x93PPPP\x92\x91PPV[_`\xE0\x82\x01\x90PaHF_\x83\x01\x8AaGOV[\x81\x81\x03` \x83\x01RaHX\x81\x89a@WV[\x90P\x81\x81\x03`@\x83\x01RaHl\x81\x88a@WV[\x90PaH{``\x83\x01\x87aG^V[aH\x88`\x80\x83\x01\x86aGmV[aH\x95`\xA0\x83\x01\x85aF\x0EV[\x81\x81\x03`\xC0\x83\x01RaH\xA7\x81\x84aG\xD7V[\x90P\x98\x97PPPPPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aH\xF8\x82a@\x05V[aI\x02\x81\x85aH\xDEV[\x93PaI\x12\x81\x85` \x86\x01a@\x1FV[aI\x1B\x81a@GV[\x84\x01\x91PP\x92\x91PPV[_aI1\x83\x83aH\xEEV[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aIO\x82aH\xB5V[aIY\x81\x85aH\xBFV[\x93P\x83` \x82\x02\x85\x01aIk\x85aH\xCFV[\x80_[\x85\x81\x10\x15aI\xA6W\x84\x84\x03\x89R\x81QaI\x87\x85\x82aI&V[\x94PaI\x92\x83aI9V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaInV[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[`\x02\x81\x10aI\xF2WaI\xF1aB%V[[PV[_\x81\x90PaJ\x02\x82aI\xE1V[\x91\x90PV[_aJ\x11\x82aI\xF5V[\x90P\x91\x90PV[aJ!\x81aJ\x07V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aJK\x82aJ'V[aJU\x81\x85aJ1V[\x93PaJe\x81\x85` \x86\x01a@\x1FV[aJn\x81a@GV[\x84\x01\x91PP\x92\x91PPV[_`@\x83\x01_\x83\x01QaJ\x8E_\x86\x01\x82aJ\x18V[P` \x83\x01Q\x84\x82\x03` \x86\x01RaJ\xA6\x82\x82aJAV[\x91PP\x80\x91PP\x92\x91PPV[_aJ\xBE\x83\x83aJyV[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aJ\xDC\x82aI\xB8V[aJ\xE6\x81\x85aI\xC2V[\x93P\x83` \x82\x02\x85\x01aJ\xF8\x85aI\xD2V[\x80_[\x85\x81\x10\x15aK3W\x84\x84\x03\x89R\x81QaK\x14\x85\x82aJ\xB3V[\x94PaK\x1F\x83aJ\xC6V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaJ\xFBV[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01RaK]\x81\x85aIEV[\x90P\x81\x81\x03` \x83\x01RaKq\x81\x84aJ\xD2V[\x90P\x93\x92PPPV[_` \x82\x01\x90PaK\x8D_\x83\x01\x84aG^V[\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aK\xAD\x82aJ'V[aK\xB7\x81\x85aK\x93V[\x93PaK\xC7\x81\x85` \x86\x01a@\x1FV[aK\xD0\x81a@GV[\x84\x01\x91PP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01RaK\xF3\x81\x85aIEV[\x90P\x81\x81\x03` \x83\x01RaL\x07\x81\x84aK\xA3V[\x90P\x93\x92PPPV[_` \x82\x84\x03\x12\x15aL%WaL$a@\xB8V[[_aL2\x84\x82\x85\x01aB\xC0V[\x91PP\x92\x91PPV[_\x81\x90P\x92\x91PPV[_aLO\x82a@\x05V[aLY\x81\x85aL;V[\x93PaLi\x81\x85` \x86\x01a@\x1FV[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aL\xA9`\x02\x83aL;V[\x91PaL\xB4\x82aLuV[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aL\xF3`\x01\x83aL;V[\x91PaL\xFE\x82aL\xBFV[`\x01\x82\x01\x90P\x91\x90PV[_aM\x14\x82\x87aLEV[\x91PaM\x1F\x82aL\x9DV[\x91PaM+\x82\x86aLEV[\x91PaM6\x82aL\xE7V[\x91PaMB\x82\x85aLEV[\x91PaMM\x82aL\xE7V[\x91PaMY\x82\x84aLEV[\x91P\x81\x90P\x95\x94PPPPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[aM\x83\x81aMgV[\x82RPPV[_` \x82\x01\x90PaM\x9C_\x83\x01\x84aMzV[\x92\x91PPV[_\x81Q\x90PaM\xB0\x81aDYV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aM\xCBWaM\xCAa@\xB8V[[_aM\xD8\x84\x82\x85\x01aM\xA2V[\x91PP\x92\x91PPV[_` \x82\x01\x90PaM\xF4_\x83\x01\x84aGmV[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_aN1\x82a@\xC0V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03aNcWaNbaM\xFAV[[`\x01\x82\x01\x90P\x91\x90PV[_``\x82\x01\x90PaN\x81_\x83\x01\x86aG^V[aN\x8E` \x83\x01\x85aG^V[aN\x9B`@\x83\x01\x84aB\x89V[\x94\x93PPPPV[_\x81\x15\x15\x90P\x91\x90PV[aN\xB7\x81aN\xA3V[\x81\x14aN\xC1W_\x80\xFD[PV[_\x81Q\x90PaN\xD2\x81aN\xAEV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aN\xEDWaN\xECa@\xB8V[[_aN\xFA\x84\x82\x85\x01aN\xC4V[\x91PP\x92\x91PPV[_`@\x82\x01\x90PaO\x16_\x83\x01\x85aG^V[aO#` \x83\x01\x84aGmV[\x93\x92PPPV[_\x81\x90P\x91\x90PV[`\x02\x81\x10aO?W_\x80\xFD[PV[_\x815\x90PaOP\x81aO3V[\x92\x91PPV[_aOd` \x84\x01\x84aOBV[\x90P\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aO\x94WaO\x93aOtV[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aO\xBCWaO\xBBaOlV[[`\x01\x82\x026\x03\x83\x13\x15aO\xD2WaO\xD1aOpV[[P\x92P\x92\x90PV[_aO\xE5\x83\x85aJ1V[\x93PaO\xF2\x83\x85\x84aE/V[aO\xFB\x83a@GV[\x84\x01\x90P\x93\x92PPPV[_`@\x83\x01aP\x17_\x84\x01\x84aOVV[aP#_\x86\x01\x82aJ\x18V[PaP1` \x84\x01\x84aOxV[\x85\x83\x03` \x87\x01RaPD\x83\x82\x84aO\xDAV[\x92PPP\x80\x91PP\x92\x91PPV[_aP]\x83\x83aP\x06V[\x90P\x92\x91PPV[_\x825`\x01`@\x03\x836\x03\x03\x81\x12aP\x80WaP\x7FaOtV[[\x82\x81\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aP\xA3\x83\x85aI\xC2V[\x93P\x83` \x84\x02\x85\x01aP\xB5\x84aO*V[\x80_[\x87\x81\x10\x15aP\xF8W\x84\x84\x03\x89RaP\xCF\x82\x84aPeV[aP\xD9\x85\x82aPRV[\x94PaP\xE4\x83aP\x8CV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaP\xB8V[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_aQ\x15\x83\x85aK\x93V[\x93PaQ\"\x83\x85\x84aE/V[aQ+\x83a@GV[\x84\x01\x90P\x93\x92PPPV[_`\x80\x82\x01\x90PaQI_\x83\x01\x89aG^V[\x81\x81\x03` \x83\x01RaQ\\\x81\x87\x89aP\x98V[\x90P\x81\x81\x03`@\x83\x01RaQq\x81\x85\x87aQ\nV[\x90PaQ\x80``\x83\x01\x84aGmV[\x97\x96PPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x825`\x01`@\x03\x836\x03\x03\x81\x12aQ\xDFWaQ\xDEaQ\xB8V[[\x80\x83\x01\x91PP\x92\x91PPV[_\x815aQ\xF7\x81aO3V[\x80\x91PP\x91\x90PV[_\x81_\x1B\x90P\x91\x90PV[_`\xFFaR\x17\x84aR\0V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_aR7\x82aI\xF5V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[aRP\x82aR-V[aRcaR\\\x82aR>V[\x83TaR\x0BV[\x82UPPPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aR\x86WaR\x85aQ\xB8V[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aR\xA8WaR\xA7aQ\xBCV[[` \x83\x01\x92P`\x01\x82\x026\x03\x83\x13\x15aR\xC4WaR\xC3aQ\xC0V[[P\x92P\x92\x90PV[_\x82\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80aS\x1AW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aS-WaS,aR\xD6V[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02aS\x8F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82aSTV[aS\x99\x86\x83aSTV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_aS\xD4aS\xCFaS\xCA\x84a@\xC0V[aS\xB1V[a@\xC0V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[aS\xED\x83aS\xBAV[aT\x01aS\xF9\x82aS\xDBV[\x84\x84TaS`V[\x82UPPPPV[_\x90V[aT\x15aT\tV[aT \x81\x84\x84aS\xE4V[PPPV[[\x81\x81\x10\x15aTCWaT8_\x82aT\rV[`\x01\x81\x01\x90PaT&V[PPV[`\x1F\x82\x11\x15aT\x88WaTY\x81aS3V[aTb\x84aSEV[\x81\x01` \x85\x10\x15aTqW\x81\x90P[aT\x85aT}\x85aSEV[\x83\x01\x82aT%V[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_aT\xA8_\x19\x84`\x08\x02aT\x8DV[\x19\x80\x83\x16\x91PP\x92\x91PPV[_aT\xC0\x83\x83aT\x99V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[aT\xDA\x83\x83aR\xCCV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aT\xF3WaT\xF2aD\x87V[[aT\xFD\x82TaS\x03V[aU\x08\x82\x82\x85aTGV[_`\x1F\x83\x11`\x01\x81\x14aU5W_\x84\x15aU#W\x82\x87\x015\x90P[aU-\x85\x82aT\xB5V[\x86UPaU\x94V[`\x1F\x19\x84\x16aUC\x86aS3V[_[\x82\x81\x10\x15aUjW\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PaUEV[\x86\x83\x10\x15aU\x87W\x84\x89\x015aU\x83`\x1F\x89\x16\x82aT\x99V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[aU\xA8\x83\x83\x83aT\xD0V[PPPV[_\x81\x01_\x83\x01\x80aU\xBD\x81aQ\xEBV[\x90PaU\xC9\x81\x84aRGV[PPP`\x01\x81\x01` \x83\x01aU\xDE\x81\x85aRjV[aU\xE9\x81\x83\x86aU\x9DV[PPPPPPV[aU\xFB\x82\x82aU\xADV[PPV[_\x80\xFD[_\x80\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aV!WaV aD\x87V[[aV*\x82a@GV[\x90P` \x81\x01\x90P\x91\x90PV[_aVIaVD\x84aV\x07V[aD\xE5V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15aVeWaVdaD\x83V[[aVp\x84\x82\x85a@\x1FV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aV\x8CWaV\x8BaC\x12V[[\x81QaV\x9C\x84\x82` \x86\x01aV7V[\x91PP\x92\x91PPV[_`\xA0\x82\x84\x03\x12\x15aV\xBAWaV\xB9aU\xFFV[[aV\xC4`\xA0aD\xE5V[\x90P_aV\xD3\x84\x82\x85\x01aM\xA2V[_\x83\x01RP` aV\xE6\x84\x82\x85\x01aM\xA2V[` \x83\x01RP`@\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aW\nWaW\taV\x03V[[aW\x16\x84\x82\x85\x01aVxV[`@\x83\x01RP``\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aW:WaW9aV\x03V[[aWF\x84\x82\x85\x01aVxV[``\x83\x01RP`\x80\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aWjWaWiaV\x03V[[aWv\x84\x82\x85\x01aVxV[`\x80\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15aW\x97WaW\x96a@\xB8V[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aW\xB4WaW\xB3a@\xBCV[[aW\xC0\x84\x82\x85\x01aV\xA5V[\x91PP\x92\x91PPV[_``\x82\x01\x90PaW\xDC_\x83\x01\x87aG^V[\x81\x81\x03` \x83\x01RaW\xEE\x81\x86aIEV[\x90P\x81\x81\x03`@\x83\x01RaX\x03\x81\x84\x86aP\x98V[\x90P\x95\x94PPPPPV[_``\x82\x01\x90PaX!_\x83\x01\x87aG^V[\x81\x81\x03` \x83\x01RaX4\x81\x85\x87aQ\nV[\x90PaXC`@\x83\x01\x84aGmV[\x95\x94PPPPPV[_`@\x82\x01\x90PaX__\x83\x01\x85aG^V[aXl` \x83\x01\x84aG^V[\x93\x92PPPV[_`\x80\x82\x01\x90PaX\x86_\x83\x01\x89aG^V[\x81\x81\x03` \x83\x01RaX\x99\x81\x87\x89aQ\nV[\x90P\x81\x81\x03`@\x83\x01RaX\xAE\x81\x85\x87aQ\nV[\x90PaX\xBD``\x83\x01\x84aGmV[\x97\x96PPPPPPPV[_``\x82\x01\x90PaX\xDB_\x83\x01\x87aG^V[\x81\x81\x03` \x83\x01RaX\xED\x81\x86aIEV[\x90P\x81\x81\x03`@\x83\x01RaY\x02\x81\x84\x86aQ\nV[\x90P\x95\x94PPPPPV[\x7FEIP712: Uninitialized\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aYA`\x15\x83a@\x0FV[\x91PaYL\x82aY\rV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaYn\x81aY5V[\x90P\x91\x90PV[_`\x80\x82\x01\x90PaY\x88_\x83\x01\x87aG^V[aY\x95` \x83\x01\x86aG^V[aY\xA2`@\x83\x01\x85aG^V[aY\xAF``\x83\x01\x84aB\x89V[\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aY\xFAWaY\xF9a@\xB8V[[_aZ\x07\x84\x82\x85\x01aOBV[\x91PP\x92\x91PPV[_\x81\x90P\x92\x91PPV[_aZ%\x83\x85aZ\x10V[\x93PaZ2\x83\x85\x84aE/V[\x82\x84\x01\x90P\x93\x92PPPV[_aZJ\x82\x84\x86aZ\x1AV[\x91P\x81\x90P\x93\x92PPPV[aZ_\x81aJ\x07V[\x82RPPV[_``\x82\x01\x90PaZx_\x83\x01\x86aF\x0EV[aZ\x85` \x83\x01\x85aZVV[aZ\x92`@\x83\x01\x84aF\x0EV[\x94\x93PPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[aZ\xC6\x81aF\x05V[\x82RPPV[_aZ\xD7\x83\x83aZ\xBDV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aZ\xF9\x82aZ\x9AV[a[\x03\x81\x85aZ\xA4V[\x93Pa[\x0E\x83aZ\xAEV[\x80_[\x83\x81\x10\x15a[>W\x81Qa[%\x88\x82aZ\xCCV[\x97Pa[0\x83aZ\xE3V[\x92PP`\x01\x81\x01\x90Pa[\x11V[P\x85\x93PPPP\x92\x91PPV[_a[V\x82\x84aZ\xEFV[\x91P\x81\x90P\x92\x91PPV[_`\x80\x82\x01\x90Pa[t_\x83\x01\x87aF\x0EV[a[\x81` \x83\x01\x86aG^V[a[\x8E`@\x83\x01\x85aG^V[a[\x9B``\x83\x01\x84aF\x0EV[\x95\x94PPPPPV[_\x81Q\x90Pa[\xB2\x81a@\xC9V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a[\xCDWa[\xCCa@\xB8V[[_a[\xDA\x84\x82\x85\x01a[\xA4V[\x91PP\x92\x91PPV[a[\xEC\x81aF\x05V[\x81\x14a[\xF6W_\x80\xFD[PV[_\x81Q\x90Pa\\\x07\x81a[\xE3V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\\\"Wa\\!a@\xB8V[[_a\\/\x84\x82\x85\x01a[\xF9V[\x91PP\x92\x91PPV[_`@\x82\x01\x90Pa\\K_\x83\x01\x85aF\x0EV[a\\X` \x83\x01\x84aG^V[\x93\x92PPPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[`\x1F\x82\x11\x15a\\\xB2Wa\\\x83\x81a\\_V[a\\\x8C\x84aSEV[\x81\x01` \x85\x10\x15a\\\x9BW\x81\x90P[a\\\xAFa\\\xA7\x85aSEV[\x83\x01\x82aT%V[PP[PPPV[a\\\xC0\x82a@\x05V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\\\xD9Wa\\\xD8aD\x87V[[a\\\xE3\x82TaS\x03V[a\\\xEE\x82\x82\x85a\\qV[_` \x90P`\x1F\x83\x11`\x01\x81\x14a]\x1FW_\x84\x15a]\rW\x82\x87\x01Q\x90P[a]\x17\x85\x82aT\xB5V[\x86UPa]~V[`\x1F\x19\x84\x16a]-\x86a\\_V[_[\x82\x81\x10\x15a]TW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa]/V[\x86\x83\x10\x15a]qW\x84\x89\x01Qa]m`\x1F\x89\x16\x82aT\x99V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_`@\x82\x01\x90Pa]\x99_\x83\x01\x85aGmV[a]\xA6` \x83\x01\x84aGmV[\x93\x92PPPV[_a]\xB7\x82aJ'V[a]\xC1\x81\x85aZ\x10V[\x93Pa]\xD1\x81\x85` \x86\x01a@\x1FV[\x80\x84\x01\x91PP\x92\x91PPV[_a]\xE8\x82\x84a]\xADV[\x91P\x81\x90P\x92\x91PPV[_`\xA0\x82\x01\x90Pa^\x06_\x83\x01\x88aF\x0EV[a^\x13` \x83\x01\x87aF\x0EV[a^ `@\x83\x01\x86aF\x0EV[a^-``\x83\x01\x85aG^V[a^:`\x80\x83\x01\x84aGmV[\x96\x95PPPPPPV[_`\xFF\x82\x16\x90P\x91\x90PV[a^Y\x81a^DV[\x82RPPV[_`\x80\x82\x01\x90Pa^r_\x83\x01\x87aF\x0EV[a^\x7F` \x83\x01\x86a^PV[a^\x8C`@\x83\x01\x85aF\x0EV[a^\x99``\x83\x01\x84aF\x0EV[\x95\x94PPPPPV\xFECrsgenVerification(uint256 crsId,uint256 maxBitLength,bytes crsDigest)PrepKeygenVerification(uint256 prepKeygenId)KeygenVerification(uint256 prepKeygenId,uint256 keyId,KeyDigest[] keyDigests)KeyDigest(uint8 keyType,bytes digest)KeyDigest(uint8 keyType,bytes digest)",
    );
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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
    /**Custom error with signature `CrsNotGenerated(uint256)` and selector `0xda32d00f`.
```solidity
error CrsNotGenerated(uint256 crsId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CrsNotGenerated {
        #[allow(missing_docs)]
        pub crsId: alloy::sol_types::private::primitives::aliases::U256,
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
        #[allow(dead_code)]
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
        impl ::core::convert::From<CrsNotGenerated> for UnderlyingRustTuple<'_> {
            fn from(value: CrsNotGenerated) -> Self {
                (value.crsId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CrsNotGenerated {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { crsId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CrsNotGenerated {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CrsNotGenerated(uint256)";
            const SELECTOR: [u8; 4] = [218u8, 50u8, 208u8, 15u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.crsId),
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
    /**Custom error with signature `CrsgenNotRequested(uint256)` and selector `0x8d8c940a`.
```solidity
error CrsgenNotRequested(uint256 crsId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CrsgenNotRequested {
        #[allow(missing_docs)]
        pub crsId: alloy::sol_types::private::primitives::aliases::U256,
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
        #[allow(dead_code)]
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
        impl ::core::convert::From<CrsgenNotRequested> for UnderlyingRustTuple<'_> {
            fn from(value: CrsgenNotRequested) -> Self {
                (value.crsId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CrsgenNotRequested {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { crsId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CrsgenNotRequested {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CrsgenNotRequested(uint256)";
            const SELECTOR: [u8; 4] = [141u8, 140u8, 148u8, 10u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.crsId),
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
    /**Custom error with signature `CrsgenOngoing(uint256)` and selector `0x061ac61d`.
```solidity
error CrsgenOngoing(uint256 crsId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CrsgenOngoing {
        #[allow(missing_docs)]
        pub crsId: alloy::sol_types::private::primitives::aliases::U256,
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
        #[allow(dead_code)]
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
        impl ::core::convert::From<CrsgenOngoing> for UnderlyingRustTuple<'_> {
            fn from(value: CrsgenOngoing) -> Self {
                (value.crsId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CrsgenOngoing {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { crsId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CrsgenOngoing {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CrsgenOngoing(uint256)";
            const SELECTOR: [u8; 4] = [6u8, 26u8, 198u8, 29u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.crsId),
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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
    /**Custom error with signature `EmptyKeyDigests(uint256)` and selector `0xe6f9083b`.
```solidity
error EmptyKeyDigests(uint256 keyId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyKeyDigests {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
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
        #[allow(dead_code)]
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
        impl ::core::convert::From<EmptyKeyDigests> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyKeyDigests) -> Self {
                (value.keyId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyKeyDigests {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { keyId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyKeyDigests {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyKeyDigests(uint256)";
            const SELECTOR: [u8; 4] = [230u8, 249u8, 8u8, 59u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
    /**Custom error with signature `KeyNotGenerated(uint256)` and selector `0x84de1331`.
```solidity
error KeyNotGenerated(uint256 keyId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KeyNotGenerated {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
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
        #[allow(dead_code)]
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
        impl ::core::convert::From<KeyNotGenerated> for UnderlyingRustTuple<'_> {
            fn from(value: KeyNotGenerated) -> Self {
                (value.keyId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KeyNotGenerated {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { keyId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KeyNotGenerated {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KeyNotGenerated(uint256)";
            const SELECTOR: [u8; 4] = [132u8, 222u8, 19u8, 49u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
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
    /**Custom error with signature `KeygenNotRequested(uint256)` and selector `0xadfab904`.
```solidity
error KeygenNotRequested(uint256 keyId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KeygenNotRequested {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
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
        #[allow(dead_code)]
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
        impl ::core::convert::From<KeygenNotRequested> for UnderlyingRustTuple<'_> {
            fn from(value: KeygenNotRequested) -> Self {
                (value.keyId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KeygenNotRequested {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { keyId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KeygenNotRequested {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KeygenNotRequested(uint256)";
            const SELECTOR: [u8; 4] = [173u8, 250u8, 185u8, 4u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
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
    /**Custom error with signature `KeygenOngoing(uint256)` and selector `0x3b853da8`.
```solidity
error KeygenOngoing(uint256 keyId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KeygenOngoing {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
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
        #[allow(dead_code)]
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
        impl ::core::convert::From<KeygenOngoing> for UnderlyingRustTuple<'_> {
            fn from(value: KeygenOngoing) -> Self {
                (value.keyId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KeygenOngoing {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { keyId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KeygenOngoing {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KeygenOngoing(uint256)";
            const SELECTOR: [u8; 4] = [59u8, 133u8, 61u8, 168u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
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
    /**Custom error with signature `KmsAlreadySignedForCrsgen(uint256,address)` and selector `0xfcf5a6e9`.
```solidity
error KmsAlreadySignedForCrsgen(uint256 crsId, address kmsSigner);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsAlreadySignedForCrsgen {
        #[allow(missing_docs)]
        pub crsId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub kmsSigner: alloy::sol_types::private::Address,
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
        #[allow(dead_code)]
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
        impl ::core::convert::From<KmsAlreadySignedForCrsgen>
        for UnderlyingRustTuple<'_> {
            fn from(value: KmsAlreadySignedForCrsgen) -> Self {
                (value.crsId, value.kmsSigner)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for KmsAlreadySignedForCrsgen {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    crsId: tuple.0,
                    kmsSigner: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KmsAlreadySignedForCrsgen {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KmsAlreadySignedForCrsgen(uint256,address)";
            const SELECTOR: [u8; 4] = [252u8, 245u8, 166u8, 233u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.crsId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.kmsSigner,
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
    /**Custom error with signature `KmsAlreadySignedForKeygen(uint256,address)` and selector `0x98fb957d`.
```solidity
error KmsAlreadySignedForKeygen(uint256 keyId, address kmsSigner);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsAlreadySignedForKeygen {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub kmsSigner: alloy::sol_types::private::Address,
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
        #[allow(dead_code)]
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
        impl ::core::convert::From<KmsAlreadySignedForKeygen>
        for UnderlyingRustTuple<'_> {
            fn from(value: KmsAlreadySignedForKeygen) -> Self {
                (value.keyId, value.kmsSigner)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for KmsAlreadySignedForKeygen {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    keyId: tuple.0,
                    kmsSigner: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KmsAlreadySignedForKeygen {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KmsAlreadySignedForKeygen(uint256,address)";
            const SELECTOR: [u8; 4] = [152u8, 251u8, 149u8, 125u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.kmsSigner,
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
    /**Custom error with signature `KmsAlreadySignedForPrepKeygen(uint256,address)` and selector `0x33ca1fe3`.
```solidity
error KmsAlreadySignedForPrepKeygen(uint256 prepKeygenId, address kmsSigner);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsAlreadySignedForPrepKeygen {
        #[allow(missing_docs)]
        pub prepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub kmsSigner: alloy::sol_types::private::Address,
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
        #[allow(dead_code)]
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
        impl ::core::convert::From<KmsAlreadySignedForPrepKeygen>
        for UnderlyingRustTuple<'_> {
            fn from(value: KmsAlreadySignedForPrepKeygen) -> Self {
                (value.prepKeygenId, value.kmsSigner)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for KmsAlreadySignedForPrepKeygen {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    prepKeygenId: tuple.0,
                    kmsSigner: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KmsAlreadySignedForPrepKeygen {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KmsAlreadySignedForPrepKeygen(uint256,address)";
            const SELECTOR: [u8; 4] = [51u8, 202u8, 31u8, 227u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.prepKeygenId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.kmsSigner,
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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
    /**Custom error with signature `PrepKeygenNotRequested(uint256)` and selector `0x0ab7f687`.
```solidity
error PrepKeygenNotRequested(uint256 prepKeygenId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PrepKeygenNotRequested {
        #[allow(missing_docs)]
        pub prepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
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
        #[allow(dead_code)]
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
        impl ::core::convert::From<PrepKeygenNotRequested> for UnderlyingRustTuple<'_> {
            fn from(value: PrepKeygenNotRequested) -> Self {
                (value.prepKeygenId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PrepKeygenNotRequested {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { prepKeygenId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for PrepKeygenNotRequested {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PrepKeygenNotRequested(uint256)";
            const SELECTOR: [u8; 4] = [10u8, 183u8, 246u8, 135u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.prepKeygenId),
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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
    /**Event with signature `ActivateCrs(uint256,string[],bytes)` and selector `0x2258b73faed33fb2e2ea454403bef974920caf682ab3a723484fcf67553b16a2`.
```solidity
event ActivateCrs(uint256 crsId, string[] kmsNodeStorageUrls, bytes crsDigest);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ActivateCrs {
        #[allow(missing_docs)]
        pub crsId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub kmsNodeStorageUrls: alloy::sol_types::private::Vec<
            alloy::sol_types::private::String,
        >,
        #[allow(missing_docs)]
        pub crsDigest: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for ActivateCrs {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ActivateCrs(uint256,string[],bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                34u8, 88u8, 183u8, 63u8, 174u8, 211u8, 63u8, 178u8, 226u8, 234u8, 69u8,
                68u8, 3u8, 190u8, 249u8, 116u8, 146u8, 12u8, 175u8, 104u8, 42u8, 179u8,
                167u8, 35u8, 72u8, 79u8, 207u8, 103u8, 85u8, 59u8, 22u8, 162u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    crsId: data.0,
                    kmsNodeStorageUrls: data.1,
                    crsDigest: data.2,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.crsId),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsNodeStorageUrls),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.crsDigest,
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
        impl alloy_sol_types::private::IntoLogData for ActivateCrs {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ActivateCrs> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ActivateCrs) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    /**Event with signature `ActivateKey(uint256,string[],(uint8,bytes)[])` and selector `0xeb85c26dbcad46b80a68a0f24cce7c2c90f0a1faded84184138839fc9e80a25b`.
```solidity
event ActivateKey(uint256 keyId, string[] kmsNodeStorageUrls, IKMSGeneration.KeyDigest[] keyDigests);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ActivateKey {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub kmsNodeStorageUrls: alloy::sol_types::private::Vec<
            alloy::sol_types::private::String,
        >,
        #[allow(missing_docs)]
        pub keyDigests: alloy::sol_types::private::Vec<
            <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ActivateKey {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
                alloy::sol_types::sol_data::Array<IKMSGeneration::KeyDigest>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ActivateKey(uint256,string[],(uint8,bytes)[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                235u8, 133u8, 194u8, 109u8, 188u8, 173u8, 70u8, 184u8, 10u8, 104u8,
                160u8, 242u8, 76u8, 206u8, 124u8, 44u8, 144u8, 240u8, 161u8, 250u8,
                222u8, 216u8, 65u8, 132u8, 19u8, 136u8, 57u8, 252u8, 158u8, 128u8, 162u8,
                91u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    keyId: data.0,
                    kmsNodeStorageUrls: data.1,
                    keyDigests: data.2,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsNodeStorageUrls),
                    <alloy::sol_types::sol_data::Array<
                        IKMSGeneration::KeyDigest,
                    > as alloy_sol_types::SolType>::tokenize(&self.keyDigests),
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
        impl alloy_sol_types::private::IntoLogData for ActivateKey {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ActivateKey> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ActivateKey) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `CrsgenRequest(uint256,uint256,uint8)` and selector `0x3f038f6f88cb3031b7718588403a2ec220576a868be07dde4c02b846ca352ef5`.
```solidity
event CrsgenRequest(uint256 crsId, uint256 maxBitLength, IKMSGeneration.ParamsType paramsType);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct CrsgenRequest {
        #[allow(missing_docs)]
        pub crsId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub maxBitLength: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub paramsType: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for CrsgenRequest {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                IKMSGeneration::ParamsType,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "CrsgenRequest(uint256,uint256,uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                63u8, 3u8, 143u8, 111u8, 136u8, 203u8, 48u8, 49u8, 183u8, 113u8, 133u8,
                136u8, 64u8, 58u8, 46u8, 194u8, 32u8, 87u8, 106u8, 134u8, 139u8, 224u8,
                125u8, 222u8, 76u8, 2u8, 184u8, 70u8, 202u8, 53u8, 46u8, 245u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    crsId: data.0,
                    maxBitLength: data.1,
                    paramsType: data.2,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.crsId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxBitLength),
                    <IKMSGeneration::ParamsType as alloy_sol_types::SolType>::tokenize(
                        &self.paramsType,
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
        impl alloy_sol_types::private::IntoLogData for CrsgenRequest {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&CrsgenRequest> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &CrsgenRequest) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `CrsgenResponse(uint256,bytes,bytes,address)` and selector `0x7bf1b42c10e9497c879620c5b7afced10bda17d8c90b22f0e3bc6b2fd6ced0bd`.
```solidity
event CrsgenResponse(uint256 crsId, bytes crsDigest, bytes signature, address kmsTxSender);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct CrsgenResponse {
        #[allow(missing_docs)]
        pub crsId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub crsDigest: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub kmsTxSender: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for CrsgenResponse {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "CrsgenResponse(uint256,bytes,bytes,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                123u8, 241u8, 180u8, 44u8, 16u8, 233u8, 73u8, 124u8, 135u8, 150u8, 32u8,
                197u8, 183u8, 175u8, 206u8, 209u8, 11u8, 218u8, 23u8, 216u8, 201u8, 11u8,
                34u8, 240u8, 227u8, 188u8, 107u8, 47u8, 214u8, 206u8, 208u8, 189u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    crsId: data.0,
                    crsDigest: data.1,
                    signature: data.2,
                    kmsTxSender: data.3,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.crsId),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.crsDigest,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.kmsTxSender,
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
        impl alloy_sol_types::private::IntoLogData for CrsgenResponse {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&CrsgenResponse> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &CrsgenResponse) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
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
    /**Event with signature `KeyReshareSameSet(uint256,uint256,uint256,uint8)` and selector `0x1ccb5545c4c8db50a0f5b416499526929f68534ed47f6cfd4c9f069075e60b45`.
```solidity
event KeyReshareSameSet(uint256 prepKeygenId, uint256 keyId, uint256 keyReshareId, IKMSGeneration.ParamsType paramsType);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct KeyReshareSameSet {
        #[allow(missing_docs)]
        pub prepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub keyReshareId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub paramsType: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for KeyReshareSameSet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                IKMSGeneration::ParamsType,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "KeyReshareSameSet(uint256,uint256,uint256,uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                28u8, 203u8, 85u8, 69u8, 196u8, 200u8, 219u8, 80u8, 160u8, 245u8, 180u8,
                22u8, 73u8, 149u8, 38u8, 146u8, 159u8, 104u8, 83u8, 78u8, 212u8, 127u8,
                108u8, 253u8, 76u8, 159u8, 6u8, 144u8, 117u8, 230u8, 11u8, 69u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    prepKeygenId: data.0,
                    keyId: data.1,
                    keyReshareId: data.2,
                    paramsType: data.3,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.prepKeygenId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.keyReshareId),
                    <IKMSGeneration::ParamsType as alloy_sol_types::SolType>::tokenize(
                        &self.paramsType,
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
        impl alloy_sol_types::private::IntoLogData for KeyReshareSameSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&KeyReshareSameSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &KeyReshareSameSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `KeygenRequest(uint256,uint256)` and selector `0x78b179176d1f19d7c28e80823deba2624da2ca2ec64b1701f3632a87c9aedc92`.
```solidity
event KeygenRequest(uint256 prepKeygenId, uint256 keyId);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct KeygenRequest {
        #[allow(missing_docs)]
        pub prepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for KeygenRequest {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "KeygenRequest(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                120u8, 177u8, 121u8, 23u8, 109u8, 31u8, 25u8, 215u8, 194u8, 142u8, 128u8,
                130u8, 61u8, 235u8, 162u8, 98u8, 77u8, 162u8, 202u8, 46u8, 198u8, 75u8,
                23u8, 1u8, 243u8, 99u8, 42u8, 135u8, 201u8, 174u8, 220u8, 146u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    prepKeygenId: data.0,
                    keyId: data.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.prepKeygenId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
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
        impl alloy_sol_types::private::IntoLogData for KeygenRequest {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&KeygenRequest> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &KeygenRequest) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    /**Event with signature `KeygenResponse(uint256,(uint8,bytes)[],bytes,address)` and selector `0x2afe64fb3afde8e2678aea84cf36223f330e2fb1286d37aed573ab9cd1db47c7`.
```solidity
event KeygenResponse(uint256 keyId, IKMSGeneration.KeyDigest[] keyDigests, bytes signature, address kmsTxSender);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct KeygenResponse {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub keyDigests: alloy::sol_types::private::Vec<
            <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub kmsTxSender: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for KeygenResponse {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IKMSGeneration::KeyDigest>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "KeygenResponse(uint256,(uint8,bytes)[],bytes,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                42u8, 254u8, 100u8, 251u8, 58u8, 253u8, 232u8, 226u8, 103u8, 138u8,
                234u8, 132u8, 207u8, 54u8, 34u8, 63u8, 51u8, 14u8, 47u8, 177u8, 40u8,
                109u8, 55u8, 174u8, 213u8, 115u8, 171u8, 156u8, 209u8, 219u8, 71u8, 199u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    keyId: data.0,
                    keyDigests: data.1,
                    signature: data.2,
                    kmsTxSender: data.3,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                    <alloy::sol_types::sol_data::Array<
                        IKMSGeneration::KeyDigest,
                    > as alloy_sol_types::SolType>::tokenize(&self.keyDigests),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.kmsTxSender,
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
        impl alloy_sol_types::private::IntoLogData for KeygenResponse {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&KeygenResponse> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &KeygenResponse) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `PRSSInit()` and selector `0x11db42c1878f2e2819241f5250984563f06cf22818e7adb86a66921d15d59d3f`.
```solidity
event PRSSInit();
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PRSSInit;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for PRSSInit {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "PRSSInit()";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                17u8, 219u8, 66u8, 193u8, 135u8, 143u8, 46u8, 40u8, 25u8, 36u8, 31u8,
                82u8, 80u8, 152u8, 69u8, 99u8, 240u8, 108u8, 242u8, 40u8, 24u8, 231u8,
                173u8, 184u8, 106u8, 102u8, 146u8, 29u8, 21u8, 213u8, 157u8, 63u8,
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
        impl alloy_sol_types::private::IntoLogData for PRSSInit {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PRSSInit> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PRSSInit) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `PrepKeygenRequest(uint256,uint256,uint8)` and selector `0x02024007d96574dbc9d11328bfee9893e7c7bb4ef4aa806df33bfdf454eb5e60`.
```solidity
event PrepKeygenRequest(uint256 prepKeygenId, uint256 epochId, IKMSGeneration.ParamsType paramsType);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PrepKeygenRequest {
        #[allow(missing_docs)]
        pub prepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub epochId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub paramsType: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for PrepKeygenRequest {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                IKMSGeneration::ParamsType,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "PrepKeygenRequest(uint256,uint256,uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                2u8, 2u8, 64u8, 7u8, 217u8, 101u8, 116u8, 219u8, 201u8, 209u8, 19u8,
                40u8, 191u8, 238u8, 152u8, 147u8, 231u8, 199u8, 187u8, 78u8, 244u8,
                170u8, 128u8, 109u8, 243u8, 59u8, 253u8, 244u8, 84u8, 235u8, 94u8, 96u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    prepKeygenId: data.0,
                    epochId: data.1,
                    paramsType: data.2,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.prepKeygenId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.epochId),
                    <IKMSGeneration::ParamsType as alloy_sol_types::SolType>::tokenize(
                        &self.paramsType,
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
        impl alloy_sol_types::private::IntoLogData for PrepKeygenRequest {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PrepKeygenRequest> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PrepKeygenRequest) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `PrepKeygenResponse(uint256,bytes,address)` and selector `0x4c715c5734ce5c18c9c12e8496e53d2a65f1ec381d476957f0f596b364a59b0c`.
```solidity
event PrepKeygenResponse(uint256 prepKeygenId, bytes signature, address kmsTxSender);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PrepKeygenResponse {
        #[allow(missing_docs)]
        pub prepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub kmsTxSender: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for PrepKeygenResponse {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "PrepKeygenResponse(uint256,bytes,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                76u8, 113u8, 92u8, 87u8, 52u8, 206u8, 92u8, 24u8, 201u8, 193u8, 46u8,
                132u8, 150u8, 229u8, 61u8, 42u8, 101u8, 241u8, 236u8, 56u8, 29u8, 71u8,
                105u8, 87u8, 240u8, 245u8, 150u8, 179u8, 100u8, 165u8, 155u8, 12u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    prepKeygenId: data.0,
                    signature: data.1,
                    kmsTxSender: data.2,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.prepKeygenId),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.kmsTxSender,
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
        impl alloy_sol_types::private::IntoLogData for PrepKeygenResponse {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PrepKeygenResponse> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PrepKeygenResponse) -> alloy_sol_types::private::LogData {
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
            #[allow(dead_code)]
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
            #[allow(dead_code)]
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
            #[allow(dead_code)]
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
    /**Function with signature `crsgenRequest(uint256,uint8)` and selector `0x3c02f834`.
```solidity
function crsgenRequest(uint256 maxBitLength, IKMSGeneration.ParamsType paramsType) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct crsgenRequestCall {
        #[allow(missing_docs)]
        pub maxBitLength: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub paramsType: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`crsgenRequest(uint256,uint8)`](crsgenRequestCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct crsgenRequestReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                IKMSGeneration::ParamsType,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<crsgenRequestCall> for UnderlyingRustTuple<'_> {
                fn from(value: crsgenRequestCall) -> Self {
                    (value.maxBitLength, value.paramsType)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for crsgenRequestCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        maxBitLength: tuple.0,
                        paramsType: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<crsgenRequestReturn> for UnderlyingRustTuple<'_> {
                fn from(value: crsgenRequestReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for crsgenRequestReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl crsgenRequestReturn {
            fn _tokenize(
                &self,
            ) -> <crsgenRequestCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for crsgenRequestCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                IKMSGeneration::ParamsType,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = crsgenRequestReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "crsgenRequest(uint256,uint8)";
            const SELECTOR: [u8; 4] = [60u8, 2u8, 248u8, 52u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.maxBitLength),
                    <IKMSGeneration::ParamsType as alloy_sol_types::SolType>::tokenize(
                        &self.paramsType,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                crsgenRequestReturn::_tokenize(ret)
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
    /**Function with signature `crsgenResponse(uint256,bytes,bytes)` and selector `0x62978787`.
```solidity
function crsgenResponse(uint256 crsId, bytes memory crsDigest, bytes memory signature) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct crsgenResponseCall {
        #[allow(missing_docs)]
        pub crsId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub crsDigest: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`crsgenResponse(uint256,bytes,bytes)`](crsgenResponseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct crsgenResponseReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<crsgenResponseCall> for UnderlyingRustTuple<'_> {
                fn from(value: crsgenResponseCall) -> Self {
                    (value.crsId, value.crsDigest, value.signature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for crsgenResponseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        crsId: tuple.0,
                        crsDigest: tuple.1,
                        signature: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<crsgenResponseReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: crsgenResponseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for crsgenResponseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl crsgenResponseReturn {
            fn _tokenize(
                &self,
            ) -> <crsgenResponseCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for crsgenResponseCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = crsgenResponseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "crsgenResponse(uint256,bytes,bytes)";
            const SELECTOR: [u8; 4] = [98u8, 151u8, 135u8, 135u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.crsId),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.crsDigest,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                crsgenResponseReturn::_tokenize(ret)
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
            #[allow(dead_code)]
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
            #[allow(dead_code)]
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
    /**Function with signature `getActiveCrsId()` and selector `0xbaff211e`.
```solidity
function getActiveCrsId() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getActiveCrsIdCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getActiveCrsId()`](getActiveCrsIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getActiveCrsIdReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<getActiveCrsIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: getActiveCrsIdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getActiveCrsIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<getActiveCrsIdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getActiveCrsIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getActiveCrsIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getActiveCrsIdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getActiveCrsId()";
            const SELECTOR: [u8; 4] = [186u8, 255u8, 33u8, 30u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getActiveCrsIdReturn = r.into();
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
                        let r: getActiveCrsIdReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getActiveKeyId()` and selector `0xd52f10eb`.
```solidity
function getActiveKeyId() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getActiveKeyIdCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getActiveKeyId()`](getActiveKeyIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getActiveKeyIdReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<getActiveKeyIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: getActiveKeyIdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getActiveKeyIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<getActiveKeyIdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getActiveKeyIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getActiveKeyIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getActiveKeyIdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getActiveKeyId()";
            const SELECTOR: [u8; 4] = [213u8, 47u8, 16u8, 235u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getActiveKeyIdReturn = r.into();
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
                        let r: getActiveKeyIdReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getConsensusTxSenders(uint256)` and selector `0x16c713d9`.
```solidity
function getConsensusTxSenders(uint256 requestId) external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getConsensusTxSendersCall {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getConsensusTxSenders(uint256)`](getConsensusTxSendersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getConsensusTxSendersReturn {
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<getConsensusTxSendersCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getConsensusTxSendersCall) -> Self {
                    (value.requestId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getConsensusTxSendersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { requestId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<getConsensusTxSendersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getConsensusTxSendersReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getConsensusTxSendersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getConsensusTxSendersCall {
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
            const SIGNATURE: &'static str = "getConsensusTxSenders(uint256)";
            const SELECTOR: [u8; 4] = [22u8, 199u8, 19u8, 217u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.requestId),
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
                        let r: getConsensusTxSendersReturn = r.into();
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
                        let r: getConsensusTxSendersReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getCrsMaterials(uint256)` and selector `0xc55b8724`.
```solidity
function getCrsMaterials(uint256 crsId) external view returns (string[] memory, bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCrsMaterialsCall {
        #[allow(missing_docs)]
        pub crsId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCrsMaterials(uint256)`](getCrsMaterialsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCrsMaterialsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Bytes,
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<getCrsMaterialsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getCrsMaterialsCall) -> Self {
                    (value.crsId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCrsMaterialsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { crsId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
            impl ::core::convert::From<getCrsMaterialsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCrsMaterialsReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCrsMaterialsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        impl getCrsMaterialsReturn {
            fn _tokenize(
                &self,
            ) -> <getCrsMaterialsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._1,
                    ),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCrsMaterialsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCrsMaterialsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
                alloy::sol_types::sol_data::Bytes,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCrsMaterials(uint256)";
            const SELECTOR: [u8; 4] = [197u8, 91u8, 135u8, 36u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.crsId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                getCrsMaterialsReturn::_tokenize(ret)
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
    /**Function with signature `getCrsParamsType(uint256)` and selector `0x45af261b`.
```solidity
function getCrsParamsType(uint256 crsId) external view returns (IKMSGeneration.ParamsType);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCrsParamsTypeCall {
        #[allow(missing_docs)]
        pub crsId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCrsParamsType(uint256)`](getCrsParamsTypeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCrsParamsTypeReturn {
        #[allow(missing_docs)]
        pub _0: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<getCrsParamsTypeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCrsParamsTypeCall) -> Self {
                    (value.crsId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCrsParamsTypeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { crsId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (IKMSGeneration::ParamsType,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getCrsParamsTypeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCrsParamsTypeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCrsParamsTypeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCrsParamsTypeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (IKMSGeneration::ParamsType,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCrsParamsType(uint256)";
            const SELECTOR: [u8; 4] = [69u8, 175u8, 38u8, 27u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.crsId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <IKMSGeneration::ParamsType as alloy_sol_types::SolType>::tokenize(
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
                        let r: getCrsParamsTypeReturn = r.into();
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
                        let r: getCrsParamsTypeReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getKeyMaterials(uint256)` and selector `0x936608ae`.
```solidity
function getKeyMaterials(uint256 keyId) external view returns (string[] memory, IKMSGeneration.KeyDigest[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeyMaterialsCall {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    ///Container type for the return parameters of the [`getKeyMaterials(uint256)`](getKeyMaterialsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeyMaterialsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Vec<
            <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<getKeyMaterialsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getKeyMaterialsCall) -> Self {
                    (value.keyId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getKeyMaterialsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { keyId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
                alloy::sol_types::sol_data::Array<IKMSGeneration::KeyDigest>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
                alloy::sol_types::private::Vec<
                    <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getKeyMaterialsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKeyMaterialsReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKeyMaterialsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        impl getKeyMaterialsReturn {
            fn _tokenize(
                &self,
            ) -> <getKeyMaterialsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::Array<
                        IKMSGeneration::KeyDigest,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getKeyMaterialsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getKeyMaterialsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
                alloy::sol_types::sol_data::Array<IKMSGeneration::KeyDigest>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getKeyMaterials(uint256)";
            const SELECTOR: [u8; 4] = [147u8, 102u8, 8u8, 174u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                getKeyMaterialsReturn::_tokenize(ret)
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
    /**Function with signature `getKeyParamsType(uint256)` and selector `0x19f4f632`.
```solidity
function getKeyParamsType(uint256 keyId) external view returns (IKMSGeneration.ParamsType);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeyParamsTypeCall {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getKeyParamsType(uint256)`](getKeyParamsTypeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeyParamsTypeReturn {
        #[allow(missing_docs)]
        pub _0: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<getKeyParamsTypeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKeyParamsTypeCall) -> Self {
                    (value.keyId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKeyParamsTypeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { keyId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (IKMSGeneration::ParamsType,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getKeyParamsTypeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKeyParamsTypeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKeyParamsTypeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getKeyParamsTypeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (IKMSGeneration::ParamsType,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getKeyParamsType(uint256)";
            const SELECTOR: [u8; 4] = [25u8, 244u8, 246u8, 50u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <IKMSGeneration::ParamsType as alloy_sol_types::SolType>::tokenize(
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
                        let r: getKeyParamsTypeReturn = r.into();
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
                        let r: getKeyParamsTypeReturn = r.into();
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
            #[allow(dead_code)]
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
            #[allow(dead_code)]
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
            #[allow(dead_code)]
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
            #[allow(dead_code)]
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
    /**Function with signature `keyReshareSameSet(uint256)` and selector `0xd65d8373`.
```solidity
function keyReshareSameSet(uint256 keyId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct keyReshareSameSetCall {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`keyReshareSameSet(uint256)`](keyReshareSameSetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct keyReshareSameSetReturn {}
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<keyReshareSameSetCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: keyReshareSameSetCall) -> Self {
                    (value.keyId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for keyReshareSameSetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { keyId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<keyReshareSameSetReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: keyReshareSameSetReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for keyReshareSameSetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl keyReshareSameSetReturn {
            fn _tokenize(
                &self,
            ) -> <keyReshareSameSetCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for keyReshareSameSetCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = keyReshareSameSetReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "keyReshareSameSet(uint256)";
            const SELECTOR: [u8; 4] = [214u8, 93u8, 131u8, 115u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                keyReshareSameSetReturn::_tokenize(ret)
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
    /**Function with signature `keygen(uint8)` and selector `0xcaa367db`.
```solidity
function keygen(IKMSGeneration.ParamsType paramsType) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct keygenCall {
        #[allow(missing_docs)]
        pub paramsType: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`keygen(uint8)`](keygenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct keygenReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (IKMSGeneration::ParamsType,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<keygenCall> for UnderlyingRustTuple<'_> {
                fn from(value: keygenCall) -> Self {
                    (value.paramsType,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for keygenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { paramsType: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<keygenReturn> for UnderlyingRustTuple<'_> {
                fn from(value: keygenReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for keygenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl keygenReturn {
            fn _tokenize(
                &self,
            ) -> <keygenCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for keygenCall {
            type Parameters<'a> = (IKMSGeneration::ParamsType,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = keygenReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "keygen(uint8)";
            const SELECTOR: [u8; 4] = [202u8, 163u8, 103u8, 219u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IKMSGeneration::ParamsType as alloy_sol_types::SolType>::tokenize(
                        &self.paramsType,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                keygenReturn::_tokenize(ret)
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
    #[derive()]
    /**Function with signature `keygenResponse(uint256,(uint8,bytes)[],bytes)` and selector `0x4610ffe8`.
```solidity
function keygenResponse(uint256 keyId, IKMSGeneration.KeyDigest[] memory keyDigests, bytes memory signature) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct keygenResponseCall {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub keyDigests: alloy::sol_types::private::Vec<
            <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`keygenResponse(uint256,(uint8,bytes)[],bytes)`](keygenResponseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct keygenResponseReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IKMSGeneration::KeyDigest>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<keygenResponseCall> for UnderlyingRustTuple<'_> {
                fn from(value: keygenResponseCall) -> Self {
                    (value.keyId, value.keyDigests, value.signature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for keygenResponseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        keyId: tuple.0,
                        keyDigests: tuple.1,
                        signature: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<keygenResponseReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: keygenResponseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for keygenResponseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl keygenResponseReturn {
            fn _tokenize(
                &self,
            ) -> <keygenResponseCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for keygenResponseCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IKMSGeneration::KeyDigest>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = keygenResponseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "keygenResponse(uint256,(uint8,bytes)[],bytes)";
            const SELECTOR: [u8; 4] = [70u8, 16u8, 255u8, 232u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                    <alloy::sol_types::sol_data::Array<
                        IKMSGeneration::KeyDigest,
                    > as alloy_sol_types::SolType>::tokenize(&self.keyDigests),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                keygenResponseReturn::_tokenize(ret)
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
    /**Function with signature `prepKeygenResponse(uint256,bytes)` and selector `0x589adb0e`.
```solidity
function prepKeygenResponse(uint256 prepKeygenId, bytes memory signature) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct prepKeygenResponseCall {
        #[allow(missing_docs)]
        pub prepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`prepKeygenResponse(uint256,bytes)`](prepKeygenResponseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct prepKeygenResponseReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<prepKeygenResponseCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: prepKeygenResponseCall) -> Self {
                    (value.prepKeygenId, value.signature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for prepKeygenResponseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        prepKeygenId: tuple.0,
                        signature: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<prepKeygenResponseReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: prepKeygenResponseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for prepKeygenResponseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl prepKeygenResponseReturn {
            fn _tokenize(
                &self,
            ) -> <prepKeygenResponseCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for prepKeygenResponseCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = prepKeygenResponseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "prepKeygenResponse(uint256,bytes)";
            const SELECTOR: [u8; 4] = [88u8, 154u8, 219u8, 14u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.prepKeygenId),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                prepKeygenResponseReturn::_tokenize(ret)
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
            #[allow(dead_code)]
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
            #[allow(dead_code)]
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
    /**Function with signature `prssInit()` and selector `0x7514a2ac`.
```solidity
function prssInit() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct prssInitCall;
    ///Container type for the return parameters of the [`prssInit()`](prssInitCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct prssInitReturn {}
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<prssInitCall> for UnderlyingRustTuple<'_> {
                fn from(value: prssInitCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for prssInitCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<prssInitReturn> for UnderlyingRustTuple<'_> {
                fn from(value: prssInitReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for prssInitReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl prssInitReturn {
            fn _tokenize(
                &self,
            ) -> <prssInitCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for prssInitCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = prssInitReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "prssInit()";
            const SELECTOR: [u8; 4] = [117u8, 20u8, 162u8, 172u8];
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
                prssInitReturn::_tokenize(ret)
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
            #[allow(dead_code)]
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
            #[allow(dead_code)]
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
            #[allow(dead_code)]
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
            #[allow(dead_code)]
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
    ///Container for all the [`KMSGeneration`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum KMSGenerationCalls {
        #[allow(missing_docs)]
        UPGRADE_INTERFACE_VERSION(UPGRADE_INTERFACE_VERSIONCall),
        #[allow(missing_docs)]
        crsgenRequest(crsgenRequestCall),
        #[allow(missing_docs)]
        crsgenResponse(crsgenResponseCall),
        #[allow(missing_docs)]
        eip712Domain(eip712DomainCall),
        #[allow(missing_docs)]
        getActiveCrsId(getActiveCrsIdCall),
        #[allow(missing_docs)]
        getActiveKeyId(getActiveKeyIdCall),
        #[allow(missing_docs)]
        getConsensusTxSenders(getConsensusTxSendersCall),
        #[allow(missing_docs)]
        getCrsMaterials(getCrsMaterialsCall),
        #[allow(missing_docs)]
        getCrsParamsType(getCrsParamsTypeCall),
        #[allow(missing_docs)]
        getKeyMaterials(getKeyMaterialsCall),
        #[allow(missing_docs)]
        getKeyParamsType(getKeyParamsTypeCall),
        #[allow(missing_docs)]
        getVersion(getVersionCall),
        #[allow(missing_docs)]
        initializeFromEmptyProxy(initializeFromEmptyProxyCall),
        #[allow(missing_docs)]
        keyReshareSameSet(keyReshareSameSetCall),
        #[allow(missing_docs)]
        keygen(keygenCall),
        #[allow(missing_docs)]
        keygenResponse(keygenResponseCall),
        #[allow(missing_docs)]
        prepKeygenResponse(prepKeygenResponseCall),
        #[allow(missing_docs)]
        proxiableUUID(proxiableUUIDCall),
        #[allow(missing_docs)]
        prssInit(prssInitCall),
        #[allow(missing_docs)]
        reinitializeV4(reinitializeV4Call),
        #[allow(missing_docs)]
        upgradeToAndCall(upgradeToAndCallCall),
    }
    impl KMSGenerationCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [13u8, 142u8, 110u8, 44u8],
            [18u8, 58u8, 187u8, 40u8],
            [22u8, 199u8, 19u8, 217u8],
            [25u8, 244u8, 246u8, 50u8],
            [57u8, 247u8, 56u8, 16u8],
            [60u8, 2u8, 248u8, 52u8],
            [69u8, 175u8, 38u8, 27u8],
            [70u8, 16u8, 255u8, 232u8],
            [79u8, 30u8, 242u8, 134u8],
            [82u8, 209u8, 144u8, 45u8],
            [88u8, 154u8, 219u8, 14u8],
            [98u8, 151u8, 135u8, 135u8],
            [117u8, 20u8, 162u8, 172u8],
            [132u8, 176u8, 25u8, 110u8],
            [147u8, 102u8, 8u8, 174u8],
            [173u8, 60u8, 177u8, 204u8],
            [186u8, 255u8, 33u8, 30u8],
            [197u8, 91u8, 135u8, 36u8],
            [202u8, 163u8, 103u8, 219u8],
            [213u8, 47u8, 16u8, 235u8],
            [214u8, 93u8, 131u8, 115u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(getVersion),
            ::core::stringify!(reinitializeV4),
            ::core::stringify!(getConsensusTxSenders),
            ::core::stringify!(getKeyParamsType),
            ::core::stringify!(initializeFromEmptyProxy),
            ::core::stringify!(crsgenRequest),
            ::core::stringify!(getCrsParamsType),
            ::core::stringify!(keygenResponse),
            ::core::stringify!(upgradeToAndCall),
            ::core::stringify!(proxiableUUID),
            ::core::stringify!(prepKeygenResponse),
            ::core::stringify!(crsgenResponse),
            ::core::stringify!(prssInit),
            ::core::stringify!(eip712Domain),
            ::core::stringify!(getKeyMaterials),
            ::core::stringify!(UPGRADE_INTERFACE_VERSION),
            ::core::stringify!(getActiveCrsId),
            ::core::stringify!(getCrsMaterials),
            ::core::stringify!(keygen),
            ::core::stringify!(getActiveKeyId),
            ::core::stringify!(keyReshareSameSet),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <getVersionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <reinitializeV4Call as alloy_sol_types::SolCall>::SIGNATURE,
            <getConsensusTxSendersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getKeyParamsTypeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::SIGNATURE,
            <crsgenRequestCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getCrsParamsTypeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <keygenResponseCall as alloy_sol_types::SolCall>::SIGNATURE,
            <upgradeToAndCallCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proxiableUUIDCall as alloy_sol_types::SolCall>::SIGNATURE,
            <prepKeygenResponseCall as alloy_sol_types::SolCall>::SIGNATURE,
            <crsgenResponseCall as alloy_sol_types::SolCall>::SIGNATURE,
            <prssInitCall as alloy_sol_types::SolCall>::SIGNATURE,
            <eip712DomainCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getKeyMaterialsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getActiveCrsIdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getCrsMaterialsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <keygenCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getActiveKeyIdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <keyReshareSameSetCall as alloy_sol_types::SolCall>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for KMSGenerationCalls {
        const NAME: &'static str = "KMSGenerationCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 21usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::UPGRADE_INTERFACE_VERSION(_) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::crsgenRequest(_) => {
                    <crsgenRequestCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::crsgenResponse(_) => {
                    <crsgenResponseCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eip712Domain(_) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getActiveCrsId(_) => {
                    <getActiveCrsIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getActiveKeyId(_) => {
                    <getActiveKeyIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getConsensusTxSenders(_) => {
                    <getConsensusTxSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCrsMaterials(_) => {
                    <getCrsMaterialsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCrsParamsType(_) => {
                    <getCrsParamsTypeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKeyMaterials(_) => {
                    <getKeyMaterialsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKeyParamsType(_) => {
                    <getKeyParamsTypeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getVersion(_) => {
                    <getVersionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initializeFromEmptyProxy(_) => {
                    <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::keyReshareSameSet(_) => {
                    <keyReshareSameSetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::keygen(_) => <keygenCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::keygenResponse(_) => {
                    <keygenResponseCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::prepKeygenResponse(_) => {
                    <prepKeygenResponseCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proxiableUUID(_) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::prssInit(_) => <prssInitCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::reinitializeV4(_) => {
                    <reinitializeV4Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::upgradeToAndCall(_) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<KMSGenerationCalls>] = &[
                {
                    fn getVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getVersionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::getVersion)
                    }
                    getVersion
                },
                {
                    fn reinitializeV4(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <reinitializeV4Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::reinitializeV4)
                    }
                    reinitializeV4
                },
                {
                    fn getConsensusTxSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getConsensusTxSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::getConsensusTxSenders)
                    }
                    getConsensusTxSenders
                },
                {
                    fn getKeyParamsType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getKeyParamsTypeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::getKeyParamsType)
                    }
                    getKeyParamsType
                },
                {
                    fn initializeFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::initializeFromEmptyProxy)
                    }
                    initializeFromEmptyProxy
                },
                {
                    fn crsgenRequest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <crsgenRequestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::crsgenRequest)
                    }
                    crsgenRequest
                },
                {
                    fn getCrsParamsType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getCrsParamsTypeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::getCrsParamsType)
                    }
                    getCrsParamsType
                },
                {
                    fn keygenResponse(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <keygenResponseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::keygenResponse)
                    }
                    keygenResponse
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn prepKeygenResponse(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <prepKeygenResponseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::prepKeygenResponse)
                    }
                    prepKeygenResponse
                },
                {
                    fn crsgenResponse(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <crsgenResponseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::crsgenResponse)
                    }
                    crsgenResponse
                },
                {
                    fn prssInit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <prssInitCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(KMSGenerationCalls::prssInit)
                    }
                    prssInit
                },
                {
                    fn eip712Domain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <eip712DomainCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::eip712Domain)
                    }
                    eip712Domain
                },
                {
                    fn getKeyMaterials(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getKeyMaterialsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::getKeyMaterials)
                    }
                    getKeyMaterials
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn getActiveCrsId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getActiveCrsIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::getActiveCrsId)
                    }
                    getActiveCrsId
                },
                {
                    fn getCrsMaterials(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getCrsMaterialsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::getCrsMaterials)
                    }
                    getCrsMaterials
                },
                {
                    fn keygen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <keygenCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(KMSGenerationCalls::keygen)
                    }
                    keygen
                },
                {
                    fn getActiveKeyId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getActiveKeyIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::getActiveKeyId)
                    }
                    getActiveKeyId
                },
                {
                    fn keyReshareSameSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <keyReshareSameSetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::keyReshareSameSet)
                    }
                    keyReshareSameSet
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
            ) -> alloy_sol_types::Result<KMSGenerationCalls>] = &[
                {
                    fn getVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getVersionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::getVersion)
                    }
                    getVersion
                },
                {
                    fn reinitializeV4(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <reinitializeV4Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::reinitializeV4)
                    }
                    reinitializeV4
                },
                {
                    fn getConsensusTxSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getConsensusTxSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::getConsensusTxSenders)
                    }
                    getConsensusTxSenders
                },
                {
                    fn getKeyParamsType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getKeyParamsTypeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::getKeyParamsType)
                    }
                    getKeyParamsType
                },
                {
                    fn initializeFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::initializeFromEmptyProxy)
                    }
                    initializeFromEmptyProxy
                },
                {
                    fn crsgenRequest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <crsgenRequestCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::crsgenRequest)
                    }
                    crsgenRequest
                },
                {
                    fn getCrsParamsType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getCrsParamsTypeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::getCrsParamsType)
                    }
                    getCrsParamsType
                },
                {
                    fn keygenResponse(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <keygenResponseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::keygenResponse)
                    }
                    keygenResponse
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn prepKeygenResponse(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <prepKeygenResponseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::prepKeygenResponse)
                    }
                    prepKeygenResponse
                },
                {
                    fn crsgenResponse(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <crsgenResponseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::crsgenResponse)
                    }
                    crsgenResponse
                },
                {
                    fn prssInit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <prssInitCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::prssInit)
                    }
                    prssInit
                },
                {
                    fn eip712Domain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <eip712DomainCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::eip712Domain)
                    }
                    eip712Domain
                },
                {
                    fn getKeyMaterials(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getKeyMaterialsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::getKeyMaterials)
                    }
                    getKeyMaterials
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn getActiveCrsId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getActiveCrsIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::getActiveCrsId)
                    }
                    getActiveCrsId
                },
                {
                    fn getCrsMaterials(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getCrsMaterialsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::getCrsMaterials)
                    }
                    getCrsMaterials
                },
                {
                    fn keygen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <keygenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::keygen)
                    }
                    keygen
                },
                {
                    fn getActiveKeyId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getActiveKeyIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::getActiveKeyId)
                    }
                    getActiveKeyId
                },
                {
                    fn keyReshareSameSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <keyReshareSameSetCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::keyReshareSameSet)
                    }
                    keyReshareSameSet
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
                Self::crsgenRequest(inner) => {
                    <crsgenRequestCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::crsgenResponse(inner) => {
                    <crsgenResponseCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::eip712Domain(inner) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getActiveCrsId(inner) => {
                    <getActiveCrsIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getActiveKeyId(inner) => {
                    <getActiveKeyIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getConsensusTxSenders(inner) => {
                    <getConsensusTxSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCrsMaterials(inner) => {
                    <getCrsMaterialsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCrsParamsType(inner) => {
                    <getCrsParamsTypeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getKeyMaterials(inner) => {
                    <getKeyMaterialsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getKeyParamsType(inner) => {
                    <getKeyParamsTypeCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::keyReshareSameSet(inner) => {
                    <keyReshareSameSetCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::keygen(inner) => {
                    <keygenCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::keygenResponse(inner) => {
                    <keygenResponseCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::prepKeygenResponse(inner) => {
                    <prepKeygenResponseCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proxiableUUID(inner) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::prssInit(inner) => {
                    <prssInitCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::reinitializeV4(inner) => {
                    <reinitializeV4Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::upgradeToAndCall(inner) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::crsgenRequest(inner) => {
                    <crsgenRequestCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::crsgenResponse(inner) => {
                    <crsgenResponseCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getActiveCrsId(inner) => {
                    <getActiveCrsIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getActiveKeyId(inner) => {
                    <getActiveKeyIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getConsensusTxSenders(inner) => {
                    <getConsensusTxSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCrsMaterials(inner) => {
                    <getCrsMaterialsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCrsParamsType(inner) => {
                    <getCrsParamsTypeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getKeyMaterials(inner) => {
                    <getKeyMaterialsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getKeyParamsType(inner) => {
                    <getKeyParamsTypeCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::keyReshareSameSet(inner) => {
                    <keyReshareSameSetCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::keygen(inner) => {
                    <keygenCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::keygenResponse(inner) => {
                    <keygenResponseCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::prepKeygenResponse(inner) => {
                    <prepKeygenResponseCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::prssInit(inner) => {
                    <prssInitCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::upgradeToAndCall(inner) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`KMSGeneration`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum KMSGenerationErrors {
        #[allow(missing_docs)]
        AddressEmptyCode(AddressEmptyCode),
        #[allow(missing_docs)]
        CoprocessorSignerDoesNotMatchTxSender(CoprocessorSignerDoesNotMatchTxSender),
        #[allow(missing_docs)]
        CrsNotGenerated(CrsNotGenerated),
        #[allow(missing_docs)]
        CrsgenNotRequested(CrsgenNotRequested),
        #[allow(missing_docs)]
        CrsgenOngoing(CrsgenOngoing),
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
        EmptyKeyDigests(EmptyKeyDigests),
        #[allow(missing_docs)]
        FailedCall(FailedCall),
        #[allow(missing_docs)]
        HostChainNotRegistered(HostChainNotRegistered),
        #[allow(missing_docs)]
        InvalidInitialization(InvalidInitialization),
        #[allow(missing_docs)]
        KeyNotGenerated(KeyNotGenerated),
        #[allow(missing_docs)]
        KeygenNotRequested(KeygenNotRequested),
        #[allow(missing_docs)]
        KeygenOngoing(KeygenOngoing),
        #[allow(missing_docs)]
        KmsAlreadySignedForCrsgen(KmsAlreadySignedForCrsgen),
        #[allow(missing_docs)]
        KmsAlreadySignedForKeygen(KmsAlreadySignedForKeygen),
        #[allow(missing_docs)]
        KmsAlreadySignedForPrepKeygen(KmsAlreadySignedForPrepKeygen),
        #[allow(missing_docs)]
        KmsSignerDoesNotMatchTxSender(KmsSignerDoesNotMatchTxSender),
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
        PrepKeygenNotRequested(PrepKeygenNotRequested),
        #[allow(missing_docs)]
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        #[allow(missing_docs)]
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
    }
    impl KMSGenerationErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [6u8, 26u8, 198u8, 29u8],
            [10u8, 183u8, 246u8, 135u8],
            [13u8, 134u8, 245u8, 33u8],
            [14u8, 86u8, 207u8, 61u8],
            [38u8, 205u8, 117u8, 220u8],
            [42u8, 124u8, 110u8, 246u8],
            [51u8, 202u8, 31u8, 227u8],
            [57u8, 22u8, 114u8, 167u8],
            [59u8, 133u8, 61u8, 168u8],
            [76u8, 156u8, 140u8, 227u8],
            [82u8, 215u8, 37u8, 245u8],
            [111u8, 79u8, 115u8, 31u8],
            [132u8, 222u8, 19u8, 49u8],
            [141u8, 140u8, 148u8, 10u8],
            [152u8, 251u8, 149u8, 125u8],
            [153u8, 150u8, 179u8, 21u8],
            [170u8, 29u8, 73u8, 164u8],
            [173u8, 250u8, 185u8, 4u8],
            [174u8, 232u8, 99u8, 35u8],
            [179u8, 152u8, 151u8, 159u8],
            [182u8, 103u8, 156u8, 59u8],
            [214u8, 189u8, 162u8, 117u8],
            [215u8, 139u8, 206u8, 12u8],
            [215u8, 230u8, 188u8, 248u8],
            [218u8, 50u8, 208u8, 15u8],
            [224u8, 124u8, 141u8, 186u8],
            [225u8, 52u8, 191u8, 98u8],
            [230u8, 249u8, 8u8, 59u8],
            [246u8, 69u8, 238u8, 223u8],
            [249u8, 36u8, 160u8, 207u8],
            [249u8, 46u8, 232u8, 169u8],
            [252u8, 230u8, 152u8, 247u8],
            [252u8, 245u8, 166u8, 233u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(CrsgenOngoing),
            ::core::stringify!(PrepKeygenNotRequested),
            ::core::stringify!(KmsSignerDoesNotMatchTxSender),
            ::core::stringify!(NotGatewayOwner),
            ::core::stringify!(NotCoprocessorSigner),
            ::core::stringify!(NotKmsSigner),
            ::core::stringify!(KmsAlreadySignedForPrepKeygen),
            ::core::stringify!(NotCustodianSigner),
            ::core::stringify!(KeygenOngoing),
            ::core::stringify!(ERC1967InvalidImplementation),
            ::core::stringify!(NotCoprocessorTxSender),
            ::core::stringify!(NotInitializingFromEmptyProxy),
            ::core::stringify!(KeyNotGenerated),
            ::core::stringify!(CrsgenNotRequested),
            ::core::stringify!(KmsAlreadySignedForKeygen),
            ::core::stringify!(AddressEmptyCode),
            ::core::stringify!(UUPSUnsupportedProxiableUUID),
            ::core::stringify!(KeygenNotRequested),
            ::core::stringify!(NotKmsTxSender),
            ::core::stringify!(ERC1967NonPayable),
            ::core::stringify!(HostChainNotRegistered),
            ::core::stringify!(FailedCall),
            ::core::stringify!(ECDSAInvalidSignatureS),
            ::core::stringify!(NotInitializing),
            ::core::stringify!(CrsNotGenerated),
            ::core::stringify!(UUPSUnauthorizedCallContext),
            ::core::stringify!(CoprocessorSignerDoesNotMatchTxSender),
            ::core::stringify!(EmptyKeyDigests),
            ::core::stringify!(ECDSAInvalidSignature),
            ::core::stringify!(NotCustodianTxSender),
            ::core::stringify!(InvalidInitialization),
            ::core::stringify!(ECDSAInvalidSignatureLength),
            ::core::stringify!(KmsAlreadySignedForCrsgen),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <CrsgenOngoing as alloy_sol_types::SolError>::SIGNATURE,
            <PrepKeygenNotRequested as alloy_sol_types::SolError>::SIGNATURE,
            <KmsSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::SIGNATURE,
            <NotGatewayOwner as alloy_sol_types::SolError>::SIGNATURE,
            <NotCoprocessorSigner as alloy_sol_types::SolError>::SIGNATURE,
            <NotKmsSigner as alloy_sol_types::SolError>::SIGNATURE,
            <KmsAlreadySignedForPrepKeygen as alloy_sol_types::SolError>::SIGNATURE,
            <NotCustodianSigner as alloy_sol_types::SolError>::SIGNATURE,
            <KeygenOngoing as alloy_sol_types::SolError>::SIGNATURE,
            <ERC1967InvalidImplementation as alloy_sol_types::SolError>::SIGNATURE,
            <NotCoprocessorTxSender as alloy_sol_types::SolError>::SIGNATURE,
            <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::SIGNATURE,
            <KeyNotGenerated as alloy_sol_types::SolError>::SIGNATURE,
            <CrsgenNotRequested as alloy_sol_types::SolError>::SIGNATURE,
            <KmsAlreadySignedForKeygen as alloy_sol_types::SolError>::SIGNATURE,
            <AddressEmptyCode as alloy_sol_types::SolError>::SIGNATURE,
            <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::SIGNATURE,
            <KeygenNotRequested as alloy_sol_types::SolError>::SIGNATURE,
            <NotKmsTxSender as alloy_sol_types::SolError>::SIGNATURE,
            <ERC1967NonPayable as alloy_sol_types::SolError>::SIGNATURE,
            <HostChainNotRegistered as alloy_sol_types::SolError>::SIGNATURE,
            <FailedCall as alloy_sol_types::SolError>::SIGNATURE,
            <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::SIGNATURE,
            <NotInitializing as alloy_sol_types::SolError>::SIGNATURE,
            <CrsNotGenerated as alloy_sol_types::SolError>::SIGNATURE,
            <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::SIGNATURE,
            <CoprocessorSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::SIGNATURE,
            <EmptyKeyDigests as alloy_sol_types::SolError>::SIGNATURE,
            <ECDSAInvalidSignature as alloy_sol_types::SolError>::SIGNATURE,
            <NotCustodianTxSender as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidInitialization as alloy_sol_types::SolError>::SIGNATURE,
            <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::SIGNATURE,
            <KmsAlreadySignedForCrsgen as alloy_sol_types::SolError>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for KMSGenerationErrors {
        const NAME: &'static str = "KMSGenerationErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 33usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AddressEmptyCode(_) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CoprocessorSignerDoesNotMatchTxSender(_) => {
                    <CoprocessorSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CrsNotGenerated(_) => {
                    <CrsNotGenerated as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CrsgenNotRequested(_) => {
                    <CrsgenNotRequested as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CrsgenOngoing(_) => {
                    <CrsgenOngoing as alloy_sol_types::SolError>::SELECTOR
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
                Self::EmptyKeyDigests(_) => {
                    <EmptyKeyDigests as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FailedCall(_) => {
                    <FailedCall as alloy_sol_types::SolError>::SELECTOR
                }
                Self::HostChainNotRegistered(_) => {
                    <HostChainNotRegistered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidInitialization(_) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KeyNotGenerated(_) => {
                    <KeyNotGenerated as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KeygenNotRequested(_) => {
                    <KeygenNotRequested as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KeygenOngoing(_) => {
                    <KeygenOngoing as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsAlreadySignedForCrsgen(_) => {
                    <KmsAlreadySignedForCrsgen as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsAlreadySignedForKeygen(_) => {
                    <KmsAlreadySignedForKeygen as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsAlreadySignedForPrepKeygen(_) => {
                    <KmsAlreadySignedForPrepKeygen as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsSignerDoesNotMatchTxSender(_) => {
                    <KmsSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::SELECTOR
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
                Self::PrepKeygenNotRequested(_) => {
                    <PrepKeygenNotRequested as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UUPSUnauthorizedCallContext(_) => {
                    <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UUPSUnsupportedProxiableUUID(_) => {
                    <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<KMSGenerationErrors>] = &[
                {
                    fn CrsgenOngoing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <CrsgenOngoing as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::CrsgenOngoing)
                    }
                    CrsgenOngoing
                },
                {
                    fn PrepKeygenNotRequested(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <PrepKeygenNotRequested as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::PrepKeygenNotRequested)
                    }
                    PrepKeygenNotRequested
                },
                {
                    fn KmsSignerDoesNotMatchTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KmsSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::KmsSignerDoesNotMatchTxSender)
                    }
                    KmsSignerDoesNotMatchTxSender
                },
                {
                    fn NotGatewayOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotGatewayOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::NotGatewayOwner)
                    }
                    NotGatewayOwner
                },
                {
                    fn NotCoprocessorSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotCoprocessorSigner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::NotCoprocessorSigner)
                    }
                    NotCoprocessorSigner
                },
                {
                    fn NotKmsSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotKmsSigner as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(KMSGenerationErrors::NotKmsSigner)
                    }
                    NotKmsSigner
                },
                {
                    fn KmsAlreadySignedForPrepKeygen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KmsAlreadySignedForPrepKeygen as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::KmsAlreadySignedForPrepKeygen)
                    }
                    KmsAlreadySignedForPrepKeygen
                },
                {
                    fn NotCustodianSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotCustodianSigner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::NotCustodianSigner)
                    }
                    NotCustodianSigner
                },
                {
                    fn KeygenOngoing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KeygenOngoing as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::KeygenOngoing)
                    }
                    KeygenOngoing
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn NotCoprocessorTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotCoprocessorTxSender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::NotCoprocessorTxSender)
                    }
                    NotCoprocessorTxSender
                },
                {
                    fn NotInitializingFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::NotInitializingFromEmptyProxy)
                    }
                    NotInitializingFromEmptyProxy
                },
                {
                    fn KeyNotGenerated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KeyNotGenerated as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::KeyNotGenerated)
                    }
                    KeyNotGenerated
                },
                {
                    fn CrsgenNotRequested(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <CrsgenNotRequested as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::CrsgenNotRequested)
                    }
                    CrsgenNotRequested
                },
                {
                    fn KmsAlreadySignedForKeygen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KmsAlreadySignedForKeygen as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::KmsAlreadySignedForKeygen)
                    }
                    KmsAlreadySignedForKeygen
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn KeygenNotRequested(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KeygenNotRequested as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::KeygenNotRequested)
                    }
                    KeygenNotRequested
                },
                {
                    fn NotKmsTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotKmsTxSender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::NotKmsTxSender)
                    }
                    NotKmsTxSender
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn HostChainNotRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <HostChainNotRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::HostChainNotRegistered)
                    }
                    HostChainNotRegistered
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(KMSGenerationErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn CrsNotGenerated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <CrsNotGenerated as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::CrsNotGenerated)
                    }
                    CrsNotGenerated
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn CoprocessorSignerDoesNotMatchTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <CoprocessorSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                KMSGenerationErrors::CoprocessorSignerDoesNotMatchTxSender,
                            )
                    }
                    CoprocessorSignerDoesNotMatchTxSender
                },
                {
                    fn EmptyKeyDigests(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <EmptyKeyDigests as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::EmptyKeyDigests)
                    }
                    EmptyKeyDigests
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn NotCustodianTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotCustodianTxSender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::NotCustodianTxSender)
                    }
                    NotCustodianTxSender
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::InvalidInitialization)
                    }
                    InvalidInitialization
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::ECDSAInvalidSignatureLength)
                    }
                    ECDSAInvalidSignatureLength
                },
                {
                    fn KmsAlreadySignedForCrsgen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KmsAlreadySignedForCrsgen as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::KmsAlreadySignedForCrsgen)
                    }
                    KmsAlreadySignedForCrsgen
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
            ) -> alloy_sol_types::Result<KMSGenerationErrors>] = &[
                {
                    fn CrsgenOngoing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <CrsgenOngoing as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::CrsgenOngoing)
                    }
                    CrsgenOngoing
                },
                {
                    fn PrepKeygenNotRequested(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <PrepKeygenNotRequested as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::PrepKeygenNotRequested)
                    }
                    PrepKeygenNotRequested
                },
                {
                    fn KmsSignerDoesNotMatchTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KmsSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::KmsSignerDoesNotMatchTxSender)
                    }
                    KmsSignerDoesNotMatchTxSender
                },
                {
                    fn NotGatewayOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotGatewayOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::NotGatewayOwner)
                    }
                    NotGatewayOwner
                },
                {
                    fn NotCoprocessorSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotCoprocessorSigner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::NotCoprocessorSigner)
                    }
                    NotCoprocessorSigner
                },
                {
                    fn NotKmsSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotKmsSigner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::NotKmsSigner)
                    }
                    NotKmsSigner
                },
                {
                    fn KmsAlreadySignedForPrepKeygen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KmsAlreadySignedForPrepKeygen as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::KmsAlreadySignedForPrepKeygen)
                    }
                    KmsAlreadySignedForPrepKeygen
                },
                {
                    fn NotCustodianSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotCustodianSigner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::NotCustodianSigner)
                    }
                    NotCustodianSigner
                },
                {
                    fn KeygenOngoing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KeygenOngoing as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::KeygenOngoing)
                    }
                    KeygenOngoing
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn NotCoprocessorTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotCoprocessorTxSender as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::NotCoprocessorTxSender)
                    }
                    NotCoprocessorTxSender
                },
                {
                    fn NotInitializingFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::NotInitializingFromEmptyProxy)
                    }
                    NotInitializingFromEmptyProxy
                },
                {
                    fn KeyNotGenerated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KeyNotGenerated as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::KeyNotGenerated)
                    }
                    KeyNotGenerated
                },
                {
                    fn CrsgenNotRequested(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <CrsgenNotRequested as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::CrsgenNotRequested)
                    }
                    CrsgenNotRequested
                },
                {
                    fn KmsAlreadySignedForKeygen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KmsAlreadySignedForKeygen as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::KmsAlreadySignedForKeygen)
                    }
                    KmsAlreadySignedForKeygen
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn KeygenNotRequested(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KeygenNotRequested as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::KeygenNotRequested)
                    }
                    KeygenNotRequested
                },
                {
                    fn NotKmsTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotKmsTxSender as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::NotKmsTxSender)
                    }
                    NotKmsTxSender
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn HostChainNotRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <HostChainNotRegistered as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::HostChainNotRegistered)
                    }
                    HostChainNotRegistered
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn CrsNotGenerated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <CrsNotGenerated as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::CrsNotGenerated)
                    }
                    CrsNotGenerated
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn CoprocessorSignerDoesNotMatchTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <CoprocessorSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                KMSGenerationErrors::CoprocessorSignerDoesNotMatchTxSender,
                            )
                    }
                    CoprocessorSignerDoesNotMatchTxSender
                },
                {
                    fn EmptyKeyDigests(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <EmptyKeyDigests as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::EmptyKeyDigests)
                    }
                    EmptyKeyDigests
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn NotCustodianTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotCustodianTxSender as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::NotCustodianTxSender)
                    }
                    NotCustodianTxSender
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::InvalidInitialization)
                    }
                    InvalidInitialization
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::ECDSAInvalidSignatureLength)
                    }
                    ECDSAInvalidSignatureLength
                },
                {
                    fn KmsAlreadySignedForCrsgen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KmsAlreadySignedForCrsgen as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::KmsAlreadySignedForCrsgen)
                    }
                    KmsAlreadySignedForCrsgen
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
                Self::CoprocessorSignerDoesNotMatchTxSender(inner) => {
                    <CoprocessorSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CrsNotGenerated(inner) => {
                    <CrsNotGenerated as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CrsgenNotRequested(inner) => {
                    <CrsgenNotRequested as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CrsgenOngoing(inner) => {
                    <CrsgenOngoing as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::EmptyKeyDigests(inner) => {
                    <EmptyKeyDigests as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::HostChainNotRegistered(inner) => {
                    <HostChainNotRegistered as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidInitialization(inner) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KeyNotGenerated(inner) => {
                    <KeyNotGenerated as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KeygenNotRequested(inner) => {
                    <KeygenNotRequested as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KeygenOngoing(inner) => {
                    <KeygenOngoing as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::KmsAlreadySignedForCrsgen(inner) => {
                    <KmsAlreadySignedForCrsgen as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KmsAlreadySignedForKeygen(inner) => {
                    <KmsAlreadySignedForKeygen as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KmsAlreadySignedForPrepKeygen(inner) => {
                    <KmsAlreadySignedForPrepKeygen as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KmsSignerDoesNotMatchTxSender(inner) => {
                    <KmsSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::PrepKeygenNotRequested(inner) => {
                    <PrepKeygenNotRequested as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::CoprocessorSignerDoesNotMatchTxSender(inner) => {
                    <CoprocessorSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CrsNotGenerated(inner) => {
                    <CrsNotGenerated as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CrsgenNotRequested(inner) => {
                    <CrsgenNotRequested as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CrsgenOngoing(inner) => {
                    <CrsgenOngoing as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::EmptyKeyDigests(inner) => {
                    <EmptyKeyDigests as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidInitialization(inner) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KeyNotGenerated(inner) => {
                    <KeyNotGenerated as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KeygenNotRequested(inner) => {
                    <KeygenNotRequested as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KeygenOngoing(inner) => {
                    <KeygenOngoing as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KmsAlreadySignedForCrsgen(inner) => {
                    <KmsAlreadySignedForCrsgen as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KmsAlreadySignedForKeygen(inner) => {
                    <KmsAlreadySignedForKeygen as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KmsAlreadySignedForPrepKeygen(inner) => {
                    <KmsAlreadySignedForPrepKeygen as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::PrepKeygenNotRequested(inner) => {
                    <PrepKeygenNotRequested as alloy_sol_types::SolError>::abi_encode_raw(
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
            }
        }
    }
    ///Container for all the [`KMSGeneration`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum KMSGenerationEvents {
        #[allow(missing_docs)]
        ActivateCrs(ActivateCrs),
        #[allow(missing_docs)]
        ActivateKey(ActivateKey),
        #[allow(missing_docs)]
        CrsgenRequest(CrsgenRequest),
        #[allow(missing_docs)]
        CrsgenResponse(CrsgenResponse),
        #[allow(missing_docs)]
        EIP712DomainChanged(EIP712DomainChanged),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        KeyReshareSameSet(KeyReshareSameSet),
        #[allow(missing_docs)]
        KeygenRequest(KeygenRequest),
        #[allow(missing_docs)]
        KeygenResponse(KeygenResponse),
        #[allow(missing_docs)]
        PRSSInit(PRSSInit),
        #[allow(missing_docs)]
        PrepKeygenRequest(PrepKeygenRequest),
        #[allow(missing_docs)]
        PrepKeygenResponse(PrepKeygenResponse),
        #[allow(missing_docs)]
        Upgraded(Upgraded),
    }
    impl KMSGenerationEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                2u8, 2u8, 64u8, 7u8, 217u8, 101u8, 116u8, 219u8, 201u8, 209u8, 19u8,
                40u8, 191u8, 238u8, 152u8, 147u8, 231u8, 199u8, 187u8, 78u8, 244u8,
                170u8, 128u8, 109u8, 243u8, 59u8, 253u8, 244u8, 84u8, 235u8, 94u8, 96u8,
            ],
            [
                10u8, 99u8, 135u8, 201u8, 234u8, 54u8, 40u8, 184u8, 138u8, 99u8, 59u8,
                180u8, 243u8, 177u8, 81u8, 119u8, 15u8, 112u8, 8u8, 81u8, 23u8, 161u8,
                95u8, 155u8, 243u8, 120u8, 124u8, 218u8, 83u8, 241u8, 61u8, 49u8,
            ],
            [
                17u8, 219u8, 66u8, 193u8, 135u8, 143u8, 46u8, 40u8, 25u8, 36u8, 31u8,
                82u8, 80u8, 152u8, 69u8, 99u8, 240u8, 108u8, 242u8, 40u8, 24u8, 231u8,
                173u8, 184u8, 106u8, 102u8, 146u8, 29u8, 21u8, 213u8, 157u8, 63u8,
            ],
            [
                28u8, 203u8, 85u8, 69u8, 196u8, 200u8, 219u8, 80u8, 160u8, 245u8, 180u8,
                22u8, 73u8, 149u8, 38u8, 146u8, 159u8, 104u8, 83u8, 78u8, 212u8, 127u8,
                108u8, 253u8, 76u8, 159u8, 6u8, 144u8, 117u8, 230u8, 11u8, 69u8,
            ],
            [
                34u8, 88u8, 183u8, 63u8, 174u8, 211u8, 63u8, 178u8, 226u8, 234u8, 69u8,
                68u8, 3u8, 190u8, 249u8, 116u8, 146u8, 12u8, 175u8, 104u8, 42u8, 179u8,
                167u8, 35u8, 72u8, 79u8, 207u8, 103u8, 85u8, 59u8, 22u8, 162u8,
            ],
            [
                42u8, 254u8, 100u8, 251u8, 58u8, 253u8, 232u8, 226u8, 103u8, 138u8,
                234u8, 132u8, 207u8, 54u8, 34u8, 63u8, 51u8, 14u8, 47u8, 177u8, 40u8,
                109u8, 55u8, 174u8, 213u8, 115u8, 171u8, 156u8, 209u8, 219u8, 71u8, 199u8,
            ],
            [
                63u8, 3u8, 143u8, 111u8, 136u8, 203u8, 48u8, 49u8, 183u8, 113u8, 133u8,
                136u8, 64u8, 58u8, 46u8, 194u8, 32u8, 87u8, 106u8, 134u8, 139u8, 224u8,
                125u8, 222u8, 76u8, 2u8, 184u8, 70u8, 202u8, 53u8, 46u8, 245u8,
            ],
            [
                76u8, 113u8, 92u8, 87u8, 52u8, 206u8, 92u8, 24u8, 201u8, 193u8, 46u8,
                132u8, 150u8, 229u8, 61u8, 42u8, 101u8, 241u8, 236u8, 56u8, 29u8, 71u8,
                105u8, 87u8, 240u8, 245u8, 150u8, 179u8, 100u8, 165u8, 155u8, 12u8,
            ],
            [
                120u8, 177u8, 121u8, 23u8, 109u8, 31u8, 25u8, 215u8, 194u8, 142u8, 128u8,
                130u8, 61u8, 235u8, 162u8, 98u8, 77u8, 162u8, 202u8, 46u8, 198u8, 75u8,
                23u8, 1u8, 243u8, 99u8, 42u8, 135u8, 201u8, 174u8, 220u8, 146u8,
            ],
            [
                123u8, 241u8, 180u8, 44u8, 16u8, 233u8, 73u8, 124u8, 135u8, 150u8, 32u8,
                197u8, 183u8, 175u8, 206u8, 209u8, 11u8, 218u8, 23u8, 216u8, 201u8, 11u8,
                34u8, 240u8, 227u8, 188u8, 107u8, 47u8, 214u8, 206u8, 208u8, 189u8,
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
                235u8, 133u8, 194u8, 109u8, 188u8, 173u8, 70u8, 184u8, 10u8, 104u8,
                160u8, 242u8, 76u8, 206u8, 124u8, 44u8, 144u8, 240u8, 161u8, 250u8,
                222u8, 216u8, 65u8, 132u8, 19u8, 136u8, 57u8, 252u8, 158u8, 128u8, 162u8,
                91u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(PrepKeygenRequest),
            ::core::stringify!(EIP712DomainChanged),
            ::core::stringify!(PRSSInit),
            ::core::stringify!(KeyReshareSameSet),
            ::core::stringify!(ActivateCrs),
            ::core::stringify!(KeygenResponse),
            ::core::stringify!(CrsgenRequest),
            ::core::stringify!(PrepKeygenResponse),
            ::core::stringify!(KeygenRequest),
            ::core::stringify!(CrsgenResponse),
            ::core::stringify!(Upgraded),
            ::core::stringify!(Initialized),
            ::core::stringify!(ActivateKey),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <PrepKeygenRequest as alloy_sol_types::SolEvent>::SIGNATURE,
            <EIP712DomainChanged as alloy_sol_types::SolEvent>::SIGNATURE,
            <PRSSInit as alloy_sol_types::SolEvent>::SIGNATURE,
            <KeyReshareSameSet as alloy_sol_types::SolEvent>::SIGNATURE,
            <ActivateCrs as alloy_sol_types::SolEvent>::SIGNATURE,
            <KeygenResponse as alloy_sol_types::SolEvent>::SIGNATURE,
            <CrsgenRequest as alloy_sol_types::SolEvent>::SIGNATURE,
            <PrepKeygenResponse as alloy_sol_types::SolEvent>::SIGNATURE,
            <KeygenRequest as alloy_sol_types::SolEvent>::SIGNATURE,
            <CrsgenResponse as alloy_sol_types::SolEvent>::SIGNATURE,
            <Upgraded as alloy_sol_types::SolEvent>::SIGNATURE,
            <Initialized as alloy_sol_types::SolEvent>::SIGNATURE,
            <ActivateKey as alloy_sol_types::SolEvent>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 32usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 32usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for KMSGenerationEvents {
        const NAME: &'static str = "KMSGenerationEvents";
        const COUNT: usize = 13usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<ActivateCrs as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ActivateCrs as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ActivateCrs)
                }
                Some(<ActivateKey as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ActivateKey as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ActivateKey)
                }
                Some(<CrsgenRequest as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <CrsgenRequest as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::CrsgenRequest)
                }
                Some(<CrsgenResponse as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <CrsgenResponse as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::CrsgenResponse)
                }
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
                    <KeyReshareSameSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <KeyReshareSameSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::KeyReshareSameSet)
                }
                Some(<KeygenRequest as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <KeygenRequest as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::KeygenRequest)
                }
                Some(<KeygenResponse as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <KeygenResponse as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::KeygenResponse)
                }
                Some(<PRSSInit as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <PRSSInit as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::PRSSInit)
                }
                Some(
                    <PrepKeygenRequest as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PrepKeygenRequest as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PrepKeygenRequest)
                }
                Some(
                    <PrepKeygenResponse as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PrepKeygenResponse as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PrepKeygenResponse)
                }
                Some(<Upgraded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Upgraded as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Upgraded)
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
    impl alloy_sol_types::private::IntoLogData for KMSGenerationEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ActivateCrs(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ActivateKey(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::CrsgenRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::CrsgenResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EIP712DomainChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::KeyReshareSameSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::KeygenRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::KeygenResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PRSSInit(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PrepKeygenRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PrepKeygenResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ActivateCrs(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ActivateKey(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::CrsgenRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::CrsgenResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EIP712DomainChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::KeyReshareSameSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::KeygenRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::KeygenResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PRSSInit(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PrepKeygenRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PrepKeygenResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`KMSGeneration`](self) contract instance.

See the [wrapper's documentation](`KMSGenerationInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> KMSGenerationInstance<P, N> {
        KMSGenerationInstance::<P, N>::new(address, __provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        __provider: P,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<KMSGenerationInstance<P, N>>,
    > {
        KMSGenerationInstance::<P, N>::deploy(__provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(__provider: P) -> alloy_contract::RawCallBuilder<P, N> {
        KMSGenerationInstance::<P, N>::deploy_builder(__provider)
    }
    /**A [`KMSGeneration`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`KMSGeneration`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct KMSGenerationInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for KMSGenerationInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("KMSGenerationInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > KMSGenerationInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`KMSGeneration`](self) contract instance.

See the [wrapper's documentation](`KMSGenerationInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            __provider: P,
        ) -> alloy_contract::Result<KMSGenerationInstance<P, N>> {
            let call_builder = Self::deploy_builder(__provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(__provider: P) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
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
    impl<P: ::core::clone::Clone, N> KMSGenerationInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> KMSGenerationInstance<P, N> {
            KMSGenerationInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > KMSGenerationInstance<P, N> {
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
        ///Creates a new call builder for the [`crsgenRequest`] function.
        pub fn crsgenRequest(
            &self,
            maxBitLength: alloy::sol_types::private::primitives::aliases::U256,
            paramsType: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, crsgenRequestCall, N> {
            self.call_builder(
                &crsgenRequestCall {
                    maxBitLength,
                    paramsType,
                },
            )
        }
        ///Creates a new call builder for the [`crsgenResponse`] function.
        pub fn crsgenResponse(
            &self,
            crsId: alloy::sol_types::private::primitives::aliases::U256,
            crsDigest: alloy::sol_types::private::Bytes,
            signature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, crsgenResponseCall, N> {
            self.call_builder(
                &crsgenResponseCall {
                    crsId,
                    crsDigest,
                    signature,
                },
            )
        }
        ///Creates a new call builder for the [`eip712Domain`] function.
        pub fn eip712Domain(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, eip712DomainCall, N> {
            self.call_builder(&eip712DomainCall)
        }
        ///Creates a new call builder for the [`getActiveCrsId`] function.
        pub fn getActiveCrsId(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getActiveCrsIdCall, N> {
            self.call_builder(&getActiveCrsIdCall)
        }
        ///Creates a new call builder for the [`getActiveKeyId`] function.
        pub fn getActiveKeyId(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getActiveKeyIdCall, N> {
            self.call_builder(&getActiveKeyIdCall)
        }
        ///Creates a new call builder for the [`getConsensusTxSenders`] function.
        pub fn getConsensusTxSenders(
            &self,
            requestId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getConsensusTxSendersCall, N> {
            self.call_builder(
                &getConsensusTxSendersCall {
                    requestId,
                },
            )
        }
        ///Creates a new call builder for the [`getCrsMaterials`] function.
        pub fn getCrsMaterials(
            &self,
            crsId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getCrsMaterialsCall, N> {
            self.call_builder(&getCrsMaterialsCall { crsId })
        }
        ///Creates a new call builder for the [`getCrsParamsType`] function.
        pub fn getCrsParamsType(
            &self,
            crsId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getCrsParamsTypeCall, N> {
            self.call_builder(&getCrsParamsTypeCall { crsId })
        }
        ///Creates a new call builder for the [`getKeyMaterials`] function.
        pub fn getKeyMaterials(
            &self,
            keyId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getKeyMaterialsCall, N> {
            self.call_builder(&getKeyMaterialsCall { keyId })
        }
        ///Creates a new call builder for the [`getKeyParamsType`] function.
        pub fn getKeyParamsType(
            &self,
            keyId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getKeyParamsTypeCall, N> {
            self.call_builder(&getKeyParamsTypeCall { keyId })
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
        ///Creates a new call builder for the [`keyReshareSameSet`] function.
        pub fn keyReshareSameSet(
            &self,
            keyId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, keyReshareSameSetCall, N> {
            self.call_builder(&keyReshareSameSetCall { keyId })
        }
        ///Creates a new call builder for the [`keygen`] function.
        pub fn keygen(
            &self,
            paramsType: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, keygenCall, N> {
            self.call_builder(&keygenCall { paramsType })
        }
        ///Creates a new call builder for the [`keygenResponse`] function.
        pub fn keygenResponse(
            &self,
            keyId: alloy::sol_types::private::primitives::aliases::U256,
            keyDigests: alloy::sol_types::private::Vec<
                <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
            >,
            signature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, keygenResponseCall, N> {
            self.call_builder(
                &keygenResponseCall {
                    keyId,
                    keyDigests,
                    signature,
                },
            )
        }
        ///Creates a new call builder for the [`prepKeygenResponse`] function.
        pub fn prepKeygenResponse(
            &self,
            prepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
            signature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, prepKeygenResponseCall, N> {
            self.call_builder(
                &prepKeygenResponseCall {
                    prepKeygenId,
                    signature,
                },
            )
        }
        ///Creates a new call builder for the [`proxiableUUID`] function.
        pub fn proxiableUUID(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, proxiableUUIDCall, N> {
            self.call_builder(&proxiableUUIDCall)
        }
        ///Creates a new call builder for the [`prssInit`] function.
        pub fn prssInit(&self) -> alloy_contract::SolCallBuilder<&P, prssInitCall, N> {
            self.call_builder(&prssInitCall)
        }
        ///Creates a new call builder for the [`reinitializeV4`] function.
        pub fn reinitializeV4(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, reinitializeV4Call, N> {
            self.call_builder(&reinitializeV4Call)
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
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > KMSGenerationInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`ActivateCrs`] event.
        pub fn ActivateCrs_filter(&self) -> alloy_contract::Event<&P, ActivateCrs, N> {
            self.event_filter::<ActivateCrs>()
        }
        ///Creates a new event filter for the [`ActivateKey`] event.
        pub fn ActivateKey_filter(&self) -> alloy_contract::Event<&P, ActivateKey, N> {
            self.event_filter::<ActivateKey>()
        }
        ///Creates a new event filter for the [`CrsgenRequest`] event.
        pub fn CrsgenRequest_filter(
            &self,
        ) -> alloy_contract::Event<&P, CrsgenRequest, N> {
            self.event_filter::<CrsgenRequest>()
        }
        ///Creates a new event filter for the [`CrsgenResponse`] event.
        pub fn CrsgenResponse_filter(
            &self,
        ) -> alloy_contract::Event<&P, CrsgenResponse, N> {
            self.event_filter::<CrsgenResponse>()
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
        ///Creates a new event filter for the [`KeyReshareSameSet`] event.
        pub fn KeyReshareSameSet_filter(
            &self,
        ) -> alloy_contract::Event<&P, KeyReshareSameSet, N> {
            self.event_filter::<KeyReshareSameSet>()
        }
        ///Creates a new event filter for the [`KeygenRequest`] event.
        pub fn KeygenRequest_filter(
            &self,
        ) -> alloy_contract::Event<&P, KeygenRequest, N> {
            self.event_filter::<KeygenRequest>()
        }
        ///Creates a new event filter for the [`KeygenResponse`] event.
        pub fn KeygenResponse_filter(
            &self,
        ) -> alloy_contract::Event<&P, KeygenResponse, N> {
            self.event_filter::<KeygenResponse>()
        }
        ///Creates a new event filter for the [`PRSSInit`] event.
        pub fn PRSSInit_filter(&self) -> alloy_contract::Event<&P, PRSSInit, N> {
            self.event_filter::<PRSSInit>()
        }
        ///Creates a new event filter for the [`PrepKeygenRequest`] event.
        pub fn PrepKeygenRequest_filter(
            &self,
        ) -> alloy_contract::Event<&P, PrepKeygenRequest, N> {
            self.event_filter::<PrepKeygenRequest>()
        }
        ///Creates a new event filter for the [`PrepKeygenResponse`] event.
        pub fn PrepKeygenResponse_filter(
            &self,
        ) -> alloy_contract::Event<&P, PrepKeygenResponse, N> {
            self.event_filter::<PrepKeygenResponse>()
        }
        ///Creates a new event filter for the [`Upgraded`] event.
        pub fn Upgraded_filter(&self) -> alloy_contract::Event<&P, Upgraded, N> {
            self.event_filter::<Upgraded>()
        }
    }
}
