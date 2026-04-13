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
        #[automatically_derived]
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
        #[automatically_derived]
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
        provider: P,
    ) -> IKMSGenerationInstance<P, N> {
        IKMSGenerationInstance::<P, N>::new(address, provider)
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
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IKMSGenerationInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`IKMSGeneration`](self) contract instance.

See the [wrapper's documentation](`IKMSGenerationInstance`) for more details.*/
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
    #[automatically_derived]
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
    #[automatically_derived]
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
    struct MigrationState {
        uint256 prepKeygenCounter;
        uint256 keyCounter;
        uint256 crsCounter;
        uint256 activeKeyId;
        uint256 activeCrsId;
        uint256 activePrepKeygenId;
        IKMSGeneration.KeyDigest[] activeKeyDigests;
        bytes activeCrsDigest;
        address[] keyConsensusTxSenders;
        bytes32 keyConsensusDigest;
        address[] crsConsensusTxSenders;
        bytes32 crsConsensusDigest;
        address[] prepKeygenConsensusTxSenders;
        bytes32 prepKeygenConsensusDigest;
        uint256 crsMaxBitLength;
        IKMSGeneration.ParamsType prepKeygenParamsType;
        IKMSGeneration.ParamsType crsParamsType;
        uint256 contextId;
    }

    error AbortCrsgenAlreadyDone(uint256 crsId);
    error AbortCrsgenInvalidId(uint256 crsId);
    error AbortKeygenAlreadyDone(uint256 keyId);
    error AbortKeygenInvalidId(uint256 keyId);
    error AddressEmptyCode(address target);
    error CrsNotGenerated(uint256 crsId);
    error CrsgenNotRequested(uint256 crsId);
    error CrsgenOngoing(uint256 crsId);
    error DeserializingExtraDataFail();
    error ECDSAInvalidSignature();
    error ECDSAInvalidSignatureLength(uint256 length);
    error ECDSAInvalidSignatureS(bytes32 s);
    error ERC1967InvalidImplementation(address implementation);
    error ERC1967NonPayable();
    error EmptyKeyDigests(uint256 keyId);
    error FailedCall();
    error InvalidInitialization();
    error InvalidMigrationConsensusState(uint256 requestId);
    error InvalidMigrationCounterState();
    error InvalidMigrationMaterial(uint256 requestId);
    error KeyManagementRequestPending();
    error KeyNotGenerated(uint256 keyId);
    error KeygenNotRequested(uint256 keyId);
    error KeygenOngoing(uint256 keyId);
    error KmsAlreadySignedForCrsgen(uint256 crsId, address kmsSigner);
    error KmsAlreadySignedForKeygen(uint256 keyId, address kmsSigner);
    error KmsAlreadySignedForPrepKeygen(uint256 prepKeygenId, address kmsSigner);
    error KmsSignerDoesNotMatchTxSender(address signerAddress, address txSenderAddress);
    error NotHostOwner(address sender);
    error NotInitializing();
    error NotInitializingFromEmptyProxy();
    error NotKmsTxSender(address txSenderAddress);
    error PrepKeygenNotRequested(uint256 prepKeygenId);
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);
    error UnknownMigrationConsensusTxSender(uint256 requestId, address txSender);
    error UnsupportedExtraDataVersion(uint8 version);

    event AbortCrsgen(uint256 crsId);
    event AbortKeygen(uint256 keyId);
    event ActivateCrs(uint256 crsId, string[] kmsNodeStorageUrls, bytes crsDigest);
    event ActivateKey(uint256 keyId, string[] kmsNodeStorageUrls, IKMSGeneration.KeyDigest[] keyDigests);
    event CrsgenRequest(uint256 crsId, uint256 maxBitLength, IKMSGeneration.ParamsType paramsType, bytes extraData);
    event CrsgenResponse(uint256 crsId, bytes crsDigest, bytes signature, address kmsTxSender);
    event EIP712DomainChanged();
    event Initialized(uint64 version);
    event KeygenRequest(uint256 prepKeygenId, uint256 keyId, bytes extraData);
    event KeygenResponse(uint256 keyId, IKMSGeneration.KeyDigest[] keyDigests, bytes signature, address kmsTxSender);
    event PrepKeygenRequest(uint256 prepKeygenId, IKMSGeneration.ParamsType paramsType, bytes extraData);
    event PrepKeygenResponse(uint256 prepKeygenId, bytes signature, address kmsTxSender);
    event Upgraded(address indexed implementation);

    constructor();

    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function abortCrsgen(uint256 crsId) external;
    function abortKeygen(uint256 keyId) external;
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
    function hasPendingKeyManagementRequest() external view returns (bool);
    function initializeFromEmptyProxy() external;
    function initializeFromMigration(MigrationState memory state) external;
    function keygen(IKMSGeneration.ParamsType paramsType) external;
    function keygenResponse(uint256 keyId, IKMSGeneration.KeyDigest[] memory keyDigests, bytes memory signature) external;
    function prepKeygenResponse(uint256 prepKeygenId, bytes memory signature) external;
    function proxiableUUID() external view returns (bytes32);
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
    "name": "abortCrsgen",
    "inputs": [
      {
        "name": "crsId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "abortKeygen",
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
    "name": "hasPendingKeyManagementRequest",
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
    "name": "initializeFromEmptyProxy",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "initializeFromMigration",
    "inputs": [
      {
        "name": "state",
        "type": "tuple",
        "internalType": "struct KMSGeneration.MigrationState",
        "components": [
          {
            "name": "prepKeygenCounter",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "keyCounter",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "crsCounter",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "activeKeyId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "activeCrsId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "activePrepKeygenId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "activeKeyDigests",
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
            "name": "activeCrsDigest",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "keyConsensusTxSenders",
            "type": "address[]",
            "internalType": "address[]"
          },
          {
            "name": "keyConsensusDigest",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "crsConsensusTxSenders",
            "type": "address[]",
            "internalType": "address[]"
          },
          {
            "name": "crsConsensusDigest",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "prepKeygenConsensusTxSenders",
            "type": "address[]",
            "internalType": "address[]"
          },
          {
            "name": "prepKeygenConsensusDigest",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "crsMaxBitLength",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "prepKeygenParamsType",
            "type": "uint8",
            "internalType": "enum IKMSGeneration.ParamsType"
          },
          {
            "name": "crsParamsType",
            "type": "uint8",
            "internalType": "enum IKMSGeneration.ParamsType"
          },
          {
            "name": "contextId",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
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
    "name": "AbortCrsgen",
    "inputs": [
      {
        "name": "crsId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "AbortKeygen",
    "inputs": [
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
    "name": "PrepKeygenRequest",
    "inputs": [
      {
        "name": "prepKeygenId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "paramsType",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum IKMSGeneration.ParamsType"
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
    "name": "AbortCrsgenAlreadyDone",
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
    "name": "AbortCrsgenInvalidId",
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
    "name": "AbortKeygenAlreadyDone",
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
    "name": "AbortKeygenInvalidId",
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
    "name": "DeserializingExtraDataFail",
    "inputs": []
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
    "name": "InvalidInitialization",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidMigrationConsensusState",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidMigrationCounterState",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidMigrationMaterial",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "KeyManagementRequestPending",
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
    "name": "NotHostOwner",
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
  },
  {
    "type": "error",
    "name": "UnknownMigrationConsensusTxSender",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "txSender",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "UnsupportedExtraDataVersion",
    "inputs": [
      {
        "name": "version",
        "type": "uint8",
        "internalType": "uint8"
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
    ///0x60a06040523073ffffffffffffffffffffffffffffffffffffffff1660809073ffffffffffffffffffffffffffffffffffffffff1681525034801562000043575f80fd5b50620000546200005a60201b60201c565b620001c4565b5f6200006b6200015e60201b60201c565b9050805f0160089054906101000a900460ff1615620000b6576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b67ffffffffffffffff8016815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff16146200015b5767ffffffffffffffff815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d267ffffffffffffffff604051620001529190620001a9565b60405180910390a15b50565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b5f67ffffffffffffffff82169050919050565b620001a38162000185565b82525050565b5f602082019050620001be5f83018462000198565b92915050565b60805161766a620001eb5f395f8181613cf001528181613d450152613fe7015261766a5ff3fe608060405260043610610134575f3560e01c806362978787116100aa578063ad3cb1cc1161006e578063ad3cb1cc146103f9578063baff211e14610423578063c2c1faee1461044d578063c55b872414610475578063caa367db146104b2578063d52f10eb146104da57610134565b80636297878714610312578063735b12631461033a57806384b0196e14610364578063936608ae14610394578063a0079e0f146103d157610134565b80633c02f834116100fc5780633c02f8341461021857806345af261b146102405780634610ffe81461027c5780634f1ef286146102a457806352d1902d146102c0578063589adb0e146102ea57610134565b80630d8e6e2c1461013857806316c713d9146101625780631703c61a1461019e57806319f4f632146101c657806339f7381014610202575b5f80fd5b348015610143575f80fd5b5061014c610504565b60405161015991906153b2565b60405180910390f35b34801561016d575f80fd5b5061018860048036038101906101839190615416565b61057f565b6040516101959190615528565b60405180910390f35b3480156101a9575f80fd5b506101c460048036038101906101bf9190615416565b610650565b005b3480156101d1575f80fd5b506101ec60048036038101906101e79190615416565b61086f565b6040516101f991906155bb565b60405180910390f35b34801561020d575f80fd5b5061021661091c565b005b348015610223575f80fd5b5061023e600480360381019061023991906155f7565b610b3f565b005b34801561024b575f80fd5b5061026660048036038101906102619190615416565b610e23565b60405161027391906155bb565b60405180910390f35b348015610287575f80fd5b506102a2600480360381019061029d91906156eb565b610eb8565b005b6102be60048036038101906102b991906158ce565b611639565b005b3480156102cb575f80fd5b506102d4611658565b6040516102e19190615940565b60405180910390f35b3480156102f5575f80fd5b50610310600480360381019061030b9190615959565b611689565b005b34801561031d575f80fd5b50610338600480360381019061033391906159b6565b611b0f565b005b348015610345575f80fd5b5061034e61218f565b60405161035b9190615a61565b60405180910390f35b34801561036f575f80fd5b50610378612240565b60405161038b9796959493929190615b89565b60405180910390f35b34801561039f575f80fd5b506103ba60048036038101906103b59190615416565b612349565b6040516103c8929190615e9b565b60405180910390f35b3480156103dc575f80fd5b506103f760048036038101906103f29190615ef3565b612669565b005b348015610404575f80fd5b5061040d612d56565b60405161041a91906153b2565b60405180910390f35b34801561042e575f80fd5b50610437612d8f565b6040516104449190615f3a565b60405180910390f35b348015610458575f80fd5b50610473600480360381019061046e9190615416565b612da6565b005b348015610480575f80fd5b5061049b60048036038101906104969190615416565b613010565b6040516104a9929190615f9b565b60405180910390f35b3480156104bd575f80fd5b506104d860048036038101906104d39190615fd0565b61329b565b005b3480156104e5575f80fd5b506104ee6135d7565b6040516104fb9190615f3a565b60405180910390f35b60606040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506105455f6135ee565b61054f60016135ee565b6105585f6135ee565b60405160200161056b94939291906160c9565b604051602081830303815290604052905090565b60605f61058a6136b8565b90505f816003015f8581526020019081526020015f20549050816002015f8581526020019081526020015f205f8281526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561064257602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116105f9575b505050505092505050919050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156106ad573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106d1919061613b565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461074057336040517f21bfda100000000000000000000000000000000000000000000000000000000081526004016107379190616166565b60405180910390fd5b5f6107496136b8565b90508060090154821180610765575060f8600560ff16901b8211155b156107a757816040517fcbe9265600000000000000000000000000000000000000000000000000000000815260040161079e9190615f3a565b60405180910390fd5b806001015f8381526020019081526020015f205f9054906101000a900460ff161561080957816040517fdf0db5fb0000000000000000000000000000000000000000000000000000000081526004016108009190615f3a565b60405180910390fd5b6001816001015f8481526020019081526020015f205f6101000a81548160ff0219169083151502179055507f384f90fefbcfaa68f22e00094aeaa52b2bc693936d2ce1afed1212520b59b58e826040516108639190615f3a565b60405180910390a15050565b5f806108796136b8565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff166108dc57826040517f84de13310000000000000000000000000000000000000000000000000000000081526004016108d39190615f3a565b60405180910390fd5b5f816006015f8581526020019081526020015f2054905081600d015f8281526020019081526020015f205f9054906101000a900460ff1692505050919050565b60016109266136df565b67ffffffffffffffff1614610967576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60025f610972613703565b9050805f0160089054906101000a900460ff16806109ba57508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b156109f1576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff021916908315150217905550610aaa6040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506040518060400160405280600181526020017f310000000000000000000000000000000000000000000000000000000000000081525061372a565b5f610ab36136b8565b905060f8600360ff16901b816004018190555060f8600460ff16901b816005018190555060f8600560ff16901b8160090181905550505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051610b3391906161a1565b60405180910390a15050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610b9c573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610bc0919061613b565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610c2f57336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401610c269190616166565b60405180910390fd5b5f610c386136b8565b90505f8160090154905060f8600560ff16901b8114158015610c775750816001015f8281526020019081526020015f205f9054906101000a900460ff16155b15610cb957806040517f061ac61d000000000000000000000000000000000000000000000000000000008152600401610cb09190615f3a565b60405180910390fd5b816009015f815480929190610ccd906161e7565b91905055505f826009015490508483600a015f8381526020019081526020015f20819055508383600d015f8381526020019081526020015f205f6101000a81548160ff02191690836001811115610d2757610d26615548565b5b02179055505f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa158015610d8a573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610dae9190616242565b90505f610dba82613740565b90508085600e015f8581526020019081526020015f209081610ddc9190616467565b507f8cf0151393f84fd694c5e315cb74cc05b247de0a454fd9e9129c661efdf9401d83888884604051610e129493929190616536565b60405180910390a150505050505050565b5f80610e2d6136b8565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff16610e9057826040517fda32d00f000000000000000000000000000000000000000000000000000000008152600401610e879190615f3a565b60405180910390fd5b80600d015f8481526020019081526020015f205f9054906101000a900460ff16915050919050565b5f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa158015610f16573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f3a9190616242565b90507344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166346c5bbbd82336040518363ffffffff1660e01b8152600401610f8b929190616580565b602060405180830381865afa158015610fa6573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610fca91906165d1565b61100b57336040517faee863230000000000000000000000000000000000000000000000000000000081526004016110029190616166565b60405180910390fd5b5f6110146136b8565b90508060050154871180611030575060f8600460ff16901b8711155b1561107257866040517fadfab9040000000000000000000000000000000000000000000000000000000081526004016110699190615f3a565b60405180910390fd5b5f86869050036110b957866040517fe6f9083b0000000000000000000000000000000000000000000000000000000081526004016110b09190615f3a565b60405180910390fd5b5f816006015f8981526020019081526020015f20549050816001015f8281526020019081526020015f205f9054906101000a900460ff16611126576040517f6fbcdd2b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f6111cd828a8a8a87600e015f8f81526020019081526020015f20805461114c9061629a565b80601f01602080910402602001604051908101604052809291908181526020018280546111789061629a565b80156111c35780601f1061119a576101008083540402835291602001916111c3565b820191905f5260205f20905b8154815290600101906020018083116111a657829003601f168201915b505050505061376e565b90505f6111db82888861394f565b9050835f015f8b81526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff161561127b5789816040517f98fb957d000000000000000000000000000000000000000000000000000000008152600401611272929190616580565b60405180910390fd5b6001845f015f8c81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f846002015f8c81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505f818054905090507f2afe64fb3afde8e2678aea84cf36223f330e2fb1286d37aed573ab9cd1db47c78c8c8c8c8c336040516113a596959493929190616808565b60405180910390a1856001015f8d81526020019081526020015f205f9054906101000a900460ff161580156113df57506113de816139b5565b5b1561162b576001866001015f8e81526020019081526020015f205f6101000a81548160ff0219169083151502179055505f5b8b8b905081101561149557866007015f8e81526020019081526020015f208c8c838181106114425761144161685d565b5b90506020028101906114549190616896565b908060018154018082558091505060019003905f5260205f2090600202015f9091909190915081816114869190616ac9565b50508080600101915050611411565b5083866003015f8e81526020019081526020015f20819055508b86600801819055505f61155a87600e015f8f81526020019081526020015f2080546114d99061629a565b80601f01602080910402602001604051908101604052809291908181526020018280546115059061629a565b80156115505780601f1061152757610100808354040283529160200191611550565b820191905f5260205f20905b81548152906001019060200180831161153357829003601f168201915b5050505050613a46565b90505f6115e982858054806020026020016040519081016040528092919081815260200182805480156115df57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311611596575b5050505050613ba6565b90507feb85c26dbcad46b80a68a0f24cce7c2c90f0a1faded84184138839fc9e80a25b8e828f8f6040516116209493929190616ad7565b60405180910390a150505b505050505050505050505050565b611641613cee565b61164a82613dd4565b6116548282613ec7565b5050565b5f611661613fe5565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b5f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa1580156116e7573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061170b9190616242565b90507344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166346c5bbbd82336040518363ffffffff1660e01b815260040161175c929190616580565b602060405180830381865afa158015611777573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061179b91906165d1565b6117dc57336040517faee863230000000000000000000000000000000000000000000000000000000081526004016117d39190616166565b60405180910390fd5b5f6117e56136b8565b90508060040154851180611801575060f8600360ff16901b8511155b1561184357846040517f0ab7f68700000000000000000000000000000000000000000000000000000000815260040161183a9190615f3a565b60405180910390fd5b5f61184d8661406c565b90505f61185b82878761394f565b9050825f015f8881526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16156118fb5786816040517f33ca1fe30000000000000000000000000000000000000000000000000000000081526004016118f2929190616580565b60405180910390fd5b6001835f015f8981526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f836002015f8981526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507f4c715c5734ce5c18c9c12e8496e53d2a65f1ec381d476957f0f596b364a59b0c88888833604051611a199493929190616b1c565b60405180910390a1836001015f8981526020019081526020015f205f9054906101000a900460ff16158015611a575750611a5681805490506139b5565b5b15611b05576001846001015f8a81526020019081526020015f205f6101000a81548160ff02191690831515021790555082846003015f8a81526020019081526020015f20819055505f846006015f8a81526020019081526020015f205490507f3a116120cca5d4f073cc1fc31ff26133ab7b0499f2712fa010023b87d5a1f9ee898287600e015f8d81526020019081526020015f20604051611afb93929190616bdb565b60405180910390a1505b5050505050505050565b5f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa158015611b6d573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b919190616242565b90507344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166346c5bbbd82336040518363ffffffff1660e01b8152600401611be2929190616580565b602060405180830381865afa158015611bfd573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c2191906165d1565b611c6257336040517faee86323000000000000000000000000000000000000000000000000000000008152600401611c599190616166565b60405180910390fd5b5f611c6b6136b8565b90508060090154871180611c87575060f8600560ff16901b8711155b15611cc957866040517f8d8c940a000000000000000000000000000000000000000000000000000000008152600401611cc09190615f3a565b60405180910390fd5b5f81600a015f8981526020019081526020015f205490505f611d8789838a8a87600e015f8f81526020019081526020015f208054611d069061629a565b80601f0160208091040260200160405190810160405280929190818152602001828054611d329061629a565b8015611d7d5780601f10611d5457610100808354040283529160200191611d7d565b820191905f5260205f20905b815481529060010190602001808311611d6057829003601f168201915b50505050506140c4565b90505f611d9582888861394f565b9050835f015f8b81526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615611e355789816040517ffcf5a6e9000000000000000000000000000000000000000000000000000000008152600401611e2c929190616580565b60405180910390fd5b6001845f015f8c81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f846002015f8c81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505f818054905090507f7bf1b42c10e9497c879620c5b7afced10bda17d8c90b22f0e3bc6b2fd6ced0bd8c8c8c8c8c33604051611f5f96959493929190616c17565b60405180910390a1856001015f8d81526020019081526020015f205f9054906101000a900460ff16158015611f995750611f98816139b5565b5b15612181576001866001015f8e81526020019081526020015f205f6101000a81548160ff0219169083151502179055508a8a87600b015f8f81526020019081526020015f209182611feb9291906169a8565b5083866003015f8e81526020019081526020015f20819055508b86600c01819055505f6120b087600e015f8f81526020019081526020015f20805461202f9061629a565b80601f016020809104026020016040519081016040528092919081815260200182805461205b9061629a565b80156120a65780601f1061207d576101008083540402835291602001916120a6565b820191905f5260205f20905b81548152906001019060200180831161208957829003601f168201915b5050505050613a46565b90505f61213f828580548060200260200160405190810160405280929190818152602001828054801561213557602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116120ec575b5050505050613ba6565b90507f2258b73faed33fb2e2ea454403bef974920caf682ab3a723484fcf67553b16a28e828f8f6040516121769493929190616c6c565b60405180910390a150505b505050505050505050505050565b5f806121996136b8565b90505f8160050154905060f8600460ff16901b81141580156121d85750816001015f8281526020019081526020015f205f9054906101000a900460ff16155b156121e85760019250505061223d565b5f8260090154905060f8600560ff16901b81141580156122255750826001015f8281526020019081526020015f205f9054906101000a900460ff16155b15612236576001935050505061223d565b5f93505050505b90565b5f6060805f805f60605f612252614155565b90505f801b815f015414801561226d57505f801b8160010154145b6122ac576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016122a390616cfb565b60405180910390fd5b6122b461417c565b6122bc61421a565b46305f801b5f67ffffffffffffffff8111156122db576122da6157aa565b5b6040519080825280602002602001820160405280156123095781602001602082028036833780820191505090505b507f0f0000000000000000000000000000000000000000000000000000000000000095949392919097509750975097509750975097505090919293949596565b6060805f6123556136b8565b9050806001015f8581526020019081526020015f205f9054906101000a900460ff166123b857836040517f84de13310000000000000000000000000000000000000000000000000000000081526004016123af9190615f3a565b60405180910390fd5b5f816003015f8681526020019081526020015f205490505f826002015f8781526020019081526020015f205f8381526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561246f57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311612426575b505050505090505f61251984600e015f8981526020019081526020015f2080546124989061629a565b80601f01602080910402602001604051908101604052809291908181526020018280546124c49061629a565b801561250f5780601f106124e65761010080835404028352916020019161250f565b820191905f5260205f20905b8154815290600101906020018083116124f257829003601f168201915b5050505050613a46565b90505f6125268284613ba6565b905080856007015f8a81526020019081526020015f2080805480602002602001604051908101604052809291908181526020015f905b82821015612655578382905f5260205f2090600202016040518060400160405290815f82015f9054906101000a900460ff1660018111156125a05761259f615548565b5b60018111156125b2576125b1615548565b5b81526020016001820180546125c69061629a565b80601f01602080910402602001604051908101604052809291908181526020018280546125f29061629a565b801561263d5780601f106126145761010080835404028352916020019161263d565b820191905f5260205f20905b81548152906001019060200180831161262057829003601f168201915b5050505050815250508152602001906001019061255c565b505050509050965096505050505050915091565b60016126736136df565b67ffffffffffffffff16146126b4576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60025f6126bf613703565b9050805f0160089054906101000a900460ff168061270757508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b1561273e576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055506127f76040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506040518060400160405280600181526020017f310000000000000000000000000000000000000000000000000000000000000081525061372a565b5f6128006136b8565b905061280b846142b8565b6128358460600135858061010001906128249190616d19565b8761012001358861022001356143c8565b61285f84608001358580610140019061284e9190616d19565b8761016001358861022001356143c8565b6128898460a00135858061018001906128789190616d19565b876101a001358861022001356143c8565b5f848060c0019061289a9190616d7b565b9050036128e25783606001356040517f16bbaf8d0000000000000000000000000000000000000000000000000000000081526004016128d99190615f3a565b60405180910390fd5b5f848060e001906128f3919061693c565b90500361293b5783608001356040517f16bbaf8d0000000000000000000000000000000000000000000000000000000081526004016129329190615f3a565b60405180910390fd5b835f01358160040181905550836020013581600501819055508360400135816009018190555083606001358160080181905550836080013581600c01819055508360600135816006015f8660a0013581526020019081526020015f20819055508360a00135816006015f866060013581526020019081526020015f20819055505f5b848060c001906129cd9190616d7b565b9050811015612a6157816007015f866060013581526020019081526020015f20858060c001906129fd9190616d7b565b83818110612a0e57612a0d61685d565b5b9050602002810190612a209190616896565b908060018154018082558091505060019003905f5260205f2090600202015f909190919091508181612a529190616ac9565b505080806001019150506129bd565b50838060e00190612a72919061693c565b82600b015f876080013581526020019081526020015f209182612a969291906169a8565b50836101c0013581600a015f866080013581526020019081526020015f2081905550612ae381856060013586806101000190612ad29190616d19565b88610120013589610220013561453b565b612b0e81856080013586806101400190612afd9190616d19565b88610160013589610220013561453b565b612b39818560a0013586806101800190612b289190616d19565b886101a0013589610220013561453b565b6001816001015f8660a0013581526020019081526020015f205f6101000a81548160ff0219169083151502179055506001816001015f866060013581526020019081526020015f205f6101000a81548160ff0219169083151502179055506001816001015f866080013581526020019081526020015f205f6101000a81548160ff021916908315150217905550836101e0016020810190612bda9190615fd0565b81600d015f8660a0013581526020019081526020015f205f6101000a81548160ff02191690836001811115612c1257612c11615548565b5b021790555083610200016020810190612c2b9190615fd0565b81600d015f866080013581526020019081526020015f205f6101000a81548160ff02191690836001811115612c6357612c62615548565b5b0217905550612c76846102200135613740565b81600e015f8660a0013581526020019081526020015f209081612c999190616467565b50612ca8846102200135613740565b81600e015f866060013581526020019081526020015f209081612ccb9190616467565b50612cda846102200135613740565b81600e015f866080013581526020019081526020015f209081612cfd9190616467565b50505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051612d4991906161a1565b60405180910390a1505050565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b5f80612d996136b8565b905080600c015491505090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612e03573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612e27919061613b565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614612e9657336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401612e8d9190616166565b60405180910390fd5b5f612e9f6136b8565b90508060050154821180612ebb575060f8600460ff16901b8211155b15612efd57816040517ffcf2db7a000000000000000000000000000000000000000000000000000000008152600401612ef49190615f3a565b60405180910390fd5b806001015f8381526020019081526020015f205f9054906101000a900460ff1615612f5f57816040517f92789b67000000000000000000000000000000000000000000000000000000008152600401612f569190615f3a565b60405180910390fd5b6001816001015f8481526020019081526020015f205f6101000a81548160ff0219169083151502179055505f816006015f8481526020019081526020015f205490505f8114612fd4576001826001015f8381526020019081526020015f205f6101000a81548160ff0219169083151502179055505b7f2b087b884b35a81d769d1a1e092880f1da56de964e4b339eabcb1f45f5fe3264836040516130039190615f3a565b60405180910390a1505050565b6060805f61301c6136b8565b9050806001015f8581526020019081526020015f205f9054906101000a900460ff1661307f57836040517fda32d00f0000000000000000000000000000000000000000000000000000000081526004016130769190615f3a565b60405180910390fd5b5f816003015f8681526020019081526020015f205490505f826002015f8781526020019081526020015f205f8381526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561313657602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116130ed575b505050505090505f6131e084600e015f8981526020019081526020015f20805461315f9061629a565b80601f016020809104026020016040519081016040528092919081815260200182805461318b9061629a565b80156131d65780601f106131ad576101008083540402835291602001916131d6565b820191905f5260205f20905b8154815290600101906020018083116131b957829003601f168201915b5050505050613a46565b90505f6131ed8284613ba6565b90508085600b015f8a81526020019081526020015f2080805461320f9061629a565b80601f016020809104026020016040519081016040528092919081815260200182805461323b9061629a565b80156132865780601f1061325d57610100808354040283529160200191613286565b820191905f5260205f20905b81548152906001019060200180831161326957829003601f168201915b50505050509050965096505050505050915091565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156132f8573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061331c919061613b565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461338b57336040517f21bfda100000000000000000000000000000000000000000000000000000000081526004016133829190616166565b60405180910390fd5b5f6133946136b8565b90505f8160050154905060f8600460ff16901b81141580156133d35750816001015f8281526020019081526020015f205f9054906101000a900460ff16155b1561341557806040517f3b853da800000000000000000000000000000000000000000000000000000000815260040161340c9190615f3a565b60405180910390fd5b816004015f815480929190613429906161e7565b91905055505f82600401549050826005015f81548092919061344a906161e7565b91905055505f8360050154905080846006015f8481526020019081526020015f208190555081846006015f8381526020019081526020015f20819055508484600d015f8481526020019081526020015f205f6101000a81548160ff021916908360018111156134bc576134bb615548565b5b02179055505f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa15801561351f573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906135439190616242565b90505f61354f82613740565b90508086600e015f8681526020019081526020015f2090816135719190616467565b508086600e015f8581526020019081526020015f2090816135929190616467565b507ffbf5274810b94f86970c1147e8ffaebed246ee9777d695a69004dc6256d1fe918488836040516135c693929190616ddd565b60405180910390a150505050505050565b5f806135e16136b8565b9050806008015491505090565b60605f60016135fc84614720565b0190505f8167ffffffffffffffff81111561361a576136196157aa565b5b6040519080825280601f01601f19166020018201604052801561364c5781602001600182028036833780820191505090505b5090505f82602001820190505b6001156136ad578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a85816136a2576136a1616e19565b5b0494505f8503613659575b819350505050919050565b5f7f26fdaf8a2cb20d20b55e36218986905e534ee7a970dd2fa827946e4b7496db00905090565b5f6136e8613703565b5f015f9054906101000a900467ffffffffffffffff16905090565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b613732614871565b61373c82826148b1565b5050565b60606002825f60405160200161375893929190616ea6565b6040516020818303038152906040529050919050565b5f808484905067ffffffffffffffff81111561378d5761378c6157aa565b5b6040519080825280602002602001820160405280156137bb5781602001602082028036833780820191505090505b5090505f5b858590508110156138bf5760405180606001604052806025815260200161764560259139805190602001208686838181106137fe576137fd61685d565b5b90506020028101906138109190616896565b5f0160208101906138219190616ee2565b8787848181106138345761383361685d565b5b90506020028101906138469190616896565b8060200190613855919061693c565b604051613863929190616f3b565b604051809103902060405160200161387d93929190616f62565b604051602081830303815290604052805190602001208282815181106138a6576138a561685d565b5b60200260200101818152505080806001019150506137c0565b506139436040518060c00160405280608281526020016175c360829139805190602001208888846040516020016138f69190617048565b60405160208183030381529060405280519060200120878051906020012060405160200161392895949392919061705e565b60405160208183030381529060405280519060200120614902565b91505095945050505050565b5f8061399e8585858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505061491b565b90506139aa8133614945565b809150509392505050565b5f807344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663b4722bc46040518163ffffffff1660e01b8152600401602060405180830381865afa158015613a14573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613a389190616242565b905080831015915050919050565b5f80825103613ad7577344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa158015613aac573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613ad09190616242565b9050613ba1565b5f825f81518110613aeb57613aea61685d565b5b602001015160f81c60f81b60f81c9050600160ff168160ff161480613b165750600260ff168160ff16145b15613b6457602183511015613b57576040517f8b248b6000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6021830151915050613ba1565b806040517f2139cc2c000000000000000000000000000000000000000000000000000000008152600401613b9891906170be565b60405180910390fd5b919050565b60605f825190505f8167ffffffffffffffff811115613bc857613bc76157aa565b5b604051908082528060200260200182016040528015613bfb57816020015b6060815260200190600190039081613be65790505b5090505f5b82811015613ce2577344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166331ff41c887878481518110613c4c57613c4b61685d565b5b60200260200101516040518363ffffffff1660e01b8152600401613c71929190616580565b5f60405180830381865afa158015613c8b573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f82011682018060405250810190613cb3919061722a565b60600151828281518110613cca57613cc961685d565b5b60200260200101819052508080600101915050613c00565b50809250505092915050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff161480613d9b57507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16613d82614baa565b73ffffffffffffffffffffffffffffffffffffffff1614155b15613dd2576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613e31573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613e55919061613b565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614613ec457336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401613ebb9190616166565b60405180910390fd5b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa925050508015613f2f57506040513d601f19601f82011682018060405250810190613f2c919061729b565b60015b613f7057816040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401613f679190616166565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b8114613fd657806040517faa1d49a4000000000000000000000000000000000000000000000000000000008152600401613fcd9190615940565b60405180910390fd5b613fe08383614bfd565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff161461406a576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f6140bd6040518060600160405280602c8152602001617541602c913980519060200120836040516020016140a29291906172c6565b60405160208183030381529060405280519060200120614902565b9050919050565b5f61414a60405180608001604052806056815260200161756d6056913980519060200120878787876040516020016140fd929190616f3b565b60405160208183030381529060405280519060200120868051906020012060405160200161412f95949392919061705e565b60405160208183030381529060405280519060200120614902565b905095945050505050565b5f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100905090565b60605f614187614155565b90508060020180546141989061629a565b80601f01602080910402602001604051908101604052809291908181526020018280546141c49061629a565b801561420f5780601f106141e65761010080835404028352916020019161420f565b820191905f5260205f20905b8154815290600101906020018083116141f257829003601f168201915b505050505091505090565b60605f614225614155565b90508060030180546142369061629a565b80601f01602080910402602001604051908101604052809291908181526020018280546142629061629a565b80156142ad5780601f10614284576101008083540402835291602001916142ad565b820191905f5260205f20905b81548152906001019060200180831161429057829003601f168201915b505050505091505090565b60f8600360ff16901b8160a001351115806142da57508060a00135815f013514155b15614311576040517fa4d3d4f200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60f8600460ff16901b816060013511158061433457508060600135816020013514155b1561436b576040517fa4d3d4f200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60f8600560ff16901b816080013511158061438e57508060800135816040013514155b156143c5576040517fa4d3d4f200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50565b5f801b8214806143da57505f84849050145b1561441c57846040517f4502cbf10000000000000000000000000000000000000000000000000000000081526004016144139190615f3a565b60405180910390fd5b5f5b84849050811015614533575f85858381811061443d5761443c61685d565b5b905060200201602081019061445291906172ed565b90507344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166346c5bbbd84836040518363ffffffff1660e01b81526004016144a3929190616580565b602060405180830381865afa1580156144be573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906144e291906165d1565b6145255786816040517f8bd0309700000000000000000000000000000000000000000000000000000000815260040161451c929190616580565b60405180910390fd5b50808060010191505061441e565b505050505050565b81866003015f8781526020019081526020015f20819055505f5b84849050811015614717575f8585838181106145745761457361685d565b5b905060200201602081019061458991906172ed565b9050876002015f8881526020019081526020015f205f8581526020019081526020015f2081908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166331ff41c885846040518363ffffffff1660e01b815260040161465b929190616580565b5f60405180830381865afa158015614675573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f8201168201806040525081019061469d919061722a565b6020015190506001895f015f8a81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555050508080600101915050614555565b50505050505050565b5f805f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f010000000000000000831061477c577a184f03e93ff9f4daa797ed6e38ed64bf6a1f010000000000000000838161477257614771616e19565b5b0492506040810190505b6d04ee2d6d415b85acef810000000083106147b9576d04ee2d6d415b85acef810000000083816147af576147ae616e19565b5b0492506020810190505b662386f26fc1000083106147e857662386f26fc1000083816147de576147dd616e19565b5b0492506010810190505b6305f5e1008310614811576305f5e100838161480757614806616e19565b5b0492506008810190505b612710831061483657612710838161482c5761482b616e19565b5b0492506004810190505b60648310614859576064838161484f5761484e616e19565b5b0492506002810190505b600a8310614868576001810190505b80915050919050565b614879614c6f565b6148af576040517fd7e6bcf800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b6148b9614871565b5f6148c2614155565b9050828160020190816148d59190617370565b50818160030190816148e79190617370565b505f801b815f01819055505f801b8160010181905550505050565b5f61491461490e614c8d565b83614c9b565b9050919050565b5f805f806149298686614cdb565b9250925092506149398282614d30565b82935050505092915050565b5f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa1580156149a3573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906149c79190616242565b90507344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff16639447cfd482856040518363ffffffff1660e01b8152600401614a18929190616580565b602060405180830381865afa158015614a33573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614a5791906165d1565b614a9a5782826040517f0d86f521000000000000000000000000000000000000000000000000000000008152600401614a9192919061743f565b60405180910390fd5b5f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166331ff41c883856040518363ffffffff1660e01b8152600401614aea929190616580565b5f60405180830381865afa158015614b04573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f82011682018060405250810190614b2c919061722a565b90508373ffffffffffffffffffffffffffffffffffffffff16816020015173ffffffffffffffffffffffffffffffffffffffff1614614ba45783836040517f0d86f521000000000000000000000000000000000000000000000000000000008152600401614b9b92919061743f565b60405180910390fd5b50505050565b5f614bd67f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b614e92565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b614c0682614e9b565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f81511115614c6257614c5c8282614f64565b50614c6b565b614c6a614fe4565b5b5050565b5f614c78613703565b5f0160089054906101000a900460ff16905090565b5f614c96615020565b905090565b5f6040517f190100000000000000000000000000000000000000000000000000000000000081528360028201528260228201526042812091505092915050565b5f805f6041845103614d1b575f805f602087015192506040870151915060608701515f1a9050614d0d88828585615083565b955095509550505050614d29565b5f600285515f1b9250925092505b9250925092565b5f6003811115614d4357614d42615548565b5b826003811115614d5657614d55615548565b5b0315614e8e5760016003811115614d7057614d6f615548565b5b826003811115614d8357614d82615548565b5b03614dba576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60026003811115614dce57614dcd615548565b5b826003811115614de157614de0615548565b5b03614e2557805f1c6040517ffce698f7000000000000000000000000000000000000000000000000000000008152600401614e1c9190615f3a565b60405180910390fd5b600380811115614e3857614e37615548565b5b826003811115614e4b57614e4a615548565b5b03614e8d57806040517fd78bce0c000000000000000000000000000000000000000000000000000000008152600401614e849190615940565b60405180910390fd5b5b5050565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b03614ef657806040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401614eed9190616166565b60405180910390fd5b80614f227f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b614e92565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f808473ffffffffffffffffffffffffffffffffffffffff1684604051614f8d9190617496565b5f60405180830381855af49150503d805f8114614fc5576040519150601f19603f3d011682016040523d82523d5f602084013e614fca565b606091505b5091509150614fda85838361516a565b9250505092915050565b5f34111561501e576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f61504a6151f7565b61505261526d565b46306040516020016150689594939291906174ac565b60405160208183030381529060405280519060200120905090565b5f805f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c11156150bf575f600385925092509250615160565b5f6001888888886040515f81526020016040526040516150e294939291906174fd565b6020604051602081039080840390855afa158015615102573d5f803e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603615153575f60015f801b93509350935050615160565b805f805f1b935093509350505b9450945094915050565b60608261517f5761517a826152e4565b6151ef565b5f82511480156151a557505f8473ffffffffffffffffffffffffffffffffffffffff163b145b156151e757836040517f9996b3150000000000000000000000000000000000000000000000000000000081526004016151de9190616166565b60405180910390fd5b8190506151f0565b5b9392505050565b5f80615201614155565b90505f61520c61417c565b90505f815111156152285780805190602001209250505061526a565b5f825f015490505f801b81146152435780935050505061526a565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f80615277614155565b90505f61528261421a565b90505f8151111561529e578080519060200120925050506152e1565b5f826001015490505f801b81146152ba578093505050506152e1565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f815111156152f65780518082602001fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f81519050919050565b5f82825260208201905092915050565b5f5b8381101561535f578082015181840152602081019050615344565b5f8484015250505050565b5f601f19601f8301169050919050565b5f61538482615328565b61538e8185615332565b935061539e818560208601615342565b6153a78161536a565b840191505092915050565b5f6020820190508181035f8301526153ca818461537a565b905092915050565b5f604051905090565b5f80fd5b5f80fd5b5f819050919050565b6153f5816153e3565b81146153ff575f80fd5b50565b5f81359050615410816153ec565b92915050565b5f6020828403121561542b5761542a6153db565b5b5f61543884828501615402565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6154938261546a565b9050919050565b6154a381615489565b82525050565b5f6154b4838361549a565b60208301905092915050565b5f602082019050919050565b5f6154d682615441565b6154e0818561544b565b93506154eb8361545b565b805f5b8381101561551b57815161550288826154a9565b975061550d836154c0565b9250506001810190506154ee565b5085935050505092915050565b5f6020820190508181035f83015261554081846154cc565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b6002811061558657615585615548565b5b50565b5f81905061559682615575565b919050565b5f6155a582615589565b9050919050565b6155b58161559b565b82525050565b5f6020820190506155ce5f8301846155ac565b92915050565b600281106155e0575f80fd5b50565b5f813590506155f1816155d4565b92915050565b5f806040838503121561560d5761560c6153db565b5b5f61561a85828601615402565b925050602061562b858286016155e3565b9150509250929050565b5f80fd5b5f80fd5b5f80fd5b5f8083601f84011261565657615655615635565b5b8235905067ffffffffffffffff81111561567357615672615639565b5b60208301915083602082028301111561568f5761568e61563d565b5b9250929050565b5f8083601f8401126156ab576156aa615635565b5b8235905067ffffffffffffffff8111156156c8576156c7615639565b5b6020830191508360018202830111156156e4576156e361563d565b5b9250929050565b5f805f805f60608688031215615704576157036153db565b5b5f61571188828901615402565b955050602086013567ffffffffffffffff811115615732576157316153df565b5b61573e88828901615641565b9450945050604086013567ffffffffffffffff811115615761576157606153df565b5b61576d88828901615696565b92509250509295509295909350565b61578581615489565b811461578f575f80fd5b50565b5f813590506157a08161577c565b92915050565b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6157e08261536a565b810181811067ffffffffffffffff821117156157ff576157fe6157aa565b5b80604052505050565b5f6158116153d2565b905061581d82826157d7565b919050565b5f67ffffffffffffffff82111561583c5761583b6157aa565b5b6158458261536a565b9050602081019050919050565b828183375f83830152505050565b5f61587261586d84615822565b615808565b90508281526020810184848401111561588e5761588d6157a6565b5b615899848285615852565b509392505050565b5f82601f8301126158b5576158b4615635565b5b81356158c5848260208601615860565b91505092915050565b5f80604083850312156158e4576158e36153db565b5b5f6158f185828601615792565b925050602083013567ffffffffffffffff811115615912576159116153df565b5b61591e858286016158a1565b9150509250929050565b5f819050919050565b61593a81615928565b82525050565b5f6020820190506159535f830184615931565b92915050565b5f805f604084860312156159705761596f6153db565b5b5f61597d86828701615402565b935050602084013567ffffffffffffffff81111561599e5761599d6153df565b5b6159aa86828701615696565b92509250509250925092565b5f805f805f606086880312156159cf576159ce6153db565b5b5f6159dc88828901615402565b955050602086013567ffffffffffffffff8111156159fd576159fc6153df565b5b615a0988828901615696565b9450945050604086013567ffffffffffffffff811115615a2c57615a2b6153df565b5b615a3888828901615696565b92509250509295509295909350565b5f8115159050919050565b615a5b81615a47565b82525050565b5f602082019050615a745f830184615a52565b92915050565b5f7fff0000000000000000000000000000000000000000000000000000000000000082169050919050565b615aae81615a7a565b82525050565b615abd816153e3565b82525050565b615acc81615489565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b615b04816153e3565b82525050565b5f615b158383615afb565b60208301905092915050565b5f602082019050919050565b5f615b3782615ad2565b615b418185615adc565b9350615b4c83615aec565b805f5b83811015615b7c578151615b638882615b0a565b9750615b6e83615b21565b925050600181019050615b4f565b5085935050505092915050565b5f60e082019050615b9c5f83018a615aa5565b8181036020830152615bae818961537a565b90508181036040830152615bc2818861537a565b9050615bd16060830187615ab4565b615bde6080830186615ac3565b615beb60a0830185615931565b81810360c0830152615bfd8184615b2d565b905098975050505050505050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f82825260208201905092915050565b5f615c4e82615328565b615c588185615c34565b9350615c68818560208601615342565b615c718161536a565b840191505092915050565b5f615c878383615c44565b905092915050565b5f602082019050919050565b5f615ca582615c0b565b615caf8185615c15565b935083602082028501615cc185615c25565b805f5b85811015615cfc5784840389528151615cdd8582615c7c565b9450615ce883615c8f565b925060208a01995050600181019050615cc4565b50829750879550505050505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b60028110615d4857615d47615548565b5b50565b5f819050615d5882615d37565b919050565b5f615d6782615d4b565b9050919050565b615d7781615d5d565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f615da182615d7d565b615dab8185615d87565b9350615dbb818560208601615342565b615dc48161536a565b840191505092915050565b5f604083015f830151615de45f860182615d6e565b5060208301518482036020860152615dfc8282615d97565b9150508091505092915050565b5f615e148383615dcf565b905092915050565b5f602082019050919050565b5f615e3282615d0e565b615e3c8185615d18565b935083602082028501615e4e85615d28565b805f5b85811015615e895784840389528151615e6a8582615e09565b9450615e7583615e1c565b925060208a01995050600181019050615e51565b50829750879550505050505092915050565b5f6040820190508181035f830152615eb38185615c9b565b90508181036020830152615ec78184615e28565b90509392505050565b5f80fd5b5f6102408284031215615eea57615ee9615ed0565b5b81905092915050565b5f60208284031215615f0857615f076153db565b5b5f82013567ffffffffffffffff811115615f2557615f246153df565b5b615f3184828501615ed4565b91505092915050565b5f602082019050615f4d5f830184615ab4565b92915050565b5f82825260208201905092915050565b5f615f6d82615d7d565b615f778185615f53565b9350615f87818560208601615342565b615f908161536a565b840191505092915050565b5f6040820190508181035f830152615fb38185615c9b565b90508181036020830152615fc78184615f63565b90509392505050565b5f60208284031215615fe557615fe46153db565b5b5f615ff2848285016155e3565b91505092915050565b5f81905092915050565b5f61600f82615328565b6160198185615ffb565b9350616029818560208601615342565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f616069600283615ffb565b915061607482616035565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f6160b3600183615ffb565b91506160be8261607f565b600182019050919050565b5f6160d48287616005565b91506160df8261605d565b91506160eb8286616005565b91506160f6826160a7565b91506161028285616005565b915061610d826160a7565b91506161198284616005565b915081905095945050505050565b5f815190506161358161577c565b92915050565b5f602082840312156161505761614f6153db565b5b5f61615d84828501616127565b91505092915050565b5f6020820190506161795f830184615ac3565b92915050565b5f67ffffffffffffffff82169050919050565b61619b8161617f565b82525050565b5f6020820190506161b45f830184616192565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f6161f1826153e3565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203616223576162226161ba565b5b600182019050919050565b5f8151905061623c816153ec565b92915050565b5f60208284031215616257576162566153db565b5b5f6162648482850161622e565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f60028204905060018216806162b157607f821691505b6020821081036162c4576162c361626d565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f600883026163267fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff826162eb565b61633086836162eb565b95508019841693508086168417925050509392505050565b5f819050919050565b5f61636b616366616361846153e3565b616348565b6153e3565b9050919050565b5f819050919050565b61638483616351565b61639861639082616372565b8484546162f7565b825550505050565b5f90565b6163ac6163a0565b6163b781848461637b565b505050565b5b818110156163da576163cf5f826163a4565b6001810190506163bd565b5050565b601f82111561641f576163f0816162ca565b6163f9846162dc565b81016020851015616408578190505b61641c616414856162dc565b8301826163bc565b50505b505050565b5f82821c905092915050565b5f61643f5f1984600802616424565b1980831691505092915050565b5f6164578383616430565b9150826002028217905092915050565b61647082615d7d565b67ffffffffffffffff811115616489576164886157aa565b5b616493825461629a565b61649e8282856163de565b5f60209050601f8311600181146164cf575f84156164bd578287015190505b6164c7858261644c565b86555061652e565b601f1984166164dd866162ca565b5f5b82811015616504578489015182556001820191506020850194506020810190506164df565b86831015616521578489015161651d601f891682616430565b8355505b6001600288020188555050505b505050505050565b5f6080820190506165495f830187615ab4565b6165566020830186615ab4565b61656360408301856155ac565b81810360608301526165758184615f63565b905095945050505050565b5f6040820190506165935f830185615ab4565b6165a06020830184615ac3565b9392505050565b6165b081615a47565b81146165ba575f80fd5b50565b5f815190506165cb816165a7565b92915050565b5f602082840312156165e6576165e56153db565b5b5f6165f3848285016165bd565b91505092915050565b5f819050919050565b60028110616611575f80fd5b50565b5f8135905061662281616605565b92915050565b5f6166366020840184616614565b905092915050565b5f80fd5b5f80fd5b5f80fd5b5f808335600160200384360303811261666657616665616646565b5b83810192508235915060208301925067ffffffffffffffff82111561668e5761668d61663e565b5b6001820236038313156166a4576166a3616642565b5b509250929050565b5f6166b78385615d87565b93506166c4838584615852565b6166cd8361536a565b840190509392505050565b5f604083016166e95f840184616628565b6166f55f860182615d6e565b50616703602084018461664a565b85830360208701526167168382846166ac565b925050508091505092915050565b5f61672f83836166d8565b905092915050565b5f8235600160400383360303811261675257616751616646565b5b82810191505092915050565b5f602082019050919050565b5f6167758385615d18565b935083602084028501616787846165fc565b805f5b878110156167ca5784840389526167a18284616737565b6167ab8582616724565b94506167b68361675e565b925060208a0199505060018101905061678a565b50829750879450505050509392505050565b5f6167e78385615f53565b93506167f4838584615852565b6167fd8361536a565b840190509392505050565b5f60808201905061681b5f830189615ab4565b818103602083015261682e81878961676a565b905081810360408301526168438185876167dc565b90506168526060830184615ac3565b979650505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b5f823560016040038336030381126168b1576168b061688a565b5b80830191505092915050565b5f81356168c981616605565b80915050919050565b5f815f1b9050919050565b5f60ff6168e9846168d2565b9350801983169250808416831791505092915050565b5f61690982615d4b565b9050919050565b5f819050919050565b616922826168ff565b61693561692e82616910565b83546168dd565b8255505050565b5f80833560016020038436030381126169585761695761688a565b5b80840192508235915067ffffffffffffffff82111561697a5761697961688e565b5b60208301925060018202360383131561699657616995616892565b5b509250929050565b5f82905092915050565b6169b2838361699e565b67ffffffffffffffff8111156169cb576169ca6157aa565b5b6169d5825461629a565b6169e08282856163de565b5f601f831160018114616a0d575f84156169fb578287013590505b616a05858261644c565b865550616a6c565b601f198416616a1b866162ca565b5f5b82811015616a4257848901358255600182019150602085019450602081019050616a1d565b86831015616a5f5784890135616a5b601f891682616430565b8355505b6001600288020188555050505b50505050505050565b616a808383836169a8565b505050565b5f81015f830180616a95816168bd565b9050616aa18184616919565b5050506001810160208301616ab6818561693c565b616ac1818386616a75565b505050505050565b616ad38282616a85565b5050565b5f606082019050616aea5f830187615ab4565b8181036020830152616afc8186615c9b565b90508181036040830152616b1181848661676a565b905095945050505050565b5f606082019050616b2f5f830187615ab4565b8181036020830152616b428185876167dc565b9050616b516040830184615ac3565b95945050505050565b5f8154616b668161629a565b616b708186615f53565b9450600182165f8114616b8a5760018114616ba057616bd2565b60ff198316865281151560200286019350616bd2565b616ba9856162ca565b5f5b83811015616bca57815481890152600182019150602081019050616bab565b808801955050505b50505092915050565b5f606082019050616bee5f830186615ab4565b616bfb6020830185615ab4565b8181036040830152616c0d8184616b5a565b9050949350505050565b5f608082019050616c2a5f830189615ab4565b8181036020830152616c3d8187896167dc565b90508181036040830152616c528185876167dc565b9050616c616060830184615ac3565b979650505050505050565b5f606082019050616c7f5f830187615ab4565b8181036020830152616c918186615c9b565b90508181036040830152616ca68184866167dc565b905095945050505050565b7f4549503731323a20556e696e697469616c697a656400000000000000000000005f82015250565b5f616ce5601583615332565b9150616cf082616cb1565b602082019050919050565b5f6020820190508181035f830152616d1281616cd9565b9050919050565b5f8083356001602003843603038112616d3557616d3461688a565b5b80840192508235915067ffffffffffffffff821115616d5757616d5661688e565b5b602083019250602082023603831315616d7357616d72616892565b5b509250929050565b5f8083356001602003843603038112616d9757616d9661688a565b5b80840192508235915067ffffffffffffffff821115616db957616db861688e565b5b602083019250602082023603831315616dd557616dd4616892565b5b509250929050565b5f606082019050616df05f830186615ab4565b616dfd60208301856155ac565b8181036040830152616e0f8184615f63565b9050949350505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f60ff82169050919050565b5f8160f81b9050919050565b5f616e6882616e52565b9050919050565b616e80616e7b82616e46565b616e5e565b82525050565b5f819050919050565b616ea0616e9b826153e3565b616e86565b82525050565b5f616eb18286616e6f565b600182019150616ec18285616e8f565b602082019150616ed18284616e8f565b602082019150819050949350505050565b5f60208284031215616ef757616ef66153db565b5b5f616f0484828501616614565b91505092915050565b5f81905092915050565b5f616f228385616f0d565b9350616f2f838584615852565b82840190509392505050565b5f616f47828486616f17565b91508190509392505050565b616f5c81615d5d565b82525050565b5f606082019050616f755f830186615931565b616f826020830185616f53565b616f8f6040830184615931565b949350505050565b5f81519050919050565b5f81905092915050565b5f819050602082019050919050565b616fc381615928565b82525050565b5f616fd48383616fba565b60208301905092915050565b5f602082019050919050565b5f616ff682616f97565b6170008185616fa1565b935061700b83616fab565b805f5b8381101561703b5781516170228882616fc9565b975061702d83616fe0565b92505060018101905061700e565b5085935050505092915050565b5f6170538284616fec565b915081905092915050565b5f60a0820190506170715f830188615931565b61707e6020830187615ab4565b61708b6040830186615ab4565b6170986060830185615931565b6170a56080830184615931565b9695505050505050565b6170b881616e46565b82525050565b5f6020820190506170d15f8301846170af565b92915050565b5f80fd5b5f80fd5b5f67ffffffffffffffff8211156170f9576170f86157aa565b5b6171028261536a565b9050602081019050919050565b5f61712161711c846170df565b615808565b90508281526020810184848401111561713d5761713c6157a6565b5b617148848285615342565b509392505050565b5f82601f83011261716457617163615635565b5b815161717484826020860161710f565b91505092915050565b5f60808284031215617192576171916170d7565b5b61719c6080615808565b90505f6171ab84828501616127565b5f8301525060206171be84828501616127565b602083015250604082015167ffffffffffffffff8111156171e2576171e16170db565b5b6171ee84828501617150565b604083015250606082015167ffffffffffffffff811115617212576172116170db565b5b61721e84828501617150565b60608301525092915050565b5f6020828403121561723f5761723e6153db565b5b5f82015167ffffffffffffffff81111561725c5761725b6153df565b5b6172688482850161717d565b91505092915050565b61727a81615928565b8114617284575f80fd5b50565b5f8151905061729581617271565b92915050565b5f602082840312156172b0576172af6153db565b5b5f6172bd84828501617287565b91505092915050565b5f6040820190506172d95f830185615931565b6172e66020830184615ab4565b9392505050565b5f60208284031215617302576173016153db565b5b5f61730f84828501615792565b91505092915050565b5f819050815f5260205f209050919050565b601f82111561736b5761733c81617318565b617345846162dc565b81016020851015617354578190505b617368617360856162dc565b8301826163bc565b50505b505050565b61737982615328565b67ffffffffffffffff811115617392576173916157aa565b5b61739c825461629a565b6173a782828561732a565b5f60209050601f8311600181146173d8575f84156173c6578287015190505b6173d0858261644c565b865550617437565b601f1984166173e686617318565b5f5b8281101561740d578489015182556001820191506020850194506020810190506173e8565b8683101561742a5784890151617426601f891682616430565b8355505b6001600288020188555050505b505050505050565b5f6040820190506174525f830185615ac3565b61745f6020830184615ac3565b9392505050565b5f61747082615d7d565b61747a8185616f0d565b935061748a818560208601615342565b80840191505092915050565b5f6174a18284617466565b915081905092915050565b5f60a0820190506174bf5f830188615931565b6174cc6020830187615931565b6174d96040830186615931565b6174e66060830185615ab4565b6174f36080830184615ac3565b9695505050505050565b5f6080820190506175105f830187615931565b61751d60208301866170af565b61752a6040830185615931565b6175376060830184615931565b9594505050505056fe507265704b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e49642943727367656e566572696669636174696f6e2875696e743235362063727349642c75696e74323536206d61784269744c656e6774682c6279746573206372734469676573742c627974657320657874726144617461294b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e49642c75696e74323536206b657949642c4b65794469676573745b5d206b6579446967657374732c627974657320657874726144617461294b65794469676573742875696e7438206b6579547970652c627974657320646967657374294b65794469676573742875696e7438206b6579547970652c62797465732064696765737429
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP4\x80\x15b\0\0CW_\x80\xFD[Pb\0\0Tb\0\0Z` \x1B` \x1CV[b\0\x01\xC4V[_b\0\0kb\0\x01^` \x1B` \x1CV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15b\0\0\xB6W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14b\0\x01[Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Qb\0\x01R\x91\x90b\0\x01\xA9V[`@Q\x80\x91\x03\x90\xA1[PV[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[b\0\x01\xA3\x81b\0\x01\x85V[\x82RPPV[_` \x82\x01\x90Pb\0\x01\xBE_\x83\x01\x84b\0\x01\x98V[\x92\x91PPV[`\x80Qavjb\0\x01\xEB_9_\x81\x81a<\xF0\x01R\x81\x81a=E\x01Ra?\xE7\x01Ravj_\xF3\xFE`\x80`@R`\x046\x10a\x014W_5`\xE0\x1C\x80cb\x97\x87\x87\x11a\0\xAAW\x80c\xAD<\xB1\xCC\x11a\0nW\x80c\xAD<\xB1\xCC\x14a\x03\xF9W\x80c\xBA\xFF!\x1E\x14a\x04#W\x80c\xC2\xC1\xFA\xEE\x14a\x04MW\x80c\xC5[\x87$\x14a\x04uW\x80c\xCA\xA3g\xDB\x14a\x04\xB2W\x80c\xD5/\x10\xEB\x14a\x04\xDAWa\x014V[\x80cb\x97\x87\x87\x14a\x03\x12W\x80cs[\x12c\x14a\x03:W\x80c\x84\xB0\x19n\x14a\x03dW\x80c\x93f\x08\xAE\x14a\x03\x94W\x80c\xA0\x07\x9E\x0F\x14a\x03\xD1Wa\x014V[\x80c<\x02\xF84\x11a\0\xFCW\x80c<\x02\xF84\x14a\x02\x18W\x80cE\xAF&\x1B\x14a\x02@W\x80cF\x10\xFF\xE8\x14a\x02|W\x80cO\x1E\xF2\x86\x14a\x02\xA4W\x80cR\xD1\x90-\x14a\x02\xC0W\x80cX\x9A\xDB\x0E\x14a\x02\xEAWa\x014V[\x80c\r\x8En,\x14a\x018W\x80c\x16\xC7\x13\xD9\x14a\x01bW\x80c\x17\x03\xC6\x1A\x14a\x01\x9EW\x80c\x19\xF4\xF62\x14a\x01\xC6W\x80c9\xF78\x10\x14a\x02\x02W[_\x80\xFD[4\x80\x15a\x01CW_\x80\xFD[Pa\x01La\x05\x04V[`@Qa\x01Y\x91\x90aS\xB2V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01mW_\x80\xFD[Pa\x01\x88`\x04\x806\x03\x81\x01\x90a\x01\x83\x91\x90aT\x16V[a\x05\x7FV[`@Qa\x01\x95\x91\x90aU(V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xA9W_\x80\xFD[Pa\x01\xC4`\x04\x806\x03\x81\x01\x90a\x01\xBF\x91\x90aT\x16V[a\x06PV[\0[4\x80\x15a\x01\xD1W_\x80\xFD[Pa\x01\xEC`\x04\x806\x03\x81\x01\x90a\x01\xE7\x91\x90aT\x16V[a\x08oV[`@Qa\x01\xF9\x91\x90aU\xBBV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\rW_\x80\xFD[Pa\x02\x16a\t\x1CV[\0[4\x80\x15a\x02#W_\x80\xFD[Pa\x02>`\x04\x806\x03\x81\x01\x90a\x029\x91\x90aU\xF7V[a\x0B?V[\0[4\x80\x15a\x02KW_\x80\xFD[Pa\x02f`\x04\x806\x03\x81\x01\x90a\x02a\x91\x90aT\x16V[a\x0E#V[`@Qa\x02s\x91\x90aU\xBBV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x87W_\x80\xFD[Pa\x02\xA2`\x04\x806\x03\x81\x01\x90a\x02\x9D\x91\x90aV\xEBV[a\x0E\xB8V[\0[a\x02\xBE`\x04\x806\x03\x81\x01\x90a\x02\xB9\x91\x90aX\xCEV[a\x169V[\0[4\x80\x15a\x02\xCBW_\x80\xFD[Pa\x02\xD4a\x16XV[`@Qa\x02\xE1\x91\x90aY@V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xF5W_\x80\xFD[Pa\x03\x10`\x04\x806\x03\x81\x01\x90a\x03\x0B\x91\x90aYYV[a\x16\x89V[\0[4\x80\x15a\x03\x1DW_\x80\xFD[Pa\x038`\x04\x806\x03\x81\x01\x90a\x033\x91\x90aY\xB6V[a\x1B\x0FV[\0[4\x80\x15a\x03EW_\x80\xFD[Pa\x03Na!\x8FV[`@Qa\x03[\x91\x90aZaV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03oW_\x80\xFD[Pa\x03xa\"@V[`@Qa\x03\x8B\x97\x96\x95\x94\x93\x92\x91\x90a[\x89V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x9FW_\x80\xFD[Pa\x03\xBA`\x04\x806\x03\x81\x01\x90a\x03\xB5\x91\x90aT\x16V[a#IV[`@Qa\x03\xC8\x92\x91\x90a^\x9BV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xDCW_\x80\xFD[Pa\x03\xF7`\x04\x806\x03\x81\x01\x90a\x03\xF2\x91\x90a^\xF3V[a&iV[\0[4\x80\x15a\x04\x04W_\x80\xFD[Pa\x04\ra-VV[`@Qa\x04\x1A\x91\x90aS\xB2V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04.W_\x80\xFD[Pa\x047a-\x8FV[`@Qa\x04D\x91\x90a_:V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04XW_\x80\xFD[Pa\x04s`\x04\x806\x03\x81\x01\x90a\x04n\x91\x90aT\x16V[a-\xA6V[\0[4\x80\x15a\x04\x80W_\x80\xFD[Pa\x04\x9B`\x04\x806\x03\x81\x01\x90a\x04\x96\x91\x90aT\x16V[a0\x10V[`@Qa\x04\xA9\x92\x91\x90a_\x9BV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\xBDW_\x80\xFD[Pa\x04\xD8`\x04\x806\x03\x81\x01\x90a\x04\xD3\x91\x90a_\xD0V[a2\x9BV[\0[4\x80\x15a\x04\xE5W_\x80\xFD[Pa\x04\xEEa5\xD7V[`@Qa\x04\xFB\x91\x90a_:V[`@Q\x80\x91\x03\x90\xF3[```@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x05E_a5\xEEV[a\x05O`\x01a5\xEEV[a\x05X_a5\xEEV[`@Q` \x01a\x05k\x94\x93\x92\x91\x90a`\xC9V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[``_a\x05\x8Aa6\xB8V[\x90P_\x81`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\x02\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x82\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x06BW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x05\xF9W[PPPPP\x92PPP\x91\x90PV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xADW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xD1\x91\x90aa;V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x07@W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x077\x91\x90aafV[`@Q\x80\x91\x03\x90\xFD[_a\x07Ia6\xB8V[\x90P\x80`\t\x01T\x82\x11\x80a\x07eWP`\xF8`\x05`\xFF\x16\x90\x1B\x82\x11\x15[\x15a\x07\xA7W\x81`@Q\x7F\xCB\xE9&V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\x9E\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x08\tW\x81`@Q\x7F\xDF\r\xB5\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\0\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[`\x01\x81`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F8O\x90\xFE\xFB\xCF\xAAh\xF2.\0\tJ\xEA\xA5++\xC6\x93\x93m,\xE1\xAF\xED\x12\x12R\x0BY\xB5\x8E\x82`@Qa\x08c\x91\x90a_:V[`@Q\x80\x91\x03\x90\xA1PPV[_\x80a\x08ya6\xB8V[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x08\xDCW\x82`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xD3\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\r\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x92PPP\x91\x90PV[`\x01a\t&a6\xDFV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\tgW`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02_a\tra7\x03V[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\t\xBAWP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\t\xF1W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\n\xAA`@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa7*V[_a\n\xB3a6\xB8V[\x90P`\xF8`\x03`\xFF\x16\x90\x1B\x81`\x04\x01\x81\x90UP`\xF8`\x04`\xFF\x16\x90\x1B\x81`\x05\x01\x81\x90UP`\xF8`\x05`\xFF\x16\x90\x1B\x81`\t\x01\x81\x90UPP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x0B3\x91\x90aa\xA1V[`@Q\x80\x91\x03\x90\xA1PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x9CW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xC0\x91\x90aa;V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0C/W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C&\x91\x90aafV[`@Q\x80\x91\x03\x90\xFD[_a\x0C8a6\xB8V[\x90P_\x81`\t\x01T\x90P`\xF8`\x05`\xFF\x16\x90\x1B\x81\x14\x15\x80\x15a\x0CwWP\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15a\x0C\xB9W\x80`@Q\x7F\x06\x1A\xC6\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\xB0\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[\x81`\t\x01_\x81T\x80\x92\x91\x90a\x0C\xCD\x90aa\xE7V[\x91\x90PUP_\x82`\t\x01T\x90P\x84\x83`\n\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x83\x83`\r\x01_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a\r'Wa\r&aUHV[[\x02\x17\x90UP_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x8AW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xAE\x91\x90abBV[\x90P_a\r\xBA\x82a7@V[\x90P\x80\x85`\x0E\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x90\x81a\r\xDC\x91\x90adgV[P\x7F\x8C\xF0\x15\x13\x93\xF8O\xD6\x94\xC5\xE3\x15\xCBt\xCC\x05\xB2G\xDE\nEO\xD9\xE9\x12\x9Cf\x1E\xFD\xF9@\x1D\x83\x88\x88\x84`@Qa\x0E\x12\x94\x93\x92\x91\x90ae6V[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[_\x80a\x0E-a6\xB8V[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x0E\x90W\x82`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\x87\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[\x80`\r\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x16W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F:\x91\x90abBV[\x90PsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cF\xC5\xBB\xBD\x823`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\x8B\x92\x91\x90ae\x80V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xA6W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xCA\x91\x90ae\xD1V[a\x10\x0BW3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\x02\x91\x90aafV[`@Q\x80\x91\x03\x90\xFD[_a\x10\x14a6\xB8V[\x90P\x80`\x05\x01T\x87\x11\x80a\x100WP`\xF8`\x04`\xFF\x16\x90\x1B\x87\x11\x15[\x15a\x10rW\x86`@Q\x7F\xAD\xFA\xB9\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10i\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[_\x86\x86\x90P\x03a\x10\xB9W\x86`@Q\x7F\xE6\xF9\x08;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\xB0\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x89\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x11&W`@Q\x7Fo\xBC\xDD+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x11\xCD\x82\x8A\x8A\x8A\x87`\x0E\x01_\x8F\x81R` \x01\x90\x81R` \x01_ \x80Ta\x11L\x90ab\x9AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x11x\x90ab\x9AV[\x80\x15a\x11\xC3W\x80`\x1F\x10a\x11\x9AWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x11\xC3V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x11\xA6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa7nV[\x90P_a\x11\xDB\x82\x88\x88a9OV[\x90P\x83_\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x12{W\x89\x81`@Q\x7F\x98\xFB\x95}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12r\x92\x91\x90ae\x80V[`@Q\x80\x91\x03\x90\xFD[`\x01\x84_\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x84`\x02\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_\x81\x80T\x90P\x90P\x7F*\xFEd\xFB:\xFD\xE8\xE2g\x8A\xEA\x84\xCF6\"?3\x0E/\xB1(m7\xAE\xD5s\xAB\x9C\xD1\xDBG\xC7\x8C\x8C\x8C\x8C\x8C3`@Qa\x13\xA5\x96\x95\x94\x93\x92\x91\x90ah\x08V[`@Q\x80\x91\x03\x90\xA1\x85`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x13\xDFWPa\x13\xDE\x81a9\xB5V[[\x15a\x16+W`\x01\x86`\x01\x01_\x8E\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_[\x8B\x8B\x90P\x81\x10\x15a\x14\x95W\x86`\x07\x01_\x8E\x81R` \x01\x90\x81R` \x01_ \x8C\x8C\x83\x81\x81\x10a\x14BWa\x14Aah]V[[\x90P` \x02\x81\x01\x90a\x14T\x91\x90ah\x96V[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P\x81\x81a\x14\x86\x91\x90aj\xC9V[PP\x80\x80`\x01\x01\x91PPa\x14\x11V[P\x83\x86`\x03\x01_\x8E\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x8B\x86`\x08\x01\x81\x90UP_a\x15Z\x87`\x0E\x01_\x8F\x81R` \x01\x90\x81R` \x01_ \x80Ta\x14\xD9\x90ab\x9AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15\x05\x90ab\x9AV[\x80\x15a\x15PW\x80`\x1F\x10a\x15'Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15PV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x153W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa:FV[\x90P_a\x15\xE9\x82\x85\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x15\xDFW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x15\x96W[PPPPPa;\xA6V[\x90P\x7F\xEB\x85\xC2m\xBC\xADF\xB8\nh\xA0\xF2L\xCE|,\x90\xF0\xA1\xFA\xDE\xD8A\x84\x13\x889\xFC\x9E\x80\xA2[\x8E\x82\x8F\x8F`@Qa\x16 \x94\x93\x92\x91\x90aj\xD7V[`@Q\x80\x91\x03\x90\xA1PP[PPPPPPPPPPPPV[a\x16Aa<\xEEV[a\x16J\x82a=\xD4V[a\x16T\x82\x82a>\xC7V[PPV[_a\x16aa?\xE5V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xE7W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x0B\x91\x90abBV[\x90PsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cF\xC5\xBB\xBD\x823`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\\\x92\x91\x90ae\x80V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17wW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x9B\x91\x90ae\xD1V[a\x17\xDCW3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17\xD3\x91\x90aafV[`@Q\x80\x91\x03\x90\xFD[_a\x17\xE5a6\xB8V[\x90P\x80`\x04\x01T\x85\x11\x80a\x18\x01WP`\xF8`\x03`\xFF\x16\x90\x1B\x85\x11\x15[\x15a\x18CW\x84`@Q\x7F\n\xB7\xF6\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18:\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[_a\x18M\x86a@lV[\x90P_a\x18[\x82\x87\x87a9OV[\x90P\x82_\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x18\xFBW\x86\x81`@Q\x7F3\xCA\x1F\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18\xF2\x92\x91\x90ae\x80V[`@Q\x80\x91\x03\x90\xFD[`\x01\x83_\x01_\x89\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x83`\x02\x01_\x89\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7FLq\\W4\xCE\\\x18\xC9\xC1.\x84\x96\xE5=*e\xF1\xEC8\x1DGiW\xF0\xF5\x96\xB3d\xA5\x9B\x0C\x88\x88\x883`@Qa\x1A\x19\x94\x93\x92\x91\x90ak\x1CV[`@Q\x80\x91\x03\x90\xA1\x83`\x01\x01_\x89\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x1AWWPa\x1AV\x81\x80T\x90Pa9\xB5V[[\x15a\x1B\x05W`\x01\x84`\x01\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x82\x84`\x03\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x81\x90UP_\x84`\x06\x01_\x8A\x81R` \x01\x90\x81R` \x01_ T\x90P\x7F:\x11a \xCC\xA5\xD4\xF0s\xCC\x1F\xC3\x1F\xF2a3\xAB{\x04\x99\xF2q/\xA0\x10\x02;\x87\xD5\xA1\xF9\xEE\x89\x82\x87`\x0E\x01_\x8D\x81R` \x01\x90\x81R` \x01_ `@Qa\x1A\xFB\x93\x92\x91\x90ak\xDBV[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPPV[_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BmW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\x91\x91\x90abBV[\x90PsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cF\xC5\xBB\xBD\x823`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B\xE2\x92\x91\x90ae\x80V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xFDW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C!\x91\x90ae\xD1V[a\x1CbW3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1CY\x91\x90aafV[`@Q\x80\x91\x03\x90\xFD[_a\x1Cka6\xB8V[\x90P\x80`\t\x01T\x87\x11\x80a\x1C\x87WP`\xF8`\x05`\xFF\x16\x90\x1B\x87\x11\x15[\x15a\x1C\xC9W\x86`@Q\x7F\x8D\x8C\x94\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1C\xC0\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[_\x81`\n\x01_\x89\x81R` \x01\x90\x81R` \x01_ T\x90P_a\x1D\x87\x89\x83\x8A\x8A\x87`\x0E\x01_\x8F\x81R` \x01\x90\x81R` \x01_ \x80Ta\x1D\x06\x90ab\x9AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1D2\x90ab\x9AV[\x80\x15a\x1D}W\x80`\x1F\x10a\x1DTWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1D}V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1D`W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa@\xC4V[\x90P_a\x1D\x95\x82\x88\x88a9OV[\x90P\x83_\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x1E5W\x89\x81`@Q\x7F\xFC\xF5\xA6\xE9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1E,\x92\x91\x90ae\x80V[`@Q\x80\x91\x03\x90\xFD[`\x01\x84_\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x84`\x02\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_\x81\x80T\x90P\x90P\x7F{\xF1\xB4,\x10\xE9I|\x87\x96 \xC5\xB7\xAF\xCE\xD1\x0B\xDA\x17\xD8\xC9\x0B\"\xF0\xE3\xBCk/\xD6\xCE\xD0\xBD\x8C\x8C\x8C\x8C\x8C3`@Qa\x1F_\x96\x95\x94\x93\x92\x91\x90al\x17V[`@Q\x80\x91\x03\x90\xA1\x85`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x1F\x99WPa\x1F\x98\x81a9\xB5V[[\x15a!\x81W`\x01\x86`\x01\x01_\x8E\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x8A\x8A\x87`\x0B\x01_\x8F\x81R` \x01\x90\x81R` \x01_ \x91\x82a\x1F\xEB\x92\x91\x90ai\xA8V[P\x83\x86`\x03\x01_\x8E\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x8B\x86`\x0C\x01\x81\x90UP_a \xB0\x87`\x0E\x01_\x8F\x81R` \x01\x90\x81R` \x01_ \x80Ta /\x90ab\x9AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta [\x90ab\x9AV[\x80\x15a \xA6W\x80`\x1F\x10a }Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a \xA6V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a \x89W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa:FV[\x90P_a!?\x82\x85\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a!5W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a \xECW[PPPPPa;\xA6V[\x90P\x7F\"X\xB7?\xAE\xD3?\xB2\xE2\xEAED\x03\xBE\xF9t\x92\x0C\xAFh*\xB3\xA7#HO\xCFgU;\x16\xA2\x8E\x82\x8F\x8F`@Qa!v\x94\x93\x92\x91\x90allV[`@Q\x80\x91\x03\x90\xA1PP[PPPPPPPPPPPPV[_\x80a!\x99a6\xB8V[\x90P_\x81`\x05\x01T\x90P`\xF8`\x04`\xFF\x16\x90\x1B\x81\x14\x15\x80\x15a!\xD8WP\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15a!\xE8W`\x01\x92PPPa\"=V[_\x82`\t\x01T\x90P`\xF8`\x05`\xFF\x16\x90\x1B\x81\x14\x15\x80\x15a\"%WP\x82`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15a\"6W`\x01\x93PPPPa\"=V[_\x93PPPP[\x90V[_``\x80_\x80_``_a\"RaAUV[\x90P_\x80\x1B\x81_\x01T\x14\x80\x15a\"mWP_\x80\x1B\x81`\x01\x01T\x14[a\"\xACW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\"\xA3\x90al\xFBV[`@Q\x80\x91\x03\x90\xFD[a\"\xB4aA|V[a\"\xBCaB\x1AV[F0_\x80\x1B_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\xDBWa\"\xDAaW\xAAV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a#\tW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94\x93\x92\x91\x90\x97P\x97P\x97P\x97P\x97P\x97P\x97PP\x90\x91\x92\x93\x94\x95\x96V[``\x80_a#Ua6\xB8V[\x90P\x80`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a#\xB8W\x83`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a#\xAF\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x82`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a$oW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a$&W[PPPPP\x90P_a%\x19\x84`\x0E\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x80Ta$\x98\x90ab\x9AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta$\xC4\x90ab\x9AV[\x80\x15a%\x0FW\x80`\x1F\x10a$\xE6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a%\x0FV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a$\xF2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa:FV[\x90P_a%&\x82\x84a;\xA6V[\x90P\x80\x85`\x07\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a&UW\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x01\x81\x11\x15a%\xA0Wa%\x9FaUHV[[`\x01\x81\x11\x15a%\xB2Wa%\xB1aUHV[[\x81R` \x01`\x01\x82\x01\x80Ta%\xC6\x90ab\x9AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta%\xF2\x90ab\x9AV[\x80\x15a&=W\x80`\x1F\x10a&\x14Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a&=V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a& W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a%\\V[PPPP\x90P\x96P\x96PPPPPP\x91P\x91V[`\x01a&sa6\xDFV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a&\xB4W`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02_a&\xBFa7\x03V[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a'\x07WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a'>W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa'\xF7`@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa7*V[_a(\0a6\xB8V[\x90Pa(\x0B\x84aB\xB8V[a(5\x84``\x015\x85\x80a\x01\0\x01\x90a($\x91\x90am\x19V[\x87a\x01 \x015\x88a\x02 \x015aC\xC8V[a(_\x84`\x80\x015\x85\x80a\x01@\x01\x90a(N\x91\x90am\x19V[\x87a\x01`\x015\x88a\x02 \x015aC\xC8V[a(\x89\x84`\xA0\x015\x85\x80a\x01\x80\x01\x90a(x\x91\x90am\x19V[\x87a\x01\xA0\x015\x88a\x02 \x015aC\xC8V[_\x84\x80`\xC0\x01\x90a(\x9A\x91\x90am{V[\x90P\x03a(\xE2W\x83``\x015`@Q\x7F\x16\xBB\xAF\x8D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a(\xD9\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[_\x84\x80`\xE0\x01\x90a(\xF3\x91\x90ai<V[\x90P\x03a);W\x83`\x80\x015`@Q\x7F\x16\xBB\xAF\x8D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a)2\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[\x83_\x015\x81`\x04\x01\x81\x90UP\x83` \x015\x81`\x05\x01\x81\x90UP\x83`@\x015\x81`\t\x01\x81\x90UP\x83``\x015\x81`\x08\x01\x81\x90UP\x83`\x80\x015\x81`\x0C\x01\x81\x90UP\x83``\x015\x81`\x06\x01_\x86`\xA0\x015\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x83`\xA0\x015\x81`\x06\x01_\x86``\x015\x81R` \x01\x90\x81R` \x01_ \x81\x90UP_[\x84\x80`\xC0\x01\x90a)\xCD\x91\x90am{V[\x90P\x81\x10\x15a*aW\x81`\x07\x01_\x86``\x015\x81R` \x01\x90\x81R` \x01_ \x85\x80`\xC0\x01\x90a)\xFD\x91\x90am{V[\x83\x81\x81\x10a*\x0EWa*\rah]V[[\x90P` \x02\x81\x01\x90a* \x91\x90ah\x96V[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P\x81\x81a*R\x91\x90aj\xC9V[PP\x80\x80`\x01\x01\x91PPa)\xBDV[P\x83\x80`\xE0\x01\x90a*r\x91\x90ai<V[\x82`\x0B\x01_\x87`\x80\x015\x81R` \x01\x90\x81R` \x01_ \x91\x82a*\x96\x92\x91\x90ai\xA8V[P\x83a\x01\xC0\x015\x81`\n\x01_\x86`\x80\x015\x81R` \x01\x90\x81R` \x01_ \x81\x90UPa*\xE3\x81\x85``\x015\x86\x80a\x01\0\x01\x90a*\xD2\x91\x90am\x19V[\x88a\x01 \x015\x89a\x02 \x015aE;V[a+\x0E\x81\x85`\x80\x015\x86\x80a\x01@\x01\x90a*\xFD\x91\x90am\x19V[\x88a\x01`\x015\x89a\x02 \x015aE;V[a+9\x81\x85`\xA0\x015\x86\x80a\x01\x80\x01\x90a+(\x91\x90am\x19V[\x88a\x01\xA0\x015\x89a\x02 \x015aE;V[`\x01\x81`\x01\x01_\x86`\xA0\x015\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01\x81`\x01\x01_\x86``\x015\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01\x81`\x01\x01_\x86`\x80\x015\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x83a\x01\xE0\x01` \x81\x01\x90a+\xDA\x91\x90a_\xD0V[\x81`\r\x01_\x86`\xA0\x015\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a,\x12Wa,\x11aUHV[[\x02\x17\x90UP\x83a\x02\0\x01` \x81\x01\x90a,+\x91\x90a_\xD0V[\x81`\r\x01_\x86`\x80\x015\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a,cWa,baUHV[[\x02\x17\x90UPa,v\x84a\x02 \x015a7@V[\x81`\x0E\x01_\x86`\xA0\x015\x81R` \x01\x90\x81R` \x01_ \x90\x81a,\x99\x91\x90adgV[Pa,\xA8\x84a\x02 \x015a7@V[\x81`\x0E\x01_\x86``\x015\x81R` \x01\x90\x81R` \x01_ \x90\x81a,\xCB\x91\x90adgV[Pa,\xDA\x84a\x02 \x015a7@V[\x81`\x0E\x01_\x86`\x80\x015\x81R` \x01\x90\x81R` \x01_ \x90\x81a,\xFD\x91\x90adgV[PP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa-I\x91\x90aa\xA1V[`@Q\x80\x91\x03\x90\xA1PPPV[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[_\x80a-\x99a6\xB8V[\x90P\x80`\x0C\x01T\x91PP\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.\x03W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.'\x91\x90aa;V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a.\x96W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.\x8D\x91\x90aafV[`@Q\x80\x91\x03\x90\xFD[_a.\x9Fa6\xB8V[\x90P\x80`\x05\x01T\x82\x11\x80a.\xBBWP`\xF8`\x04`\xFF\x16\x90\x1B\x82\x11\x15[\x15a.\xFDW\x81`@Q\x7F\xFC\xF2\xDBz\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.\xF4\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a/_W\x81`@Q\x7F\x92x\x9Bg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a/V\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[`\x01\x81`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x81`\x06\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P_\x81\x14a/\xD4W`\x01\x82`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP[\x7F+\x08{\x88K5\xA8\x1Dv\x9D\x1A\x1E\t(\x80\xF1\xDAV\xDE\x96NK3\x9E\xAB\xCB\x1FE\xF5\xFE2d\x83`@Qa0\x03\x91\x90a_:V[`@Q\x80\x91\x03\x90\xA1PPPV[``\x80_a0\x1Ca6\xB8V[\x90P\x80`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a0\x7FW\x83`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a0v\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x82`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a16W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a0\xEDW[PPPPP\x90P_a1\xE0\x84`\x0E\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x80Ta1_\x90ab\x9AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta1\x8B\x90ab\x9AV[\x80\x15a1\xD6W\x80`\x1F\x10a1\xADWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a1\xD6V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a1\xB9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa:FV[\x90P_a1\xED\x82\x84a;\xA6V[\x90P\x80\x85`\x0B\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80\x80Ta2\x0F\x90ab\x9AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta2;\x90ab\x9AV[\x80\x15a2\x86W\x80`\x1F\x10a2]Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a2\x86V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a2iW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x96P\x96PPPPPP\x91P\x91V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2\xF8W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\x1C\x91\x90aa;V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a3\x8BW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a3\x82\x91\x90aafV[`@Q\x80\x91\x03\x90\xFD[_a3\x94a6\xB8V[\x90P_\x81`\x05\x01T\x90P`\xF8`\x04`\xFF\x16\x90\x1B\x81\x14\x15\x80\x15a3\xD3WP\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15a4\x15W\x80`@Q\x7F;\x85=\xA8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a4\x0C\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[\x81`\x04\x01_\x81T\x80\x92\x91\x90a4)\x90aa\xE7V[\x91\x90PUP_\x82`\x04\x01T\x90P\x82`\x05\x01_\x81T\x80\x92\x91\x90a4J\x90aa\xE7V[\x91\x90PUP_\x83`\x05\x01T\x90P\x80\x84`\x06\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x81\x84`\x06\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x84\x84`\r\x01_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a4\xBCWa4\xBBaUHV[[\x02\x17\x90UP_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5\x1FW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5C\x91\x90abBV[\x90P_a5O\x82a7@V[\x90P\x80\x86`\x0E\x01_\x86\x81R` \x01\x90\x81R` \x01_ \x90\x81a5q\x91\x90adgV[P\x80\x86`\x0E\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x90\x81a5\x92\x91\x90adgV[P\x7F\xFB\xF5'H\x10\xB9O\x86\x97\x0C\x11G\xE8\xFF\xAE\xBE\xD2F\xEE\x97w\xD6\x95\xA6\x90\x04\xDCbV\xD1\xFE\x91\x84\x88\x83`@Qa5\xC6\x93\x92\x91\x90am\xDDV[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[_\x80a5\xE1a6\xB8V[\x90P\x80`\x08\x01T\x91PP\x90V[``_`\x01a5\xFC\x84aG V[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\x1AWa6\x19aW\xAAV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a6LW\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x01\x82\x01\x90P[`\x01\x15a6\xADW\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a6\xA2Wa6\xA1an\x19V[[\x04\x94P_\x85\x03a6YW[\x81\x93PPPP\x91\x90PV[_\x7F&\xFD\xAF\x8A,\xB2\r \xB5^6!\x89\x86\x90^SN\xE7\xA9p\xDD/\xA8'\x94nKt\x96\xDB\0\x90P\x90V[_a6\xE8a7\x03V[_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[a72aHqV[a7<\x82\x82aH\xB1V[PPV[```\x02\x82_`@Q` \x01a7X\x93\x92\x91\x90an\xA6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[_\x80\x84\x84\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7\x8DWa7\x8CaW\xAAV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a7\xBBW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_[\x85\x85\x90P\x81\x10\x15a8\xBFW`@Q\x80``\x01`@R\x80`%\x81R` \x01avE`%\x919\x80Q\x90` \x01 \x86\x86\x83\x81\x81\x10a7\xFEWa7\xFDah]V[[\x90P` \x02\x81\x01\x90a8\x10\x91\x90ah\x96V[_\x01` \x81\x01\x90a8!\x91\x90an\xE2V[\x87\x87\x84\x81\x81\x10a84Wa83ah]V[[\x90P` \x02\x81\x01\x90a8F\x91\x90ah\x96V[\x80` \x01\x90a8U\x91\x90ai<V[`@Qa8c\x92\x91\x90ao;V[`@Q\x80\x91\x03\x90 `@Q` \x01a8}\x93\x92\x91\x90aobV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x82\x81Q\x81\x10a8\xA6Wa8\xA5ah]V[[` \x02` \x01\x01\x81\x81RPP\x80\x80`\x01\x01\x91PPa7\xC0V[Pa9C`@Q\x80`\xC0\x01`@R\x80`\x82\x81R` \x01au\xC3`\x82\x919\x80Q\x90` \x01 \x88\x88\x84`@Q` \x01a8\xF6\x91\x90apHV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x87\x80Q\x90` \x01 `@Q` \x01a9(\x95\x94\x93\x92\x91\x90ap^V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 aI\x02V[\x91PP\x95\x94PPPPPV[_\x80a9\x9E\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPaI\x1BV[\x90Pa9\xAA\x813aIEV[\x80\x91PP\x93\x92PPPV[_\x80sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xB4r+\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:\x14W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:8\x91\x90abBV[\x90P\x80\x83\x10\x15\x91PP\x91\x90PV[_\x80\x82Q\x03a:\xD7WsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:\xACW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:\xD0\x91\x90abBV[\x90Pa;\xA1V[_\x82_\x81Q\x81\x10a:\xEBWa:\xEAah]V[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P`\x01`\xFF\x16\x81`\xFF\x16\x14\x80a;\x16WP`\x02`\xFF\x16\x81`\xFF\x16\x14[\x15a;dW`!\x83Q\x10\x15a;WW`@Q\x7F\x8B$\x8B`\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`!\x83\x01Q\x91PPa;\xA1V[\x80`@Q\x7F!9\xCC,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a;\x98\x91\x90ap\xBEV[`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[``_\x82Q\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a;\xC8Wa;\xC7aW\xAAV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a;\xFBW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a;\xE6W\x90P[P\x90P_[\x82\x81\x10\x15a<\xE2WsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c1\xFFA\xC8\x87\x87\x84\x81Q\x81\x10a<LWa<Kah]V[[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a<q\x92\x91\x90ae\x80V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<\x8BW=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<\xB3\x91\x90ar*V[``\x01Q\x82\x82\x81Q\x81\x10a<\xCAWa<\xC9ah]V[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa<\0V[P\x80\x92PPP\x92\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a=\x9BWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a=\x82aK\xAAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a=\xD2W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>1W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>U\x91\x90aa;V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a>\xC4W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a>\xBB\x91\x90aafV[`@Q\x80\x91\x03\x90\xFD[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a?/WP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a?,\x91\x90ar\x9BV[`\x01[a?pW\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a?g\x91\x90aafV[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14a?\xD6W\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a?\xCD\x91\x90aY@V[`@Q\x80\x91\x03\x90\xFD[a?\xE0\x83\x83aK\xFDV[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a@jW`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_a@\xBD`@Q\x80``\x01`@R\x80`,\x81R` \x01auA`,\x919\x80Q\x90` \x01 \x83`@Q` \x01a@\xA2\x92\x91\x90ar\xC6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 aI\x02V[\x90P\x91\x90PV[_aAJ`@Q\x80`\x80\x01`@R\x80`V\x81R` \x01aum`V\x919\x80Q\x90` \x01 \x87\x87\x87\x87`@Q` \x01a@\xFD\x92\x91\x90ao;V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x86\x80Q\x90` \x01 `@Q` \x01aA/\x95\x94\x93\x92\x91\x90ap^V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 aI\x02V[\x90P\x95\x94PPPPPV[_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x90P\x90V[``_aA\x87aAUV[\x90P\x80`\x02\x01\x80TaA\x98\x90ab\x9AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80TaA\xC4\x90ab\x9AV[\x80\x15aB\x0FW\x80`\x1F\x10aA\xE6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91aB\x0FV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11aA\xF2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[``_aB%aAUV[\x90P\x80`\x03\x01\x80TaB6\x90ab\x9AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80TaBb\x90ab\x9AV[\x80\x15aB\xADW\x80`\x1F\x10aB\x84Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91aB\xADV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11aB\x90W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[`\xF8`\x03`\xFF\x16\x90\x1B\x81`\xA0\x015\x11\x15\x80aB\xDAWP\x80`\xA0\x015\x81_\x015\x14\x15[\x15aC\x11W`@Q\x7F\xA4\xD3\xD4\xF2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xF8`\x04`\xFF\x16\x90\x1B\x81``\x015\x11\x15\x80aC4WP\x80``\x015\x81` \x015\x14\x15[\x15aCkW`@Q\x7F\xA4\xD3\xD4\xF2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xF8`\x05`\xFF\x16\x90\x1B\x81`\x80\x015\x11\x15\x80aC\x8EWP\x80`\x80\x015\x81`@\x015\x14\x15[\x15aC\xC5W`@Q\x7F\xA4\xD3\xD4\xF2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[_\x80\x1B\x82\x14\x80aC\xDAWP_\x84\x84\x90P\x14[\x15aD\x1CW\x84`@Q\x7FE\x02\xCB\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aD\x13\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[_[\x84\x84\x90P\x81\x10\x15aE3W_\x85\x85\x83\x81\x81\x10aD=WaD<ah]V[[\x90P` \x02\x01` \x81\x01\x90aDR\x91\x90ar\xEDV[\x90PsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cF\xC5\xBB\xBD\x84\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aD\xA3\x92\x91\x90ae\x80V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aD\xBEW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD\xE2\x91\x90ae\xD1V[aE%W\x86\x81`@Q\x7F\x8B\xD00\x97\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aE\x1C\x92\x91\x90ae\x80V[`@Q\x80\x91\x03\x90\xFD[P\x80\x80`\x01\x01\x91PPaD\x1EV[PPPPPPV[\x81\x86`\x03\x01_\x87\x81R` \x01\x90\x81R` \x01_ \x81\x90UP_[\x84\x84\x90P\x81\x10\x15aG\x17W_\x85\x85\x83\x81\x81\x10aEtWaEsah]V[[\x90P` \x02\x01` \x81\x01\x90aE\x89\x91\x90ar\xEDV[\x90P\x87`\x02\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x85\x81R` \x01\x90\x81R` \x01_ \x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c1\xFFA\xC8\x85\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aF[\x92\x91\x90ae\x80V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aFuW=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aF\x9D\x91\x90ar*V[` \x01Q\x90P`\x01\x89_\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPP\x80\x80`\x01\x01\x91PPaEUV[PPPPPPPV[_\x80_\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10aG|Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81aGrWaGqan\x19V[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10aG\xB9Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81aG\xAFWaG\xAEan\x19V[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10aG\xE8Wf#\x86\xF2o\xC1\0\0\x83\x81aG\xDEWaG\xDDan\x19V[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10aH\x11Wc\x05\xF5\xE1\0\x83\x81aH\x07WaH\x06an\x19V[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10aH6Wa'\x10\x83\x81aH,WaH+an\x19V[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10aHYW`d\x83\x81aHOWaHNan\x19V[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10aHhW`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[aHyaLoV[aH\xAFW`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[aH\xB9aHqV[_aH\xC2aAUV[\x90P\x82\x81`\x02\x01\x90\x81aH\xD5\x91\x90aspV[P\x81\x81`\x03\x01\x90\x81aH\xE7\x91\x90aspV[P_\x80\x1B\x81_\x01\x81\x90UP_\x80\x1B\x81`\x01\x01\x81\x90UPPPPV[_aI\x14aI\x0EaL\x8DV[\x83aL\x9BV[\x90P\x91\x90PV[_\x80_\x80aI)\x86\x86aL\xDBV[\x92P\x92P\x92PaI9\x82\x82aM0V[\x82\x93PPPP\x92\x91PPV[_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aI\xA3W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aI\xC7\x91\x90abBV[\x90PsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x94G\xCF\xD4\x82\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aJ\x18\x92\x91\x90ae\x80V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aJ3W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aJW\x91\x90ae\xD1V[aJ\x9AW\x82\x82`@Q\x7F\r\x86\xF5!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aJ\x91\x92\x91\x90at?V[`@Q\x80\x91\x03\x90\xFD[_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c1\xFFA\xC8\x83\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aJ\xEA\x92\x91\x90ae\x80V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aK\x04W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aK,\x91\x90ar*V[\x90P\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14aK\xA4W\x83\x83`@Q\x7F\r\x86\xF5!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aK\x9B\x92\x91\x90at?V[`@Q\x80\x91\x03\x90\xFD[PPPPV[_aK\xD6\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1BaN\x92V[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[aL\x06\x82aN\x9BV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15aLbWaL\\\x82\x82aOdV[PaLkV[aLjaO\xE4V[[PPV[_aLxa7\x03V[_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[_aL\x96aP V[\x90P\x90V[_`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x02\x82\x01R\x82`\"\x82\x01R`B\x81 \x91PP\x92\x91PPV[_\x80_`A\x84Q\x03aM\x1BW_\x80_` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90PaM\r\x88\x82\x85\x85aP\x83V[\x95P\x95P\x95PPPPaM)V[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15aMCWaMBaUHV[[\x82`\x03\x81\x11\x15aMVWaMUaUHV[[\x03\x15aN\x8EW`\x01`\x03\x81\x11\x15aMpWaMoaUHV[[\x82`\x03\x81\x11\x15aM\x83WaM\x82aUHV[[\x03aM\xBAW`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15aM\xCEWaM\xCDaUHV[[\x82`\x03\x81\x11\x15aM\xE1WaM\xE0aUHV[[\x03aN%W\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aN\x1C\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15aN8WaN7aUHV[[\x82`\x03\x81\x11\x15aNKWaNJaUHV[[\x03aN\x8DW\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aN\x84\x91\x90aY@V[`@Q\x80\x91\x03\x90\xFD[[PPV[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03aN\xF6W\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aN\xED\x91\x90aafV[`@Q\x80\x91\x03\x90\xFD[\x80aO\"\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1BaN\x92V[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@QaO\x8D\x91\x90at\x96V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14aO\xC5W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>aO\xCAV[``\x91P[P\x91P\x91PaO\xDA\x85\x83\x83aQjV[\x92PPP\x92\x91PPV[_4\x11\x15aP\x1EW`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0FaPJaQ\xF7V[aPRaRmV[F0`@Q` \x01aPh\x95\x94\x93\x92\x91\x90at\xACV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x80_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15aP\xBFW_`\x03\x85\x92P\x92P\x92PaQ`V[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@QaP\xE2\x94\x93\x92\x91\x90at\xFDV[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aQ\x02W=_\x80>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03aQSW_`\x01_\x80\x1B\x93P\x93P\x93PPaQ`V[\x80_\x80_\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[``\x82aQ\x7FWaQz\x82aR\xE4V[aQ\xEFV[_\x82Q\x14\x80\x15aQ\xA5WP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15aQ\xE7W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aQ\xDE\x91\x90aafV[`@Q\x80\x91\x03\x90\xFD[\x81\x90PaQ\xF0V[[\x93\x92PPPV[_\x80aR\x01aAUV[\x90P_aR\x0CaA|V[\x90P_\x81Q\x11\x15aR(W\x80\x80Q\x90` \x01 \x92PPPaRjV[_\x82_\x01T\x90P_\x80\x1B\x81\x14aRCW\x80\x93PPPPaRjV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x80aRwaAUV[\x90P_aR\x82aB\x1AV[\x90P_\x81Q\x11\x15aR\x9EW\x80\x80Q\x90` \x01 \x92PPPaR\xE1V[_\x82`\x01\x01T\x90P_\x80\x1B\x81\x14aR\xBAW\x80\x93PPPPaR\xE1V[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x81Q\x11\x15aR\xF6W\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15aS_W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90PaSDV[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_aS\x84\x82aS(V[aS\x8E\x81\x85aS2V[\x93PaS\x9E\x81\x85` \x86\x01aSBV[aS\xA7\x81aSjV[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaS\xCA\x81\x84aSzV[\x90P\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x81\x90P\x91\x90PV[aS\xF5\x81aS\xE3V[\x81\x14aS\xFFW_\x80\xFD[PV[_\x815\x90PaT\x10\x81aS\xECV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aT+WaT*aS\xDBV[[_aT8\x84\x82\x85\x01aT\x02V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_aT\x93\x82aTjV[\x90P\x91\x90PV[aT\xA3\x81aT\x89V[\x82RPPV[_aT\xB4\x83\x83aT\x9AV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aT\xD6\x82aTAV[aT\xE0\x81\x85aTKV[\x93PaT\xEB\x83aT[V[\x80_[\x83\x81\x10\x15aU\x1BW\x81QaU\x02\x88\x82aT\xA9V[\x97PaU\r\x83aT\xC0V[\x92PP`\x01\x81\x01\x90PaT\xEEV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaU@\x81\x84aT\xCCV[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x02\x81\x10aU\x86WaU\x85aUHV[[PV[_\x81\x90PaU\x96\x82aUuV[\x91\x90PV[_aU\xA5\x82aU\x89V[\x90P\x91\x90PV[aU\xB5\x81aU\x9BV[\x82RPPV[_` \x82\x01\x90PaU\xCE_\x83\x01\x84aU\xACV[\x92\x91PPV[`\x02\x81\x10aU\xE0W_\x80\xFD[PV[_\x815\x90PaU\xF1\x81aU\xD4V[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15aV\rWaV\x0CaS\xDBV[[_aV\x1A\x85\x82\x86\x01aT\x02V[\x92PP` aV+\x85\x82\x86\x01aU\xE3V[\x91PP\x92P\x92\x90PV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12aVVWaVUaV5V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aVsWaVraV9V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aV\x8FWaV\x8EaV=V[[\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12aV\xABWaV\xAAaV5V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aV\xC8WaV\xC7aV9V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15aV\xE4WaV\xE3aV=V[[\x92P\x92\x90PV[_\x80_\x80_``\x86\x88\x03\x12\x15aW\x04WaW\x03aS\xDBV[[_aW\x11\x88\x82\x89\x01aT\x02V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aW2WaW1aS\xDFV[[aW>\x88\x82\x89\x01aVAV[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aWaWaW`aS\xDFV[[aWm\x88\x82\x89\x01aV\x96V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[aW\x85\x81aT\x89V[\x81\x14aW\x8FW_\x80\xFD[PV[_\x815\x90PaW\xA0\x81aW|V[\x92\x91PPV[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[aW\xE0\x82aSjV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aW\xFFWaW\xFEaW\xAAV[[\x80`@RPPPV[_aX\x11aS\xD2V[\x90PaX\x1D\x82\x82aW\xD7V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aX<WaX;aW\xAAV[[aXE\x82aSjV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_aXraXm\x84aX\"V[aX\x08V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15aX\x8EWaX\x8DaW\xA6V[[aX\x99\x84\x82\x85aXRV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aX\xB5WaX\xB4aV5V[[\x815aX\xC5\x84\x82` \x86\x01aX`V[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15aX\xE4WaX\xE3aS\xDBV[[_aX\xF1\x85\x82\x86\x01aW\x92V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aY\x12WaY\x11aS\xDFV[[aY\x1E\x85\x82\x86\x01aX\xA1V[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[aY:\x81aY(V[\x82RPPV[_` \x82\x01\x90PaYS_\x83\x01\x84aY1V[\x92\x91PPV[_\x80_`@\x84\x86\x03\x12\x15aYpWaYoaS\xDBV[[_aY}\x86\x82\x87\x01aT\x02V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aY\x9EWaY\x9DaS\xDFV[[aY\xAA\x86\x82\x87\x01aV\x96V[\x92P\x92PP\x92P\x92P\x92V[_\x80_\x80_``\x86\x88\x03\x12\x15aY\xCFWaY\xCEaS\xDBV[[_aY\xDC\x88\x82\x89\x01aT\x02V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aY\xFDWaY\xFCaS\xDFV[[aZ\t\x88\x82\x89\x01aV\x96V[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZ,WaZ+aS\xDFV[[aZ8\x88\x82\x89\x01aV\x96V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[_\x81\x15\x15\x90P\x91\x90PV[aZ[\x81aZGV[\x82RPPV[_` \x82\x01\x90PaZt_\x83\x01\x84aZRV[\x92\x91PPV[_\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[aZ\xAE\x81aZzV[\x82RPPV[aZ\xBD\x81aS\xE3V[\x82RPPV[aZ\xCC\x81aT\x89V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a[\x04\x81aS\xE3V[\x82RPPV[_a[\x15\x83\x83aZ\xFBV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a[7\x82aZ\xD2V[a[A\x81\x85aZ\xDCV[\x93Pa[L\x83aZ\xECV[\x80_[\x83\x81\x10\x15a[|W\x81Qa[c\x88\x82a[\nV[\x97Pa[n\x83a[!V[\x92PP`\x01\x81\x01\x90Pa[OV[P\x85\x93PPPP\x92\x91PPV[_`\xE0\x82\x01\x90Pa[\x9C_\x83\x01\x8AaZ\xA5V[\x81\x81\x03` \x83\x01Ra[\xAE\x81\x89aSzV[\x90P\x81\x81\x03`@\x83\x01Ra[\xC2\x81\x88aSzV[\x90Pa[\xD1``\x83\x01\x87aZ\xB4V[a[\xDE`\x80\x83\x01\x86aZ\xC3V[a[\xEB`\xA0\x83\x01\x85aY1V[\x81\x81\x03`\xC0\x83\x01Ra[\xFD\x81\x84a[-V[\x90P\x98\x97PPPPPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a\\N\x82aS(V[a\\X\x81\x85a\\4V[\x93Pa\\h\x81\x85` \x86\x01aSBV[a\\q\x81aSjV[\x84\x01\x91PP\x92\x91PPV[_a\\\x87\x83\x83a\\DV[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\\\xA5\x82a\\\x0BV[a\\\xAF\x81\x85a\\\x15V[\x93P\x83` \x82\x02\x85\x01a\\\xC1\x85a\\%V[\x80_[\x85\x81\x10\x15a\\\xFCW\x84\x84\x03\x89R\x81Qa\\\xDD\x85\x82a\\|V[\x94Pa\\\xE8\x83a\\\x8FV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa\\\xC4V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[`\x02\x81\x10a]HWa]GaUHV[[PV[_\x81\x90Pa]X\x82a]7V[\x91\x90PV[_a]g\x82a]KV[\x90P\x91\x90PV[a]w\x81a]]V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a]\xA1\x82a]}V[a]\xAB\x81\x85a]\x87V[\x93Pa]\xBB\x81\x85` \x86\x01aSBV[a]\xC4\x81aSjV[\x84\x01\x91PP\x92\x91PPV[_`@\x83\x01_\x83\x01Qa]\xE4_\x86\x01\x82a]nV[P` \x83\x01Q\x84\x82\x03` \x86\x01Ra]\xFC\x82\x82a]\x97V[\x91PP\x80\x91PP\x92\x91PPV[_a^\x14\x83\x83a]\xCFV[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a^2\x82a]\x0EV[a^<\x81\x85a]\x18V[\x93P\x83` \x82\x02\x85\x01a^N\x85a](V[\x80_[\x85\x81\x10\x15a^\x89W\x84\x84\x03\x89R\x81Qa^j\x85\x82a^\tV[\x94Pa^u\x83a^\x1CV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa^QV[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra^\xB3\x81\x85a\\\x9BV[\x90P\x81\x81\x03` \x83\x01Ra^\xC7\x81\x84a^(V[\x90P\x93\x92PPPV[_\x80\xFD[_a\x02@\x82\x84\x03\x12\x15a^\xEAWa^\xE9a^\xD0V[[\x81\x90P\x92\x91PPV[_` \x82\x84\x03\x12\x15a_\x08Wa_\x07aS\xDBV[[_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a_%Wa_$aS\xDFV[[a_1\x84\x82\x85\x01a^\xD4V[\x91PP\x92\x91PPV[_` \x82\x01\x90Pa_M_\x83\x01\x84aZ\xB4V[\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a_m\x82a]}V[a_w\x81\x85a_SV[\x93Pa_\x87\x81\x85` \x86\x01aSBV[a_\x90\x81aSjV[\x84\x01\x91PP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra_\xB3\x81\x85a\\\x9BV[\x90P\x81\x81\x03` \x83\x01Ra_\xC7\x81\x84a_cV[\x90P\x93\x92PPPV[_` \x82\x84\x03\x12\x15a_\xE5Wa_\xE4aS\xDBV[[_a_\xF2\x84\x82\x85\x01aU\xE3V[\x91PP\x92\x91PPV[_\x81\x90P\x92\x91PPV[_a`\x0F\x82aS(V[a`\x19\x81\x85a_\xFBV[\x93Pa`)\x81\x85` \x86\x01aSBV[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a`i`\x02\x83a_\xFBV[\x91Pa`t\x82a`5V[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a`\xB3`\x01\x83a_\xFBV[\x91Pa`\xBE\x82a`\x7FV[`\x01\x82\x01\x90P\x91\x90PV[_a`\xD4\x82\x87a`\x05V[\x91Pa`\xDF\x82a`]V[\x91Pa`\xEB\x82\x86a`\x05V[\x91Pa`\xF6\x82a`\xA7V[\x91Paa\x02\x82\x85a`\x05V[\x91Paa\r\x82a`\xA7V[\x91Paa\x19\x82\x84a`\x05V[\x91P\x81\x90P\x95\x94PPPPPV[_\x81Q\x90Paa5\x81aW|V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aaPWaaOaS\xDBV[[_aa]\x84\x82\x85\x01aa'V[\x91PP\x92\x91PPV[_` \x82\x01\x90Paay_\x83\x01\x84aZ\xC3V[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[aa\x9B\x81aa\x7FV[\x82RPPV[_` \x82\x01\x90Paa\xB4_\x83\x01\x84aa\x92V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_aa\xF1\x82aS\xE3V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03ab#Wab\"aa\xBAV[[`\x01\x82\x01\x90P\x91\x90PV[_\x81Q\x90Pab<\x81aS\xECV[\x92\x91PPV[_` \x82\x84\x03\x12\x15abWWabVaS\xDBV[[_abd\x84\x82\x85\x01ab.V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80ab\xB1W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03ab\xC4Wab\xC3abmV[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02ac&\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82ab\xEBV[ac0\x86\x83ab\xEBV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_ackacfaca\x84aS\xE3V[acHV[aS\xE3V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[ac\x84\x83acQV[ac\x98ac\x90\x82acrV[\x84\x84Tab\xF7V[\x82UPPPPV[_\x90V[ac\xACac\xA0V[ac\xB7\x81\x84\x84ac{V[PPPV[[\x81\x81\x10\x15ac\xDAWac\xCF_\x82ac\xA4V[`\x01\x81\x01\x90Pac\xBDV[PPV[`\x1F\x82\x11\x15ad\x1FWac\xF0\x81ab\xCAV[ac\xF9\x84ab\xDCV[\x81\x01` \x85\x10\x15ad\x08W\x81\x90P[ad\x1Cad\x14\x85ab\xDCV[\x83\x01\x82ac\xBCV[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_ad?_\x19\x84`\x08\x02ad$V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_adW\x83\x83ad0V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[adp\x82a]}V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ad\x89Wad\x88aW\xAAV[[ad\x93\x82Tab\x9AV[ad\x9E\x82\x82\x85ac\xDEV[_` \x90P`\x1F\x83\x11`\x01\x81\x14ad\xCFW_\x84\x15ad\xBDW\x82\x87\x01Q\x90P[ad\xC7\x85\x82adLV[\x86UPae.V[`\x1F\x19\x84\x16ad\xDD\x86ab\xCAV[_[\x82\x81\x10\x15ae\x04W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pad\xDFV[\x86\x83\x10\x15ae!W\x84\x89\x01Qae\x1D`\x1F\x89\x16\x82ad0V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_`\x80\x82\x01\x90PaeI_\x83\x01\x87aZ\xB4V[aeV` \x83\x01\x86aZ\xB4V[aec`@\x83\x01\x85aU\xACV[\x81\x81\x03``\x83\x01Raeu\x81\x84a_cV[\x90P\x95\x94PPPPPV[_`@\x82\x01\x90Pae\x93_\x83\x01\x85aZ\xB4V[ae\xA0` \x83\x01\x84aZ\xC3V[\x93\x92PPPV[ae\xB0\x81aZGV[\x81\x14ae\xBAW_\x80\xFD[PV[_\x81Q\x90Pae\xCB\x81ae\xA7V[\x92\x91PPV[_` \x82\x84\x03\x12\x15ae\xE6Wae\xE5aS\xDBV[[_ae\xF3\x84\x82\x85\x01ae\xBDV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[`\x02\x81\x10af\x11W_\x80\xFD[PV[_\x815\x90Paf\"\x81af\x05V[\x92\x91PPV[_af6` \x84\x01\x84af\x14V[\x90P\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12affWafeafFV[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15af\x8EWaf\x8Daf>V[[`\x01\x82\x026\x03\x83\x13\x15af\xA4Waf\xA3afBV[[P\x92P\x92\x90PV[_af\xB7\x83\x85a]\x87V[\x93Paf\xC4\x83\x85\x84aXRV[af\xCD\x83aSjV[\x84\x01\x90P\x93\x92PPPV[_`@\x83\x01af\xE9_\x84\x01\x84af(V[af\xF5_\x86\x01\x82a]nV[Pag\x03` \x84\x01\x84afJV[\x85\x83\x03` \x87\x01Rag\x16\x83\x82\x84af\xACV[\x92PPP\x80\x91PP\x92\x91PPV[_ag/\x83\x83af\xD8V[\x90P\x92\x91PPV[_\x825`\x01`@\x03\x836\x03\x03\x81\x12agRWagQafFV[[\x82\x81\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_agu\x83\x85a]\x18V[\x93P\x83` \x84\x02\x85\x01ag\x87\x84ae\xFCV[\x80_[\x87\x81\x10\x15ag\xCAW\x84\x84\x03\x89Rag\xA1\x82\x84ag7V[ag\xAB\x85\x82ag$V[\x94Pag\xB6\x83ag^V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pag\x8AV[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_ag\xE7\x83\x85a_SV[\x93Pag\xF4\x83\x85\x84aXRV[ag\xFD\x83aSjV[\x84\x01\x90P\x93\x92PPPV[_`\x80\x82\x01\x90Pah\x1B_\x83\x01\x89aZ\xB4V[\x81\x81\x03` \x83\x01Rah.\x81\x87\x89agjV[\x90P\x81\x81\x03`@\x83\x01RahC\x81\x85\x87ag\xDCV[\x90PahR``\x83\x01\x84aZ\xC3V[\x97\x96PPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x825`\x01`@\x03\x836\x03\x03\x81\x12ah\xB1Wah\xB0ah\x8AV[[\x80\x83\x01\x91PP\x92\x91PPV[_\x815ah\xC9\x81af\x05V[\x80\x91PP\x91\x90PV[_\x81_\x1B\x90P\x91\x90PV[_`\xFFah\xE9\x84ah\xD2V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_ai\t\x82a]KV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[ai\"\x82ah\xFFV[ai5ai.\x82ai\x10V[\x83Tah\xDDV[\x82UPPPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aiXWaiWah\x8AV[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aizWaiyah\x8EV[[` \x83\x01\x92P`\x01\x82\x026\x03\x83\x13\x15ai\x96Wai\x95ah\x92V[[P\x92P\x92\x90PV[_\x82\x90P\x92\x91PPV[ai\xB2\x83\x83ai\x9EV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ai\xCBWai\xCAaW\xAAV[[ai\xD5\x82Tab\x9AV[ai\xE0\x82\x82\x85ac\xDEV[_`\x1F\x83\x11`\x01\x81\x14aj\rW_\x84\x15ai\xFBW\x82\x87\x015\x90P[aj\x05\x85\x82adLV[\x86UPajlV[`\x1F\x19\x84\x16aj\x1B\x86ab\xCAV[_[\x82\x81\x10\x15ajBW\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Paj\x1DV[\x86\x83\x10\x15aj_W\x84\x89\x015aj[`\x1F\x89\x16\x82ad0V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[aj\x80\x83\x83\x83ai\xA8V[PPPV[_\x81\x01_\x83\x01\x80aj\x95\x81ah\xBDV[\x90Paj\xA1\x81\x84ai\x19V[PPP`\x01\x81\x01` \x83\x01aj\xB6\x81\x85ai<V[aj\xC1\x81\x83\x86ajuV[PPPPPPV[aj\xD3\x82\x82aj\x85V[PPV[_``\x82\x01\x90Paj\xEA_\x83\x01\x87aZ\xB4V[\x81\x81\x03` \x83\x01Raj\xFC\x81\x86a\\\x9BV[\x90P\x81\x81\x03`@\x83\x01Rak\x11\x81\x84\x86agjV[\x90P\x95\x94PPPPPV[_``\x82\x01\x90Pak/_\x83\x01\x87aZ\xB4V[\x81\x81\x03` \x83\x01RakB\x81\x85\x87ag\xDCV[\x90PakQ`@\x83\x01\x84aZ\xC3V[\x95\x94PPPPPV[_\x81Takf\x81ab\x9AV[akp\x81\x86a_SV[\x94P`\x01\x82\x16_\x81\x14ak\x8AW`\x01\x81\x14ak\xA0Wak\xD2V[`\xFF\x19\x83\x16\x86R\x81\x15\x15` \x02\x86\x01\x93Pak\xD2V[ak\xA9\x85ab\xCAV[_[\x83\x81\x10\x15ak\xCAW\x81T\x81\x89\x01R`\x01\x82\x01\x91P` \x81\x01\x90Pak\xABV[\x80\x88\x01\x95PPP[PPP\x92\x91PPV[_``\x82\x01\x90Pak\xEE_\x83\x01\x86aZ\xB4V[ak\xFB` \x83\x01\x85aZ\xB4V[\x81\x81\x03`@\x83\x01Ral\r\x81\x84akZV[\x90P\x94\x93PPPPV[_`\x80\x82\x01\x90Pal*_\x83\x01\x89aZ\xB4V[\x81\x81\x03` \x83\x01Ral=\x81\x87\x89ag\xDCV[\x90P\x81\x81\x03`@\x83\x01RalR\x81\x85\x87ag\xDCV[\x90Pala``\x83\x01\x84aZ\xC3V[\x97\x96PPPPPPPV[_``\x82\x01\x90Pal\x7F_\x83\x01\x87aZ\xB4V[\x81\x81\x03` \x83\x01Ral\x91\x81\x86a\\\x9BV[\x90P\x81\x81\x03`@\x83\x01Ral\xA6\x81\x84\x86ag\xDCV[\x90P\x95\x94PPPPPV[\x7FEIP712: Uninitialized\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_al\xE5`\x15\x83aS2V[\x91Pal\xF0\x82al\xB1V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ram\x12\x81al\xD9V[\x90P\x91\x90PV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12am5Wam4ah\x8AV[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15amWWamVah\x8EV[[` \x83\x01\x92P` \x82\x026\x03\x83\x13\x15amsWamrah\x92V[[P\x92P\x92\x90PV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12am\x97Wam\x96ah\x8AV[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15am\xB9Wam\xB8ah\x8EV[[` \x83\x01\x92P` \x82\x026\x03\x83\x13\x15am\xD5Wam\xD4ah\x92V[[P\x92P\x92\x90PV[_``\x82\x01\x90Pam\xF0_\x83\x01\x86aZ\xB4V[am\xFD` \x83\x01\x85aU\xACV[\x81\x81\x03`@\x83\x01Ran\x0F\x81\x84a_cV[\x90P\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_`\xFF\x82\x16\x90P\x91\x90PV[_\x81`\xF8\x1B\x90P\x91\x90PV[_anh\x82anRV[\x90P\x91\x90PV[an\x80an{\x82anFV[an^V[\x82RPPV[_\x81\x90P\x91\x90PV[an\xA0an\x9B\x82aS\xE3V[an\x86V[\x82RPPV[_an\xB1\x82\x86anoV[`\x01\x82\x01\x91Pan\xC1\x82\x85an\x8FV[` \x82\x01\x91Pan\xD1\x82\x84an\x8FV[` \x82\x01\x91P\x81\x90P\x94\x93PPPPV[_` \x82\x84\x03\x12\x15an\xF7Wan\xF6aS\xDBV[[_ao\x04\x84\x82\x85\x01af\x14V[\x91PP\x92\x91PPV[_\x81\x90P\x92\x91PPV[_ao\"\x83\x85ao\rV[\x93Pao/\x83\x85\x84aXRV[\x82\x84\x01\x90P\x93\x92PPPV[_aoG\x82\x84\x86ao\x17V[\x91P\x81\x90P\x93\x92PPPV[ao\\\x81a]]V[\x82RPPV[_``\x82\x01\x90Paou_\x83\x01\x86aY1V[ao\x82` \x83\x01\x85aoSV[ao\x8F`@\x83\x01\x84aY1V[\x94\x93PPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[ao\xC3\x81aY(V[\x82RPPV[_ao\xD4\x83\x83ao\xBAV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_ao\xF6\x82ao\x97V[ap\0\x81\x85ao\xA1V[\x93Pap\x0B\x83ao\xABV[\x80_[\x83\x81\x10\x15ap;W\x81Qap\"\x88\x82ao\xC9V[\x97Pap-\x83ao\xE0V[\x92PP`\x01\x81\x01\x90Pap\x0EV[P\x85\x93PPPP\x92\x91PPV[_apS\x82\x84ao\xECV[\x91P\x81\x90P\x92\x91PPV[_`\xA0\x82\x01\x90Papq_\x83\x01\x88aY1V[ap~` \x83\x01\x87aZ\xB4V[ap\x8B`@\x83\x01\x86aZ\xB4V[ap\x98``\x83\x01\x85aY1V[ap\xA5`\x80\x83\x01\x84aY1V[\x96\x95PPPPPPV[ap\xB8\x81anFV[\x82RPPV[_` \x82\x01\x90Pap\xD1_\x83\x01\x84ap\xAFV[\x92\x91PPV[_\x80\xFD[_\x80\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15ap\xF9Wap\xF8aW\xAAV[[aq\x02\x82aSjV[\x90P` \x81\x01\x90P\x91\x90PV[_aq!aq\x1C\x84ap\xDFV[aX\x08V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15aq=Waq<aW\xA6V[[aqH\x84\x82\x85aSBV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aqdWaqcaV5V[[\x81Qaqt\x84\x82` \x86\x01aq\x0FV[\x91PP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15aq\x92Waq\x91ap\xD7V[[aq\x9C`\x80aX\x08V[\x90P_aq\xAB\x84\x82\x85\x01aa'V[_\x83\x01RP` aq\xBE\x84\x82\x85\x01aa'V[` \x83\x01RP`@\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aq\xE2Waq\xE1ap\xDBV[[aq\xEE\x84\x82\x85\x01aqPV[`@\x83\x01RP``\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ar\x12War\x11ap\xDBV[[ar\x1E\x84\x82\x85\x01aqPV[``\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15ar?War>aS\xDBV[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ar\\War[aS\xDFV[[arh\x84\x82\x85\x01aq}V[\x91PP\x92\x91PPV[arz\x81aY(V[\x81\x14ar\x84W_\x80\xFD[PV[_\x81Q\x90Par\x95\x81arqV[\x92\x91PPV[_` \x82\x84\x03\x12\x15ar\xB0War\xAFaS\xDBV[[_ar\xBD\x84\x82\x85\x01ar\x87V[\x91PP\x92\x91PPV[_`@\x82\x01\x90Par\xD9_\x83\x01\x85aY1V[ar\xE6` \x83\x01\x84aZ\xB4V[\x93\x92PPPV[_` \x82\x84\x03\x12\x15as\x02Was\x01aS\xDBV[[_as\x0F\x84\x82\x85\x01aW\x92V[\x91PP\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[`\x1F\x82\x11\x15askWas<\x81as\x18V[asE\x84ab\xDCV[\x81\x01` \x85\x10\x15asTW\x81\x90P[ashas`\x85ab\xDCV[\x83\x01\x82ac\xBCV[PP[PPPV[asy\x82aS(V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15as\x92Was\x91aW\xAAV[[as\x9C\x82Tab\x9AV[as\xA7\x82\x82\x85as*V[_` \x90P`\x1F\x83\x11`\x01\x81\x14as\xD8W_\x84\x15as\xC6W\x82\x87\x01Q\x90P[as\xD0\x85\x82adLV[\x86UPat7V[`\x1F\x19\x84\x16as\xE6\x86as\x18V[_[\x82\x81\x10\x15at\rW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pas\xE8V[\x86\x83\x10\x15at*W\x84\x89\x01Qat&`\x1F\x89\x16\x82ad0V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_`@\x82\x01\x90PatR_\x83\x01\x85aZ\xC3V[at_` \x83\x01\x84aZ\xC3V[\x93\x92PPPV[_atp\x82a]}V[atz\x81\x85ao\rV[\x93Pat\x8A\x81\x85` \x86\x01aSBV[\x80\x84\x01\x91PP\x92\x91PPV[_at\xA1\x82\x84atfV[\x91P\x81\x90P\x92\x91PPV[_`\xA0\x82\x01\x90Pat\xBF_\x83\x01\x88aY1V[at\xCC` \x83\x01\x87aY1V[at\xD9`@\x83\x01\x86aY1V[at\xE6``\x83\x01\x85aZ\xB4V[at\xF3`\x80\x83\x01\x84aZ\xC3V[\x96\x95PPPPPPV[_`\x80\x82\x01\x90Pau\x10_\x83\x01\x87aY1V[au\x1D` \x83\x01\x86ap\xAFV[au*`@\x83\x01\x85aY1V[au7``\x83\x01\x84aY1V[\x95\x94PPPPPV\xFEPrepKeygenVerification(uint256 prepKeygenId)CrsgenVerification(uint256 crsId,uint256 maxBitLength,bytes crsDigest,bytes extraData)KeygenVerification(uint256 prepKeygenId,uint256 keyId,KeyDigest[] keyDigests,bytes extraData)KeyDigest(uint8 keyType,bytes digest)KeyDigest(uint8 keyType,bytes digest)",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405260043610610134575f3560e01c806362978787116100aa578063ad3cb1cc1161006e578063ad3cb1cc146103f9578063baff211e14610423578063c2c1faee1461044d578063c55b872414610475578063caa367db146104b2578063d52f10eb146104da57610134565b80636297878714610312578063735b12631461033a57806384b0196e14610364578063936608ae14610394578063a0079e0f146103d157610134565b80633c02f834116100fc5780633c02f8341461021857806345af261b146102405780634610ffe81461027c5780634f1ef286146102a457806352d1902d146102c0578063589adb0e146102ea57610134565b80630d8e6e2c1461013857806316c713d9146101625780631703c61a1461019e57806319f4f632146101c657806339f7381014610202575b5f80fd5b348015610143575f80fd5b5061014c610504565b60405161015991906153b2565b60405180910390f35b34801561016d575f80fd5b5061018860048036038101906101839190615416565b61057f565b6040516101959190615528565b60405180910390f35b3480156101a9575f80fd5b506101c460048036038101906101bf9190615416565b610650565b005b3480156101d1575f80fd5b506101ec60048036038101906101e79190615416565b61086f565b6040516101f991906155bb565b60405180910390f35b34801561020d575f80fd5b5061021661091c565b005b348015610223575f80fd5b5061023e600480360381019061023991906155f7565b610b3f565b005b34801561024b575f80fd5b5061026660048036038101906102619190615416565b610e23565b60405161027391906155bb565b60405180910390f35b348015610287575f80fd5b506102a2600480360381019061029d91906156eb565b610eb8565b005b6102be60048036038101906102b991906158ce565b611639565b005b3480156102cb575f80fd5b506102d4611658565b6040516102e19190615940565b60405180910390f35b3480156102f5575f80fd5b50610310600480360381019061030b9190615959565b611689565b005b34801561031d575f80fd5b50610338600480360381019061033391906159b6565b611b0f565b005b348015610345575f80fd5b5061034e61218f565b60405161035b9190615a61565b60405180910390f35b34801561036f575f80fd5b50610378612240565b60405161038b9796959493929190615b89565b60405180910390f35b34801561039f575f80fd5b506103ba60048036038101906103b59190615416565b612349565b6040516103c8929190615e9b565b60405180910390f35b3480156103dc575f80fd5b506103f760048036038101906103f29190615ef3565b612669565b005b348015610404575f80fd5b5061040d612d56565b60405161041a91906153b2565b60405180910390f35b34801561042e575f80fd5b50610437612d8f565b6040516104449190615f3a565b60405180910390f35b348015610458575f80fd5b50610473600480360381019061046e9190615416565b612da6565b005b348015610480575f80fd5b5061049b60048036038101906104969190615416565b613010565b6040516104a9929190615f9b565b60405180910390f35b3480156104bd575f80fd5b506104d860048036038101906104d39190615fd0565b61329b565b005b3480156104e5575f80fd5b506104ee6135d7565b6040516104fb9190615f3a565b60405180910390f35b60606040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506105455f6135ee565b61054f60016135ee565b6105585f6135ee565b60405160200161056b94939291906160c9565b604051602081830303815290604052905090565b60605f61058a6136b8565b90505f816003015f8581526020019081526020015f20549050816002015f8581526020019081526020015f205f8281526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561064257602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116105f9575b505050505092505050919050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156106ad573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106d1919061613b565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461074057336040517f21bfda100000000000000000000000000000000000000000000000000000000081526004016107379190616166565b60405180910390fd5b5f6107496136b8565b90508060090154821180610765575060f8600560ff16901b8211155b156107a757816040517fcbe9265600000000000000000000000000000000000000000000000000000000815260040161079e9190615f3a565b60405180910390fd5b806001015f8381526020019081526020015f205f9054906101000a900460ff161561080957816040517fdf0db5fb0000000000000000000000000000000000000000000000000000000081526004016108009190615f3a565b60405180910390fd5b6001816001015f8481526020019081526020015f205f6101000a81548160ff0219169083151502179055507f384f90fefbcfaa68f22e00094aeaa52b2bc693936d2ce1afed1212520b59b58e826040516108639190615f3a565b60405180910390a15050565b5f806108796136b8565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff166108dc57826040517f84de13310000000000000000000000000000000000000000000000000000000081526004016108d39190615f3a565b60405180910390fd5b5f816006015f8581526020019081526020015f2054905081600d015f8281526020019081526020015f205f9054906101000a900460ff1692505050919050565b60016109266136df565b67ffffffffffffffff1614610967576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60025f610972613703565b9050805f0160089054906101000a900460ff16806109ba57508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b156109f1576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff021916908315150217905550610aaa6040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506040518060400160405280600181526020017f310000000000000000000000000000000000000000000000000000000000000081525061372a565b5f610ab36136b8565b905060f8600360ff16901b816004018190555060f8600460ff16901b816005018190555060f8600560ff16901b8160090181905550505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051610b3391906161a1565b60405180910390a15050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610b9c573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610bc0919061613b565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610c2f57336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401610c269190616166565b60405180910390fd5b5f610c386136b8565b90505f8160090154905060f8600560ff16901b8114158015610c775750816001015f8281526020019081526020015f205f9054906101000a900460ff16155b15610cb957806040517f061ac61d000000000000000000000000000000000000000000000000000000008152600401610cb09190615f3a565b60405180910390fd5b816009015f815480929190610ccd906161e7565b91905055505f826009015490508483600a015f8381526020019081526020015f20819055508383600d015f8381526020019081526020015f205f6101000a81548160ff02191690836001811115610d2757610d26615548565b5b02179055505f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa158015610d8a573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610dae9190616242565b90505f610dba82613740565b90508085600e015f8581526020019081526020015f209081610ddc9190616467565b507f8cf0151393f84fd694c5e315cb74cc05b247de0a454fd9e9129c661efdf9401d83888884604051610e129493929190616536565b60405180910390a150505050505050565b5f80610e2d6136b8565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff16610e9057826040517fda32d00f000000000000000000000000000000000000000000000000000000008152600401610e879190615f3a565b60405180910390fd5b80600d015f8481526020019081526020015f205f9054906101000a900460ff16915050919050565b5f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa158015610f16573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f3a9190616242565b90507344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166346c5bbbd82336040518363ffffffff1660e01b8152600401610f8b929190616580565b602060405180830381865afa158015610fa6573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610fca91906165d1565b61100b57336040517faee863230000000000000000000000000000000000000000000000000000000081526004016110029190616166565b60405180910390fd5b5f6110146136b8565b90508060050154871180611030575060f8600460ff16901b8711155b1561107257866040517fadfab9040000000000000000000000000000000000000000000000000000000081526004016110699190615f3a565b60405180910390fd5b5f86869050036110b957866040517fe6f9083b0000000000000000000000000000000000000000000000000000000081526004016110b09190615f3a565b60405180910390fd5b5f816006015f8981526020019081526020015f20549050816001015f8281526020019081526020015f205f9054906101000a900460ff16611126576040517f6fbcdd2b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f6111cd828a8a8a87600e015f8f81526020019081526020015f20805461114c9061629a565b80601f01602080910402602001604051908101604052809291908181526020018280546111789061629a565b80156111c35780601f1061119a576101008083540402835291602001916111c3565b820191905f5260205f20905b8154815290600101906020018083116111a657829003601f168201915b505050505061376e565b90505f6111db82888861394f565b9050835f015f8b81526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff161561127b5789816040517f98fb957d000000000000000000000000000000000000000000000000000000008152600401611272929190616580565b60405180910390fd5b6001845f015f8c81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f846002015f8c81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505f818054905090507f2afe64fb3afde8e2678aea84cf36223f330e2fb1286d37aed573ab9cd1db47c78c8c8c8c8c336040516113a596959493929190616808565b60405180910390a1856001015f8d81526020019081526020015f205f9054906101000a900460ff161580156113df57506113de816139b5565b5b1561162b576001866001015f8e81526020019081526020015f205f6101000a81548160ff0219169083151502179055505f5b8b8b905081101561149557866007015f8e81526020019081526020015f208c8c838181106114425761144161685d565b5b90506020028101906114549190616896565b908060018154018082558091505060019003905f5260205f2090600202015f9091909190915081816114869190616ac9565b50508080600101915050611411565b5083866003015f8e81526020019081526020015f20819055508b86600801819055505f61155a87600e015f8f81526020019081526020015f2080546114d99061629a565b80601f01602080910402602001604051908101604052809291908181526020018280546115059061629a565b80156115505780601f1061152757610100808354040283529160200191611550565b820191905f5260205f20905b81548152906001019060200180831161153357829003601f168201915b5050505050613a46565b90505f6115e982858054806020026020016040519081016040528092919081815260200182805480156115df57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311611596575b5050505050613ba6565b90507feb85c26dbcad46b80a68a0f24cce7c2c90f0a1faded84184138839fc9e80a25b8e828f8f6040516116209493929190616ad7565b60405180910390a150505b505050505050505050505050565b611641613cee565b61164a82613dd4565b6116548282613ec7565b5050565b5f611661613fe5565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b5f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa1580156116e7573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061170b9190616242565b90507344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166346c5bbbd82336040518363ffffffff1660e01b815260040161175c929190616580565b602060405180830381865afa158015611777573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061179b91906165d1565b6117dc57336040517faee863230000000000000000000000000000000000000000000000000000000081526004016117d39190616166565b60405180910390fd5b5f6117e56136b8565b90508060040154851180611801575060f8600360ff16901b8511155b1561184357846040517f0ab7f68700000000000000000000000000000000000000000000000000000000815260040161183a9190615f3a565b60405180910390fd5b5f61184d8661406c565b90505f61185b82878761394f565b9050825f015f8881526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16156118fb5786816040517f33ca1fe30000000000000000000000000000000000000000000000000000000081526004016118f2929190616580565b60405180910390fd5b6001835f015f8981526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f836002015f8981526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507f4c715c5734ce5c18c9c12e8496e53d2a65f1ec381d476957f0f596b364a59b0c88888833604051611a199493929190616b1c565b60405180910390a1836001015f8981526020019081526020015f205f9054906101000a900460ff16158015611a575750611a5681805490506139b5565b5b15611b05576001846001015f8a81526020019081526020015f205f6101000a81548160ff02191690831515021790555082846003015f8a81526020019081526020015f20819055505f846006015f8a81526020019081526020015f205490507f3a116120cca5d4f073cc1fc31ff26133ab7b0499f2712fa010023b87d5a1f9ee898287600e015f8d81526020019081526020015f20604051611afb93929190616bdb565b60405180910390a1505b5050505050505050565b5f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa158015611b6d573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b919190616242565b90507344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166346c5bbbd82336040518363ffffffff1660e01b8152600401611be2929190616580565b602060405180830381865afa158015611bfd573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c2191906165d1565b611c6257336040517faee86323000000000000000000000000000000000000000000000000000000008152600401611c599190616166565b60405180910390fd5b5f611c6b6136b8565b90508060090154871180611c87575060f8600560ff16901b8711155b15611cc957866040517f8d8c940a000000000000000000000000000000000000000000000000000000008152600401611cc09190615f3a565b60405180910390fd5b5f81600a015f8981526020019081526020015f205490505f611d8789838a8a87600e015f8f81526020019081526020015f208054611d069061629a565b80601f0160208091040260200160405190810160405280929190818152602001828054611d329061629a565b8015611d7d5780601f10611d5457610100808354040283529160200191611d7d565b820191905f5260205f20905b815481529060010190602001808311611d6057829003601f168201915b50505050506140c4565b90505f611d9582888861394f565b9050835f015f8b81526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615611e355789816040517ffcf5a6e9000000000000000000000000000000000000000000000000000000008152600401611e2c929190616580565b60405180910390fd5b6001845f015f8c81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f846002015f8c81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505f818054905090507f7bf1b42c10e9497c879620c5b7afced10bda17d8c90b22f0e3bc6b2fd6ced0bd8c8c8c8c8c33604051611f5f96959493929190616c17565b60405180910390a1856001015f8d81526020019081526020015f205f9054906101000a900460ff16158015611f995750611f98816139b5565b5b15612181576001866001015f8e81526020019081526020015f205f6101000a81548160ff0219169083151502179055508a8a87600b015f8f81526020019081526020015f209182611feb9291906169a8565b5083866003015f8e81526020019081526020015f20819055508b86600c01819055505f6120b087600e015f8f81526020019081526020015f20805461202f9061629a565b80601f016020809104026020016040519081016040528092919081815260200182805461205b9061629a565b80156120a65780601f1061207d576101008083540402835291602001916120a6565b820191905f5260205f20905b81548152906001019060200180831161208957829003601f168201915b5050505050613a46565b90505f61213f828580548060200260200160405190810160405280929190818152602001828054801561213557602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116120ec575b5050505050613ba6565b90507f2258b73faed33fb2e2ea454403bef974920caf682ab3a723484fcf67553b16a28e828f8f6040516121769493929190616c6c565b60405180910390a150505b505050505050505050505050565b5f806121996136b8565b90505f8160050154905060f8600460ff16901b81141580156121d85750816001015f8281526020019081526020015f205f9054906101000a900460ff16155b156121e85760019250505061223d565b5f8260090154905060f8600560ff16901b81141580156122255750826001015f8281526020019081526020015f205f9054906101000a900460ff16155b15612236576001935050505061223d565b5f93505050505b90565b5f6060805f805f60605f612252614155565b90505f801b815f015414801561226d57505f801b8160010154145b6122ac576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016122a390616cfb565b60405180910390fd5b6122b461417c565b6122bc61421a565b46305f801b5f67ffffffffffffffff8111156122db576122da6157aa565b5b6040519080825280602002602001820160405280156123095781602001602082028036833780820191505090505b507f0f0000000000000000000000000000000000000000000000000000000000000095949392919097509750975097509750975097505090919293949596565b6060805f6123556136b8565b9050806001015f8581526020019081526020015f205f9054906101000a900460ff166123b857836040517f84de13310000000000000000000000000000000000000000000000000000000081526004016123af9190615f3a565b60405180910390fd5b5f816003015f8681526020019081526020015f205490505f826002015f8781526020019081526020015f205f8381526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561246f57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311612426575b505050505090505f61251984600e015f8981526020019081526020015f2080546124989061629a565b80601f01602080910402602001604051908101604052809291908181526020018280546124c49061629a565b801561250f5780601f106124e65761010080835404028352916020019161250f565b820191905f5260205f20905b8154815290600101906020018083116124f257829003601f168201915b5050505050613a46565b90505f6125268284613ba6565b905080856007015f8a81526020019081526020015f2080805480602002602001604051908101604052809291908181526020015f905b82821015612655578382905f5260205f2090600202016040518060400160405290815f82015f9054906101000a900460ff1660018111156125a05761259f615548565b5b60018111156125b2576125b1615548565b5b81526020016001820180546125c69061629a565b80601f01602080910402602001604051908101604052809291908181526020018280546125f29061629a565b801561263d5780601f106126145761010080835404028352916020019161263d565b820191905f5260205f20905b81548152906001019060200180831161262057829003601f168201915b5050505050815250508152602001906001019061255c565b505050509050965096505050505050915091565b60016126736136df565b67ffffffffffffffff16146126b4576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60025f6126bf613703565b9050805f0160089054906101000a900460ff168061270757508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b1561273e576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055506127f76040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506040518060400160405280600181526020017f310000000000000000000000000000000000000000000000000000000000000081525061372a565b5f6128006136b8565b905061280b846142b8565b6128358460600135858061010001906128249190616d19565b8761012001358861022001356143c8565b61285f84608001358580610140019061284e9190616d19565b8761016001358861022001356143c8565b6128898460a00135858061018001906128789190616d19565b876101a001358861022001356143c8565b5f848060c0019061289a9190616d7b565b9050036128e25783606001356040517f16bbaf8d0000000000000000000000000000000000000000000000000000000081526004016128d99190615f3a565b60405180910390fd5b5f848060e001906128f3919061693c565b90500361293b5783608001356040517f16bbaf8d0000000000000000000000000000000000000000000000000000000081526004016129329190615f3a565b60405180910390fd5b835f01358160040181905550836020013581600501819055508360400135816009018190555083606001358160080181905550836080013581600c01819055508360600135816006015f8660a0013581526020019081526020015f20819055508360a00135816006015f866060013581526020019081526020015f20819055505f5b848060c001906129cd9190616d7b565b9050811015612a6157816007015f866060013581526020019081526020015f20858060c001906129fd9190616d7b565b83818110612a0e57612a0d61685d565b5b9050602002810190612a209190616896565b908060018154018082558091505060019003905f5260205f2090600202015f909190919091508181612a529190616ac9565b505080806001019150506129bd565b50838060e00190612a72919061693c565b82600b015f876080013581526020019081526020015f209182612a969291906169a8565b50836101c0013581600a015f866080013581526020019081526020015f2081905550612ae381856060013586806101000190612ad29190616d19565b88610120013589610220013561453b565b612b0e81856080013586806101400190612afd9190616d19565b88610160013589610220013561453b565b612b39818560a0013586806101800190612b289190616d19565b886101a0013589610220013561453b565b6001816001015f8660a0013581526020019081526020015f205f6101000a81548160ff0219169083151502179055506001816001015f866060013581526020019081526020015f205f6101000a81548160ff0219169083151502179055506001816001015f866080013581526020019081526020015f205f6101000a81548160ff021916908315150217905550836101e0016020810190612bda9190615fd0565b81600d015f8660a0013581526020019081526020015f205f6101000a81548160ff02191690836001811115612c1257612c11615548565b5b021790555083610200016020810190612c2b9190615fd0565b81600d015f866080013581526020019081526020015f205f6101000a81548160ff02191690836001811115612c6357612c62615548565b5b0217905550612c76846102200135613740565b81600e015f8660a0013581526020019081526020015f209081612c999190616467565b50612ca8846102200135613740565b81600e015f866060013581526020019081526020015f209081612ccb9190616467565b50612cda846102200135613740565b81600e015f866080013581526020019081526020015f209081612cfd9190616467565b50505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051612d4991906161a1565b60405180910390a1505050565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b5f80612d996136b8565b905080600c015491505090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612e03573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612e27919061613b565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614612e9657336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401612e8d9190616166565b60405180910390fd5b5f612e9f6136b8565b90508060050154821180612ebb575060f8600460ff16901b8211155b15612efd57816040517ffcf2db7a000000000000000000000000000000000000000000000000000000008152600401612ef49190615f3a565b60405180910390fd5b806001015f8381526020019081526020015f205f9054906101000a900460ff1615612f5f57816040517f92789b67000000000000000000000000000000000000000000000000000000008152600401612f569190615f3a565b60405180910390fd5b6001816001015f8481526020019081526020015f205f6101000a81548160ff0219169083151502179055505f816006015f8481526020019081526020015f205490505f8114612fd4576001826001015f8381526020019081526020015f205f6101000a81548160ff0219169083151502179055505b7f2b087b884b35a81d769d1a1e092880f1da56de964e4b339eabcb1f45f5fe3264836040516130039190615f3a565b60405180910390a1505050565b6060805f61301c6136b8565b9050806001015f8581526020019081526020015f205f9054906101000a900460ff1661307f57836040517fda32d00f0000000000000000000000000000000000000000000000000000000081526004016130769190615f3a565b60405180910390fd5b5f816003015f8681526020019081526020015f205490505f826002015f8781526020019081526020015f205f8381526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561313657602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116130ed575b505050505090505f6131e084600e015f8981526020019081526020015f20805461315f9061629a565b80601f016020809104026020016040519081016040528092919081815260200182805461318b9061629a565b80156131d65780601f106131ad576101008083540402835291602001916131d6565b820191905f5260205f20905b8154815290600101906020018083116131b957829003601f168201915b5050505050613a46565b90505f6131ed8284613ba6565b90508085600b015f8a81526020019081526020015f2080805461320f9061629a565b80601f016020809104026020016040519081016040528092919081815260200182805461323b9061629a565b80156132865780601f1061325d57610100808354040283529160200191613286565b820191905f5260205f20905b81548152906001019060200180831161326957829003601f168201915b50505050509050965096505050505050915091565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156132f8573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061331c919061613b565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461338b57336040517f21bfda100000000000000000000000000000000000000000000000000000000081526004016133829190616166565b60405180910390fd5b5f6133946136b8565b90505f8160050154905060f8600460ff16901b81141580156133d35750816001015f8281526020019081526020015f205f9054906101000a900460ff16155b1561341557806040517f3b853da800000000000000000000000000000000000000000000000000000000815260040161340c9190615f3a565b60405180910390fd5b816004015f815480929190613429906161e7565b91905055505f82600401549050826005015f81548092919061344a906161e7565b91905055505f8360050154905080846006015f8481526020019081526020015f208190555081846006015f8381526020019081526020015f20819055508484600d015f8481526020019081526020015f205f6101000a81548160ff021916908360018111156134bc576134bb615548565b5b02179055505f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa15801561351f573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906135439190616242565b90505f61354f82613740565b90508086600e015f8681526020019081526020015f2090816135719190616467565b508086600e015f8581526020019081526020015f2090816135929190616467565b507ffbf5274810b94f86970c1147e8ffaebed246ee9777d695a69004dc6256d1fe918488836040516135c693929190616ddd565b60405180910390a150505050505050565b5f806135e16136b8565b9050806008015491505090565b60605f60016135fc84614720565b0190505f8167ffffffffffffffff81111561361a576136196157aa565b5b6040519080825280601f01601f19166020018201604052801561364c5781602001600182028036833780820191505090505b5090505f82602001820190505b6001156136ad578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a85816136a2576136a1616e19565b5b0494505f8503613659575b819350505050919050565b5f7f26fdaf8a2cb20d20b55e36218986905e534ee7a970dd2fa827946e4b7496db00905090565b5f6136e8613703565b5f015f9054906101000a900467ffffffffffffffff16905090565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b613732614871565b61373c82826148b1565b5050565b60606002825f60405160200161375893929190616ea6565b6040516020818303038152906040529050919050565b5f808484905067ffffffffffffffff81111561378d5761378c6157aa565b5b6040519080825280602002602001820160405280156137bb5781602001602082028036833780820191505090505b5090505f5b858590508110156138bf5760405180606001604052806025815260200161764560259139805190602001208686838181106137fe576137fd61685d565b5b90506020028101906138109190616896565b5f0160208101906138219190616ee2565b8787848181106138345761383361685d565b5b90506020028101906138469190616896565b8060200190613855919061693c565b604051613863929190616f3b565b604051809103902060405160200161387d93929190616f62565b604051602081830303815290604052805190602001208282815181106138a6576138a561685d565b5b60200260200101818152505080806001019150506137c0565b506139436040518060c00160405280608281526020016175c360829139805190602001208888846040516020016138f69190617048565b60405160208183030381529060405280519060200120878051906020012060405160200161392895949392919061705e565b60405160208183030381529060405280519060200120614902565b91505095945050505050565b5f8061399e8585858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505061491b565b90506139aa8133614945565b809150509392505050565b5f807344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663b4722bc46040518163ffffffff1660e01b8152600401602060405180830381865afa158015613a14573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613a389190616242565b905080831015915050919050565b5f80825103613ad7577344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa158015613aac573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613ad09190616242565b9050613ba1565b5f825f81518110613aeb57613aea61685d565b5b602001015160f81c60f81b60f81c9050600160ff168160ff161480613b165750600260ff168160ff16145b15613b6457602183511015613b57576040517f8b248b6000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6021830151915050613ba1565b806040517f2139cc2c000000000000000000000000000000000000000000000000000000008152600401613b9891906170be565b60405180910390fd5b919050565b60605f825190505f8167ffffffffffffffff811115613bc857613bc76157aa565b5b604051908082528060200260200182016040528015613bfb57816020015b6060815260200190600190039081613be65790505b5090505f5b82811015613ce2577344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166331ff41c887878481518110613c4c57613c4b61685d565b5b60200260200101516040518363ffffffff1660e01b8152600401613c71929190616580565b5f60405180830381865afa158015613c8b573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f82011682018060405250810190613cb3919061722a565b60600151828281518110613cca57613cc961685d565b5b60200260200101819052508080600101915050613c00565b50809250505092915050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff161480613d9b57507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16613d82614baa565b73ffffffffffffffffffffffffffffffffffffffff1614155b15613dd2576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613e31573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613e55919061613b565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614613ec457336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401613ebb9190616166565b60405180910390fd5b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa925050508015613f2f57506040513d601f19601f82011682018060405250810190613f2c919061729b565b60015b613f7057816040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401613f679190616166565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b8114613fd657806040517faa1d49a4000000000000000000000000000000000000000000000000000000008152600401613fcd9190615940565b60405180910390fd5b613fe08383614bfd565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff161461406a576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f6140bd6040518060600160405280602c8152602001617541602c913980519060200120836040516020016140a29291906172c6565b60405160208183030381529060405280519060200120614902565b9050919050565b5f61414a60405180608001604052806056815260200161756d6056913980519060200120878787876040516020016140fd929190616f3b565b60405160208183030381529060405280519060200120868051906020012060405160200161412f95949392919061705e565b60405160208183030381529060405280519060200120614902565b905095945050505050565b5f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100905090565b60605f614187614155565b90508060020180546141989061629a565b80601f01602080910402602001604051908101604052809291908181526020018280546141c49061629a565b801561420f5780601f106141e65761010080835404028352916020019161420f565b820191905f5260205f20905b8154815290600101906020018083116141f257829003601f168201915b505050505091505090565b60605f614225614155565b90508060030180546142369061629a565b80601f01602080910402602001604051908101604052809291908181526020018280546142629061629a565b80156142ad5780601f10614284576101008083540402835291602001916142ad565b820191905f5260205f20905b81548152906001019060200180831161429057829003601f168201915b505050505091505090565b60f8600360ff16901b8160a001351115806142da57508060a00135815f013514155b15614311576040517fa4d3d4f200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60f8600460ff16901b816060013511158061433457508060600135816020013514155b1561436b576040517fa4d3d4f200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60f8600560ff16901b816080013511158061438e57508060800135816040013514155b156143c5576040517fa4d3d4f200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50565b5f801b8214806143da57505f84849050145b1561441c57846040517f4502cbf10000000000000000000000000000000000000000000000000000000081526004016144139190615f3a565b60405180910390fd5b5f5b84849050811015614533575f85858381811061443d5761443c61685d565b5b905060200201602081019061445291906172ed565b90507344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166346c5bbbd84836040518363ffffffff1660e01b81526004016144a3929190616580565b602060405180830381865afa1580156144be573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906144e291906165d1565b6145255786816040517f8bd0309700000000000000000000000000000000000000000000000000000000815260040161451c929190616580565b60405180910390fd5b50808060010191505061441e565b505050505050565b81866003015f8781526020019081526020015f20819055505f5b84849050811015614717575f8585838181106145745761457361685d565b5b905060200201602081019061458991906172ed565b9050876002015f8881526020019081526020015f205f8581526020019081526020015f2081908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166331ff41c885846040518363ffffffff1660e01b815260040161465b929190616580565b5f60405180830381865afa158015614675573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f8201168201806040525081019061469d919061722a565b6020015190506001895f015f8a81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555050508080600101915050614555565b50505050505050565b5f805f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f010000000000000000831061477c577a184f03e93ff9f4daa797ed6e38ed64bf6a1f010000000000000000838161477257614771616e19565b5b0492506040810190505b6d04ee2d6d415b85acef810000000083106147b9576d04ee2d6d415b85acef810000000083816147af576147ae616e19565b5b0492506020810190505b662386f26fc1000083106147e857662386f26fc1000083816147de576147dd616e19565b5b0492506010810190505b6305f5e1008310614811576305f5e100838161480757614806616e19565b5b0492506008810190505b612710831061483657612710838161482c5761482b616e19565b5b0492506004810190505b60648310614859576064838161484f5761484e616e19565b5b0492506002810190505b600a8310614868576001810190505b80915050919050565b614879614c6f565b6148af576040517fd7e6bcf800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b6148b9614871565b5f6148c2614155565b9050828160020190816148d59190617370565b50818160030190816148e79190617370565b505f801b815f01819055505f801b8160010181905550505050565b5f61491461490e614c8d565b83614c9b565b9050919050565b5f805f806149298686614cdb565b9250925092506149398282614d30565b82935050505092915050565b5f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa1580156149a3573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906149c79190616242565b90507344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff16639447cfd482856040518363ffffffff1660e01b8152600401614a18929190616580565b602060405180830381865afa158015614a33573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614a5791906165d1565b614a9a5782826040517f0d86f521000000000000000000000000000000000000000000000000000000008152600401614a9192919061743f565b60405180910390fd5b5f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166331ff41c883856040518363ffffffff1660e01b8152600401614aea929190616580565b5f60405180830381865afa158015614b04573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f82011682018060405250810190614b2c919061722a565b90508373ffffffffffffffffffffffffffffffffffffffff16816020015173ffffffffffffffffffffffffffffffffffffffff1614614ba45783836040517f0d86f521000000000000000000000000000000000000000000000000000000008152600401614b9b92919061743f565b60405180910390fd5b50505050565b5f614bd67f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b614e92565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b614c0682614e9b565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f81511115614c6257614c5c8282614f64565b50614c6b565b614c6a614fe4565b5b5050565b5f614c78613703565b5f0160089054906101000a900460ff16905090565b5f614c96615020565b905090565b5f6040517f190100000000000000000000000000000000000000000000000000000000000081528360028201528260228201526042812091505092915050565b5f805f6041845103614d1b575f805f602087015192506040870151915060608701515f1a9050614d0d88828585615083565b955095509550505050614d29565b5f600285515f1b9250925092505b9250925092565b5f6003811115614d4357614d42615548565b5b826003811115614d5657614d55615548565b5b0315614e8e5760016003811115614d7057614d6f615548565b5b826003811115614d8357614d82615548565b5b03614dba576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60026003811115614dce57614dcd615548565b5b826003811115614de157614de0615548565b5b03614e2557805f1c6040517ffce698f7000000000000000000000000000000000000000000000000000000008152600401614e1c9190615f3a565b60405180910390fd5b600380811115614e3857614e37615548565b5b826003811115614e4b57614e4a615548565b5b03614e8d57806040517fd78bce0c000000000000000000000000000000000000000000000000000000008152600401614e849190615940565b60405180910390fd5b5b5050565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b03614ef657806040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401614eed9190616166565b60405180910390fd5b80614f227f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b614e92565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f808473ffffffffffffffffffffffffffffffffffffffff1684604051614f8d9190617496565b5f60405180830381855af49150503d805f8114614fc5576040519150601f19603f3d011682016040523d82523d5f602084013e614fca565b606091505b5091509150614fda85838361516a565b9250505092915050565b5f34111561501e576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f61504a6151f7565b61505261526d565b46306040516020016150689594939291906174ac565b60405160208183030381529060405280519060200120905090565b5f805f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c11156150bf575f600385925092509250615160565b5f6001888888886040515f81526020016040526040516150e294939291906174fd565b6020604051602081039080840390855afa158015615102573d5f803e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603615153575f60015f801b93509350935050615160565b805f805f1b935093509350505b9450945094915050565b60608261517f5761517a826152e4565b6151ef565b5f82511480156151a557505f8473ffffffffffffffffffffffffffffffffffffffff163b145b156151e757836040517f9996b3150000000000000000000000000000000000000000000000000000000081526004016151de9190616166565b60405180910390fd5b8190506151f0565b5b9392505050565b5f80615201614155565b90505f61520c61417c565b90505f815111156152285780805190602001209250505061526a565b5f825f015490505f801b81146152435780935050505061526a565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f80615277614155565b90505f61528261421a565b90505f8151111561529e578080519060200120925050506152e1565b5f826001015490505f801b81146152ba578093505050506152e1565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f815111156152f65780518082602001fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f81519050919050565b5f82825260208201905092915050565b5f5b8381101561535f578082015181840152602081019050615344565b5f8484015250505050565b5f601f19601f8301169050919050565b5f61538482615328565b61538e8185615332565b935061539e818560208601615342565b6153a78161536a565b840191505092915050565b5f6020820190508181035f8301526153ca818461537a565b905092915050565b5f604051905090565b5f80fd5b5f80fd5b5f819050919050565b6153f5816153e3565b81146153ff575f80fd5b50565b5f81359050615410816153ec565b92915050565b5f6020828403121561542b5761542a6153db565b5b5f61543884828501615402565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6154938261546a565b9050919050565b6154a381615489565b82525050565b5f6154b4838361549a565b60208301905092915050565b5f602082019050919050565b5f6154d682615441565b6154e0818561544b565b93506154eb8361545b565b805f5b8381101561551b57815161550288826154a9565b975061550d836154c0565b9250506001810190506154ee565b5085935050505092915050565b5f6020820190508181035f83015261554081846154cc565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b6002811061558657615585615548565b5b50565b5f81905061559682615575565b919050565b5f6155a582615589565b9050919050565b6155b58161559b565b82525050565b5f6020820190506155ce5f8301846155ac565b92915050565b600281106155e0575f80fd5b50565b5f813590506155f1816155d4565b92915050565b5f806040838503121561560d5761560c6153db565b5b5f61561a85828601615402565b925050602061562b858286016155e3565b9150509250929050565b5f80fd5b5f80fd5b5f80fd5b5f8083601f84011261565657615655615635565b5b8235905067ffffffffffffffff81111561567357615672615639565b5b60208301915083602082028301111561568f5761568e61563d565b5b9250929050565b5f8083601f8401126156ab576156aa615635565b5b8235905067ffffffffffffffff8111156156c8576156c7615639565b5b6020830191508360018202830111156156e4576156e361563d565b5b9250929050565b5f805f805f60608688031215615704576157036153db565b5b5f61571188828901615402565b955050602086013567ffffffffffffffff811115615732576157316153df565b5b61573e88828901615641565b9450945050604086013567ffffffffffffffff811115615761576157606153df565b5b61576d88828901615696565b92509250509295509295909350565b61578581615489565b811461578f575f80fd5b50565b5f813590506157a08161577c565b92915050565b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6157e08261536a565b810181811067ffffffffffffffff821117156157ff576157fe6157aa565b5b80604052505050565b5f6158116153d2565b905061581d82826157d7565b919050565b5f67ffffffffffffffff82111561583c5761583b6157aa565b5b6158458261536a565b9050602081019050919050565b828183375f83830152505050565b5f61587261586d84615822565b615808565b90508281526020810184848401111561588e5761588d6157a6565b5b615899848285615852565b509392505050565b5f82601f8301126158b5576158b4615635565b5b81356158c5848260208601615860565b91505092915050565b5f80604083850312156158e4576158e36153db565b5b5f6158f185828601615792565b925050602083013567ffffffffffffffff811115615912576159116153df565b5b61591e858286016158a1565b9150509250929050565b5f819050919050565b61593a81615928565b82525050565b5f6020820190506159535f830184615931565b92915050565b5f805f604084860312156159705761596f6153db565b5b5f61597d86828701615402565b935050602084013567ffffffffffffffff81111561599e5761599d6153df565b5b6159aa86828701615696565b92509250509250925092565b5f805f805f606086880312156159cf576159ce6153db565b5b5f6159dc88828901615402565b955050602086013567ffffffffffffffff8111156159fd576159fc6153df565b5b615a0988828901615696565b9450945050604086013567ffffffffffffffff811115615a2c57615a2b6153df565b5b615a3888828901615696565b92509250509295509295909350565b5f8115159050919050565b615a5b81615a47565b82525050565b5f602082019050615a745f830184615a52565b92915050565b5f7fff0000000000000000000000000000000000000000000000000000000000000082169050919050565b615aae81615a7a565b82525050565b615abd816153e3565b82525050565b615acc81615489565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b615b04816153e3565b82525050565b5f615b158383615afb565b60208301905092915050565b5f602082019050919050565b5f615b3782615ad2565b615b418185615adc565b9350615b4c83615aec565b805f5b83811015615b7c578151615b638882615b0a565b9750615b6e83615b21565b925050600181019050615b4f565b5085935050505092915050565b5f60e082019050615b9c5f83018a615aa5565b8181036020830152615bae818961537a565b90508181036040830152615bc2818861537a565b9050615bd16060830187615ab4565b615bde6080830186615ac3565b615beb60a0830185615931565b81810360c0830152615bfd8184615b2d565b905098975050505050505050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f82825260208201905092915050565b5f615c4e82615328565b615c588185615c34565b9350615c68818560208601615342565b615c718161536a565b840191505092915050565b5f615c878383615c44565b905092915050565b5f602082019050919050565b5f615ca582615c0b565b615caf8185615c15565b935083602082028501615cc185615c25565b805f5b85811015615cfc5784840389528151615cdd8582615c7c565b9450615ce883615c8f565b925060208a01995050600181019050615cc4565b50829750879550505050505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b60028110615d4857615d47615548565b5b50565b5f819050615d5882615d37565b919050565b5f615d6782615d4b565b9050919050565b615d7781615d5d565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f615da182615d7d565b615dab8185615d87565b9350615dbb818560208601615342565b615dc48161536a565b840191505092915050565b5f604083015f830151615de45f860182615d6e565b5060208301518482036020860152615dfc8282615d97565b9150508091505092915050565b5f615e148383615dcf565b905092915050565b5f602082019050919050565b5f615e3282615d0e565b615e3c8185615d18565b935083602082028501615e4e85615d28565b805f5b85811015615e895784840389528151615e6a8582615e09565b9450615e7583615e1c565b925060208a01995050600181019050615e51565b50829750879550505050505092915050565b5f6040820190508181035f830152615eb38185615c9b565b90508181036020830152615ec78184615e28565b90509392505050565b5f80fd5b5f6102408284031215615eea57615ee9615ed0565b5b81905092915050565b5f60208284031215615f0857615f076153db565b5b5f82013567ffffffffffffffff811115615f2557615f246153df565b5b615f3184828501615ed4565b91505092915050565b5f602082019050615f4d5f830184615ab4565b92915050565b5f82825260208201905092915050565b5f615f6d82615d7d565b615f778185615f53565b9350615f87818560208601615342565b615f908161536a565b840191505092915050565b5f6040820190508181035f830152615fb38185615c9b565b90508181036020830152615fc78184615f63565b90509392505050565b5f60208284031215615fe557615fe46153db565b5b5f615ff2848285016155e3565b91505092915050565b5f81905092915050565b5f61600f82615328565b6160198185615ffb565b9350616029818560208601615342565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f616069600283615ffb565b915061607482616035565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f6160b3600183615ffb565b91506160be8261607f565b600182019050919050565b5f6160d48287616005565b91506160df8261605d565b91506160eb8286616005565b91506160f6826160a7565b91506161028285616005565b915061610d826160a7565b91506161198284616005565b915081905095945050505050565b5f815190506161358161577c565b92915050565b5f602082840312156161505761614f6153db565b5b5f61615d84828501616127565b91505092915050565b5f6020820190506161795f830184615ac3565b92915050565b5f67ffffffffffffffff82169050919050565b61619b8161617f565b82525050565b5f6020820190506161b45f830184616192565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f6161f1826153e3565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203616223576162226161ba565b5b600182019050919050565b5f8151905061623c816153ec565b92915050565b5f60208284031215616257576162566153db565b5b5f6162648482850161622e565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f60028204905060018216806162b157607f821691505b6020821081036162c4576162c361626d565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f600883026163267fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff826162eb565b61633086836162eb565b95508019841693508086168417925050509392505050565b5f819050919050565b5f61636b616366616361846153e3565b616348565b6153e3565b9050919050565b5f819050919050565b61638483616351565b61639861639082616372565b8484546162f7565b825550505050565b5f90565b6163ac6163a0565b6163b781848461637b565b505050565b5b818110156163da576163cf5f826163a4565b6001810190506163bd565b5050565b601f82111561641f576163f0816162ca565b6163f9846162dc565b81016020851015616408578190505b61641c616414856162dc565b8301826163bc565b50505b505050565b5f82821c905092915050565b5f61643f5f1984600802616424565b1980831691505092915050565b5f6164578383616430565b9150826002028217905092915050565b61647082615d7d565b67ffffffffffffffff811115616489576164886157aa565b5b616493825461629a565b61649e8282856163de565b5f60209050601f8311600181146164cf575f84156164bd578287015190505b6164c7858261644c565b86555061652e565b601f1984166164dd866162ca565b5f5b82811015616504578489015182556001820191506020850194506020810190506164df565b86831015616521578489015161651d601f891682616430565b8355505b6001600288020188555050505b505050505050565b5f6080820190506165495f830187615ab4565b6165566020830186615ab4565b61656360408301856155ac565b81810360608301526165758184615f63565b905095945050505050565b5f6040820190506165935f830185615ab4565b6165a06020830184615ac3565b9392505050565b6165b081615a47565b81146165ba575f80fd5b50565b5f815190506165cb816165a7565b92915050565b5f602082840312156165e6576165e56153db565b5b5f6165f3848285016165bd565b91505092915050565b5f819050919050565b60028110616611575f80fd5b50565b5f8135905061662281616605565b92915050565b5f6166366020840184616614565b905092915050565b5f80fd5b5f80fd5b5f80fd5b5f808335600160200384360303811261666657616665616646565b5b83810192508235915060208301925067ffffffffffffffff82111561668e5761668d61663e565b5b6001820236038313156166a4576166a3616642565b5b509250929050565b5f6166b78385615d87565b93506166c4838584615852565b6166cd8361536a565b840190509392505050565b5f604083016166e95f840184616628565b6166f55f860182615d6e565b50616703602084018461664a565b85830360208701526167168382846166ac565b925050508091505092915050565b5f61672f83836166d8565b905092915050565b5f8235600160400383360303811261675257616751616646565b5b82810191505092915050565b5f602082019050919050565b5f6167758385615d18565b935083602084028501616787846165fc565b805f5b878110156167ca5784840389526167a18284616737565b6167ab8582616724565b94506167b68361675e565b925060208a0199505060018101905061678a565b50829750879450505050509392505050565b5f6167e78385615f53565b93506167f4838584615852565b6167fd8361536a565b840190509392505050565b5f60808201905061681b5f830189615ab4565b818103602083015261682e81878961676a565b905081810360408301526168438185876167dc565b90506168526060830184615ac3565b979650505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b5f823560016040038336030381126168b1576168b061688a565b5b80830191505092915050565b5f81356168c981616605565b80915050919050565b5f815f1b9050919050565b5f60ff6168e9846168d2565b9350801983169250808416831791505092915050565b5f61690982615d4b565b9050919050565b5f819050919050565b616922826168ff565b61693561692e82616910565b83546168dd565b8255505050565b5f80833560016020038436030381126169585761695761688a565b5b80840192508235915067ffffffffffffffff82111561697a5761697961688e565b5b60208301925060018202360383131561699657616995616892565b5b509250929050565b5f82905092915050565b6169b2838361699e565b67ffffffffffffffff8111156169cb576169ca6157aa565b5b6169d5825461629a565b6169e08282856163de565b5f601f831160018114616a0d575f84156169fb578287013590505b616a05858261644c565b865550616a6c565b601f198416616a1b866162ca565b5f5b82811015616a4257848901358255600182019150602085019450602081019050616a1d565b86831015616a5f5784890135616a5b601f891682616430565b8355505b6001600288020188555050505b50505050505050565b616a808383836169a8565b505050565b5f81015f830180616a95816168bd565b9050616aa18184616919565b5050506001810160208301616ab6818561693c565b616ac1818386616a75565b505050505050565b616ad38282616a85565b5050565b5f606082019050616aea5f830187615ab4565b8181036020830152616afc8186615c9b565b90508181036040830152616b1181848661676a565b905095945050505050565b5f606082019050616b2f5f830187615ab4565b8181036020830152616b428185876167dc565b9050616b516040830184615ac3565b95945050505050565b5f8154616b668161629a565b616b708186615f53565b9450600182165f8114616b8a5760018114616ba057616bd2565b60ff198316865281151560200286019350616bd2565b616ba9856162ca565b5f5b83811015616bca57815481890152600182019150602081019050616bab565b808801955050505b50505092915050565b5f606082019050616bee5f830186615ab4565b616bfb6020830185615ab4565b8181036040830152616c0d8184616b5a565b9050949350505050565b5f608082019050616c2a5f830189615ab4565b8181036020830152616c3d8187896167dc565b90508181036040830152616c528185876167dc565b9050616c616060830184615ac3565b979650505050505050565b5f606082019050616c7f5f830187615ab4565b8181036020830152616c918186615c9b565b90508181036040830152616ca68184866167dc565b905095945050505050565b7f4549503731323a20556e696e697469616c697a656400000000000000000000005f82015250565b5f616ce5601583615332565b9150616cf082616cb1565b602082019050919050565b5f6020820190508181035f830152616d1281616cd9565b9050919050565b5f8083356001602003843603038112616d3557616d3461688a565b5b80840192508235915067ffffffffffffffff821115616d5757616d5661688e565b5b602083019250602082023603831315616d7357616d72616892565b5b509250929050565b5f8083356001602003843603038112616d9757616d9661688a565b5b80840192508235915067ffffffffffffffff821115616db957616db861688e565b5b602083019250602082023603831315616dd557616dd4616892565b5b509250929050565b5f606082019050616df05f830186615ab4565b616dfd60208301856155ac565b8181036040830152616e0f8184615f63565b9050949350505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f60ff82169050919050565b5f8160f81b9050919050565b5f616e6882616e52565b9050919050565b616e80616e7b82616e46565b616e5e565b82525050565b5f819050919050565b616ea0616e9b826153e3565b616e86565b82525050565b5f616eb18286616e6f565b600182019150616ec18285616e8f565b602082019150616ed18284616e8f565b602082019150819050949350505050565b5f60208284031215616ef757616ef66153db565b5b5f616f0484828501616614565b91505092915050565b5f81905092915050565b5f616f228385616f0d565b9350616f2f838584615852565b82840190509392505050565b5f616f47828486616f17565b91508190509392505050565b616f5c81615d5d565b82525050565b5f606082019050616f755f830186615931565b616f826020830185616f53565b616f8f6040830184615931565b949350505050565b5f81519050919050565b5f81905092915050565b5f819050602082019050919050565b616fc381615928565b82525050565b5f616fd48383616fba565b60208301905092915050565b5f602082019050919050565b5f616ff682616f97565b6170008185616fa1565b935061700b83616fab565b805f5b8381101561703b5781516170228882616fc9565b975061702d83616fe0565b92505060018101905061700e565b5085935050505092915050565b5f6170538284616fec565b915081905092915050565b5f60a0820190506170715f830188615931565b61707e6020830187615ab4565b61708b6040830186615ab4565b6170986060830185615931565b6170a56080830184615931565b9695505050505050565b6170b881616e46565b82525050565b5f6020820190506170d15f8301846170af565b92915050565b5f80fd5b5f80fd5b5f67ffffffffffffffff8211156170f9576170f86157aa565b5b6171028261536a565b9050602081019050919050565b5f61712161711c846170df565b615808565b90508281526020810184848401111561713d5761713c6157a6565b5b617148848285615342565b509392505050565b5f82601f83011261716457617163615635565b5b815161717484826020860161710f565b91505092915050565b5f60808284031215617192576171916170d7565b5b61719c6080615808565b90505f6171ab84828501616127565b5f8301525060206171be84828501616127565b602083015250604082015167ffffffffffffffff8111156171e2576171e16170db565b5b6171ee84828501617150565b604083015250606082015167ffffffffffffffff811115617212576172116170db565b5b61721e84828501617150565b60608301525092915050565b5f6020828403121561723f5761723e6153db565b5b5f82015167ffffffffffffffff81111561725c5761725b6153df565b5b6172688482850161717d565b91505092915050565b61727a81615928565b8114617284575f80fd5b50565b5f8151905061729581617271565b92915050565b5f602082840312156172b0576172af6153db565b5b5f6172bd84828501617287565b91505092915050565b5f6040820190506172d95f830185615931565b6172e66020830184615ab4565b9392505050565b5f60208284031215617302576173016153db565b5b5f61730f84828501615792565b91505092915050565b5f819050815f5260205f209050919050565b601f82111561736b5761733c81617318565b617345846162dc565b81016020851015617354578190505b617368617360856162dc565b8301826163bc565b50505b505050565b61737982615328565b67ffffffffffffffff811115617392576173916157aa565b5b61739c825461629a565b6173a782828561732a565b5f60209050601f8311600181146173d8575f84156173c6578287015190505b6173d0858261644c565b865550617437565b601f1984166173e686617318565b5f5b8281101561740d578489015182556001820191506020850194506020810190506173e8565b8683101561742a5784890151617426601f891682616430565b8355505b6001600288020188555050505b505050505050565b5f6040820190506174525f830185615ac3565b61745f6020830184615ac3565b9392505050565b5f61747082615d7d565b61747a8185616f0d565b935061748a818560208601615342565b80840191505092915050565b5f6174a18284617466565b915081905092915050565b5f60a0820190506174bf5f830188615931565b6174cc6020830187615931565b6174d96040830186615931565b6174e66060830185615ab4565b6174f36080830184615ac3565b9695505050505050565b5f6080820190506175105f830187615931565b61751d60208301866170af565b61752a6040830185615931565b6175376060830184615931565b9594505050505056fe507265704b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e49642943727367656e566572696669636174696f6e2875696e743235362063727349642c75696e74323536206d61784269744c656e6774682c6279746573206372734469676573742c627974657320657874726144617461294b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e49642c75696e74323536206b657949642c4b65794469676573745b5d206b6579446967657374732c627974657320657874726144617461294b65794469676573742875696e7438206b6579547970652c627974657320646967657374294b65794469676573742875696e7438206b6579547970652c62797465732064696765737429
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x014W_5`\xE0\x1C\x80cb\x97\x87\x87\x11a\0\xAAW\x80c\xAD<\xB1\xCC\x11a\0nW\x80c\xAD<\xB1\xCC\x14a\x03\xF9W\x80c\xBA\xFF!\x1E\x14a\x04#W\x80c\xC2\xC1\xFA\xEE\x14a\x04MW\x80c\xC5[\x87$\x14a\x04uW\x80c\xCA\xA3g\xDB\x14a\x04\xB2W\x80c\xD5/\x10\xEB\x14a\x04\xDAWa\x014V[\x80cb\x97\x87\x87\x14a\x03\x12W\x80cs[\x12c\x14a\x03:W\x80c\x84\xB0\x19n\x14a\x03dW\x80c\x93f\x08\xAE\x14a\x03\x94W\x80c\xA0\x07\x9E\x0F\x14a\x03\xD1Wa\x014V[\x80c<\x02\xF84\x11a\0\xFCW\x80c<\x02\xF84\x14a\x02\x18W\x80cE\xAF&\x1B\x14a\x02@W\x80cF\x10\xFF\xE8\x14a\x02|W\x80cO\x1E\xF2\x86\x14a\x02\xA4W\x80cR\xD1\x90-\x14a\x02\xC0W\x80cX\x9A\xDB\x0E\x14a\x02\xEAWa\x014V[\x80c\r\x8En,\x14a\x018W\x80c\x16\xC7\x13\xD9\x14a\x01bW\x80c\x17\x03\xC6\x1A\x14a\x01\x9EW\x80c\x19\xF4\xF62\x14a\x01\xC6W\x80c9\xF78\x10\x14a\x02\x02W[_\x80\xFD[4\x80\x15a\x01CW_\x80\xFD[Pa\x01La\x05\x04V[`@Qa\x01Y\x91\x90aS\xB2V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01mW_\x80\xFD[Pa\x01\x88`\x04\x806\x03\x81\x01\x90a\x01\x83\x91\x90aT\x16V[a\x05\x7FV[`@Qa\x01\x95\x91\x90aU(V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xA9W_\x80\xFD[Pa\x01\xC4`\x04\x806\x03\x81\x01\x90a\x01\xBF\x91\x90aT\x16V[a\x06PV[\0[4\x80\x15a\x01\xD1W_\x80\xFD[Pa\x01\xEC`\x04\x806\x03\x81\x01\x90a\x01\xE7\x91\x90aT\x16V[a\x08oV[`@Qa\x01\xF9\x91\x90aU\xBBV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\rW_\x80\xFD[Pa\x02\x16a\t\x1CV[\0[4\x80\x15a\x02#W_\x80\xFD[Pa\x02>`\x04\x806\x03\x81\x01\x90a\x029\x91\x90aU\xF7V[a\x0B?V[\0[4\x80\x15a\x02KW_\x80\xFD[Pa\x02f`\x04\x806\x03\x81\x01\x90a\x02a\x91\x90aT\x16V[a\x0E#V[`@Qa\x02s\x91\x90aU\xBBV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x87W_\x80\xFD[Pa\x02\xA2`\x04\x806\x03\x81\x01\x90a\x02\x9D\x91\x90aV\xEBV[a\x0E\xB8V[\0[a\x02\xBE`\x04\x806\x03\x81\x01\x90a\x02\xB9\x91\x90aX\xCEV[a\x169V[\0[4\x80\x15a\x02\xCBW_\x80\xFD[Pa\x02\xD4a\x16XV[`@Qa\x02\xE1\x91\x90aY@V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xF5W_\x80\xFD[Pa\x03\x10`\x04\x806\x03\x81\x01\x90a\x03\x0B\x91\x90aYYV[a\x16\x89V[\0[4\x80\x15a\x03\x1DW_\x80\xFD[Pa\x038`\x04\x806\x03\x81\x01\x90a\x033\x91\x90aY\xB6V[a\x1B\x0FV[\0[4\x80\x15a\x03EW_\x80\xFD[Pa\x03Na!\x8FV[`@Qa\x03[\x91\x90aZaV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03oW_\x80\xFD[Pa\x03xa\"@V[`@Qa\x03\x8B\x97\x96\x95\x94\x93\x92\x91\x90a[\x89V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x9FW_\x80\xFD[Pa\x03\xBA`\x04\x806\x03\x81\x01\x90a\x03\xB5\x91\x90aT\x16V[a#IV[`@Qa\x03\xC8\x92\x91\x90a^\x9BV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xDCW_\x80\xFD[Pa\x03\xF7`\x04\x806\x03\x81\x01\x90a\x03\xF2\x91\x90a^\xF3V[a&iV[\0[4\x80\x15a\x04\x04W_\x80\xFD[Pa\x04\ra-VV[`@Qa\x04\x1A\x91\x90aS\xB2V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04.W_\x80\xFD[Pa\x047a-\x8FV[`@Qa\x04D\x91\x90a_:V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04XW_\x80\xFD[Pa\x04s`\x04\x806\x03\x81\x01\x90a\x04n\x91\x90aT\x16V[a-\xA6V[\0[4\x80\x15a\x04\x80W_\x80\xFD[Pa\x04\x9B`\x04\x806\x03\x81\x01\x90a\x04\x96\x91\x90aT\x16V[a0\x10V[`@Qa\x04\xA9\x92\x91\x90a_\x9BV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\xBDW_\x80\xFD[Pa\x04\xD8`\x04\x806\x03\x81\x01\x90a\x04\xD3\x91\x90a_\xD0V[a2\x9BV[\0[4\x80\x15a\x04\xE5W_\x80\xFD[Pa\x04\xEEa5\xD7V[`@Qa\x04\xFB\x91\x90a_:V[`@Q\x80\x91\x03\x90\xF3[```@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x05E_a5\xEEV[a\x05O`\x01a5\xEEV[a\x05X_a5\xEEV[`@Q` \x01a\x05k\x94\x93\x92\x91\x90a`\xC9V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[``_a\x05\x8Aa6\xB8V[\x90P_\x81`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\x02\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x82\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x06BW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x05\xF9W[PPPPP\x92PPP\x91\x90PV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xADW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xD1\x91\x90aa;V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x07@W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x077\x91\x90aafV[`@Q\x80\x91\x03\x90\xFD[_a\x07Ia6\xB8V[\x90P\x80`\t\x01T\x82\x11\x80a\x07eWP`\xF8`\x05`\xFF\x16\x90\x1B\x82\x11\x15[\x15a\x07\xA7W\x81`@Q\x7F\xCB\xE9&V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\x9E\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x08\tW\x81`@Q\x7F\xDF\r\xB5\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\0\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[`\x01\x81`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F8O\x90\xFE\xFB\xCF\xAAh\xF2.\0\tJ\xEA\xA5++\xC6\x93\x93m,\xE1\xAF\xED\x12\x12R\x0BY\xB5\x8E\x82`@Qa\x08c\x91\x90a_:V[`@Q\x80\x91\x03\x90\xA1PPV[_\x80a\x08ya6\xB8V[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x08\xDCW\x82`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xD3\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\r\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x92PPP\x91\x90PV[`\x01a\t&a6\xDFV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\tgW`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02_a\tra7\x03V[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\t\xBAWP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\t\xF1W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\n\xAA`@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa7*V[_a\n\xB3a6\xB8V[\x90P`\xF8`\x03`\xFF\x16\x90\x1B\x81`\x04\x01\x81\x90UP`\xF8`\x04`\xFF\x16\x90\x1B\x81`\x05\x01\x81\x90UP`\xF8`\x05`\xFF\x16\x90\x1B\x81`\t\x01\x81\x90UPP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x0B3\x91\x90aa\xA1V[`@Q\x80\x91\x03\x90\xA1PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x9CW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xC0\x91\x90aa;V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0C/W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C&\x91\x90aafV[`@Q\x80\x91\x03\x90\xFD[_a\x0C8a6\xB8V[\x90P_\x81`\t\x01T\x90P`\xF8`\x05`\xFF\x16\x90\x1B\x81\x14\x15\x80\x15a\x0CwWP\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15a\x0C\xB9W\x80`@Q\x7F\x06\x1A\xC6\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\xB0\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[\x81`\t\x01_\x81T\x80\x92\x91\x90a\x0C\xCD\x90aa\xE7V[\x91\x90PUP_\x82`\t\x01T\x90P\x84\x83`\n\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x83\x83`\r\x01_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a\r'Wa\r&aUHV[[\x02\x17\x90UP_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x8AW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xAE\x91\x90abBV[\x90P_a\r\xBA\x82a7@V[\x90P\x80\x85`\x0E\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x90\x81a\r\xDC\x91\x90adgV[P\x7F\x8C\xF0\x15\x13\x93\xF8O\xD6\x94\xC5\xE3\x15\xCBt\xCC\x05\xB2G\xDE\nEO\xD9\xE9\x12\x9Cf\x1E\xFD\xF9@\x1D\x83\x88\x88\x84`@Qa\x0E\x12\x94\x93\x92\x91\x90ae6V[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[_\x80a\x0E-a6\xB8V[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x0E\x90W\x82`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\x87\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[\x80`\r\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x16W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F:\x91\x90abBV[\x90PsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cF\xC5\xBB\xBD\x823`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\x8B\x92\x91\x90ae\x80V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xA6W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xCA\x91\x90ae\xD1V[a\x10\x0BW3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\x02\x91\x90aafV[`@Q\x80\x91\x03\x90\xFD[_a\x10\x14a6\xB8V[\x90P\x80`\x05\x01T\x87\x11\x80a\x100WP`\xF8`\x04`\xFF\x16\x90\x1B\x87\x11\x15[\x15a\x10rW\x86`@Q\x7F\xAD\xFA\xB9\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10i\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[_\x86\x86\x90P\x03a\x10\xB9W\x86`@Q\x7F\xE6\xF9\x08;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\xB0\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x89\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x11&W`@Q\x7Fo\xBC\xDD+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x11\xCD\x82\x8A\x8A\x8A\x87`\x0E\x01_\x8F\x81R` \x01\x90\x81R` \x01_ \x80Ta\x11L\x90ab\x9AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x11x\x90ab\x9AV[\x80\x15a\x11\xC3W\x80`\x1F\x10a\x11\x9AWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x11\xC3V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x11\xA6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa7nV[\x90P_a\x11\xDB\x82\x88\x88a9OV[\x90P\x83_\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x12{W\x89\x81`@Q\x7F\x98\xFB\x95}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12r\x92\x91\x90ae\x80V[`@Q\x80\x91\x03\x90\xFD[`\x01\x84_\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x84`\x02\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_\x81\x80T\x90P\x90P\x7F*\xFEd\xFB:\xFD\xE8\xE2g\x8A\xEA\x84\xCF6\"?3\x0E/\xB1(m7\xAE\xD5s\xAB\x9C\xD1\xDBG\xC7\x8C\x8C\x8C\x8C\x8C3`@Qa\x13\xA5\x96\x95\x94\x93\x92\x91\x90ah\x08V[`@Q\x80\x91\x03\x90\xA1\x85`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x13\xDFWPa\x13\xDE\x81a9\xB5V[[\x15a\x16+W`\x01\x86`\x01\x01_\x8E\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_[\x8B\x8B\x90P\x81\x10\x15a\x14\x95W\x86`\x07\x01_\x8E\x81R` \x01\x90\x81R` \x01_ \x8C\x8C\x83\x81\x81\x10a\x14BWa\x14Aah]V[[\x90P` \x02\x81\x01\x90a\x14T\x91\x90ah\x96V[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P\x81\x81a\x14\x86\x91\x90aj\xC9V[PP\x80\x80`\x01\x01\x91PPa\x14\x11V[P\x83\x86`\x03\x01_\x8E\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x8B\x86`\x08\x01\x81\x90UP_a\x15Z\x87`\x0E\x01_\x8F\x81R` \x01\x90\x81R` \x01_ \x80Ta\x14\xD9\x90ab\x9AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15\x05\x90ab\x9AV[\x80\x15a\x15PW\x80`\x1F\x10a\x15'Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15PV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x153W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa:FV[\x90P_a\x15\xE9\x82\x85\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x15\xDFW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x15\x96W[PPPPPa;\xA6V[\x90P\x7F\xEB\x85\xC2m\xBC\xADF\xB8\nh\xA0\xF2L\xCE|,\x90\xF0\xA1\xFA\xDE\xD8A\x84\x13\x889\xFC\x9E\x80\xA2[\x8E\x82\x8F\x8F`@Qa\x16 \x94\x93\x92\x91\x90aj\xD7V[`@Q\x80\x91\x03\x90\xA1PP[PPPPPPPPPPPPV[a\x16Aa<\xEEV[a\x16J\x82a=\xD4V[a\x16T\x82\x82a>\xC7V[PPV[_a\x16aa?\xE5V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xE7W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x0B\x91\x90abBV[\x90PsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cF\xC5\xBB\xBD\x823`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\\\x92\x91\x90ae\x80V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17wW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x9B\x91\x90ae\xD1V[a\x17\xDCW3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17\xD3\x91\x90aafV[`@Q\x80\x91\x03\x90\xFD[_a\x17\xE5a6\xB8V[\x90P\x80`\x04\x01T\x85\x11\x80a\x18\x01WP`\xF8`\x03`\xFF\x16\x90\x1B\x85\x11\x15[\x15a\x18CW\x84`@Q\x7F\n\xB7\xF6\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18:\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[_a\x18M\x86a@lV[\x90P_a\x18[\x82\x87\x87a9OV[\x90P\x82_\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x18\xFBW\x86\x81`@Q\x7F3\xCA\x1F\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18\xF2\x92\x91\x90ae\x80V[`@Q\x80\x91\x03\x90\xFD[`\x01\x83_\x01_\x89\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x83`\x02\x01_\x89\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7FLq\\W4\xCE\\\x18\xC9\xC1.\x84\x96\xE5=*e\xF1\xEC8\x1DGiW\xF0\xF5\x96\xB3d\xA5\x9B\x0C\x88\x88\x883`@Qa\x1A\x19\x94\x93\x92\x91\x90ak\x1CV[`@Q\x80\x91\x03\x90\xA1\x83`\x01\x01_\x89\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x1AWWPa\x1AV\x81\x80T\x90Pa9\xB5V[[\x15a\x1B\x05W`\x01\x84`\x01\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x82\x84`\x03\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x81\x90UP_\x84`\x06\x01_\x8A\x81R` \x01\x90\x81R` \x01_ T\x90P\x7F:\x11a \xCC\xA5\xD4\xF0s\xCC\x1F\xC3\x1F\xF2a3\xAB{\x04\x99\xF2q/\xA0\x10\x02;\x87\xD5\xA1\xF9\xEE\x89\x82\x87`\x0E\x01_\x8D\x81R` \x01\x90\x81R` \x01_ `@Qa\x1A\xFB\x93\x92\x91\x90ak\xDBV[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPPV[_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BmW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\x91\x91\x90abBV[\x90PsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cF\xC5\xBB\xBD\x823`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B\xE2\x92\x91\x90ae\x80V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xFDW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C!\x91\x90ae\xD1V[a\x1CbW3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1CY\x91\x90aafV[`@Q\x80\x91\x03\x90\xFD[_a\x1Cka6\xB8V[\x90P\x80`\t\x01T\x87\x11\x80a\x1C\x87WP`\xF8`\x05`\xFF\x16\x90\x1B\x87\x11\x15[\x15a\x1C\xC9W\x86`@Q\x7F\x8D\x8C\x94\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1C\xC0\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[_\x81`\n\x01_\x89\x81R` \x01\x90\x81R` \x01_ T\x90P_a\x1D\x87\x89\x83\x8A\x8A\x87`\x0E\x01_\x8F\x81R` \x01\x90\x81R` \x01_ \x80Ta\x1D\x06\x90ab\x9AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1D2\x90ab\x9AV[\x80\x15a\x1D}W\x80`\x1F\x10a\x1DTWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1D}V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1D`W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa@\xC4V[\x90P_a\x1D\x95\x82\x88\x88a9OV[\x90P\x83_\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x1E5W\x89\x81`@Q\x7F\xFC\xF5\xA6\xE9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1E,\x92\x91\x90ae\x80V[`@Q\x80\x91\x03\x90\xFD[`\x01\x84_\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x84`\x02\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_\x81\x80T\x90P\x90P\x7F{\xF1\xB4,\x10\xE9I|\x87\x96 \xC5\xB7\xAF\xCE\xD1\x0B\xDA\x17\xD8\xC9\x0B\"\xF0\xE3\xBCk/\xD6\xCE\xD0\xBD\x8C\x8C\x8C\x8C\x8C3`@Qa\x1F_\x96\x95\x94\x93\x92\x91\x90al\x17V[`@Q\x80\x91\x03\x90\xA1\x85`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x1F\x99WPa\x1F\x98\x81a9\xB5V[[\x15a!\x81W`\x01\x86`\x01\x01_\x8E\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x8A\x8A\x87`\x0B\x01_\x8F\x81R` \x01\x90\x81R` \x01_ \x91\x82a\x1F\xEB\x92\x91\x90ai\xA8V[P\x83\x86`\x03\x01_\x8E\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x8B\x86`\x0C\x01\x81\x90UP_a \xB0\x87`\x0E\x01_\x8F\x81R` \x01\x90\x81R` \x01_ \x80Ta /\x90ab\x9AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta [\x90ab\x9AV[\x80\x15a \xA6W\x80`\x1F\x10a }Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a \xA6V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a \x89W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa:FV[\x90P_a!?\x82\x85\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a!5W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a \xECW[PPPPPa;\xA6V[\x90P\x7F\"X\xB7?\xAE\xD3?\xB2\xE2\xEAED\x03\xBE\xF9t\x92\x0C\xAFh*\xB3\xA7#HO\xCFgU;\x16\xA2\x8E\x82\x8F\x8F`@Qa!v\x94\x93\x92\x91\x90allV[`@Q\x80\x91\x03\x90\xA1PP[PPPPPPPPPPPPV[_\x80a!\x99a6\xB8V[\x90P_\x81`\x05\x01T\x90P`\xF8`\x04`\xFF\x16\x90\x1B\x81\x14\x15\x80\x15a!\xD8WP\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15a!\xE8W`\x01\x92PPPa\"=V[_\x82`\t\x01T\x90P`\xF8`\x05`\xFF\x16\x90\x1B\x81\x14\x15\x80\x15a\"%WP\x82`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15a\"6W`\x01\x93PPPPa\"=V[_\x93PPPP[\x90V[_``\x80_\x80_``_a\"RaAUV[\x90P_\x80\x1B\x81_\x01T\x14\x80\x15a\"mWP_\x80\x1B\x81`\x01\x01T\x14[a\"\xACW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\"\xA3\x90al\xFBV[`@Q\x80\x91\x03\x90\xFD[a\"\xB4aA|V[a\"\xBCaB\x1AV[F0_\x80\x1B_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\xDBWa\"\xDAaW\xAAV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a#\tW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94\x93\x92\x91\x90\x97P\x97P\x97P\x97P\x97P\x97P\x97PP\x90\x91\x92\x93\x94\x95\x96V[``\x80_a#Ua6\xB8V[\x90P\x80`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a#\xB8W\x83`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a#\xAF\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x82`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a$oW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a$&W[PPPPP\x90P_a%\x19\x84`\x0E\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x80Ta$\x98\x90ab\x9AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta$\xC4\x90ab\x9AV[\x80\x15a%\x0FW\x80`\x1F\x10a$\xE6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a%\x0FV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a$\xF2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa:FV[\x90P_a%&\x82\x84a;\xA6V[\x90P\x80\x85`\x07\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a&UW\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x01\x81\x11\x15a%\xA0Wa%\x9FaUHV[[`\x01\x81\x11\x15a%\xB2Wa%\xB1aUHV[[\x81R` \x01`\x01\x82\x01\x80Ta%\xC6\x90ab\x9AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta%\xF2\x90ab\x9AV[\x80\x15a&=W\x80`\x1F\x10a&\x14Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a&=V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a& W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a%\\V[PPPP\x90P\x96P\x96PPPPPP\x91P\x91V[`\x01a&sa6\xDFV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a&\xB4W`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02_a&\xBFa7\x03V[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a'\x07WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a'>W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa'\xF7`@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa7*V[_a(\0a6\xB8V[\x90Pa(\x0B\x84aB\xB8V[a(5\x84``\x015\x85\x80a\x01\0\x01\x90a($\x91\x90am\x19V[\x87a\x01 \x015\x88a\x02 \x015aC\xC8V[a(_\x84`\x80\x015\x85\x80a\x01@\x01\x90a(N\x91\x90am\x19V[\x87a\x01`\x015\x88a\x02 \x015aC\xC8V[a(\x89\x84`\xA0\x015\x85\x80a\x01\x80\x01\x90a(x\x91\x90am\x19V[\x87a\x01\xA0\x015\x88a\x02 \x015aC\xC8V[_\x84\x80`\xC0\x01\x90a(\x9A\x91\x90am{V[\x90P\x03a(\xE2W\x83``\x015`@Q\x7F\x16\xBB\xAF\x8D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a(\xD9\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[_\x84\x80`\xE0\x01\x90a(\xF3\x91\x90ai<V[\x90P\x03a);W\x83`\x80\x015`@Q\x7F\x16\xBB\xAF\x8D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a)2\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[\x83_\x015\x81`\x04\x01\x81\x90UP\x83` \x015\x81`\x05\x01\x81\x90UP\x83`@\x015\x81`\t\x01\x81\x90UP\x83``\x015\x81`\x08\x01\x81\x90UP\x83`\x80\x015\x81`\x0C\x01\x81\x90UP\x83``\x015\x81`\x06\x01_\x86`\xA0\x015\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x83`\xA0\x015\x81`\x06\x01_\x86``\x015\x81R` \x01\x90\x81R` \x01_ \x81\x90UP_[\x84\x80`\xC0\x01\x90a)\xCD\x91\x90am{V[\x90P\x81\x10\x15a*aW\x81`\x07\x01_\x86``\x015\x81R` \x01\x90\x81R` \x01_ \x85\x80`\xC0\x01\x90a)\xFD\x91\x90am{V[\x83\x81\x81\x10a*\x0EWa*\rah]V[[\x90P` \x02\x81\x01\x90a* \x91\x90ah\x96V[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P\x81\x81a*R\x91\x90aj\xC9V[PP\x80\x80`\x01\x01\x91PPa)\xBDV[P\x83\x80`\xE0\x01\x90a*r\x91\x90ai<V[\x82`\x0B\x01_\x87`\x80\x015\x81R` \x01\x90\x81R` \x01_ \x91\x82a*\x96\x92\x91\x90ai\xA8V[P\x83a\x01\xC0\x015\x81`\n\x01_\x86`\x80\x015\x81R` \x01\x90\x81R` \x01_ \x81\x90UPa*\xE3\x81\x85``\x015\x86\x80a\x01\0\x01\x90a*\xD2\x91\x90am\x19V[\x88a\x01 \x015\x89a\x02 \x015aE;V[a+\x0E\x81\x85`\x80\x015\x86\x80a\x01@\x01\x90a*\xFD\x91\x90am\x19V[\x88a\x01`\x015\x89a\x02 \x015aE;V[a+9\x81\x85`\xA0\x015\x86\x80a\x01\x80\x01\x90a+(\x91\x90am\x19V[\x88a\x01\xA0\x015\x89a\x02 \x015aE;V[`\x01\x81`\x01\x01_\x86`\xA0\x015\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01\x81`\x01\x01_\x86``\x015\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01\x81`\x01\x01_\x86`\x80\x015\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x83a\x01\xE0\x01` \x81\x01\x90a+\xDA\x91\x90a_\xD0V[\x81`\r\x01_\x86`\xA0\x015\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a,\x12Wa,\x11aUHV[[\x02\x17\x90UP\x83a\x02\0\x01` \x81\x01\x90a,+\x91\x90a_\xD0V[\x81`\r\x01_\x86`\x80\x015\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a,cWa,baUHV[[\x02\x17\x90UPa,v\x84a\x02 \x015a7@V[\x81`\x0E\x01_\x86`\xA0\x015\x81R` \x01\x90\x81R` \x01_ \x90\x81a,\x99\x91\x90adgV[Pa,\xA8\x84a\x02 \x015a7@V[\x81`\x0E\x01_\x86``\x015\x81R` \x01\x90\x81R` \x01_ \x90\x81a,\xCB\x91\x90adgV[Pa,\xDA\x84a\x02 \x015a7@V[\x81`\x0E\x01_\x86`\x80\x015\x81R` \x01\x90\x81R` \x01_ \x90\x81a,\xFD\x91\x90adgV[PP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa-I\x91\x90aa\xA1V[`@Q\x80\x91\x03\x90\xA1PPPV[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[_\x80a-\x99a6\xB8V[\x90P\x80`\x0C\x01T\x91PP\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.\x03W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.'\x91\x90aa;V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a.\x96W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.\x8D\x91\x90aafV[`@Q\x80\x91\x03\x90\xFD[_a.\x9Fa6\xB8V[\x90P\x80`\x05\x01T\x82\x11\x80a.\xBBWP`\xF8`\x04`\xFF\x16\x90\x1B\x82\x11\x15[\x15a.\xFDW\x81`@Q\x7F\xFC\xF2\xDBz\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.\xF4\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a/_W\x81`@Q\x7F\x92x\x9Bg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a/V\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[`\x01\x81`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x81`\x06\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P_\x81\x14a/\xD4W`\x01\x82`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP[\x7F+\x08{\x88K5\xA8\x1Dv\x9D\x1A\x1E\t(\x80\xF1\xDAV\xDE\x96NK3\x9E\xAB\xCB\x1FE\xF5\xFE2d\x83`@Qa0\x03\x91\x90a_:V[`@Q\x80\x91\x03\x90\xA1PPPV[``\x80_a0\x1Ca6\xB8V[\x90P\x80`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a0\x7FW\x83`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a0v\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x82`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a16W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a0\xEDW[PPPPP\x90P_a1\xE0\x84`\x0E\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x80Ta1_\x90ab\x9AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta1\x8B\x90ab\x9AV[\x80\x15a1\xD6W\x80`\x1F\x10a1\xADWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a1\xD6V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a1\xB9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa:FV[\x90P_a1\xED\x82\x84a;\xA6V[\x90P\x80\x85`\x0B\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80\x80Ta2\x0F\x90ab\x9AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta2;\x90ab\x9AV[\x80\x15a2\x86W\x80`\x1F\x10a2]Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a2\x86V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a2iW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x96P\x96PPPPPP\x91P\x91V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2\xF8W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\x1C\x91\x90aa;V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a3\x8BW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a3\x82\x91\x90aafV[`@Q\x80\x91\x03\x90\xFD[_a3\x94a6\xB8V[\x90P_\x81`\x05\x01T\x90P`\xF8`\x04`\xFF\x16\x90\x1B\x81\x14\x15\x80\x15a3\xD3WP\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15a4\x15W\x80`@Q\x7F;\x85=\xA8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a4\x0C\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[\x81`\x04\x01_\x81T\x80\x92\x91\x90a4)\x90aa\xE7V[\x91\x90PUP_\x82`\x04\x01T\x90P\x82`\x05\x01_\x81T\x80\x92\x91\x90a4J\x90aa\xE7V[\x91\x90PUP_\x83`\x05\x01T\x90P\x80\x84`\x06\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x81\x84`\x06\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x84\x84`\r\x01_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a4\xBCWa4\xBBaUHV[[\x02\x17\x90UP_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5\x1FW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5C\x91\x90abBV[\x90P_a5O\x82a7@V[\x90P\x80\x86`\x0E\x01_\x86\x81R` \x01\x90\x81R` \x01_ \x90\x81a5q\x91\x90adgV[P\x80\x86`\x0E\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x90\x81a5\x92\x91\x90adgV[P\x7F\xFB\xF5'H\x10\xB9O\x86\x97\x0C\x11G\xE8\xFF\xAE\xBE\xD2F\xEE\x97w\xD6\x95\xA6\x90\x04\xDCbV\xD1\xFE\x91\x84\x88\x83`@Qa5\xC6\x93\x92\x91\x90am\xDDV[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[_\x80a5\xE1a6\xB8V[\x90P\x80`\x08\x01T\x91PP\x90V[``_`\x01a5\xFC\x84aG V[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\x1AWa6\x19aW\xAAV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a6LW\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x01\x82\x01\x90P[`\x01\x15a6\xADW\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a6\xA2Wa6\xA1an\x19V[[\x04\x94P_\x85\x03a6YW[\x81\x93PPPP\x91\x90PV[_\x7F&\xFD\xAF\x8A,\xB2\r \xB5^6!\x89\x86\x90^SN\xE7\xA9p\xDD/\xA8'\x94nKt\x96\xDB\0\x90P\x90V[_a6\xE8a7\x03V[_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[a72aHqV[a7<\x82\x82aH\xB1V[PPV[```\x02\x82_`@Q` \x01a7X\x93\x92\x91\x90an\xA6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[_\x80\x84\x84\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7\x8DWa7\x8CaW\xAAV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a7\xBBW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_[\x85\x85\x90P\x81\x10\x15a8\xBFW`@Q\x80``\x01`@R\x80`%\x81R` \x01avE`%\x919\x80Q\x90` \x01 \x86\x86\x83\x81\x81\x10a7\xFEWa7\xFDah]V[[\x90P` \x02\x81\x01\x90a8\x10\x91\x90ah\x96V[_\x01` \x81\x01\x90a8!\x91\x90an\xE2V[\x87\x87\x84\x81\x81\x10a84Wa83ah]V[[\x90P` \x02\x81\x01\x90a8F\x91\x90ah\x96V[\x80` \x01\x90a8U\x91\x90ai<V[`@Qa8c\x92\x91\x90ao;V[`@Q\x80\x91\x03\x90 `@Q` \x01a8}\x93\x92\x91\x90aobV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x82\x81Q\x81\x10a8\xA6Wa8\xA5ah]V[[` \x02` \x01\x01\x81\x81RPP\x80\x80`\x01\x01\x91PPa7\xC0V[Pa9C`@Q\x80`\xC0\x01`@R\x80`\x82\x81R` \x01au\xC3`\x82\x919\x80Q\x90` \x01 \x88\x88\x84`@Q` \x01a8\xF6\x91\x90apHV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x87\x80Q\x90` \x01 `@Q` \x01a9(\x95\x94\x93\x92\x91\x90ap^V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 aI\x02V[\x91PP\x95\x94PPPPPV[_\x80a9\x9E\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPaI\x1BV[\x90Pa9\xAA\x813aIEV[\x80\x91PP\x93\x92PPPV[_\x80sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xB4r+\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:\x14W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:8\x91\x90abBV[\x90P\x80\x83\x10\x15\x91PP\x91\x90PV[_\x80\x82Q\x03a:\xD7WsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:\xACW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:\xD0\x91\x90abBV[\x90Pa;\xA1V[_\x82_\x81Q\x81\x10a:\xEBWa:\xEAah]V[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P`\x01`\xFF\x16\x81`\xFF\x16\x14\x80a;\x16WP`\x02`\xFF\x16\x81`\xFF\x16\x14[\x15a;dW`!\x83Q\x10\x15a;WW`@Q\x7F\x8B$\x8B`\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`!\x83\x01Q\x91PPa;\xA1V[\x80`@Q\x7F!9\xCC,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a;\x98\x91\x90ap\xBEV[`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[``_\x82Q\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a;\xC8Wa;\xC7aW\xAAV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a;\xFBW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a;\xE6W\x90P[P\x90P_[\x82\x81\x10\x15a<\xE2WsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c1\xFFA\xC8\x87\x87\x84\x81Q\x81\x10a<LWa<Kah]V[[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a<q\x92\x91\x90ae\x80V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<\x8BW=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<\xB3\x91\x90ar*V[``\x01Q\x82\x82\x81Q\x81\x10a<\xCAWa<\xC9ah]V[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa<\0V[P\x80\x92PPP\x92\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a=\x9BWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a=\x82aK\xAAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a=\xD2W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>1W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>U\x91\x90aa;V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a>\xC4W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a>\xBB\x91\x90aafV[`@Q\x80\x91\x03\x90\xFD[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a?/WP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a?,\x91\x90ar\x9BV[`\x01[a?pW\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a?g\x91\x90aafV[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14a?\xD6W\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a?\xCD\x91\x90aY@V[`@Q\x80\x91\x03\x90\xFD[a?\xE0\x83\x83aK\xFDV[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a@jW`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_a@\xBD`@Q\x80``\x01`@R\x80`,\x81R` \x01auA`,\x919\x80Q\x90` \x01 \x83`@Q` \x01a@\xA2\x92\x91\x90ar\xC6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 aI\x02V[\x90P\x91\x90PV[_aAJ`@Q\x80`\x80\x01`@R\x80`V\x81R` \x01aum`V\x919\x80Q\x90` \x01 \x87\x87\x87\x87`@Q` \x01a@\xFD\x92\x91\x90ao;V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x86\x80Q\x90` \x01 `@Q` \x01aA/\x95\x94\x93\x92\x91\x90ap^V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 aI\x02V[\x90P\x95\x94PPPPPV[_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x90P\x90V[``_aA\x87aAUV[\x90P\x80`\x02\x01\x80TaA\x98\x90ab\x9AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80TaA\xC4\x90ab\x9AV[\x80\x15aB\x0FW\x80`\x1F\x10aA\xE6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91aB\x0FV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11aA\xF2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[``_aB%aAUV[\x90P\x80`\x03\x01\x80TaB6\x90ab\x9AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80TaBb\x90ab\x9AV[\x80\x15aB\xADW\x80`\x1F\x10aB\x84Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91aB\xADV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11aB\x90W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[`\xF8`\x03`\xFF\x16\x90\x1B\x81`\xA0\x015\x11\x15\x80aB\xDAWP\x80`\xA0\x015\x81_\x015\x14\x15[\x15aC\x11W`@Q\x7F\xA4\xD3\xD4\xF2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xF8`\x04`\xFF\x16\x90\x1B\x81``\x015\x11\x15\x80aC4WP\x80``\x015\x81` \x015\x14\x15[\x15aCkW`@Q\x7F\xA4\xD3\xD4\xF2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xF8`\x05`\xFF\x16\x90\x1B\x81`\x80\x015\x11\x15\x80aC\x8EWP\x80`\x80\x015\x81`@\x015\x14\x15[\x15aC\xC5W`@Q\x7F\xA4\xD3\xD4\xF2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[_\x80\x1B\x82\x14\x80aC\xDAWP_\x84\x84\x90P\x14[\x15aD\x1CW\x84`@Q\x7FE\x02\xCB\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aD\x13\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[_[\x84\x84\x90P\x81\x10\x15aE3W_\x85\x85\x83\x81\x81\x10aD=WaD<ah]V[[\x90P` \x02\x01` \x81\x01\x90aDR\x91\x90ar\xEDV[\x90PsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cF\xC5\xBB\xBD\x84\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aD\xA3\x92\x91\x90ae\x80V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aD\xBEW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD\xE2\x91\x90ae\xD1V[aE%W\x86\x81`@Q\x7F\x8B\xD00\x97\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aE\x1C\x92\x91\x90ae\x80V[`@Q\x80\x91\x03\x90\xFD[P\x80\x80`\x01\x01\x91PPaD\x1EV[PPPPPPV[\x81\x86`\x03\x01_\x87\x81R` \x01\x90\x81R` \x01_ \x81\x90UP_[\x84\x84\x90P\x81\x10\x15aG\x17W_\x85\x85\x83\x81\x81\x10aEtWaEsah]V[[\x90P` \x02\x01` \x81\x01\x90aE\x89\x91\x90ar\xEDV[\x90P\x87`\x02\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x85\x81R` \x01\x90\x81R` \x01_ \x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c1\xFFA\xC8\x85\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aF[\x92\x91\x90ae\x80V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aFuW=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aF\x9D\x91\x90ar*V[` \x01Q\x90P`\x01\x89_\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPP\x80\x80`\x01\x01\x91PPaEUV[PPPPPPPV[_\x80_\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10aG|Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81aGrWaGqan\x19V[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10aG\xB9Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81aG\xAFWaG\xAEan\x19V[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10aG\xE8Wf#\x86\xF2o\xC1\0\0\x83\x81aG\xDEWaG\xDDan\x19V[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10aH\x11Wc\x05\xF5\xE1\0\x83\x81aH\x07WaH\x06an\x19V[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10aH6Wa'\x10\x83\x81aH,WaH+an\x19V[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10aHYW`d\x83\x81aHOWaHNan\x19V[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10aHhW`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[aHyaLoV[aH\xAFW`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[aH\xB9aHqV[_aH\xC2aAUV[\x90P\x82\x81`\x02\x01\x90\x81aH\xD5\x91\x90aspV[P\x81\x81`\x03\x01\x90\x81aH\xE7\x91\x90aspV[P_\x80\x1B\x81_\x01\x81\x90UP_\x80\x1B\x81`\x01\x01\x81\x90UPPPPV[_aI\x14aI\x0EaL\x8DV[\x83aL\x9BV[\x90P\x91\x90PV[_\x80_\x80aI)\x86\x86aL\xDBV[\x92P\x92P\x92PaI9\x82\x82aM0V[\x82\x93PPPP\x92\x91PPV[_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aI\xA3W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aI\xC7\x91\x90abBV[\x90PsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x94G\xCF\xD4\x82\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aJ\x18\x92\x91\x90ae\x80V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aJ3W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aJW\x91\x90ae\xD1V[aJ\x9AW\x82\x82`@Q\x7F\r\x86\xF5!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aJ\x91\x92\x91\x90at?V[`@Q\x80\x91\x03\x90\xFD[_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c1\xFFA\xC8\x83\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aJ\xEA\x92\x91\x90ae\x80V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aK\x04W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aK,\x91\x90ar*V[\x90P\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14aK\xA4W\x83\x83`@Q\x7F\r\x86\xF5!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aK\x9B\x92\x91\x90at?V[`@Q\x80\x91\x03\x90\xFD[PPPPV[_aK\xD6\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1BaN\x92V[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[aL\x06\x82aN\x9BV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15aLbWaL\\\x82\x82aOdV[PaLkV[aLjaO\xE4V[[PPV[_aLxa7\x03V[_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[_aL\x96aP V[\x90P\x90V[_`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x02\x82\x01R\x82`\"\x82\x01R`B\x81 \x91PP\x92\x91PPV[_\x80_`A\x84Q\x03aM\x1BW_\x80_` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90PaM\r\x88\x82\x85\x85aP\x83V[\x95P\x95P\x95PPPPaM)V[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15aMCWaMBaUHV[[\x82`\x03\x81\x11\x15aMVWaMUaUHV[[\x03\x15aN\x8EW`\x01`\x03\x81\x11\x15aMpWaMoaUHV[[\x82`\x03\x81\x11\x15aM\x83WaM\x82aUHV[[\x03aM\xBAW`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15aM\xCEWaM\xCDaUHV[[\x82`\x03\x81\x11\x15aM\xE1WaM\xE0aUHV[[\x03aN%W\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aN\x1C\x91\x90a_:V[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15aN8WaN7aUHV[[\x82`\x03\x81\x11\x15aNKWaNJaUHV[[\x03aN\x8DW\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aN\x84\x91\x90aY@V[`@Q\x80\x91\x03\x90\xFD[[PPV[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03aN\xF6W\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aN\xED\x91\x90aafV[`@Q\x80\x91\x03\x90\xFD[\x80aO\"\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1BaN\x92V[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@QaO\x8D\x91\x90at\x96V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14aO\xC5W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>aO\xCAV[``\x91P[P\x91P\x91PaO\xDA\x85\x83\x83aQjV[\x92PPP\x92\x91PPV[_4\x11\x15aP\x1EW`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0FaPJaQ\xF7V[aPRaRmV[F0`@Q` \x01aPh\x95\x94\x93\x92\x91\x90at\xACV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x80_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15aP\xBFW_`\x03\x85\x92P\x92P\x92PaQ`V[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@QaP\xE2\x94\x93\x92\x91\x90at\xFDV[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aQ\x02W=_\x80>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03aQSW_`\x01_\x80\x1B\x93P\x93P\x93PPaQ`V[\x80_\x80_\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[``\x82aQ\x7FWaQz\x82aR\xE4V[aQ\xEFV[_\x82Q\x14\x80\x15aQ\xA5WP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15aQ\xE7W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aQ\xDE\x91\x90aafV[`@Q\x80\x91\x03\x90\xFD[\x81\x90PaQ\xF0V[[\x93\x92PPPV[_\x80aR\x01aAUV[\x90P_aR\x0CaA|V[\x90P_\x81Q\x11\x15aR(W\x80\x80Q\x90` \x01 \x92PPPaRjV[_\x82_\x01T\x90P_\x80\x1B\x81\x14aRCW\x80\x93PPPPaRjV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x80aRwaAUV[\x90P_aR\x82aB\x1AV[\x90P_\x81Q\x11\x15aR\x9EW\x80\x80Q\x90` \x01 \x92PPPaR\xE1V[_\x82`\x01\x01T\x90P_\x80\x1B\x81\x14aR\xBAW\x80\x93PPPPaR\xE1V[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x81Q\x11\x15aR\xF6W\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15aS_W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90PaSDV[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_aS\x84\x82aS(V[aS\x8E\x81\x85aS2V[\x93PaS\x9E\x81\x85` \x86\x01aSBV[aS\xA7\x81aSjV[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaS\xCA\x81\x84aSzV[\x90P\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x81\x90P\x91\x90PV[aS\xF5\x81aS\xE3V[\x81\x14aS\xFFW_\x80\xFD[PV[_\x815\x90PaT\x10\x81aS\xECV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aT+WaT*aS\xDBV[[_aT8\x84\x82\x85\x01aT\x02V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_aT\x93\x82aTjV[\x90P\x91\x90PV[aT\xA3\x81aT\x89V[\x82RPPV[_aT\xB4\x83\x83aT\x9AV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aT\xD6\x82aTAV[aT\xE0\x81\x85aTKV[\x93PaT\xEB\x83aT[V[\x80_[\x83\x81\x10\x15aU\x1BW\x81QaU\x02\x88\x82aT\xA9V[\x97PaU\r\x83aT\xC0V[\x92PP`\x01\x81\x01\x90PaT\xEEV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaU@\x81\x84aT\xCCV[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x02\x81\x10aU\x86WaU\x85aUHV[[PV[_\x81\x90PaU\x96\x82aUuV[\x91\x90PV[_aU\xA5\x82aU\x89V[\x90P\x91\x90PV[aU\xB5\x81aU\x9BV[\x82RPPV[_` \x82\x01\x90PaU\xCE_\x83\x01\x84aU\xACV[\x92\x91PPV[`\x02\x81\x10aU\xE0W_\x80\xFD[PV[_\x815\x90PaU\xF1\x81aU\xD4V[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15aV\rWaV\x0CaS\xDBV[[_aV\x1A\x85\x82\x86\x01aT\x02V[\x92PP` aV+\x85\x82\x86\x01aU\xE3V[\x91PP\x92P\x92\x90PV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12aVVWaVUaV5V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aVsWaVraV9V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aV\x8FWaV\x8EaV=V[[\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12aV\xABWaV\xAAaV5V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aV\xC8WaV\xC7aV9V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15aV\xE4WaV\xE3aV=V[[\x92P\x92\x90PV[_\x80_\x80_``\x86\x88\x03\x12\x15aW\x04WaW\x03aS\xDBV[[_aW\x11\x88\x82\x89\x01aT\x02V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aW2WaW1aS\xDFV[[aW>\x88\x82\x89\x01aVAV[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aWaWaW`aS\xDFV[[aWm\x88\x82\x89\x01aV\x96V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[aW\x85\x81aT\x89V[\x81\x14aW\x8FW_\x80\xFD[PV[_\x815\x90PaW\xA0\x81aW|V[\x92\x91PPV[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[aW\xE0\x82aSjV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aW\xFFWaW\xFEaW\xAAV[[\x80`@RPPPV[_aX\x11aS\xD2V[\x90PaX\x1D\x82\x82aW\xD7V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aX<WaX;aW\xAAV[[aXE\x82aSjV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_aXraXm\x84aX\"V[aX\x08V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15aX\x8EWaX\x8DaW\xA6V[[aX\x99\x84\x82\x85aXRV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aX\xB5WaX\xB4aV5V[[\x815aX\xC5\x84\x82` \x86\x01aX`V[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15aX\xE4WaX\xE3aS\xDBV[[_aX\xF1\x85\x82\x86\x01aW\x92V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aY\x12WaY\x11aS\xDFV[[aY\x1E\x85\x82\x86\x01aX\xA1V[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[aY:\x81aY(V[\x82RPPV[_` \x82\x01\x90PaYS_\x83\x01\x84aY1V[\x92\x91PPV[_\x80_`@\x84\x86\x03\x12\x15aYpWaYoaS\xDBV[[_aY}\x86\x82\x87\x01aT\x02V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aY\x9EWaY\x9DaS\xDFV[[aY\xAA\x86\x82\x87\x01aV\x96V[\x92P\x92PP\x92P\x92P\x92V[_\x80_\x80_``\x86\x88\x03\x12\x15aY\xCFWaY\xCEaS\xDBV[[_aY\xDC\x88\x82\x89\x01aT\x02V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aY\xFDWaY\xFCaS\xDFV[[aZ\t\x88\x82\x89\x01aV\x96V[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZ,WaZ+aS\xDFV[[aZ8\x88\x82\x89\x01aV\x96V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[_\x81\x15\x15\x90P\x91\x90PV[aZ[\x81aZGV[\x82RPPV[_` \x82\x01\x90PaZt_\x83\x01\x84aZRV[\x92\x91PPV[_\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[aZ\xAE\x81aZzV[\x82RPPV[aZ\xBD\x81aS\xE3V[\x82RPPV[aZ\xCC\x81aT\x89V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a[\x04\x81aS\xE3V[\x82RPPV[_a[\x15\x83\x83aZ\xFBV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a[7\x82aZ\xD2V[a[A\x81\x85aZ\xDCV[\x93Pa[L\x83aZ\xECV[\x80_[\x83\x81\x10\x15a[|W\x81Qa[c\x88\x82a[\nV[\x97Pa[n\x83a[!V[\x92PP`\x01\x81\x01\x90Pa[OV[P\x85\x93PPPP\x92\x91PPV[_`\xE0\x82\x01\x90Pa[\x9C_\x83\x01\x8AaZ\xA5V[\x81\x81\x03` \x83\x01Ra[\xAE\x81\x89aSzV[\x90P\x81\x81\x03`@\x83\x01Ra[\xC2\x81\x88aSzV[\x90Pa[\xD1``\x83\x01\x87aZ\xB4V[a[\xDE`\x80\x83\x01\x86aZ\xC3V[a[\xEB`\xA0\x83\x01\x85aY1V[\x81\x81\x03`\xC0\x83\x01Ra[\xFD\x81\x84a[-V[\x90P\x98\x97PPPPPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a\\N\x82aS(V[a\\X\x81\x85a\\4V[\x93Pa\\h\x81\x85` \x86\x01aSBV[a\\q\x81aSjV[\x84\x01\x91PP\x92\x91PPV[_a\\\x87\x83\x83a\\DV[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\\\xA5\x82a\\\x0BV[a\\\xAF\x81\x85a\\\x15V[\x93P\x83` \x82\x02\x85\x01a\\\xC1\x85a\\%V[\x80_[\x85\x81\x10\x15a\\\xFCW\x84\x84\x03\x89R\x81Qa\\\xDD\x85\x82a\\|V[\x94Pa\\\xE8\x83a\\\x8FV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa\\\xC4V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[`\x02\x81\x10a]HWa]GaUHV[[PV[_\x81\x90Pa]X\x82a]7V[\x91\x90PV[_a]g\x82a]KV[\x90P\x91\x90PV[a]w\x81a]]V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a]\xA1\x82a]}V[a]\xAB\x81\x85a]\x87V[\x93Pa]\xBB\x81\x85` \x86\x01aSBV[a]\xC4\x81aSjV[\x84\x01\x91PP\x92\x91PPV[_`@\x83\x01_\x83\x01Qa]\xE4_\x86\x01\x82a]nV[P` \x83\x01Q\x84\x82\x03` \x86\x01Ra]\xFC\x82\x82a]\x97V[\x91PP\x80\x91PP\x92\x91PPV[_a^\x14\x83\x83a]\xCFV[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a^2\x82a]\x0EV[a^<\x81\x85a]\x18V[\x93P\x83` \x82\x02\x85\x01a^N\x85a](V[\x80_[\x85\x81\x10\x15a^\x89W\x84\x84\x03\x89R\x81Qa^j\x85\x82a^\tV[\x94Pa^u\x83a^\x1CV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa^QV[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra^\xB3\x81\x85a\\\x9BV[\x90P\x81\x81\x03` \x83\x01Ra^\xC7\x81\x84a^(V[\x90P\x93\x92PPPV[_\x80\xFD[_a\x02@\x82\x84\x03\x12\x15a^\xEAWa^\xE9a^\xD0V[[\x81\x90P\x92\x91PPV[_` \x82\x84\x03\x12\x15a_\x08Wa_\x07aS\xDBV[[_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a_%Wa_$aS\xDFV[[a_1\x84\x82\x85\x01a^\xD4V[\x91PP\x92\x91PPV[_` \x82\x01\x90Pa_M_\x83\x01\x84aZ\xB4V[\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a_m\x82a]}V[a_w\x81\x85a_SV[\x93Pa_\x87\x81\x85` \x86\x01aSBV[a_\x90\x81aSjV[\x84\x01\x91PP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra_\xB3\x81\x85a\\\x9BV[\x90P\x81\x81\x03` \x83\x01Ra_\xC7\x81\x84a_cV[\x90P\x93\x92PPPV[_` \x82\x84\x03\x12\x15a_\xE5Wa_\xE4aS\xDBV[[_a_\xF2\x84\x82\x85\x01aU\xE3V[\x91PP\x92\x91PPV[_\x81\x90P\x92\x91PPV[_a`\x0F\x82aS(V[a`\x19\x81\x85a_\xFBV[\x93Pa`)\x81\x85` \x86\x01aSBV[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a`i`\x02\x83a_\xFBV[\x91Pa`t\x82a`5V[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a`\xB3`\x01\x83a_\xFBV[\x91Pa`\xBE\x82a`\x7FV[`\x01\x82\x01\x90P\x91\x90PV[_a`\xD4\x82\x87a`\x05V[\x91Pa`\xDF\x82a`]V[\x91Pa`\xEB\x82\x86a`\x05V[\x91Pa`\xF6\x82a`\xA7V[\x91Paa\x02\x82\x85a`\x05V[\x91Paa\r\x82a`\xA7V[\x91Paa\x19\x82\x84a`\x05V[\x91P\x81\x90P\x95\x94PPPPPV[_\x81Q\x90Paa5\x81aW|V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aaPWaaOaS\xDBV[[_aa]\x84\x82\x85\x01aa'V[\x91PP\x92\x91PPV[_` \x82\x01\x90Paay_\x83\x01\x84aZ\xC3V[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[aa\x9B\x81aa\x7FV[\x82RPPV[_` \x82\x01\x90Paa\xB4_\x83\x01\x84aa\x92V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_aa\xF1\x82aS\xE3V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03ab#Wab\"aa\xBAV[[`\x01\x82\x01\x90P\x91\x90PV[_\x81Q\x90Pab<\x81aS\xECV[\x92\x91PPV[_` \x82\x84\x03\x12\x15abWWabVaS\xDBV[[_abd\x84\x82\x85\x01ab.V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80ab\xB1W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03ab\xC4Wab\xC3abmV[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02ac&\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82ab\xEBV[ac0\x86\x83ab\xEBV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_ackacfaca\x84aS\xE3V[acHV[aS\xE3V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[ac\x84\x83acQV[ac\x98ac\x90\x82acrV[\x84\x84Tab\xF7V[\x82UPPPPV[_\x90V[ac\xACac\xA0V[ac\xB7\x81\x84\x84ac{V[PPPV[[\x81\x81\x10\x15ac\xDAWac\xCF_\x82ac\xA4V[`\x01\x81\x01\x90Pac\xBDV[PPV[`\x1F\x82\x11\x15ad\x1FWac\xF0\x81ab\xCAV[ac\xF9\x84ab\xDCV[\x81\x01` \x85\x10\x15ad\x08W\x81\x90P[ad\x1Cad\x14\x85ab\xDCV[\x83\x01\x82ac\xBCV[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_ad?_\x19\x84`\x08\x02ad$V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_adW\x83\x83ad0V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[adp\x82a]}V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ad\x89Wad\x88aW\xAAV[[ad\x93\x82Tab\x9AV[ad\x9E\x82\x82\x85ac\xDEV[_` \x90P`\x1F\x83\x11`\x01\x81\x14ad\xCFW_\x84\x15ad\xBDW\x82\x87\x01Q\x90P[ad\xC7\x85\x82adLV[\x86UPae.V[`\x1F\x19\x84\x16ad\xDD\x86ab\xCAV[_[\x82\x81\x10\x15ae\x04W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pad\xDFV[\x86\x83\x10\x15ae!W\x84\x89\x01Qae\x1D`\x1F\x89\x16\x82ad0V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_`\x80\x82\x01\x90PaeI_\x83\x01\x87aZ\xB4V[aeV` \x83\x01\x86aZ\xB4V[aec`@\x83\x01\x85aU\xACV[\x81\x81\x03``\x83\x01Raeu\x81\x84a_cV[\x90P\x95\x94PPPPPV[_`@\x82\x01\x90Pae\x93_\x83\x01\x85aZ\xB4V[ae\xA0` \x83\x01\x84aZ\xC3V[\x93\x92PPPV[ae\xB0\x81aZGV[\x81\x14ae\xBAW_\x80\xFD[PV[_\x81Q\x90Pae\xCB\x81ae\xA7V[\x92\x91PPV[_` \x82\x84\x03\x12\x15ae\xE6Wae\xE5aS\xDBV[[_ae\xF3\x84\x82\x85\x01ae\xBDV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[`\x02\x81\x10af\x11W_\x80\xFD[PV[_\x815\x90Paf\"\x81af\x05V[\x92\x91PPV[_af6` \x84\x01\x84af\x14V[\x90P\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12affWafeafFV[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15af\x8EWaf\x8Daf>V[[`\x01\x82\x026\x03\x83\x13\x15af\xA4Waf\xA3afBV[[P\x92P\x92\x90PV[_af\xB7\x83\x85a]\x87V[\x93Paf\xC4\x83\x85\x84aXRV[af\xCD\x83aSjV[\x84\x01\x90P\x93\x92PPPV[_`@\x83\x01af\xE9_\x84\x01\x84af(V[af\xF5_\x86\x01\x82a]nV[Pag\x03` \x84\x01\x84afJV[\x85\x83\x03` \x87\x01Rag\x16\x83\x82\x84af\xACV[\x92PPP\x80\x91PP\x92\x91PPV[_ag/\x83\x83af\xD8V[\x90P\x92\x91PPV[_\x825`\x01`@\x03\x836\x03\x03\x81\x12agRWagQafFV[[\x82\x81\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_agu\x83\x85a]\x18V[\x93P\x83` \x84\x02\x85\x01ag\x87\x84ae\xFCV[\x80_[\x87\x81\x10\x15ag\xCAW\x84\x84\x03\x89Rag\xA1\x82\x84ag7V[ag\xAB\x85\x82ag$V[\x94Pag\xB6\x83ag^V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pag\x8AV[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_ag\xE7\x83\x85a_SV[\x93Pag\xF4\x83\x85\x84aXRV[ag\xFD\x83aSjV[\x84\x01\x90P\x93\x92PPPV[_`\x80\x82\x01\x90Pah\x1B_\x83\x01\x89aZ\xB4V[\x81\x81\x03` \x83\x01Rah.\x81\x87\x89agjV[\x90P\x81\x81\x03`@\x83\x01RahC\x81\x85\x87ag\xDCV[\x90PahR``\x83\x01\x84aZ\xC3V[\x97\x96PPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x825`\x01`@\x03\x836\x03\x03\x81\x12ah\xB1Wah\xB0ah\x8AV[[\x80\x83\x01\x91PP\x92\x91PPV[_\x815ah\xC9\x81af\x05V[\x80\x91PP\x91\x90PV[_\x81_\x1B\x90P\x91\x90PV[_`\xFFah\xE9\x84ah\xD2V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_ai\t\x82a]KV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[ai\"\x82ah\xFFV[ai5ai.\x82ai\x10V[\x83Tah\xDDV[\x82UPPPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aiXWaiWah\x8AV[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aizWaiyah\x8EV[[` \x83\x01\x92P`\x01\x82\x026\x03\x83\x13\x15ai\x96Wai\x95ah\x92V[[P\x92P\x92\x90PV[_\x82\x90P\x92\x91PPV[ai\xB2\x83\x83ai\x9EV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ai\xCBWai\xCAaW\xAAV[[ai\xD5\x82Tab\x9AV[ai\xE0\x82\x82\x85ac\xDEV[_`\x1F\x83\x11`\x01\x81\x14aj\rW_\x84\x15ai\xFBW\x82\x87\x015\x90P[aj\x05\x85\x82adLV[\x86UPajlV[`\x1F\x19\x84\x16aj\x1B\x86ab\xCAV[_[\x82\x81\x10\x15ajBW\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Paj\x1DV[\x86\x83\x10\x15aj_W\x84\x89\x015aj[`\x1F\x89\x16\x82ad0V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[aj\x80\x83\x83\x83ai\xA8V[PPPV[_\x81\x01_\x83\x01\x80aj\x95\x81ah\xBDV[\x90Paj\xA1\x81\x84ai\x19V[PPP`\x01\x81\x01` \x83\x01aj\xB6\x81\x85ai<V[aj\xC1\x81\x83\x86ajuV[PPPPPPV[aj\xD3\x82\x82aj\x85V[PPV[_``\x82\x01\x90Paj\xEA_\x83\x01\x87aZ\xB4V[\x81\x81\x03` \x83\x01Raj\xFC\x81\x86a\\\x9BV[\x90P\x81\x81\x03`@\x83\x01Rak\x11\x81\x84\x86agjV[\x90P\x95\x94PPPPPV[_``\x82\x01\x90Pak/_\x83\x01\x87aZ\xB4V[\x81\x81\x03` \x83\x01RakB\x81\x85\x87ag\xDCV[\x90PakQ`@\x83\x01\x84aZ\xC3V[\x95\x94PPPPPV[_\x81Takf\x81ab\x9AV[akp\x81\x86a_SV[\x94P`\x01\x82\x16_\x81\x14ak\x8AW`\x01\x81\x14ak\xA0Wak\xD2V[`\xFF\x19\x83\x16\x86R\x81\x15\x15` \x02\x86\x01\x93Pak\xD2V[ak\xA9\x85ab\xCAV[_[\x83\x81\x10\x15ak\xCAW\x81T\x81\x89\x01R`\x01\x82\x01\x91P` \x81\x01\x90Pak\xABV[\x80\x88\x01\x95PPP[PPP\x92\x91PPV[_``\x82\x01\x90Pak\xEE_\x83\x01\x86aZ\xB4V[ak\xFB` \x83\x01\x85aZ\xB4V[\x81\x81\x03`@\x83\x01Ral\r\x81\x84akZV[\x90P\x94\x93PPPPV[_`\x80\x82\x01\x90Pal*_\x83\x01\x89aZ\xB4V[\x81\x81\x03` \x83\x01Ral=\x81\x87\x89ag\xDCV[\x90P\x81\x81\x03`@\x83\x01RalR\x81\x85\x87ag\xDCV[\x90Pala``\x83\x01\x84aZ\xC3V[\x97\x96PPPPPPPV[_``\x82\x01\x90Pal\x7F_\x83\x01\x87aZ\xB4V[\x81\x81\x03` \x83\x01Ral\x91\x81\x86a\\\x9BV[\x90P\x81\x81\x03`@\x83\x01Ral\xA6\x81\x84\x86ag\xDCV[\x90P\x95\x94PPPPPV[\x7FEIP712: Uninitialized\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_al\xE5`\x15\x83aS2V[\x91Pal\xF0\x82al\xB1V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ram\x12\x81al\xD9V[\x90P\x91\x90PV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12am5Wam4ah\x8AV[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15amWWamVah\x8EV[[` \x83\x01\x92P` \x82\x026\x03\x83\x13\x15amsWamrah\x92V[[P\x92P\x92\x90PV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12am\x97Wam\x96ah\x8AV[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15am\xB9Wam\xB8ah\x8EV[[` \x83\x01\x92P` \x82\x026\x03\x83\x13\x15am\xD5Wam\xD4ah\x92V[[P\x92P\x92\x90PV[_``\x82\x01\x90Pam\xF0_\x83\x01\x86aZ\xB4V[am\xFD` \x83\x01\x85aU\xACV[\x81\x81\x03`@\x83\x01Ran\x0F\x81\x84a_cV[\x90P\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_`\xFF\x82\x16\x90P\x91\x90PV[_\x81`\xF8\x1B\x90P\x91\x90PV[_anh\x82anRV[\x90P\x91\x90PV[an\x80an{\x82anFV[an^V[\x82RPPV[_\x81\x90P\x91\x90PV[an\xA0an\x9B\x82aS\xE3V[an\x86V[\x82RPPV[_an\xB1\x82\x86anoV[`\x01\x82\x01\x91Pan\xC1\x82\x85an\x8FV[` \x82\x01\x91Pan\xD1\x82\x84an\x8FV[` \x82\x01\x91P\x81\x90P\x94\x93PPPPV[_` \x82\x84\x03\x12\x15an\xF7Wan\xF6aS\xDBV[[_ao\x04\x84\x82\x85\x01af\x14V[\x91PP\x92\x91PPV[_\x81\x90P\x92\x91PPV[_ao\"\x83\x85ao\rV[\x93Pao/\x83\x85\x84aXRV[\x82\x84\x01\x90P\x93\x92PPPV[_aoG\x82\x84\x86ao\x17V[\x91P\x81\x90P\x93\x92PPPV[ao\\\x81a]]V[\x82RPPV[_``\x82\x01\x90Paou_\x83\x01\x86aY1V[ao\x82` \x83\x01\x85aoSV[ao\x8F`@\x83\x01\x84aY1V[\x94\x93PPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[ao\xC3\x81aY(V[\x82RPPV[_ao\xD4\x83\x83ao\xBAV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_ao\xF6\x82ao\x97V[ap\0\x81\x85ao\xA1V[\x93Pap\x0B\x83ao\xABV[\x80_[\x83\x81\x10\x15ap;W\x81Qap\"\x88\x82ao\xC9V[\x97Pap-\x83ao\xE0V[\x92PP`\x01\x81\x01\x90Pap\x0EV[P\x85\x93PPPP\x92\x91PPV[_apS\x82\x84ao\xECV[\x91P\x81\x90P\x92\x91PPV[_`\xA0\x82\x01\x90Papq_\x83\x01\x88aY1V[ap~` \x83\x01\x87aZ\xB4V[ap\x8B`@\x83\x01\x86aZ\xB4V[ap\x98``\x83\x01\x85aY1V[ap\xA5`\x80\x83\x01\x84aY1V[\x96\x95PPPPPPV[ap\xB8\x81anFV[\x82RPPV[_` \x82\x01\x90Pap\xD1_\x83\x01\x84ap\xAFV[\x92\x91PPV[_\x80\xFD[_\x80\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15ap\xF9Wap\xF8aW\xAAV[[aq\x02\x82aSjV[\x90P` \x81\x01\x90P\x91\x90PV[_aq!aq\x1C\x84ap\xDFV[aX\x08V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15aq=Waq<aW\xA6V[[aqH\x84\x82\x85aSBV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aqdWaqcaV5V[[\x81Qaqt\x84\x82` \x86\x01aq\x0FV[\x91PP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15aq\x92Waq\x91ap\xD7V[[aq\x9C`\x80aX\x08V[\x90P_aq\xAB\x84\x82\x85\x01aa'V[_\x83\x01RP` aq\xBE\x84\x82\x85\x01aa'V[` \x83\x01RP`@\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aq\xE2Waq\xE1ap\xDBV[[aq\xEE\x84\x82\x85\x01aqPV[`@\x83\x01RP``\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ar\x12War\x11ap\xDBV[[ar\x1E\x84\x82\x85\x01aqPV[``\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15ar?War>aS\xDBV[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ar\\War[aS\xDFV[[arh\x84\x82\x85\x01aq}V[\x91PP\x92\x91PPV[arz\x81aY(V[\x81\x14ar\x84W_\x80\xFD[PV[_\x81Q\x90Par\x95\x81arqV[\x92\x91PPV[_` \x82\x84\x03\x12\x15ar\xB0War\xAFaS\xDBV[[_ar\xBD\x84\x82\x85\x01ar\x87V[\x91PP\x92\x91PPV[_`@\x82\x01\x90Par\xD9_\x83\x01\x85aY1V[ar\xE6` \x83\x01\x84aZ\xB4V[\x93\x92PPPV[_` \x82\x84\x03\x12\x15as\x02Was\x01aS\xDBV[[_as\x0F\x84\x82\x85\x01aW\x92V[\x91PP\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[`\x1F\x82\x11\x15askWas<\x81as\x18V[asE\x84ab\xDCV[\x81\x01` \x85\x10\x15asTW\x81\x90P[ashas`\x85ab\xDCV[\x83\x01\x82ac\xBCV[PP[PPPV[asy\x82aS(V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15as\x92Was\x91aW\xAAV[[as\x9C\x82Tab\x9AV[as\xA7\x82\x82\x85as*V[_` \x90P`\x1F\x83\x11`\x01\x81\x14as\xD8W_\x84\x15as\xC6W\x82\x87\x01Q\x90P[as\xD0\x85\x82adLV[\x86UPat7V[`\x1F\x19\x84\x16as\xE6\x86as\x18V[_[\x82\x81\x10\x15at\rW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pas\xE8V[\x86\x83\x10\x15at*W\x84\x89\x01Qat&`\x1F\x89\x16\x82ad0V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_`@\x82\x01\x90PatR_\x83\x01\x85aZ\xC3V[at_` \x83\x01\x84aZ\xC3V[\x93\x92PPPV[_atp\x82a]}V[atz\x81\x85ao\rV[\x93Pat\x8A\x81\x85` \x86\x01aSBV[\x80\x84\x01\x91PP\x92\x91PPV[_at\xA1\x82\x84atfV[\x91P\x81\x90P\x92\x91PPV[_`\xA0\x82\x01\x90Pat\xBF_\x83\x01\x88aY1V[at\xCC` \x83\x01\x87aY1V[at\xD9`@\x83\x01\x86aY1V[at\xE6``\x83\x01\x85aZ\xB4V[at\xF3`\x80\x83\x01\x84aZ\xC3V[\x96\x95PPPPPPV[_`\x80\x82\x01\x90Pau\x10_\x83\x01\x87aY1V[au\x1D` \x83\x01\x86ap\xAFV[au*`@\x83\x01\x85aY1V[au7``\x83\x01\x84aY1V[\x95\x94PPPPPV\xFEPrepKeygenVerification(uint256 prepKeygenId)CrsgenVerification(uint256 crsId,uint256 maxBitLength,bytes crsDigest,bytes extraData)KeygenVerification(uint256 prepKeygenId,uint256 keyId,KeyDigest[] keyDigests,bytes extraData)KeyDigest(uint8 keyType,bytes digest)KeyDigest(uint8 keyType,bytes digest)",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    /**```solidity
struct MigrationState { uint256 prepKeygenCounter; uint256 keyCounter; uint256 crsCounter; uint256 activeKeyId; uint256 activeCrsId; uint256 activePrepKeygenId; IKMSGeneration.KeyDigest[] activeKeyDigests; bytes activeCrsDigest; address[] keyConsensusTxSenders; bytes32 keyConsensusDigest; address[] crsConsensusTxSenders; bytes32 crsConsensusDigest; address[] prepKeygenConsensusTxSenders; bytes32 prepKeygenConsensusDigest; uint256 crsMaxBitLength; IKMSGeneration.ParamsType prepKeygenParamsType; IKMSGeneration.ParamsType crsParamsType; uint256 contextId; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MigrationState {
        #[allow(missing_docs)]
        pub prepKeygenCounter: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub keyCounter: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub crsCounter: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub activeKeyId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub activeCrsId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub activePrepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub activeKeyDigests: alloy::sol_types::private::Vec<
            <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub activeCrsDigest: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub keyConsensusTxSenders: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub keyConsensusDigest: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub crsConsensusTxSenders: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub crsConsensusDigest: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub prepKeygenConsensusTxSenders: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub prepKeygenConsensusDigest: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub crsMaxBitLength: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub prepKeygenParamsType: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub crsParamsType: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub contextId: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Array<IKMSGeneration::KeyDigest>,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<256>,
            IKMSGeneration::ParamsType,
            IKMSGeneration::ParamsType,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Vec<
                <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
            >,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::primitives::aliases::U256,
            <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
            <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<MigrationState> for UnderlyingRustTuple<'_> {
            fn from(value: MigrationState) -> Self {
                (
                    value.prepKeygenCounter,
                    value.keyCounter,
                    value.crsCounter,
                    value.activeKeyId,
                    value.activeCrsId,
                    value.activePrepKeygenId,
                    value.activeKeyDigests,
                    value.activeCrsDigest,
                    value.keyConsensusTxSenders,
                    value.keyConsensusDigest,
                    value.crsConsensusTxSenders,
                    value.crsConsensusDigest,
                    value.prepKeygenConsensusTxSenders,
                    value.prepKeygenConsensusDigest,
                    value.crsMaxBitLength,
                    value.prepKeygenParamsType,
                    value.crsParamsType,
                    value.contextId,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MigrationState {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    prepKeygenCounter: tuple.0,
                    keyCounter: tuple.1,
                    crsCounter: tuple.2,
                    activeKeyId: tuple.3,
                    activeCrsId: tuple.4,
                    activePrepKeygenId: tuple.5,
                    activeKeyDigests: tuple.6,
                    activeCrsDigest: tuple.7,
                    keyConsensusTxSenders: tuple.8,
                    keyConsensusDigest: tuple.9,
                    crsConsensusTxSenders: tuple.10,
                    crsConsensusDigest: tuple.11,
                    prepKeygenConsensusTxSenders: tuple.12,
                    prepKeygenConsensusDigest: tuple.13,
                    crsMaxBitLength: tuple.14,
                    prepKeygenParamsType: tuple.15,
                    crsParamsType: tuple.16,
                    contextId: tuple.17,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for MigrationState {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for MigrationState {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.prepKeygenCounter),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.keyCounter),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.crsCounter),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.activeKeyId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.activeCrsId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.activePrepKeygenId),
                    <alloy::sol_types::sol_data::Array<
                        IKMSGeneration::KeyDigest,
                    > as alloy_sol_types::SolType>::tokenize(&self.activeKeyDigests),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.activeCrsDigest,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.keyConsensusTxSenders,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.keyConsensusDigest),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.crsConsensusTxSenders,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.crsConsensusDigest),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.prepKeygenConsensusTxSenders,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.prepKeygenConsensusDigest,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.crsMaxBitLength),
                    <IKMSGeneration::ParamsType as alloy_sol_types::SolType>::tokenize(
                        &self.prepKeygenParamsType,
                    ),
                    <IKMSGeneration::ParamsType as alloy_sol_types::SolType>::tokenize(
                        &self.crsParamsType,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.contextId),
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
        impl alloy_sol_types::SolType for MigrationState {
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
        impl alloy_sol_types::SolStruct for MigrationState {
            const NAME: &'static str = "MigrationState";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "MigrationState(uint256 prepKeygenCounter,uint256 keyCounter,uint256 crsCounter,uint256 activeKeyId,uint256 activeCrsId,uint256 activePrepKeygenId,IKMSGeneration.KeyDigest[] activeKeyDigests,bytes activeCrsDigest,address[] keyConsensusTxSenders,bytes32 keyConsensusDigest,address[] crsConsensusTxSenders,bytes32 crsConsensusDigest,address[] prepKeygenConsensusTxSenders,bytes32 prepKeygenConsensusDigest,uint256 crsMaxBitLength,uint8 prepKeygenParamsType,uint8 crsParamsType,uint256 contextId)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <IKMSGeneration::KeyDigest as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <IKMSGeneration::KeyDigest as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.prepKeygenCounter,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.keyCounter)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.crsCounter)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.activeKeyId)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.activeCrsId)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.activePrepKeygenId,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        IKMSGeneration::KeyDigest,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.activeKeyDigests,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.activeCrsDigest,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.keyConsensusTxSenders,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.keyConsensusDigest,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.crsConsensusTxSenders,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.crsConsensusDigest,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.prepKeygenConsensusTxSenders,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.prepKeygenConsensusDigest,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.crsMaxBitLength,
                        )
                        .0,
                    <IKMSGeneration::ParamsType as alloy_sol_types::SolType>::eip712_data_word(
                            &self.prepKeygenParamsType,
                        )
                        .0,
                    <IKMSGeneration::ParamsType as alloy_sol_types::SolType>::eip712_data_word(
                            &self.crsParamsType,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.contextId)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for MigrationState {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.prepKeygenCounter,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.keyCounter,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.crsCounter,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.activeKeyId,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.activeCrsId,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.activePrepKeygenId,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        IKMSGeneration::KeyDigest,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.activeKeyDigests,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.activeCrsDigest,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.keyConsensusTxSenders,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.keyConsensusDigest,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.crsConsensusTxSenders,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.crsConsensusDigest,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.prepKeygenConsensusTxSenders,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.prepKeygenConsensusDigest,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.crsMaxBitLength,
                    )
                    + <IKMSGeneration::ParamsType as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.prepKeygenParamsType,
                    )
                    + <IKMSGeneration::ParamsType as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.crsParamsType,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.contextId,
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
                    &rust.prepKeygenCounter,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.keyCounter,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.crsCounter,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.activeKeyId,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.activeCrsId,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.activePrepKeygenId,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    IKMSGeneration::KeyDigest,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.activeKeyDigests,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.activeCrsDigest,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.keyConsensusTxSenders,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.keyConsensusDigest,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.crsConsensusTxSenders,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.crsConsensusDigest,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.prepKeygenConsensusTxSenders,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.prepKeygenConsensusDigest,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.crsMaxBitLength,
                    out,
                );
                <IKMSGeneration::ParamsType as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.prepKeygenParamsType,
                    out,
                );
                <IKMSGeneration::ParamsType as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.crsParamsType,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.contextId,
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
    /**Custom error with signature `AbortCrsgenAlreadyDone(uint256)` and selector `0xdf0db5fb`.
```solidity
error AbortCrsgenAlreadyDone(uint256 crsId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AbortCrsgenAlreadyDone {
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
        impl ::core::convert::From<AbortCrsgenAlreadyDone> for UnderlyingRustTuple<'_> {
            fn from(value: AbortCrsgenAlreadyDone) -> Self {
                (value.crsId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AbortCrsgenAlreadyDone {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { crsId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AbortCrsgenAlreadyDone {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AbortCrsgenAlreadyDone(uint256)";
            const SELECTOR: [u8; 4] = [223u8, 13u8, 181u8, 251u8];
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
    /**Custom error with signature `AbortCrsgenInvalidId(uint256)` and selector `0xcbe92656`.
```solidity
error AbortCrsgenInvalidId(uint256 crsId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AbortCrsgenInvalidId {
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
        impl ::core::convert::From<AbortCrsgenInvalidId> for UnderlyingRustTuple<'_> {
            fn from(value: AbortCrsgenInvalidId) -> Self {
                (value.crsId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AbortCrsgenInvalidId {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { crsId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AbortCrsgenInvalidId {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AbortCrsgenInvalidId(uint256)";
            const SELECTOR: [u8; 4] = [203u8, 233u8, 38u8, 86u8];
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
    /**Custom error with signature `AbortKeygenAlreadyDone(uint256)` and selector `0x92789b67`.
```solidity
error AbortKeygenAlreadyDone(uint256 keyId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AbortKeygenAlreadyDone {
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
        impl ::core::convert::From<AbortKeygenAlreadyDone> for UnderlyingRustTuple<'_> {
            fn from(value: AbortKeygenAlreadyDone) -> Self {
                (value.keyId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AbortKeygenAlreadyDone {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { keyId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AbortKeygenAlreadyDone {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AbortKeygenAlreadyDone(uint256)";
            const SELECTOR: [u8; 4] = [146u8, 120u8, 155u8, 103u8];
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
    /**Custom error with signature `AbortKeygenInvalidId(uint256)` and selector `0xfcf2db7a`.
```solidity
error AbortKeygenInvalidId(uint256 keyId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AbortKeygenInvalidId {
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
        impl ::core::convert::From<AbortKeygenInvalidId> for UnderlyingRustTuple<'_> {
            fn from(value: AbortKeygenInvalidId) -> Self {
                (value.keyId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AbortKeygenInvalidId {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { keyId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AbortKeygenInvalidId {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AbortKeygenInvalidId(uint256)";
            const SELECTOR: [u8; 4] = [252u8, 242u8, 219u8, 122u8];
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
    /**Custom error with signature `DeserializingExtraDataFail()` and selector `0x8b248b60`.
```solidity
error DeserializingExtraDataFail();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DeserializingExtraDataFail;
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
        impl ::core::convert::From<DeserializingExtraDataFail>
        for UnderlyingRustTuple<'_> {
            fn from(value: DeserializingExtraDataFail) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for DeserializingExtraDataFail {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DeserializingExtraDataFail {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DeserializingExtraDataFail()";
            const SELECTOR: [u8; 4] = [139u8, 36u8, 139u8, 96u8];
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
    /**Custom error with signature `InvalidMigrationConsensusState(uint256)` and selector `0x4502cbf1`.
```solidity
error InvalidMigrationConsensusState(uint256 requestId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidMigrationConsensusState {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<InvalidMigrationConsensusState>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidMigrationConsensusState) -> Self {
                (value.requestId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidMigrationConsensusState {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { requestId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidMigrationConsensusState {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidMigrationConsensusState(uint256)";
            const SELECTOR: [u8; 4] = [69u8, 2u8, 203u8, 241u8];
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
    /**Custom error with signature `InvalidMigrationCounterState()` and selector `0xa4d3d4f2`.
```solidity
error InvalidMigrationCounterState();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidMigrationCounterState;
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
        impl ::core::convert::From<InvalidMigrationCounterState>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidMigrationCounterState) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidMigrationCounterState {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidMigrationCounterState {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidMigrationCounterState()";
            const SELECTOR: [u8; 4] = [164u8, 211u8, 212u8, 242u8];
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
    /**Custom error with signature `InvalidMigrationMaterial(uint256)` and selector `0x16bbaf8d`.
```solidity
error InvalidMigrationMaterial(uint256 requestId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidMigrationMaterial {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<InvalidMigrationMaterial>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidMigrationMaterial) -> Self {
                (value.requestId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidMigrationMaterial {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { requestId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidMigrationMaterial {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidMigrationMaterial(uint256)";
            const SELECTOR: [u8; 4] = [22u8, 187u8, 175u8, 141u8];
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
    /**Custom error with signature `KeyManagementRequestPending()` and selector `0x6fbcdd2b`.
```solidity
error KeyManagementRequestPending();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KeyManagementRequestPending;
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
        impl ::core::convert::From<KeyManagementRequestPending>
        for UnderlyingRustTuple<'_> {
            fn from(value: KeyManagementRequestPending) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for KeyManagementRequestPending {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KeyManagementRequestPending {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KeyManagementRequestPending()";
            const SELECTOR: [u8; 4] = [111u8, 188u8, 221u8, 43u8];
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
    /**Custom error with signature `NotHostOwner(address)` and selector `0x21bfda10`.
```solidity
error NotHostOwner(address sender);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotHostOwner {
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
        impl ::core::convert::From<NotHostOwner> for UnderlyingRustTuple<'_> {
            fn from(value: NotHostOwner) -> Self {
                (value.sender,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotHostOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { sender: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotHostOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotHostOwner(address)";
            const SELECTOR: [u8; 4] = [33u8, 191u8, 218u8, 16u8];
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
    /**Custom error with signature `UnknownMigrationConsensusTxSender(uint256,address)` and selector `0x8bd03097`.
```solidity
error UnknownMigrationConsensusTxSender(uint256 requestId, address txSender);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnknownMigrationConsensusTxSender {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub txSender: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<UnknownMigrationConsensusTxSender>
        for UnderlyingRustTuple<'_> {
            fn from(value: UnknownMigrationConsensusTxSender) -> Self {
                (value.requestId, value.txSender)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for UnknownMigrationConsensusTxSender {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    requestId: tuple.0,
                    txSender: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnknownMigrationConsensusTxSender {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnknownMigrationConsensusTxSender(uint256,address)";
            const SELECTOR: [u8; 4] = [139u8, 208u8, 48u8, 151u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.txSender,
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
    /**Custom error with signature `UnsupportedExtraDataVersion(uint8)` and selector `0x2139cc2c`.
```solidity
error UnsupportedExtraDataVersion(uint8 version);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnsupportedExtraDataVersion {
        #[allow(missing_docs)]
        pub version: u8,
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
        impl ::core::convert::From<UnsupportedExtraDataVersion>
        for UnderlyingRustTuple<'_> {
            fn from(value: UnsupportedExtraDataVersion) -> Self {
                (value.version,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for UnsupportedExtraDataVersion {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { version: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnsupportedExtraDataVersion {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnsupportedExtraDataVersion(uint8)";
            const SELECTOR: [u8; 4] = [33u8, 57u8, 204u8, 44u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.version),
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
    /**Event with signature `AbortCrsgen(uint256)` and selector `0x384f90fefbcfaa68f22e00094aeaa52b2bc693936d2ce1afed1212520b59b58e`.
```solidity
event AbortCrsgen(uint256 crsId);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct AbortCrsgen {
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for AbortCrsgen {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "AbortCrsgen(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                56u8, 79u8, 144u8, 254u8, 251u8, 207u8, 170u8, 104u8, 242u8, 46u8, 0u8,
                9u8, 74u8, 234u8, 165u8, 43u8, 43u8, 198u8, 147u8, 147u8, 109u8, 44u8,
                225u8, 175u8, 237u8, 18u8, 18u8, 82u8, 11u8, 89u8, 181u8, 142u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { crsId: data.0 }
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
        impl alloy_sol_types::private::IntoLogData for AbortCrsgen {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&AbortCrsgen> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &AbortCrsgen) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `AbortKeygen(uint256)` and selector `0x2b087b884b35a81d769d1a1e092880f1da56de964e4b339eabcb1f45f5fe3264`.
```solidity
event AbortKeygen(uint256 keyId);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct AbortKeygen {
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
        impl alloy_sol_types::SolEvent for AbortKeygen {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "AbortKeygen(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                43u8, 8u8, 123u8, 136u8, 75u8, 53u8, 168u8, 29u8, 118u8, 157u8, 26u8,
                30u8, 9u8, 40u8, 128u8, 241u8, 218u8, 86u8, 222u8, 150u8, 78u8, 75u8,
                51u8, 158u8, 171u8, 203u8, 31u8, 69u8, 245u8, 254u8, 50u8, 100u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { keyId: data.0 }
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
        impl alloy_sol_types::private::IntoLogData for AbortKeygen {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&AbortKeygen> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &AbortKeygen) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
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
    /**Event with signature `CrsgenRequest(uint256,uint256,uint8,bytes)` and selector `0x8cf0151393f84fd694c5e315cb74cc05b247de0a454fd9e9129c661efdf9401d`.
```solidity
event CrsgenRequest(uint256 crsId, uint256 maxBitLength, IKMSGeneration.ParamsType paramsType, bytes extraData);
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
        impl alloy_sol_types::SolEvent for CrsgenRequest {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                IKMSGeneration::ParamsType,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "CrsgenRequest(uint256,uint256,uint8,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                140u8, 240u8, 21u8, 19u8, 147u8, 248u8, 79u8, 214u8, 148u8, 197u8, 227u8,
                21u8, 203u8, 116u8, 204u8, 5u8, 178u8, 71u8, 222u8, 10u8, 69u8, 79u8,
                217u8, 233u8, 18u8, 156u8, 102u8, 30u8, 253u8, 249u8, 64u8, 29u8,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.crsId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxBitLength),
                    <IKMSGeneration::ParamsType as alloy_sol_types::SolType>::tokenize(
                        &self.paramsType,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
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
    /**Event with signature `KeygenRequest(uint256,uint256,bytes)` and selector `0x3a116120cca5d4f073cc1fc31ff26133ab7b0499f2712fa010023b87d5a1f9ee`.
```solidity
event KeygenRequest(uint256 prepKeygenId, uint256 keyId, bytes extraData);
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
        impl alloy_sol_types::SolEvent for KeygenRequest {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "KeygenRequest(uint256,uint256,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                58u8, 17u8, 97u8, 32u8, 204u8, 165u8, 212u8, 240u8, 115u8, 204u8, 31u8,
                195u8, 31u8, 242u8, 97u8, 51u8, 171u8, 123u8, 4u8, 153u8, 242u8, 113u8,
                47u8, 160u8, 16u8, 2u8, 59u8, 135u8, 213u8, 161u8, 249u8, 238u8,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.prepKeygenId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
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
    /**Event with signature `PrepKeygenRequest(uint256,uint8,bytes)` and selector `0xfbf5274810b94f86970c1147e8ffaebed246ee9777d695a69004dc6256d1fe91`.
```solidity
event PrepKeygenRequest(uint256 prepKeygenId, IKMSGeneration.ParamsType paramsType, bytes extraData);
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
        pub paramsType: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for PrepKeygenRequest {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                IKMSGeneration::ParamsType,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "PrepKeygenRequest(uint256,uint8,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                251u8, 245u8, 39u8, 72u8, 16u8, 185u8, 79u8, 134u8, 151u8, 12u8, 17u8,
                71u8, 232u8, 255u8, 174u8, 190u8, 210u8, 70u8, 238u8, 151u8, 119u8,
                214u8, 149u8, 166u8, 144u8, 4u8, 220u8, 98u8, 86u8, 209u8, 254u8, 145u8,
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
                    paramsType: data.1,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.prepKeygenId),
                    <IKMSGeneration::ParamsType as alloy_sol_types::SolType>::tokenize(
                        &self.paramsType,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
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
    /**Function with signature `abortCrsgen(uint256)` and selector `0x1703c61a`.
```solidity
function abortCrsgen(uint256 crsId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct abortCrsgenCall {
        #[allow(missing_docs)]
        pub crsId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`abortCrsgen(uint256)`](abortCrsgenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct abortCrsgenReturn {}
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
            impl ::core::convert::From<abortCrsgenCall> for UnderlyingRustTuple<'_> {
                fn from(value: abortCrsgenCall) -> Self {
                    (value.crsId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for abortCrsgenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { crsId: tuple.0 }
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
            impl ::core::convert::From<abortCrsgenReturn> for UnderlyingRustTuple<'_> {
                fn from(value: abortCrsgenReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for abortCrsgenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl abortCrsgenReturn {
            fn _tokenize(
                &self,
            ) -> <abortCrsgenCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for abortCrsgenCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = abortCrsgenReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "abortCrsgen(uint256)";
            const SELECTOR: [u8; 4] = [23u8, 3u8, 198u8, 26u8];
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
                abortCrsgenReturn::_tokenize(ret)
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
    /**Function with signature `abortKeygen(uint256)` and selector `0xc2c1faee`.
```solidity
function abortKeygen(uint256 keyId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct abortKeygenCall {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`abortKeygen(uint256)`](abortKeygenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct abortKeygenReturn {}
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
            impl ::core::convert::From<abortKeygenCall> for UnderlyingRustTuple<'_> {
                fn from(value: abortKeygenCall) -> Self {
                    (value.keyId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for abortKeygenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { keyId: tuple.0 }
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
            impl ::core::convert::From<abortKeygenReturn> for UnderlyingRustTuple<'_> {
                fn from(value: abortKeygenReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for abortKeygenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl abortKeygenReturn {
            fn _tokenize(
                &self,
            ) -> <abortKeygenCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for abortKeygenCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = abortKeygenReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "abortKeygen(uint256)";
            const SELECTOR: [u8; 4] = [194u8, 193u8, 250u8, 238u8];
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
                abortKeygenReturn::_tokenize(ret)
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
    /**Function with signature `hasPendingKeyManagementRequest()` and selector `0x735b1263`.
```solidity
function hasPendingKeyManagementRequest() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasPendingKeyManagementRequestCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`hasPendingKeyManagementRequest()`](hasPendingKeyManagementRequestCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasPendingKeyManagementRequestReturn {
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
            impl ::core::convert::From<hasPendingKeyManagementRequestCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: hasPendingKeyManagementRequestCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for hasPendingKeyManagementRequestCall {
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
            impl ::core::convert::From<hasPendingKeyManagementRequestReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: hasPendingKeyManagementRequestReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for hasPendingKeyManagementRequestReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for hasPendingKeyManagementRequestCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "hasPendingKeyManagementRequest()";
            const SELECTOR: [u8; 4] = [115u8, 91u8, 18u8, 99u8];
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
                        let r: hasPendingKeyManagementRequestReturn = r.into();
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
                        let r: hasPendingKeyManagementRequestReturn = r.into();
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
    #[derive()]
    /**Function with signature `initializeFromMigration((uint256,uint256,uint256,uint256,uint256,uint256,(uint8,bytes)[],bytes,address[],bytes32,address[],bytes32,address[],bytes32,uint256,uint8,uint8,uint256))` and selector `0xa0079e0f`.
```solidity
function initializeFromMigration(MigrationState memory state) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeFromMigrationCall {
        #[allow(missing_docs)]
        pub state: <MigrationState as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`initializeFromMigration((uint256,uint256,uint256,uint256,uint256,uint256,(uint8,bytes)[],bytes,address[],bytes32,address[],bytes32,address[],bytes32,uint256,uint8,uint8,uint256))`](initializeFromMigrationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeFromMigrationReturn {}
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
            type UnderlyingSolTuple<'a> = (MigrationState,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <MigrationState as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<initializeFromMigrationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializeFromMigrationCall) -> Self {
                    (value.state,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeFromMigrationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { state: tuple.0 }
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
            impl ::core::convert::From<initializeFromMigrationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializeFromMigrationReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeFromMigrationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl initializeFromMigrationReturn {
            fn _tokenize(
                &self,
            ) -> <initializeFromMigrationCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeFromMigrationCall {
            type Parameters<'a> = (MigrationState,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeFromMigrationReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initializeFromMigration((uint256,uint256,uint256,uint256,uint256,uint256,(uint8,bytes)[],bytes,address[],bytes32,address[],bytes32,address[],bytes32,uint256,uint8,uint8,uint256))";
            const SELECTOR: [u8; 4] = [160u8, 7u8, 158u8, 15u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<MigrationState as alloy_sol_types::SolType>::tokenize(&self.state),)
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                initializeFromMigrationReturn::_tokenize(ret)
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
    ///Container for all the [`KMSGeneration`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum KMSGenerationCalls {
        #[allow(missing_docs)]
        UPGRADE_INTERFACE_VERSION(UPGRADE_INTERFACE_VERSIONCall),
        #[allow(missing_docs)]
        abortCrsgen(abortCrsgenCall),
        #[allow(missing_docs)]
        abortKeygen(abortKeygenCall),
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
        hasPendingKeyManagementRequest(hasPendingKeyManagementRequestCall),
        #[allow(missing_docs)]
        initializeFromEmptyProxy(initializeFromEmptyProxyCall),
        #[allow(missing_docs)]
        initializeFromMigration(initializeFromMigrationCall),
        #[allow(missing_docs)]
        keygen(keygenCall),
        #[allow(missing_docs)]
        keygenResponse(keygenResponseCall),
        #[allow(missing_docs)]
        prepKeygenResponse(prepKeygenResponseCall),
        #[allow(missing_docs)]
        proxiableUUID(proxiableUUIDCall),
        #[allow(missing_docs)]
        upgradeToAndCall(upgradeToAndCallCall),
    }
    #[automatically_derived]
    impl KMSGenerationCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [13u8, 142u8, 110u8, 44u8],
            [22u8, 199u8, 19u8, 217u8],
            [23u8, 3u8, 198u8, 26u8],
            [25u8, 244u8, 246u8, 50u8],
            [57u8, 247u8, 56u8, 16u8],
            [60u8, 2u8, 248u8, 52u8],
            [69u8, 175u8, 38u8, 27u8],
            [70u8, 16u8, 255u8, 232u8],
            [79u8, 30u8, 242u8, 134u8],
            [82u8, 209u8, 144u8, 45u8],
            [88u8, 154u8, 219u8, 14u8],
            [98u8, 151u8, 135u8, 135u8],
            [115u8, 91u8, 18u8, 99u8],
            [132u8, 176u8, 25u8, 110u8],
            [147u8, 102u8, 8u8, 174u8],
            [160u8, 7u8, 158u8, 15u8],
            [173u8, 60u8, 177u8, 204u8],
            [186u8, 255u8, 33u8, 30u8],
            [194u8, 193u8, 250u8, 238u8],
            [197u8, 91u8, 135u8, 36u8],
            [202u8, 163u8, 103u8, 219u8],
            [213u8, 47u8, 16u8, 235u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for KMSGenerationCalls {
        const NAME: &'static str = "KMSGenerationCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 22usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::UPGRADE_INTERFACE_VERSION(_) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::abortCrsgen(_) => {
                    <abortCrsgenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::abortKeygen(_) => {
                    <abortKeygenCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::hasPendingKeyManagementRequest(_) => {
                    <hasPendingKeyManagementRequestCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initializeFromEmptyProxy(_) => {
                    <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initializeFromMigration(_) => {
                    <initializeFromMigrationCall as alloy_sol_types::SolCall>::SELECTOR
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
                    fn abortCrsgen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <abortCrsgenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::abortCrsgen)
                    }
                    abortCrsgen
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
                    fn hasPendingKeyManagementRequest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <hasPendingKeyManagementRequestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::hasPendingKeyManagementRequest)
                    }
                    hasPendingKeyManagementRequest
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
                    fn initializeFromMigration(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <initializeFromMigrationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::initializeFromMigration)
                    }
                    initializeFromMigration
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
                    fn abortKeygen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <abortKeygenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::abortKeygen)
                    }
                    abortKeygen
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
                    fn abortCrsgen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <abortCrsgenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::abortCrsgen)
                    }
                    abortCrsgen
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
                    fn hasPendingKeyManagementRequest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <hasPendingKeyManagementRequestCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::hasPendingKeyManagementRequest)
                    }
                    hasPendingKeyManagementRequest
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
                    fn initializeFromMigration(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <initializeFromMigrationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::initializeFromMigration)
                    }
                    initializeFromMigration
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
                    fn abortKeygen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <abortKeygenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::abortKeygen)
                    }
                    abortKeygen
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
                Self::abortCrsgen(inner) => {
                    <abortCrsgenCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::abortKeygen(inner) => {
                    <abortKeygenCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::hasPendingKeyManagementRequest(inner) => {
                    <hasPendingKeyManagementRequestCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initializeFromEmptyProxy(inner) => {
                    <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initializeFromMigration(inner) => {
                    <initializeFromMigrationCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::abortCrsgen(inner) => {
                    <abortCrsgenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::abortKeygen(inner) => {
                    <abortKeygenCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::hasPendingKeyManagementRequest(inner) => {
                    <hasPendingKeyManagementRequestCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::initializeFromMigration(inner) => {
                    <initializeFromMigrationCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum KMSGenerationErrors {
        #[allow(missing_docs)]
        AbortCrsgenAlreadyDone(AbortCrsgenAlreadyDone),
        #[allow(missing_docs)]
        AbortCrsgenInvalidId(AbortCrsgenInvalidId),
        #[allow(missing_docs)]
        AbortKeygenAlreadyDone(AbortKeygenAlreadyDone),
        #[allow(missing_docs)]
        AbortKeygenInvalidId(AbortKeygenInvalidId),
        #[allow(missing_docs)]
        AddressEmptyCode(AddressEmptyCode),
        #[allow(missing_docs)]
        CrsNotGenerated(CrsNotGenerated),
        #[allow(missing_docs)]
        CrsgenNotRequested(CrsgenNotRequested),
        #[allow(missing_docs)]
        CrsgenOngoing(CrsgenOngoing),
        #[allow(missing_docs)]
        DeserializingExtraDataFail(DeserializingExtraDataFail),
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
        InvalidInitialization(InvalidInitialization),
        #[allow(missing_docs)]
        InvalidMigrationConsensusState(InvalidMigrationConsensusState),
        #[allow(missing_docs)]
        InvalidMigrationCounterState(InvalidMigrationCounterState),
        #[allow(missing_docs)]
        InvalidMigrationMaterial(InvalidMigrationMaterial),
        #[allow(missing_docs)]
        KeyManagementRequestPending(KeyManagementRequestPending),
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
        NotHostOwner(NotHostOwner),
        #[allow(missing_docs)]
        NotInitializing(NotInitializing),
        #[allow(missing_docs)]
        NotInitializingFromEmptyProxy(NotInitializingFromEmptyProxy),
        #[allow(missing_docs)]
        NotKmsTxSender(NotKmsTxSender),
        #[allow(missing_docs)]
        PrepKeygenNotRequested(PrepKeygenNotRequested),
        #[allow(missing_docs)]
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        #[allow(missing_docs)]
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
        #[allow(missing_docs)]
        UnknownMigrationConsensusTxSender(UnknownMigrationConsensusTxSender),
        #[allow(missing_docs)]
        UnsupportedExtraDataVersion(UnsupportedExtraDataVersion),
    }
    #[automatically_derived]
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
            [22u8, 187u8, 175u8, 141u8],
            [33u8, 57u8, 204u8, 44u8],
            [33u8, 191u8, 218u8, 16u8],
            [51u8, 202u8, 31u8, 227u8],
            [59u8, 133u8, 61u8, 168u8],
            [69u8, 2u8, 203u8, 241u8],
            [76u8, 156u8, 140u8, 227u8],
            [111u8, 79u8, 115u8, 31u8],
            [111u8, 188u8, 221u8, 43u8],
            [132u8, 222u8, 19u8, 49u8],
            [139u8, 36u8, 139u8, 96u8],
            [139u8, 208u8, 48u8, 151u8],
            [141u8, 140u8, 148u8, 10u8],
            [146u8, 120u8, 155u8, 103u8],
            [152u8, 251u8, 149u8, 125u8],
            [153u8, 150u8, 179u8, 21u8],
            [164u8, 211u8, 212u8, 242u8],
            [170u8, 29u8, 73u8, 164u8],
            [173u8, 250u8, 185u8, 4u8],
            [174u8, 232u8, 99u8, 35u8],
            [179u8, 152u8, 151u8, 159u8],
            [203u8, 233u8, 38u8, 86u8],
            [214u8, 189u8, 162u8, 117u8],
            [215u8, 139u8, 206u8, 12u8],
            [215u8, 230u8, 188u8, 248u8],
            [218u8, 50u8, 208u8, 15u8],
            [223u8, 13u8, 181u8, 251u8],
            [224u8, 124u8, 141u8, 186u8],
            [230u8, 249u8, 8u8, 59u8],
            [246u8, 69u8, 238u8, 223u8],
            [249u8, 46u8, 232u8, 169u8],
            [252u8, 230u8, 152u8, 247u8],
            [252u8, 242u8, 219u8, 122u8],
            [252u8, 245u8, 166u8, 233u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for KMSGenerationErrors {
        const NAME: &'static str = "KMSGenerationErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 37usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AbortCrsgenAlreadyDone(_) => {
                    <AbortCrsgenAlreadyDone as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AbortCrsgenInvalidId(_) => {
                    <AbortCrsgenInvalidId as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AbortKeygenAlreadyDone(_) => {
                    <AbortKeygenAlreadyDone as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AbortKeygenInvalidId(_) => {
                    <AbortKeygenInvalidId as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AddressEmptyCode(_) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::SELECTOR
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
                Self::DeserializingExtraDataFail(_) => {
                    <DeserializingExtraDataFail as alloy_sol_types::SolError>::SELECTOR
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
                Self::InvalidInitialization(_) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidMigrationConsensusState(_) => {
                    <InvalidMigrationConsensusState as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidMigrationCounterState(_) => {
                    <InvalidMigrationCounterState as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidMigrationMaterial(_) => {
                    <InvalidMigrationMaterial as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KeyManagementRequestPending(_) => {
                    <KeyManagementRequestPending as alloy_sol_types::SolError>::SELECTOR
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
                Self::NotHostOwner(_) => {
                    <NotHostOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotInitializing(_) => {
                    <NotInitializing as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotInitializingFromEmptyProxy(_) => {
                    <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::SELECTOR
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
                Self::UnknownMigrationConsensusTxSender(_) => {
                    <UnknownMigrationConsensusTxSender as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnsupportedExtraDataVersion(_) => {
                    <UnsupportedExtraDataVersion as alloy_sol_types::SolError>::SELECTOR
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
                    fn InvalidMigrationMaterial(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <InvalidMigrationMaterial as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::InvalidMigrationMaterial)
                    }
                    InvalidMigrationMaterial
                },
                {
                    fn UnsupportedExtraDataVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <UnsupportedExtraDataVersion as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::UnsupportedExtraDataVersion)
                    }
                    UnsupportedExtraDataVersion
                },
                {
                    fn NotHostOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotHostOwner as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(KMSGenerationErrors::NotHostOwner)
                    }
                    NotHostOwner
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
                    fn InvalidMigrationConsensusState(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <InvalidMigrationConsensusState as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::InvalidMigrationConsensusState)
                    }
                    InvalidMigrationConsensusState
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
                    fn KeyManagementRequestPending(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KeyManagementRequestPending as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::KeyManagementRequestPending)
                    }
                    KeyManagementRequestPending
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
                    fn DeserializingExtraDataFail(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <DeserializingExtraDataFail as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::DeserializingExtraDataFail)
                    }
                    DeserializingExtraDataFail
                },
                {
                    fn UnknownMigrationConsensusTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <UnknownMigrationConsensusTxSender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::UnknownMigrationConsensusTxSender)
                    }
                    UnknownMigrationConsensusTxSender
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
                    fn AbortKeygenAlreadyDone(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <AbortKeygenAlreadyDone as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::AbortKeygenAlreadyDone)
                    }
                    AbortKeygenAlreadyDone
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
                    fn InvalidMigrationCounterState(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <InvalidMigrationCounterState as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::InvalidMigrationCounterState)
                    }
                    InvalidMigrationCounterState
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
                    fn AbortCrsgenInvalidId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <AbortCrsgenInvalidId as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::AbortCrsgenInvalidId)
                    }
                    AbortCrsgenInvalidId
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
                    fn AbortCrsgenAlreadyDone(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <AbortCrsgenAlreadyDone as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::AbortCrsgenAlreadyDone)
                    }
                    AbortCrsgenAlreadyDone
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
                    fn AbortKeygenInvalidId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <AbortKeygenInvalidId as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::AbortKeygenInvalidId)
                    }
                    AbortKeygenInvalidId
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
                    fn InvalidMigrationMaterial(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <InvalidMigrationMaterial as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::InvalidMigrationMaterial)
                    }
                    InvalidMigrationMaterial
                },
                {
                    fn UnsupportedExtraDataVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <UnsupportedExtraDataVersion as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::UnsupportedExtraDataVersion)
                    }
                    UnsupportedExtraDataVersion
                },
                {
                    fn NotHostOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotHostOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::NotHostOwner)
                    }
                    NotHostOwner
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
                    fn InvalidMigrationConsensusState(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <InvalidMigrationConsensusState as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::InvalidMigrationConsensusState)
                    }
                    InvalidMigrationConsensusState
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
                    fn KeyManagementRequestPending(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KeyManagementRequestPending as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::KeyManagementRequestPending)
                    }
                    KeyManagementRequestPending
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
                    fn DeserializingExtraDataFail(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <DeserializingExtraDataFail as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::DeserializingExtraDataFail)
                    }
                    DeserializingExtraDataFail
                },
                {
                    fn UnknownMigrationConsensusTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <UnknownMigrationConsensusTxSender as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::UnknownMigrationConsensusTxSender)
                    }
                    UnknownMigrationConsensusTxSender
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
                    fn AbortKeygenAlreadyDone(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <AbortKeygenAlreadyDone as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::AbortKeygenAlreadyDone)
                    }
                    AbortKeygenAlreadyDone
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
                    fn InvalidMigrationCounterState(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <InvalidMigrationCounterState as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::InvalidMigrationCounterState)
                    }
                    InvalidMigrationCounterState
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
                    fn AbortCrsgenInvalidId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <AbortCrsgenInvalidId as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::AbortCrsgenInvalidId)
                    }
                    AbortCrsgenInvalidId
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
                    fn AbortCrsgenAlreadyDone(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <AbortCrsgenAlreadyDone as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::AbortCrsgenAlreadyDone)
                    }
                    AbortCrsgenAlreadyDone
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
                    fn AbortKeygenInvalidId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <AbortKeygenInvalidId as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::AbortKeygenInvalidId)
                    }
                    AbortKeygenInvalidId
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
                Self::AbortCrsgenAlreadyDone(inner) => {
                    <AbortCrsgenAlreadyDone as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AbortCrsgenInvalidId(inner) => {
                    <AbortCrsgenInvalidId as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AbortKeygenAlreadyDone(inner) => {
                    <AbortKeygenAlreadyDone as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AbortKeygenInvalidId(inner) => {
                    <AbortKeygenInvalidId as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AddressEmptyCode(inner) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::DeserializingExtraDataFail(inner) => {
                    <DeserializingExtraDataFail as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::EmptyKeyDigests(inner) => {
                    <EmptyKeyDigests as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidInitialization(inner) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidMigrationConsensusState(inner) => {
                    <InvalidMigrationConsensusState as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidMigrationCounterState(inner) => {
                    <InvalidMigrationCounterState as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidMigrationMaterial(inner) => {
                    <InvalidMigrationMaterial as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KeyManagementRequestPending(inner) => {
                    <KeyManagementRequestPending as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::NotHostOwner(inner) => {
                    <NotHostOwner as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::UnknownMigrationConsensusTxSender(inner) => {
                    <UnknownMigrationConsensusTxSender as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UnsupportedExtraDataVersion(inner) => {
                    <UnsupportedExtraDataVersion as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AbortCrsgenAlreadyDone(inner) => {
                    <AbortCrsgenAlreadyDone as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AbortCrsgenInvalidId(inner) => {
                    <AbortCrsgenInvalidId as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AbortKeygenAlreadyDone(inner) => {
                    <AbortKeygenAlreadyDone as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AbortKeygenInvalidId(inner) => {
                    <AbortKeygenInvalidId as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::DeserializingExtraDataFail(inner) => {
                    <DeserializingExtraDataFail as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidInitialization(inner) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidMigrationConsensusState(inner) => {
                    <InvalidMigrationConsensusState as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidMigrationCounterState(inner) => {
                    <InvalidMigrationCounterState as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidMigrationMaterial(inner) => {
                    <InvalidMigrationMaterial as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KeyManagementRequestPending(inner) => {
                    <KeyManagementRequestPending as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::NotHostOwner(inner) => {
                    <NotHostOwner as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::UnknownMigrationConsensusTxSender(inner) => {
                    <UnknownMigrationConsensusTxSender as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UnsupportedExtraDataVersion(inner) => {
                    <UnsupportedExtraDataVersion as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`KMSGeneration`](self) events.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum KMSGenerationEvents {
        #[allow(missing_docs)]
        AbortCrsgen(AbortCrsgen),
        #[allow(missing_docs)]
        AbortKeygen(AbortKeygen),
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
        KeygenRequest(KeygenRequest),
        #[allow(missing_docs)]
        KeygenResponse(KeygenResponse),
        #[allow(missing_docs)]
        PrepKeygenRequest(PrepKeygenRequest),
        #[allow(missing_docs)]
        PrepKeygenResponse(PrepKeygenResponse),
        #[allow(missing_docs)]
        Upgraded(Upgraded),
    }
    #[automatically_derived]
    impl KMSGenerationEvents {
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
                43u8, 8u8, 123u8, 136u8, 75u8, 53u8, 168u8, 29u8, 118u8, 157u8, 26u8,
                30u8, 9u8, 40u8, 128u8, 241u8, 218u8, 86u8, 222u8, 150u8, 78u8, 75u8,
                51u8, 158u8, 171u8, 203u8, 31u8, 69u8, 245u8, 254u8, 50u8, 100u8,
            ],
            [
                56u8, 79u8, 144u8, 254u8, 251u8, 207u8, 170u8, 104u8, 242u8, 46u8, 0u8,
                9u8, 74u8, 234u8, 165u8, 43u8, 43u8, 198u8, 147u8, 147u8, 109u8, 44u8,
                225u8, 175u8, 237u8, 18u8, 18u8, 82u8, 11u8, 89u8, 181u8, 142u8,
            ],
            [
                58u8, 17u8, 97u8, 32u8, 204u8, 165u8, 212u8, 240u8, 115u8, 204u8, 31u8,
                195u8, 31u8, 242u8, 97u8, 51u8, 171u8, 123u8, 4u8, 153u8, 242u8, 113u8,
                47u8, 160u8, 16u8, 2u8, 59u8, 135u8, 213u8, 161u8, 249u8, 238u8,
            ],
            [
                76u8, 113u8, 92u8, 87u8, 52u8, 206u8, 92u8, 24u8, 201u8, 193u8, 46u8,
                132u8, 150u8, 229u8, 61u8, 42u8, 101u8, 241u8, 236u8, 56u8, 29u8, 71u8,
                105u8, 87u8, 240u8, 245u8, 150u8, 179u8, 100u8, 165u8, 155u8, 12u8,
            ],
            [
                123u8, 241u8, 180u8, 44u8, 16u8, 233u8, 73u8, 124u8, 135u8, 150u8, 32u8,
                197u8, 183u8, 175u8, 206u8, 209u8, 11u8, 218u8, 23u8, 216u8, 201u8, 11u8,
                34u8, 240u8, 227u8, 188u8, 107u8, 47u8, 214u8, 206u8, 208u8, 189u8,
            ],
            [
                140u8, 240u8, 21u8, 19u8, 147u8, 248u8, 79u8, 214u8, 148u8, 197u8, 227u8,
                21u8, 203u8, 116u8, 204u8, 5u8, 178u8, 71u8, 222u8, 10u8, 69u8, 79u8,
                217u8, 233u8, 18u8, 156u8, 102u8, 30u8, 253u8, 249u8, 64u8, 29u8,
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
            [
                251u8, 245u8, 39u8, 72u8, 16u8, 185u8, 79u8, 134u8, 151u8, 12u8, 17u8,
                71u8, 232u8, 255u8, 174u8, 190u8, 210u8, 70u8, 238u8, 151u8, 119u8,
                214u8, 149u8, 166u8, 144u8, 4u8, 220u8, 98u8, 86u8, 209u8, 254u8, 145u8,
            ],
        ];
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
                Some(<AbortCrsgen as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <AbortCrsgen as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::AbortCrsgen)
                }
                Some(<AbortKeygen as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <AbortKeygen as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::AbortKeygen)
                }
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
                Self::AbortCrsgen(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::AbortKeygen(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
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
                Self::KeygenRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::KeygenResponse(inner) => {
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
                Self::AbortCrsgen(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::AbortKeygen(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
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
                Self::KeygenRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::KeygenResponse(inner) => {
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
        provider: P,
    ) -> KMSGenerationInstance<P, N> {
        KMSGenerationInstance::<P, N>::new(address, provider)
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
        Output = alloy_contract::Result<KMSGenerationInstance<P, N>>,
    > {
        KMSGenerationInstance::<P, N>::deploy(provider)
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
        KMSGenerationInstance::<P, N>::deploy_builder(provider)
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
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > KMSGenerationInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`KMSGeneration`](self) contract instance.

See the [wrapper's documentation](`KMSGenerationInstance`) for more details.*/
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
        ) -> alloy_contract::Result<KMSGenerationInstance<P, N>> {
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
    #[automatically_derived]
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
        ///Creates a new call builder for the [`abortCrsgen`] function.
        pub fn abortCrsgen(
            &self,
            crsId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, abortCrsgenCall, N> {
            self.call_builder(&abortCrsgenCall { crsId })
        }
        ///Creates a new call builder for the [`abortKeygen`] function.
        pub fn abortKeygen(
            &self,
            keyId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, abortKeygenCall, N> {
            self.call_builder(&abortKeygenCall { keyId })
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
        ///Creates a new call builder for the [`hasPendingKeyManagementRequest`] function.
        pub fn hasPendingKeyManagementRequest(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, hasPendingKeyManagementRequestCall, N> {
            self.call_builder(&hasPendingKeyManagementRequestCall)
        }
        ///Creates a new call builder for the [`initializeFromEmptyProxy`] function.
        pub fn initializeFromEmptyProxy(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, initializeFromEmptyProxyCall, N> {
            self.call_builder(&initializeFromEmptyProxyCall)
        }
        ///Creates a new call builder for the [`initializeFromMigration`] function.
        pub fn initializeFromMigration(
            &self,
            state: <MigrationState as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, initializeFromMigrationCall, N> {
            self.call_builder(
                &initializeFromMigrationCall {
                    state,
                },
            )
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
    #[automatically_derived]
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
        ///Creates a new event filter for the [`AbortCrsgen`] event.
        pub fn AbortCrsgen_filter(&self) -> alloy_contract::Event<&P, AbortCrsgen, N> {
            self.event_filter::<AbortCrsgen>()
        }
        ///Creates a new event filter for the [`AbortKeygen`] event.
        pub fn AbortKeygen_filter(&self) -> alloy_contract::Event<&P, AbortKeygen, N> {
            self.event_filter::<AbortKeygen>()
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
