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
    error AbortCrsgenAlreadyDone(uint256 crsId);
    error AbortCrsgenInvalidId(uint256 crsId);
    error AbortKeygenAlreadyDone(uint256 prepKeygenId);
    error AbortKeygenInvalidId(uint256 prepKeygenId);
    error AddressEmptyCode(address target);
    error CrsAborted(uint256 crsId);
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
    error KeyAborted(uint256 keyId);
    error KeyManagementRequestPending();
    error KeyMaterialNotPublished(uint256 keyId);
    error KeyNotGenerated(uint256 keyId);
    error KeygenNotRequested(uint256 keyId);
    error KeygenOngoing(uint256 keyId);
    error KmsAlreadySignedForCrsgen(uint256 crsId, address kmsSigner);
    error KmsAlreadySignedForKeygen(uint256 keyId, address kmsSigner);
    error KmsAlreadySignedForPrepKeygen(uint256 prepKeygenId, address kmsSigner);
    error KmsSignerDoesNotMatchTxSender(address signerAddress, address txSenderAddress);
    error MismatchedMigrationArrays();
    error NotHostOwner(address sender);
    error NotInitializing();
    error NotInitializingFromEmptyProxy();
    error NotKmsTxSender(address txSenderAddress);
    error PrepKeygenNotRequested(uint256 prepKeygenId);
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);
    error UnsupportedExtraDataVersion(uint8 version);
    error UnsupportedMaterialVersion(uint256 version);

    event AbortCrsgen(uint256 crsId);
    event AbortKeygen(uint256 prepKeygenId);
    event ActivateCrs(uint256 crsId, string[] kmsNodeStorageUrls, bytes crsDigest);
    event ActivateKey(uint256 keyId, string[] kmsNodeStorageUrls, IKMSGeneration.KeyDigest[] keyDigests);
    event CrsgenRequest(uint256 crsId, uint256 maxBitLength, IKMSGeneration.ParamsType paramsType, bytes extraData);
    event CrsgenResponse(uint256 crsId, bytes crsDigest, bytes signature, address kmsTxSender);
    event EIP712DomainChanged();
    event Initialized(uint64 version);
    event KeyMaterialAdded(uint256 keyId, string[] kmsNodeStorageUrls, IKMSGeneration.KeyDigest[] keyDigests, uint256 materialVersion);
    event KeyMaterialMigrationScheduled(uint256 keyId, uint256[] hostChainIds, uint256[] hostMigrationBlocks, uint256 gatewayMigrationBlock, uint256 materialVersion);
    event KeygenRequest(uint256 prepKeygenId, uint256 keyId, bytes extraData);
    event KeygenResponse(uint256 keyId, IKMSGeneration.KeyDigest[] keyDigests, bytes signature, address kmsTxSender);
    event PrepKeygenRequest(uint256 prepKeygenId, IKMSGeneration.ParamsType paramsType, bytes extraData);
    event PrepKeygenResponse(uint256 prepKeygenId, bytes signature, address kmsTxSender);
    event Upgraded(address indexed implementation);

    constructor();

    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function abortCrsgen(uint256 crsId) external;
    function abortKeygen(uint256 prepKeygenId) external;
    function addKeyMaterials(uint256 keyId, IKMSGeneration.KeyDigest[] memory keyDigests, string[] memory kmsNodeStorageUrls, uint256 materialVersion) external;
    function crsgenRequest(uint256 maxBitLength, IKMSGeneration.ParamsType paramsType) external;
    function crsgenResponse(uint256 crsId, bytes memory crsDigest, bytes memory signature) external;
    function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
    function getActiveCrsId() external view returns (uint256);
    function getActiveKeyId() external view returns (uint256);
    function getConsensusTxSenders(uint256 requestId) external view returns (address[] memory);
    function getCrsCounter() external view returns (uint256);
    function getCrsMaterials(uint256 crsId) external view returns (string[] memory, bytes memory);
    function getCrsParamsType(uint256 crsId) external view returns (IKMSGeneration.ParamsType);
    function getKeyCounter() external view returns (uint256);
    function getKeyMaterialVersion(uint256 keyId) external view returns (uint256);
    function getKeyMaterials(uint256 keyId) external view returns (string[] memory, IKMSGeneration.KeyDigest[] memory);
    function getKeyParamsType(uint256 keyId) external view returns (IKMSGeneration.ParamsType);
    function getVersion() external pure returns (string memory);
    function initializeFromEmptyProxy() external;
    function isRequestDone(uint256 requestId) external view returns (bool);
    function keygen(IKMSGeneration.ParamsType paramsType, bytes memory extraData) external;
    function keygen(IKMSGeneration.ParamsType paramsType) external;
    function keygenResponse(uint256 keyId, IKMSGeneration.KeyDigest[] memory keyDigests, bytes memory signature) external;
    function prepKeygenResponse(uint256 prepKeygenId, bytes memory signature) external;
    function proxiableUUID() external view returns (bytes32);
    function reinitializeV2() external;
    function scheduleKeyMaterialMigration(uint256 keyId, uint256[] memory hostChainIds, uint256[] memory hostMigrationBlocks, uint256 gatewayMigrationBlock, uint256 materialVersion) external;
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
        "name": "prepKeygenId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "addKeyMaterials",
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
        "name": "kmsNodeStorageUrls",
        "type": "string[]",
        "internalType": "string[]"
      },
      {
        "name": "materialVersion",
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
    "name": "getCrsCounter",
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
    "name": "getKeyCounter",
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
    "name": "getKeyMaterialVersion",
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
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "isRequestDone",
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
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "keygen",
    "inputs": [
      {
        "name": "paramsType",
        "type": "uint8",
        "internalType": "enum IKMSGeneration.ParamsType"
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
    "name": "reinitializeV2",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "scheduleKeyMaterialMigration",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "hostChainIds",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "hostMigrationBlocks",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "gatewayMigrationBlock",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "materialVersion",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
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
        "name": "prepKeygenId",
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
    "name": "KeyMaterialAdded",
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
      },
      {
        "name": "materialVersion",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "KeyMaterialMigrationScheduled",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "hostChainIds",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      },
      {
        "name": "hostMigrationBlocks",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      },
      {
        "name": "gatewayMigrationBlock",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "materialVersion",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
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
        "name": "prepKeygenId",
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
        "name": "prepKeygenId",
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
    "name": "CrsAborted",
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
    "name": "KeyAborted",
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
    "name": "KeyManagementRequestPending",
    "inputs": []
  },
  {
    "type": "error",
    "name": "KeyMaterialNotPublished",
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
    "name": "MismatchedMigrationArrays",
    "inputs": []
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
    "name": "UnsupportedExtraDataVersion",
    "inputs": [
      {
        "name": "version",
        "type": "uint8",
        "internalType": "uint8"
      }
    ]
  },
  {
    "type": "error",
    "name": "UnsupportedMaterialVersion",
    "inputs": [
      {
        "name": "version",
        "type": "uint256",
        "internalType": "uint256"
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
    ///0x60a06040523073ffffffffffffffffffffffffffffffffffffffff1660809073ffffffffffffffffffffffffffffffffffffffff1681525034801562000043575f80fd5b50620000546200005a60201b60201c565b620001c4565b5f6200006b6200015e60201b60201c565b9050805f0160089054906101000a900460ff1615620000b6576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b67ffffffffffffffff8016815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff16146200015b5767ffffffffffffffff815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d267ffffffffffffffff604051620001529190620001a9565b60405180910390a15b50565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b5f67ffffffffffffffff82169050919050565b620001a38162000185565b82525050565b5f602082019050620001be5f83018462000198565b92915050565b608051617655620001eb5f395f8181613a2d01528181613a820152613d2401526176555ff3fe6080604052600436106101b6575f3560e01c8063589adb0e116100eb578063baff211e11610089578063c55b872411610063578063c55b8724146105c3578063caa367db14610600578063d52f10eb14610628578063f0f8cbc614610652576101b6565b8063baff211e1461055b578063c2c1faee14610585578063c4115874146105ad576101b6565b80637fc80c5d116100c55780637fc80c5d1461049c57806384b0196e146104c4578063936608ae146104f4578063ad3cb1cc14610531576101b6565b8063589adb0e14610424578063608b4ad51461044c5780636297878714610474576101b6565b80633c02f834116101585780634610ffe8116101325780634610ffe81461038e5780634c298229146103b65780634f1ef286146103de57806352d1902d146103fa576101b6565b80633c02f834146102ee5780633d5ec7e31461031657806345af261b14610352576101b6565b80631703c61a116101945780631703c61a1461024a57806319f4f6321461027257806339f73810146102ae5780633ac50072146102c4576101b6565b80630b680733146101ba5780630d8e6e2c146101e457806316c713d91461020e575b5f80fd5b3480156101c5575f80fd5b506101ce61068e565b6040516101db9190614ff8565b60405180910390f35b3480156101ef575f80fd5b506101f86106a5565b604051610205919061509b565b60405180910390f35b348015610219575f80fd5b50610234600480360381019061022f91906150f6565b610720565b6040516102419190615208565b60405180910390f35b348015610255575f80fd5b50610270600480360381019061026b91906150f6565b6107f1565b005b34801561027d575f80fd5b50610298600480360381019061029391906150f6565b610a10565b6040516102a5919061529b565b60405180910390f35b3480156102b9575f80fd5b506102c2610b16565b005b3480156102cf575f80fd5b506102d8610d39565b6040516102e59190614ff8565b60405180910390f35b3480156102f9575f80fd5b50610314600480360381019061030f91906152d7565b610d50565b005b348015610321575f80fd5b5061033c600480360381019061033791906150f6565b611034565b604051610349919061532f565b60405180910390f35b34801561035d575f80fd5b50610378600480360381019061037391906150f6565b611068565b604051610385919061529b565b60405180910390f35b348015610399575f80fd5b506103b460048036038101906103af91906153fe565b611156565b005b3480156103c1575f80fd5b506103dc60048036038101906103d791906154e4565b611667565b005b6103f860048036038101906103f391906156ed565b6118ab565b005b348015610405575f80fd5b5061040e6118ca565b60405161041b919061575f565b60405180910390f35b34801561042f575f80fd5b5061044a60048036038101906104459190615778565b6118fb565b005b348015610457575f80fd5b50610472600480360381019061046d91906157d5565b611c2f565b005b34801561047f575f80fd5b5061049a60048036038101906104959190615832565b611d71565b005b3480156104a7575f80fd5b506104c260048036038101906104bd9190615918565b612167565b005b3480156104cf575f80fd5b506104d861246b565b6040516104eb9796959493929190615abb565b60405180910390f35b3480156104ff575f80fd5b5061051a600480360381019061051591906150f6565b612574565b604051610528929190615dcd565b60405180910390f35b34801561053c575f80fd5b506105456128da565b604051610552919061509b565b60405180910390f35b348015610566575f80fd5b5061056f612913565b60405161057c9190614ff8565b60405180910390f35b348015610590575f80fd5b506105ab60048036038101906105a691906150f6565b61292a565b005b3480156105b8575f80fd5b506105c1612b94565b005b3480156105ce575f80fd5b506105e960048036038101906105e491906150f6565b612cb9565b6040516105f7929190615e4a565b60405180910390f35b34801561060b575f80fd5b5061062660048036038101906106219190615e7f565b612f8a565b005b348015610633575f80fd5b5061063c613114565b6040516106499190614ff8565b60405180910390f35b34801561065d575f80fd5b50610678600480360381019061067391906150f6565b61312b565b6040516106859190614ff8565b60405180910390f35b5f8061069861314e565b9050806005015491505090565b60606040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506106e65f613175565b6106f06002613175565b6106f95f613175565b60405160200161070c9493929190615f78565b604051602081830303815290604052905090565b60605f61072b61314e565b90505f816003015f8581526020019081526020015f20549050816002015f8581526020019081526020015f205f8281526020019081526020015f208054806020026020016040519081016040528092919081815260200182805480156107e357602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001906001019080831161079a575b505050505092505050919050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561084e573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108729190615fea565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146108e157336040517f21bfda100000000000000000000000000000000000000000000000000000000081526004016108d89190616015565b60405180910390fd5b5f6108ea61314e565b90508060090154821180610906575060f8600560ff16901b8211155b1561094857816040517fcbe9265600000000000000000000000000000000000000000000000000000000815260040161093f9190614ff8565b60405180910390fd5b806001015f8381526020019081526020015f205f9054906101000a900460ff16156109aa57816040517fdf0db5fb0000000000000000000000000000000000000000000000000000000081526004016109a19190614ff8565b60405180910390fd5b6001816001015f8481526020019081526020015f205f6101000a81548160ff0219169083151502179055507f384f90fefbcfaa68f22e00094aeaa52b2bc693936d2ce1afed1212520b59b58e82604051610a049190614ff8565b60405180910390a15050565b5f80610a1a61314e565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff16610a7d57826040517f84de1331000000000000000000000000000000000000000000000000000000008152600401610a749190614ff8565b60405180910390fd5b5f801b816003015f8581526020019081526020015f205403610ad657826040517f83f18335000000000000000000000000000000000000000000000000000000008152600401610acd9190614ff8565b60405180910390fd5b5f816006015f8581526020019081526020015f2054905081600d015f8281526020019081526020015f205f9054906101000a900460ff1692505050919050565b6001610b2061323f565b67ffffffffffffffff1614610b61576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60035f610b6c613263565b9050805f0160089054906101000a900460ff1680610bb457508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15610beb576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff021916908315150217905550610ca46040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506040518060400160405280600181526020017f310000000000000000000000000000000000000000000000000000000000000081525061328a565b5f610cad61314e565b905060f8600360ff16901b816004018190555060f8600460ff16901b816005018190555060f8600560ff16901b8160090181905550505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051610d2d9190616050565b60405180910390a15050565b5f80610d4361314e565b9050806009015491505090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610dad573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610dd19190615fea565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610e4057336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401610e379190616015565b60405180910390fd5b5f610e4961314e565b90505f8160090154905060f8600560ff16901b8114158015610e885750816001015f8281526020019081526020015f205f9054906101000a900460ff16155b15610eca57806040517f061ac61d000000000000000000000000000000000000000000000000000000008152600401610ec19190614ff8565b60405180910390fd5b816009015f815480929190610ede90616096565b91905055505f826009015490508483600a015f8381526020019081526020015f20819055508383600d015f8381526020019081526020015f205f6101000a81548160ff02191690836001811115610f3857610f37615228565b5b02179055505f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa158015610f9b573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610fbf91906160f1565b90505f610fcb826132a0565b90508085600e015f8581526020019081526020015f209081610fed9190616316565b507f8cf0151393f84fd694c5e315cb74cc05b247de0a454fd9e9129c661efdf9401d8388888460405161102394939291906163e5565b60405180910390a150505050505050565b5f8061103e61314e565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff16915050919050565b5f8061107261314e565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff166110d557826040517fda32d00f0000000000000000000000000000000000000000000000000000000081526004016110cc9190614ff8565b60405180910390fd5b5f801b816003015f8581526020019081526020015f20540361112e57826040517fd5fd3cd70000000000000000000000000000000000000000000000000000000081526004016111259190614ff8565b60405180910390fd5b80600d015f8481526020019081526020015f205f9054906101000a900460ff16915050919050565b5f61115f61314e565b9050806005015486118061117b575060f8600460ff16901b8611155b156111bd57856040517fadfab9040000000000000000000000000000000000000000000000000000000081526004016111b49190614ff8565b60405180910390fd5b5f858590500361120457856040517fe6f9083b0000000000000000000000000000000000000000000000000000000081526004016111fb9190614ff8565b60405180910390fd5b5f8061120f886132cc565b915091505f836006015f8a81526020019081526020015f20549050836001015f8281526020019081526020015f205f9054906101000a900460ff16611280576040517f6fbcdd2b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f61128e828b8b8b88613456565b90505f61129d84838a8a613637565b9050855f015f8c81526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff161561133d578a816040517f98fb957d00000000000000000000000000000000000000000000000000000000815260040161133492919061642f565b60405180910390fd5b6001865f015f8d81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f866002015f8d81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507f2afe64fb3afde8e2678aea84cf36223f330e2fb1286d37aed573ab9cd1db47c78c8c8c8c8c3360405161145f96959493929190616662565b60405180910390a1866001015f8d81526020019081526020015f205f9054906101000a900460ff1615801561149e575061149d85828054905061369f565b5b15611659576001876001015f8e81526020019081526020015f205f6101000a81548160ff0219169083151502179055505f5b8b8b905081101561155457876007015f8e81526020019081526020015f208c8c83818110611501576115006166b7565b5b905060200281019061151391906166f0565b908060018154018082558091505060019003905f5260205f2090600202015f9091909190915081816115459190616923565b505080806001019150506114d0565b5082876003015f8e81526020019081526020015f20819055505f6115fa86838054806020026020016040519081016040528092919081815260200182805480156115f057602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116115a7575b505050505061373c565b90505f87511180156116325750600360ff16875f8151811061161f5761161e6166b7565b5b602001015160f81c60f81b60f81c60ff16145b1561164957611644888d8d848b613884565b611657565b611656888e8e8e85613996565b5b505b505050505050505050505050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156116c4573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116e89190615fea565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461175757336040517f21bfda1000000000000000000000000000000000000000000000000000000000815260040161174e9190616015565b60405180910390fd5b5f61176061314e565b9050806005015488118061177c575060f8600460ff16901b8811155b156117be57876040517fadfab9040000000000000000000000000000000000000000000000000000000081526004016117b59190614ff8565b60405180910390fd5b8484905087879050146117fd576040517f894b2ab300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b611806826139e3565b6001816010015f8a81526020019081526020015f20541461185e57876040517f05b083f20000000000000000000000000000000000000000000000000000000081526004016118559190614ff8565b60405180910390fd5b7f8bfa7d0ed6f87e526b62342918ee7bfa53952badd463dc934054d7dd940eafdc888888888888886040516118999796959493929190616999565b60405180910390a15050505050505050565b6118b3613a2b565b6118bc82613b11565b6118c68282613c04565b5050565b5f6118d3613d22565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b5f61190461314e565b90508060040154841180611920575060f8600360ff16901b8411155b1561196257836040517f0ab7f6870000000000000000000000000000000000000000000000000000000081526004016119599190614ff8565b60405180910390fd5b5f8061196d866132cc565b915091505f61197c8784613da9565b90505f61198b83838989613637565b9050845f015f8981526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615611a2b5787816040517f33ca1fe3000000000000000000000000000000000000000000000000000000008152600401611a2292919061642f565b60405180910390fd5b6001855f015f8a81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f856002015f8a81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507f4c715c5734ce5c18c9c12e8496e53d2a65f1ec381d476957f0f596b364a59b0c89898933604051611b4994939291906169fc565b60405180910390a1856001015f8a81526020019081526020015f205f9054906101000a900460ff16158015611b885750611b8784828054905061369f565b5b15611c24576001866001015f8b81526020019081526020015f205f6101000a81548160ff02191690831515021790555082866003015f8b81526020019081526020015f20819055505f866006015f8b81526020019081526020015f205490507f3a116120cca5d4f073cc1fc31ff26133ab7b0499f2712fa010023b87d5a1f9ee8a8288604051611c1a93929190616a3a565b60405180910390a1505b505050505050505050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611c8c573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611cb09190615fea565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614611d1f57336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401611d169190616015565b60405180910390fd5b611d6c8383838080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050613e0b565b505050565b5f611d7a61314e565b90508060090154861180611d96575060f8600560ff16901b8611155b15611dd857856040517f8d8c940a000000000000000000000000000000000000000000000000000000008152600401611dcf9190614ff8565b60405180910390fd5b5f80611de3886132cc565b915091505f611e088985600a015f8c81526020019081526020015f20548a8a87613fc6565b90505f611e1783838989613637565b9050845f015f8b81526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615611eb75789816040517ffcf5a6e9000000000000000000000000000000000000000000000000000000008152600401611eae92919061642f565b60405180910390fd5b6001855f015f8c81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f856002015f8c81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507f7bf1b42c10e9497c879620c5b7afced10bda17d8c90b22f0e3bc6b2fd6ced0bd8b8b8b8b8b33604051611fd996959493929190616a76565b60405180910390a1856001015f8c81526020019081526020015f205f9054906101000a900460ff16158015612018575061201784828054905061369f565b5b1561215a576001866001015f8d81526020019081526020015f205f6101000a81548160ff021916908315150217905550898987600b015f8e81526020019081526020015f20918261206a929190616802565b5082866003015f8d81526020019081526020015f20819055508a86600c01819055505f612119858380548060200260200160405190810160405280929190818152602001828054801561210f57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116120c6575b505050505061373c565b90507f2258b73faed33fb2e2ea454403bef974920caf682ab3a723484fcf67553b16a28c828d8d6040516121509493929190616acb565b60405180910390a1505b5050505050505050505050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156121c4573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906121e89190615fea565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461225757336040517f21bfda1000000000000000000000000000000000000000000000000000000000815260040161224e9190616015565b60405180910390fd5b5f61226061314e565b9050806005015487118061227c575060f8600460ff16901b8711155b156122be57866040517fadfab9040000000000000000000000000000000000000000000000000000000081526004016122b59190614ff8565b60405180910390fd5b806001015f8881526020019081526020015f205f9054906101000a900460ff16612314576040517f6fbcdd2b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f868690500361235b57866040517fe6f9083b0000000000000000000000000000000000000000000000000000000081526004016123529190614ff8565b60405180910390fd5b612364826139e3565b80600f015f8881526020019081526020015f205f6123829190614f2f565b5f5b868690508110156124085781600f015f8981526020019081526020015f208787838181106123b5576123b46166b7565b5b90506020028101906123c791906166f0565b908060018154018082558091505060019003905f5260205f2090600202015f9091909190915081816123f99190616923565b50508080600101915050612384565b5081816010015f8981526020019081526020015f20819055507fa47664861ab58c5bd5040e9cc45e68d0e48ec04371035fd75099e217e0a6aa8187858589898760405161245a96959493929190616c3c565b60405180910390a150505050505050565b5f6060805f805f60605f61247d614057565b90505f801b815f015414801561249857505f801b8160010154145b6124d7576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016124ce90616cdb565b60405180910390fd5b6124df61407e565b6124e761411c565b46305f801b5f67ffffffffffffffff811115612506576125056155c9565b5b6040519080825280602002602001820160405280156125345781602001602082028036833780820191505090505b507f0f0000000000000000000000000000000000000000000000000000000000000095949392919097509750975097509750975097505090919293949596565b6060805f61258061314e565b9050806001015f8581526020019081526020015f205f9054906101000a900460ff166125e357836040517f84de13310000000000000000000000000000000000000000000000000000000081526004016125da9190614ff8565b60405180910390fd5b5f816003015f8681526020019081526020015f205490505f801b810361264057846040517f83f183350000000000000000000000000000000000000000000000000000000081526004016126379190614ff8565b60405180910390fd5b5f826002015f8781526020019081526020015f205f8381526020019081526020015f208054806020026020016040519081016040528092919081815260200182805480156126e057602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311612697575b505050505090505f61278a84600e015f8981526020019081526020015f20805461270990616149565b80601f016020809104026020016040519081016040528092919081815260200182805461273590616149565b80156127805780601f1061275757610100808354040283529160200191612780565b820191905f5260205f20905b81548152906001019060200180831161276357829003601f168201915b50505050506141ba565b90505f612797828461373c565b905080856007015f8a81526020019081526020015f2080805480602002602001604051908101604052809291908181526020015f905b828210156128c6578382905f5260205f2090600202016040518060400160405290815f82015f9054906101000a900460ff16600181111561281157612810615228565b5b600181111561282357612822615228565b5b815260200160018201805461283790616149565b80601f016020809104026020016040519081016040528092919081815260200182805461286390616149565b80156128ae5780601f10612885576101008083540402835291602001916128ae565b820191905f5260205f20905b81548152906001019060200180831161289157829003601f168201915b505050505081525050815260200190600101906127cd565b505050509050965096505050505050915091565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b5f8061291d61314e565b905080600c015491505090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612987573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129ab9190615fea565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614612a1a57336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401612a119190616015565b60405180910390fd5b5f612a2361314e565b90508060040154821180612a3f575060f8600360ff16901b8211155b15612a8157816040517ffcf2db7a000000000000000000000000000000000000000000000000000000008152600401612a789190614ff8565b60405180910390fd5b5f816006015f8481526020019081526020015f20549050816001015f8281526020019081526020015f205f9054906101000a900460ff1615612afa57826040517f92789b67000000000000000000000000000000000000000000000000000000008152600401612af19190614ff8565b60405180910390fd5b6001826001015f8581526020019081526020015f205f6101000a81548160ff0219169083151502179055505f8114612b58576001826001015f8381526020019081526020015f205f6101000a81548160ff0219169083151502179055505b7f2b087b884b35a81d769d1a1e092880f1da56de964e4b339eabcb1f45f5fe326483604051612b879190614ff8565b60405180910390a1505050565b60035f612b9f613263565b9050805f0160089054906101000a900460ff1680612be757508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15612c1e576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051612cad9190616050565b60405180910390a15050565b6060805f612cc561314e565b9050806001015f8581526020019081526020015f205f9054906101000a900460ff16612d2857836040517fda32d00f000000000000000000000000000000000000000000000000000000008152600401612d1f9190614ff8565b60405180910390fd5b5f816003015f8681526020019081526020015f205490505f801b8103612d8557846040517fd5fd3cd7000000000000000000000000000000000000000000000000000000008152600401612d7c9190614ff8565b60405180910390fd5b5f826002015f8781526020019081526020015f205f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020018280548015612e2557602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311612ddc575b505050505090505f612ecf84600e015f8981526020019081526020015f208054612e4e90616149565b80601f0160208091040260200160405190810160405280929190818152602001828054612e7a90616149565b8015612ec55780601f10612e9c57610100808354040283529160200191612ec5565b820191905f5260205f20905b815481529060010190602001808311612ea857829003601f168201915b50505050506141ba565b90505f612edc828461373c565b90508085600b015f8a81526020019081526020015f20808054612efe90616149565b80601f0160208091040260200160405190810160405280929190818152602001828054612f2a90616149565b8015612f755780601f10612f4c57610100808354040283529160200191612f75565b820191905f5260205f20905b815481529060010190602001808311612f5857829003601f168201915b50505050509050965096505050505050915091565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612fe7573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061300b9190615fea565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461307a57336040517f21bfda100000000000000000000000000000000000000000000000000000000081526004016130719190616015565b60405180910390fd5b5f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa1580156130d8573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906130fc91906160f1565b90506131108261310b836132a0565b613e0b565b5050565b5f8061311e61314e565b9050806008015491505090565b5f61313461314e565b6010015f8381526020019081526020015f20549050919050565b5f7f26fdaf8a2cb20d20b55e36218986905e534ee7a970dd2fa827946e4b7496db00905090565b60605f600161318384614362565b0190505f8167ffffffffffffffff8111156131a1576131a06155c9565b5b6040519080825280601f01601f1916602001820160405280156131d35781602001600182028036833780820191505090505b5090505f82602001820190505b600115613234578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a858161322957613228616cf9565b5b0494505f85036131e0575b819350505050919050565b5f613248613263565b5f015f9054906101000a900467ffffffffffffffff16905090565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b6132926144b3565b61329c82826144f3565b5050565b60606001826040516020016132b6929190616d86565b6040516020818303038152906040529050919050565b60605f806132d861314e565b905080600e015f8581526020019081526020015f2080546132f890616149565b80601f016020809104026020016040519081016040528092919081815260200182805461332490616149565b801561336f5780601f106133465761010080835404028352916020019161336f565b820191905f5260205f20905b81548152906001019060200180831161335257829003601f168201915b5050505050925061337f836141ba565b91507344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166346c5bbbd83336040518363ffffffff1660e01b81526004016133d092919061642f565b602060405180830381865afa1580156133eb573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061340f9190616ddb565b61345057336040517faee863230000000000000000000000000000000000000000000000000000000081526004016134479190616015565b60405180910390fd5b50915091565b5f808484905067ffffffffffffffff811115613475576134746155c9565b5b6040519080825280602002602001820160405280156134a35781602001602082028036833780820191505090505b5090505f5b858590508110156135a75760405180606001604052806025815260200161763060259139805190602001208686838181106134e6576134e56166b7565b5b90506020028101906134f891906166f0565b5f0160208101906135099190616e06565b87878481811061351c5761351b6166b7565b5b905060200281019061352e91906166f0565b806020019061353d9190616796565b60405161354b929190616e5f565b604051809103902060405160200161356593929190616e86565b6040516020818303038152906040528051906020012082828151811061358e5761358d6166b7565b5b60200260200101818152505080806001019150506134a8565b5061362b6040518060c00160405280608281526020016175ae60829139805190602001208888846040516020016135de9190616f6c565b604051602081830303815290604052805190602001208780519060200120604051602001613610959493929190616f82565b60405160208183030381529060405280519060200120614544565b91505095945050505050565b5f806136868585858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505061455d565b9050613693868233614587565b80915050949350505050565b5f807344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166341ad069c856040518263ffffffff1660e01b81526004016136ee9190614ff8565b602060405180830381865afa158015613709573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061372d91906160f1565b90508083101591505092915050565b60605f825190505f8167ffffffffffffffff81111561375e5761375d6155c9565b5b60405190808252806020026020018201604052801561379157816020015b606081526020019060019003908161377c5790505b5090505f5b82811015613878577344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166331ff41c8878784815181106137e2576137e16166b7565b5b60200260200101516040518363ffffffff1660e01b815260040161380792919061642f565b5f60405180830381865afa158015613821573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f820116820180604052508101906138499190617126565b606001518282815181106138605761385f6166b7565b5b60200260200101819052508080600101915050613796565b50809250505092915050565b5f61388e82614768565b905085600f015f8281526020019081526020015f205f6138ae9190614f2f565b5f5b858590508110156139345786600f015f8381526020019081526020015f208686838181106138e1576138e06166b7565b5b90506020028101906138f391906166f0565b908060018154018082558091505060019003905f5260205f2090600202015f9091909190915081816139259190616923565b505080806001019150506138b0565b506001866010015f8381526020019081526020015f20819055507fa47664861ab58c5bd5040e9cc45e68d0e48ec04371035fd75099e217e0a6aa8181848787600160405161398695949392919061716d565b60405180910390a1505050505050565b8385600801819055507feb85c26dbcad46b80a68a0f24cce7c2c90f0a1faded84184138839fc9e80a25b848285856040516139d494939291906171c0565b60405180910390a15050505050565b60018114613a2857806040517fa3e9ed7e000000000000000000000000000000000000000000000000000000008152600401613a1f9190614ff8565b60405180910390fd5b50565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff161480613ad857507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16613abf6147b1565b73ffffffffffffffffffffffffffffffffffffffff1614155b15613b0f576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613b6e573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613b929190615fea565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614613c0157336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401613bf89190616015565b60405180910390fd5b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa925050508015613c6c57506040513d601f19601f82011682018060405250810190613c69919061722f565b60015b613cad57816040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401613ca49190616015565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b8114613d1357806040517faa1d49a4000000000000000000000000000000000000000000000000000000008152600401613d0a919061575f565b60405180910390fd5b613d1d8383614804565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff1614613da7576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f613e036040518060600160405280603c815260200161751c603c913980519060200120848480519060200120604051602001613de89392919061725a565b60405160208183030381529060405280519060200120614544565b905092915050565b5f613e1461314e565b90505f8160050154905060f8600460ff16901b8114158015613e535750816001015f8281526020019081526020015f205f9054906101000a900460ff16155b15613e9557806040517f3b853da8000000000000000000000000000000000000000000000000000000008152600401613e8c9190614ff8565b60405180910390fd5b816004015f815480929190613ea990616096565b91905055505f82600401549050826005015f815480929190613eca90616096565b91905055505f8360050154905080846006015f8481526020019081526020015f208190555081846006015f8381526020019081526020015f20819055508584600d015f8481526020019081526020015f205f6101000a81548160ff02191690836001811115613f3c57613f3b615228565b5b02179055508484600e015f8481526020019081526020015f209081613f619190616316565b508484600e015f8381526020019081526020015f209081613f829190616316565b507ffbf5274810b94f86970c1147e8ffaebed246ee9777d695a69004dc6256d1fe91828787604051613fb69392919061728f565b60405180910390a1505050505050565b5f61404c604051806080016040528060568152602001617558605691398051906020012087878787604051602001613fff929190616e5f565b604051602081830303815290604052805190602001208680519060200120604051602001614031959493929190616f82565b60405160208183030381529060405280519060200120614544565b905095945050505050565b5f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100905090565b60605f614089614057565b905080600201805461409a90616149565b80601f01602080910402602001604051908101604052809291908181526020018280546140c690616149565b80156141115780601f106140e857610100808354040283529160200191614111565b820191905f5260205f20905b8154815290600101906020018083116140f457829003601f168201915b505050505091505090565b60605f614127614057565b905080600301805461413890616149565b80601f016020809104026020016040519081016040528092919081815260200182805461416490616149565b80156141af5780601f10614186576101008083540402835291602001916141af565b820191905f5260205f20905b81548152906001019060200180831161419257829003601f168201915b505050505091505090565b5f80825114806141ec57505f825f815181106141d9576141d86166b7565b5b602001015160f81c60f81b60f81c60ff16145b15614279577344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa15801561424e573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061427291906160f1565b905061435d565b5f825f8151811061428d5761428c6166b7565b5b602001015160f81c60f81b60f81c9050600160ff168160ff16141580156142bb5750600360ff168160ff1614155b156142fd57806040517f2139cc2c0000000000000000000000000000000000000000000000000000000081526004016142f491906172da565b60405180910390fd5b5f600360ff168260ff1614614313576021614316565b60425b90508084511015614353576040517f8b248b6000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6021840151925050505b919050565b5f805f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f01000000000000000083106143be577a184f03e93ff9f4daa797ed6e38ed64bf6a1f01000000000000000083816143b4576143b3616cf9565b5b0492506040810190505b6d04ee2d6d415b85acef810000000083106143fb576d04ee2d6d415b85acef810000000083816143f1576143f0616cf9565b5b0492506020810190505b662386f26fc10000831061442a57662386f26fc1000083816144205761441f616cf9565b5b0492506010810190505b6305f5e1008310614453576305f5e100838161444957614448616cf9565b5b0492506008810190505b612710831061447857612710838161446e5761446d616cf9565b5b0492506004810190505b6064831061449b576064838161449157614490616cf9565b5b0492506002810190505b600a83106144aa576001810190505b80915050919050565b6144bb614876565b6144f1576040517fd7e6bcf800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b6144fb6144b3565b5f614504614057565b905082816002019081614517919061734b565b5081816003019081614529919061734b565b505f801b815f01819055505f801b8160010181905550505050565b5f614556614550614894565b836148a2565b9050919050565b5f805f8061456b86866148e2565b92509250925061457b8282614937565b82935050505092915050565b7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff16639447cfd484846040518363ffffffff1660e01b81526004016145d692919061642f565b602060405180830381865afa1580156145f1573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906146159190616ddb565b6146585781816040517f0d86f52100000000000000000000000000000000000000000000000000000000815260040161464f92919061741a565b60405180910390fd5b5f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166331ff41c885846040518363ffffffff1660e01b81526004016146a892919061642f565b5f60405180830381865afa1580156146c2573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f820116820180604052508101906146ea9190617126565b90508273ffffffffffffffffffffffffffffffffffffffff16816020015173ffffffffffffffffffffffffffffffffffffffff16146147625782826040517f0d86f52100000000000000000000000000000000000000000000000000000000815260040161475992919061741a565b60405180910390fd5b50505050565b5f6042825110156147a5576040517f8b248b6000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60418201519050919050565b5f6147dd7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b614a99565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b61480d82614aa2565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f81511115614869576148638282614b6b565b50614872565b614871614beb565b5b5050565b5f61487f613263565b5f0160089054906101000a900460ff16905090565b5f61489d614c27565b905090565b5f6040517f190100000000000000000000000000000000000000000000000000000000000081528360028201528260228201526042812091505092915050565b5f805f6041845103614922575f805f602087015192506040870151915060608701515f1a905061491488828585614c8a565b955095509550505050614930565b5f600285515f1b9250925092505b9250925092565b5f600381111561494a57614949615228565b5b82600381111561495d5761495c615228565b5b0315614a95576001600381111561497757614976615228565b5b82600381111561498a57614989615228565b5b036149c1576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600260038111156149d5576149d4615228565b5b8260038111156149e8576149e7615228565b5b03614a2c57805f1c6040517ffce698f7000000000000000000000000000000000000000000000000000000008152600401614a239190614ff8565b60405180910390fd5b600380811115614a3f57614a3e615228565b5b826003811115614a5257614a51615228565b5b03614a9457806040517fd78bce0c000000000000000000000000000000000000000000000000000000008152600401614a8b919061575f565b60405180910390fd5b5b5050565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b03614afd57806040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401614af49190616015565b60405180910390fd5b80614b297f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b614a99565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f808473ffffffffffffffffffffffffffffffffffffffff1684604051614b949190617471565b5f60405180830381855af49150503d805f8114614bcc576040519150601f19603f3d011682016040523d82523d5f602084013e614bd1565b606091505b5091509150614be1858383614d71565b9250505092915050565b5f341115614c25576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f614c51614dfe565b614c59614e74565b4630604051602001614c6f959493929190617487565b60405160208183030381529060405280519060200120905090565b5f805f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c1115614cc6575f600385925092509250614d67565b5f6001888888886040515f8152602001604052604051614ce994939291906174d8565b6020604051602081039080840390855afa158015614d09573d5f803e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603614d5a575f60015f801b93509350935050614d67565b805f805f1b935093509350505b9450945094915050565b606082614d8657614d8182614eeb565b614df6565b5f8251148015614dac57505f8473ffffffffffffffffffffffffffffffffffffffff163b145b15614dee57836040517f9996b315000000000000000000000000000000000000000000000000000000008152600401614de59190616015565b60405180910390fd5b819050614df7565b5b9392505050565b5f80614e08614057565b90505f614e1361407e565b90505f81511115614e2f57808051906020012092505050614e71565b5f825f015490505f801b8114614e4a57809350505050614e71565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f80614e7e614057565b90505f614e8961411c565b90505f81511115614ea557808051906020012092505050614ee8565b5f826001015490505f801b8114614ec157809350505050614ee8565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f81511115614efd5780518082602001fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5080545f8255600202905f5260205f2090810190614f4d9190614f50565b50565b5b80821115614f84575f8082015f6101000a81549060ff0219169055600182015f614f7b9190614f88565b50600201614f51565b5090565b508054614f9490616149565b5f825580601f10614fa55750614fc2565b601f0160209004905f5260205f2090810190614fc19190614fc5565b5b50565b5b80821115614fdc575f815f905550600101614fc6565b5090565b5f819050919050565b614ff281614fe0565b82525050565b5f60208201905061500b5f830184614fe9565b92915050565b5f81519050919050565b5f82825260208201905092915050565b5f5b8381101561504857808201518184015260208101905061502d565b5f8484015250505050565b5f601f19601f8301169050919050565b5f61506d82615011565b615077818561501b565b935061508781856020860161502b565b61509081615053565b840191505092915050565b5f6020820190508181035f8301526150b38184615063565b905092915050565b5f604051905090565b5f80fd5b5f80fd5b6150d581614fe0565b81146150df575f80fd5b50565b5f813590506150f0816150cc565b92915050565b5f6020828403121561510b5761510a6150c4565b5b5f615118848285016150e2565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6151738261514a565b9050919050565b61518381615169565b82525050565b5f615194838361517a565b60208301905092915050565b5f602082019050919050565b5f6151b682615121565b6151c0818561512b565b93506151cb8361513b565b805f5b838110156151fb5781516151e28882615189565b97506151ed836151a0565b9250506001810190506151ce565b5085935050505092915050565b5f6020820190508181035f83015261522081846151ac565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b6002811061526657615265615228565b5b50565b5f81905061527682615255565b919050565b5f61528582615269565b9050919050565b6152958161527b565b82525050565b5f6020820190506152ae5f83018461528c565b92915050565b600281106152c0575f80fd5b50565b5f813590506152d1816152b4565b92915050565b5f80604083850312156152ed576152ec6150c4565b5b5f6152fa858286016150e2565b925050602061530b858286016152c3565b9150509250929050565b5f8115159050919050565b61532981615315565b82525050565b5f6020820190506153425f830184615320565b92915050565b5f80fd5b5f80fd5b5f80fd5b5f8083601f84011261536957615368615348565b5b8235905067ffffffffffffffff8111156153865761538561534c565b5b6020830191508360208202830111156153a2576153a1615350565b5b9250929050565b5f8083601f8401126153be576153bd615348565b5b8235905067ffffffffffffffff8111156153db576153da61534c565b5b6020830191508360018202830111156153f7576153f6615350565b5b9250929050565b5f805f805f60608688031215615417576154166150c4565b5b5f615424888289016150e2565b955050602086013567ffffffffffffffff811115615445576154446150c8565b5b61545188828901615354565b9450945050604086013567ffffffffffffffff811115615474576154736150c8565b5b615480888289016153a9565b92509250509295509295909350565b5f8083601f8401126154a4576154a3615348565b5b8235905067ffffffffffffffff8111156154c1576154c061534c565b5b6020830191508360208202830111156154dd576154dc615350565b5b9250929050565b5f805f805f805f60a0888a0312156154ff576154fe6150c4565b5b5f61550c8a828b016150e2565b975050602088013567ffffffffffffffff81111561552d5761552c6150c8565b5b6155398a828b0161548f565b9650965050604088013567ffffffffffffffff81111561555c5761555b6150c8565b5b6155688a828b0161548f565b9450945050606061557b8a828b016150e2565b925050608061558c8a828b016150e2565b91505092959891949750929550565b6155a481615169565b81146155ae575f80fd5b50565b5f813590506155bf8161559b565b92915050565b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6155ff82615053565b810181811067ffffffffffffffff8211171561561e5761561d6155c9565b5b80604052505050565b5f6156306150bb565b905061563c82826155f6565b919050565b5f67ffffffffffffffff82111561565b5761565a6155c9565b5b61566482615053565b9050602081019050919050565b828183375f83830152505050565b5f61569161568c84615641565b615627565b9050828152602081018484840111156156ad576156ac6155c5565b5b6156b8848285615671565b509392505050565b5f82601f8301126156d4576156d3615348565b5b81356156e484826020860161567f565b91505092915050565b5f8060408385031215615703576157026150c4565b5b5f615710858286016155b1565b925050602083013567ffffffffffffffff811115615731576157306150c8565b5b61573d858286016156c0565b9150509250929050565b5f819050919050565b61575981615747565b82525050565b5f6020820190506157725f830184615750565b92915050565b5f805f6040848603121561578f5761578e6150c4565b5b5f61579c868287016150e2565b935050602084013567ffffffffffffffff8111156157bd576157bc6150c8565b5b6157c9868287016153a9565b92509250509250925092565b5f805f604084860312156157ec576157eb6150c4565b5b5f6157f9868287016152c3565b935050602084013567ffffffffffffffff81111561581a576158196150c8565b5b615826868287016153a9565b92509250509250925092565b5f805f805f6060868803121561584b5761584a6150c4565b5b5f615858888289016150e2565b955050602086013567ffffffffffffffff811115615879576158786150c8565b5b615885888289016153a9565b9450945050604086013567ffffffffffffffff8111156158a8576158a76150c8565b5b6158b4888289016153a9565b92509250509295509295909350565b5f8083601f8401126158d8576158d7615348565b5b8235905067ffffffffffffffff8111156158f5576158f461534c565b5b60208301915083602082028301111561591157615910615350565b5b9250929050565b5f805f805f8060808789031215615932576159316150c4565b5b5f61593f89828a016150e2565b965050602087013567ffffffffffffffff8111156159605761595f6150c8565b5b61596c89828a01615354565b9550955050604087013567ffffffffffffffff81111561598f5761598e6150c8565b5b61599b89828a016158c3565b935093505060606159ae89828a016150e2565b9150509295509295509295565b5f7fff0000000000000000000000000000000000000000000000000000000000000082169050919050565b6159ef816159bb565b82525050565b6159fe81615169565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b615a3681614fe0565b82525050565b5f615a478383615a2d565b60208301905092915050565b5f602082019050919050565b5f615a6982615a04565b615a738185615a0e565b9350615a7e83615a1e565b805f5b83811015615aae578151615a958882615a3c565b9750615aa083615a53565b925050600181019050615a81565b5085935050505092915050565b5f60e082019050615ace5f83018a6159e6565b8181036020830152615ae08189615063565b90508181036040830152615af48188615063565b9050615b036060830187614fe9565b615b1060808301866159f5565b615b1d60a0830185615750565b81810360c0830152615b2f8184615a5f565b905098975050505050505050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f82825260208201905092915050565b5f615b8082615011565b615b8a8185615b66565b9350615b9a81856020860161502b565b615ba381615053565b840191505092915050565b5f615bb98383615b76565b905092915050565b5f602082019050919050565b5f615bd782615b3d565b615be18185615b47565b935083602082028501615bf385615b57565b805f5b85811015615c2e5784840389528151615c0f8582615bae565b9450615c1a83615bc1565b925060208a01995050600181019050615bf6565b50829750879550505050505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b60028110615c7a57615c79615228565b5b50565b5f819050615c8a82615c69565b919050565b5f615c9982615c7d565b9050919050565b615ca981615c8f565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f615cd382615caf565b615cdd8185615cb9565b9350615ced81856020860161502b565b615cf681615053565b840191505092915050565b5f604083015f830151615d165f860182615ca0565b5060208301518482036020860152615d2e8282615cc9565b9150508091505092915050565b5f615d468383615d01565b905092915050565b5f602082019050919050565b5f615d6482615c40565b615d6e8185615c4a565b935083602082028501615d8085615c5a565b805f5b85811015615dbb5784840389528151615d9c8582615d3b565b9450615da783615d4e565b925060208a01995050600181019050615d83565b50829750879550505050505092915050565b5f6040820190508181035f830152615de58185615bcd565b90508181036020830152615df98184615d5a565b90509392505050565b5f82825260208201905092915050565b5f615e1c82615caf565b615e268185615e02565b9350615e3681856020860161502b565b615e3f81615053565b840191505092915050565b5f6040820190508181035f830152615e628185615bcd565b90508181036020830152615e768184615e12565b90509392505050565b5f60208284031215615e9457615e936150c4565b5b5f615ea1848285016152c3565b91505092915050565b5f81905092915050565b5f615ebe82615011565b615ec88185615eaa565b9350615ed881856020860161502b565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f615f18600283615eaa565b9150615f2382615ee4565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f615f62600183615eaa565b9150615f6d82615f2e565b600182019050919050565b5f615f838287615eb4565b9150615f8e82615f0c565b9150615f9a8286615eb4565b9150615fa582615f56565b9150615fb18285615eb4565b9150615fbc82615f56565b9150615fc88284615eb4565b915081905095945050505050565b5f81519050615fe48161559b565b92915050565b5f60208284031215615fff57615ffe6150c4565b5b5f61600c84828501615fd6565b91505092915050565b5f6020820190506160285f8301846159f5565b92915050565b5f67ffffffffffffffff82169050919050565b61604a8161602e565b82525050565b5f6020820190506160635f830184616041565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f6160a082614fe0565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82036160d2576160d1616069565b5b600182019050919050565b5f815190506160eb816150cc565b92915050565b5f60208284031215616106576161056150c4565b5b5f616113848285016160dd565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f600282049050600182168061616057607f821691505b6020821081036161735761617261611c565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f600883026161d57fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8261619a565b6161df868361619a565b95508019841693508086168417925050509392505050565b5f819050919050565b5f61621a61621561621084614fe0565b6161f7565b614fe0565b9050919050565b5f819050919050565b61623383616200565b61624761623f82616221565b8484546161a6565b825550505050565b5f90565b61625b61624f565b61626681848461622a565b505050565b5b818110156162895761627e5f82616253565b60018101905061626c565b5050565b601f8211156162ce5761629f81616179565b6162a88461618b565b810160208510156162b7578190505b6162cb6162c38561618b565b83018261626b565b50505b505050565b5f82821c905092915050565b5f6162ee5f19846008026162d3565b1980831691505092915050565b5f61630683836162df565b9150826002028217905092915050565b61631f82615caf565b67ffffffffffffffff811115616338576163376155c9565b5b6163428254616149565b61634d82828561628d565b5f60209050601f83116001811461637e575f841561636c578287015190505b61637685826162fb565b8655506163dd565b601f19841661638c86616179565b5f5b828110156163b35784890151825560018201915060208501945060208101905061638e565b868310156163d057848901516163cc601f8916826162df565b8355505b6001600288020188555050505b505050505050565b5f6080820190506163f85f830187614fe9565b6164056020830186614fe9565b616412604083018561528c565b81810360608301526164248184615e12565b905095945050505050565b5f6040820190506164425f830185614fe9565b61644f60208301846159f5565b9392505050565b5f819050919050565b6002811061646b575f80fd5b50565b5f8135905061647c8161645f565b92915050565b5f616490602084018461646e565b905092915050565b5f80fd5b5f80fd5b5f80fd5b5f80833560016020038436030381126164c0576164bf6164a0565b5b83810192508235915060208301925067ffffffffffffffff8211156164e8576164e7616498565b5b6001820236038313156164fe576164fd61649c565b5b509250929050565b5f6165118385615cb9565b935061651e838584615671565b61652783615053565b840190509392505050565b5f604083016165435f840184616482565b61654f5f860182615ca0565b5061655d60208401846164a4565b8583036020870152616570838284616506565b925050508091505092915050565b5f6165898383616532565b905092915050565b5f823560016040038336030381126165ac576165ab6164a0565b5b82810191505092915050565b5f602082019050919050565b5f6165cf8385615c4a565b9350836020840285016165e184616456565b805f5b878110156166245784840389526165fb8284616591565b616605858261657e565b9450616610836165b8565b925060208a019950506001810190506165e4565b50829750879450505050509392505050565b5f6166418385615e02565b935061664e838584615671565b61665783615053565b840190509392505050565b5f6080820190506166755f830189614fe9565b81810360208301526166888187896165c4565b9050818103604083015261669d818587616636565b90506166ac60608301846159f5565b979650505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b5f8235600160400383360303811261670b5761670a6166e4565b5b80830191505092915050565b5f81356167238161645f565b80915050919050565b5f815f1b9050919050565b5f60ff6167438461672c565b9350801983169250808416831791505092915050565b5f61676382615c7d565b9050919050565b5f819050919050565b61677c82616759565b61678f6167888261676a565b8354616737565b8255505050565b5f80833560016020038436030381126167b2576167b16166e4565b5b80840192508235915067ffffffffffffffff8211156167d4576167d36166e8565b5b6020830192506001820236038313156167f0576167ef6166ec565b5b509250929050565b5f82905092915050565b61680c83836167f8565b67ffffffffffffffff811115616825576168246155c9565b5b61682f8254616149565b61683a82828561628d565b5f601f831160018114616867575f8415616855578287013590505b61685f85826162fb565b8655506168c6565b601f19841661687586616179565b5f5b8281101561689c57848901358255600182019150602085019450602081019050616877565b868310156168b957848901356168b5601f8916826162df565b8355505b6001600288020188555050505b50505050505050565b6168da838383616802565b505050565b5f81015f8301806168ef81616717565b90506168fb8184616773565b50505060018101602083016169108185616796565b61691b8183866168cf565b505050505050565b61692d82826168df565b5050565b5f80fd5b82818337505050565b5f6169498385615a0e565b93507f07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff83111561697c5761697b616931565b5b60208302925061698d838584616935565b82840190509392505050565b5f60a0820190506169ac5f83018a614fe9565b81810360208301526169bf81888a61693e565b905081810360408301526169d481868861693e565b90506169e36060830185614fe9565b6169f06080830184614fe9565b98975050505050505050565b5f606082019050616a0f5f830187614fe9565b8181036020830152616a22818587616636565b9050616a3160408301846159f5565b95945050505050565b5f606082019050616a4d5f830186614fe9565b616a5a6020830185614fe9565b8181036040830152616a6c8184615e12565b9050949350505050565b5f608082019050616a895f830189614fe9565b8181036020830152616a9c818789616636565b90508181036040830152616ab1818587616636565b9050616ac060608301846159f5565b979650505050505050565b5f606082019050616ade5f830187614fe9565b8181036020830152616af08186615bcd565b90508181036040830152616b05818486616636565b905095945050505050565b5f819050919050565b5f616b248385615b66565b9350616b31838584615671565b616b3a83615053565b840190509392505050565b5f616b51848484616b19565b90509392505050565b5f8083356001602003843603038112616b7657616b756164a0565b5b83810192508235915060208301925067ffffffffffffffff821115616b9e57616b9d616498565b5b600182023603831315616bb457616bb361649c565b5b509250929050565b5f602082019050919050565b5f616bd38385615b47565b935083602084028501616be584616b10565b805f5b87811015616c2a578484038952616bff8284616b5a565b616c0a868284616b45565b9550616c1584616bbc565b935060208b019a505050600181019050616be8565b50829750879450505050509392505050565b5f608082019050616c4f5f830189614fe9565b8181036020830152616c62818789616bc8565b90508181036040830152616c778185876165c4565b9050616c866060830184614fe9565b979650505050505050565b7f4549503731323a20556e696e697469616c697a656400000000000000000000005f82015250565b5f616cc560158361501b565b9150616cd082616c91565b602082019050919050565b5f6020820190508181035f830152616cf281616cb9565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f60ff82169050919050565b5f8160f81b9050919050565b5f616d4882616d32565b9050919050565b616d60616d5b82616d26565b616d3e565b82525050565b5f819050919050565b616d80616d7b82614fe0565b616d66565b82525050565b5f616d918285616d4f565b600182019150616da18284616d6f565b6020820191508190509392505050565b616dba81615315565b8114616dc4575f80fd5b50565b5f81519050616dd581616db1565b92915050565b5f60208284031215616df057616def6150c4565b5b5f616dfd84828501616dc7565b91505092915050565b5f60208284031215616e1b57616e1a6150c4565b5b5f616e288482850161646e565b91505092915050565b5f81905092915050565b5f616e468385616e31565b9350616e53838584615671565b82840190509392505050565b5f616e6b828486616e3b565b91508190509392505050565b616e8081615c8f565b82525050565b5f606082019050616e995f830186615750565b616ea66020830185616e77565b616eb36040830184615750565b949350505050565b5f81519050919050565b5f81905092915050565b5f819050602082019050919050565b616ee781615747565b82525050565b5f616ef88383616ede565b60208301905092915050565b5f602082019050919050565b5f616f1a82616ebb565b616f248185616ec5565b9350616f2f83616ecf565b805f5b83811015616f5f578151616f468882616eed565b9750616f5183616f04565b925050600181019050616f32565b5085935050505092915050565b5f616f778284616f10565b915081905092915050565b5f60a082019050616f955f830188615750565b616fa26020830187614fe9565b616faf6040830186614fe9565b616fbc6060830185615750565b616fc96080830184615750565b9695505050505050565b5f80fd5b5f80fd5b5f67ffffffffffffffff821115616ff557616ff46155c9565b5b616ffe82615053565b9050602081019050919050565b5f61701d61701884616fdb565b615627565b905082815260208101848484011115617039576170386155c5565b5b61704484828561502b565b509392505050565b5f82601f8301126170605761705f615348565b5b815161707084826020860161700b565b91505092915050565b5f6080828403121561708e5761708d616fd3565b5b6170986080615627565b90505f6170a784828501615fd6565b5f8301525060206170ba84828501615fd6565b602083015250604082015167ffffffffffffffff8111156170de576170dd616fd7565b5b6170ea8482850161704c565b604083015250606082015167ffffffffffffffff81111561710e5761710d616fd7565b5b61711a8482850161704c565b60608301525092915050565b5f6020828403121561713b5761713a6150c4565b5b5f82015167ffffffffffffffff811115617158576171576150c8565b5b61716484828501617079565b91505092915050565b5f6080820190506171805f830188614fe9565b81810360208301526171928187615bcd565b905081810360408301526171a78185876165c4565b90506171b66060830184614fe9565b9695505050505050565b5f6060820190506171d35f830187614fe9565b81810360208301526171e58186615bcd565b905081810360408301526171fa8184866165c4565b905095945050505050565b61720e81615747565b8114617218575f80fd5b50565b5f8151905061722981617205565b92915050565b5f60208284031215617244576172436150c4565b5b5f6172518482850161721b565b91505092915050565b5f60608201905061726d5f830186615750565b61727a6020830185614fe9565b6172876040830184615750565b949350505050565b5f6060820190506172a25f830186614fe9565b6172af602083018561528c565b81810360408301526172c18184615e12565b9050949350505050565b6172d481616d26565b82525050565b5f6020820190506172ed5f8301846172cb565b92915050565b5f819050815f5260205f209050919050565b601f82111561734657617317816172f3565b6173208461618b565b8101602085101561732f578190505b61734361733b8561618b565b83018261626b565b50505b505050565b61735482615011565b67ffffffffffffffff81111561736d5761736c6155c9565b5b6173778254616149565b617382828285617305565b5f60209050601f8311600181146173b3575f84156173a1578287015190505b6173ab85826162fb565b865550617412565b601f1984166173c1866172f3565b5f5b828110156173e8578489015182556001820191506020850194506020810190506173c3565b868310156174055784890151617401601f8916826162df565b8355505b6001600288020188555050505b505050505050565b5f60408201905061742d5f8301856159f5565b61743a60208301846159f5565b9392505050565b5f61744b82615caf565b6174558185616e31565b935061746581856020860161502b565b80840191505092915050565b5f61747c8284617441565b915081905092915050565b5f60a08201905061749a5f830188615750565b6174a76020830187615750565b6174b46040830186615750565b6174c16060830185614fe9565b6174ce60808301846159f5565b9695505050505050565b5f6080820190506174eb5f830187615750565b6174f860208301866172cb565b6175056040830185615750565b6175126060830184615750565b9594505050505056fe507265704b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e49642c6279746573206578747261446174612943727367656e566572696669636174696f6e2875696e743235362063727349642c75696e74323536206d61784269744c656e6774682c6279746573206372734469676573742c627974657320657874726144617461294b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e49642c75696e74323536206b657949642c4b65794469676573745b5d206b6579446967657374732c627974657320657874726144617461294b65794469676573742875696e7438206b6579547970652c627974657320646967657374294b65794469676573742875696e7438206b6579547970652c62797465732064696765737429
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP4\x80\x15b\0\0CW_\x80\xFD[Pb\0\0Tb\0\0Z` \x1B` \x1CV[b\0\x01\xC4V[_b\0\0kb\0\x01^` \x1B` \x1CV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15b\0\0\xB6W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14b\0\x01[Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Qb\0\x01R\x91\x90b\0\x01\xA9V[`@Q\x80\x91\x03\x90\xA1[PV[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[b\0\x01\xA3\x81b\0\x01\x85V[\x82RPPV[_` \x82\x01\x90Pb\0\x01\xBE_\x83\x01\x84b\0\x01\x98V[\x92\x91PPV[`\x80QavUb\0\x01\xEB_9_\x81\x81a:-\x01R\x81\x81a:\x82\x01Ra=$\x01RavU_\xF3\xFE`\x80`@R`\x046\x10a\x01\xB6W_5`\xE0\x1C\x80cX\x9A\xDB\x0E\x11a\0\xEBW\x80c\xBA\xFF!\x1E\x11a\0\x89W\x80c\xC5[\x87$\x11a\0cW\x80c\xC5[\x87$\x14a\x05\xC3W\x80c\xCA\xA3g\xDB\x14a\x06\0W\x80c\xD5/\x10\xEB\x14a\x06(W\x80c\xF0\xF8\xCB\xC6\x14a\x06RWa\x01\xB6V[\x80c\xBA\xFF!\x1E\x14a\x05[W\x80c\xC2\xC1\xFA\xEE\x14a\x05\x85W\x80c\xC4\x11Xt\x14a\x05\xADWa\x01\xB6V[\x80c\x7F\xC8\x0C]\x11a\0\xC5W\x80c\x7F\xC8\x0C]\x14a\x04\x9CW\x80c\x84\xB0\x19n\x14a\x04\xC4W\x80c\x93f\x08\xAE\x14a\x04\xF4W\x80c\xAD<\xB1\xCC\x14a\x051Wa\x01\xB6V[\x80cX\x9A\xDB\x0E\x14a\x04$W\x80c`\x8BJ\xD5\x14a\x04LW\x80cb\x97\x87\x87\x14a\x04tWa\x01\xB6V[\x80c<\x02\xF84\x11a\x01XW\x80cF\x10\xFF\xE8\x11a\x012W\x80cF\x10\xFF\xE8\x14a\x03\x8EW\x80cL)\x82)\x14a\x03\xB6W\x80cO\x1E\xF2\x86\x14a\x03\xDEW\x80cR\xD1\x90-\x14a\x03\xFAWa\x01\xB6V[\x80c<\x02\xF84\x14a\x02\xEEW\x80c=^\xC7\xE3\x14a\x03\x16W\x80cE\xAF&\x1B\x14a\x03RWa\x01\xB6V[\x80c\x17\x03\xC6\x1A\x11a\x01\x94W\x80c\x17\x03\xC6\x1A\x14a\x02JW\x80c\x19\xF4\xF62\x14a\x02rW\x80c9\xF78\x10\x14a\x02\xAEW\x80c:\xC5\0r\x14a\x02\xC4Wa\x01\xB6V[\x80c\x0Bh\x073\x14a\x01\xBAW\x80c\r\x8En,\x14a\x01\xE4W\x80c\x16\xC7\x13\xD9\x14a\x02\x0EW[_\x80\xFD[4\x80\x15a\x01\xC5W_\x80\xFD[Pa\x01\xCEa\x06\x8EV[`@Qa\x01\xDB\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xEFW_\x80\xFD[Pa\x01\xF8a\x06\xA5V[`@Qa\x02\x05\x91\x90aP\x9BV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x19W_\x80\xFD[Pa\x024`\x04\x806\x03\x81\x01\x90a\x02/\x91\x90aP\xF6V[a\x07 V[`@Qa\x02A\x91\x90aR\x08V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02UW_\x80\xFD[Pa\x02p`\x04\x806\x03\x81\x01\x90a\x02k\x91\x90aP\xF6V[a\x07\xF1V[\0[4\x80\x15a\x02}W_\x80\xFD[Pa\x02\x98`\x04\x806\x03\x81\x01\x90a\x02\x93\x91\x90aP\xF6V[a\n\x10V[`@Qa\x02\xA5\x91\x90aR\x9BV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xB9W_\x80\xFD[Pa\x02\xC2a\x0B\x16V[\0[4\x80\x15a\x02\xCFW_\x80\xFD[Pa\x02\xD8a\r9V[`@Qa\x02\xE5\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xF9W_\x80\xFD[Pa\x03\x14`\x04\x806\x03\x81\x01\x90a\x03\x0F\x91\x90aR\xD7V[a\rPV[\0[4\x80\x15a\x03!W_\x80\xFD[Pa\x03<`\x04\x806\x03\x81\x01\x90a\x037\x91\x90aP\xF6V[a\x104V[`@Qa\x03I\x91\x90aS/V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03]W_\x80\xFD[Pa\x03x`\x04\x806\x03\x81\x01\x90a\x03s\x91\x90aP\xF6V[a\x10hV[`@Qa\x03\x85\x91\x90aR\x9BV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x99W_\x80\xFD[Pa\x03\xB4`\x04\x806\x03\x81\x01\x90a\x03\xAF\x91\x90aS\xFEV[a\x11VV[\0[4\x80\x15a\x03\xC1W_\x80\xFD[Pa\x03\xDC`\x04\x806\x03\x81\x01\x90a\x03\xD7\x91\x90aT\xE4V[a\x16gV[\0[a\x03\xF8`\x04\x806\x03\x81\x01\x90a\x03\xF3\x91\x90aV\xEDV[a\x18\xABV[\0[4\x80\x15a\x04\x05W_\x80\xFD[Pa\x04\x0Ea\x18\xCAV[`@Qa\x04\x1B\x91\x90aW_V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04/W_\x80\xFD[Pa\x04J`\x04\x806\x03\x81\x01\x90a\x04E\x91\x90aWxV[a\x18\xFBV[\0[4\x80\x15a\x04WW_\x80\xFD[Pa\x04r`\x04\x806\x03\x81\x01\x90a\x04m\x91\x90aW\xD5V[a\x1C/V[\0[4\x80\x15a\x04\x7FW_\x80\xFD[Pa\x04\x9A`\x04\x806\x03\x81\x01\x90a\x04\x95\x91\x90aX2V[a\x1DqV[\0[4\x80\x15a\x04\xA7W_\x80\xFD[Pa\x04\xC2`\x04\x806\x03\x81\x01\x90a\x04\xBD\x91\x90aY\x18V[a!gV[\0[4\x80\x15a\x04\xCFW_\x80\xFD[Pa\x04\xD8a$kV[`@Qa\x04\xEB\x97\x96\x95\x94\x93\x92\x91\x90aZ\xBBV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\xFFW_\x80\xFD[Pa\x05\x1A`\x04\x806\x03\x81\x01\x90a\x05\x15\x91\x90aP\xF6V[a%tV[`@Qa\x05(\x92\x91\x90a]\xCDV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05<W_\x80\xFD[Pa\x05Ea(\xDAV[`@Qa\x05R\x91\x90aP\x9BV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05fW_\x80\xFD[Pa\x05oa)\x13V[`@Qa\x05|\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\x90W_\x80\xFD[Pa\x05\xAB`\x04\x806\x03\x81\x01\x90a\x05\xA6\x91\x90aP\xF6V[a)*V[\0[4\x80\x15a\x05\xB8W_\x80\xFD[Pa\x05\xC1a+\x94V[\0[4\x80\x15a\x05\xCEW_\x80\xFD[Pa\x05\xE9`\x04\x806\x03\x81\x01\x90a\x05\xE4\x91\x90aP\xF6V[a,\xB9V[`@Qa\x05\xF7\x92\x91\x90a^JV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06\x0BW_\x80\xFD[Pa\x06&`\x04\x806\x03\x81\x01\x90a\x06!\x91\x90a^\x7FV[a/\x8AV[\0[4\x80\x15a\x063W_\x80\xFD[Pa\x06<a1\x14V[`@Qa\x06I\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06]W_\x80\xFD[Pa\x06x`\x04\x806\x03\x81\x01\x90a\x06s\x91\x90aP\xF6V[a1+V[`@Qa\x06\x85\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xF3[_\x80a\x06\x98a1NV[\x90P\x80`\x05\x01T\x91PP\x90V[```@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x06\xE6_a1uV[a\x06\xF0`\x02a1uV[a\x06\xF9_a1uV[`@Q` \x01a\x07\x0C\x94\x93\x92\x91\x90a_xV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[``_a\x07+a1NV[\x90P_\x81`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\x02\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x82\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07\xE3W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x07\x9AW[PPPPP\x92PPP\x91\x90PV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08NW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08r\x91\x90a_\xEAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x08\xE1W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xD8\x91\x90a`\x15V[`@Q\x80\x91\x03\x90\xFD[_a\x08\xEAa1NV[\x90P\x80`\t\x01T\x82\x11\x80a\t\x06WP`\xF8`\x05`\xFF\x16\x90\x1B\x82\x11\x15[\x15a\tHW\x81`@Q\x7F\xCB\xE9&V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t?\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\t\xAAW\x81`@Q\x7F\xDF\r\xB5\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xA1\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[`\x01\x81`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F8O\x90\xFE\xFB\xCF\xAAh\xF2.\0\tJ\xEA\xA5++\xC6\x93\x93m,\xE1\xAF\xED\x12\x12R\x0BY\xB5\x8E\x82`@Qa\n\x04\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xA1PPV[_\x80a\n\x1Aa1NV[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\n}W\x82`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\nt\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[_\x80\x1B\x81`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x03a\n\xD6W\x82`@Q\x7F\x83\xF1\x835\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\xCD\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\r\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x92PPP\x91\x90PV[`\x01a\x0B a2?V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0BaW`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03_a\x0Bla2cV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x0B\xB4WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x0B\xEBW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x0C\xA4`@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa2\x8AV[_a\x0C\xADa1NV[\x90P`\xF8`\x03`\xFF\x16\x90\x1B\x81`\x04\x01\x81\x90UP`\xF8`\x04`\xFF\x16\x90\x1B\x81`\x05\x01\x81\x90UP`\xF8`\x05`\xFF\x16\x90\x1B\x81`\t\x01\x81\x90UPP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\r-\x91\x90a`PV[`@Q\x80\x91\x03\x90\xA1PPV[_\x80a\rCa1NV[\x90P\x80`\t\x01T\x91PP\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xADW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xD1\x91\x90a_\xEAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0E@W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E7\x91\x90a`\x15V[`@Q\x80\x91\x03\x90\xFD[_a\x0EIa1NV[\x90P_\x81`\t\x01T\x90P`\xF8`\x05`\xFF\x16\x90\x1B\x81\x14\x15\x80\x15a\x0E\x88WP\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15a\x0E\xCAW\x80`@Q\x7F\x06\x1A\xC6\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\xC1\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[\x81`\t\x01_\x81T\x80\x92\x91\x90a\x0E\xDE\x90a`\x96V[\x91\x90PUP_\x82`\t\x01T\x90P\x84\x83`\n\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x83\x83`\r\x01_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a\x0F8Wa\x0F7aR(V[[\x02\x17\x90UP_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x9BW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xBF\x91\x90a`\xF1V[\x90P_a\x0F\xCB\x82a2\xA0V[\x90P\x80\x85`\x0E\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x90\x81a\x0F\xED\x91\x90ac\x16V[P\x7F\x8C\xF0\x15\x13\x93\xF8O\xD6\x94\xC5\xE3\x15\xCBt\xCC\x05\xB2G\xDE\nEO\xD9\xE9\x12\x9Cf\x1E\xFD\xF9@\x1D\x83\x88\x88\x84`@Qa\x10#\x94\x93\x92\x91\x90ac\xE5V[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[_\x80a\x10>a1NV[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[_\x80a\x10ra1NV[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x10\xD5W\x82`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\xCC\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[_\x80\x1B\x81`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x03a\x11.W\x82`@Q\x7F\xD5\xFD<\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11%\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[\x80`\r\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[_a\x11_a1NV[\x90P\x80`\x05\x01T\x86\x11\x80a\x11{WP`\xF8`\x04`\xFF\x16\x90\x1B\x86\x11\x15[\x15a\x11\xBDW\x85`@Q\x7F\xAD\xFA\xB9\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\xB4\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[_\x85\x85\x90P\x03a\x12\x04W\x85`@Q\x7F\xE6\xF9\x08;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\xFB\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[_\x80a\x12\x0F\x88a2\xCCV[\x91P\x91P_\x83`\x06\x01_\x8A\x81R` \x01\x90\x81R` \x01_ T\x90P\x83`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x12\x80W`@Q\x7Fo\xBC\xDD+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x12\x8E\x82\x8B\x8B\x8B\x88a4VV[\x90P_a\x12\x9D\x84\x83\x8A\x8Aa67V[\x90P\x85_\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x13=W\x8A\x81`@Q\x7F\x98\xFB\x95}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x134\x92\x91\x90ad/V[`@Q\x80\x91\x03\x90\xFD[`\x01\x86_\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x86`\x02\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F*\xFEd\xFB:\xFD\xE8\xE2g\x8A\xEA\x84\xCF6\"?3\x0E/\xB1(m7\xAE\xD5s\xAB\x9C\xD1\xDBG\xC7\x8C\x8C\x8C\x8C\x8C3`@Qa\x14_\x96\x95\x94\x93\x92\x91\x90afbV[`@Q\x80\x91\x03\x90\xA1\x86`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x14\x9EWPa\x14\x9D\x85\x82\x80T\x90Pa6\x9FV[[\x15a\x16YW`\x01\x87`\x01\x01_\x8E\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_[\x8B\x8B\x90P\x81\x10\x15a\x15TW\x87`\x07\x01_\x8E\x81R` \x01\x90\x81R` \x01_ \x8C\x8C\x83\x81\x81\x10a\x15\x01Wa\x15\0af\xB7V[[\x90P` \x02\x81\x01\x90a\x15\x13\x91\x90af\xF0V[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P\x81\x81a\x15E\x91\x90ai#V[PP\x80\x80`\x01\x01\x91PPa\x14\xD0V[P\x82\x87`\x03\x01_\x8E\x81R` \x01\x90\x81R` \x01_ \x81\x90UP_a\x15\xFA\x86\x83\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x15\xF0W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x15\xA7W[PPPPPa7<V[\x90P_\x87Q\x11\x80\x15a\x162WP`\x03`\xFF\x16\x87_\x81Q\x81\x10a\x16\x1FWa\x16\x1Eaf\xB7V[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16\x14[\x15a\x16IWa\x16D\x88\x8D\x8D\x84\x8Ba8\x84V[a\x16WV[a\x16V\x88\x8E\x8E\x8E\x85a9\x96V[[P[PPPPPPPPPPPPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xC4W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xE8\x91\x90a_\xEAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x17WW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17N\x91\x90a`\x15V[`@Q\x80\x91\x03\x90\xFD[_a\x17`a1NV[\x90P\x80`\x05\x01T\x88\x11\x80a\x17|WP`\xF8`\x04`\xFF\x16\x90\x1B\x88\x11\x15[\x15a\x17\xBEW\x87`@Q\x7F\xAD\xFA\xB9\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17\xB5\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[\x84\x84\x90P\x87\x87\x90P\x14a\x17\xFDW`@Q\x7F\x89K*\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x18\x06\x82a9\xE3V[`\x01\x81`\x10\x01_\x8A\x81R` \x01\x90\x81R` \x01_ T\x14a\x18^W\x87`@Q\x7F\x05\xB0\x83\xF2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18U\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[\x7F\x8B\xFA}\x0E\xD6\xF8~Rkb4)\x18\xEE{\xFAS\x95+\xAD\xD4c\xDC\x93@T\xD7\xDD\x94\x0E\xAF\xDC\x88\x88\x88\x88\x88\x88\x88`@Qa\x18\x99\x97\x96\x95\x94\x93\x92\x91\x90ai\x99V[`@Q\x80\x91\x03\x90\xA1PPPPPPPPV[a\x18\xB3a:+V[a\x18\xBC\x82a;\x11V[a\x18\xC6\x82\x82a<\x04V[PPV[_a\x18\xD3a=\"V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[_a\x19\x04a1NV[\x90P\x80`\x04\x01T\x84\x11\x80a\x19 WP`\xF8`\x03`\xFF\x16\x90\x1B\x84\x11\x15[\x15a\x19bW\x83`@Q\x7F\n\xB7\xF6\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19Y\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[_\x80a\x19m\x86a2\xCCV[\x91P\x91P_a\x19|\x87\x84a=\xA9V[\x90P_a\x19\x8B\x83\x83\x89\x89a67V[\x90P\x84_\x01_\x89\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x1A+W\x87\x81`@Q\x7F3\xCA\x1F\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1A\"\x92\x91\x90ad/V[`@Q\x80\x91\x03\x90\xFD[`\x01\x85_\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x85`\x02\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7FLq\\W4\xCE\\\x18\xC9\xC1.\x84\x96\xE5=*e\xF1\xEC8\x1DGiW\xF0\xF5\x96\xB3d\xA5\x9B\x0C\x89\x89\x893`@Qa\x1BI\x94\x93\x92\x91\x90ai\xFCV[`@Q\x80\x91\x03\x90\xA1\x85`\x01\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x1B\x88WPa\x1B\x87\x84\x82\x80T\x90Pa6\x9FV[[\x15a\x1C$W`\x01\x86`\x01\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x82\x86`\x03\x01_\x8B\x81R` \x01\x90\x81R` \x01_ \x81\x90UP_\x86`\x06\x01_\x8B\x81R` \x01\x90\x81R` \x01_ T\x90P\x7F:\x11a \xCC\xA5\xD4\xF0s\xCC\x1F\xC3\x1F\xF2a3\xAB{\x04\x99\xF2q/\xA0\x10\x02;\x87\xD5\xA1\xF9\xEE\x8A\x82\x88`@Qa\x1C\x1A\x93\x92\x91\x90aj:V[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPPPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\x8CW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xB0\x91\x90a_\xEAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x1D\x1FW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1D\x16\x91\x90a`\x15V[`@Q\x80\x91\x03\x90\xFD[a\x1Dl\x83\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa>\x0BV[PPPV[_a\x1Dza1NV[\x90P\x80`\t\x01T\x86\x11\x80a\x1D\x96WP`\xF8`\x05`\xFF\x16\x90\x1B\x86\x11\x15[\x15a\x1D\xD8W\x85`@Q\x7F\x8D\x8C\x94\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1D\xCF\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[_\x80a\x1D\xE3\x88a2\xCCV[\x91P\x91P_a\x1E\x08\x89\x85`\n\x01_\x8C\x81R` \x01\x90\x81R` \x01_ T\x8A\x8A\x87a?\xC6V[\x90P_a\x1E\x17\x83\x83\x89\x89a67V[\x90P\x84_\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x1E\xB7W\x89\x81`@Q\x7F\xFC\xF5\xA6\xE9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1E\xAE\x92\x91\x90ad/V[`@Q\x80\x91\x03\x90\xFD[`\x01\x85_\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x85`\x02\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F{\xF1\xB4,\x10\xE9I|\x87\x96 \xC5\xB7\xAF\xCE\xD1\x0B\xDA\x17\xD8\xC9\x0B\"\xF0\xE3\xBCk/\xD6\xCE\xD0\xBD\x8B\x8B\x8B\x8B\x8B3`@Qa\x1F\xD9\x96\x95\x94\x93\x92\x91\x90ajvV[`@Q\x80\x91\x03\x90\xA1\x85`\x01\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a \x18WPa \x17\x84\x82\x80T\x90Pa6\x9FV[[\x15a!ZW`\x01\x86`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x89\x89\x87`\x0B\x01_\x8E\x81R` \x01\x90\x81R` \x01_ \x91\x82a j\x92\x91\x90ah\x02V[P\x82\x86`\x03\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x8A\x86`\x0C\x01\x81\x90UP_a!\x19\x85\x83\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a!\x0FW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a \xC6W[PPPPPa7<V[\x90P\x7F\"X\xB7?\xAE\xD3?\xB2\xE2\xEAED\x03\xBE\xF9t\x92\x0C\xAFh*\xB3\xA7#HO\xCFgU;\x16\xA2\x8C\x82\x8D\x8D`@Qa!P\x94\x93\x92\x91\x90aj\xCBV[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPPPPPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\xC4W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xE8\x91\x90a_\xEAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\"WW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\"N\x91\x90a`\x15V[`@Q\x80\x91\x03\x90\xFD[_a\"`a1NV[\x90P\x80`\x05\x01T\x87\x11\x80a\"|WP`\xF8`\x04`\xFF\x16\x90\x1B\x87\x11\x15[\x15a\"\xBEW\x86`@Q\x7F\xAD\xFA\xB9\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\"\xB5\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a#\x14W`@Q\x7Fo\xBC\xDD+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x86\x86\x90P\x03a#[W\x86`@Q\x7F\xE6\xF9\x08;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a#R\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[a#d\x82a9\xE3V[\x80`\x0F\x01_\x88\x81R` \x01\x90\x81R` \x01_ _a#\x82\x91\x90aO/V[_[\x86\x86\x90P\x81\x10\x15a$\x08W\x81`\x0F\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x87\x87\x83\x81\x81\x10a#\xB5Wa#\xB4af\xB7V[[\x90P` \x02\x81\x01\x90a#\xC7\x91\x90af\xF0V[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P\x81\x81a#\xF9\x91\x90ai#V[PP\x80\x80`\x01\x01\x91PPa#\x84V[P\x81\x81`\x10\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x7F\xA4vd\x86\x1A\xB5\x8C[\xD5\x04\x0E\x9C\xC4^h\xD0\xE4\x8E\xC0Cq\x03_\xD7P\x99\xE2\x17\xE0\xA6\xAA\x81\x87\x85\x85\x89\x89\x87`@Qa$Z\x96\x95\x94\x93\x92\x91\x90al<V[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[_``\x80_\x80_``_a$}a@WV[\x90P_\x80\x1B\x81_\x01T\x14\x80\x15a$\x98WP_\x80\x1B\x81`\x01\x01T\x14[a$\xD7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a$\xCE\x90al\xDBV[`@Q\x80\x91\x03\x90\xFD[a$\xDFa@~V[a$\xE7aA\x1CV[F0_\x80\x1B_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%\x06Wa%\x05aU\xC9V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a%4W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94\x93\x92\x91\x90\x97P\x97P\x97P\x97P\x97P\x97P\x97PP\x90\x91\x92\x93\x94\x95\x96V[``\x80_a%\x80a1NV[\x90P\x80`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a%\xE3W\x83`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a%\xDA\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x80\x1B\x81\x03a&@W\x84`@Q\x7F\x83\xF1\x835\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&7\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[_\x82`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a&\xE0W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a&\x97W[PPPPP\x90P_a'\x8A\x84`\x0E\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x80Ta'\t\x90aaIV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta'5\x90aaIV[\x80\x15a'\x80W\x80`\x1F\x10a'WWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a'\x80V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a'cW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPaA\xBAV[\x90P_a'\x97\x82\x84a7<V[\x90P\x80\x85`\x07\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a(\xC6W\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x01\x81\x11\x15a(\x11Wa(\x10aR(V[[`\x01\x81\x11\x15a(#Wa(\"aR(V[[\x81R` \x01`\x01\x82\x01\x80Ta(7\x90aaIV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta(c\x90aaIV[\x80\x15a(\xAEW\x80`\x1F\x10a(\x85Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a(\xAEV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a(\x91W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a'\xCDV[PPPP\x90P\x96P\x96PPPPPP\x91P\x91V[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[_\x80a)\x1Da1NV[\x90P\x80`\x0C\x01T\x91PP\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)\x87W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\xAB\x91\x90a_\xEAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a*\x1AW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a*\x11\x91\x90a`\x15V[`@Q\x80\x91\x03\x90\xFD[_a*#a1NV[\x90P\x80`\x04\x01T\x82\x11\x80a*?WP`\xF8`\x03`\xFF\x16\x90\x1B\x82\x11\x15[\x15a*\x81W\x81`@Q\x7F\xFC\xF2\xDBz\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a*x\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a*\xFAW\x82`@Q\x7F\x92x\x9Bg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a*\xF1\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[`\x01\x82`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x81\x14a+XW`\x01\x82`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP[\x7F+\x08{\x88K5\xA8\x1Dv\x9D\x1A\x1E\t(\x80\xF1\xDAV\xDE\x96NK3\x9E\xAB\xCB\x1FE\xF5\xFE2d\x83`@Qa+\x87\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xA1PPPV[`\x03_a+\x9Fa2cV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a+\xE7WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a,\x1EW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa,\xAD\x91\x90a`PV[`@Q\x80\x91\x03\x90\xA1PPV[``\x80_a,\xC5a1NV[\x90P\x80`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a-(W\x83`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a-\x1F\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x80\x1B\x81\x03a-\x85W\x84`@Q\x7F\xD5\xFD<\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a-|\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[_\x82`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a.%W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a-\xDCW[PPPPP\x90P_a.\xCF\x84`\x0E\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x80Ta.N\x90aaIV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta.z\x90aaIV[\x80\x15a.\xC5W\x80`\x1F\x10a.\x9CWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a.\xC5V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a.\xA8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPaA\xBAV[\x90P_a.\xDC\x82\x84a7<V[\x90P\x80\x85`\x0B\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80\x80Ta.\xFE\x90aaIV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta/*\x90aaIV[\x80\x15a/uW\x80`\x1F\x10a/LWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a/uV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a/XW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x96P\x96PPPPPP\x91P\x91V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/\xE7W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\x0B\x91\x90a_\xEAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a0zW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a0q\x91\x90a`\x15V[`@Q\x80\x91\x03\x90\xFD[_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a0\xD8W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\xFC\x91\x90a`\xF1V[\x90Pa1\x10\x82a1\x0B\x83a2\xA0V[a>\x0BV[PPV[_\x80a1\x1Ea1NV[\x90P\x80`\x08\x01T\x91PP\x90V[_a14a1NV[`\x10\x01_\x83\x81R` \x01\x90\x81R` \x01_ T\x90P\x91\x90PV[_\x7F&\xFD\xAF\x8A,\xB2\r \xB5^6!\x89\x86\x90^SN\xE7\xA9p\xDD/\xA8'\x94nKt\x96\xDB\0\x90P\x90V[``_`\x01a1\x83\x84aCbV[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\xA1Wa1\xA0aU\xC9V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a1\xD3W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x01\x82\x01\x90P[`\x01\x15a24W\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a2)Wa2(al\xF9V[[\x04\x94P_\x85\x03a1\xE0W[\x81\x93PPPP\x91\x90PV[_a2Ha2cV[_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[a2\x92aD\xB3V[a2\x9C\x82\x82aD\xF3V[PPV[```\x01\x82`@Q` \x01a2\xB6\x92\x91\x90am\x86V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[``_\x80a2\xD8a1NV[\x90P\x80`\x0E\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x80Ta2\xF8\x90aaIV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta3$\x90aaIV[\x80\x15a3oW\x80`\x1F\x10a3FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a3oV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a3RW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x92Pa3\x7F\x83aA\xBAV[\x91PsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cF\xC5\xBB\xBD\x833`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a3\xD0\x92\x91\x90ad/V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3\xEBW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4\x0F\x91\x90am\xDBV[a4PW3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a4G\x91\x90a`\x15V[`@Q\x80\x91\x03\x90\xFD[P\x91P\x91V[_\x80\x84\x84\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4uWa4taU\xC9V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a4\xA3W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_[\x85\x85\x90P\x81\x10\x15a5\xA7W`@Q\x80``\x01`@R\x80`%\x81R` \x01av0`%\x919\x80Q\x90` \x01 \x86\x86\x83\x81\x81\x10a4\xE6Wa4\xE5af\xB7V[[\x90P` \x02\x81\x01\x90a4\xF8\x91\x90af\xF0V[_\x01` \x81\x01\x90a5\t\x91\x90an\x06V[\x87\x87\x84\x81\x81\x10a5\x1CWa5\x1Baf\xB7V[[\x90P` \x02\x81\x01\x90a5.\x91\x90af\xF0V[\x80` \x01\x90a5=\x91\x90ag\x96V[`@Qa5K\x92\x91\x90an_V[`@Q\x80\x91\x03\x90 `@Q` \x01a5e\x93\x92\x91\x90an\x86V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x82\x81Q\x81\x10a5\x8EWa5\x8Daf\xB7V[[` \x02` \x01\x01\x81\x81RPP\x80\x80`\x01\x01\x91PPa4\xA8V[Pa6+`@Q\x80`\xC0\x01`@R\x80`\x82\x81R` \x01au\xAE`\x82\x919\x80Q\x90` \x01 \x88\x88\x84`@Q` \x01a5\xDE\x91\x90aolV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x87\x80Q\x90` \x01 `@Q` \x01a6\x10\x95\x94\x93\x92\x91\x90ao\x82V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 aEDV[\x91PP\x95\x94PPPPPV[_\x80a6\x86\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPaE]V[\x90Pa6\x93\x86\x823aE\x87V[\x80\x91PP\x94\x93PPPPV[_\x80sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cA\xAD\x06\x9C\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a6\xEE\x91\x90aO\xF8V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\tW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7-\x91\x90a`\xF1V[\x90P\x80\x83\x10\x15\x91PP\x92\x91PPV[``_\x82Q\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7^Wa7]aU\xC9V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a7\x91W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a7|W\x90P[P\x90P_[\x82\x81\x10\x15a8xWsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c1\xFFA\xC8\x87\x87\x84\x81Q\x81\x10a7\xE2Wa7\xE1af\xB7V[[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a8\x07\x92\x91\x90ad/V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8!W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8I\x91\x90aq&V[``\x01Q\x82\x82\x81Q\x81\x10a8`Wa8_af\xB7V[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa7\x96V[P\x80\x92PPP\x92\x91PPV[_a8\x8E\x82aGhV[\x90P\x85`\x0F\x01_\x82\x81R` \x01\x90\x81R` \x01_ _a8\xAE\x91\x90aO/V[_[\x85\x85\x90P\x81\x10\x15a94W\x86`\x0F\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x86\x86\x83\x81\x81\x10a8\xE1Wa8\xE0af\xB7V[[\x90P` \x02\x81\x01\x90a8\xF3\x91\x90af\xF0V[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P\x81\x81a9%\x91\x90ai#V[PP\x80\x80`\x01\x01\x91PPa8\xB0V[P`\x01\x86`\x10\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x7F\xA4vd\x86\x1A\xB5\x8C[\xD5\x04\x0E\x9C\xC4^h\xD0\xE4\x8E\xC0Cq\x03_\xD7P\x99\xE2\x17\xE0\xA6\xAA\x81\x81\x84\x87\x87`\x01`@Qa9\x86\x95\x94\x93\x92\x91\x90aqmV[`@Q\x80\x91\x03\x90\xA1PPPPPPV[\x83\x85`\x08\x01\x81\x90UP\x7F\xEB\x85\xC2m\xBC\xADF\xB8\nh\xA0\xF2L\xCE|,\x90\xF0\xA1\xFA\xDE\xD8A\x84\x13\x889\xFC\x9E\x80\xA2[\x84\x82\x85\x85`@Qa9\xD4\x94\x93\x92\x91\x90aq\xC0V[`@Q\x80\x91\x03\x90\xA1PPPPPV[`\x01\x81\x14a:(W\x80`@Q\x7F\xA3\xE9\xED~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a:\x1F\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a:\xD8WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a:\xBFaG\xB1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a;\x0FW`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;nW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;\x92\x91\x90a_\xEAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a<\x01W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a;\xF8\x91\x90a`\x15V[`@Q\x80\x91\x03\x90\xFD[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a<lWP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<i\x91\x90ar/V[`\x01[a<\xADW\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a<\xA4\x91\x90a`\x15V[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14a=\x13W\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a=\n\x91\x90aW_V[`@Q\x80\x91\x03\x90\xFD[a=\x1D\x83\x83aH\x04V[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a=\xA7W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_a>\x03`@Q\x80``\x01`@R\x80`<\x81R` \x01au\x1C`<\x919\x80Q\x90` \x01 \x84\x84\x80Q\x90` \x01 `@Q` \x01a=\xE8\x93\x92\x91\x90arZV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 aEDV[\x90P\x92\x91PPV[_a>\x14a1NV[\x90P_\x81`\x05\x01T\x90P`\xF8`\x04`\xFF\x16\x90\x1B\x81\x14\x15\x80\x15a>SWP\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15a>\x95W\x80`@Q\x7F;\x85=\xA8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a>\x8C\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[\x81`\x04\x01_\x81T\x80\x92\x91\x90a>\xA9\x90a`\x96V[\x91\x90PUP_\x82`\x04\x01T\x90P\x82`\x05\x01_\x81T\x80\x92\x91\x90a>\xCA\x90a`\x96V[\x91\x90PUP_\x83`\x05\x01T\x90P\x80\x84`\x06\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x81\x84`\x06\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x85\x84`\r\x01_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a?<Wa?;aR(V[[\x02\x17\x90UP\x84\x84`\x0E\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x90\x81a?a\x91\x90ac\x16V[P\x84\x84`\x0E\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x90\x81a?\x82\x91\x90ac\x16V[P\x7F\xFB\xF5'H\x10\xB9O\x86\x97\x0C\x11G\xE8\xFF\xAE\xBE\xD2F\xEE\x97w\xD6\x95\xA6\x90\x04\xDCbV\xD1\xFE\x91\x82\x87\x87`@Qa?\xB6\x93\x92\x91\x90ar\x8FV[`@Q\x80\x91\x03\x90\xA1PPPPPPV[_a@L`@Q\x80`\x80\x01`@R\x80`V\x81R` \x01auX`V\x919\x80Q\x90` \x01 \x87\x87\x87\x87`@Q` \x01a?\xFF\x92\x91\x90an_V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x86\x80Q\x90` \x01 `@Q` \x01a@1\x95\x94\x93\x92\x91\x90ao\x82V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 aEDV[\x90P\x95\x94PPPPPV[_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x90P\x90V[``_a@\x89a@WV[\x90P\x80`\x02\x01\x80Ta@\x9A\x90aaIV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta@\xC6\x90aaIV[\x80\x15aA\x11W\x80`\x1F\x10a@\xE8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91aA\x11V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a@\xF4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[``_aA'a@WV[\x90P\x80`\x03\x01\x80TaA8\x90aaIV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80TaAd\x90aaIV[\x80\x15aA\xAFW\x80`\x1F\x10aA\x86Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91aA\xAFV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11aA\x92W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[_\x80\x82Q\x14\x80aA\xECWP_\x82_\x81Q\x81\x10aA\xD9WaA\xD8af\xB7V[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16\x14[\x15aByWsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aBNW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aBr\x91\x90a`\xF1V[\x90PaC]V[_\x82_\x81Q\x81\x10aB\x8DWaB\x8Caf\xB7V[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P`\x01`\xFF\x16\x81`\xFF\x16\x14\x15\x80\x15aB\xBBWP`\x03`\xFF\x16\x81`\xFF\x16\x14\x15[\x15aB\xFDW\x80`@Q\x7F!9\xCC,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aB\xF4\x91\x90ar\xDAV[`@Q\x80\x91\x03\x90\xFD[_`\x03`\xFF\x16\x82`\xFF\x16\x14aC\x13W`!aC\x16V[`B[\x90P\x80\x84Q\x10\x15aCSW`@Q\x7F\x8B$\x8B`\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`!\x84\x01Q\x92PPP[\x91\x90PV[_\x80_\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10aC\xBEWz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81aC\xB4WaC\xB3al\xF9V[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10aC\xFBWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81aC\xF1WaC\xF0al\xF9V[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10aD*Wf#\x86\xF2o\xC1\0\0\x83\x81aD WaD\x1Fal\xF9V[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10aDSWc\x05\xF5\xE1\0\x83\x81aDIWaDHal\xF9V[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10aDxWa'\x10\x83\x81aDnWaDmal\xF9V[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10aD\x9BW`d\x83\x81aD\x91WaD\x90al\xF9V[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10aD\xAAW`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[aD\xBBaHvV[aD\xF1W`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[aD\xFBaD\xB3V[_aE\x04a@WV[\x90P\x82\x81`\x02\x01\x90\x81aE\x17\x91\x90asKV[P\x81\x81`\x03\x01\x90\x81aE)\x91\x90asKV[P_\x80\x1B\x81_\x01\x81\x90UP_\x80\x1B\x81`\x01\x01\x81\x90UPPPPV[_aEVaEPaH\x94V[\x83aH\xA2V[\x90P\x91\x90PV[_\x80_\x80aEk\x86\x86aH\xE2V[\x92P\x92P\x92PaE{\x82\x82aI7V[\x82\x93PPPP\x92\x91PPV[sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x94G\xCF\xD4\x84\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aE\xD6\x92\x91\x90ad/V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aE\xF1W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aF\x15\x91\x90am\xDBV[aFXW\x81\x81`@Q\x7F\r\x86\xF5!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aFO\x92\x91\x90at\x1AV[`@Q\x80\x91\x03\x90\xFD[_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c1\xFFA\xC8\x85\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aF\xA8\x92\x91\x90ad/V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aF\xC2W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aF\xEA\x91\x90aq&V[\x90P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14aGbW\x82\x82`@Q\x7F\r\x86\xF5!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aGY\x92\x91\x90at\x1AV[`@Q\x80\x91\x03\x90\xFD[PPPPV[_`B\x82Q\x10\x15aG\xA5W`@Q\x7F\x8B$\x8B`\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`A\x82\x01Q\x90P\x91\x90PV[_aG\xDD\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1BaJ\x99V[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[aH\r\x82aJ\xA2V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15aHiWaHc\x82\x82aKkV[PaHrV[aHqaK\xEBV[[PPV[_aH\x7Fa2cV[_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[_aH\x9DaL'V[\x90P\x90V[_`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x02\x82\x01R\x82`\"\x82\x01R`B\x81 \x91PP\x92\x91PPV[_\x80_`A\x84Q\x03aI\"W_\x80_` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90PaI\x14\x88\x82\x85\x85aL\x8AV[\x95P\x95P\x95PPPPaI0V[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15aIJWaIIaR(V[[\x82`\x03\x81\x11\x15aI]WaI\\aR(V[[\x03\x15aJ\x95W`\x01`\x03\x81\x11\x15aIwWaIvaR(V[[\x82`\x03\x81\x11\x15aI\x8AWaI\x89aR(V[[\x03aI\xC1W`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15aI\xD5WaI\xD4aR(V[[\x82`\x03\x81\x11\x15aI\xE8WaI\xE7aR(V[[\x03aJ,W\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aJ#\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15aJ?WaJ>aR(V[[\x82`\x03\x81\x11\x15aJRWaJQaR(V[[\x03aJ\x94W\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aJ\x8B\x91\x90aW_V[`@Q\x80\x91\x03\x90\xFD[[PPV[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03aJ\xFDW\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aJ\xF4\x91\x90a`\x15V[`@Q\x80\x91\x03\x90\xFD[\x80aK)\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1BaJ\x99V[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@QaK\x94\x91\x90atqV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14aK\xCCW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>aK\xD1V[``\x91P[P\x91P\x91PaK\xE1\x85\x83\x83aMqV[\x92PPP\x92\x91PPV[_4\x11\x15aL%W`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0FaLQaM\xFEV[aLYaNtV[F0`@Q` \x01aLo\x95\x94\x93\x92\x91\x90at\x87V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x80_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15aL\xC6W_`\x03\x85\x92P\x92P\x92PaMgV[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@QaL\xE9\x94\x93\x92\x91\x90at\xD8V[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aM\tW=_\x80>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03aMZW_`\x01_\x80\x1B\x93P\x93P\x93PPaMgV[\x80_\x80_\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[``\x82aM\x86WaM\x81\x82aN\xEBV[aM\xF6V[_\x82Q\x14\x80\x15aM\xACWP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15aM\xEEW\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aM\xE5\x91\x90a`\x15V[`@Q\x80\x91\x03\x90\xFD[\x81\x90PaM\xF7V[[\x93\x92PPPV[_\x80aN\x08a@WV[\x90P_aN\x13a@~V[\x90P_\x81Q\x11\x15aN/W\x80\x80Q\x90` \x01 \x92PPPaNqV[_\x82_\x01T\x90P_\x80\x1B\x81\x14aNJW\x80\x93PPPPaNqV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x80aN~a@WV[\x90P_aN\x89aA\x1CV[\x90P_\x81Q\x11\x15aN\xA5W\x80\x80Q\x90` \x01 \x92PPPaN\xE8V[_\x82`\x01\x01T\x90P_\x80\x1B\x81\x14aN\xC1W\x80\x93PPPPaN\xE8V[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x81Q\x11\x15aN\xFDW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T_\x82U`\x02\x02\x90_R` _ \x90\x81\x01\x90aOM\x91\x90aOPV[PV[[\x80\x82\x11\x15aO\x84W_\x80\x82\x01_a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U`\x01\x82\x01_aO{\x91\x90aO\x88V[P`\x02\x01aOQV[P\x90V[P\x80TaO\x94\x90aaIV[_\x82U\x80`\x1F\x10aO\xA5WPaO\xC2V[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90aO\xC1\x91\x90aO\xC5V[[PV[[\x80\x82\x11\x15aO\xDCW_\x81_\x90UP`\x01\x01aO\xC6V[P\x90V[_\x81\x90P\x91\x90PV[aO\xF2\x81aO\xE0V[\x82RPPV[_` \x82\x01\x90PaP\x0B_\x83\x01\x84aO\xE9V[\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15aPHW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90PaP-V[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_aPm\x82aP\x11V[aPw\x81\x85aP\x1BV[\x93PaP\x87\x81\x85` \x86\x01aP+V[aP\x90\x81aPSV[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaP\xB3\x81\x84aPcV[\x90P\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[aP\xD5\x81aO\xE0V[\x81\x14aP\xDFW_\x80\xFD[PV[_\x815\x90PaP\xF0\x81aP\xCCV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aQ\x0BWaQ\naP\xC4V[[_aQ\x18\x84\x82\x85\x01aP\xE2V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_aQs\x82aQJV[\x90P\x91\x90PV[aQ\x83\x81aQiV[\x82RPPV[_aQ\x94\x83\x83aQzV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aQ\xB6\x82aQ!V[aQ\xC0\x81\x85aQ+V[\x93PaQ\xCB\x83aQ;V[\x80_[\x83\x81\x10\x15aQ\xFBW\x81QaQ\xE2\x88\x82aQ\x89V[\x97PaQ\xED\x83aQ\xA0V[\x92PP`\x01\x81\x01\x90PaQ\xCEV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaR \x81\x84aQ\xACV[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x02\x81\x10aRfWaReaR(V[[PV[_\x81\x90PaRv\x82aRUV[\x91\x90PV[_aR\x85\x82aRiV[\x90P\x91\x90PV[aR\x95\x81aR{V[\x82RPPV[_` \x82\x01\x90PaR\xAE_\x83\x01\x84aR\x8CV[\x92\x91PPV[`\x02\x81\x10aR\xC0W_\x80\xFD[PV[_\x815\x90PaR\xD1\x81aR\xB4V[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15aR\xEDWaR\xECaP\xC4V[[_aR\xFA\x85\x82\x86\x01aP\xE2V[\x92PP` aS\x0B\x85\x82\x86\x01aR\xC3V[\x91PP\x92P\x92\x90PV[_\x81\x15\x15\x90P\x91\x90PV[aS)\x81aS\x15V[\x82RPPV[_` \x82\x01\x90PaSB_\x83\x01\x84aS V[\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12aSiWaShaSHV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aS\x86WaS\x85aSLV[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aS\xA2WaS\xA1aSPV[[\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12aS\xBEWaS\xBDaSHV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aS\xDBWaS\xDAaSLV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15aS\xF7WaS\xF6aSPV[[\x92P\x92\x90PV[_\x80_\x80_``\x86\x88\x03\x12\x15aT\x17WaT\x16aP\xC4V[[_aT$\x88\x82\x89\x01aP\xE2V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aTEWaTDaP\xC8V[[aTQ\x88\x82\x89\x01aSTV[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aTtWaTsaP\xC8V[[aT\x80\x88\x82\x89\x01aS\xA9V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[_\x80\x83`\x1F\x84\x01\x12aT\xA4WaT\xA3aSHV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aT\xC1WaT\xC0aSLV[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aT\xDDWaT\xDCaSPV[[\x92P\x92\x90PV[_\x80_\x80_\x80_`\xA0\x88\x8A\x03\x12\x15aT\xFFWaT\xFEaP\xC4V[[_aU\x0C\x8A\x82\x8B\x01aP\xE2V[\x97PP` \x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aU-WaU,aP\xC8V[[aU9\x8A\x82\x8B\x01aT\x8FV[\x96P\x96PP`@\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aU\\WaU[aP\xC8V[[aUh\x8A\x82\x8B\x01aT\x8FV[\x94P\x94PP``aU{\x8A\x82\x8B\x01aP\xE2V[\x92PP`\x80aU\x8C\x8A\x82\x8B\x01aP\xE2V[\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[aU\xA4\x81aQiV[\x81\x14aU\xAEW_\x80\xFD[PV[_\x815\x90PaU\xBF\x81aU\x9BV[\x92\x91PPV[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[aU\xFF\x82aPSV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aV\x1EWaV\x1DaU\xC9V[[\x80`@RPPPV[_aV0aP\xBBV[\x90PaV<\x82\x82aU\xF6V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aV[WaVZaU\xC9V[[aVd\x82aPSV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_aV\x91aV\x8C\x84aVAV[aV'V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15aV\xADWaV\xACaU\xC5V[[aV\xB8\x84\x82\x85aVqV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aV\xD4WaV\xD3aSHV[[\x815aV\xE4\x84\x82` \x86\x01aV\x7FV[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15aW\x03WaW\x02aP\xC4V[[_aW\x10\x85\x82\x86\x01aU\xB1V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aW1WaW0aP\xC8V[[aW=\x85\x82\x86\x01aV\xC0V[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[aWY\x81aWGV[\x82RPPV[_` \x82\x01\x90PaWr_\x83\x01\x84aWPV[\x92\x91PPV[_\x80_`@\x84\x86\x03\x12\x15aW\x8FWaW\x8EaP\xC4V[[_aW\x9C\x86\x82\x87\x01aP\xE2V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aW\xBDWaW\xBCaP\xC8V[[aW\xC9\x86\x82\x87\x01aS\xA9V[\x92P\x92PP\x92P\x92P\x92V[_\x80_`@\x84\x86\x03\x12\x15aW\xECWaW\xEBaP\xC4V[[_aW\xF9\x86\x82\x87\x01aR\xC3V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aX\x1AWaX\x19aP\xC8V[[aX&\x86\x82\x87\x01aS\xA9V[\x92P\x92PP\x92P\x92P\x92V[_\x80_\x80_``\x86\x88\x03\x12\x15aXKWaXJaP\xC4V[[_aXX\x88\x82\x89\x01aP\xE2V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aXyWaXxaP\xC8V[[aX\x85\x88\x82\x89\x01aS\xA9V[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aX\xA8WaX\xA7aP\xC8V[[aX\xB4\x88\x82\x89\x01aS\xA9V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[_\x80\x83`\x1F\x84\x01\x12aX\xD8WaX\xD7aSHV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aX\xF5WaX\xF4aSLV[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aY\x11WaY\x10aSPV[[\x92P\x92\x90PV[_\x80_\x80_\x80`\x80\x87\x89\x03\x12\x15aY2WaY1aP\xC4V[[_aY?\x89\x82\x8A\x01aP\xE2V[\x96PP` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aY`WaY_aP\xC8V[[aYl\x89\x82\x8A\x01aSTV[\x95P\x95PP`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aY\x8FWaY\x8EaP\xC8V[[aY\x9B\x89\x82\x8A\x01aX\xC3V[\x93P\x93PP``aY\xAE\x89\x82\x8A\x01aP\xE2V[\x91PP\x92\x95P\x92\x95P\x92\x95V[_\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[aY\xEF\x81aY\xBBV[\x82RPPV[aY\xFE\x81aQiV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[aZ6\x81aO\xE0V[\x82RPPV[_aZG\x83\x83aZ-V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aZi\x82aZ\x04V[aZs\x81\x85aZ\x0EV[\x93PaZ~\x83aZ\x1EV[\x80_[\x83\x81\x10\x15aZ\xAEW\x81QaZ\x95\x88\x82aZ<V[\x97PaZ\xA0\x83aZSV[\x92PP`\x01\x81\x01\x90PaZ\x81V[P\x85\x93PPPP\x92\x91PPV[_`\xE0\x82\x01\x90PaZ\xCE_\x83\x01\x8AaY\xE6V[\x81\x81\x03` \x83\x01RaZ\xE0\x81\x89aPcV[\x90P\x81\x81\x03`@\x83\x01RaZ\xF4\x81\x88aPcV[\x90Pa[\x03``\x83\x01\x87aO\xE9V[a[\x10`\x80\x83\x01\x86aY\xF5V[a[\x1D`\xA0\x83\x01\x85aWPV[\x81\x81\x03`\xC0\x83\x01Ra[/\x81\x84aZ_V[\x90P\x98\x97PPPPPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a[\x80\x82aP\x11V[a[\x8A\x81\x85a[fV[\x93Pa[\x9A\x81\x85` \x86\x01aP+V[a[\xA3\x81aPSV[\x84\x01\x91PP\x92\x91PPV[_a[\xB9\x83\x83a[vV[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a[\xD7\x82a[=V[a[\xE1\x81\x85a[GV[\x93P\x83` \x82\x02\x85\x01a[\xF3\x85a[WV[\x80_[\x85\x81\x10\x15a\\.W\x84\x84\x03\x89R\x81Qa\\\x0F\x85\x82a[\xAEV[\x94Pa\\\x1A\x83a[\xC1V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa[\xF6V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[`\x02\x81\x10a\\zWa\\yaR(V[[PV[_\x81\x90Pa\\\x8A\x82a\\iV[\x91\x90PV[_a\\\x99\x82a\\}V[\x90P\x91\x90PV[a\\\xA9\x81a\\\x8FV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a\\\xD3\x82a\\\xAFV[a\\\xDD\x81\x85a\\\xB9V[\x93Pa\\\xED\x81\x85` \x86\x01aP+V[a\\\xF6\x81aPSV[\x84\x01\x91PP\x92\x91PPV[_`@\x83\x01_\x83\x01Qa]\x16_\x86\x01\x82a\\\xA0V[P` \x83\x01Q\x84\x82\x03` \x86\x01Ra].\x82\x82a\\\xC9V[\x91PP\x80\x91PP\x92\x91PPV[_a]F\x83\x83a]\x01V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a]d\x82a\\@V[a]n\x81\x85a\\JV[\x93P\x83` \x82\x02\x85\x01a]\x80\x85a\\ZV[\x80_[\x85\x81\x10\x15a]\xBBW\x84\x84\x03\x89R\x81Qa]\x9C\x85\x82a];V[\x94Pa]\xA7\x83a]NV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa]\x83V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra]\xE5\x81\x85a[\xCDV[\x90P\x81\x81\x03` \x83\x01Ra]\xF9\x81\x84a]ZV[\x90P\x93\x92PPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a^\x1C\x82a\\\xAFV[a^&\x81\x85a^\x02V[\x93Pa^6\x81\x85` \x86\x01aP+V[a^?\x81aPSV[\x84\x01\x91PP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra^b\x81\x85a[\xCDV[\x90P\x81\x81\x03` \x83\x01Ra^v\x81\x84a^\x12V[\x90P\x93\x92PPPV[_` \x82\x84\x03\x12\x15a^\x94Wa^\x93aP\xC4V[[_a^\xA1\x84\x82\x85\x01aR\xC3V[\x91PP\x92\x91PPV[_\x81\x90P\x92\x91PPV[_a^\xBE\x82aP\x11V[a^\xC8\x81\x85a^\xAAV[\x93Pa^\xD8\x81\x85` \x86\x01aP+V[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a_\x18`\x02\x83a^\xAAV[\x91Pa_#\x82a^\xE4V[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a_b`\x01\x83a^\xAAV[\x91Pa_m\x82a_.V[`\x01\x82\x01\x90P\x91\x90PV[_a_\x83\x82\x87a^\xB4V[\x91Pa_\x8E\x82a_\x0CV[\x91Pa_\x9A\x82\x86a^\xB4V[\x91Pa_\xA5\x82a_VV[\x91Pa_\xB1\x82\x85a^\xB4V[\x91Pa_\xBC\x82a_VV[\x91Pa_\xC8\x82\x84a^\xB4V[\x91P\x81\x90P\x95\x94PPPPPV[_\x81Q\x90Pa_\xE4\x81aU\x9BV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a_\xFFWa_\xFEaP\xC4V[[_a`\x0C\x84\x82\x85\x01a_\xD6V[\x91PP\x92\x91PPV[_` \x82\x01\x90Pa`(_\x83\x01\x84aY\xF5V[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a`J\x81a`.V[\x82RPPV[_` \x82\x01\x90Pa`c_\x83\x01\x84a`AV[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a`\xA0\x82aO\xE0V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a`\xD2Wa`\xD1a`iV[[`\x01\x82\x01\x90P\x91\x90PV[_\x81Q\x90Pa`\xEB\x81aP\xCCV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aa\x06Waa\x05aP\xC4V[[_aa\x13\x84\x82\x85\x01a`\xDDV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80aa`W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aasWaaraa\x1CV[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02aa\xD5\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82aa\x9AV[aa\xDF\x86\x83aa\x9AV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_ab\x1Aab\x15ab\x10\x84aO\xE0V[aa\xF7V[aO\xE0V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[ab3\x83ab\0V[abGab?\x82ab!V[\x84\x84Taa\xA6V[\x82UPPPPV[_\x90V[ab[abOV[abf\x81\x84\x84ab*V[PPPV[[\x81\x81\x10\x15ab\x89Wab~_\x82abSV[`\x01\x81\x01\x90PablV[PPV[`\x1F\x82\x11\x15ab\xCEWab\x9F\x81aayV[ab\xA8\x84aa\x8BV[\x81\x01` \x85\x10\x15ab\xB7W\x81\x90P[ab\xCBab\xC3\x85aa\x8BV[\x83\x01\x82abkV[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_ab\xEE_\x19\x84`\x08\x02ab\xD3V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_ac\x06\x83\x83ab\xDFV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[ac\x1F\x82a\\\xAFV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ac8Wac7aU\xC9V[[acB\x82TaaIV[acM\x82\x82\x85ab\x8DV[_` \x90P`\x1F\x83\x11`\x01\x81\x14ac~W_\x84\x15aclW\x82\x87\x01Q\x90P[acv\x85\x82ab\xFBV[\x86UPac\xDDV[`\x1F\x19\x84\x16ac\x8C\x86aayV[_[\x82\x81\x10\x15ac\xB3W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pac\x8EV[\x86\x83\x10\x15ac\xD0W\x84\x89\x01Qac\xCC`\x1F\x89\x16\x82ab\xDFV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_`\x80\x82\x01\x90Pac\xF8_\x83\x01\x87aO\xE9V[ad\x05` \x83\x01\x86aO\xE9V[ad\x12`@\x83\x01\x85aR\x8CV[\x81\x81\x03``\x83\x01Rad$\x81\x84a^\x12V[\x90P\x95\x94PPPPPV[_`@\x82\x01\x90PadB_\x83\x01\x85aO\xE9V[adO` \x83\x01\x84aY\xF5V[\x93\x92PPPV[_\x81\x90P\x91\x90PV[`\x02\x81\x10adkW_\x80\xFD[PV[_\x815\x90Pad|\x81ad_V[\x92\x91PPV[_ad\x90` \x84\x01\x84adnV[\x90P\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12ad\xC0Wad\xBFad\xA0V[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15ad\xE8Wad\xE7ad\x98V[[`\x01\x82\x026\x03\x83\x13\x15ad\xFEWad\xFDad\x9CV[[P\x92P\x92\x90PV[_ae\x11\x83\x85a\\\xB9V[\x93Pae\x1E\x83\x85\x84aVqV[ae'\x83aPSV[\x84\x01\x90P\x93\x92PPPV[_`@\x83\x01aeC_\x84\x01\x84ad\x82V[aeO_\x86\x01\x82a\\\xA0V[Pae]` \x84\x01\x84ad\xA4V[\x85\x83\x03` \x87\x01Raep\x83\x82\x84ae\x06V[\x92PPP\x80\x91PP\x92\x91PPV[_ae\x89\x83\x83ae2V[\x90P\x92\x91PPV[_\x825`\x01`@\x03\x836\x03\x03\x81\x12ae\xACWae\xABad\xA0V[[\x82\x81\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_ae\xCF\x83\x85a\\JV[\x93P\x83` \x84\x02\x85\x01ae\xE1\x84adVV[\x80_[\x87\x81\x10\x15af$W\x84\x84\x03\x89Rae\xFB\x82\x84ae\x91V[af\x05\x85\x82ae~V[\x94Paf\x10\x83ae\xB8V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pae\xE4V[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_afA\x83\x85a^\x02V[\x93PafN\x83\x85\x84aVqV[afW\x83aPSV[\x84\x01\x90P\x93\x92PPPV[_`\x80\x82\x01\x90Pafu_\x83\x01\x89aO\xE9V[\x81\x81\x03` \x83\x01Raf\x88\x81\x87\x89ae\xC4V[\x90P\x81\x81\x03`@\x83\x01Raf\x9D\x81\x85\x87af6V[\x90Paf\xAC``\x83\x01\x84aY\xF5V[\x97\x96PPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x825`\x01`@\x03\x836\x03\x03\x81\x12ag\x0BWag\naf\xE4V[[\x80\x83\x01\x91PP\x92\x91PPV[_\x815ag#\x81ad_V[\x80\x91PP\x91\x90PV[_\x81_\x1B\x90P\x91\x90PV[_`\xFFagC\x84ag,V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_agc\x82a\\}V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[ag|\x82agYV[ag\x8Fag\x88\x82agjV[\x83Tag7V[\x82UPPPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12ag\xB2Wag\xB1af\xE4V[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15ag\xD4Wag\xD3af\xE8V[[` \x83\x01\x92P`\x01\x82\x026\x03\x83\x13\x15ag\xF0Wag\xEFaf\xECV[[P\x92P\x92\x90PV[_\x82\x90P\x92\x91PPV[ah\x0C\x83\x83ag\xF8V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ah%Wah$aU\xC9V[[ah/\x82TaaIV[ah:\x82\x82\x85ab\x8DV[_`\x1F\x83\x11`\x01\x81\x14ahgW_\x84\x15ahUW\x82\x87\x015\x90P[ah_\x85\x82ab\xFBV[\x86UPah\xC6V[`\x1F\x19\x84\x16ahu\x86aayV[_[\x82\x81\x10\x15ah\x9CW\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PahwV[\x86\x83\x10\x15ah\xB9W\x84\x89\x015ah\xB5`\x1F\x89\x16\x82ab\xDFV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[ah\xDA\x83\x83\x83ah\x02V[PPPV[_\x81\x01_\x83\x01\x80ah\xEF\x81ag\x17V[\x90Pah\xFB\x81\x84agsV[PPP`\x01\x81\x01` \x83\x01ai\x10\x81\x85ag\x96V[ai\x1B\x81\x83\x86ah\xCFV[PPPPPPV[ai-\x82\x82ah\xDFV[PPV[_\x80\xFD[\x82\x81\x837PPPV[_aiI\x83\x85aZ\x0EV[\x93P\x7F\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15ai|Wai{ai1V[[` \x83\x02\x92Pai\x8D\x83\x85\x84ai5V[\x82\x84\x01\x90P\x93\x92PPPV[_`\xA0\x82\x01\x90Pai\xAC_\x83\x01\x8AaO\xE9V[\x81\x81\x03` \x83\x01Rai\xBF\x81\x88\x8Aai>V[\x90P\x81\x81\x03`@\x83\x01Rai\xD4\x81\x86\x88ai>V[\x90Pai\xE3``\x83\x01\x85aO\xE9V[ai\xF0`\x80\x83\x01\x84aO\xE9V[\x98\x97PPPPPPPPV[_``\x82\x01\x90Paj\x0F_\x83\x01\x87aO\xE9V[\x81\x81\x03` \x83\x01Raj\"\x81\x85\x87af6V[\x90Paj1`@\x83\x01\x84aY\xF5V[\x95\x94PPPPPV[_``\x82\x01\x90PajM_\x83\x01\x86aO\xE9V[ajZ` \x83\x01\x85aO\xE9V[\x81\x81\x03`@\x83\x01Rajl\x81\x84a^\x12V[\x90P\x94\x93PPPPV[_`\x80\x82\x01\x90Paj\x89_\x83\x01\x89aO\xE9V[\x81\x81\x03` \x83\x01Raj\x9C\x81\x87\x89af6V[\x90P\x81\x81\x03`@\x83\x01Raj\xB1\x81\x85\x87af6V[\x90Paj\xC0``\x83\x01\x84aY\xF5V[\x97\x96PPPPPPPV[_``\x82\x01\x90Paj\xDE_\x83\x01\x87aO\xE9V[\x81\x81\x03` \x83\x01Raj\xF0\x81\x86a[\xCDV[\x90P\x81\x81\x03`@\x83\x01Rak\x05\x81\x84\x86af6V[\x90P\x95\x94PPPPPV[_\x81\x90P\x91\x90PV[_ak$\x83\x85a[fV[\x93Pak1\x83\x85\x84aVqV[ak:\x83aPSV[\x84\x01\x90P\x93\x92PPPV[_akQ\x84\x84\x84ak\x19V[\x90P\x93\x92PPPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12akvWakuad\xA0V[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15ak\x9EWak\x9Dad\x98V[[`\x01\x82\x026\x03\x83\x13\x15ak\xB4Wak\xB3ad\x9CV[[P\x92P\x92\x90PV[_` \x82\x01\x90P\x91\x90PV[_ak\xD3\x83\x85a[GV[\x93P\x83` \x84\x02\x85\x01ak\xE5\x84ak\x10V[\x80_[\x87\x81\x10\x15al*W\x84\x84\x03\x89Rak\xFF\x82\x84akZV[al\n\x86\x82\x84akEV[\x95Pal\x15\x84ak\xBCV[\x93P` \x8B\x01\x9APPP`\x01\x81\x01\x90Pak\xE8V[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_`\x80\x82\x01\x90PalO_\x83\x01\x89aO\xE9V[\x81\x81\x03` \x83\x01Ralb\x81\x87\x89ak\xC8V[\x90P\x81\x81\x03`@\x83\x01Ralw\x81\x85\x87ae\xC4V[\x90Pal\x86``\x83\x01\x84aO\xE9V[\x97\x96PPPPPPPV[\x7FEIP712: Uninitialized\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_al\xC5`\x15\x83aP\x1BV[\x91Pal\xD0\x82al\x91V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ral\xF2\x81al\xB9V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_`\xFF\x82\x16\x90P\x91\x90PV[_\x81`\xF8\x1B\x90P\x91\x90PV[_amH\x82am2V[\x90P\x91\x90PV[am`am[\x82am&V[am>V[\x82RPPV[_\x81\x90P\x91\x90PV[am\x80am{\x82aO\xE0V[amfV[\x82RPPV[_am\x91\x82\x85amOV[`\x01\x82\x01\x91Pam\xA1\x82\x84amoV[` \x82\x01\x91P\x81\x90P\x93\x92PPPV[am\xBA\x81aS\x15V[\x81\x14am\xC4W_\x80\xFD[PV[_\x81Q\x90Pam\xD5\x81am\xB1V[\x92\x91PPV[_` \x82\x84\x03\x12\x15am\xF0Wam\xEFaP\xC4V[[_am\xFD\x84\x82\x85\x01am\xC7V[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15an\x1BWan\x1AaP\xC4V[[_an(\x84\x82\x85\x01adnV[\x91PP\x92\x91PPV[_\x81\x90P\x92\x91PPV[_anF\x83\x85an1V[\x93PanS\x83\x85\x84aVqV[\x82\x84\x01\x90P\x93\x92PPPV[_ank\x82\x84\x86an;V[\x91P\x81\x90P\x93\x92PPPV[an\x80\x81a\\\x8FV[\x82RPPV[_``\x82\x01\x90Pan\x99_\x83\x01\x86aWPV[an\xA6` \x83\x01\x85anwV[an\xB3`@\x83\x01\x84aWPV[\x94\x93PPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[an\xE7\x81aWGV[\x82RPPV[_an\xF8\x83\x83an\xDEV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_ao\x1A\x82an\xBBV[ao$\x81\x85an\xC5V[\x93Pao/\x83an\xCFV[\x80_[\x83\x81\x10\x15ao_W\x81QaoF\x88\x82an\xEDV[\x97PaoQ\x83ao\x04V[\x92PP`\x01\x81\x01\x90Pao2V[P\x85\x93PPPP\x92\x91PPV[_aow\x82\x84ao\x10V[\x91P\x81\x90P\x92\x91PPV[_`\xA0\x82\x01\x90Pao\x95_\x83\x01\x88aWPV[ao\xA2` \x83\x01\x87aO\xE9V[ao\xAF`@\x83\x01\x86aO\xE9V[ao\xBC``\x83\x01\x85aWPV[ao\xC9`\x80\x83\x01\x84aWPV[\x96\x95PPPPPPV[_\x80\xFD[_\x80\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15ao\xF5Wao\xF4aU\xC9V[[ao\xFE\x82aPSV[\x90P` \x81\x01\x90P\x91\x90PV[_ap\x1Dap\x18\x84ao\xDBV[aV'V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15ap9Wap8aU\xC5V[[apD\x84\x82\x85aP+V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12ap`Wap_aSHV[[\x81Qapp\x84\x82` \x86\x01ap\x0BV[\x91PP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15ap\x8EWap\x8Dao\xD3V[[ap\x98`\x80aV'V[\x90P_ap\xA7\x84\x82\x85\x01a_\xD6V[_\x83\x01RP` ap\xBA\x84\x82\x85\x01a_\xD6V[` \x83\x01RP`@\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ap\xDEWap\xDDao\xD7V[[ap\xEA\x84\x82\x85\x01apLV[`@\x83\x01RP``\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aq\x0EWaq\rao\xD7V[[aq\x1A\x84\x82\x85\x01apLV[``\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15aq;Waq:aP\xC4V[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aqXWaqWaP\xC8V[[aqd\x84\x82\x85\x01apyV[\x91PP\x92\x91PPV[_`\x80\x82\x01\x90Paq\x80_\x83\x01\x88aO\xE9V[\x81\x81\x03` \x83\x01Raq\x92\x81\x87a[\xCDV[\x90P\x81\x81\x03`@\x83\x01Raq\xA7\x81\x85\x87ae\xC4V[\x90Paq\xB6``\x83\x01\x84aO\xE9V[\x96\x95PPPPPPV[_``\x82\x01\x90Paq\xD3_\x83\x01\x87aO\xE9V[\x81\x81\x03` \x83\x01Raq\xE5\x81\x86a[\xCDV[\x90P\x81\x81\x03`@\x83\x01Raq\xFA\x81\x84\x86ae\xC4V[\x90P\x95\x94PPPPPV[ar\x0E\x81aWGV[\x81\x14ar\x18W_\x80\xFD[PV[_\x81Q\x90Par)\x81ar\x05V[\x92\x91PPV[_` \x82\x84\x03\x12\x15arDWarCaP\xC4V[[_arQ\x84\x82\x85\x01ar\x1BV[\x91PP\x92\x91PPV[_``\x82\x01\x90Parm_\x83\x01\x86aWPV[arz` \x83\x01\x85aO\xE9V[ar\x87`@\x83\x01\x84aWPV[\x94\x93PPPPV[_``\x82\x01\x90Par\xA2_\x83\x01\x86aO\xE9V[ar\xAF` \x83\x01\x85aR\x8CV[\x81\x81\x03`@\x83\x01Rar\xC1\x81\x84a^\x12V[\x90P\x94\x93PPPPV[ar\xD4\x81am&V[\x82RPPV[_` \x82\x01\x90Par\xED_\x83\x01\x84ar\xCBV[\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[`\x1F\x82\x11\x15asFWas\x17\x81ar\xF3V[as \x84aa\x8BV[\x81\x01` \x85\x10\x15as/W\x81\x90P[asCas;\x85aa\x8BV[\x83\x01\x82abkV[PP[PPPV[asT\x82aP\x11V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15asmWaslaU\xC9V[[asw\x82TaaIV[as\x82\x82\x82\x85as\x05V[_` \x90P`\x1F\x83\x11`\x01\x81\x14as\xB3W_\x84\x15as\xA1W\x82\x87\x01Q\x90P[as\xAB\x85\x82ab\xFBV[\x86UPat\x12V[`\x1F\x19\x84\x16as\xC1\x86ar\xF3V[_[\x82\x81\x10\x15as\xE8W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pas\xC3V[\x86\x83\x10\x15at\x05W\x84\x89\x01Qat\x01`\x1F\x89\x16\x82ab\xDFV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_`@\x82\x01\x90Pat-_\x83\x01\x85aY\xF5V[at:` \x83\x01\x84aY\xF5V[\x93\x92PPPV[_atK\x82a\\\xAFV[atU\x81\x85an1V[\x93Pate\x81\x85` \x86\x01aP+V[\x80\x84\x01\x91PP\x92\x91PPV[_at|\x82\x84atAV[\x91P\x81\x90P\x92\x91PPV[_`\xA0\x82\x01\x90Pat\x9A_\x83\x01\x88aWPV[at\xA7` \x83\x01\x87aWPV[at\xB4`@\x83\x01\x86aWPV[at\xC1``\x83\x01\x85aO\xE9V[at\xCE`\x80\x83\x01\x84aY\xF5V[\x96\x95PPPPPPV[_`\x80\x82\x01\x90Pat\xEB_\x83\x01\x87aWPV[at\xF8` \x83\x01\x86ar\xCBV[au\x05`@\x83\x01\x85aWPV[au\x12``\x83\x01\x84aWPV[\x95\x94PPPPPV\xFEPrepKeygenVerification(uint256 prepKeygenId,bytes extraData)CrsgenVerification(uint256 crsId,uint256 maxBitLength,bytes crsDigest,bytes extraData)KeygenVerification(uint256 prepKeygenId,uint256 keyId,KeyDigest[] keyDigests,bytes extraData)KeyDigest(uint8 keyType,bytes digest)KeyDigest(uint8 keyType,bytes digest)",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106101b6575f3560e01c8063589adb0e116100eb578063baff211e11610089578063c55b872411610063578063c55b8724146105c3578063caa367db14610600578063d52f10eb14610628578063f0f8cbc614610652576101b6565b8063baff211e1461055b578063c2c1faee14610585578063c4115874146105ad576101b6565b80637fc80c5d116100c55780637fc80c5d1461049c57806384b0196e146104c4578063936608ae146104f4578063ad3cb1cc14610531576101b6565b8063589adb0e14610424578063608b4ad51461044c5780636297878714610474576101b6565b80633c02f834116101585780634610ffe8116101325780634610ffe81461038e5780634c298229146103b65780634f1ef286146103de57806352d1902d146103fa576101b6565b80633c02f834146102ee5780633d5ec7e31461031657806345af261b14610352576101b6565b80631703c61a116101945780631703c61a1461024a57806319f4f6321461027257806339f73810146102ae5780633ac50072146102c4576101b6565b80630b680733146101ba5780630d8e6e2c146101e457806316c713d91461020e575b5f80fd5b3480156101c5575f80fd5b506101ce61068e565b6040516101db9190614ff8565b60405180910390f35b3480156101ef575f80fd5b506101f86106a5565b604051610205919061509b565b60405180910390f35b348015610219575f80fd5b50610234600480360381019061022f91906150f6565b610720565b6040516102419190615208565b60405180910390f35b348015610255575f80fd5b50610270600480360381019061026b91906150f6565b6107f1565b005b34801561027d575f80fd5b50610298600480360381019061029391906150f6565b610a10565b6040516102a5919061529b565b60405180910390f35b3480156102b9575f80fd5b506102c2610b16565b005b3480156102cf575f80fd5b506102d8610d39565b6040516102e59190614ff8565b60405180910390f35b3480156102f9575f80fd5b50610314600480360381019061030f91906152d7565b610d50565b005b348015610321575f80fd5b5061033c600480360381019061033791906150f6565b611034565b604051610349919061532f565b60405180910390f35b34801561035d575f80fd5b50610378600480360381019061037391906150f6565b611068565b604051610385919061529b565b60405180910390f35b348015610399575f80fd5b506103b460048036038101906103af91906153fe565b611156565b005b3480156103c1575f80fd5b506103dc60048036038101906103d791906154e4565b611667565b005b6103f860048036038101906103f391906156ed565b6118ab565b005b348015610405575f80fd5b5061040e6118ca565b60405161041b919061575f565b60405180910390f35b34801561042f575f80fd5b5061044a60048036038101906104459190615778565b6118fb565b005b348015610457575f80fd5b50610472600480360381019061046d91906157d5565b611c2f565b005b34801561047f575f80fd5b5061049a60048036038101906104959190615832565b611d71565b005b3480156104a7575f80fd5b506104c260048036038101906104bd9190615918565b612167565b005b3480156104cf575f80fd5b506104d861246b565b6040516104eb9796959493929190615abb565b60405180910390f35b3480156104ff575f80fd5b5061051a600480360381019061051591906150f6565b612574565b604051610528929190615dcd565b60405180910390f35b34801561053c575f80fd5b506105456128da565b604051610552919061509b565b60405180910390f35b348015610566575f80fd5b5061056f612913565b60405161057c9190614ff8565b60405180910390f35b348015610590575f80fd5b506105ab60048036038101906105a691906150f6565b61292a565b005b3480156105b8575f80fd5b506105c1612b94565b005b3480156105ce575f80fd5b506105e960048036038101906105e491906150f6565b612cb9565b6040516105f7929190615e4a565b60405180910390f35b34801561060b575f80fd5b5061062660048036038101906106219190615e7f565b612f8a565b005b348015610633575f80fd5b5061063c613114565b6040516106499190614ff8565b60405180910390f35b34801561065d575f80fd5b50610678600480360381019061067391906150f6565b61312b565b6040516106859190614ff8565b60405180910390f35b5f8061069861314e565b9050806005015491505090565b60606040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506106e65f613175565b6106f06002613175565b6106f95f613175565b60405160200161070c9493929190615f78565b604051602081830303815290604052905090565b60605f61072b61314e565b90505f816003015f8581526020019081526020015f20549050816002015f8581526020019081526020015f205f8281526020019081526020015f208054806020026020016040519081016040528092919081815260200182805480156107e357602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001906001019080831161079a575b505050505092505050919050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561084e573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108729190615fea565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146108e157336040517f21bfda100000000000000000000000000000000000000000000000000000000081526004016108d89190616015565b60405180910390fd5b5f6108ea61314e565b90508060090154821180610906575060f8600560ff16901b8211155b1561094857816040517fcbe9265600000000000000000000000000000000000000000000000000000000815260040161093f9190614ff8565b60405180910390fd5b806001015f8381526020019081526020015f205f9054906101000a900460ff16156109aa57816040517fdf0db5fb0000000000000000000000000000000000000000000000000000000081526004016109a19190614ff8565b60405180910390fd5b6001816001015f8481526020019081526020015f205f6101000a81548160ff0219169083151502179055507f384f90fefbcfaa68f22e00094aeaa52b2bc693936d2ce1afed1212520b59b58e82604051610a049190614ff8565b60405180910390a15050565b5f80610a1a61314e565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff16610a7d57826040517f84de1331000000000000000000000000000000000000000000000000000000008152600401610a749190614ff8565b60405180910390fd5b5f801b816003015f8581526020019081526020015f205403610ad657826040517f83f18335000000000000000000000000000000000000000000000000000000008152600401610acd9190614ff8565b60405180910390fd5b5f816006015f8581526020019081526020015f2054905081600d015f8281526020019081526020015f205f9054906101000a900460ff1692505050919050565b6001610b2061323f565b67ffffffffffffffff1614610b61576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60035f610b6c613263565b9050805f0160089054906101000a900460ff1680610bb457508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15610beb576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff021916908315150217905550610ca46040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506040518060400160405280600181526020017f310000000000000000000000000000000000000000000000000000000000000081525061328a565b5f610cad61314e565b905060f8600360ff16901b816004018190555060f8600460ff16901b816005018190555060f8600560ff16901b8160090181905550505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051610d2d9190616050565b60405180910390a15050565b5f80610d4361314e565b9050806009015491505090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610dad573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610dd19190615fea565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610e4057336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401610e379190616015565b60405180910390fd5b5f610e4961314e565b90505f8160090154905060f8600560ff16901b8114158015610e885750816001015f8281526020019081526020015f205f9054906101000a900460ff16155b15610eca57806040517f061ac61d000000000000000000000000000000000000000000000000000000008152600401610ec19190614ff8565b60405180910390fd5b816009015f815480929190610ede90616096565b91905055505f826009015490508483600a015f8381526020019081526020015f20819055508383600d015f8381526020019081526020015f205f6101000a81548160ff02191690836001811115610f3857610f37615228565b5b02179055505f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa158015610f9b573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610fbf91906160f1565b90505f610fcb826132a0565b90508085600e015f8581526020019081526020015f209081610fed9190616316565b507f8cf0151393f84fd694c5e315cb74cc05b247de0a454fd9e9129c661efdf9401d8388888460405161102394939291906163e5565b60405180910390a150505050505050565b5f8061103e61314e565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff16915050919050565b5f8061107261314e565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff166110d557826040517fda32d00f0000000000000000000000000000000000000000000000000000000081526004016110cc9190614ff8565b60405180910390fd5b5f801b816003015f8581526020019081526020015f20540361112e57826040517fd5fd3cd70000000000000000000000000000000000000000000000000000000081526004016111259190614ff8565b60405180910390fd5b80600d015f8481526020019081526020015f205f9054906101000a900460ff16915050919050565b5f61115f61314e565b9050806005015486118061117b575060f8600460ff16901b8611155b156111bd57856040517fadfab9040000000000000000000000000000000000000000000000000000000081526004016111b49190614ff8565b60405180910390fd5b5f858590500361120457856040517fe6f9083b0000000000000000000000000000000000000000000000000000000081526004016111fb9190614ff8565b60405180910390fd5b5f8061120f886132cc565b915091505f836006015f8a81526020019081526020015f20549050836001015f8281526020019081526020015f205f9054906101000a900460ff16611280576040517f6fbcdd2b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f61128e828b8b8b88613456565b90505f61129d84838a8a613637565b9050855f015f8c81526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff161561133d578a816040517f98fb957d00000000000000000000000000000000000000000000000000000000815260040161133492919061642f565b60405180910390fd5b6001865f015f8d81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f866002015f8d81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507f2afe64fb3afde8e2678aea84cf36223f330e2fb1286d37aed573ab9cd1db47c78c8c8c8c8c3360405161145f96959493929190616662565b60405180910390a1866001015f8d81526020019081526020015f205f9054906101000a900460ff1615801561149e575061149d85828054905061369f565b5b15611659576001876001015f8e81526020019081526020015f205f6101000a81548160ff0219169083151502179055505f5b8b8b905081101561155457876007015f8e81526020019081526020015f208c8c83818110611501576115006166b7565b5b905060200281019061151391906166f0565b908060018154018082558091505060019003905f5260205f2090600202015f9091909190915081816115459190616923565b505080806001019150506114d0565b5082876003015f8e81526020019081526020015f20819055505f6115fa86838054806020026020016040519081016040528092919081815260200182805480156115f057602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116115a7575b505050505061373c565b90505f87511180156116325750600360ff16875f8151811061161f5761161e6166b7565b5b602001015160f81c60f81b60f81c60ff16145b1561164957611644888d8d848b613884565b611657565b611656888e8e8e85613996565b5b505b505050505050505050505050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156116c4573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116e89190615fea565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461175757336040517f21bfda1000000000000000000000000000000000000000000000000000000000815260040161174e9190616015565b60405180910390fd5b5f61176061314e565b9050806005015488118061177c575060f8600460ff16901b8811155b156117be57876040517fadfab9040000000000000000000000000000000000000000000000000000000081526004016117b59190614ff8565b60405180910390fd5b8484905087879050146117fd576040517f894b2ab300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b611806826139e3565b6001816010015f8a81526020019081526020015f20541461185e57876040517f05b083f20000000000000000000000000000000000000000000000000000000081526004016118559190614ff8565b60405180910390fd5b7f8bfa7d0ed6f87e526b62342918ee7bfa53952badd463dc934054d7dd940eafdc888888888888886040516118999796959493929190616999565b60405180910390a15050505050505050565b6118b3613a2b565b6118bc82613b11565b6118c68282613c04565b5050565b5f6118d3613d22565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b5f61190461314e565b90508060040154841180611920575060f8600360ff16901b8411155b1561196257836040517f0ab7f6870000000000000000000000000000000000000000000000000000000081526004016119599190614ff8565b60405180910390fd5b5f8061196d866132cc565b915091505f61197c8784613da9565b90505f61198b83838989613637565b9050845f015f8981526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615611a2b5787816040517f33ca1fe3000000000000000000000000000000000000000000000000000000008152600401611a2292919061642f565b60405180910390fd5b6001855f015f8a81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f856002015f8a81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507f4c715c5734ce5c18c9c12e8496e53d2a65f1ec381d476957f0f596b364a59b0c89898933604051611b4994939291906169fc565b60405180910390a1856001015f8a81526020019081526020015f205f9054906101000a900460ff16158015611b885750611b8784828054905061369f565b5b15611c24576001866001015f8b81526020019081526020015f205f6101000a81548160ff02191690831515021790555082866003015f8b81526020019081526020015f20819055505f866006015f8b81526020019081526020015f205490507f3a116120cca5d4f073cc1fc31ff26133ab7b0499f2712fa010023b87d5a1f9ee8a8288604051611c1a93929190616a3a565b60405180910390a1505b505050505050505050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611c8c573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611cb09190615fea565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614611d1f57336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401611d169190616015565b60405180910390fd5b611d6c8383838080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050613e0b565b505050565b5f611d7a61314e565b90508060090154861180611d96575060f8600560ff16901b8611155b15611dd857856040517f8d8c940a000000000000000000000000000000000000000000000000000000008152600401611dcf9190614ff8565b60405180910390fd5b5f80611de3886132cc565b915091505f611e088985600a015f8c81526020019081526020015f20548a8a87613fc6565b90505f611e1783838989613637565b9050845f015f8b81526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615611eb75789816040517ffcf5a6e9000000000000000000000000000000000000000000000000000000008152600401611eae92919061642f565b60405180910390fd5b6001855f015f8c81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f856002015f8c81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507f7bf1b42c10e9497c879620c5b7afced10bda17d8c90b22f0e3bc6b2fd6ced0bd8b8b8b8b8b33604051611fd996959493929190616a76565b60405180910390a1856001015f8c81526020019081526020015f205f9054906101000a900460ff16158015612018575061201784828054905061369f565b5b1561215a576001866001015f8d81526020019081526020015f205f6101000a81548160ff021916908315150217905550898987600b015f8e81526020019081526020015f20918261206a929190616802565b5082866003015f8d81526020019081526020015f20819055508a86600c01819055505f612119858380548060200260200160405190810160405280929190818152602001828054801561210f57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116120c6575b505050505061373c565b90507f2258b73faed33fb2e2ea454403bef974920caf682ab3a723484fcf67553b16a28c828d8d6040516121509493929190616acb565b60405180910390a1505b5050505050505050505050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156121c4573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906121e89190615fea565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461225757336040517f21bfda1000000000000000000000000000000000000000000000000000000000815260040161224e9190616015565b60405180910390fd5b5f61226061314e565b9050806005015487118061227c575060f8600460ff16901b8711155b156122be57866040517fadfab9040000000000000000000000000000000000000000000000000000000081526004016122b59190614ff8565b60405180910390fd5b806001015f8881526020019081526020015f205f9054906101000a900460ff16612314576040517f6fbcdd2b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f868690500361235b57866040517fe6f9083b0000000000000000000000000000000000000000000000000000000081526004016123529190614ff8565b60405180910390fd5b612364826139e3565b80600f015f8881526020019081526020015f205f6123829190614f2f565b5f5b868690508110156124085781600f015f8981526020019081526020015f208787838181106123b5576123b46166b7565b5b90506020028101906123c791906166f0565b908060018154018082558091505060019003905f5260205f2090600202015f9091909190915081816123f99190616923565b50508080600101915050612384565b5081816010015f8981526020019081526020015f20819055507fa47664861ab58c5bd5040e9cc45e68d0e48ec04371035fd75099e217e0a6aa8187858589898760405161245a96959493929190616c3c565b60405180910390a150505050505050565b5f6060805f805f60605f61247d614057565b90505f801b815f015414801561249857505f801b8160010154145b6124d7576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016124ce90616cdb565b60405180910390fd5b6124df61407e565b6124e761411c565b46305f801b5f67ffffffffffffffff811115612506576125056155c9565b5b6040519080825280602002602001820160405280156125345781602001602082028036833780820191505090505b507f0f0000000000000000000000000000000000000000000000000000000000000095949392919097509750975097509750975097505090919293949596565b6060805f61258061314e565b9050806001015f8581526020019081526020015f205f9054906101000a900460ff166125e357836040517f84de13310000000000000000000000000000000000000000000000000000000081526004016125da9190614ff8565b60405180910390fd5b5f816003015f8681526020019081526020015f205490505f801b810361264057846040517f83f183350000000000000000000000000000000000000000000000000000000081526004016126379190614ff8565b60405180910390fd5b5f826002015f8781526020019081526020015f205f8381526020019081526020015f208054806020026020016040519081016040528092919081815260200182805480156126e057602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311612697575b505050505090505f61278a84600e015f8981526020019081526020015f20805461270990616149565b80601f016020809104026020016040519081016040528092919081815260200182805461273590616149565b80156127805780601f1061275757610100808354040283529160200191612780565b820191905f5260205f20905b81548152906001019060200180831161276357829003601f168201915b50505050506141ba565b90505f612797828461373c565b905080856007015f8a81526020019081526020015f2080805480602002602001604051908101604052809291908181526020015f905b828210156128c6578382905f5260205f2090600202016040518060400160405290815f82015f9054906101000a900460ff16600181111561281157612810615228565b5b600181111561282357612822615228565b5b815260200160018201805461283790616149565b80601f016020809104026020016040519081016040528092919081815260200182805461286390616149565b80156128ae5780601f10612885576101008083540402835291602001916128ae565b820191905f5260205f20905b81548152906001019060200180831161289157829003601f168201915b505050505081525050815260200190600101906127cd565b505050509050965096505050505050915091565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b5f8061291d61314e565b905080600c015491505090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612987573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129ab9190615fea565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614612a1a57336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401612a119190616015565b60405180910390fd5b5f612a2361314e565b90508060040154821180612a3f575060f8600360ff16901b8211155b15612a8157816040517ffcf2db7a000000000000000000000000000000000000000000000000000000008152600401612a789190614ff8565b60405180910390fd5b5f816006015f8481526020019081526020015f20549050816001015f8281526020019081526020015f205f9054906101000a900460ff1615612afa57826040517f92789b67000000000000000000000000000000000000000000000000000000008152600401612af19190614ff8565b60405180910390fd5b6001826001015f8581526020019081526020015f205f6101000a81548160ff0219169083151502179055505f8114612b58576001826001015f8381526020019081526020015f205f6101000a81548160ff0219169083151502179055505b7f2b087b884b35a81d769d1a1e092880f1da56de964e4b339eabcb1f45f5fe326483604051612b879190614ff8565b60405180910390a1505050565b60035f612b9f613263565b9050805f0160089054906101000a900460ff1680612be757508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15612c1e576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051612cad9190616050565b60405180910390a15050565b6060805f612cc561314e565b9050806001015f8581526020019081526020015f205f9054906101000a900460ff16612d2857836040517fda32d00f000000000000000000000000000000000000000000000000000000008152600401612d1f9190614ff8565b60405180910390fd5b5f816003015f8681526020019081526020015f205490505f801b8103612d8557846040517fd5fd3cd7000000000000000000000000000000000000000000000000000000008152600401612d7c9190614ff8565b60405180910390fd5b5f826002015f8781526020019081526020015f205f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020018280548015612e2557602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311612ddc575b505050505090505f612ecf84600e015f8981526020019081526020015f208054612e4e90616149565b80601f0160208091040260200160405190810160405280929190818152602001828054612e7a90616149565b8015612ec55780601f10612e9c57610100808354040283529160200191612ec5565b820191905f5260205f20905b815481529060010190602001808311612ea857829003601f168201915b50505050506141ba565b90505f612edc828461373c565b90508085600b015f8a81526020019081526020015f20808054612efe90616149565b80601f0160208091040260200160405190810160405280929190818152602001828054612f2a90616149565b8015612f755780601f10612f4c57610100808354040283529160200191612f75565b820191905f5260205f20905b815481529060010190602001808311612f5857829003601f168201915b50505050509050965096505050505050915091565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612fe7573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061300b9190615fea565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461307a57336040517f21bfda100000000000000000000000000000000000000000000000000000000081526004016130719190616015565b60405180910390fd5b5f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa1580156130d8573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906130fc91906160f1565b90506131108261310b836132a0565b613e0b565b5050565b5f8061311e61314e565b9050806008015491505090565b5f61313461314e565b6010015f8381526020019081526020015f20549050919050565b5f7f26fdaf8a2cb20d20b55e36218986905e534ee7a970dd2fa827946e4b7496db00905090565b60605f600161318384614362565b0190505f8167ffffffffffffffff8111156131a1576131a06155c9565b5b6040519080825280601f01601f1916602001820160405280156131d35781602001600182028036833780820191505090505b5090505f82602001820190505b600115613234578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a858161322957613228616cf9565b5b0494505f85036131e0575b819350505050919050565b5f613248613263565b5f015f9054906101000a900467ffffffffffffffff16905090565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b6132926144b3565b61329c82826144f3565b5050565b60606001826040516020016132b6929190616d86565b6040516020818303038152906040529050919050565b60605f806132d861314e565b905080600e015f8581526020019081526020015f2080546132f890616149565b80601f016020809104026020016040519081016040528092919081815260200182805461332490616149565b801561336f5780601f106133465761010080835404028352916020019161336f565b820191905f5260205f20905b81548152906001019060200180831161335257829003601f168201915b5050505050925061337f836141ba565b91507344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166346c5bbbd83336040518363ffffffff1660e01b81526004016133d092919061642f565b602060405180830381865afa1580156133eb573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061340f9190616ddb565b61345057336040517faee863230000000000000000000000000000000000000000000000000000000081526004016134479190616015565b60405180910390fd5b50915091565b5f808484905067ffffffffffffffff811115613475576134746155c9565b5b6040519080825280602002602001820160405280156134a35781602001602082028036833780820191505090505b5090505f5b858590508110156135a75760405180606001604052806025815260200161763060259139805190602001208686838181106134e6576134e56166b7565b5b90506020028101906134f891906166f0565b5f0160208101906135099190616e06565b87878481811061351c5761351b6166b7565b5b905060200281019061352e91906166f0565b806020019061353d9190616796565b60405161354b929190616e5f565b604051809103902060405160200161356593929190616e86565b6040516020818303038152906040528051906020012082828151811061358e5761358d6166b7565b5b60200260200101818152505080806001019150506134a8565b5061362b6040518060c00160405280608281526020016175ae60829139805190602001208888846040516020016135de9190616f6c565b604051602081830303815290604052805190602001208780519060200120604051602001613610959493929190616f82565b60405160208183030381529060405280519060200120614544565b91505095945050505050565b5f806136868585858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505061455d565b9050613693868233614587565b80915050949350505050565b5f807344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166341ad069c856040518263ffffffff1660e01b81526004016136ee9190614ff8565b602060405180830381865afa158015613709573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061372d91906160f1565b90508083101591505092915050565b60605f825190505f8167ffffffffffffffff81111561375e5761375d6155c9565b5b60405190808252806020026020018201604052801561379157816020015b606081526020019060019003908161377c5790505b5090505f5b82811015613878577344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166331ff41c8878784815181106137e2576137e16166b7565b5b60200260200101516040518363ffffffff1660e01b815260040161380792919061642f565b5f60405180830381865afa158015613821573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f820116820180604052508101906138499190617126565b606001518282815181106138605761385f6166b7565b5b60200260200101819052508080600101915050613796565b50809250505092915050565b5f61388e82614768565b905085600f015f8281526020019081526020015f205f6138ae9190614f2f565b5f5b858590508110156139345786600f015f8381526020019081526020015f208686838181106138e1576138e06166b7565b5b90506020028101906138f391906166f0565b908060018154018082558091505060019003905f5260205f2090600202015f9091909190915081816139259190616923565b505080806001019150506138b0565b506001866010015f8381526020019081526020015f20819055507fa47664861ab58c5bd5040e9cc45e68d0e48ec04371035fd75099e217e0a6aa8181848787600160405161398695949392919061716d565b60405180910390a1505050505050565b8385600801819055507feb85c26dbcad46b80a68a0f24cce7c2c90f0a1faded84184138839fc9e80a25b848285856040516139d494939291906171c0565b60405180910390a15050505050565b60018114613a2857806040517fa3e9ed7e000000000000000000000000000000000000000000000000000000008152600401613a1f9190614ff8565b60405180910390fd5b50565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff161480613ad857507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16613abf6147b1565b73ffffffffffffffffffffffffffffffffffffffff1614155b15613b0f576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613b6e573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613b929190615fea565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614613c0157336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401613bf89190616015565b60405180910390fd5b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa925050508015613c6c57506040513d601f19601f82011682018060405250810190613c69919061722f565b60015b613cad57816040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401613ca49190616015565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b8114613d1357806040517faa1d49a4000000000000000000000000000000000000000000000000000000008152600401613d0a919061575f565b60405180910390fd5b613d1d8383614804565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff1614613da7576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f613e036040518060600160405280603c815260200161751c603c913980519060200120848480519060200120604051602001613de89392919061725a565b60405160208183030381529060405280519060200120614544565b905092915050565b5f613e1461314e565b90505f8160050154905060f8600460ff16901b8114158015613e535750816001015f8281526020019081526020015f205f9054906101000a900460ff16155b15613e9557806040517f3b853da8000000000000000000000000000000000000000000000000000000008152600401613e8c9190614ff8565b60405180910390fd5b816004015f815480929190613ea990616096565b91905055505f82600401549050826005015f815480929190613eca90616096565b91905055505f8360050154905080846006015f8481526020019081526020015f208190555081846006015f8381526020019081526020015f20819055508584600d015f8481526020019081526020015f205f6101000a81548160ff02191690836001811115613f3c57613f3b615228565b5b02179055508484600e015f8481526020019081526020015f209081613f619190616316565b508484600e015f8381526020019081526020015f209081613f829190616316565b507ffbf5274810b94f86970c1147e8ffaebed246ee9777d695a69004dc6256d1fe91828787604051613fb69392919061728f565b60405180910390a1505050505050565b5f61404c604051806080016040528060568152602001617558605691398051906020012087878787604051602001613fff929190616e5f565b604051602081830303815290604052805190602001208680519060200120604051602001614031959493929190616f82565b60405160208183030381529060405280519060200120614544565b905095945050505050565b5f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100905090565b60605f614089614057565b905080600201805461409a90616149565b80601f01602080910402602001604051908101604052809291908181526020018280546140c690616149565b80156141115780601f106140e857610100808354040283529160200191614111565b820191905f5260205f20905b8154815290600101906020018083116140f457829003601f168201915b505050505091505090565b60605f614127614057565b905080600301805461413890616149565b80601f016020809104026020016040519081016040528092919081815260200182805461416490616149565b80156141af5780601f10614186576101008083540402835291602001916141af565b820191905f5260205f20905b81548152906001019060200180831161419257829003601f168201915b505050505091505090565b5f80825114806141ec57505f825f815181106141d9576141d86166b7565b5b602001015160f81c60f81b60f81c60ff16145b15614279577344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa15801561424e573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061427291906160f1565b905061435d565b5f825f8151811061428d5761428c6166b7565b5b602001015160f81c60f81b60f81c9050600160ff168160ff16141580156142bb5750600360ff168160ff1614155b156142fd57806040517f2139cc2c0000000000000000000000000000000000000000000000000000000081526004016142f491906172da565b60405180910390fd5b5f600360ff168260ff1614614313576021614316565b60425b90508084511015614353576040517f8b248b6000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6021840151925050505b919050565b5f805f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f01000000000000000083106143be577a184f03e93ff9f4daa797ed6e38ed64bf6a1f01000000000000000083816143b4576143b3616cf9565b5b0492506040810190505b6d04ee2d6d415b85acef810000000083106143fb576d04ee2d6d415b85acef810000000083816143f1576143f0616cf9565b5b0492506020810190505b662386f26fc10000831061442a57662386f26fc1000083816144205761441f616cf9565b5b0492506010810190505b6305f5e1008310614453576305f5e100838161444957614448616cf9565b5b0492506008810190505b612710831061447857612710838161446e5761446d616cf9565b5b0492506004810190505b6064831061449b576064838161449157614490616cf9565b5b0492506002810190505b600a83106144aa576001810190505b80915050919050565b6144bb614876565b6144f1576040517fd7e6bcf800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b6144fb6144b3565b5f614504614057565b905082816002019081614517919061734b565b5081816003019081614529919061734b565b505f801b815f01819055505f801b8160010181905550505050565b5f614556614550614894565b836148a2565b9050919050565b5f805f8061456b86866148e2565b92509250925061457b8282614937565b82935050505092915050565b7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff16639447cfd484846040518363ffffffff1660e01b81526004016145d692919061642f565b602060405180830381865afa1580156145f1573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906146159190616ddb565b6146585781816040517f0d86f52100000000000000000000000000000000000000000000000000000000815260040161464f92919061741a565b60405180910390fd5b5f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166331ff41c885846040518363ffffffff1660e01b81526004016146a892919061642f565b5f60405180830381865afa1580156146c2573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f820116820180604052508101906146ea9190617126565b90508273ffffffffffffffffffffffffffffffffffffffff16816020015173ffffffffffffffffffffffffffffffffffffffff16146147625782826040517f0d86f52100000000000000000000000000000000000000000000000000000000815260040161475992919061741a565b60405180910390fd5b50505050565b5f6042825110156147a5576040517f8b248b6000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60418201519050919050565b5f6147dd7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b614a99565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b61480d82614aa2565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f81511115614869576148638282614b6b565b50614872565b614871614beb565b5b5050565b5f61487f613263565b5f0160089054906101000a900460ff16905090565b5f61489d614c27565b905090565b5f6040517f190100000000000000000000000000000000000000000000000000000000000081528360028201528260228201526042812091505092915050565b5f805f6041845103614922575f805f602087015192506040870151915060608701515f1a905061491488828585614c8a565b955095509550505050614930565b5f600285515f1b9250925092505b9250925092565b5f600381111561494a57614949615228565b5b82600381111561495d5761495c615228565b5b0315614a95576001600381111561497757614976615228565b5b82600381111561498a57614989615228565b5b036149c1576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600260038111156149d5576149d4615228565b5b8260038111156149e8576149e7615228565b5b03614a2c57805f1c6040517ffce698f7000000000000000000000000000000000000000000000000000000008152600401614a239190614ff8565b60405180910390fd5b600380811115614a3f57614a3e615228565b5b826003811115614a5257614a51615228565b5b03614a9457806040517fd78bce0c000000000000000000000000000000000000000000000000000000008152600401614a8b919061575f565b60405180910390fd5b5b5050565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b03614afd57806040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401614af49190616015565b60405180910390fd5b80614b297f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b614a99565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f808473ffffffffffffffffffffffffffffffffffffffff1684604051614b949190617471565b5f60405180830381855af49150503d805f8114614bcc576040519150601f19603f3d011682016040523d82523d5f602084013e614bd1565b606091505b5091509150614be1858383614d71565b9250505092915050565b5f341115614c25576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f614c51614dfe565b614c59614e74565b4630604051602001614c6f959493929190617487565b60405160208183030381529060405280519060200120905090565b5f805f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c1115614cc6575f600385925092509250614d67565b5f6001888888886040515f8152602001604052604051614ce994939291906174d8565b6020604051602081039080840390855afa158015614d09573d5f803e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603614d5a575f60015f801b93509350935050614d67565b805f805f1b935093509350505b9450945094915050565b606082614d8657614d8182614eeb565b614df6565b5f8251148015614dac57505f8473ffffffffffffffffffffffffffffffffffffffff163b145b15614dee57836040517f9996b315000000000000000000000000000000000000000000000000000000008152600401614de59190616015565b60405180910390fd5b819050614df7565b5b9392505050565b5f80614e08614057565b90505f614e1361407e565b90505f81511115614e2f57808051906020012092505050614e71565b5f825f015490505f801b8114614e4a57809350505050614e71565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f80614e7e614057565b90505f614e8961411c565b90505f81511115614ea557808051906020012092505050614ee8565b5f826001015490505f801b8114614ec157809350505050614ee8565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f81511115614efd5780518082602001fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5080545f8255600202905f5260205f2090810190614f4d9190614f50565b50565b5b80821115614f84575f8082015f6101000a81549060ff0219169055600182015f614f7b9190614f88565b50600201614f51565b5090565b508054614f9490616149565b5f825580601f10614fa55750614fc2565b601f0160209004905f5260205f2090810190614fc19190614fc5565b5b50565b5b80821115614fdc575f815f905550600101614fc6565b5090565b5f819050919050565b614ff281614fe0565b82525050565b5f60208201905061500b5f830184614fe9565b92915050565b5f81519050919050565b5f82825260208201905092915050565b5f5b8381101561504857808201518184015260208101905061502d565b5f8484015250505050565b5f601f19601f8301169050919050565b5f61506d82615011565b615077818561501b565b935061508781856020860161502b565b61509081615053565b840191505092915050565b5f6020820190508181035f8301526150b38184615063565b905092915050565b5f604051905090565b5f80fd5b5f80fd5b6150d581614fe0565b81146150df575f80fd5b50565b5f813590506150f0816150cc565b92915050565b5f6020828403121561510b5761510a6150c4565b5b5f615118848285016150e2565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6151738261514a565b9050919050565b61518381615169565b82525050565b5f615194838361517a565b60208301905092915050565b5f602082019050919050565b5f6151b682615121565b6151c0818561512b565b93506151cb8361513b565b805f5b838110156151fb5781516151e28882615189565b97506151ed836151a0565b9250506001810190506151ce565b5085935050505092915050565b5f6020820190508181035f83015261522081846151ac565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b6002811061526657615265615228565b5b50565b5f81905061527682615255565b919050565b5f61528582615269565b9050919050565b6152958161527b565b82525050565b5f6020820190506152ae5f83018461528c565b92915050565b600281106152c0575f80fd5b50565b5f813590506152d1816152b4565b92915050565b5f80604083850312156152ed576152ec6150c4565b5b5f6152fa858286016150e2565b925050602061530b858286016152c3565b9150509250929050565b5f8115159050919050565b61532981615315565b82525050565b5f6020820190506153425f830184615320565b92915050565b5f80fd5b5f80fd5b5f80fd5b5f8083601f84011261536957615368615348565b5b8235905067ffffffffffffffff8111156153865761538561534c565b5b6020830191508360208202830111156153a2576153a1615350565b5b9250929050565b5f8083601f8401126153be576153bd615348565b5b8235905067ffffffffffffffff8111156153db576153da61534c565b5b6020830191508360018202830111156153f7576153f6615350565b5b9250929050565b5f805f805f60608688031215615417576154166150c4565b5b5f615424888289016150e2565b955050602086013567ffffffffffffffff811115615445576154446150c8565b5b61545188828901615354565b9450945050604086013567ffffffffffffffff811115615474576154736150c8565b5b615480888289016153a9565b92509250509295509295909350565b5f8083601f8401126154a4576154a3615348565b5b8235905067ffffffffffffffff8111156154c1576154c061534c565b5b6020830191508360208202830111156154dd576154dc615350565b5b9250929050565b5f805f805f805f60a0888a0312156154ff576154fe6150c4565b5b5f61550c8a828b016150e2565b975050602088013567ffffffffffffffff81111561552d5761552c6150c8565b5b6155398a828b0161548f565b9650965050604088013567ffffffffffffffff81111561555c5761555b6150c8565b5b6155688a828b0161548f565b9450945050606061557b8a828b016150e2565b925050608061558c8a828b016150e2565b91505092959891949750929550565b6155a481615169565b81146155ae575f80fd5b50565b5f813590506155bf8161559b565b92915050565b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6155ff82615053565b810181811067ffffffffffffffff8211171561561e5761561d6155c9565b5b80604052505050565b5f6156306150bb565b905061563c82826155f6565b919050565b5f67ffffffffffffffff82111561565b5761565a6155c9565b5b61566482615053565b9050602081019050919050565b828183375f83830152505050565b5f61569161568c84615641565b615627565b9050828152602081018484840111156156ad576156ac6155c5565b5b6156b8848285615671565b509392505050565b5f82601f8301126156d4576156d3615348565b5b81356156e484826020860161567f565b91505092915050565b5f8060408385031215615703576157026150c4565b5b5f615710858286016155b1565b925050602083013567ffffffffffffffff811115615731576157306150c8565b5b61573d858286016156c0565b9150509250929050565b5f819050919050565b61575981615747565b82525050565b5f6020820190506157725f830184615750565b92915050565b5f805f6040848603121561578f5761578e6150c4565b5b5f61579c868287016150e2565b935050602084013567ffffffffffffffff8111156157bd576157bc6150c8565b5b6157c9868287016153a9565b92509250509250925092565b5f805f604084860312156157ec576157eb6150c4565b5b5f6157f9868287016152c3565b935050602084013567ffffffffffffffff81111561581a576158196150c8565b5b615826868287016153a9565b92509250509250925092565b5f805f805f6060868803121561584b5761584a6150c4565b5b5f615858888289016150e2565b955050602086013567ffffffffffffffff811115615879576158786150c8565b5b615885888289016153a9565b9450945050604086013567ffffffffffffffff8111156158a8576158a76150c8565b5b6158b4888289016153a9565b92509250509295509295909350565b5f8083601f8401126158d8576158d7615348565b5b8235905067ffffffffffffffff8111156158f5576158f461534c565b5b60208301915083602082028301111561591157615910615350565b5b9250929050565b5f805f805f8060808789031215615932576159316150c4565b5b5f61593f89828a016150e2565b965050602087013567ffffffffffffffff8111156159605761595f6150c8565b5b61596c89828a01615354565b9550955050604087013567ffffffffffffffff81111561598f5761598e6150c8565b5b61599b89828a016158c3565b935093505060606159ae89828a016150e2565b9150509295509295509295565b5f7fff0000000000000000000000000000000000000000000000000000000000000082169050919050565b6159ef816159bb565b82525050565b6159fe81615169565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b615a3681614fe0565b82525050565b5f615a478383615a2d565b60208301905092915050565b5f602082019050919050565b5f615a6982615a04565b615a738185615a0e565b9350615a7e83615a1e565b805f5b83811015615aae578151615a958882615a3c565b9750615aa083615a53565b925050600181019050615a81565b5085935050505092915050565b5f60e082019050615ace5f83018a6159e6565b8181036020830152615ae08189615063565b90508181036040830152615af48188615063565b9050615b036060830187614fe9565b615b1060808301866159f5565b615b1d60a0830185615750565b81810360c0830152615b2f8184615a5f565b905098975050505050505050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f82825260208201905092915050565b5f615b8082615011565b615b8a8185615b66565b9350615b9a81856020860161502b565b615ba381615053565b840191505092915050565b5f615bb98383615b76565b905092915050565b5f602082019050919050565b5f615bd782615b3d565b615be18185615b47565b935083602082028501615bf385615b57565b805f5b85811015615c2e5784840389528151615c0f8582615bae565b9450615c1a83615bc1565b925060208a01995050600181019050615bf6565b50829750879550505050505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b60028110615c7a57615c79615228565b5b50565b5f819050615c8a82615c69565b919050565b5f615c9982615c7d565b9050919050565b615ca981615c8f565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f615cd382615caf565b615cdd8185615cb9565b9350615ced81856020860161502b565b615cf681615053565b840191505092915050565b5f604083015f830151615d165f860182615ca0565b5060208301518482036020860152615d2e8282615cc9565b9150508091505092915050565b5f615d468383615d01565b905092915050565b5f602082019050919050565b5f615d6482615c40565b615d6e8185615c4a565b935083602082028501615d8085615c5a565b805f5b85811015615dbb5784840389528151615d9c8582615d3b565b9450615da783615d4e565b925060208a01995050600181019050615d83565b50829750879550505050505092915050565b5f6040820190508181035f830152615de58185615bcd565b90508181036020830152615df98184615d5a565b90509392505050565b5f82825260208201905092915050565b5f615e1c82615caf565b615e268185615e02565b9350615e3681856020860161502b565b615e3f81615053565b840191505092915050565b5f6040820190508181035f830152615e628185615bcd565b90508181036020830152615e768184615e12565b90509392505050565b5f60208284031215615e9457615e936150c4565b5b5f615ea1848285016152c3565b91505092915050565b5f81905092915050565b5f615ebe82615011565b615ec88185615eaa565b9350615ed881856020860161502b565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f615f18600283615eaa565b9150615f2382615ee4565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f615f62600183615eaa565b9150615f6d82615f2e565b600182019050919050565b5f615f838287615eb4565b9150615f8e82615f0c565b9150615f9a8286615eb4565b9150615fa582615f56565b9150615fb18285615eb4565b9150615fbc82615f56565b9150615fc88284615eb4565b915081905095945050505050565b5f81519050615fe48161559b565b92915050565b5f60208284031215615fff57615ffe6150c4565b5b5f61600c84828501615fd6565b91505092915050565b5f6020820190506160285f8301846159f5565b92915050565b5f67ffffffffffffffff82169050919050565b61604a8161602e565b82525050565b5f6020820190506160635f830184616041565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f6160a082614fe0565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82036160d2576160d1616069565b5b600182019050919050565b5f815190506160eb816150cc565b92915050565b5f60208284031215616106576161056150c4565b5b5f616113848285016160dd565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f600282049050600182168061616057607f821691505b6020821081036161735761617261611c565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f600883026161d57fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8261619a565b6161df868361619a565b95508019841693508086168417925050509392505050565b5f819050919050565b5f61621a61621561621084614fe0565b6161f7565b614fe0565b9050919050565b5f819050919050565b61623383616200565b61624761623f82616221565b8484546161a6565b825550505050565b5f90565b61625b61624f565b61626681848461622a565b505050565b5b818110156162895761627e5f82616253565b60018101905061626c565b5050565b601f8211156162ce5761629f81616179565b6162a88461618b565b810160208510156162b7578190505b6162cb6162c38561618b565b83018261626b565b50505b505050565b5f82821c905092915050565b5f6162ee5f19846008026162d3565b1980831691505092915050565b5f61630683836162df565b9150826002028217905092915050565b61631f82615caf565b67ffffffffffffffff811115616338576163376155c9565b5b6163428254616149565b61634d82828561628d565b5f60209050601f83116001811461637e575f841561636c578287015190505b61637685826162fb565b8655506163dd565b601f19841661638c86616179565b5f5b828110156163b35784890151825560018201915060208501945060208101905061638e565b868310156163d057848901516163cc601f8916826162df565b8355505b6001600288020188555050505b505050505050565b5f6080820190506163f85f830187614fe9565b6164056020830186614fe9565b616412604083018561528c565b81810360608301526164248184615e12565b905095945050505050565b5f6040820190506164425f830185614fe9565b61644f60208301846159f5565b9392505050565b5f819050919050565b6002811061646b575f80fd5b50565b5f8135905061647c8161645f565b92915050565b5f616490602084018461646e565b905092915050565b5f80fd5b5f80fd5b5f80fd5b5f80833560016020038436030381126164c0576164bf6164a0565b5b83810192508235915060208301925067ffffffffffffffff8211156164e8576164e7616498565b5b6001820236038313156164fe576164fd61649c565b5b509250929050565b5f6165118385615cb9565b935061651e838584615671565b61652783615053565b840190509392505050565b5f604083016165435f840184616482565b61654f5f860182615ca0565b5061655d60208401846164a4565b8583036020870152616570838284616506565b925050508091505092915050565b5f6165898383616532565b905092915050565b5f823560016040038336030381126165ac576165ab6164a0565b5b82810191505092915050565b5f602082019050919050565b5f6165cf8385615c4a565b9350836020840285016165e184616456565b805f5b878110156166245784840389526165fb8284616591565b616605858261657e565b9450616610836165b8565b925060208a019950506001810190506165e4565b50829750879450505050509392505050565b5f6166418385615e02565b935061664e838584615671565b61665783615053565b840190509392505050565b5f6080820190506166755f830189614fe9565b81810360208301526166888187896165c4565b9050818103604083015261669d818587616636565b90506166ac60608301846159f5565b979650505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b5f8235600160400383360303811261670b5761670a6166e4565b5b80830191505092915050565b5f81356167238161645f565b80915050919050565b5f815f1b9050919050565b5f60ff6167438461672c565b9350801983169250808416831791505092915050565b5f61676382615c7d565b9050919050565b5f819050919050565b61677c82616759565b61678f6167888261676a565b8354616737565b8255505050565b5f80833560016020038436030381126167b2576167b16166e4565b5b80840192508235915067ffffffffffffffff8211156167d4576167d36166e8565b5b6020830192506001820236038313156167f0576167ef6166ec565b5b509250929050565b5f82905092915050565b61680c83836167f8565b67ffffffffffffffff811115616825576168246155c9565b5b61682f8254616149565b61683a82828561628d565b5f601f831160018114616867575f8415616855578287013590505b61685f85826162fb565b8655506168c6565b601f19841661687586616179565b5f5b8281101561689c57848901358255600182019150602085019450602081019050616877565b868310156168b957848901356168b5601f8916826162df565b8355505b6001600288020188555050505b50505050505050565b6168da838383616802565b505050565b5f81015f8301806168ef81616717565b90506168fb8184616773565b50505060018101602083016169108185616796565b61691b8183866168cf565b505050505050565b61692d82826168df565b5050565b5f80fd5b82818337505050565b5f6169498385615a0e565b93507f07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff83111561697c5761697b616931565b5b60208302925061698d838584616935565b82840190509392505050565b5f60a0820190506169ac5f83018a614fe9565b81810360208301526169bf81888a61693e565b905081810360408301526169d481868861693e565b90506169e36060830185614fe9565b6169f06080830184614fe9565b98975050505050505050565b5f606082019050616a0f5f830187614fe9565b8181036020830152616a22818587616636565b9050616a3160408301846159f5565b95945050505050565b5f606082019050616a4d5f830186614fe9565b616a5a6020830185614fe9565b8181036040830152616a6c8184615e12565b9050949350505050565b5f608082019050616a895f830189614fe9565b8181036020830152616a9c818789616636565b90508181036040830152616ab1818587616636565b9050616ac060608301846159f5565b979650505050505050565b5f606082019050616ade5f830187614fe9565b8181036020830152616af08186615bcd565b90508181036040830152616b05818486616636565b905095945050505050565b5f819050919050565b5f616b248385615b66565b9350616b31838584615671565b616b3a83615053565b840190509392505050565b5f616b51848484616b19565b90509392505050565b5f8083356001602003843603038112616b7657616b756164a0565b5b83810192508235915060208301925067ffffffffffffffff821115616b9e57616b9d616498565b5b600182023603831315616bb457616bb361649c565b5b509250929050565b5f602082019050919050565b5f616bd38385615b47565b935083602084028501616be584616b10565b805f5b87811015616c2a578484038952616bff8284616b5a565b616c0a868284616b45565b9550616c1584616bbc565b935060208b019a505050600181019050616be8565b50829750879450505050509392505050565b5f608082019050616c4f5f830189614fe9565b8181036020830152616c62818789616bc8565b90508181036040830152616c778185876165c4565b9050616c866060830184614fe9565b979650505050505050565b7f4549503731323a20556e696e697469616c697a656400000000000000000000005f82015250565b5f616cc560158361501b565b9150616cd082616c91565b602082019050919050565b5f6020820190508181035f830152616cf281616cb9565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f60ff82169050919050565b5f8160f81b9050919050565b5f616d4882616d32565b9050919050565b616d60616d5b82616d26565b616d3e565b82525050565b5f819050919050565b616d80616d7b82614fe0565b616d66565b82525050565b5f616d918285616d4f565b600182019150616da18284616d6f565b6020820191508190509392505050565b616dba81615315565b8114616dc4575f80fd5b50565b5f81519050616dd581616db1565b92915050565b5f60208284031215616df057616def6150c4565b5b5f616dfd84828501616dc7565b91505092915050565b5f60208284031215616e1b57616e1a6150c4565b5b5f616e288482850161646e565b91505092915050565b5f81905092915050565b5f616e468385616e31565b9350616e53838584615671565b82840190509392505050565b5f616e6b828486616e3b565b91508190509392505050565b616e8081615c8f565b82525050565b5f606082019050616e995f830186615750565b616ea66020830185616e77565b616eb36040830184615750565b949350505050565b5f81519050919050565b5f81905092915050565b5f819050602082019050919050565b616ee781615747565b82525050565b5f616ef88383616ede565b60208301905092915050565b5f602082019050919050565b5f616f1a82616ebb565b616f248185616ec5565b9350616f2f83616ecf565b805f5b83811015616f5f578151616f468882616eed565b9750616f5183616f04565b925050600181019050616f32565b5085935050505092915050565b5f616f778284616f10565b915081905092915050565b5f60a082019050616f955f830188615750565b616fa26020830187614fe9565b616faf6040830186614fe9565b616fbc6060830185615750565b616fc96080830184615750565b9695505050505050565b5f80fd5b5f80fd5b5f67ffffffffffffffff821115616ff557616ff46155c9565b5b616ffe82615053565b9050602081019050919050565b5f61701d61701884616fdb565b615627565b905082815260208101848484011115617039576170386155c5565b5b61704484828561502b565b509392505050565b5f82601f8301126170605761705f615348565b5b815161707084826020860161700b565b91505092915050565b5f6080828403121561708e5761708d616fd3565b5b6170986080615627565b90505f6170a784828501615fd6565b5f8301525060206170ba84828501615fd6565b602083015250604082015167ffffffffffffffff8111156170de576170dd616fd7565b5b6170ea8482850161704c565b604083015250606082015167ffffffffffffffff81111561710e5761710d616fd7565b5b61711a8482850161704c565b60608301525092915050565b5f6020828403121561713b5761713a6150c4565b5b5f82015167ffffffffffffffff811115617158576171576150c8565b5b61716484828501617079565b91505092915050565b5f6080820190506171805f830188614fe9565b81810360208301526171928187615bcd565b905081810360408301526171a78185876165c4565b90506171b66060830184614fe9565b9695505050505050565b5f6060820190506171d35f830187614fe9565b81810360208301526171e58186615bcd565b905081810360408301526171fa8184866165c4565b905095945050505050565b61720e81615747565b8114617218575f80fd5b50565b5f8151905061722981617205565b92915050565b5f60208284031215617244576172436150c4565b5b5f6172518482850161721b565b91505092915050565b5f60608201905061726d5f830186615750565b61727a6020830185614fe9565b6172876040830184615750565b949350505050565b5f6060820190506172a25f830186614fe9565b6172af602083018561528c565b81810360408301526172c18184615e12565b9050949350505050565b6172d481616d26565b82525050565b5f6020820190506172ed5f8301846172cb565b92915050565b5f819050815f5260205f209050919050565b601f82111561734657617317816172f3565b6173208461618b565b8101602085101561732f578190505b61734361733b8561618b565b83018261626b565b50505b505050565b61735482615011565b67ffffffffffffffff81111561736d5761736c6155c9565b5b6173778254616149565b617382828285617305565b5f60209050601f8311600181146173b3575f84156173a1578287015190505b6173ab85826162fb565b865550617412565b601f1984166173c1866172f3565b5f5b828110156173e8578489015182556001820191506020850194506020810190506173c3565b868310156174055784890151617401601f8916826162df565b8355505b6001600288020188555050505b505050505050565b5f60408201905061742d5f8301856159f5565b61743a60208301846159f5565b9392505050565b5f61744b82615caf565b6174558185616e31565b935061746581856020860161502b565b80840191505092915050565b5f61747c8284617441565b915081905092915050565b5f60a08201905061749a5f830188615750565b6174a76020830187615750565b6174b46040830186615750565b6174c16060830185614fe9565b6174ce60808301846159f5565b9695505050505050565b5f6080820190506174eb5f830187615750565b6174f860208301866172cb565b6175056040830185615750565b6175126060830184615750565b9594505050505056fe507265704b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e49642c6279746573206578747261446174612943727367656e566572696669636174696f6e2875696e743235362063727349642c75696e74323536206d61784269744c656e6774682c6279746573206372734469676573742c627974657320657874726144617461294b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e49642c75696e74323536206b657949642c4b65794469676573745b5d206b6579446967657374732c627974657320657874726144617461294b65794469676573742875696e7438206b6579547970652c627974657320646967657374294b65794469676573742875696e7438206b6579547970652c62797465732064696765737429
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01\xB6W_5`\xE0\x1C\x80cX\x9A\xDB\x0E\x11a\0\xEBW\x80c\xBA\xFF!\x1E\x11a\0\x89W\x80c\xC5[\x87$\x11a\0cW\x80c\xC5[\x87$\x14a\x05\xC3W\x80c\xCA\xA3g\xDB\x14a\x06\0W\x80c\xD5/\x10\xEB\x14a\x06(W\x80c\xF0\xF8\xCB\xC6\x14a\x06RWa\x01\xB6V[\x80c\xBA\xFF!\x1E\x14a\x05[W\x80c\xC2\xC1\xFA\xEE\x14a\x05\x85W\x80c\xC4\x11Xt\x14a\x05\xADWa\x01\xB6V[\x80c\x7F\xC8\x0C]\x11a\0\xC5W\x80c\x7F\xC8\x0C]\x14a\x04\x9CW\x80c\x84\xB0\x19n\x14a\x04\xC4W\x80c\x93f\x08\xAE\x14a\x04\xF4W\x80c\xAD<\xB1\xCC\x14a\x051Wa\x01\xB6V[\x80cX\x9A\xDB\x0E\x14a\x04$W\x80c`\x8BJ\xD5\x14a\x04LW\x80cb\x97\x87\x87\x14a\x04tWa\x01\xB6V[\x80c<\x02\xF84\x11a\x01XW\x80cF\x10\xFF\xE8\x11a\x012W\x80cF\x10\xFF\xE8\x14a\x03\x8EW\x80cL)\x82)\x14a\x03\xB6W\x80cO\x1E\xF2\x86\x14a\x03\xDEW\x80cR\xD1\x90-\x14a\x03\xFAWa\x01\xB6V[\x80c<\x02\xF84\x14a\x02\xEEW\x80c=^\xC7\xE3\x14a\x03\x16W\x80cE\xAF&\x1B\x14a\x03RWa\x01\xB6V[\x80c\x17\x03\xC6\x1A\x11a\x01\x94W\x80c\x17\x03\xC6\x1A\x14a\x02JW\x80c\x19\xF4\xF62\x14a\x02rW\x80c9\xF78\x10\x14a\x02\xAEW\x80c:\xC5\0r\x14a\x02\xC4Wa\x01\xB6V[\x80c\x0Bh\x073\x14a\x01\xBAW\x80c\r\x8En,\x14a\x01\xE4W\x80c\x16\xC7\x13\xD9\x14a\x02\x0EW[_\x80\xFD[4\x80\x15a\x01\xC5W_\x80\xFD[Pa\x01\xCEa\x06\x8EV[`@Qa\x01\xDB\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xEFW_\x80\xFD[Pa\x01\xF8a\x06\xA5V[`@Qa\x02\x05\x91\x90aP\x9BV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x19W_\x80\xFD[Pa\x024`\x04\x806\x03\x81\x01\x90a\x02/\x91\x90aP\xF6V[a\x07 V[`@Qa\x02A\x91\x90aR\x08V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02UW_\x80\xFD[Pa\x02p`\x04\x806\x03\x81\x01\x90a\x02k\x91\x90aP\xF6V[a\x07\xF1V[\0[4\x80\x15a\x02}W_\x80\xFD[Pa\x02\x98`\x04\x806\x03\x81\x01\x90a\x02\x93\x91\x90aP\xF6V[a\n\x10V[`@Qa\x02\xA5\x91\x90aR\x9BV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xB9W_\x80\xFD[Pa\x02\xC2a\x0B\x16V[\0[4\x80\x15a\x02\xCFW_\x80\xFD[Pa\x02\xD8a\r9V[`@Qa\x02\xE5\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xF9W_\x80\xFD[Pa\x03\x14`\x04\x806\x03\x81\x01\x90a\x03\x0F\x91\x90aR\xD7V[a\rPV[\0[4\x80\x15a\x03!W_\x80\xFD[Pa\x03<`\x04\x806\x03\x81\x01\x90a\x037\x91\x90aP\xF6V[a\x104V[`@Qa\x03I\x91\x90aS/V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03]W_\x80\xFD[Pa\x03x`\x04\x806\x03\x81\x01\x90a\x03s\x91\x90aP\xF6V[a\x10hV[`@Qa\x03\x85\x91\x90aR\x9BV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x99W_\x80\xFD[Pa\x03\xB4`\x04\x806\x03\x81\x01\x90a\x03\xAF\x91\x90aS\xFEV[a\x11VV[\0[4\x80\x15a\x03\xC1W_\x80\xFD[Pa\x03\xDC`\x04\x806\x03\x81\x01\x90a\x03\xD7\x91\x90aT\xE4V[a\x16gV[\0[a\x03\xF8`\x04\x806\x03\x81\x01\x90a\x03\xF3\x91\x90aV\xEDV[a\x18\xABV[\0[4\x80\x15a\x04\x05W_\x80\xFD[Pa\x04\x0Ea\x18\xCAV[`@Qa\x04\x1B\x91\x90aW_V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04/W_\x80\xFD[Pa\x04J`\x04\x806\x03\x81\x01\x90a\x04E\x91\x90aWxV[a\x18\xFBV[\0[4\x80\x15a\x04WW_\x80\xFD[Pa\x04r`\x04\x806\x03\x81\x01\x90a\x04m\x91\x90aW\xD5V[a\x1C/V[\0[4\x80\x15a\x04\x7FW_\x80\xFD[Pa\x04\x9A`\x04\x806\x03\x81\x01\x90a\x04\x95\x91\x90aX2V[a\x1DqV[\0[4\x80\x15a\x04\xA7W_\x80\xFD[Pa\x04\xC2`\x04\x806\x03\x81\x01\x90a\x04\xBD\x91\x90aY\x18V[a!gV[\0[4\x80\x15a\x04\xCFW_\x80\xFD[Pa\x04\xD8a$kV[`@Qa\x04\xEB\x97\x96\x95\x94\x93\x92\x91\x90aZ\xBBV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\xFFW_\x80\xFD[Pa\x05\x1A`\x04\x806\x03\x81\x01\x90a\x05\x15\x91\x90aP\xF6V[a%tV[`@Qa\x05(\x92\x91\x90a]\xCDV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05<W_\x80\xFD[Pa\x05Ea(\xDAV[`@Qa\x05R\x91\x90aP\x9BV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05fW_\x80\xFD[Pa\x05oa)\x13V[`@Qa\x05|\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\x90W_\x80\xFD[Pa\x05\xAB`\x04\x806\x03\x81\x01\x90a\x05\xA6\x91\x90aP\xF6V[a)*V[\0[4\x80\x15a\x05\xB8W_\x80\xFD[Pa\x05\xC1a+\x94V[\0[4\x80\x15a\x05\xCEW_\x80\xFD[Pa\x05\xE9`\x04\x806\x03\x81\x01\x90a\x05\xE4\x91\x90aP\xF6V[a,\xB9V[`@Qa\x05\xF7\x92\x91\x90a^JV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06\x0BW_\x80\xFD[Pa\x06&`\x04\x806\x03\x81\x01\x90a\x06!\x91\x90a^\x7FV[a/\x8AV[\0[4\x80\x15a\x063W_\x80\xFD[Pa\x06<a1\x14V[`@Qa\x06I\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06]W_\x80\xFD[Pa\x06x`\x04\x806\x03\x81\x01\x90a\x06s\x91\x90aP\xF6V[a1+V[`@Qa\x06\x85\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xF3[_\x80a\x06\x98a1NV[\x90P\x80`\x05\x01T\x91PP\x90V[```@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x06\xE6_a1uV[a\x06\xF0`\x02a1uV[a\x06\xF9_a1uV[`@Q` \x01a\x07\x0C\x94\x93\x92\x91\x90a_xV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[``_a\x07+a1NV[\x90P_\x81`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\x02\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x82\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07\xE3W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x07\x9AW[PPPPP\x92PPP\x91\x90PV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08NW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08r\x91\x90a_\xEAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x08\xE1W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xD8\x91\x90a`\x15V[`@Q\x80\x91\x03\x90\xFD[_a\x08\xEAa1NV[\x90P\x80`\t\x01T\x82\x11\x80a\t\x06WP`\xF8`\x05`\xFF\x16\x90\x1B\x82\x11\x15[\x15a\tHW\x81`@Q\x7F\xCB\xE9&V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t?\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\t\xAAW\x81`@Q\x7F\xDF\r\xB5\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xA1\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[`\x01\x81`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F8O\x90\xFE\xFB\xCF\xAAh\xF2.\0\tJ\xEA\xA5++\xC6\x93\x93m,\xE1\xAF\xED\x12\x12R\x0BY\xB5\x8E\x82`@Qa\n\x04\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xA1PPV[_\x80a\n\x1Aa1NV[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\n}W\x82`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\nt\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[_\x80\x1B\x81`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x03a\n\xD6W\x82`@Q\x7F\x83\xF1\x835\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\xCD\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\r\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x92PPP\x91\x90PV[`\x01a\x0B a2?V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0BaW`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03_a\x0Bla2cV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x0B\xB4WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x0B\xEBW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x0C\xA4`@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa2\x8AV[_a\x0C\xADa1NV[\x90P`\xF8`\x03`\xFF\x16\x90\x1B\x81`\x04\x01\x81\x90UP`\xF8`\x04`\xFF\x16\x90\x1B\x81`\x05\x01\x81\x90UP`\xF8`\x05`\xFF\x16\x90\x1B\x81`\t\x01\x81\x90UPP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\r-\x91\x90a`PV[`@Q\x80\x91\x03\x90\xA1PPV[_\x80a\rCa1NV[\x90P\x80`\t\x01T\x91PP\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xADW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xD1\x91\x90a_\xEAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0E@W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E7\x91\x90a`\x15V[`@Q\x80\x91\x03\x90\xFD[_a\x0EIa1NV[\x90P_\x81`\t\x01T\x90P`\xF8`\x05`\xFF\x16\x90\x1B\x81\x14\x15\x80\x15a\x0E\x88WP\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15a\x0E\xCAW\x80`@Q\x7F\x06\x1A\xC6\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\xC1\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[\x81`\t\x01_\x81T\x80\x92\x91\x90a\x0E\xDE\x90a`\x96V[\x91\x90PUP_\x82`\t\x01T\x90P\x84\x83`\n\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x83\x83`\r\x01_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a\x0F8Wa\x0F7aR(V[[\x02\x17\x90UP_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x9BW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xBF\x91\x90a`\xF1V[\x90P_a\x0F\xCB\x82a2\xA0V[\x90P\x80\x85`\x0E\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x90\x81a\x0F\xED\x91\x90ac\x16V[P\x7F\x8C\xF0\x15\x13\x93\xF8O\xD6\x94\xC5\xE3\x15\xCBt\xCC\x05\xB2G\xDE\nEO\xD9\xE9\x12\x9Cf\x1E\xFD\xF9@\x1D\x83\x88\x88\x84`@Qa\x10#\x94\x93\x92\x91\x90ac\xE5V[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[_\x80a\x10>a1NV[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[_\x80a\x10ra1NV[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x10\xD5W\x82`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\xCC\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[_\x80\x1B\x81`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x03a\x11.W\x82`@Q\x7F\xD5\xFD<\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11%\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[\x80`\r\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[_a\x11_a1NV[\x90P\x80`\x05\x01T\x86\x11\x80a\x11{WP`\xF8`\x04`\xFF\x16\x90\x1B\x86\x11\x15[\x15a\x11\xBDW\x85`@Q\x7F\xAD\xFA\xB9\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\xB4\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[_\x85\x85\x90P\x03a\x12\x04W\x85`@Q\x7F\xE6\xF9\x08;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\xFB\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[_\x80a\x12\x0F\x88a2\xCCV[\x91P\x91P_\x83`\x06\x01_\x8A\x81R` \x01\x90\x81R` \x01_ T\x90P\x83`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x12\x80W`@Q\x7Fo\xBC\xDD+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x12\x8E\x82\x8B\x8B\x8B\x88a4VV[\x90P_a\x12\x9D\x84\x83\x8A\x8Aa67V[\x90P\x85_\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x13=W\x8A\x81`@Q\x7F\x98\xFB\x95}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x134\x92\x91\x90ad/V[`@Q\x80\x91\x03\x90\xFD[`\x01\x86_\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x86`\x02\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F*\xFEd\xFB:\xFD\xE8\xE2g\x8A\xEA\x84\xCF6\"?3\x0E/\xB1(m7\xAE\xD5s\xAB\x9C\xD1\xDBG\xC7\x8C\x8C\x8C\x8C\x8C3`@Qa\x14_\x96\x95\x94\x93\x92\x91\x90afbV[`@Q\x80\x91\x03\x90\xA1\x86`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x14\x9EWPa\x14\x9D\x85\x82\x80T\x90Pa6\x9FV[[\x15a\x16YW`\x01\x87`\x01\x01_\x8E\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_[\x8B\x8B\x90P\x81\x10\x15a\x15TW\x87`\x07\x01_\x8E\x81R` \x01\x90\x81R` \x01_ \x8C\x8C\x83\x81\x81\x10a\x15\x01Wa\x15\0af\xB7V[[\x90P` \x02\x81\x01\x90a\x15\x13\x91\x90af\xF0V[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P\x81\x81a\x15E\x91\x90ai#V[PP\x80\x80`\x01\x01\x91PPa\x14\xD0V[P\x82\x87`\x03\x01_\x8E\x81R` \x01\x90\x81R` \x01_ \x81\x90UP_a\x15\xFA\x86\x83\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x15\xF0W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x15\xA7W[PPPPPa7<V[\x90P_\x87Q\x11\x80\x15a\x162WP`\x03`\xFF\x16\x87_\x81Q\x81\x10a\x16\x1FWa\x16\x1Eaf\xB7V[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16\x14[\x15a\x16IWa\x16D\x88\x8D\x8D\x84\x8Ba8\x84V[a\x16WV[a\x16V\x88\x8E\x8E\x8E\x85a9\x96V[[P[PPPPPPPPPPPPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xC4W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xE8\x91\x90a_\xEAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x17WW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17N\x91\x90a`\x15V[`@Q\x80\x91\x03\x90\xFD[_a\x17`a1NV[\x90P\x80`\x05\x01T\x88\x11\x80a\x17|WP`\xF8`\x04`\xFF\x16\x90\x1B\x88\x11\x15[\x15a\x17\xBEW\x87`@Q\x7F\xAD\xFA\xB9\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17\xB5\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[\x84\x84\x90P\x87\x87\x90P\x14a\x17\xFDW`@Q\x7F\x89K*\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x18\x06\x82a9\xE3V[`\x01\x81`\x10\x01_\x8A\x81R` \x01\x90\x81R` \x01_ T\x14a\x18^W\x87`@Q\x7F\x05\xB0\x83\xF2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18U\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[\x7F\x8B\xFA}\x0E\xD6\xF8~Rkb4)\x18\xEE{\xFAS\x95+\xAD\xD4c\xDC\x93@T\xD7\xDD\x94\x0E\xAF\xDC\x88\x88\x88\x88\x88\x88\x88`@Qa\x18\x99\x97\x96\x95\x94\x93\x92\x91\x90ai\x99V[`@Q\x80\x91\x03\x90\xA1PPPPPPPPV[a\x18\xB3a:+V[a\x18\xBC\x82a;\x11V[a\x18\xC6\x82\x82a<\x04V[PPV[_a\x18\xD3a=\"V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[_a\x19\x04a1NV[\x90P\x80`\x04\x01T\x84\x11\x80a\x19 WP`\xF8`\x03`\xFF\x16\x90\x1B\x84\x11\x15[\x15a\x19bW\x83`@Q\x7F\n\xB7\xF6\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19Y\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[_\x80a\x19m\x86a2\xCCV[\x91P\x91P_a\x19|\x87\x84a=\xA9V[\x90P_a\x19\x8B\x83\x83\x89\x89a67V[\x90P\x84_\x01_\x89\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x1A+W\x87\x81`@Q\x7F3\xCA\x1F\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1A\"\x92\x91\x90ad/V[`@Q\x80\x91\x03\x90\xFD[`\x01\x85_\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x85`\x02\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7FLq\\W4\xCE\\\x18\xC9\xC1.\x84\x96\xE5=*e\xF1\xEC8\x1DGiW\xF0\xF5\x96\xB3d\xA5\x9B\x0C\x89\x89\x893`@Qa\x1BI\x94\x93\x92\x91\x90ai\xFCV[`@Q\x80\x91\x03\x90\xA1\x85`\x01\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x1B\x88WPa\x1B\x87\x84\x82\x80T\x90Pa6\x9FV[[\x15a\x1C$W`\x01\x86`\x01\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x82\x86`\x03\x01_\x8B\x81R` \x01\x90\x81R` \x01_ \x81\x90UP_\x86`\x06\x01_\x8B\x81R` \x01\x90\x81R` \x01_ T\x90P\x7F:\x11a \xCC\xA5\xD4\xF0s\xCC\x1F\xC3\x1F\xF2a3\xAB{\x04\x99\xF2q/\xA0\x10\x02;\x87\xD5\xA1\xF9\xEE\x8A\x82\x88`@Qa\x1C\x1A\x93\x92\x91\x90aj:V[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPPPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\x8CW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xB0\x91\x90a_\xEAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x1D\x1FW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1D\x16\x91\x90a`\x15V[`@Q\x80\x91\x03\x90\xFD[a\x1Dl\x83\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa>\x0BV[PPPV[_a\x1Dza1NV[\x90P\x80`\t\x01T\x86\x11\x80a\x1D\x96WP`\xF8`\x05`\xFF\x16\x90\x1B\x86\x11\x15[\x15a\x1D\xD8W\x85`@Q\x7F\x8D\x8C\x94\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1D\xCF\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[_\x80a\x1D\xE3\x88a2\xCCV[\x91P\x91P_a\x1E\x08\x89\x85`\n\x01_\x8C\x81R` \x01\x90\x81R` \x01_ T\x8A\x8A\x87a?\xC6V[\x90P_a\x1E\x17\x83\x83\x89\x89a67V[\x90P\x84_\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x1E\xB7W\x89\x81`@Q\x7F\xFC\xF5\xA6\xE9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1E\xAE\x92\x91\x90ad/V[`@Q\x80\x91\x03\x90\xFD[`\x01\x85_\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x85`\x02\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F{\xF1\xB4,\x10\xE9I|\x87\x96 \xC5\xB7\xAF\xCE\xD1\x0B\xDA\x17\xD8\xC9\x0B\"\xF0\xE3\xBCk/\xD6\xCE\xD0\xBD\x8B\x8B\x8B\x8B\x8B3`@Qa\x1F\xD9\x96\x95\x94\x93\x92\x91\x90ajvV[`@Q\x80\x91\x03\x90\xA1\x85`\x01\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a \x18WPa \x17\x84\x82\x80T\x90Pa6\x9FV[[\x15a!ZW`\x01\x86`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x89\x89\x87`\x0B\x01_\x8E\x81R` \x01\x90\x81R` \x01_ \x91\x82a j\x92\x91\x90ah\x02V[P\x82\x86`\x03\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x8A\x86`\x0C\x01\x81\x90UP_a!\x19\x85\x83\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a!\x0FW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a \xC6W[PPPPPa7<V[\x90P\x7F\"X\xB7?\xAE\xD3?\xB2\xE2\xEAED\x03\xBE\xF9t\x92\x0C\xAFh*\xB3\xA7#HO\xCFgU;\x16\xA2\x8C\x82\x8D\x8D`@Qa!P\x94\x93\x92\x91\x90aj\xCBV[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPPPPPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\xC4W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xE8\x91\x90a_\xEAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\"WW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\"N\x91\x90a`\x15V[`@Q\x80\x91\x03\x90\xFD[_a\"`a1NV[\x90P\x80`\x05\x01T\x87\x11\x80a\"|WP`\xF8`\x04`\xFF\x16\x90\x1B\x87\x11\x15[\x15a\"\xBEW\x86`@Q\x7F\xAD\xFA\xB9\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\"\xB5\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a#\x14W`@Q\x7Fo\xBC\xDD+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x86\x86\x90P\x03a#[W\x86`@Q\x7F\xE6\xF9\x08;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a#R\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[a#d\x82a9\xE3V[\x80`\x0F\x01_\x88\x81R` \x01\x90\x81R` \x01_ _a#\x82\x91\x90aO/V[_[\x86\x86\x90P\x81\x10\x15a$\x08W\x81`\x0F\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x87\x87\x83\x81\x81\x10a#\xB5Wa#\xB4af\xB7V[[\x90P` \x02\x81\x01\x90a#\xC7\x91\x90af\xF0V[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P\x81\x81a#\xF9\x91\x90ai#V[PP\x80\x80`\x01\x01\x91PPa#\x84V[P\x81\x81`\x10\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x7F\xA4vd\x86\x1A\xB5\x8C[\xD5\x04\x0E\x9C\xC4^h\xD0\xE4\x8E\xC0Cq\x03_\xD7P\x99\xE2\x17\xE0\xA6\xAA\x81\x87\x85\x85\x89\x89\x87`@Qa$Z\x96\x95\x94\x93\x92\x91\x90al<V[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[_``\x80_\x80_``_a$}a@WV[\x90P_\x80\x1B\x81_\x01T\x14\x80\x15a$\x98WP_\x80\x1B\x81`\x01\x01T\x14[a$\xD7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a$\xCE\x90al\xDBV[`@Q\x80\x91\x03\x90\xFD[a$\xDFa@~V[a$\xE7aA\x1CV[F0_\x80\x1B_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%\x06Wa%\x05aU\xC9V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a%4W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94\x93\x92\x91\x90\x97P\x97P\x97P\x97P\x97P\x97P\x97PP\x90\x91\x92\x93\x94\x95\x96V[``\x80_a%\x80a1NV[\x90P\x80`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a%\xE3W\x83`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a%\xDA\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x80\x1B\x81\x03a&@W\x84`@Q\x7F\x83\xF1\x835\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&7\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[_\x82`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a&\xE0W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a&\x97W[PPPPP\x90P_a'\x8A\x84`\x0E\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x80Ta'\t\x90aaIV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta'5\x90aaIV[\x80\x15a'\x80W\x80`\x1F\x10a'WWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a'\x80V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a'cW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPaA\xBAV[\x90P_a'\x97\x82\x84a7<V[\x90P\x80\x85`\x07\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a(\xC6W\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x01\x81\x11\x15a(\x11Wa(\x10aR(V[[`\x01\x81\x11\x15a(#Wa(\"aR(V[[\x81R` \x01`\x01\x82\x01\x80Ta(7\x90aaIV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta(c\x90aaIV[\x80\x15a(\xAEW\x80`\x1F\x10a(\x85Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a(\xAEV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a(\x91W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a'\xCDV[PPPP\x90P\x96P\x96PPPPPP\x91P\x91V[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[_\x80a)\x1Da1NV[\x90P\x80`\x0C\x01T\x91PP\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)\x87W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\xAB\x91\x90a_\xEAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a*\x1AW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a*\x11\x91\x90a`\x15V[`@Q\x80\x91\x03\x90\xFD[_a*#a1NV[\x90P\x80`\x04\x01T\x82\x11\x80a*?WP`\xF8`\x03`\xFF\x16\x90\x1B\x82\x11\x15[\x15a*\x81W\x81`@Q\x7F\xFC\xF2\xDBz\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a*x\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a*\xFAW\x82`@Q\x7F\x92x\x9Bg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a*\xF1\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[`\x01\x82`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x81\x14a+XW`\x01\x82`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP[\x7F+\x08{\x88K5\xA8\x1Dv\x9D\x1A\x1E\t(\x80\xF1\xDAV\xDE\x96NK3\x9E\xAB\xCB\x1FE\xF5\xFE2d\x83`@Qa+\x87\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xA1PPPV[`\x03_a+\x9Fa2cV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a+\xE7WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a,\x1EW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa,\xAD\x91\x90a`PV[`@Q\x80\x91\x03\x90\xA1PPV[``\x80_a,\xC5a1NV[\x90P\x80`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a-(W\x83`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a-\x1F\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x80\x1B\x81\x03a-\x85W\x84`@Q\x7F\xD5\xFD<\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a-|\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[_\x82`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a.%W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a-\xDCW[PPPPP\x90P_a.\xCF\x84`\x0E\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x80Ta.N\x90aaIV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta.z\x90aaIV[\x80\x15a.\xC5W\x80`\x1F\x10a.\x9CWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a.\xC5V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a.\xA8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPaA\xBAV[\x90P_a.\xDC\x82\x84a7<V[\x90P\x80\x85`\x0B\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80\x80Ta.\xFE\x90aaIV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta/*\x90aaIV[\x80\x15a/uW\x80`\x1F\x10a/LWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a/uV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a/XW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x96P\x96PPPPPP\x91P\x91V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/\xE7W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\x0B\x91\x90a_\xEAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a0zW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a0q\x91\x90a`\x15V[`@Q\x80\x91\x03\x90\xFD[_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a0\xD8W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\xFC\x91\x90a`\xF1V[\x90Pa1\x10\x82a1\x0B\x83a2\xA0V[a>\x0BV[PPV[_\x80a1\x1Ea1NV[\x90P\x80`\x08\x01T\x91PP\x90V[_a14a1NV[`\x10\x01_\x83\x81R` \x01\x90\x81R` \x01_ T\x90P\x91\x90PV[_\x7F&\xFD\xAF\x8A,\xB2\r \xB5^6!\x89\x86\x90^SN\xE7\xA9p\xDD/\xA8'\x94nKt\x96\xDB\0\x90P\x90V[``_`\x01a1\x83\x84aCbV[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\xA1Wa1\xA0aU\xC9V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a1\xD3W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x01\x82\x01\x90P[`\x01\x15a24W\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a2)Wa2(al\xF9V[[\x04\x94P_\x85\x03a1\xE0W[\x81\x93PPPP\x91\x90PV[_a2Ha2cV[_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[a2\x92aD\xB3V[a2\x9C\x82\x82aD\xF3V[PPV[```\x01\x82`@Q` \x01a2\xB6\x92\x91\x90am\x86V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[``_\x80a2\xD8a1NV[\x90P\x80`\x0E\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x80Ta2\xF8\x90aaIV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta3$\x90aaIV[\x80\x15a3oW\x80`\x1F\x10a3FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a3oV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a3RW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x92Pa3\x7F\x83aA\xBAV[\x91PsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cF\xC5\xBB\xBD\x833`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a3\xD0\x92\x91\x90ad/V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3\xEBW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4\x0F\x91\x90am\xDBV[a4PW3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a4G\x91\x90a`\x15V[`@Q\x80\x91\x03\x90\xFD[P\x91P\x91V[_\x80\x84\x84\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4uWa4taU\xC9V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a4\xA3W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_[\x85\x85\x90P\x81\x10\x15a5\xA7W`@Q\x80``\x01`@R\x80`%\x81R` \x01av0`%\x919\x80Q\x90` \x01 \x86\x86\x83\x81\x81\x10a4\xE6Wa4\xE5af\xB7V[[\x90P` \x02\x81\x01\x90a4\xF8\x91\x90af\xF0V[_\x01` \x81\x01\x90a5\t\x91\x90an\x06V[\x87\x87\x84\x81\x81\x10a5\x1CWa5\x1Baf\xB7V[[\x90P` \x02\x81\x01\x90a5.\x91\x90af\xF0V[\x80` \x01\x90a5=\x91\x90ag\x96V[`@Qa5K\x92\x91\x90an_V[`@Q\x80\x91\x03\x90 `@Q` \x01a5e\x93\x92\x91\x90an\x86V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x82\x81Q\x81\x10a5\x8EWa5\x8Daf\xB7V[[` \x02` \x01\x01\x81\x81RPP\x80\x80`\x01\x01\x91PPa4\xA8V[Pa6+`@Q\x80`\xC0\x01`@R\x80`\x82\x81R` \x01au\xAE`\x82\x919\x80Q\x90` \x01 \x88\x88\x84`@Q` \x01a5\xDE\x91\x90aolV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x87\x80Q\x90` \x01 `@Q` \x01a6\x10\x95\x94\x93\x92\x91\x90ao\x82V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 aEDV[\x91PP\x95\x94PPPPPV[_\x80a6\x86\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPaE]V[\x90Pa6\x93\x86\x823aE\x87V[\x80\x91PP\x94\x93PPPPV[_\x80sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cA\xAD\x06\x9C\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a6\xEE\x91\x90aO\xF8V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\tW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7-\x91\x90a`\xF1V[\x90P\x80\x83\x10\x15\x91PP\x92\x91PPV[``_\x82Q\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7^Wa7]aU\xC9V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a7\x91W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a7|W\x90P[P\x90P_[\x82\x81\x10\x15a8xWsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c1\xFFA\xC8\x87\x87\x84\x81Q\x81\x10a7\xE2Wa7\xE1af\xB7V[[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a8\x07\x92\x91\x90ad/V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8!W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8I\x91\x90aq&V[``\x01Q\x82\x82\x81Q\x81\x10a8`Wa8_af\xB7V[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa7\x96V[P\x80\x92PPP\x92\x91PPV[_a8\x8E\x82aGhV[\x90P\x85`\x0F\x01_\x82\x81R` \x01\x90\x81R` \x01_ _a8\xAE\x91\x90aO/V[_[\x85\x85\x90P\x81\x10\x15a94W\x86`\x0F\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x86\x86\x83\x81\x81\x10a8\xE1Wa8\xE0af\xB7V[[\x90P` \x02\x81\x01\x90a8\xF3\x91\x90af\xF0V[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P\x81\x81a9%\x91\x90ai#V[PP\x80\x80`\x01\x01\x91PPa8\xB0V[P`\x01\x86`\x10\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x7F\xA4vd\x86\x1A\xB5\x8C[\xD5\x04\x0E\x9C\xC4^h\xD0\xE4\x8E\xC0Cq\x03_\xD7P\x99\xE2\x17\xE0\xA6\xAA\x81\x81\x84\x87\x87`\x01`@Qa9\x86\x95\x94\x93\x92\x91\x90aqmV[`@Q\x80\x91\x03\x90\xA1PPPPPPV[\x83\x85`\x08\x01\x81\x90UP\x7F\xEB\x85\xC2m\xBC\xADF\xB8\nh\xA0\xF2L\xCE|,\x90\xF0\xA1\xFA\xDE\xD8A\x84\x13\x889\xFC\x9E\x80\xA2[\x84\x82\x85\x85`@Qa9\xD4\x94\x93\x92\x91\x90aq\xC0V[`@Q\x80\x91\x03\x90\xA1PPPPPV[`\x01\x81\x14a:(W\x80`@Q\x7F\xA3\xE9\xED~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a:\x1F\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a:\xD8WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a:\xBFaG\xB1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a;\x0FW`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;nW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;\x92\x91\x90a_\xEAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a<\x01W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a;\xF8\x91\x90a`\x15V[`@Q\x80\x91\x03\x90\xFD[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a<lWP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<i\x91\x90ar/V[`\x01[a<\xADW\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a<\xA4\x91\x90a`\x15V[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14a=\x13W\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a=\n\x91\x90aW_V[`@Q\x80\x91\x03\x90\xFD[a=\x1D\x83\x83aH\x04V[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a=\xA7W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_a>\x03`@Q\x80``\x01`@R\x80`<\x81R` \x01au\x1C`<\x919\x80Q\x90` \x01 \x84\x84\x80Q\x90` \x01 `@Q` \x01a=\xE8\x93\x92\x91\x90arZV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 aEDV[\x90P\x92\x91PPV[_a>\x14a1NV[\x90P_\x81`\x05\x01T\x90P`\xF8`\x04`\xFF\x16\x90\x1B\x81\x14\x15\x80\x15a>SWP\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15a>\x95W\x80`@Q\x7F;\x85=\xA8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a>\x8C\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[\x81`\x04\x01_\x81T\x80\x92\x91\x90a>\xA9\x90a`\x96V[\x91\x90PUP_\x82`\x04\x01T\x90P\x82`\x05\x01_\x81T\x80\x92\x91\x90a>\xCA\x90a`\x96V[\x91\x90PUP_\x83`\x05\x01T\x90P\x80\x84`\x06\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x81\x84`\x06\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x85\x84`\r\x01_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a?<Wa?;aR(V[[\x02\x17\x90UP\x84\x84`\x0E\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x90\x81a?a\x91\x90ac\x16V[P\x84\x84`\x0E\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x90\x81a?\x82\x91\x90ac\x16V[P\x7F\xFB\xF5'H\x10\xB9O\x86\x97\x0C\x11G\xE8\xFF\xAE\xBE\xD2F\xEE\x97w\xD6\x95\xA6\x90\x04\xDCbV\xD1\xFE\x91\x82\x87\x87`@Qa?\xB6\x93\x92\x91\x90ar\x8FV[`@Q\x80\x91\x03\x90\xA1PPPPPPV[_a@L`@Q\x80`\x80\x01`@R\x80`V\x81R` \x01auX`V\x919\x80Q\x90` \x01 \x87\x87\x87\x87`@Q` \x01a?\xFF\x92\x91\x90an_V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x86\x80Q\x90` \x01 `@Q` \x01a@1\x95\x94\x93\x92\x91\x90ao\x82V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 aEDV[\x90P\x95\x94PPPPPV[_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x90P\x90V[``_a@\x89a@WV[\x90P\x80`\x02\x01\x80Ta@\x9A\x90aaIV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta@\xC6\x90aaIV[\x80\x15aA\x11W\x80`\x1F\x10a@\xE8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91aA\x11V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a@\xF4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[``_aA'a@WV[\x90P\x80`\x03\x01\x80TaA8\x90aaIV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80TaAd\x90aaIV[\x80\x15aA\xAFW\x80`\x1F\x10aA\x86Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91aA\xAFV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11aA\x92W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[_\x80\x82Q\x14\x80aA\xECWP_\x82_\x81Q\x81\x10aA\xD9WaA\xD8af\xB7V[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16\x14[\x15aByWsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aBNW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aBr\x91\x90a`\xF1V[\x90PaC]V[_\x82_\x81Q\x81\x10aB\x8DWaB\x8Caf\xB7V[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P`\x01`\xFF\x16\x81`\xFF\x16\x14\x15\x80\x15aB\xBBWP`\x03`\xFF\x16\x81`\xFF\x16\x14\x15[\x15aB\xFDW\x80`@Q\x7F!9\xCC,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aB\xF4\x91\x90ar\xDAV[`@Q\x80\x91\x03\x90\xFD[_`\x03`\xFF\x16\x82`\xFF\x16\x14aC\x13W`!aC\x16V[`B[\x90P\x80\x84Q\x10\x15aCSW`@Q\x7F\x8B$\x8B`\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`!\x84\x01Q\x92PPP[\x91\x90PV[_\x80_\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10aC\xBEWz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81aC\xB4WaC\xB3al\xF9V[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10aC\xFBWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81aC\xF1WaC\xF0al\xF9V[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10aD*Wf#\x86\xF2o\xC1\0\0\x83\x81aD WaD\x1Fal\xF9V[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10aDSWc\x05\xF5\xE1\0\x83\x81aDIWaDHal\xF9V[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10aDxWa'\x10\x83\x81aDnWaDmal\xF9V[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10aD\x9BW`d\x83\x81aD\x91WaD\x90al\xF9V[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10aD\xAAW`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[aD\xBBaHvV[aD\xF1W`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[aD\xFBaD\xB3V[_aE\x04a@WV[\x90P\x82\x81`\x02\x01\x90\x81aE\x17\x91\x90asKV[P\x81\x81`\x03\x01\x90\x81aE)\x91\x90asKV[P_\x80\x1B\x81_\x01\x81\x90UP_\x80\x1B\x81`\x01\x01\x81\x90UPPPPV[_aEVaEPaH\x94V[\x83aH\xA2V[\x90P\x91\x90PV[_\x80_\x80aEk\x86\x86aH\xE2V[\x92P\x92P\x92PaE{\x82\x82aI7V[\x82\x93PPPP\x92\x91PPV[sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x94G\xCF\xD4\x84\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aE\xD6\x92\x91\x90ad/V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aE\xF1W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aF\x15\x91\x90am\xDBV[aFXW\x81\x81`@Q\x7F\r\x86\xF5!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aFO\x92\x91\x90at\x1AV[`@Q\x80\x91\x03\x90\xFD[_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c1\xFFA\xC8\x85\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aF\xA8\x92\x91\x90ad/V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aF\xC2W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aF\xEA\x91\x90aq&V[\x90P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14aGbW\x82\x82`@Q\x7F\r\x86\xF5!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aGY\x92\x91\x90at\x1AV[`@Q\x80\x91\x03\x90\xFD[PPPPV[_`B\x82Q\x10\x15aG\xA5W`@Q\x7F\x8B$\x8B`\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`A\x82\x01Q\x90P\x91\x90PV[_aG\xDD\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1BaJ\x99V[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[aH\r\x82aJ\xA2V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15aHiWaHc\x82\x82aKkV[PaHrV[aHqaK\xEBV[[PPV[_aH\x7Fa2cV[_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[_aH\x9DaL'V[\x90P\x90V[_`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x02\x82\x01R\x82`\"\x82\x01R`B\x81 \x91PP\x92\x91PPV[_\x80_`A\x84Q\x03aI\"W_\x80_` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90PaI\x14\x88\x82\x85\x85aL\x8AV[\x95P\x95P\x95PPPPaI0V[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15aIJWaIIaR(V[[\x82`\x03\x81\x11\x15aI]WaI\\aR(V[[\x03\x15aJ\x95W`\x01`\x03\x81\x11\x15aIwWaIvaR(V[[\x82`\x03\x81\x11\x15aI\x8AWaI\x89aR(V[[\x03aI\xC1W`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15aI\xD5WaI\xD4aR(V[[\x82`\x03\x81\x11\x15aI\xE8WaI\xE7aR(V[[\x03aJ,W\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aJ#\x91\x90aO\xF8V[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15aJ?WaJ>aR(V[[\x82`\x03\x81\x11\x15aJRWaJQaR(V[[\x03aJ\x94W\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aJ\x8B\x91\x90aW_V[`@Q\x80\x91\x03\x90\xFD[[PPV[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03aJ\xFDW\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aJ\xF4\x91\x90a`\x15V[`@Q\x80\x91\x03\x90\xFD[\x80aK)\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1BaJ\x99V[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@QaK\x94\x91\x90atqV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14aK\xCCW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>aK\xD1V[``\x91P[P\x91P\x91PaK\xE1\x85\x83\x83aMqV[\x92PPP\x92\x91PPV[_4\x11\x15aL%W`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0FaLQaM\xFEV[aLYaNtV[F0`@Q` \x01aLo\x95\x94\x93\x92\x91\x90at\x87V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x80_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15aL\xC6W_`\x03\x85\x92P\x92P\x92PaMgV[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@QaL\xE9\x94\x93\x92\x91\x90at\xD8V[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aM\tW=_\x80>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03aMZW_`\x01_\x80\x1B\x93P\x93P\x93PPaMgV[\x80_\x80_\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[``\x82aM\x86WaM\x81\x82aN\xEBV[aM\xF6V[_\x82Q\x14\x80\x15aM\xACWP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15aM\xEEW\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aM\xE5\x91\x90a`\x15V[`@Q\x80\x91\x03\x90\xFD[\x81\x90PaM\xF7V[[\x93\x92PPPV[_\x80aN\x08a@WV[\x90P_aN\x13a@~V[\x90P_\x81Q\x11\x15aN/W\x80\x80Q\x90` \x01 \x92PPPaNqV[_\x82_\x01T\x90P_\x80\x1B\x81\x14aNJW\x80\x93PPPPaNqV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x80aN~a@WV[\x90P_aN\x89aA\x1CV[\x90P_\x81Q\x11\x15aN\xA5W\x80\x80Q\x90` \x01 \x92PPPaN\xE8V[_\x82`\x01\x01T\x90P_\x80\x1B\x81\x14aN\xC1W\x80\x93PPPPaN\xE8V[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x81Q\x11\x15aN\xFDW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T_\x82U`\x02\x02\x90_R` _ \x90\x81\x01\x90aOM\x91\x90aOPV[PV[[\x80\x82\x11\x15aO\x84W_\x80\x82\x01_a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U`\x01\x82\x01_aO{\x91\x90aO\x88V[P`\x02\x01aOQV[P\x90V[P\x80TaO\x94\x90aaIV[_\x82U\x80`\x1F\x10aO\xA5WPaO\xC2V[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90aO\xC1\x91\x90aO\xC5V[[PV[[\x80\x82\x11\x15aO\xDCW_\x81_\x90UP`\x01\x01aO\xC6V[P\x90V[_\x81\x90P\x91\x90PV[aO\xF2\x81aO\xE0V[\x82RPPV[_` \x82\x01\x90PaP\x0B_\x83\x01\x84aO\xE9V[\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15aPHW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90PaP-V[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_aPm\x82aP\x11V[aPw\x81\x85aP\x1BV[\x93PaP\x87\x81\x85` \x86\x01aP+V[aP\x90\x81aPSV[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaP\xB3\x81\x84aPcV[\x90P\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[aP\xD5\x81aO\xE0V[\x81\x14aP\xDFW_\x80\xFD[PV[_\x815\x90PaP\xF0\x81aP\xCCV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aQ\x0BWaQ\naP\xC4V[[_aQ\x18\x84\x82\x85\x01aP\xE2V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_aQs\x82aQJV[\x90P\x91\x90PV[aQ\x83\x81aQiV[\x82RPPV[_aQ\x94\x83\x83aQzV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aQ\xB6\x82aQ!V[aQ\xC0\x81\x85aQ+V[\x93PaQ\xCB\x83aQ;V[\x80_[\x83\x81\x10\x15aQ\xFBW\x81QaQ\xE2\x88\x82aQ\x89V[\x97PaQ\xED\x83aQ\xA0V[\x92PP`\x01\x81\x01\x90PaQ\xCEV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaR \x81\x84aQ\xACV[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x02\x81\x10aRfWaReaR(V[[PV[_\x81\x90PaRv\x82aRUV[\x91\x90PV[_aR\x85\x82aRiV[\x90P\x91\x90PV[aR\x95\x81aR{V[\x82RPPV[_` \x82\x01\x90PaR\xAE_\x83\x01\x84aR\x8CV[\x92\x91PPV[`\x02\x81\x10aR\xC0W_\x80\xFD[PV[_\x815\x90PaR\xD1\x81aR\xB4V[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15aR\xEDWaR\xECaP\xC4V[[_aR\xFA\x85\x82\x86\x01aP\xE2V[\x92PP` aS\x0B\x85\x82\x86\x01aR\xC3V[\x91PP\x92P\x92\x90PV[_\x81\x15\x15\x90P\x91\x90PV[aS)\x81aS\x15V[\x82RPPV[_` \x82\x01\x90PaSB_\x83\x01\x84aS V[\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12aSiWaShaSHV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aS\x86WaS\x85aSLV[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aS\xA2WaS\xA1aSPV[[\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12aS\xBEWaS\xBDaSHV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aS\xDBWaS\xDAaSLV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15aS\xF7WaS\xF6aSPV[[\x92P\x92\x90PV[_\x80_\x80_``\x86\x88\x03\x12\x15aT\x17WaT\x16aP\xC4V[[_aT$\x88\x82\x89\x01aP\xE2V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aTEWaTDaP\xC8V[[aTQ\x88\x82\x89\x01aSTV[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aTtWaTsaP\xC8V[[aT\x80\x88\x82\x89\x01aS\xA9V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[_\x80\x83`\x1F\x84\x01\x12aT\xA4WaT\xA3aSHV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aT\xC1WaT\xC0aSLV[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aT\xDDWaT\xDCaSPV[[\x92P\x92\x90PV[_\x80_\x80_\x80_`\xA0\x88\x8A\x03\x12\x15aT\xFFWaT\xFEaP\xC4V[[_aU\x0C\x8A\x82\x8B\x01aP\xE2V[\x97PP` \x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aU-WaU,aP\xC8V[[aU9\x8A\x82\x8B\x01aT\x8FV[\x96P\x96PP`@\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aU\\WaU[aP\xC8V[[aUh\x8A\x82\x8B\x01aT\x8FV[\x94P\x94PP``aU{\x8A\x82\x8B\x01aP\xE2V[\x92PP`\x80aU\x8C\x8A\x82\x8B\x01aP\xE2V[\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[aU\xA4\x81aQiV[\x81\x14aU\xAEW_\x80\xFD[PV[_\x815\x90PaU\xBF\x81aU\x9BV[\x92\x91PPV[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[aU\xFF\x82aPSV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aV\x1EWaV\x1DaU\xC9V[[\x80`@RPPPV[_aV0aP\xBBV[\x90PaV<\x82\x82aU\xF6V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aV[WaVZaU\xC9V[[aVd\x82aPSV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_aV\x91aV\x8C\x84aVAV[aV'V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15aV\xADWaV\xACaU\xC5V[[aV\xB8\x84\x82\x85aVqV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aV\xD4WaV\xD3aSHV[[\x815aV\xE4\x84\x82` \x86\x01aV\x7FV[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15aW\x03WaW\x02aP\xC4V[[_aW\x10\x85\x82\x86\x01aU\xB1V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aW1WaW0aP\xC8V[[aW=\x85\x82\x86\x01aV\xC0V[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[aWY\x81aWGV[\x82RPPV[_` \x82\x01\x90PaWr_\x83\x01\x84aWPV[\x92\x91PPV[_\x80_`@\x84\x86\x03\x12\x15aW\x8FWaW\x8EaP\xC4V[[_aW\x9C\x86\x82\x87\x01aP\xE2V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aW\xBDWaW\xBCaP\xC8V[[aW\xC9\x86\x82\x87\x01aS\xA9V[\x92P\x92PP\x92P\x92P\x92V[_\x80_`@\x84\x86\x03\x12\x15aW\xECWaW\xEBaP\xC4V[[_aW\xF9\x86\x82\x87\x01aR\xC3V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aX\x1AWaX\x19aP\xC8V[[aX&\x86\x82\x87\x01aS\xA9V[\x92P\x92PP\x92P\x92P\x92V[_\x80_\x80_``\x86\x88\x03\x12\x15aXKWaXJaP\xC4V[[_aXX\x88\x82\x89\x01aP\xE2V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aXyWaXxaP\xC8V[[aX\x85\x88\x82\x89\x01aS\xA9V[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aX\xA8WaX\xA7aP\xC8V[[aX\xB4\x88\x82\x89\x01aS\xA9V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[_\x80\x83`\x1F\x84\x01\x12aX\xD8WaX\xD7aSHV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aX\xF5WaX\xF4aSLV[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aY\x11WaY\x10aSPV[[\x92P\x92\x90PV[_\x80_\x80_\x80`\x80\x87\x89\x03\x12\x15aY2WaY1aP\xC4V[[_aY?\x89\x82\x8A\x01aP\xE2V[\x96PP` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aY`WaY_aP\xC8V[[aYl\x89\x82\x8A\x01aSTV[\x95P\x95PP`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aY\x8FWaY\x8EaP\xC8V[[aY\x9B\x89\x82\x8A\x01aX\xC3V[\x93P\x93PP``aY\xAE\x89\x82\x8A\x01aP\xE2V[\x91PP\x92\x95P\x92\x95P\x92\x95V[_\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[aY\xEF\x81aY\xBBV[\x82RPPV[aY\xFE\x81aQiV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[aZ6\x81aO\xE0V[\x82RPPV[_aZG\x83\x83aZ-V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aZi\x82aZ\x04V[aZs\x81\x85aZ\x0EV[\x93PaZ~\x83aZ\x1EV[\x80_[\x83\x81\x10\x15aZ\xAEW\x81QaZ\x95\x88\x82aZ<V[\x97PaZ\xA0\x83aZSV[\x92PP`\x01\x81\x01\x90PaZ\x81V[P\x85\x93PPPP\x92\x91PPV[_`\xE0\x82\x01\x90PaZ\xCE_\x83\x01\x8AaY\xE6V[\x81\x81\x03` \x83\x01RaZ\xE0\x81\x89aPcV[\x90P\x81\x81\x03`@\x83\x01RaZ\xF4\x81\x88aPcV[\x90Pa[\x03``\x83\x01\x87aO\xE9V[a[\x10`\x80\x83\x01\x86aY\xF5V[a[\x1D`\xA0\x83\x01\x85aWPV[\x81\x81\x03`\xC0\x83\x01Ra[/\x81\x84aZ_V[\x90P\x98\x97PPPPPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a[\x80\x82aP\x11V[a[\x8A\x81\x85a[fV[\x93Pa[\x9A\x81\x85` \x86\x01aP+V[a[\xA3\x81aPSV[\x84\x01\x91PP\x92\x91PPV[_a[\xB9\x83\x83a[vV[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a[\xD7\x82a[=V[a[\xE1\x81\x85a[GV[\x93P\x83` \x82\x02\x85\x01a[\xF3\x85a[WV[\x80_[\x85\x81\x10\x15a\\.W\x84\x84\x03\x89R\x81Qa\\\x0F\x85\x82a[\xAEV[\x94Pa\\\x1A\x83a[\xC1V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa[\xF6V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[`\x02\x81\x10a\\zWa\\yaR(V[[PV[_\x81\x90Pa\\\x8A\x82a\\iV[\x91\x90PV[_a\\\x99\x82a\\}V[\x90P\x91\x90PV[a\\\xA9\x81a\\\x8FV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a\\\xD3\x82a\\\xAFV[a\\\xDD\x81\x85a\\\xB9V[\x93Pa\\\xED\x81\x85` \x86\x01aP+V[a\\\xF6\x81aPSV[\x84\x01\x91PP\x92\x91PPV[_`@\x83\x01_\x83\x01Qa]\x16_\x86\x01\x82a\\\xA0V[P` \x83\x01Q\x84\x82\x03` \x86\x01Ra].\x82\x82a\\\xC9V[\x91PP\x80\x91PP\x92\x91PPV[_a]F\x83\x83a]\x01V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a]d\x82a\\@V[a]n\x81\x85a\\JV[\x93P\x83` \x82\x02\x85\x01a]\x80\x85a\\ZV[\x80_[\x85\x81\x10\x15a]\xBBW\x84\x84\x03\x89R\x81Qa]\x9C\x85\x82a];V[\x94Pa]\xA7\x83a]NV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa]\x83V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra]\xE5\x81\x85a[\xCDV[\x90P\x81\x81\x03` \x83\x01Ra]\xF9\x81\x84a]ZV[\x90P\x93\x92PPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a^\x1C\x82a\\\xAFV[a^&\x81\x85a^\x02V[\x93Pa^6\x81\x85` \x86\x01aP+V[a^?\x81aPSV[\x84\x01\x91PP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra^b\x81\x85a[\xCDV[\x90P\x81\x81\x03` \x83\x01Ra^v\x81\x84a^\x12V[\x90P\x93\x92PPPV[_` \x82\x84\x03\x12\x15a^\x94Wa^\x93aP\xC4V[[_a^\xA1\x84\x82\x85\x01aR\xC3V[\x91PP\x92\x91PPV[_\x81\x90P\x92\x91PPV[_a^\xBE\x82aP\x11V[a^\xC8\x81\x85a^\xAAV[\x93Pa^\xD8\x81\x85` \x86\x01aP+V[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a_\x18`\x02\x83a^\xAAV[\x91Pa_#\x82a^\xE4V[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a_b`\x01\x83a^\xAAV[\x91Pa_m\x82a_.V[`\x01\x82\x01\x90P\x91\x90PV[_a_\x83\x82\x87a^\xB4V[\x91Pa_\x8E\x82a_\x0CV[\x91Pa_\x9A\x82\x86a^\xB4V[\x91Pa_\xA5\x82a_VV[\x91Pa_\xB1\x82\x85a^\xB4V[\x91Pa_\xBC\x82a_VV[\x91Pa_\xC8\x82\x84a^\xB4V[\x91P\x81\x90P\x95\x94PPPPPV[_\x81Q\x90Pa_\xE4\x81aU\x9BV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a_\xFFWa_\xFEaP\xC4V[[_a`\x0C\x84\x82\x85\x01a_\xD6V[\x91PP\x92\x91PPV[_` \x82\x01\x90Pa`(_\x83\x01\x84aY\xF5V[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a`J\x81a`.V[\x82RPPV[_` \x82\x01\x90Pa`c_\x83\x01\x84a`AV[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a`\xA0\x82aO\xE0V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a`\xD2Wa`\xD1a`iV[[`\x01\x82\x01\x90P\x91\x90PV[_\x81Q\x90Pa`\xEB\x81aP\xCCV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aa\x06Waa\x05aP\xC4V[[_aa\x13\x84\x82\x85\x01a`\xDDV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80aa`W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aasWaaraa\x1CV[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02aa\xD5\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82aa\x9AV[aa\xDF\x86\x83aa\x9AV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_ab\x1Aab\x15ab\x10\x84aO\xE0V[aa\xF7V[aO\xE0V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[ab3\x83ab\0V[abGab?\x82ab!V[\x84\x84Taa\xA6V[\x82UPPPPV[_\x90V[ab[abOV[abf\x81\x84\x84ab*V[PPPV[[\x81\x81\x10\x15ab\x89Wab~_\x82abSV[`\x01\x81\x01\x90PablV[PPV[`\x1F\x82\x11\x15ab\xCEWab\x9F\x81aayV[ab\xA8\x84aa\x8BV[\x81\x01` \x85\x10\x15ab\xB7W\x81\x90P[ab\xCBab\xC3\x85aa\x8BV[\x83\x01\x82abkV[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_ab\xEE_\x19\x84`\x08\x02ab\xD3V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_ac\x06\x83\x83ab\xDFV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[ac\x1F\x82a\\\xAFV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ac8Wac7aU\xC9V[[acB\x82TaaIV[acM\x82\x82\x85ab\x8DV[_` \x90P`\x1F\x83\x11`\x01\x81\x14ac~W_\x84\x15aclW\x82\x87\x01Q\x90P[acv\x85\x82ab\xFBV[\x86UPac\xDDV[`\x1F\x19\x84\x16ac\x8C\x86aayV[_[\x82\x81\x10\x15ac\xB3W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pac\x8EV[\x86\x83\x10\x15ac\xD0W\x84\x89\x01Qac\xCC`\x1F\x89\x16\x82ab\xDFV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_`\x80\x82\x01\x90Pac\xF8_\x83\x01\x87aO\xE9V[ad\x05` \x83\x01\x86aO\xE9V[ad\x12`@\x83\x01\x85aR\x8CV[\x81\x81\x03``\x83\x01Rad$\x81\x84a^\x12V[\x90P\x95\x94PPPPPV[_`@\x82\x01\x90PadB_\x83\x01\x85aO\xE9V[adO` \x83\x01\x84aY\xF5V[\x93\x92PPPV[_\x81\x90P\x91\x90PV[`\x02\x81\x10adkW_\x80\xFD[PV[_\x815\x90Pad|\x81ad_V[\x92\x91PPV[_ad\x90` \x84\x01\x84adnV[\x90P\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12ad\xC0Wad\xBFad\xA0V[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15ad\xE8Wad\xE7ad\x98V[[`\x01\x82\x026\x03\x83\x13\x15ad\xFEWad\xFDad\x9CV[[P\x92P\x92\x90PV[_ae\x11\x83\x85a\\\xB9V[\x93Pae\x1E\x83\x85\x84aVqV[ae'\x83aPSV[\x84\x01\x90P\x93\x92PPPV[_`@\x83\x01aeC_\x84\x01\x84ad\x82V[aeO_\x86\x01\x82a\\\xA0V[Pae]` \x84\x01\x84ad\xA4V[\x85\x83\x03` \x87\x01Raep\x83\x82\x84ae\x06V[\x92PPP\x80\x91PP\x92\x91PPV[_ae\x89\x83\x83ae2V[\x90P\x92\x91PPV[_\x825`\x01`@\x03\x836\x03\x03\x81\x12ae\xACWae\xABad\xA0V[[\x82\x81\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_ae\xCF\x83\x85a\\JV[\x93P\x83` \x84\x02\x85\x01ae\xE1\x84adVV[\x80_[\x87\x81\x10\x15af$W\x84\x84\x03\x89Rae\xFB\x82\x84ae\x91V[af\x05\x85\x82ae~V[\x94Paf\x10\x83ae\xB8V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pae\xE4V[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_afA\x83\x85a^\x02V[\x93PafN\x83\x85\x84aVqV[afW\x83aPSV[\x84\x01\x90P\x93\x92PPPV[_`\x80\x82\x01\x90Pafu_\x83\x01\x89aO\xE9V[\x81\x81\x03` \x83\x01Raf\x88\x81\x87\x89ae\xC4V[\x90P\x81\x81\x03`@\x83\x01Raf\x9D\x81\x85\x87af6V[\x90Paf\xAC``\x83\x01\x84aY\xF5V[\x97\x96PPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x825`\x01`@\x03\x836\x03\x03\x81\x12ag\x0BWag\naf\xE4V[[\x80\x83\x01\x91PP\x92\x91PPV[_\x815ag#\x81ad_V[\x80\x91PP\x91\x90PV[_\x81_\x1B\x90P\x91\x90PV[_`\xFFagC\x84ag,V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_agc\x82a\\}V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[ag|\x82agYV[ag\x8Fag\x88\x82agjV[\x83Tag7V[\x82UPPPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12ag\xB2Wag\xB1af\xE4V[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15ag\xD4Wag\xD3af\xE8V[[` \x83\x01\x92P`\x01\x82\x026\x03\x83\x13\x15ag\xF0Wag\xEFaf\xECV[[P\x92P\x92\x90PV[_\x82\x90P\x92\x91PPV[ah\x0C\x83\x83ag\xF8V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ah%Wah$aU\xC9V[[ah/\x82TaaIV[ah:\x82\x82\x85ab\x8DV[_`\x1F\x83\x11`\x01\x81\x14ahgW_\x84\x15ahUW\x82\x87\x015\x90P[ah_\x85\x82ab\xFBV[\x86UPah\xC6V[`\x1F\x19\x84\x16ahu\x86aayV[_[\x82\x81\x10\x15ah\x9CW\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PahwV[\x86\x83\x10\x15ah\xB9W\x84\x89\x015ah\xB5`\x1F\x89\x16\x82ab\xDFV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[ah\xDA\x83\x83\x83ah\x02V[PPPV[_\x81\x01_\x83\x01\x80ah\xEF\x81ag\x17V[\x90Pah\xFB\x81\x84agsV[PPP`\x01\x81\x01` \x83\x01ai\x10\x81\x85ag\x96V[ai\x1B\x81\x83\x86ah\xCFV[PPPPPPV[ai-\x82\x82ah\xDFV[PPV[_\x80\xFD[\x82\x81\x837PPPV[_aiI\x83\x85aZ\x0EV[\x93P\x7F\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15ai|Wai{ai1V[[` \x83\x02\x92Pai\x8D\x83\x85\x84ai5V[\x82\x84\x01\x90P\x93\x92PPPV[_`\xA0\x82\x01\x90Pai\xAC_\x83\x01\x8AaO\xE9V[\x81\x81\x03` \x83\x01Rai\xBF\x81\x88\x8Aai>V[\x90P\x81\x81\x03`@\x83\x01Rai\xD4\x81\x86\x88ai>V[\x90Pai\xE3``\x83\x01\x85aO\xE9V[ai\xF0`\x80\x83\x01\x84aO\xE9V[\x98\x97PPPPPPPPV[_``\x82\x01\x90Paj\x0F_\x83\x01\x87aO\xE9V[\x81\x81\x03` \x83\x01Raj\"\x81\x85\x87af6V[\x90Paj1`@\x83\x01\x84aY\xF5V[\x95\x94PPPPPV[_``\x82\x01\x90PajM_\x83\x01\x86aO\xE9V[ajZ` \x83\x01\x85aO\xE9V[\x81\x81\x03`@\x83\x01Rajl\x81\x84a^\x12V[\x90P\x94\x93PPPPV[_`\x80\x82\x01\x90Paj\x89_\x83\x01\x89aO\xE9V[\x81\x81\x03` \x83\x01Raj\x9C\x81\x87\x89af6V[\x90P\x81\x81\x03`@\x83\x01Raj\xB1\x81\x85\x87af6V[\x90Paj\xC0``\x83\x01\x84aY\xF5V[\x97\x96PPPPPPPV[_``\x82\x01\x90Paj\xDE_\x83\x01\x87aO\xE9V[\x81\x81\x03` \x83\x01Raj\xF0\x81\x86a[\xCDV[\x90P\x81\x81\x03`@\x83\x01Rak\x05\x81\x84\x86af6V[\x90P\x95\x94PPPPPV[_\x81\x90P\x91\x90PV[_ak$\x83\x85a[fV[\x93Pak1\x83\x85\x84aVqV[ak:\x83aPSV[\x84\x01\x90P\x93\x92PPPV[_akQ\x84\x84\x84ak\x19V[\x90P\x93\x92PPPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12akvWakuad\xA0V[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15ak\x9EWak\x9Dad\x98V[[`\x01\x82\x026\x03\x83\x13\x15ak\xB4Wak\xB3ad\x9CV[[P\x92P\x92\x90PV[_` \x82\x01\x90P\x91\x90PV[_ak\xD3\x83\x85a[GV[\x93P\x83` \x84\x02\x85\x01ak\xE5\x84ak\x10V[\x80_[\x87\x81\x10\x15al*W\x84\x84\x03\x89Rak\xFF\x82\x84akZV[al\n\x86\x82\x84akEV[\x95Pal\x15\x84ak\xBCV[\x93P` \x8B\x01\x9APPP`\x01\x81\x01\x90Pak\xE8V[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_`\x80\x82\x01\x90PalO_\x83\x01\x89aO\xE9V[\x81\x81\x03` \x83\x01Ralb\x81\x87\x89ak\xC8V[\x90P\x81\x81\x03`@\x83\x01Ralw\x81\x85\x87ae\xC4V[\x90Pal\x86``\x83\x01\x84aO\xE9V[\x97\x96PPPPPPPV[\x7FEIP712: Uninitialized\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_al\xC5`\x15\x83aP\x1BV[\x91Pal\xD0\x82al\x91V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ral\xF2\x81al\xB9V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_`\xFF\x82\x16\x90P\x91\x90PV[_\x81`\xF8\x1B\x90P\x91\x90PV[_amH\x82am2V[\x90P\x91\x90PV[am`am[\x82am&V[am>V[\x82RPPV[_\x81\x90P\x91\x90PV[am\x80am{\x82aO\xE0V[amfV[\x82RPPV[_am\x91\x82\x85amOV[`\x01\x82\x01\x91Pam\xA1\x82\x84amoV[` \x82\x01\x91P\x81\x90P\x93\x92PPPV[am\xBA\x81aS\x15V[\x81\x14am\xC4W_\x80\xFD[PV[_\x81Q\x90Pam\xD5\x81am\xB1V[\x92\x91PPV[_` \x82\x84\x03\x12\x15am\xF0Wam\xEFaP\xC4V[[_am\xFD\x84\x82\x85\x01am\xC7V[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15an\x1BWan\x1AaP\xC4V[[_an(\x84\x82\x85\x01adnV[\x91PP\x92\x91PPV[_\x81\x90P\x92\x91PPV[_anF\x83\x85an1V[\x93PanS\x83\x85\x84aVqV[\x82\x84\x01\x90P\x93\x92PPPV[_ank\x82\x84\x86an;V[\x91P\x81\x90P\x93\x92PPPV[an\x80\x81a\\\x8FV[\x82RPPV[_``\x82\x01\x90Pan\x99_\x83\x01\x86aWPV[an\xA6` \x83\x01\x85anwV[an\xB3`@\x83\x01\x84aWPV[\x94\x93PPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[an\xE7\x81aWGV[\x82RPPV[_an\xF8\x83\x83an\xDEV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_ao\x1A\x82an\xBBV[ao$\x81\x85an\xC5V[\x93Pao/\x83an\xCFV[\x80_[\x83\x81\x10\x15ao_W\x81QaoF\x88\x82an\xEDV[\x97PaoQ\x83ao\x04V[\x92PP`\x01\x81\x01\x90Pao2V[P\x85\x93PPPP\x92\x91PPV[_aow\x82\x84ao\x10V[\x91P\x81\x90P\x92\x91PPV[_`\xA0\x82\x01\x90Pao\x95_\x83\x01\x88aWPV[ao\xA2` \x83\x01\x87aO\xE9V[ao\xAF`@\x83\x01\x86aO\xE9V[ao\xBC``\x83\x01\x85aWPV[ao\xC9`\x80\x83\x01\x84aWPV[\x96\x95PPPPPPV[_\x80\xFD[_\x80\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15ao\xF5Wao\xF4aU\xC9V[[ao\xFE\x82aPSV[\x90P` \x81\x01\x90P\x91\x90PV[_ap\x1Dap\x18\x84ao\xDBV[aV'V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15ap9Wap8aU\xC5V[[apD\x84\x82\x85aP+V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12ap`Wap_aSHV[[\x81Qapp\x84\x82` \x86\x01ap\x0BV[\x91PP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15ap\x8EWap\x8Dao\xD3V[[ap\x98`\x80aV'V[\x90P_ap\xA7\x84\x82\x85\x01a_\xD6V[_\x83\x01RP` ap\xBA\x84\x82\x85\x01a_\xD6V[` \x83\x01RP`@\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ap\xDEWap\xDDao\xD7V[[ap\xEA\x84\x82\x85\x01apLV[`@\x83\x01RP``\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aq\x0EWaq\rao\xD7V[[aq\x1A\x84\x82\x85\x01apLV[``\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15aq;Waq:aP\xC4V[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aqXWaqWaP\xC8V[[aqd\x84\x82\x85\x01apyV[\x91PP\x92\x91PPV[_`\x80\x82\x01\x90Paq\x80_\x83\x01\x88aO\xE9V[\x81\x81\x03` \x83\x01Raq\x92\x81\x87a[\xCDV[\x90P\x81\x81\x03`@\x83\x01Raq\xA7\x81\x85\x87ae\xC4V[\x90Paq\xB6``\x83\x01\x84aO\xE9V[\x96\x95PPPPPPV[_``\x82\x01\x90Paq\xD3_\x83\x01\x87aO\xE9V[\x81\x81\x03` \x83\x01Raq\xE5\x81\x86a[\xCDV[\x90P\x81\x81\x03`@\x83\x01Raq\xFA\x81\x84\x86ae\xC4V[\x90P\x95\x94PPPPPV[ar\x0E\x81aWGV[\x81\x14ar\x18W_\x80\xFD[PV[_\x81Q\x90Par)\x81ar\x05V[\x92\x91PPV[_` \x82\x84\x03\x12\x15arDWarCaP\xC4V[[_arQ\x84\x82\x85\x01ar\x1BV[\x91PP\x92\x91PPV[_``\x82\x01\x90Parm_\x83\x01\x86aWPV[arz` \x83\x01\x85aO\xE9V[ar\x87`@\x83\x01\x84aWPV[\x94\x93PPPPV[_``\x82\x01\x90Par\xA2_\x83\x01\x86aO\xE9V[ar\xAF` \x83\x01\x85aR\x8CV[\x81\x81\x03`@\x83\x01Rar\xC1\x81\x84a^\x12V[\x90P\x94\x93PPPPV[ar\xD4\x81am&V[\x82RPPV[_` \x82\x01\x90Par\xED_\x83\x01\x84ar\xCBV[\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[`\x1F\x82\x11\x15asFWas\x17\x81ar\xF3V[as \x84aa\x8BV[\x81\x01` \x85\x10\x15as/W\x81\x90P[asCas;\x85aa\x8BV[\x83\x01\x82abkV[PP[PPPV[asT\x82aP\x11V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15asmWaslaU\xC9V[[asw\x82TaaIV[as\x82\x82\x82\x85as\x05V[_` \x90P`\x1F\x83\x11`\x01\x81\x14as\xB3W_\x84\x15as\xA1W\x82\x87\x01Q\x90P[as\xAB\x85\x82ab\xFBV[\x86UPat\x12V[`\x1F\x19\x84\x16as\xC1\x86ar\xF3V[_[\x82\x81\x10\x15as\xE8W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pas\xC3V[\x86\x83\x10\x15at\x05W\x84\x89\x01Qat\x01`\x1F\x89\x16\x82ab\xDFV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_`@\x82\x01\x90Pat-_\x83\x01\x85aY\xF5V[at:` \x83\x01\x84aY\xF5V[\x93\x92PPPV[_atK\x82a\\\xAFV[atU\x81\x85an1V[\x93Pate\x81\x85` \x86\x01aP+V[\x80\x84\x01\x91PP\x92\x91PPV[_at|\x82\x84atAV[\x91P\x81\x90P\x92\x91PPV[_`\xA0\x82\x01\x90Pat\x9A_\x83\x01\x88aWPV[at\xA7` \x83\x01\x87aWPV[at\xB4`@\x83\x01\x86aWPV[at\xC1``\x83\x01\x85aO\xE9V[at\xCE`\x80\x83\x01\x84aY\xF5V[\x96\x95PPPPPPV[_`\x80\x82\x01\x90Pat\xEB_\x83\x01\x87aWPV[at\xF8` \x83\x01\x86ar\xCBV[au\x05`@\x83\x01\x85aWPV[au\x12``\x83\x01\x84aWPV[\x95\x94PPPPPV\xFEPrepKeygenVerification(uint256 prepKeygenId,bytes extraData)CrsgenVerification(uint256 crsId,uint256 maxBitLength,bytes crsDigest,bytes extraData)KeygenVerification(uint256 prepKeygenId,uint256 keyId,KeyDigest[] keyDigests,bytes extraData)KeyDigest(uint8 keyType,bytes digest)KeyDigest(uint8 keyType,bytes digest)",
    );
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
error AbortKeygenAlreadyDone(uint256 prepKeygenId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AbortKeygenAlreadyDone {
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
        impl ::core::convert::From<AbortKeygenAlreadyDone> for UnderlyingRustTuple<'_> {
            fn from(value: AbortKeygenAlreadyDone) -> Self {
                (value.prepKeygenId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AbortKeygenAlreadyDone {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { prepKeygenId: tuple.0 }
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
    /**Custom error with signature `AbortKeygenInvalidId(uint256)` and selector `0xfcf2db7a`.
```solidity
error AbortKeygenInvalidId(uint256 prepKeygenId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AbortKeygenInvalidId {
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
        impl ::core::convert::From<AbortKeygenInvalidId> for UnderlyingRustTuple<'_> {
            fn from(value: AbortKeygenInvalidId) -> Self {
                (value.prepKeygenId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AbortKeygenInvalidId {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { prepKeygenId: tuple.0 }
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
    /**Custom error with signature `CrsAborted(uint256)` and selector `0xd5fd3cd7`.
```solidity
error CrsAborted(uint256 crsId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CrsAborted {
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
        impl ::core::convert::From<CrsAborted> for UnderlyingRustTuple<'_> {
            fn from(value: CrsAborted) -> Self {
                (value.crsId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CrsAborted {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { crsId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CrsAborted {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CrsAborted(uint256)";
            const SELECTOR: [u8; 4] = [213u8, 253u8, 60u8, 215u8];
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
    /**Custom error with signature `KeyAborted(uint256)` and selector `0x83f18335`.
```solidity
error KeyAborted(uint256 keyId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KeyAborted {
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
        impl ::core::convert::From<KeyAborted> for UnderlyingRustTuple<'_> {
            fn from(value: KeyAborted) -> Self {
                (value.keyId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KeyAborted {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { keyId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KeyAborted {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KeyAborted(uint256)";
            const SELECTOR: [u8; 4] = [131u8, 241u8, 131u8, 53u8];
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
    /**Custom error with signature `KeyMaterialNotPublished(uint256)` and selector `0x05b083f2`.
```solidity
error KeyMaterialNotPublished(uint256 keyId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KeyMaterialNotPublished {
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
        impl ::core::convert::From<KeyMaterialNotPublished> for UnderlyingRustTuple<'_> {
            fn from(value: KeyMaterialNotPublished) -> Self {
                (value.keyId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KeyMaterialNotPublished {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { keyId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KeyMaterialNotPublished {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KeyMaterialNotPublished(uint256)";
            const SELECTOR: [u8; 4] = [5u8, 176u8, 131u8, 242u8];
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
    /**Custom error with signature `MismatchedMigrationArrays()` and selector `0x894b2ab3`.
```solidity
error MismatchedMigrationArrays();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MismatchedMigrationArrays;
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
        impl ::core::convert::From<MismatchedMigrationArrays>
        for UnderlyingRustTuple<'_> {
            fn from(value: MismatchedMigrationArrays) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for MismatchedMigrationArrays {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MismatchedMigrationArrays {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MismatchedMigrationArrays()";
            const SELECTOR: [u8; 4] = [137u8, 75u8, 42u8, 179u8];
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
    /**Custom error with signature `UnsupportedMaterialVersion(uint256)` and selector `0xa3e9ed7e`.
```solidity
error UnsupportedMaterialVersion(uint256 version);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnsupportedMaterialVersion {
        #[allow(missing_docs)]
        pub version: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<UnsupportedMaterialVersion>
        for UnderlyingRustTuple<'_> {
            fn from(value: UnsupportedMaterialVersion) -> Self {
                (value.version,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for UnsupportedMaterialVersion {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { version: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnsupportedMaterialVersion {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnsupportedMaterialVersion(uint256)";
            const SELECTOR: [u8; 4] = [163u8, 233u8, 237u8, 126u8];
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
event AbortKeygen(uint256 prepKeygenId);
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
                Self { prepKeygenId: data.0 }
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
    #[derive()]
    /**Event with signature `KeyMaterialAdded(uint256,string[],(uint8,bytes)[],uint256)` and selector `0xa47664861ab58c5bd5040e9cc45e68d0e48ec04371035fd75099e217e0a6aa81`.
```solidity
event KeyMaterialAdded(uint256 keyId, string[] kmsNodeStorageUrls, IKMSGeneration.KeyDigest[] keyDigests, uint256 materialVersion);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct KeyMaterialAdded {
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
        #[allow(missing_docs)]
        pub materialVersion: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for KeyMaterialAdded {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
                alloy::sol_types::sol_data::Array<IKMSGeneration::KeyDigest>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "KeyMaterialAdded(uint256,string[],(uint8,bytes)[],uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                164u8, 118u8, 100u8, 134u8, 26u8, 181u8, 140u8, 91u8, 213u8, 4u8, 14u8,
                156u8, 196u8, 94u8, 104u8, 208u8, 228u8, 142u8, 192u8, 67u8, 113u8, 3u8,
                95u8, 215u8, 80u8, 153u8, 226u8, 23u8, 224u8, 166u8, 170u8, 129u8,
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
                    materialVersion: data.3,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.materialVersion),
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
        impl alloy_sol_types::private::IntoLogData for KeyMaterialAdded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&KeyMaterialAdded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &KeyMaterialAdded) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `KeyMaterialMigrationScheduled(uint256,uint256[],uint256[],uint256,uint256)` and selector `0x8bfa7d0ed6f87e526b62342918ee7bfa53952badd463dc934054d7dd940eafdc`.
```solidity
event KeyMaterialMigrationScheduled(uint256 keyId, uint256[] hostChainIds, uint256[] hostMigrationBlocks, uint256 gatewayMigrationBlock, uint256 materialVersion);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct KeyMaterialMigrationScheduled {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub hostChainIds: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub hostMigrationBlocks: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub gatewayMigrationBlock: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub materialVersion: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for KeyMaterialMigrationScheduled {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "KeyMaterialMigrationScheduled(uint256,uint256[],uint256[],uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8, 250u8, 125u8, 14u8, 214u8, 248u8, 126u8, 82u8, 107u8, 98u8, 52u8,
                41u8, 24u8, 238u8, 123u8, 250u8, 83u8, 149u8, 43u8, 173u8, 212u8, 99u8,
                220u8, 147u8, 64u8, 84u8, 215u8, 221u8, 148u8, 14u8, 175u8, 220u8,
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
                    hostChainIds: data.1,
                    hostMigrationBlocks: data.2,
                    gatewayMigrationBlock: data.3,
                    materialVersion: data.4,
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
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.hostChainIds),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.hostMigrationBlocks),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.gatewayMigrationBlock,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.materialVersion),
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
        impl alloy_sol_types::private::IntoLogData for KeyMaterialMigrationScheduled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&KeyMaterialMigrationScheduled> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &KeyMaterialMigrationScheduled,
            ) -> alloy_sol_types::private::LogData {
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
function abortKeygen(uint256 prepKeygenId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct abortKeygenCall {
        #[allow(missing_docs)]
        pub prepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
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
                    (value.prepKeygenId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for abortKeygenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { prepKeygenId: tuple.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self.prepKeygenId),
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
    #[derive()]
    /**Function with signature `addKeyMaterials(uint256,(uint8,bytes)[],string[],uint256)` and selector `0x7fc80c5d`.
```solidity
function addKeyMaterials(uint256 keyId, IKMSGeneration.KeyDigest[] memory keyDigests, string[] memory kmsNodeStorageUrls, uint256 materialVersion) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addKeyMaterialsCall {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub keyDigests: alloy::sol_types::private::Vec<
            <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub kmsNodeStorageUrls: alloy::sol_types::private::Vec<
            alloy::sol_types::private::String,
        >,
        #[allow(missing_docs)]
        pub materialVersion: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`addKeyMaterials(uint256,(uint8,bytes)[],string[],uint256)`](addKeyMaterialsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addKeyMaterialsReturn {}
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
            impl ::core::convert::From<addKeyMaterialsCall> for UnderlyingRustTuple<'_> {
                fn from(value: addKeyMaterialsCall) -> Self {
                    (
                        value.keyId,
                        value.keyDigests,
                        value.kmsNodeStorageUrls,
                        value.materialVersion,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addKeyMaterialsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        keyId: tuple.0,
                        keyDigests: tuple.1,
                        kmsNodeStorageUrls: tuple.2,
                        materialVersion: tuple.3,
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
            impl ::core::convert::From<addKeyMaterialsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: addKeyMaterialsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addKeyMaterialsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl addKeyMaterialsReturn {
            fn _tokenize(
                &self,
            ) -> <addKeyMaterialsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addKeyMaterialsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IKMSGeneration::KeyDigest>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addKeyMaterialsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addKeyMaterials(uint256,(uint8,bytes)[],string[],uint256)";
            const SELECTOR: [u8; 4] = [127u8, 200u8, 12u8, 93u8];
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsNodeStorageUrls),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.materialVersion),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                addKeyMaterialsReturn::_tokenize(ret)
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
    /**Function with signature `getCrsCounter()` and selector `0x3ac50072`.
```solidity
function getCrsCounter() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCrsCounterCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCrsCounter()`](getCrsCounterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCrsCounterReturn {
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
            impl ::core::convert::From<getCrsCounterCall> for UnderlyingRustTuple<'_> {
                fn from(value: getCrsCounterCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCrsCounterCall {
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
            impl ::core::convert::From<getCrsCounterReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getCrsCounterReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCrsCounterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCrsCounterCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCrsCounter()";
            const SELECTOR: [u8; 4] = [58u8, 197u8, 0u8, 114u8];
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
                        let r: getCrsCounterReturn = r.into();
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
                        let r: getCrsCounterReturn = r.into();
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
    /**Function with signature `getKeyCounter()` and selector `0x0b680733`.
```solidity
function getKeyCounter() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeyCounterCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getKeyCounter()`](getKeyCounterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeyCounterReturn {
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
            impl ::core::convert::From<getKeyCounterCall> for UnderlyingRustTuple<'_> {
                fn from(value: getKeyCounterCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getKeyCounterCall {
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
            impl ::core::convert::From<getKeyCounterReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getKeyCounterReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getKeyCounterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getKeyCounterCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getKeyCounter()";
            const SELECTOR: [u8; 4] = [11u8, 104u8, 7u8, 51u8];
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
                        let r: getKeyCounterReturn = r.into();
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
                        let r: getKeyCounterReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getKeyMaterialVersion(uint256)` and selector `0xf0f8cbc6`.
```solidity
function getKeyMaterialVersion(uint256 keyId) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeyMaterialVersionCall {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getKeyMaterialVersion(uint256)`](getKeyMaterialVersionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeyMaterialVersionReturn {
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
            impl ::core::convert::From<getKeyMaterialVersionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKeyMaterialVersionCall) -> Self {
                    (value.keyId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKeyMaterialVersionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { keyId: tuple.0 }
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
            impl ::core::convert::From<getKeyMaterialVersionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKeyMaterialVersionReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKeyMaterialVersionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getKeyMaterialVersionCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getKeyMaterialVersion(uint256)";
            const SELECTOR: [u8; 4] = [240u8, 248u8, 203u8, 198u8];
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
                        let r: getKeyMaterialVersionReturn = r.into();
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
                        let r: getKeyMaterialVersionReturn = r.into();
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
    /**Function with signature `isRequestDone(uint256)` and selector `0x3d5ec7e3`.
```solidity
function isRequestDone(uint256 requestId) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRequestDoneCall {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isRequestDone(uint256)`](isRequestDoneCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRequestDoneReturn {
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
            impl ::core::convert::From<isRequestDoneCall> for UnderlyingRustTuple<'_> {
                fn from(value: isRequestDoneCall) -> Self {
                    (value.requestId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isRequestDoneCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { requestId: tuple.0 }
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
            impl ::core::convert::From<isRequestDoneReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isRequestDoneReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isRequestDoneReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isRequestDoneCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isRequestDone(uint256)";
            const SELECTOR: [u8; 4] = [61u8, 94u8, 199u8, 227u8];
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
                        let r: isRequestDoneReturn = r.into();
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
                        let r: isRequestDoneReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `keygen(uint8,bytes)` and selector `0x608b4ad5`.
```solidity
function keygen(IKMSGeneration.ParamsType paramsType, bytes memory extraData) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct keygen_0Call {
        #[allow(missing_docs)]
        pub paramsType: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub extraData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`keygen(uint8,bytes)`](keygen_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct keygen_0Return {}
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
                IKMSGeneration::ParamsType,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<keygen_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: keygen_0Call) -> Self {
                    (value.paramsType, value.extraData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for keygen_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        paramsType: tuple.0,
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
            impl ::core::convert::From<keygen_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: keygen_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for keygen_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl keygen_0Return {
            fn _tokenize(
                &self,
            ) -> <keygen_0Call as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for keygen_0Call {
            type Parameters<'a> = (
                IKMSGeneration::ParamsType,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = keygen_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "keygen(uint8,bytes)";
            const SELECTOR: [u8; 4] = [96u8, 139u8, 74u8, 213u8];
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                keygen_0Return::_tokenize(ret)
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
    pub struct keygen_1Call {
        #[allow(missing_docs)]
        pub paramsType: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`keygen(uint8)`](keygen_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct keygen_1Return {}
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
            impl ::core::convert::From<keygen_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: keygen_1Call) -> Self {
                    (value.paramsType,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for keygen_1Call {
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
            impl ::core::convert::From<keygen_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: keygen_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for keygen_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl keygen_1Return {
            fn _tokenize(
                &self,
            ) -> <keygen_1Call as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for keygen_1Call {
            type Parameters<'a> = (IKMSGeneration::ParamsType,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = keygen_1Return;
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
                keygen_1Return::_tokenize(ret)
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
    /**Function with signature `scheduleKeyMaterialMigration(uint256,uint256[],uint256[],uint256,uint256)` and selector `0x4c298229`.
```solidity
function scheduleKeyMaterialMigration(uint256 keyId, uint256[] memory hostChainIds, uint256[] memory hostMigrationBlocks, uint256 gatewayMigrationBlock, uint256 materialVersion) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scheduleKeyMaterialMigrationCall {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub hostChainIds: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub hostMigrationBlocks: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub gatewayMigrationBlock: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub materialVersion: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`scheduleKeyMaterialMigration(uint256,uint256[],uint256[],uint256,uint256)`](scheduleKeyMaterialMigrationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scheduleKeyMaterialMigrationReturn {}
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
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
            impl ::core::convert::From<scheduleKeyMaterialMigrationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: scheduleKeyMaterialMigrationCall) -> Self {
                    (
                        value.keyId,
                        value.hostChainIds,
                        value.hostMigrationBlocks,
                        value.gatewayMigrationBlock,
                        value.materialVersion,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for scheduleKeyMaterialMigrationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        keyId: tuple.0,
                        hostChainIds: tuple.1,
                        hostMigrationBlocks: tuple.2,
                        gatewayMigrationBlock: tuple.3,
                        materialVersion: tuple.4,
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
            impl ::core::convert::From<scheduleKeyMaterialMigrationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: scheduleKeyMaterialMigrationReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for scheduleKeyMaterialMigrationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl scheduleKeyMaterialMigrationReturn {
            fn _tokenize(
                &self,
            ) -> <scheduleKeyMaterialMigrationCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for scheduleKeyMaterialMigrationCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = scheduleKeyMaterialMigrationReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "scheduleKeyMaterialMigration(uint256,uint256[],uint256[],uint256,uint256)";
            const SELECTOR: [u8; 4] = [76u8, 41u8, 130u8, 41u8];
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
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.hostChainIds),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.hostMigrationBlocks),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.gatewayMigrationBlock,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.materialVersion),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                scheduleKeyMaterialMigrationReturn::_tokenize(ret)
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
        addKeyMaterials(addKeyMaterialsCall),
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
        getCrsCounter(getCrsCounterCall),
        #[allow(missing_docs)]
        getCrsMaterials(getCrsMaterialsCall),
        #[allow(missing_docs)]
        getCrsParamsType(getCrsParamsTypeCall),
        #[allow(missing_docs)]
        getKeyCounter(getKeyCounterCall),
        #[allow(missing_docs)]
        getKeyMaterialVersion(getKeyMaterialVersionCall),
        #[allow(missing_docs)]
        getKeyMaterials(getKeyMaterialsCall),
        #[allow(missing_docs)]
        getKeyParamsType(getKeyParamsTypeCall),
        #[allow(missing_docs)]
        getVersion(getVersionCall),
        #[allow(missing_docs)]
        initializeFromEmptyProxy(initializeFromEmptyProxyCall),
        #[allow(missing_docs)]
        isRequestDone(isRequestDoneCall),
        #[allow(missing_docs)]
        keygen_0(keygen_0Call),
        #[allow(missing_docs)]
        keygen_1(keygen_1Call),
        #[allow(missing_docs)]
        keygenResponse(keygenResponseCall),
        #[allow(missing_docs)]
        prepKeygenResponse(prepKeygenResponseCall),
        #[allow(missing_docs)]
        proxiableUUID(proxiableUUIDCall),
        #[allow(missing_docs)]
        reinitializeV2(reinitializeV2Call),
        #[allow(missing_docs)]
        scheduleKeyMaterialMigration(scheduleKeyMaterialMigrationCall),
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
            [11u8, 104u8, 7u8, 51u8],
            [13u8, 142u8, 110u8, 44u8],
            [22u8, 199u8, 19u8, 217u8],
            [23u8, 3u8, 198u8, 26u8],
            [25u8, 244u8, 246u8, 50u8],
            [57u8, 247u8, 56u8, 16u8],
            [58u8, 197u8, 0u8, 114u8],
            [60u8, 2u8, 248u8, 52u8],
            [61u8, 94u8, 199u8, 227u8],
            [69u8, 175u8, 38u8, 27u8],
            [70u8, 16u8, 255u8, 232u8],
            [76u8, 41u8, 130u8, 41u8],
            [79u8, 30u8, 242u8, 134u8],
            [82u8, 209u8, 144u8, 45u8],
            [88u8, 154u8, 219u8, 14u8],
            [96u8, 139u8, 74u8, 213u8],
            [98u8, 151u8, 135u8, 135u8],
            [127u8, 200u8, 12u8, 93u8],
            [132u8, 176u8, 25u8, 110u8],
            [147u8, 102u8, 8u8, 174u8],
            [173u8, 60u8, 177u8, 204u8],
            [186u8, 255u8, 33u8, 30u8],
            [194u8, 193u8, 250u8, 238u8],
            [196u8, 17u8, 88u8, 116u8],
            [197u8, 91u8, 135u8, 36u8],
            [202u8, 163u8, 103u8, 219u8],
            [213u8, 47u8, 16u8, 235u8],
            [240u8, 248u8, 203u8, 198u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for KMSGenerationCalls {
        const NAME: &'static str = "KMSGenerationCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 28usize;
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
                Self::addKeyMaterials(_) => {
                    <addKeyMaterialsCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::getCrsCounter(_) => {
                    <getCrsCounterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCrsMaterials(_) => {
                    <getCrsMaterialsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCrsParamsType(_) => {
                    <getCrsParamsTypeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKeyCounter(_) => {
                    <getKeyCounterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKeyMaterialVersion(_) => {
                    <getKeyMaterialVersionCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::isRequestDone(_) => {
                    <isRequestDoneCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::keygen_0(_) => <keygen_0Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::keygen_1(_) => <keygen_1Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::keygenResponse(_) => {
                    <keygenResponseCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::prepKeygenResponse(_) => {
                    <prepKeygenResponseCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proxiableUUID(_) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::reinitializeV2(_) => {
                    <reinitializeV2Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::scheduleKeyMaterialMigration(_) => {
                    <scheduleKeyMaterialMigrationCall as alloy_sol_types::SolCall>::SELECTOR
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
                    fn getKeyCounter(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getKeyCounterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::getKeyCounter)
                    }
                    getKeyCounter
                },
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
                    fn getCrsCounter(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getCrsCounterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::getCrsCounter)
                    }
                    getCrsCounter
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
                    fn isRequestDone(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <isRequestDoneCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::isRequestDone)
                    }
                    isRequestDone
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
                    fn scheduleKeyMaterialMigration(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <scheduleKeyMaterialMigrationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::scheduleKeyMaterialMigration)
                    }
                    scheduleKeyMaterialMigration
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
                    fn keygen_0(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <keygen_0Call as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(KMSGenerationCalls::keygen_0)
                    }
                    keygen_0
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
                    fn addKeyMaterials(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <addKeyMaterialsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::addKeyMaterials)
                    }
                    addKeyMaterials
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
                    fn reinitializeV2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <reinitializeV2Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::reinitializeV2)
                    }
                    reinitializeV2
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
                    fn keygen_1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <keygen_1Call as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(KMSGenerationCalls::keygen_1)
                    }
                    keygen_1
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
                    fn getKeyMaterialVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getKeyMaterialVersionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::getKeyMaterialVersion)
                    }
                    getKeyMaterialVersion
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
                    fn getKeyCounter(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getKeyCounterCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::getKeyCounter)
                    }
                    getKeyCounter
                },
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
                    fn getCrsCounter(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getCrsCounterCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::getCrsCounter)
                    }
                    getCrsCounter
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
                    fn isRequestDone(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <isRequestDoneCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::isRequestDone)
                    }
                    isRequestDone
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
                    fn scheduleKeyMaterialMigration(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <scheduleKeyMaterialMigrationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::scheduleKeyMaterialMigration)
                    }
                    scheduleKeyMaterialMigration
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
                    fn keygen_0(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <keygen_0Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::keygen_0)
                    }
                    keygen_0
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
                    fn addKeyMaterials(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <addKeyMaterialsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::addKeyMaterials)
                    }
                    addKeyMaterials
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
                    fn reinitializeV2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <reinitializeV2Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::reinitializeV2)
                    }
                    reinitializeV2
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
                    fn keygen_1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <keygen_1Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::keygen_1)
                    }
                    keygen_1
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
                    fn getKeyMaterialVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getKeyMaterialVersionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::getKeyMaterialVersion)
                    }
                    getKeyMaterialVersion
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
                Self::addKeyMaterials(inner) => {
                    <addKeyMaterialsCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::getCrsCounter(inner) => {
                    <getCrsCounterCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::getKeyCounter(inner) => {
                    <getKeyCounterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getKeyMaterialVersion(inner) => {
                    <getKeyMaterialVersionCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::isRequestDone(inner) => {
                    <isRequestDoneCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::keygen_0(inner) => {
                    <keygen_0Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::keygen_1(inner) => {
                    <keygen_1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::reinitializeV2(inner) => {
                    <reinitializeV2Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::scheduleKeyMaterialMigration(inner) => {
                    <scheduleKeyMaterialMigrationCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::addKeyMaterials(inner) => {
                    <addKeyMaterialsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getCrsCounter(inner) => {
                    <getCrsCounterCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getKeyCounter(inner) => {
                    <getKeyCounterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getKeyMaterialVersion(inner) => {
                    <getKeyMaterialVersionCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::isRequestDone(inner) => {
                    <isRequestDoneCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::keygen_0(inner) => {
                    <keygen_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::keygen_1(inner) => {
                    <keygen_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
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
                Self::reinitializeV2(inner) => {
                    <reinitializeV2Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::scheduleKeyMaterialMigration(inner) => {
                    <scheduleKeyMaterialMigrationCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
        CrsAborted(CrsAborted),
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
        KeyAborted(KeyAborted),
        #[allow(missing_docs)]
        KeyManagementRequestPending(KeyManagementRequestPending),
        #[allow(missing_docs)]
        KeyMaterialNotPublished(KeyMaterialNotPublished),
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
        MismatchedMigrationArrays(MismatchedMigrationArrays),
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
        UnsupportedExtraDataVersion(UnsupportedExtraDataVersion),
        #[allow(missing_docs)]
        UnsupportedMaterialVersion(UnsupportedMaterialVersion),
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
            [5u8, 176u8, 131u8, 242u8],
            [6u8, 26u8, 198u8, 29u8],
            [10u8, 183u8, 246u8, 135u8],
            [13u8, 134u8, 245u8, 33u8],
            [33u8, 57u8, 204u8, 44u8],
            [33u8, 191u8, 218u8, 16u8],
            [51u8, 202u8, 31u8, 227u8],
            [59u8, 133u8, 61u8, 168u8],
            [76u8, 156u8, 140u8, 227u8],
            [111u8, 79u8, 115u8, 31u8],
            [111u8, 188u8, 221u8, 43u8],
            [131u8, 241u8, 131u8, 53u8],
            [132u8, 222u8, 19u8, 49u8],
            [137u8, 75u8, 42u8, 179u8],
            [139u8, 36u8, 139u8, 96u8],
            [141u8, 140u8, 148u8, 10u8],
            [146u8, 120u8, 155u8, 103u8],
            [152u8, 251u8, 149u8, 125u8],
            [153u8, 150u8, 179u8, 21u8],
            [163u8, 233u8, 237u8, 126u8],
            [170u8, 29u8, 73u8, 164u8],
            [173u8, 250u8, 185u8, 4u8],
            [174u8, 232u8, 99u8, 35u8],
            [179u8, 152u8, 151u8, 159u8],
            [203u8, 233u8, 38u8, 86u8],
            [213u8, 253u8, 60u8, 215u8],
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
        const COUNT: usize = 38usize;
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
                Self::CrsAborted(_) => {
                    <CrsAborted as alloy_sol_types::SolError>::SELECTOR
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
                Self::KeyAborted(_) => {
                    <KeyAborted as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KeyManagementRequestPending(_) => {
                    <KeyManagementRequestPending as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KeyMaterialNotPublished(_) => {
                    <KeyMaterialNotPublished as alloy_sol_types::SolError>::SELECTOR
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
                Self::MismatchedMigrationArrays(_) => {
                    <MismatchedMigrationArrays as alloy_sol_types::SolError>::SELECTOR
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
                Self::UnsupportedExtraDataVersion(_) => {
                    <UnsupportedExtraDataVersion as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnsupportedMaterialVersion(_) => {
                    <UnsupportedMaterialVersion as alloy_sol_types::SolError>::SELECTOR
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
                    fn KeyMaterialNotPublished(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KeyMaterialNotPublished as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::KeyMaterialNotPublished)
                    }
                    KeyMaterialNotPublished
                },
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
                    fn KeyAborted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KeyAborted as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(KMSGenerationErrors::KeyAborted)
                    }
                    KeyAborted
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
                    fn MismatchedMigrationArrays(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <MismatchedMigrationArrays as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::MismatchedMigrationArrays)
                    }
                    MismatchedMigrationArrays
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
                    fn UnsupportedMaterialVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <UnsupportedMaterialVersion as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::UnsupportedMaterialVersion)
                    }
                    UnsupportedMaterialVersion
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
                    fn CrsAborted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <CrsAborted as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(KMSGenerationErrors::CrsAborted)
                    }
                    CrsAborted
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
                    fn KeyMaterialNotPublished(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KeyMaterialNotPublished as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::KeyMaterialNotPublished)
                    }
                    KeyMaterialNotPublished
                },
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
                    fn KeyAborted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KeyAborted as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::KeyAborted)
                    }
                    KeyAborted
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
                    fn MismatchedMigrationArrays(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <MismatchedMigrationArrays as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::MismatchedMigrationArrays)
                    }
                    MismatchedMigrationArrays
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
                    fn UnsupportedMaterialVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <UnsupportedMaterialVersion as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::UnsupportedMaterialVersion)
                    }
                    UnsupportedMaterialVersion
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
                    fn CrsAborted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <CrsAborted as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::CrsAborted)
                    }
                    CrsAborted
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
                Self::CrsAborted(inner) => {
                    <CrsAborted as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::KeyAborted(inner) => {
                    <KeyAborted as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::KeyManagementRequestPending(inner) => {
                    <KeyManagementRequestPending as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KeyMaterialNotPublished(inner) => {
                    <KeyMaterialNotPublished as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::MismatchedMigrationArrays(inner) => {
                    <MismatchedMigrationArrays as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::UnsupportedExtraDataVersion(inner) => {
                    <UnsupportedExtraDataVersion as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UnsupportedMaterialVersion(inner) => {
                    <UnsupportedMaterialVersion as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::CrsAborted(inner) => {
                    <CrsAborted as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
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
                Self::KeyAborted(inner) => {
                    <KeyAborted as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::KeyManagementRequestPending(inner) => {
                    <KeyManagementRequestPending as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KeyMaterialNotPublished(inner) => {
                    <KeyMaterialNotPublished as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::MismatchedMigrationArrays(inner) => {
                    <MismatchedMigrationArrays as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::UnsupportedExtraDataVersion(inner) => {
                    <UnsupportedExtraDataVersion as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UnsupportedMaterialVersion(inner) => {
                    <UnsupportedMaterialVersion as alloy_sol_types::SolError>::abi_encode_raw(
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
        KeyMaterialAdded(KeyMaterialAdded),
        #[allow(missing_docs)]
        KeyMaterialMigrationScheduled(KeyMaterialMigrationScheduled),
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
                139u8, 250u8, 125u8, 14u8, 214u8, 248u8, 126u8, 82u8, 107u8, 98u8, 52u8,
                41u8, 24u8, 238u8, 123u8, 250u8, 83u8, 149u8, 43u8, 173u8, 212u8, 99u8,
                220u8, 147u8, 64u8, 84u8, 215u8, 221u8, 148u8, 14u8, 175u8, 220u8,
            ],
            [
                140u8, 240u8, 21u8, 19u8, 147u8, 248u8, 79u8, 214u8, 148u8, 197u8, 227u8,
                21u8, 203u8, 116u8, 204u8, 5u8, 178u8, 71u8, 222u8, 10u8, 69u8, 79u8,
                217u8, 233u8, 18u8, 156u8, 102u8, 30u8, 253u8, 249u8, 64u8, 29u8,
            ],
            [
                164u8, 118u8, 100u8, 134u8, 26u8, 181u8, 140u8, 91u8, 213u8, 4u8, 14u8,
                156u8, 196u8, 94u8, 104u8, 208u8, 228u8, 142u8, 192u8, 67u8, 113u8, 3u8,
                95u8, 215u8, 80u8, 153u8, 226u8, 23u8, 224u8, 166u8, 170u8, 129u8,
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
        const COUNT: usize = 15usize;
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
                Some(<KeyMaterialAdded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <KeyMaterialAdded as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::KeyMaterialAdded)
                }
                Some(
                    <KeyMaterialMigrationScheduled as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <KeyMaterialMigrationScheduled as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::KeyMaterialMigrationScheduled)
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
                Self::KeyMaterialAdded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::KeyMaterialMigrationScheduled(inner) => {
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
                Self::KeyMaterialAdded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::KeyMaterialMigrationScheduled(inner) => {
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
            prepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, abortKeygenCall, N> {
            self.call_builder(&abortKeygenCall { prepKeygenId })
        }
        ///Creates a new call builder for the [`addKeyMaterials`] function.
        pub fn addKeyMaterials(
            &self,
            keyId: alloy::sol_types::private::primitives::aliases::U256,
            keyDigests: alloy::sol_types::private::Vec<
                <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
            >,
            kmsNodeStorageUrls: alloy::sol_types::private::Vec<
                alloy::sol_types::private::String,
            >,
            materialVersion: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, addKeyMaterialsCall, N> {
            self.call_builder(
                &addKeyMaterialsCall {
                    keyId,
                    keyDigests,
                    kmsNodeStorageUrls,
                    materialVersion,
                },
            )
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
        ///Creates a new call builder for the [`getCrsCounter`] function.
        pub fn getCrsCounter(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getCrsCounterCall, N> {
            self.call_builder(&getCrsCounterCall)
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
        ///Creates a new call builder for the [`getKeyCounter`] function.
        pub fn getKeyCounter(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getKeyCounterCall, N> {
            self.call_builder(&getKeyCounterCall)
        }
        ///Creates a new call builder for the [`getKeyMaterialVersion`] function.
        pub fn getKeyMaterialVersion(
            &self,
            keyId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getKeyMaterialVersionCall, N> {
            self.call_builder(&getKeyMaterialVersionCall { keyId })
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
        ///Creates a new call builder for the [`isRequestDone`] function.
        pub fn isRequestDone(
            &self,
            requestId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, isRequestDoneCall, N> {
            self.call_builder(&isRequestDoneCall { requestId })
        }
        ///Creates a new call builder for the [`keygen_0`] function.
        pub fn keygen_0(
            &self,
            paramsType: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
            extraData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, keygen_0Call, N> {
            self.call_builder(
                &keygen_0Call {
                    paramsType,
                    extraData,
                },
            )
        }
        ///Creates a new call builder for the [`keygen_1`] function.
        pub fn keygen_1(
            &self,
            paramsType: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, keygen_1Call, N> {
            self.call_builder(&keygen_1Call { paramsType })
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
        ///Creates a new call builder for the [`reinitializeV2`] function.
        pub fn reinitializeV2(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, reinitializeV2Call, N> {
            self.call_builder(&reinitializeV2Call)
        }
        ///Creates a new call builder for the [`scheduleKeyMaterialMigration`] function.
        pub fn scheduleKeyMaterialMigration(
            &self,
            keyId: alloy::sol_types::private::primitives::aliases::U256,
            hostChainIds: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            hostMigrationBlocks: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            gatewayMigrationBlock: alloy::sol_types::private::primitives::aliases::U256,
            materialVersion: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, scheduleKeyMaterialMigrationCall, N> {
            self.call_builder(
                &scheduleKeyMaterialMigrationCall {
                    keyId,
                    hostChainIds,
                    hostMigrationBlocks,
                    gatewayMigrationBlock,
                    materialVersion,
                },
            )
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
        ///Creates a new event filter for the [`KeyMaterialAdded`] event.
        pub fn KeyMaterialAdded_filter(
            &self,
        ) -> alloy_contract::Event<&P, KeyMaterialAdded, N> {
            self.event_filter::<KeyMaterialAdded>()
        }
        ///Creates a new event filter for the [`KeyMaterialMigrationScheduled`] event.
        pub fn KeyMaterialMigrationScheduled_filter(
            &self,
        ) -> alloy_contract::Event<&P, KeyMaterialMigrationScheduled, N> {
            self.event_filter::<KeyMaterialMigrationScheduled>()
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
