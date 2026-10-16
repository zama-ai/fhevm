///Module containing a contract's types and functions.
/**

```solidity
library IKMSGeneration {
    type KeyType is uint8;
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
///Module containing a contract's types and functions.
/**

```solidity
library IProtocolConfig {
    struct EpochCrsResult { uint256 crsId; uint256 maxBitLength; bytes crsDigest; }
    struct EpochKeyResult { uint256 prepKeygenId; uint256 keyId; IKMSGeneration.KeyDigest[] keyDigests; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IProtocolConfig {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct EpochCrsResult { uint256 crsId; uint256 maxBitLength; bytes crsDigest; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EpochCrsResult {
        #[allow(missing_docs)]
        pub crsId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub maxBitLength: alloy::sol_types::private::primitives::aliases::U256,
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
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<EpochCrsResult> for UnderlyingRustTuple<'_> {
            fn from(value: EpochCrsResult) -> Self {
                (value.crsId, value.maxBitLength, value.crsDigest)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EpochCrsResult {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    crsId: tuple.0,
                    maxBitLength: tuple.1,
                    crsDigest: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for EpochCrsResult {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for EpochCrsResult {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.crsId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxBitLength),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.crsDigest,
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
        impl alloy_sol_types::SolType for EpochCrsResult {
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
        impl alloy_sol_types::SolStruct for EpochCrsResult {
            const NAME: &'static str = "EpochCrsResult";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "EpochCrsResult(uint256 crsId,uint256 maxBitLength,bytes crsDigest)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.crsId)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.maxBitLength)
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.crsDigest,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for EpochCrsResult {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.crsId)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.maxBitLength,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.crsDigest,
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
                    &rust.crsId,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.maxBitLength,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.crsDigest,
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
    #[derive()]
    /**```solidity
struct EpochKeyResult { uint256 prepKeygenId; uint256 keyId; IKMSGeneration.KeyDigest[] keyDigests; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EpochKeyResult {
        #[allow(missing_docs)]
        pub prepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
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
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Array<IKMSGeneration::KeyDigest>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<EpochKeyResult> for UnderlyingRustTuple<'_> {
            fn from(value: EpochKeyResult) -> Self {
                (value.prepKeygenId, value.keyId, value.keyDigests)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EpochKeyResult {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    prepKeygenId: tuple.0,
                    keyId: tuple.1,
                    keyDigests: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for EpochKeyResult {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for EpochKeyResult {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.prepKeygenId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                    <alloy::sol_types::sol_data::Array<
                        IKMSGeneration::KeyDigest,
                    > as alloy_sol_types::SolType>::tokenize(&self.keyDigests),
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
        impl alloy_sol_types::SolType for EpochKeyResult {
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
        impl alloy_sol_types::SolStruct for EpochKeyResult {
            const NAME: &'static str = "EpochKeyResult";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "EpochKeyResult(uint256 prepKeygenId,uint256 keyId,IKMSGeneration.KeyDigest[] keyDigests)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.prepKeygenId)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.keyId)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        IKMSGeneration::KeyDigest,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.keyDigests)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for EpochKeyResult {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.prepKeygenId,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.keyId)
                    + <alloy::sol_types::sol_data::Array<
                        IKMSGeneration::KeyDigest,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.keyDigests,
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
                    &rust.prepKeygenId,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.keyId,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    IKMSGeneration::KeyDigest,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.keyDigests,
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
    /**Creates a new wrapper around an on-chain [`IProtocolConfig`](self) contract instance.

See the [wrapper's documentation](`IProtocolConfigInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> IProtocolConfigInstance<P, N> {
        IProtocolConfigInstance::<P, N>::new(address, __provider)
    }
    /**A [`IProtocolConfig`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IProtocolConfig`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IProtocolConfigInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IProtocolConfigInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IProtocolConfigInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IProtocolConfigInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`IProtocolConfig`](self) contract instance.

See the [wrapper's documentation](`IProtocolConfigInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> IProtocolConfigInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IProtocolConfigInstance<P, N> {
            IProtocolConfigInstance {
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
    > IProtocolConfigInstance<P, N> {
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
    > IProtocolConfigInstance<P, N> {
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
    struct KeyDigest {
        KeyType keyType;
        bytes digest;
    }
}

library IProtocolConfig {
    struct EpochCrsResult {
        uint256 crsId;
        uint256 maxBitLength;
        bytes crsDigest;
    }
    struct EpochKeyResult {
        uint256 prepKeygenId;
        uint256 keyId;
        IKMSGeneration.KeyDigest[] keyDigests;
    }
}

interface ProtocolConfig {
    struct ChainUpgradeWindow {
        uint64 chainId;
        uint64 startBlock;
        uint64 endBlock;
    }
    struct KmsNode {
        address txSenderAddress;
        address signerAddress;
        string ipAddress;
        string storageUrl;
    }
    struct KmsNodeParams {
        address txSenderAddress;
        address signerAddress;
        string ipAddress;
        string storageUrl;
        int32 partyId;
        string mpcIdentity;
        bytes caCert;
        string storagePrefix;
    }
    struct KmsThresholds {
        uint256 publicDecryption;
        uint256 userDecryption;
        uint256 kmsGen;
        uint256 mpc;
    }
    struct PcrValues {
        bytes pcr0;
        bytes pcr1;
        bytes pcr2;
    }

    error AddressEmptyCode(address target);
    error DuplicateChainId(uint64 chainId);
    error ECDSAInvalidSignature();
    error ECDSAInvalidSignatureLength(uint256 length);
    error ECDSAInvalidSignatureS(bytes32 s);
    error ERC1967InvalidImplementation(address implementation);
    error ERC1967NonPayable();
    error EmptyChainUpgradeWindows();
    error EmptyEpochActivationResults(uint256 epochId);
    error EmptyKmsNodes();
    error EmptySoftwareVersion();
    error EpochActivationAlreadyConfirmed(address signer, uint256 epochId);
    error EpochActivationUnauthorized(address signer, uint256 epochId);
    error FailedCall();
    error InvalidBlockWindow(uint64 chainId, uint64 startBlock, uint64 endBlock);
    error InvalidHighThreshold(string thresholdName, uint256 threshold, uint256 nodeCount);
    error InvalidInitialization();
    error InvalidKmsContext(uint256 kmsContextId);
    error InvalidKmsEpoch(uint256 epochId);
    error InvalidNullThreshold(string thresholdName);
    error InvalidProposalId();
    error KmsContextCreationAlreadyConfirmed(address signer, uint256 kmsContextId);
    error KmsContextCreationUnauthorized(address signer, uint256 kmsContextId);
    error KmsContextNotDestroyed(uint256 kmsContextId);
    error KmsContextNotPending(uint256 kmsContextId);
    error KmsDestructionAlreadyConfirmed(address signer, uint256 destroyedId);
    error KmsDestructionUnauthorized(address signer, uint256 destroyedId);
    error KmsEpochNotDestroyed(uint256 epochId);
    error KmsLifecycleOperationInFlight(uint256 kmsContextId, uint256 epochId);
    error KmsNodeNullSigner();
    error KmsNodeNullTxSender();
    error KmsSignerAlreadyRegistered(address signer);
    error KmsSignerSetExceedsProofFormatLimit(uint256 signerCount, uint256 maxAllowed);
    error KmsTxSenderAlreadyRegistered(address txSender);
    error LatestActiveKmsContextCannotBeDestroyed(uint256 kmsContextId);
    error LatestActiveKmsEpochCannotBeDestroyed(uint256 epochId);
    error NonIncreasingKmsContextId(uint256 contextId, uint256 latestActiveKmsContextId);
    error NotHostOwner(address sender);
    error NotInitializing();
    error NotInitializingFromEmptyProxy();
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);
    error ZeroChainId();
    error ZeroGwStartBlock();

    event ActivateEpoch(uint256 indexed kmsContextId, uint256 indexed epochId, IProtocolConfig.EpochKeyResult[] keys, IProtocolConfig.EpochCrsResult[] crsList, string[] kmsNodeStorageUrls);
    event CoprocessorUpgradeProposed(uint256 indexed proposalId, string softwareVersion, ChainUpgradeWindow[] chainUpgradeWindows, uint64 gwStartBlock);
    event EpochActivationConfirmation(uint256 indexed epochId, address indexed signer, bytes32 epochMaterialHash, bytes signature, bytes extraData);
    event Initialized(uint64 version);
    event KmsContextCreationConfirmation(uint256 indexed kmsContextId, address indexed signer, bytes signature, bytes extraData);
    event KmsContextDestroyed(uint256 indexed kmsContextId);
    event KmsContextDestructionConfirmation(uint256 indexed destroyedContextId, uint256[] destroyedEpochIds, address indexed signer, bytes signature, bytes extraData);
    event KmsEpochDestroyed(uint256 indexed epochId);
    event KmsEpochDestructionConfirmation(uint256 indexed destroyedEpochId, address indexed signer, bytes signature, bytes extraData);
    event NewKmsContext(uint256 indexed contextId, uint256 indexed previousContextId, KmsNodeParams[] kmsNodeParams, KmsThresholds thresholds, string softwareVersion, PcrValues[] pcrValues);
    event NewKmsEpoch(uint256 indexed kmsContextId, uint256 indexed epochId, uint256 previousContextId, uint256 previousEpochId, uint256 materialBlockNumber);
    event Upgraded(address indexed implementation);

    constructor();

    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function confirmEpochActivation(uint256 epochId, IProtocolConfig.EpochKeyResult[] memory keys, IProtocolConfig.EpochCrsResult[] memory crsList, bytes memory signature, bytes memory extraData) external;
    function confirmKmsContextCreation(uint256 kmsContextId, bytes memory signature, bytes memory extraData) external;
    function confirmKmsContextDestruction(uint256 destroyedContextId, uint256[] memory destroyedEpochIds, bytes memory signature, bytes memory extraData) external;
    function confirmKmsEpochDestruction(uint256 destroyedEpochId, bytes memory signature, bytes memory extraData) external;
    function defineNewEpochForCurrentKmsContext() external;
    function defineNewKmsContextAndEpoch(KmsNodeParams[] memory kmsNodeParams, KmsThresholds memory thresholds, string memory softwareVersion, PcrValues[] memory pcrValues) external;
    function destroyKmsContext(uint256 kmsContextId) external;
    function destroyKmsEpoch(uint256 epochId) external;
    function getCurrentKmsContextAndEpoch() external view returns (uint256 contextId, uint256 epochId);
    function getCurrentKmsContextId() external view returns (uint256);
    function getCurrentKmsContextIdCounter() external view returns (uint256);
    function getKmsContextAnchor(uint256 contextId) external view returns (uint256 emissionBlockNumber, bytes32 contextInfoHash);
    function getKmsGenThreshold() external view returns (uint256);
    function getKmsGenThresholdForContext(uint256 kmsContextId) external view returns (uint256);
    function getKmsNodeForContext(uint256 kmsContextId, address txSender) external view returns (KmsNode memory);
    function getKmsNodesForContext(uint256 kmsContextId) external view returns (KmsNode[] memory);
    function getKmsSigners() external view returns (address[] memory);
    function getKmsSignersForContext(uint256 kmsContextId) external view returns (address[] memory);
    function getMpcThreshold() external view returns (uint256);
    function getMpcThresholdForContext(uint256 kmsContextId) external view returns (uint256);
    function getPublicDecryptionThreshold() external view returns (uint256);
    function getPublicDecryptionThresholdForContext(uint256 kmsContextId) external view returns (uint256);
    function getUserDecryptionThreshold() external view returns (uint256);
    function getUserDecryptionThresholdForContext(uint256 kmsContextId) external view returns (uint256);
    function getVersion() external pure returns (string memory);
    function initializeFromEmptyProxy(KmsNodeParams[] memory initialKmsNodeParams, KmsThresholds memory initialThresholds, string memory softwareVersion, PcrValues[] memory pcrValues) external;
    function isKmsSigner(address signer) external view returns (bool);
    function isKmsSignerForContext(uint256 kmsContextId, address signer) external view returns (bool);
    function isKmsTxSenderForContext(uint256 kmsContextId, address txSender) external view returns (bool);
    function isLiveKmsContext(uint256 kmsContextId) external view returns (bool);
    function isValidEpochForContext(uint256 kmsContextId, uint256 epochId) external view returns (bool);
    function isValidKmsContext(uint256 kmsContextId) external view returns (bool);
    function proposeCoprocessorUpgrade(uint256 proposalId, string memory softwareVersion, ChainUpgradeWindow[] memory chainUpgradeWindows, uint64 gwStartBlock) external;
    function proxiableUUID() external view returns (bytes32);
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
    "name": "confirmEpochActivation",
    "inputs": [
      {
        "name": "epochId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "keys",
        "type": "tuple[]",
        "internalType": "struct IProtocolConfig.EpochKeyResult[]",
        "components": [
          {
            "name": "prepKeygenId",
            "type": "uint256",
            "internalType": "uint256"
          },
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
          }
        ]
      },
      {
        "name": "crsList",
        "type": "tuple[]",
        "internalType": "struct IProtocolConfig.EpochCrsResult[]",
        "components": [
          {
            "name": "crsId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "maxBitLength",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "crsDigest",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
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
    "name": "confirmKmsContextCreation",
    "inputs": [
      {
        "name": "kmsContextId",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "confirmKmsContextDestruction",
    "inputs": [
      {
        "name": "destroyedContextId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "destroyedEpochIds",
        "type": "uint256[]",
        "internalType": "uint256[]"
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
    "name": "confirmKmsEpochDestruction",
    "inputs": [
      {
        "name": "destroyedEpochId",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "defineNewEpochForCurrentKmsContext",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "defineNewKmsContextAndEpoch",
    "inputs": [
      {
        "name": "kmsNodeParams",
        "type": "tuple[]",
        "internalType": "struct KmsNodeParams[]",
        "components": [
          {
            "name": "txSenderAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "signerAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "ipAddress",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "storageUrl",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "partyId",
            "type": "int32",
            "internalType": "int32"
          },
          {
            "name": "mpcIdentity",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "caCert",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "storagePrefix",
            "type": "string",
            "internalType": "string"
          }
        ]
      },
      {
        "name": "thresholds",
        "type": "tuple",
        "internalType": "struct KmsThresholds",
        "components": [
          {
            "name": "publicDecryption",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "userDecryption",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "kmsGen",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "mpc",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "softwareVersion",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "pcrValues",
        "type": "tuple[]",
        "internalType": "struct PcrValues[]",
        "components": [
          {
            "name": "pcr0",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "pcr1",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "pcr2",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "destroyKmsContext",
    "inputs": [
      {
        "name": "kmsContextId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "destroyKmsEpoch",
    "inputs": [
      {
        "name": "epochId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getCurrentKmsContextAndEpoch",
    "inputs": [],
    "outputs": [
      {
        "name": "contextId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "epochId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCurrentKmsContextId",
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
    "name": "getCurrentKmsContextIdCounter",
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
    "name": "getKmsContextAnchor",
    "inputs": [
      {
        "name": "contextId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "emissionBlockNumber",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "contextInfoHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getKmsGenThreshold",
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
    "name": "getKmsGenThresholdForContext",
    "inputs": [
      {
        "name": "kmsContextId",
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
    "name": "getKmsNodeForContext",
    "inputs": [
      {
        "name": "kmsContextId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "txSender",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct KmsNode",
        "components": [
          {
            "name": "txSenderAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "signerAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "ipAddress",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "storageUrl",
            "type": "string",
            "internalType": "string"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getKmsNodesForContext",
    "inputs": [
      {
        "name": "kmsContextId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct KmsNode[]",
        "components": [
          {
            "name": "txSenderAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "signerAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "ipAddress",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "storageUrl",
            "type": "string",
            "internalType": "string"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getKmsSigners",
    "inputs": [],
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
    "name": "getKmsSignersForContext",
    "inputs": [
      {
        "name": "kmsContextId",
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
    "name": "getMpcThreshold",
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
    "name": "getMpcThresholdForContext",
    "inputs": [
      {
        "name": "kmsContextId",
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
    "name": "getPublicDecryptionThreshold",
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
    "name": "getPublicDecryptionThresholdForContext",
    "inputs": [
      {
        "name": "kmsContextId",
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
    "name": "getUserDecryptionThreshold",
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
    "name": "getUserDecryptionThresholdForContext",
    "inputs": [
      {
        "name": "kmsContextId",
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
    "inputs": [
      {
        "name": "initialKmsNodeParams",
        "type": "tuple[]",
        "internalType": "struct KmsNodeParams[]",
        "components": [
          {
            "name": "txSenderAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "signerAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "ipAddress",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "storageUrl",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "partyId",
            "type": "int32",
            "internalType": "int32"
          },
          {
            "name": "mpcIdentity",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "caCert",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "storagePrefix",
            "type": "string",
            "internalType": "string"
          }
        ]
      },
      {
        "name": "initialThresholds",
        "type": "tuple",
        "internalType": "struct KmsThresholds",
        "components": [
          {
            "name": "publicDecryption",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "userDecryption",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "kmsGen",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "mpc",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "softwareVersion",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "pcrValues",
        "type": "tuple[]",
        "internalType": "struct PcrValues[]",
        "components": [
          {
            "name": "pcr0",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "pcr1",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "pcr2",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isKmsSigner",
    "inputs": [
      {
        "name": "signer",
        "type": "address",
        "internalType": "address"
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
    "name": "isKmsSignerForContext",
    "inputs": [
      {
        "name": "kmsContextId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "signer",
        "type": "address",
        "internalType": "address"
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
    "name": "isKmsTxSenderForContext",
    "inputs": [
      {
        "name": "kmsContextId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "txSender",
        "type": "address",
        "internalType": "address"
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
    "name": "isLiveKmsContext",
    "inputs": [
      {
        "name": "kmsContextId",
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
    "name": "isValidEpochForContext",
    "inputs": [
      {
        "name": "kmsContextId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "epochId",
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
    "name": "isValidKmsContext",
    "inputs": [
      {
        "name": "kmsContextId",
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
    "name": "proposeCoprocessorUpgrade",
    "inputs": [
      {
        "name": "proposalId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "softwareVersion",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "chainUpgradeWindows",
        "type": "tuple[]",
        "internalType": "struct ChainUpgradeWindow[]",
        "components": [
          {
            "name": "chainId",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "startBlock",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "endBlock",
            "type": "uint64",
            "internalType": "uint64"
          }
        ]
      },
      {
        "name": "gwStartBlock",
        "type": "uint64",
        "internalType": "uint64"
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
    "name": "ActivateEpoch",
    "inputs": [
      {
        "name": "kmsContextId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "epochId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "keys",
        "type": "tuple[]",
        "indexed": false,
        "internalType": "struct IProtocolConfig.EpochKeyResult[]",
        "components": [
          {
            "name": "prepKeygenId",
            "type": "uint256",
            "internalType": "uint256"
          },
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
          }
        ]
      },
      {
        "name": "crsList",
        "type": "tuple[]",
        "indexed": false,
        "internalType": "struct IProtocolConfig.EpochCrsResult[]",
        "components": [
          {
            "name": "crsId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "maxBitLength",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "crsDigest",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "kmsNodeStorageUrls",
        "type": "string[]",
        "indexed": false,
        "internalType": "string[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "CoprocessorUpgradeProposed",
    "inputs": [
      {
        "name": "proposalId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "softwareVersion",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "chainUpgradeWindows",
        "type": "tuple[]",
        "indexed": false,
        "internalType": "struct ChainUpgradeWindow[]",
        "components": [
          {
            "name": "chainId",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "startBlock",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "endBlock",
            "type": "uint64",
            "internalType": "uint64"
          }
        ]
      },
      {
        "name": "gwStartBlock",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "EpochActivationConfirmation",
    "inputs": [
      {
        "name": "epochId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "signer",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "epochMaterialHash",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
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
    "name": "KmsContextCreationConfirmation",
    "inputs": [
      {
        "name": "kmsContextId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "signer",
        "type": "address",
        "indexed": true,
        "internalType": "address"
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
    "name": "KmsContextDestroyed",
    "inputs": [
      {
        "name": "kmsContextId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "KmsContextDestructionConfirmation",
    "inputs": [
      {
        "name": "destroyedContextId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "destroyedEpochIds",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      },
      {
        "name": "signer",
        "type": "address",
        "indexed": true,
        "internalType": "address"
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
    "name": "KmsEpochDestroyed",
    "inputs": [
      {
        "name": "epochId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "KmsEpochDestructionConfirmation",
    "inputs": [
      {
        "name": "destroyedEpochId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "signer",
        "type": "address",
        "indexed": true,
        "internalType": "address"
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
    "name": "NewKmsContext",
    "inputs": [
      {
        "name": "contextId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "previousContextId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "kmsNodeParams",
        "type": "tuple[]",
        "indexed": false,
        "internalType": "struct KmsNodeParams[]",
        "components": [
          {
            "name": "txSenderAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "signerAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "ipAddress",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "storageUrl",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "partyId",
            "type": "int32",
            "internalType": "int32"
          },
          {
            "name": "mpcIdentity",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "caCert",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "storagePrefix",
            "type": "string",
            "internalType": "string"
          }
        ]
      },
      {
        "name": "thresholds",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct KmsThresholds",
        "components": [
          {
            "name": "publicDecryption",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "userDecryption",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "kmsGen",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "mpc",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "softwareVersion",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "pcrValues",
        "type": "tuple[]",
        "indexed": false,
        "internalType": "struct PcrValues[]",
        "components": [
          {
            "name": "pcr0",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "pcr1",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "pcr2",
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
    "name": "NewKmsEpoch",
    "inputs": [
      {
        "name": "kmsContextId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "epochId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "previousContextId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "previousEpochId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "materialBlockNumber",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
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
    "name": "DuplicateChainId",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint64",
        "internalType": "uint64"
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
    "name": "EmptyChainUpgradeWindows",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EmptyEpochActivationResults",
    "inputs": [
      {
        "name": "epochId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "EmptyKmsNodes",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EmptySoftwareVersion",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EpochActivationAlreadyConfirmed",
    "inputs": [
      {
        "name": "signer",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "epochId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "EpochActivationUnauthorized",
    "inputs": [
      {
        "name": "signer",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "epochId",
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
    "name": "InvalidBlockWindow",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "startBlock",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "endBlock",
        "type": "uint64",
        "internalType": "uint64"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidHighThreshold",
    "inputs": [
      {
        "name": "thresholdName",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "threshold",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "nodeCount",
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
    "name": "InvalidKmsContext",
    "inputs": [
      {
        "name": "kmsContextId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidKmsEpoch",
    "inputs": [
      {
        "name": "epochId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidNullThreshold",
    "inputs": [
      {
        "name": "thresholdName",
        "type": "string",
        "internalType": "string"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidProposalId",
    "inputs": []
  },
  {
    "type": "error",
    "name": "KmsContextCreationAlreadyConfirmed",
    "inputs": [
      {
        "name": "signer",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "kmsContextId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "KmsContextCreationUnauthorized",
    "inputs": [
      {
        "name": "signer",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "kmsContextId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "KmsContextNotDestroyed",
    "inputs": [
      {
        "name": "kmsContextId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "KmsContextNotPending",
    "inputs": [
      {
        "name": "kmsContextId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "KmsDestructionAlreadyConfirmed",
    "inputs": [
      {
        "name": "signer",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "destroyedId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "KmsDestructionUnauthorized",
    "inputs": [
      {
        "name": "signer",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "destroyedId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "KmsEpochNotDestroyed",
    "inputs": [
      {
        "name": "epochId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "KmsLifecycleOperationInFlight",
    "inputs": [
      {
        "name": "kmsContextId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "epochId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "KmsNodeNullSigner",
    "inputs": []
  },
  {
    "type": "error",
    "name": "KmsNodeNullTxSender",
    "inputs": []
  },
  {
    "type": "error",
    "name": "KmsSignerAlreadyRegistered",
    "inputs": [
      {
        "name": "signer",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "KmsSignerSetExceedsProofFormatLimit",
    "inputs": [
      {
        "name": "signerCount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "maxAllowed",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "KmsTxSenderAlreadyRegistered",
    "inputs": [
      {
        "name": "txSender",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "LatestActiveKmsContextCannotBeDestroyed",
    "inputs": [
      {
        "name": "kmsContextId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "LatestActiveKmsEpochCannotBeDestroyed",
    "inputs": [
      {
        "name": "epochId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "NonIncreasingKmsContextId",
    "inputs": [
      {
        "name": "contextId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "latestActiveKmsContextId",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "ZeroChainId",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ZeroGwStartBlock",
    "inputs": []
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
pub mod ProtocolConfig {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a06040523060805234801562000014575f80fd5b506200001f62000025565b620000d9565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00805468010000000000000000900460ff1615620000765760405163f92ee8a960e01b815260040160405180910390fd5b80546001600160401b0390811614620000d65780546001600160401b0319166001600160401b0390811782556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a15b50565b608051615199620001005f395f81816129ba015281816129e30152612bc801526151995ff3fe608060405260043610610212575f3560e01c8063756e86521161011e578063bf9b16c8116100a8578063c999a8b41161006d578063c999a8b4146105dc578063ccbf8199146105fb578063cceac0191461061a578063ee7d52d114610639578063f9c670c314610658575f80fd5b8063bf9b16c81461054c578063c0ae64f71461056b578063c2580a2d1461058a578063c2b42986146105a9578063c3aaaa5a146105bd575f80fd5b8063976f3eb9116100ee578063976f3eb9146104c15780639d1b1be1146104d5578063a642ba0f146104e9578063ad3cb1cc14610508578063b4722bc414610538575f80fd5b8063756e8652146104505780637eaac8f21461046f5780639447cfd414610483578063976c98b5146104a2575f80fd5b806341ad069c1161019f57806352d1902d1161016f57806352d1902d146103a95780635bff76d9146103bd57806365b394af146103e95780636ec0f23f1461041257806371d76dae14610431575f80fd5b806341ad069c1461033957806346c5bbbd1461035857806347e82295146103775780634f1ef28614610396575f80fd5b8063221cdd4e116101e5578063221cdd4e1461029957806326cf5def146102b8578063281e8bfe146102da5780632a388998146102f957806331ff41c81461030d575f80fd5b80630d8e6e2c14610216578063123abb28146102405780631ce3f9bc14610256578063203d01141461026a575b5f80fd5b348015610221575f80fd5b5061022a610684565b6040516102379190613f65565b60405180910390f35b34801561024b575f80fd5b506102546106f0565b005b348015610261575f80fd5b506102546107cb565b348015610275575f80fd5b50610289610284366004613f9b565b6108f5565b6040519015158152602001610237565b3480156102a4575f80fd5b506102546102b336600461403a565b610935565b3480156102c3575f80fd5b506102cc610b4b565b604051908152602001610237565b3480156102e5575f80fd5b506102cc6102f43660046140ed565b610b71565b348015610304575f80fd5b506102cc610b96565b348015610318575f80fd5b5061032c610327366004614104565b610bbc565b604051610237919061417f565b348015610344575f80fd5b506102cc6103533660046140ed565b610d80565b348015610363575f80fd5b50610289610372366004614104565b610dc5565b348015610382575f80fd5b506102cc6103913660046140ed565b610e23565b6102546103a43660046141a5565b610e48565b3480156103b4575f80fd5b506102cc610e67565b3480156103c8575f80fd5b506103dc6103d73660046140ed565b610e82565b6040516102379190614262565b3480156103f4575f80fd5b506103fd610f05565b60408051928352602083019190915201610237565b34801561041d575f80fd5b5061025461042c3660046142ae565b610f25565b34801561043c575f80fd5b5061025461044b366004614321565b611386565b34801561045b575f80fd5b5061025461046a3660046143e6565b611626565b34801561047a575f80fd5b506103dc611795565b34801561048e575f80fd5b5061028961049d366004614104565b611816565b3480156104ad575f80fd5b506102546104bc36600461403a565b611854565b3480156104cc575f80fd5b506102cc611c04565b3480156104e0575f80fd5b506102cc611c18565b3480156104f4575f80fd5b506102546105033660046142ae565b611c29565b348015610513575f80fd5b5061022a604051806040016040528060058152602001640352e302e360dc1b81525081565b348015610543575f80fd5b506102cc611d7c565b348015610557575f80fd5b506102896105663660046140ed565b611da2565b348015610576575f80fd5b506102546105853660046140ed565b611dac565b348015610595575f80fd5b506102546105a43660046140ed565b611f30565b3480156105b4575f80fd5b506102cc612125565b3480156105c8575f80fd5b506102cc6105d73660046140ed565b61214b565b3480156105e7575f80fd5b506103fd6105f63660046140ed565b612170565b348015610606575f80fd5b50610254610615366004614484565b6121d7565b348015610625575f80fd5b50610289610634366004614533565b6124e3565b348015610644575f80fd5b506102896106533660046140ed565b612548565b348015610663575f80fd5b506106776106723660046140ed565b612552565b60405161023791906145ab565b60606040518060400160405280600e81526020016d50726f746f636f6c436f6e66696760901b8152506106b65f612715565b6106c06004612715565b6106c95f612715565b6040516020016106dc94939291906145bd565b604051602081830303815290604052905090565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00805460059190600160401b900460ff1680610739575080546001600160401b03808416911610155b156107575760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b03831617600160401b1781556107806127a4565b805460ff60401b191681556040516001600160401b03831681527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a15050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561081b573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061083f919061463a565b6001600160a01b0316336001600160a01b0316146108775760405163021bfda160e41b81523360048201526024015b60405180910390fd5b61087f6127a4565b5f610888612867565b600b8101549091505f61089a8261288b565b905080827f15aaaf475ef407543f5164f57dcf57f7f93816f55bae77ca09efc445ba40eef78486600d01546001436108d29190614669565b6040805193845260208401929092529082015260600160405180910390a3505050565b5f806108ff612867565b600b8101545f9081526003909101602090815260408083206001600160a01b039096168352949052929092205460ff1692915050565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00546001600160401b03166001600160401b031660011461098957604051636f4f731f60e01b815260040160405180910390fd5b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00805460059190600160401b900460ff16806109d2575080546001600160401b03808416911610155b156109f05760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b03831617600160401b1781555f610a1a612867565b90505f610a2c600760f81b6001614690565b9050610a4981610a41600160fb1b6001614690565b8d8d8d6128dc565b60405180604001604052804381526020018c8c8c8c8c8c8c604051602001610a7797969594939291906147e6565b60408051601f1981840301815291815281516020928301209092525f848152601786018252919091208251815591015160019091015560f86007901b817f204d6b80121154cd87d99cf54c639a3dd0a53b3084277098de972ebdd34c6be98d8d8d8d8d8d8d604051610aef97969594939291906147e6565b60405180910390a35050805460ff60401b191681556040516001600160401b03831681527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a1505050505050505050565b5f80610b55612867565b600b8101545f9081526009909101602052604090205492915050565b5f610b7b8261292d565b610b83612867565b5f92835260070160205250604090205490565b5f80610ba0612867565b600b8101545f9081526006909101602052604090205492915050565b604080516080810182525f8082526020820152606091810182905281810191909152610be783612959565b610c07576040516377ddbe8160e01b81526004810184905260240161086e565b610c0f612867565b5f848152600491909101602090815260408083206001600160a01b03808716855290835292819020815160808101835281548516815260018201549094169284019290925260028201805491840191610c67906149ae565b80601f0160208091040260200160405190810160405280929190818152602001828054610c93906149ae565b8015610cde5780601f10610cb557610100808354040283529160200191610cde565b820191905f5260205f20905b815481529060010190602001808311610cc157829003601f168201915b50505050508152602001600382018054610cf7906149ae565b80601f0160208091040260200160405190810160405280929190818152602001828054610d23906149ae565b8015610d6e5780601f10610d4557610100808354040283529160200191610d6e565b820191905f5260205f20905b815481529060010190602001808311610d5157829003601f168201915b50505050508152505090505b92915050565b5f610d8a82612977565b610daa576040516377ddbe8160e01b81526004810183905260240161086e565b610db2612867565b5f92835260080160205250604090205490565b5f610dcf83612977565b610def576040516377ddbe8160e01b81526004810184905260240161086e565b610df7612867565b5f938452600201602090815260408085206001600160a01b039490941685529290525090205460ff1690565b5f610e2d8261292d565b610e35612867565b5f92835260090160205250604090205490565b610e506129af565b610e5982612a55565b610e638282612afc565b5050565b5f610e70612bbd565b505f8051602061517983398151915290565b6060610e8d8261292d565b610e95612867565b6005015f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020018280548015610ef957602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311610edb575b50505050509050919050565b5f805f610f10612867565b905080600b0154925080600d01549150509091565b5f610f2e612867565b90505f7f98bc32bb4045abd79f66305d47272679087ff12d8a03152ce14a0954b7709c00905060015f888152600e8401602052604090205460ff166003811115610f7a57610f7a61467c565b14610f9b57604051633586efa160e01b81526004810188905260240161086e565b600b8201545f88815260208390526040808220549051611034917f8a7d3190a514cdb5d57c841cb7382e85ad21b892b7fdfd3b87ab000925d9d5019185918d91610fe8908c908c906149e6565b6040805191829003822060208301969096528101939093526060830191909152608082015260a081019190915260c0015b60405160208183030381529060405280519060200120612c06565b90505f611076828a8a8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250612d0a92505050565b90505f856003015f8581526020019081526020015f205f836001600160a01b03166001600160a01b031681526020019081526020015f205f9054906101000a900460ff1690505f866003015f8d81526020019081526020015f205f846001600160a01b03166001600160a01b031681526020019081526020015f205f9054906101000a900460ff1690508115801561110c575080155b1561113c57604051631703bf1d60e31b81526001600160a01b0384166004820152602481018d905260440161086e565b5f8c815260018701602090815260408083206001600160a01b038716845290915290205460ff161561119357604051630c4b0b9960e31b81526001600160a01b0384166004820152602481018d905260440161086e565b5f8c81526001878101602090815260408084206001600160a01b0388168552909152909120805460ff1916909117905581156111f6575f8c81526002870160209081526040808320878452909152812080549091906111f1906149f5565b909155505b8015611229575f8c8152600387016020908152604080832087845290915281208054909190611224906149f5565b909155505b826001600160a01b03168c7f4254d948402ba059c02f9feecfca09b40c0b39026a344a159e3a7679807a83d28d8d8d8d6040516112699493929190614a0d565b60405180910390a35f85815260098801602090815260408083205460018b019092528220546112989190614669565b5f8e815260018a01602090815260408083205460038c0183528184208a8552909252909120549192501480156112f757505f81116112d75760016112d9565b805b5f8e8152600289016020908152604080832089845290915290205410155b15611377575f8d8152600e890160205260408120805460ff1916600217905561131f8e61288b565b9050808e7f15aaaf475ef407543f5164f57dcf57f7f93816f55bae77ca09efc445ba40eef7898c600d01546001436113579190614669565b6040805193845260208401929092529082015260600160405180910390a3505b50505050505050505050505050565b5f61138f612867565b905060015f8b8152600f8301602052604090205460ff1660028111156113b7576113b761467c565b146113d857604051637995bbcf60e01b8152600481018b905260240161086e565b5f8a81526010820160205260408120549080806113fd848f8f8f8f8f8f8f8f8f612d32565b9250925092505f6114108f868685612e9f565b9050836001600160a01b03168f7f7faef745af79a3af23d4199ce0366975cc2dd7149384d631337d133b226b5be4858d8d8d8d604051611454959493929190614a3e565b60405180910390a38061146c5750505050505061161b565b5050505f828152600e840160205260409020805460ff1916600317905550600b820181905561149b8b82612f75565b5f818152600183016020526040812080549091906001600160401b038111156114c6576114c6614191565b6040519080825280602002602001820160405280156114f957816020015b60608152602001906001900390816114e45790505b5090505f5b82548110156115d45782818154811061151957611519614a76565b905f5260205f2090600402016003018054611533906149ae565b80601f016020809104026020016040519081016040528092919081815260200182805461155f906149ae565b80156115aa5780601f10611581576101008083540402835291602001916115aa565b820191905f5260205f20905b81548152906001019060200180831161158d57829003601f168201915b50505050508282815181106115c1576115c1614a76565b60209081029190910101526001016114fe565b508c837f6c6eb35881f805f284be59b840b4ea990f96b13dd9e7192c4581e1bc81c0af988e8e8e8e8760405161160e959493929190614b7b565b60405180910390a3505050505b505050505050505050565b61162e612867565b5f888152600a91909101602052604090205460ff1661166357604051632b73d9b760e01b81526004810188905260240161086e565b5f8781527f98bc32bb4045abd79f66305d47272679087ff12d8a03152ce14a0954b7709c0460209081526040808320905161173d928b917fc687b63525ef20d95442d56bd140c9da6c10c6e23f482486c246c05236ceef9c9183916116cc918e918e9101614cdd565b6040516020818303038152906040528051906020012088886040516116f29291906149e6565b6040519081900381206117209493929160200193845260208401929092526040830152606082015260800190565b604051602081830303815290604052805190602001208888612faf565b9050806001600160a01b0316887fea1640f34deada2f815def0e112e6cd249533849a0fd53e9293ba34d7c4688cb89898989898960405161178396959493929190614d04565b60405180910390a35050505050505050565b60605f6117a0612867565b9050806005015f82600b015481526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561180b57602002820191905f5260205f20905b81546001600160a01b031681526001909101906020018083116117ed575b505050505091505090565b5f6118208361292d565b611828612867565b5f938452600301602090815260408085206001600160a01b039490941685529290525090205460ff1690565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156118a4573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118c8919061463a565b6001600160a01b0316336001600160a01b0316146118fb5760405163021bfda160e41b815233600482015260240161086e565b6119036127a4565b5f61190c612867565b600b8101548154919250905f90611924906001614690565b9050611932818b8b8b6130d9565b5f818152600e840160209081526040808320805460ff191660019081179091558601825280832080548251818502810185019093528083529192909190849084015b82821015611ae8575f848152602090819020604080516080810182526004860290920180546001600160a01b03908116845260018201541693830193909352600283018054929392918401916119c9906149ae565b80601f01602080910402602001604051908101604052809291908181526020018280546119f5906149ae565b8015611a405780601f10611a1757610100808354040283529160200191611a40565b820191905f5260205f20905b815481529060010190602001808311611a2357829003601f168201915b50505050508152602001600382018054611a59906149ae565b80601f0160208091040260200160405190810160405280929190818152602001828054611a85906149ae565b8015611ad05780601f10611aa757610100808354040283529160200191611ad0565b820191905f5260205f20905b815481529060010190602001808311611ab357829003601f168201915b50505050508152505081526020019060010190611974565b5050505090508089604051602001611b01929190614d6b565b60405160208183030381529060405280519060200120611b3e7f98bc32bb4045abd79f66305d47272679087ff12d8a03152ce14a0954b7709c0090565b5f84815260209182526040908190209290925581518083018352438152915182820191611b79918f918f918f918f918f918f918f91016147e6565b60408051601f1981840301815291815281516020928301209092525f8581526017880182528290208351815592015160019092019190915551839083907f204d6b80121154cd87d99cf54c639a3dd0a53b3084277098de972ebdd34c6be990611bef908f908f908f908f908f908f908f906147e6565b60405180910390a35050505050505050505050565b5f80611c0e612867565b600b015492915050565b5f80611c22612867565b5492915050565b5f8581527f98bc32bb4045abd79f66305d47272679087ff12d8a03152ce14a0954b7709c0660205260409020547f98bc32bb4045abd79f66305d47272679087ff12d8a03152ce14a0954b7709c009060ff16611c9a57604051623d8b4d60e41b81526004810187905260240161086e565b5f611d29826005015f8981526020019081526020015f20887f4af2c2967c6c1fa8706c2504bb0cad843f8d0fac79f31f2d0206e747f049fca28a8888604051611ce49291906149e6565b604051908190038120611d0c9392916020019283526020830191909152604082015260600190565b604051602081830303815290604052805190602001208989612faf565b9050806001600160a01b0316877fee1b8f29d516d563f41f2a3b75c200752feb512e6256eb561d17a4e61ee6335e88888888604051611d6b9493929190614a0d565b60405180910390a350505050505050565b5f80611d86612867565b600b8101545f9081526008909101602052604090205492915050565b5f610d7a82613616565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611dfc573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e20919061463a565b6001600160a01b0316336001600160a01b031614611e535760405163021bfda160e41b815233600482015260240161086e565b5f611e5c612867565b905080600b01548203611e855760405163030f5d5560e31b81526004810183905260240161086e565b611e8e82612977565b611eae576040516377ddbe8160e01b81526004810183905260240161086e565b5f828152600a820160209081526040808320805460ff19908116600117909155600e8501835281842080549091169055600c8401548084526010850190925290912054839003611f0157611f018161364e565b60405183907fda075d09198d207e3a918d4b8dfc87df2d60a00be703fd39eaac90962da0b7f0905f90a2505050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611f80573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611fa4919061463a565b6001600160a01b0316336001600160a01b031614611fd75760405163021bfda160e41b815233600482015260240161086e565b5f611fe0612867565b905080600d015482036120095760405163f0e781c160e01b81526004810183905260240161086e565b5f828152600f8201602052604081205460ff169060028260028111156120315761203161467c565b1490505f60018360028111156120495761204961467c565b148015612088575060035f8681526010860160209081526040808320548352600e880190915290205460ff1660038111156120865761208661467c565b145b905081158015612096575080155b156120b757604051637995bbcf60e01b81526004810186905260240161086e565b6120c08561364e565b5f8581527f98bc32bb4045abd79f66305d47272679087ff12d8a03152ce14a0954b7709c066020526040808220805460ff191660011790555186917f03436013075f8d1abb1781dbcaf418fc76fc797d1ab51c38664c1a51f3cd57f991a25050505050565b5f8061212f612867565b600b8101545f9081526007909101602052604090205492915050565b5f6121558261292d565b61215d612867565b5f92835260060160205250604090205490565b5f8061217b83612959565b61219b576040516377ddbe8160e01b81526004810184905260240161086e565b5f6121a4612867565b5f948552601701602090815260409485902085518087019096528054808752600190910154959091018590529492505050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612227573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061224b919061463a565b6001600160a01b0316336001600160a01b03161461227e5760405163021bfda160e41b815233600482015260240161086e565b855f0361229e57604051630992f7ad60e01b815260040160405180910390fd5b5f8490036122bf5760405163b548914760e01b815260040160405180910390fd5b5f8290036122e057604051632f94141160e21b815260040160405180910390fd5b806001600160401b03165f03612309576040516302fa7d2960e31b815260040160405180910390fd5b5f5b8281101561249a573684848381811061232657612326614a76565b60600291909101915061233e90506020820182614dad565b6001600160401b03165f0361236657604051633212217560e21b815260040160405180910390fd5b6123766060820160408301614dad565b6001600160401b031661238f6040830160208401614dad565b6001600160401b03161115612400576123ab6020820182614dad565b6123bb6040830160208401614dad565b6123cb6060840160408501614dad565b60405163790cee0760e11b81526001600160401b0393841660048201529183166024830152909116604482015260640161086e565b5f5b82811015612490576124176020830183614dad565b6001600160401b031686868381811061243257612432614a76565b6124489260206060909202019081019150614dad565b6001600160401b031603612488576124636020830183614dad565b6040516306c67e4760e41b81526001600160401b03909116600482015260240161086e565b600101612402565b505060010161230b565b50857fc42f1ecac8d4881ef9fd335ca98ee7254a436b92ba874a609e50a7d30c487b8d86868686866040516124d3959493929190614dc6565b60405180910390a2505050505050565b5f806124ed612867565b905060025f848152600f8301602052604090205460ff1660028111156125155761251561467c565b14801561253057505f83815260108201602052604090205484145b8015612540575061254084613616565b949350505050565b5f610d7a82612977565b606061255d8261292d565b612565612867565b6001015f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020015f905b8282101561270a575f848152602090819020604080516080810182526004860290920180546001600160a01b03908116845260018201541693830193909352600283018054929392918401916125eb906149ae565b80601f0160208091040260200160405190810160405280929190818152602001828054612617906149ae565b80156126625780601f1061263957610100808354040283529160200191612662565b820191905f5260205f20905b81548152906001019060200180831161264557829003601f168201915b5050505050815260200160038201805461267b906149ae565b80601f01602080910402602001604051908101604052809291908181526020018280546126a7906149ae565b80156126f25780601f106126c9576101008083540402835291602001916126f2565b820191905f5260205f20905b8154815290600101906020018083116126d557829003601f168201915b50505050508152505081526020019060010190612596565b505050509050919050565b60605f61272183613681565b60010190505f816001600160401b0381111561273f5761273f614191565b6040519080825280601f01601f191660200182016040528015612769576020820181803683370190505b5090508181016020015b5f19016f181899199a1a9b1b9c1cb0b131b232b360811b600a86061a8153600a850494508461277357509392505050565b5f6127ad612867565b80545f908152600e8201602052604081205491925060ff9091169060018260038111156127dc576127dc61467c565b14806127f9575060028260038111156127f7576127f761467c565b145b90505f6001600c8501545f908152600f8601602052604090205460ff1660028111156128275761282761467c565b14905081806128335750805b15612861578354600c8501546040516303c0105f60e11b81526004810192909252602482015260440161086e565b50505050565b7f80f3585af86806c5774303b06c1ee640aa83b6ef3e45df49bb26c8524500c20090565b5f80612895612867565b905080600c015f81546128a7906149f5565b91829055505f818152600f830160209081526040808320805460ff191660011790556010909401905291909120929092555090565b5f6128e5612867565b90506128f3868585856130d9565b5f868152600e820160205260409020805460ff19166003179055600b8101869055600c81018590556129258587612f75565b505050505050565b61293681613616565b612956576040516377ddbe8160e01b81526004810182905260240161086e565b50565b5f612962612867565b5f928352600101602052506040902054151590565b5f80612981612867565b5f848152600e91909101602052604090205460ff1660038111156129a7576129a761467c565b141592915050565b306001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161480612a3557507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316612a295f80516020615179833981519152546001600160a01b031690565b6001600160a01b031614155b15612a535760405163703e46dd60e11b815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612aa5573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612ac9919061463a565b6001600160a01b0316336001600160a01b0316146129565760405163021bfda160e41b815233600482015260240161086e565b816001600160a01b03166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa925050508015612b56575060408051601f3d908101601f19168201909252612b5391810190614e62565b60015b612b7e57604051634c9c8ce360e01b81526001600160a01b038316600482015260240161086e565b5f805160206151798339815191528114612bae57604051632a87526960e21b81526004810182905260240161086e565b612bb88383613758565b505050565b306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614612a535760405163703e46dd60e11b815260040160405180910390fd5b604080518082018252600e81526d50726f746f636f6c436f6e66696760901b6020918201528151808301835260018152603160f81b9082015281517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f818301527fa3de1880cf083e8318b77a7965d02dd9765e85a48e418a4463af7a0d57b4b3ee818401527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc660608201524660808201523060a0808301919091528351808303909101815260c08201845280519083012061190160f01b60e083015260e2820152610102808201949094528251808203909401845261012201909152815191012090565b5f805f80612d1886866137ad565b925092509250612d2882826137f6565b5090949350505050565b5f8080891580612d40575087155b15612d61576040516318855a7960e21b8152600481018d905260240161086e565b612d6d8b8b8b8b6138ae565b9150612dec7f6cf47d8f5bed73ad2a8c60adbdd5b555d665357db758662abd49ea977f80d19a8e612d9c612867565b600d01548f868a8a604051612db29291906149e6565b6040805191829003822060208301979097528101949094526060840192909252608083015260a082015260c081019190915260e001611019565b9050612e2d8188888080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250612d0a92505050565b9250612e37612867565b5f8e8152600391909101602090815260408083206001600160a01b038716845290915290205460ff16612e8f5760405163a3f4afeb60e01b81526001600160a01b0384166004820152602481018d905260440161086e565b9a509a509a975050505050505050565b5f80612ea9612867565b5f87815260128201602090815260408083206001600160a01b038916845290915290205490915060ff1615612f03576040516324a0bb1b60e11b81526001600160a01b03851660048201526024810187905260440161086e565b5f86815260128201602090815260408083206001600160a01b03881684528252808320805460ff19166001179055888352601384018252808320868452909152812080548290612f52906149f5565b91829055505f968752600590920160205250604090942054909314949350505050565b5f612f7e612867565b5f848152600f820160209081526040808320805460ff191660021790556010840190915290209290925550600d0155565b5f80612fb9612867565b9050613002612fc786612c06565b85858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250612d0a92505050565b600b8201545f90815260038301602090815260408083206001600160a01b038516845290915290205490925060ff1661306057604051631ac5164960e31b81526001600160a01b03831660048201526024810187905260440161086e565b6001600160a01b0382165f9081526020889052604090205460ff16156130ab57604051639d2799e960e01b81526001600160a01b03831660048201526024810187905260440161086e565b506001600160a01b0381165f9081526020969096526040909520805460ff1916600117905550929392505050565b5f8290036130f957604051621a323560e61b815260040160405180910390fd5b60ff821115613125576040516302d4e4ef60e31b81526004810183905260ff602482015260440161086e565b60408051808201909152601081526f383ab13634b1a232b1b93cb83a34b7b760811b602082015261315890823584613c8b565b6131906040518060400160405280600e81526020016d3ab9b2b92232b1b93cb83a34b7b760911b815250826020013585859050613c8b565b604080518082018252600681526535b6b9a3b2b760d11b60208201526131b99183013584613c8b565b6040805180820190915260038152626d706360e81b60208201526131e290606083013584613c8b565b5f6131eb612867565b8054909150851161321c57805460405163efd55f6760e01b815261086e918791600401918252602082015260400190565b8481555f5b838110156135ca573685858381811061323c5761323c614a76565b905060200281019061324e9190614e79565b60408051608081019091529091505f908061326c6020850185613f9b565b6001600160a01b0316815260200183602001602081019061328d9190613f9b565b6001600160a01b031681526020016132a86040850185614e97565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152505050908252506020016132ee6060850185614e97565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250505091525080519091506001600160a01b031661334e57604051634233402560e11b815260040160405180910390fd5b60208101516001600160a01b031661337957604051632deccf4d60e01b815260040160405180910390fd5b5f888152600285016020908152604080832084516001600160a01b0316845290915290205460ff16156133cd578051604051630d18c4ff60e41b81526001600160a01b03909116600482015260240161086e565b5f8881526003850160209081526040808320848301516001600160a01b0316845290915290205460ff161561342657602081015160405163f51af6bb60e01b81526001600160a01b03909116600482015260240161086e565b5f8881526001858101602090815260408084208054808501825590855293829020855160049095020180546001600160a01b03199081166001600160a01b039687161782559286015193810180549093169390941692909217905582015182919060028201906134969082614f24565b50606082015160038201906134ab9082614f24565b5050505f88815260028086016020908152604080842085516001600160a01b039081168652908352818520805460ff1990811660019081179092558e875260038b018552838720888601805185168952908652848820805490921683179091558e875260048b01855283872088518416885290945294829020865181549083166001600160a01b031991821617825593519581018054969092169590931694909417909355918301518392918201906135649082614f24565b50606082015160038201906135799082614f24565b5050505f8881526005850160209081526040822092810151835460018082018655948452919092200180546001600160a01b0319166001600160a01b03909216919091179055919091019050613221565b505f948552600681016020908152604080872084359055600783018252808720828501359055600883018252808720818501359055600990920190529093206060909301359092555050565b5f6003613621612867565b5f848152600e91909101602052604090205460ff1660038111156136475761364761467c565b1492915050565b5f613657612867565b5f928352600f810160209081526040808520805460ff191690556010909201905282209190915550565b5f8072184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b83106136bf5772184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b830492506040015b6d04ee2d6d415b85acef810000000083106136eb576d04ee2d6d415b85acef8100000000830492506020015b662386f26fc10000831061370957662386f26fc10000830492506010015b6305f5e1008310613721576305f5e100830492506008015b612710831061373557612710830492506004015b60648310613747576064830492506002015b600a8310610d7a5760010192915050565b61376182613cd4565b6040516001600160a01b038316907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a28051156137a557612bb88282613d37565b610e63613da9565b5f805f83516041036137e4576020840151604085015160608601515f1a6137d688828585613dc8565b9550955095505050506137ef565b505081515f91506002905b9250925092565b5f8260038111156138095761380961467c565b03613812575050565b60018260038111156138265761382661467c565b036138445760405163f645eedf60e01b815260040160405180910390fd5b60028260038111156138585761385861467c565b036138795760405163fce698f760e01b81526004810182905260240161086e565b600382600381111561388d5761388d61467c565b03610e63576040516335e2f38360e21b81526004810182905260240161086e565b5f80846001600160401b038111156138c8576138c8614191565b6040519080825280602002602001820160405280156138f1578160200160208202803683370190505b5090505f5b85811015613b3857365f88888481811061391257613912614a76565b90506020028101906139249190614fdf565b613932906040810190614ff3565b90925090505f816001600160401b0381111561395057613950614191565b604051908082528060200260200182016040528015613979578160200160208202803683370190505b5090505f5b82811015613a6b577fddd108772e6a3899feb04d148ae915cbe3eb5ebd202688080399e9921ac3616b8484838181106139b9576139b9614a76565b90506020028101906139cb9190615038565b6139d990602081019061504c565b8585848181106139eb576139eb614a76565b90506020028101906139fd9190615038565b613a0b906020810190614e97565b604051613a199291906149e6565b604051908190038120613a30939291602001615065565b60405160208183030381529060405280519060200120828281518110613a5857613a58614a76565b602090810291909101015260010161397e565b50898985818110613a7e57613a7e614a76565b9050602002810190613a909190614fdf565b358a8a86818110613aa357613aa3614a76565b9050602002810190613ab59190614fdf565b6020013582604051602001613aca9190615087565b60408051601f19818403018152828252805160209182012090830194909452810191909152606081019190915260800160405160208183030381529060405280519060200120858581518110613b2257613b22614a76565b60209081029190910101525050506001016138f6565b505f836001600160401b03811115613b5257613b52614191565b604051908082528060200260200182016040528015613b7b578160200160208202803683370190505b5090505f5b84811015613c5557858582818110613b9a57613b9a614a76565b9050602002810190613bac9190614fdf565b35868683818110613bbf57613bbf614a76565b9050602002810190613bd19190614fdf565b60200135878784818110613be757613be7614a76565b9050602002810190613bf99190614fdf565b613c07906040810190614e97565b604051602001613c1a94939291906150bc565b60405160208183030381529060405280519060200120828281518110613c4257613c42614a76565b6020908102919091010152600101613b80565b508181604051602001613c6992919061511f565b6040516020818303038152906040528051906020012092505050949350505050565b815f03613cad5782604051631b5fdb0760e11b815260040161086e9190613f65565b80821115612bb85782828260405163caa814a360e01b815260040161086e93929190615143565b806001600160a01b03163b5f03613d0957604051634c9c8ce360e01b81526001600160a01b038216600482015260240161086e565b5f8051602061517983398151915280546001600160a01b0319166001600160a01b0392909216919091179055565b60605f80846001600160a01b031684604051613d539190615167565b5f60405180830381855af49150503d805f8114613d8b576040519150601f19603f3d011682016040523d82523d5f602084013e613d90565b606091505b5091509150613da0858383613e90565b95945050505050565b3415612a535760405163b398979f60e01b815260040160405180910390fd5b5f80807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0841115613e0157505f91506003905082613e86565b604080515f808252602082018084528a905260ff891692820192909252606081018790526080810186905260019060a0016020604051602081039080840390855afa158015613e52573d5f803e3d5ffd5b5050604051601f1901519150506001600160a01b038116613e7d57505f925060019150829050613e86565b92505f91508190505b9450945094915050565b606082613ea557613ea082613eef565b613ee8565b8151158015613ebc57506001600160a01b0384163b155b15613ee557604051639996b31560e01b81526001600160a01b038516600482015260240161086e565b50805b9392505050565b805115613eff5780518082602001fd5b60405163d6bda27560e01b815260040160405180910390fd5b5f5b83811015613f32578181015183820152602001613f1a565b50505f910152565b5f8151808452613f51816020860160208601613f18565b601f01601f19169290920160200192915050565b602081525f613ee86020830184613f3a565b6001600160a01b0381168114612956575f80fd5b8035613f9681613f77565b919050565b5f60208284031215613fab575f80fd5b8135613ee881613f77565b5f8083601f840112613fc6575f80fd5b5081356001600160401b03811115613fdc575f80fd5b6020830191508360208260051b8501011115613ff6575f80fd5b9250929050565b5f8083601f84011261400d575f80fd5b5081356001600160401b03811115614023575f80fd5b602083019150836020828501011115613ff6575f80fd5b5f805f805f805f87890360e0811215614051575f80fd5b88356001600160401b0380821115614067575f80fd5b6140738c838d01613fb6565b909a5098508891506080601f198401121561408c575f80fd5b60208b01975060a08b01359250808311156140a5575f80fd5b6140b18c848d01613ffd565b909750955060c08b01359250869150808311156140cc575f80fd5b50506140da8a828b01613fb6565b989b979a50959850939692959293505050565b5f602082840312156140fd575f80fd5b5035919050565b5f8060408385031215614115575f80fd5b82359150602083013561412781613f77565b809150509250929050565b5f60018060a01b03808351168452806020840151166020850152506040820151608060408501526141666080850182613f3a565b905060608301518482036060860152613da08282613f3a565b602081525f613ee86020830184614132565b634e487b7160e01b5f52604160045260245ffd5b5f80604083850312156141b6575f80fd5b82356141c181613f77565b915060208301356001600160401b03808211156141dc575f80fd5b818501915085601f8301126141ef575f80fd5b81358181111561420157614201614191565b604051601f8201601f19908116603f0116810190838211818310171561422957614229614191565b81604052828152886020848701011115614241575f80fd5b826020860160208301375f6020848301015280955050505050509250929050565b602080825282518282018190525f9190848201906040850190845b818110156142a25783516001600160a01b03168352928401929184019160010161427d565b50909695505050505050565b5f805f805f606086880312156142c2575f80fd5b8535945060208601356001600160401b03808211156142df575f80fd5b6142eb89838a01613ffd565b90965094506040880135915080821115614303575f80fd5b5061431088828901613ffd565b969995985093965092949392505050565b5f805f805f805f805f60a08a8c031215614339575f80fd5b8935985060208a01356001600160401b0380821115614356575f80fd5b6143628d838e01613fb6565b909a50985060408c013591508082111561437a575f80fd5b6143868d838e01613fb6565b909850965060608c013591508082111561439e575f80fd5b6143aa8d838e01613ffd565b909650945060808c01359150808211156143c2575f80fd5b506143cf8c828d01613ffd565b915080935050809150509295985092959850929598565b5f805f805f805f6080888a0312156143fc575f80fd5b8735965060208801356001600160401b0380821115614419575f80fd5b6144258b838c01613fb6565b909850965060408a013591508082111561443d575f80fd5b6144498b838c01613ffd565b909650945060608a0135915080821115614461575f80fd5b506140da8a828b01613ffd565b80356001600160401b0381168114613f96575f80fd5b5f805f805f8060808789031215614499575f80fd5b8635955060208701356001600160401b03808211156144b6575f80fd5b6144c28a838b01613ffd565b909750955060408901359150808211156144da575f80fd5b818901915089601f8301126144ed575f80fd5b8135818111156144fb575f80fd5b8a602060608302850101111561450f575f80fd5b6020830195508094505050506145276060880161446e565b90509295509295509295565b5f8060408385031215614544575f80fd5b50508035926020909101359150565b5f8282518085526020808601955060208260051b840101602086015f5b8481101561459e57601f1986840301895261458c838351614132565b98840198925090830190600101614570565b5090979650505050505050565b602081525f613ee86020830184614553565b5f85516145ce818460208a01613f18565b61103b60f11b90830190815285516145ed816002840160208a01613f18565b808201915050601760f91b8060028301528551614611816003850160208a01613f18565b6003920191820152835161462c816004840160208801613f18565b016004019695505050505050565b5f6020828403121561464a575f80fd5b8151613ee881613f77565b634e487b7160e01b5f52601160045260245ffd5b81810381811115610d7a57610d7a614655565b634e487b7160e01b5f52602160045260245ffd5b80820180821115610d7a57610d7a614655565b5f808335601e198436030181126146b8575f80fd5b83016020810192503590506001600160401b038111156146d6575f80fd5b803603821315613ff6575f80fd5b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b8035600381900b8114613f96575f80fd5b5f8235605e19833603018112614731575f80fd5b90910192915050565b5f8383855260208086019550808560051b830101845f5b8781101561459e57848303601f1901895261476c828861471d565b606061477882836146a3565b82875261478883880182846146e4565b92505050614798868301836146a3565b868303888801526147aa8382846146e4565b9250505060406147bc818401846146a3565b9350868303828801526147d08385836146e4565b9c88019c96505050928501925050600101614751565b60e080825281018790525f61010080830160058a901b840182018b845b8c8110156149465786830360ff190184528135368f900360fe19018112614828575f80fd5b8e016148448461483783613f8b565b6001600160a01b03169052565b6020614851818301613f8b565b6001600160a01b031681860152604061486c838201846146a3565b898389015261487e8a890182846146e4565b925050506060614890818501856146a3565b888403838a01526148a28482846146e4565b935050505060806148b481850161470c565b6148c28289018260030b9052565b505060a06148d2818501856146a3565b888403838a01526148e48482846146e4565b935050505060c06148f7818501856146a3565b888403838a01526149098482846146e4565b935050505061491b60e08401846146a3565b935086820360e08801526149308285836146e4565b9783019796505050929092019150600101614803565b5050614976602086018b803582526020810135602083015260408101356040830152606081013560608301525050565b84810360a086015261498981898b6146e4565b9250505082810360c08401526149a081858761473a565b9a9950505050505050505050565b600181811c908216806149c257607f821691505b6020821081036149e057634e487b7160e01b5f52602260045260245ffd5b50919050565b818382375f9101908152919050565b5f60018201614a0657614a06614655565b5060010190565b604081525f614a206040830186886146e4565b8281036020840152614a338185876146e4565b979650505050505050565b858152606060208201525f614a576060830186886146e4565b8281036040840152614a6a8185876146e4565b98975050505050505050565b634e487b7160e01b5f52603260045260245ffd5b803560048110613f96575f80fd5b60048110614ab457634e487b7160e01b5f52602160045260245ffd5b9052565b5f8383855260208086019550808560051b830101845f5b8781101561459e57848303601f19018952614aea828861471d565b60608135855285820135868601526040614b06818401846146a3565b93508282880152614b1a83880185836146e4565b9c88019c96505050928501925050600101614acf565b5f8282518085526020808601955060208260051b840101602086015f5b8481101561459e57601f19868403018952614b69838351613f3a565b98840198925090830190600101614b4d565b60608082528181018690525f906080808401600589901b850182018a855b8b811015614cb157607f1980898503018552614bb5838f61471d565b8785018135865260208083013581880152604080840135601e19853603018112614bdd575f80fd5b9093019283358285016001600160401b03821115614bf9575f80fd5b8160051b803603821315614c0b575f80fd5b8a84018e9052948290529389018b01935f8c8b0136889003605e19015b84831015614c9557898d89030182528335818112614c44575f80fd5b8901878101614c5b8a614c5683614a8a565b614a98565b614c67888301826146a3565b9250905087898b0152614c7d888b0183836146e4565b99505050928601926001929092019190860190614c28565b5050509983019950929750509490940193505050600101614b99565b50508581036020870152614cc681898b614ab8565b93505050508281036040840152614a6a8185614b30565b5f6001600160fb1b03831115614cf1575f80fd5b8260051b80858437919091019392505050565b606080825281018690525f6001600160fb1b03871115614d22575f80fd5b8660051b80896080850137820182810360809081016020850152614d4990820187896146e4565b90508281036040840152614d5e8185876146e4565b9998505050505050505050565b60a081525f614d7d60a0830185614553565b9050613ee86020830184803582526020810135602083015260408101356040830152606081013560608301525050565b5f60208284031215614dbd575f80fd5b613ee88261446e565b5f6060808352614dda60608401888a6146e4565b838103602085810191909152868252879181015f5b88811015614e41576001600160401b0380614e098661446e565b16835280614e1885870161446e565b1684840152604081614e2b82880161446e565b1690840152509284019290840190600101614def565b50809450505050506001600160401b03831660408301529695505050505050565b5f60208284031215614e72575f80fd5b5051919050565b5f823560fe19833603018112614e8d575f80fd5b9190910192915050565b5f808335601e19843603018112614eac575f80fd5b8301803591506001600160401b03821115614ec5575f80fd5b602001915036819003821315613ff6575f80fd5b601f821115612bb857805f5260205f20601f840160051c81016020851015614efe5750805b601f840160051c820191505b81811015614f1d575f8155600101614f0a565b5050505050565b81516001600160401b03811115614f3d57614f3d614191565b614f5181614f4b84546149ae565b84614ed9565b602080601f831160018114614f84575f8415614f6d5750858301515b5f19600386901b1c1916600185901b178555612925565b5f85815260208120601f198616915b82811015614fb257888601518255948401946001909101908401614f93565b5085821015614fcf57878501515f19600388901b60f8161c191681555b5050505050600190811b01905550565b5f8235605e19833603018112614e8d575f80fd5b5f808335601e19843603018112615008575f80fd5b8301803591506001600160401b03821115615021575f80fd5b6020019150600581901b3603821315613ff6575f80fd5b5f8235603e19833603018112614e8d575f80fd5b5f6020828403121561505c575f80fd5b613ee882614a8a565b838152606081016150796020830185614a98565b826040830152949350505050565b81515f9082906020808601845b838110156150b057815185529382019390820190600101615094565b50929695505050505050565b848152836020820152606060408201525f6150db6060830184866146e4565b9695505050505050565b5f815180845260208085019450602084015f5b83811015615114578151875295820195908201906001016150f8565b509495945050505050565b604081525f61513160408301856150e5565b8281036020840152613da081856150e5565b606081525f6151556060830186613f3a565b60208301949094525060400152919050565b5f8251614e8d818460208701613f1856fe360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R0`\x80R4\x80\x15b\0\0\x14W_\x80\xFD[Pb\0\0\x1Fb\0\0%V[b\0\0\xD9V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80Th\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15b\0\0vW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80T`\x01`\x01`@\x1B\x03\x90\x81\x16\x14b\0\0\xD6W\x80T`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x81\x17\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PV[`\x80QaQ\x99b\0\x01\0_9_\x81\x81a)\xBA\x01R\x81\x81a)\xE3\x01Ra+\xC8\x01RaQ\x99_\xF3\xFE`\x80`@R`\x046\x10a\x02\x12W_5`\xE0\x1C\x80cun\x86R\x11a\x01\x1EW\x80c\xBF\x9B\x16\xC8\x11a\0\xA8W\x80c\xC9\x99\xA8\xB4\x11a\0mW\x80c\xC9\x99\xA8\xB4\x14a\x05\xDCW\x80c\xCC\xBF\x81\x99\x14a\x05\xFBW\x80c\xCC\xEA\xC0\x19\x14a\x06\x1AW\x80c\xEE}R\xD1\x14a\x069W\x80c\xF9\xC6p\xC3\x14a\x06XW_\x80\xFD[\x80c\xBF\x9B\x16\xC8\x14a\x05LW\x80c\xC0\xAEd\xF7\x14a\x05kW\x80c\xC2X\n-\x14a\x05\x8AW\x80c\xC2\xB4)\x86\x14a\x05\xA9W\x80c\xC3\xAA\xAAZ\x14a\x05\xBDW_\x80\xFD[\x80c\x97o>\xB9\x11a\0\xEEW\x80c\x97o>\xB9\x14a\x04\xC1W\x80c\x9D\x1B\x1B\xE1\x14a\x04\xD5W\x80c\xA6B\xBA\x0F\x14a\x04\xE9W\x80c\xAD<\xB1\xCC\x14a\x05\x08W\x80c\xB4r+\xC4\x14a\x058W_\x80\xFD[\x80cun\x86R\x14a\x04PW\x80c~\xAA\xC8\xF2\x14a\x04oW\x80c\x94G\xCF\xD4\x14a\x04\x83W\x80c\x97l\x98\xB5\x14a\x04\xA2W_\x80\xFD[\x80cA\xAD\x06\x9C\x11a\x01\x9FW\x80cR\xD1\x90-\x11a\x01oW\x80cR\xD1\x90-\x14a\x03\xA9W\x80c[\xFFv\xD9\x14a\x03\xBDW\x80ce\xB3\x94\xAF\x14a\x03\xE9W\x80cn\xC0\xF2?\x14a\x04\x12W\x80cq\xD7m\xAE\x14a\x041W_\x80\xFD[\x80cA\xAD\x06\x9C\x14a\x039W\x80cF\xC5\xBB\xBD\x14a\x03XW\x80cG\xE8\"\x95\x14a\x03wW\x80cO\x1E\xF2\x86\x14a\x03\x96W_\x80\xFD[\x80c\"\x1C\xDDN\x11a\x01\xE5W\x80c\"\x1C\xDDN\x14a\x02\x99W\x80c&\xCF]\xEF\x14a\x02\xB8W\x80c(\x1E\x8B\xFE\x14a\x02\xDAW\x80c*8\x89\x98\x14a\x02\xF9W\x80c1\xFFA\xC8\x14a\x03\rW_\x80\xFD[\x80c\r\x8En,\x14a\x02\x16W\x80c\x12:\xBB(\x14a\x02@W\x80c\x1C\xE3\xF9\xBC\x14a\x02VW\x80c =\x01\x14\x14a\x02jW[_\x80\xFD[4\x80\x15a\x02!W_\x80\xFD[Pa\x02*a\x06\x84V[`@Qa\x027\x91\x90a?eV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02KW_\x80\xFD[Pa\x02Ta\x06\xF0V[\0[4\x80\x15a\x02aW_\x80\xFD[Pa\x02Ta\x07\xCBV[4\x80\x15a\x02uW_\x80\xFD[Pa\x02\x89a\x02\x846`\x04a?\x9BV[a\x08\xF5V[`@Q\x90\x15\x15\x81R` \x01a\x027V[4\x80\x15a\x02\xA4W_\x80\xFD[Pa\x02Ta\x02\xB36`\x04a@:V[a\t5V[4\x80\x15a\x02\xC3W_\x80\xFD[Pa\x02\xCCa\x0BKV[`@Q\x90\x81R` \x01a\x027V[4\x80\x15a\x02\xE5W_\x80\xFD[Pa\x02\xCCa\x02\xF46`\x04a@\xEDV[a\x0BqV[4\x80\x15a\x03\x04W_\x80\xFD[Pa\x02\xCCa\x0B\x96V[4\x80\x15a\x03\x18W_\x80\xFD[Pa\x03,a\x03'6`\x04aA\x04V[a\x0B\xBCV[`@Qa\x027\x91\x90aA\x7FV[4\x80\x15a\x03DW_\x80\xFD[Pa\x02\xCCa\x03S6`\x04a@\xEDV[a\r\x80V[4\x80\x15a\x03cW_\x80\xFD[Pa\x02\x89a\x03r6`\x04aA\x04V[a\r\xC5V[4\x80\x15a\x03\x82W_\x80\xFD[Pa\x02\xCCa\x03\x916`\x04a@\xEDV[a\x0E#V[a\x02Ta\x03\xA46`\x04aA\xA5V[a\x0EHV[4\x80\x15a\x03\xB4W_\x80\xFD[Pa\x02\xCCa\x0EgV[4\x80\x15a\x03\xC8W_\x80\xFD[Pa\x03\xDCa\x03\xD76`\x04a@\xEDV[a\x0E\x82V[`@Qa\x027\x91\x90aBbV[4\x80\x15a\x03\xF4W_\x80\xFD[Pa\x03\xFDa\x0F\x05V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x027V[4\x80\x15a\x04\x1DW_\x80\xFD[Pa\x02Ta\x04,6`\x04aB\xAEV[a\x0F%V[4\x80\x15a\x04<W_\x80\xFD[Pa\x02Ta\x04K6`\x04aC!V[a\x13\x86V[4\x80\x15a\x04[W_\x80\xFD[Pa\x02Ta\x04j6`\x04aC\xE6V[a\x16&V[4\x80\x15a\x04zW_\x80\xFD[Pa\x03\xDCa\x17\x95V[4\x80\x15a\x04\x8EW_\x80\xFD[Pa\x02\x89a\x04\x9D6`\x04aA\x04V[a\x18\x16V[4\x80\x15a\x04\xADW_\x80\xFD[Pa\x02Ta\x04\xBC6`\x04a@:V[a\x18TV[4\x80\x15a\x04\xCCW_\x80\xFD[Pa\x02\xCCa\x1C\x04V[4\x80\x15a\x04\xE0W_\x80\xFD[Pa\x02\xCCa\x1C\x18V[4\x80\x15a\x04\xF4W_\x80\xFD[Pa\x02Ta\x05\x036`\x04aB\xAEV[a\x1C)V[4\x80\x15a\x05\x13W_\x80\xFD[Pa\x02*`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[4\x80\x15a\x05CW_\x80\xFD[Pa\x02\xCCa\x1D|V[4\x80\x15a\x05WW_\x80\xFD[Pa\x02\x89a\x05f6`\x04a@\xEDV[a\x1D\xA2V[4\x80\x15a\x05vW_\x80\xFD[Pa\x02Ta\x05\x856`\x04a@\xEDV[a\x1D\xACV[4\x80\x15a\x05\x95W_\x80\xFD[Pa\x02Ta\x05\xA46`\x04a@\xEDV[a\x1F0V[4\x80\x15a\x05\xB4W_\x80\xFD[Pa\x02\xCCa!%V[4\x80\x15a\x05\xC8W_\x80\xFD[Pa\x02\xCCa\x05\xD76`\x04a@\xEDV[a!KV[4\x80\x15a\x05\xE7W_\x80\xFD[Pa\x03\xFDa\x05\xF66`\x04a@\xEDV[a!pV[4\x80\x15a\x06\x06W_\x80\xFD[Pa\x02Ta\x06\x156`\x04aD\x84V[a!\xD7V[4\x80\x15a\x06%W_\x80\xFD[Pa\x02\x89a\x0646`\x04aE3V[a$\xE3V[4\x80\x15a\x06DW_\x80\xFD[Pa\x02\x89a\x06S6`\x04a@\xEDV[a%HV[4\x80\x15a\x06cW_\x80\xFD[Pa\x06wa\x06r6`\x04a@\xEDV[a%RV[`@Qa\x027\x91\x90aE\xABV[```@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mProtocolConfig`\x90\x1B\x81RPa\x06\xB6_a'\x15V[a\x06\xC0`\x04a'\x15V[a\x06\xC9_a'\x15V[`@Q` \x01a\x06\xDC\x94\x93\x92\x91\x90aE\xBDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x05\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\x079WP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a\x07WW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x17`\x01`@\x1B\x17\x81Ua\x07\x80a'\xA4V[\x80T`\xFF`@\x1B\x19\x16\x81U`@Q`\x01`\x01`@\x1B\x03\x83\x16\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x1BW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08?\x91\x90aF:V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x08wW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\x08\x7Fa'\xA4V[_a\x08\x88a(gV[`\x0B\x81\x01T\x90\x91P_a\x08\x9A\x82a(\x8BV[\x90P\x80\x82\x7F\x15\xAA\xAFG^\xF4\x07T?Qd\xF5}\xCFW\xF7\xF98\x16\xF5[\xAEw\xCA\t\xEF\xC4E\xBA@\xEE\xF7\x84\x86`\r\x01T`\x01Ca\x08\xD2\x91\x90aFiV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01`@Q\x80\x91\x03\x90\xA3PPPV[_\x80a\x08\xFFa(gV[`\x0B\x81\x01T_\x90\x81R`\x03\x90\x91\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x90\x96\x16\x83R\x94\x90R\x92\x90\x92 T`\xFF\x16\x92\x91PPV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x16`\x01\x14a\t\x89W`@QcoOs\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x05\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\t\xD2WP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a\t\xF0W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x17`\x01`@\x1B\x17\x81U_a\n\x1Aa(gV[\x90P_a\n,`\x07`\xF8\x1B`\x01aF\x90V[\x90Pa\nI\x81a\nA`\x01`\xFB\x1B`\x01aF\x90V[\x8D\x8D\x8Da(\xDCV[`@Q\x80`@\x01`@R\x80C\x81R` \x01\x8C\x8C\x8C\x8C\x8C\x8C\x8C`@Q` \x01a\nw\x97\x96\x95\x94\x93\x92\x91\x90aG\xE6V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x90\x92R_\x84\x81R`\x17\x86\x01\x82R\x91\x90\x91 \x82Q\x81U\x91\x01Q`\x01\x90\x91\x01U`\xF8`\x07\x90\x1B\x81\x7F Mk\x80\x12\x11T\xCD\x87\xD9\x9C\xF5Lc\x9A=\xD0\xA5;0\x84'p\x98\xDE\x97.\xBD\xD3Lk\xE9\x8D\x8D\x8D\x8D\x8D\x8D\x8D`@Qa\n\xEF\x97\x96\x95\x94\x93\x92\x91\x90aG\xE6V[`@Q\x80\x91\x03\x90\xA3PP\x80T`\xFF`@\x1B\x19\x16\x81U`@Q`\x01`\x01`@\x1B\x03\x83\x16\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPPPPV[_\x80a\x0BUa(gV[`\x0B\x81\x01T_\x90\x81R`\t\x90\x91\x01` R`@\x90 T\x92\x91PPV[_a\x0B{\x82a)-V[a\x0B\x83a(gV[_\x92\x83R`\x07\x01` RP`@\x90 T\x90V[_\x80a\x0B\xA0a(gV[`\x0B\x81\x01T_\x90\x81R`\x06\x90\x91\x01` R`@\x90 T\x92\x91PPV[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01R``\x91\x81\x01\x82\x90R\x81\x81\x01\x91\x90\x91Ra\x0B\xE7\x83a)YV[a\x0C\x07W`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x08nV[a\x0C\x0Fa(gV[_\x84\x81R`\x04\x91\x90\x91\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x85R\x90\x83R\x92\x81\x90 \x81Q`\x80\x81\x01\x83R\x81T\x85\x16\x81R`\x01\x82\x01T\x90\x94\x16\x92\x84\x01\x92\x90\x92R`\x02\x82\x01\x80T\x91\x84\x01\x91a\x0Cg\x90aI\xAEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0C\x93\x90aI\xAEV[\x80\x15a\x0C\xDEW\x80`\x1F\x10a\x0C\xB5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\xDEV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0C\xC1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\x0C\xF7\x90aI\xAEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r#\x90aI\xAEV[\x80\x15a\rnW\x80`\x1F\x10a\rEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\rnV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\rQW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P[\x92\x91PPV[_a\r\x8A\x82a)wV[a\r\xAAW`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x08nV[a\r\xB2a(gV[_\x92\x83R`\x08\x01` RP`@\x90 T\x90V[_a\r\xCF\x83a)wV[a\r\xEFW`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x08nV[a\r\xF7a(gV[_\x93\x84R`\x02\x01` \x90\x81R`@\x80\x85 `\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x85R\x92\x90RP\x90 T`\xFF\x16\x90V[_a\x0E-\x82a)-V[a\x0E5a(gV[_\x92\x83R`\t\x01` RP`@\x90 T\x90V[a\x0EPa)\xAFV[a\x0EY\x82a*UV[a\x0Ec\x82\x82a*\xFCV[PPV[_a\x0Epa+\xBDV[P_\x80Q` aQy\x839\x81Q\x91R\x90V[``a\x0E\x8D\x82a)-V[a\x0E\x95a(gV[`\x05\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0E\xF9W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x0E\xDBW[PPPPP\x90P\x91\x90PV[_\x80_a\x0F\x10a(gV[\x90P\x80`\x0B\x01T\x92P\x80`\r\x01T\x91PP\x90\x91V[_a\x0F.a(gV[\x90P_\x7F\x98\xBC2\xBB@E\xAB\xD7\x9Ff0]G'&y\x08\x7F\xF1-\x8A\x03\x15,\xE1J\tT\xB7p\x9C\0\x90P`\x01_\x88\x81R`\x0E\x84\x01` R`@\x90 T`\xFF\x16`\x03\x81\x11\x15a\x0FzWa\x0FzaF|V[\x14a\x0F\x9BW`@Qc5\x86\xEF\xA1`\xE0\x1B\x81R`\x04\x81\x01\x88\x90R`$\x01a\x08nV[`\x0B\x82\x01T_\x88\x81R` \x83\x90R`@\x80\x82 T\x90Qa\x104\x91\x7F\x8A}1\x90\xA5\x14\xCD\xB5\xD5|\x84\x1C\xB78.\x85\xAD!\xB8\x92\xB7\xFD\xFD;\x87\xAB\0\t%\xD9\xD5\x01\x91\x85\x91\x8D\x91a\x0F\xE8\x90\x8C\x90\x8C\x90aI\xE6V[`@\x80Q\x91\x82\x90\x03\x82 ` \x83\x01\x96\x90\x96R\x81\x01\x93\x90\x93R``\x83\x01\x91\x90\x91R`\x80\x82\x01R`\xA0\x81\x01\x91\x90\x91R`\xC0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a,\x06V[\x90P_a\x10v\x82\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa-\n\x92PPPV[\x90P_\x85`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x83`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P_\x86`\x03\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _\x84`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x81\x15\x80\x15a\x11\x0CWP\x80\x15[\x15a\x11<W`@Qc\x17\x03\xBF\x1D`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x8D\x90R`D\x01a\x08nV[_\x8C\x81R`\x01\x87\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15a\x11\x93W`@Qc\x0CK\x0B\x99`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x8D\x90R`D\x01a\x08nV[_\x8C\x81R`\x01\x87\x81\x01` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x88\x16\x85R\x90\x91R\x90\x91 \x80T`\xFF\x19\x16\x90\x91\x17\x90U\x81\x15a\x11\xF6W_\x8C\x81R`\x02\x87\x01` \x90\x81R`@\x80\x83 \x87\x84R\x90\x91R\x81 \x80T\x90\x91\x90a\x11\xF1\x90aI\xF5V[\x90\x91UP[\x80\x15a\x12)W_\x8C\x81R`\x03\x87\x01` \x90\x81R`@\x80\x83 \x87\x84R\x90\x91R\x81 \x80T\x90\x91\x90a\x12$\x90aI\xF5V[\x90\x91UP[\x82`\x01`\x01`\xA0\x1B\x03\x16\x8C\x7FBT\xD9H@+\xA0Y\xC0/\x9F\xEE\xCF\xCA\t\xB4\x0C\x0B9\x02j4J\x15\x9E:vy\x80z\x83\xD2\x8D\x8D\x8D\x8D`@Qa\x12i\x94\x93\x92\x91\x90aJ\rV[`@Q\x80\x91\x03\x90\xA3_\x85\x81R`\t\x88\x01` \x90\x81R`@\x80\x83 T`\x01\x8B\x01\x90\x92R\x82 Ta\x12\x98\x91\x90aFiV[_\x8E\x81R`\x01\x8A\x01` \x90\x81R`@\x80\x83 T`\x03\x8C\x01\x83R\x81\x84 \x8A\x85R\x90\x92R\x90\x91 T\x91\x92P\x14\x80\x15a\x12\xF7WP_\x81\x11a\x12\xD7W`\x01a\x12\xD9V[\x80[_\x8E\x81R`\x02\x89\x01` \x90\x81R`@\x80\x83 \x89\x84R\x90\x91R\x90 T\x10\x15[\x15a\x13wW_\x8D\x81R`\x0E\x89\x01` R`@\x81 \x80T`\xFF\x19\x16`\x02\x17\x90Ua\x13\x1F\x8Ea(\x8BV[\x90P\x80\x8E\x7F\x15\xAA\xAFG^\xF4\x07T?Qd\xF5}\xCFW\xF7\xF98\x16\xF5[\xAEw\xCA\t\xEF\xC4E\xBA@\xEE\xF7\x89\x8C`\r\x01T`\x01Ca\x13W\x91\x90aFiV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01`@Q\x80\x91\x03\x90\xA3P[PPPPPPPPPPPPPV[_a\x13\x8Fa(gV[\x90P`\x01_\x8B\x81R`\x0F\x83\x01` R`@\x90 T`\xFF\x16`\x02\x81\x11\x15a\x13\xB7Wa\x13\xB7aF|V[\x14a\x13\xD8W`@Qcy\x95\xBB\xCF`\xE0\x1B\x81R`\x04\x81\x01\x8B\x90R`$\x01a\x08nV[_\x8A\x81R`\x10\x82\x01` R`@\x81 T\x90\x80\x80a\x13\xFD\x84\x8F\x8F\x8F\x8F\x8F\x8F\x8F\x8F\x8Fa-2V[\x92P\x92P\x92P_a\x14\x10\x8F\x86\x86\x85a.\x9FV[\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16\x8F\x7F\x7F\xAE\xF7E\xAFy\xA3\xAF#\xD4\x19\x9C\xE06iu\xCC-\xD7\x14\x93\x84\xD613}\x13;\"k[\xE4\x85\x8D\x8D\x8D\x8D`@Qa\x14T\x95\x94\x93\x92\x91\x90aJ>V[`@Q\x80\x91\x03\x90\xA3\x80a\x14lWPPPPPPa\x16\x1BV[PPP_\x82\x81R`\x0E\x84\x01` R`@\x90 \x80T`\xFF\x19\x16`\x03\x17\x90UP`\x0B\x82\x01\x81\x90Ua\x14\x9B\x8B\x82a/uV[_\x81\x81R`\x01\x83\x01` R`@\x81 \x80T\x90\x91\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xC6Wa\x14\xC6aA\x91V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14\xF9W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x14\xE4W\x90P[P\x90P_[\x82T\x81\x10\x15a\x15\xD4W\x82\x81\x81T\x81\x10a\x15\x19Wa\x15\x19aJvV[\x90_R` _ \x90`\x04\x02\x01`\x03\x01\x80Ta\x153\x90aI\xAEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15_\x90aI\xAEV[\x80\x15a\x15\xAAW\x80`\x1F\x10a\x15\x81Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\xAAV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\x8DW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x82\x82\x81Q\x81\x10a\x15\xC1Wa\x15\xC1aJvV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x14\xFEV[P\x8C\x83\x7Fln\xB3X\x81\xF8\x05\xF2\x84\xBEY\xB8@\xB4\xEA\x99\x0F\x96\xB1=\xD9\xE7\x19,E\x81\xE1\xBC\x81\xC0\xAF\x98\x8E\x8E\x8E\x8E\x87`@Qa\x16\x0E\x95\x94\x93\x92\x91\x90aK{V[`@Q\x80\x91\x03\x90\xA3PPPP[PPPPPPPPPV[a\x16.a(gV[_\x88\x81R`\n\x91\x90\x91\x01` R`@\x90 T`\xFF\x16a\x16cW`@Qc+s\xD9\xB7`\xE0\x1B\x81R`\x04\x81\x01\x88\x90R`$\x01a\x08nV[_\x87\x81R\x7F\x98\xBC2\xBB@E\xAB\xD7\x9Ff0]G'&y\x08\x7F\xF1-\x8A\x03\x15,\xE1J\tT\xB7p\x9C\x04` \x90\x81R`@\x80\x83 \x90Qa\x17=\x92\x8B\x91\x7F\xC6\x87\xB65%\xEF \xD9TB\xD5k\xD1@\xC9\xDAl\x10\xC6\xE2?H$\x86\xC2F\xC0R6\xCE\xEF\x9C\x91\x83\x91a\x16\xCC\x91\x8E\x91\x8E\x91\x01aL\xDDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x88\x88`@Qa\x16\xF2\x92\x91\x90aI\xE6V[`@Q\x90\x81\x90\x03\x81 a\x17 \x94\x93\x92\x91` \x01\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x88\x88a/\xAFV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x88\x7F\xEA\x16@\xF3M\xEA\xDA/\x81]\xEF\x0E\x11.l\xD2IS8I\xA0\xFDS\xE9);\xA3M|F\x88\xCB\x89\x89\x89\x89\x89\x89`@Qa\x17\x83\x96\x95\x94\x93\x92\x91\x90aM\x04V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPV[``_a\x17\xA0a(gV[\x90P\x80`\x05\x01_\x82`\x0B\x01T\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x18\x0BW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x17\xEDW[PPPPP\x91PP\x90V[_a\x18 \x83a)-V[a\x18(a(gV[_\x93\x84R`\x03\x01` \x90\x81R`@\x80\x85 `\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x85R\x92\x90RP\x90 T`\xFF\x16\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xA4W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xC8\x91\x90aF:V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18\xFBW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x08nV[a\x19\x03a'\xA4V[_a\x19\x0Ca(gV[`\x0B\x81\x01T\x81T\x91\x92P\x90_\x90a\x19$\x90`\x01aF\x90V[\x90Pa\x192\x81\x8B\x8B\x8Ba0\xD9V[_\x81\x81R`\x0E\x84\x01` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x86\x01\x82R\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R\x91\x92\x90\x91\x90\x84\x90\x84\x01[\x82\x82\x10\x15a\x1A\xE8W_\x84\x81R` \x90\x81\x90 `@\x80Q`\x80\x81\x01\x82R`\x04\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x84R`\x01\x82\x01T\x16\x93\x83\x01\x93\x90\x93R`\x02\x83\x01\x80T\x92\x93\x92\x91\x84\x01\x91a\x19\xC9\x90aI\xAEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x19\xF5\x90aI\xAEV[\x80\x15a\x1A@W\x80`\x1F\x10a\x1A\x17Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1A@V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1A#W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\x1AY\x90aI\xAEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1A\x85\x90aI\xAEV[\x80\x15a\x1A\xD0W\x80`\x1F\x10a\x1A\xA7Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1A\xD0V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1A\xB3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x19tV[PPPP\x90P\x80\x89`@Q` \x01a\x1B\x01\x92\x91\x90aMkV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x1B>\x7F\x98\xBC2\xBB@E\xAB\xD7\x9Ff0]G'&y\x08\x7F\xF1-\x8A\x03\x15,\xE1J\tT\xB7p\x9C\0\x90V[_\x84\x81R` \x91\x82R`@\x90\x81\x90 \x92\x90\x92U\x81Q\x80\x83\x01\x83RC\x81R\x91Q\x82\x82\x01\x91a\x1By\x91\x8F\x91\x8F\x91\x8F\x91\x8F\x91\x8F\x91\x8F\x91\x8F\x91\x01aG\xE6V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x90\x92R_\x85\x81R`\x17\x88\x01\x82R\x82\x90 \x83Q\x81U\x92\x01Q`\x01\x90\x92\x01\x91\x90\x91UQ\x83\x90\x83\x90\x7F Mk\x80\x12\x11T\xCD\x87\xD9\x9C\xF5Lc\x9A=\xD0\xA5;0\x84'p\x98\xDE\x97.\xBD\xD3Lk\xE9\x90a\x1B\xEF\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90aG\xE6V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPPPPV[_\x80a\x1C\x0Ea(gV[`\x0B\x01T\x92\x91PPV[_\x80a\x1C\"a(gV[T\x92\x91PPV[_\x85\x81R\x7F\x98\xBC2\xBB@E\xAB\xD7\x9Ff0]G'&y\x08\x7F\xF1-\x8A\x03\x15,\xE1J\tT\xB7p\x9C\x06` R`@\x90 T\x7F\x98\xBC2\xBB@E\xAB\xD7\x9Ff0]G'&y\x08\x7F\xF1-\x8A\x03\x15,\xE1J\tT\xB7p\x9C\0\x90`\xFF\x16a\x1C\x9AW`@Qb=\x8BM`\xE4\x1B\x81R`\x04\x81\x01\x87\x90R`$\x01a\x08nV[_a\x1D)\x82`\x05\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x88\x7FJ\xF2\xC2\x96|l\x1F\xA8pl%\x04\xBB\x0C\xAD\x84?\x8D\x0F\xACy\xF3\x1F-\x02\x06\xE7G\xF0I\xFC\xA2\x8A\x88\x88`@Qa\x1C\xE4\x92\x91\x90aI\xE6V[`@Q\x90\x81\x90\x03\x81 a\x1D\x0C\x93\x92\x91` \x01\x92\x83R` \x83\x01\x91\x90\x91R`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x89\x89a/\xAFV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x87\x7F\xEE\x1B\x8F)\xD5\x16\xD5c\xF4\x1F*;u\xC2\0u/\xEBQ.bV\xEBV\x1D\x17\xA4\xE6\x1E\xE63^\x88\x88\x88\x88`@Qa\x1Dk\x94\x93\x92\x91\x90aJ\rV[`@Q\x80\x91\x03\x90\xA3PPPPPPPV[_\x80a\x1D\x86a(gV[`\x0B\x81\x01T_\x90\x81R`\x08\x90\x91\x01` R`@\x90 T\x92\x91PPV[_a\rz\x82a6\x16V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xFCW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E \x91\x90aF:V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1ESW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x08nV[_a\x1E\\a(gV[\x90P\x80`\x0B\x01T\x82\x03a\x1E\x85W`@Qc\x03\x0F]U`\xE3\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x08nV[a\x1E\x8E\x82a)wV[a\x1E\xAEW`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x08nV[_\x82\x81R`\n\x82\x01` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x90\x81\x16`\x01\x17\x90\x91U`\x0E\x85\x01\x83R\x81\x84 \x80T\x90\x91\x16\x90U`\x0C\x84\x01T\x80\x84R`\x10\x85\x01\x90\x92R\x90\x91 T\x83\x90\x03a\x1F\x01Wa\x1F\x01\x81a6NV[`@Q\x83\x90\x7F\xDA\x07]\t\x19\x8D ~:\x91\x8DK\x8D\xFC\x87\xDF-`\xA0\x0B\xE7\x03\xFD9\xEA\xAC\x90\x96-\xA0\xB7\xF0\x90_\x90\xA2PPPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\x80W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xA4\x91\x90aF:V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1F\xD7W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x08nV[_a\x1F\xE0a(gV[\x90P\x80`\r\x01T\x82\x03a \tW`@Qc\xF0\xE7\x81\xC1`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x08nV[_\x82\x81R`\x0F\x82\x01` R`@\x81 T`\xFF\x16\x90`\x02\x82`\x02\x81\x11\x15a 1Wa 1aF|V[\x14\x90P_`\x01\x83`\x02\x81\x11\x15a IWa IaF|V[\x14\x80\x15a \x88WP`\x03_\x86\x81R`\x10\x86\x01` \x90\x81R`@\x80\x83 T\x83R`\x0E\x88\x01\x90\x91R\x90 T`\xFF\x16`\x03\x81\x11\x15a \x86Wa \x86aF|V[\x14[\x90P\x81\x15\x80\x15a \x96WP\x80\x15[\x15a \xB7W`@Qcy\x95\xBB\xCF`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R`$\x01a\x08nV[a \xC0\x85a6NV[_\x85\x81R\x7F\x98\xBC2\xBB@E\xAB\xD7\x9Ff0]G'&y\x08\x7F\xF1-\x8A\x03\x15,\xE1J\tT\xB7p\x9C\x06` R`@\x80\x82 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x86\x91\x7F\x03C`\x13\x07_\x8D\x1A\xBB\x17\x81\xDB\xCA\xF4\x18\xFCv\xFCy}\x1A\xB5\x1C8fL\x1AQ\xF3\xCDW\xF9\x91\xA2PPPPPV[_\x80a!/a(gV[`\x0B\x81\x01T_\x90\x81R`\x07\x90\x91\x01` R`@\x90 T\x92\x91PPV[_a!U\x82a)-V[a!]a(gV[_\x92\x83R`\x06\x01` RP`@\x90 T\x90V[_\x80a!{\x83a)YV[a!\x9BW`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x08nV[_a!\xA4a(gV[_\x94\x85R`\x17\x01` \x90\x81R`@\x94\x85\x90 \x85Q\x80\x87\x01\x90\x96R\x80T\x80\x87R`\x01\x90\x91\x01T\x95\x90\x91\x01\x85\x90R\x94\x92PPPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"'W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"K\x91\x90aF:V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\"~W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x08nV[\x85_\x03a\"\x9EW`@Qc\t\x92\xF7\xAD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x90\x03a\"\xBFW`@Qc\xB5H\x91G`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x82\x90\x03a\"\xE0W`@Qc/\x94\x14\x11`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`@\x1B\x03\x16_\x03a#\tW`@Qc\x02\xFA})`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x82\x81\x10\x15a$\x9AW6\x84\x84\x83\x81\x81\x10a#&Wa#&aJvV[``\x02\x91\x90\x91\x01\x91Pa#>\x90P` \x82\x01\x82aM\xADV[`\x01`\x01`@\x1B\x03\x16_\x03a#fW`@Qc2\x12!u`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a#v``\x82\x01`@\x83\x01aM\xADV[`\x01`\x01`@\x1B\x03\x16a#\x8F`@\x83\x01` \x84\x01aM\xADV[`\x01`\x01`@\x1B\x03\x16\x11\x15a$\0Wa#\xAB` \x82\x01\x82aM\xADV[a#\xBB`@\x83\x01` \x84\x01aM\xADV[a#\xCB``\x84\x01`@\x85\x01aM\xADV[`@Qcy\x0C\xEE\x07`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x93\x84\x16`\x04\x82\x01R\x91\x83\x16`$\x83\x01R\x90\x91\x16`D\x82\x01R`d\x01a\x08nV[_[\x82\x81\x10\x15a$\x90Wa$\x17` \x83\x01\x83aM\xADV[`\x01`\x01`@\x1B\x03\x16\x86\x86\x83\x81\x81\x10a$2Wa$2aJvV[a$H\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91PaM\xADV[`\x01`\x01`@\x1B\x03\x16\x03a$\x88Wa$c` \x83\x01\x83aM\xADV[`@Qc\x06\xC6~G`\xE4\x1B\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01a\x08nV[`\x01\x01a$\x02V[PP`\x01\x01a#\x0BV[P\x85\x7F\xC4/\x1E\xCA\xC8\xD4\x88\x1E\xF9\xFD3\\\xA9\x8E\xE7%JCk\x92\xBA\x87J`\x9EP\xA7\xD3\x0CH{\x8D\x86\x86\x86\x86\x86`@Qa$\xD3\x95\x94\x93\x92\x91\x90aM\xC6V[`@Q\x80\x91\x03\x90\xA2PPPPPPV[_\x80a$\xEDa(gV[\x90P`\x02_\x84\x81R`\x0F\x83\x01` R`@\x90 T`\xFF\x16`\x02\x81\x11\x15a%\x15Wa%\x15aF|V[\x14\x80\x15a%0WP_\x83\x81R`\x10\x82\x01` R`@\x90 T\x84\x14[\x80\x15a%@WPa%@\x84a6\x16V[\x94\x93PPPPV[_a\rz\x82a)wV[``a%]\x82a)-V[a%ea(gV[`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a'\nW_\x84\x81R` \x90\x81\x90 `@\x80Q`\x80\x81\x01\x82R`\x04\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x84R`\x01\x82\x01T\x16\x93\x83\x01\x93\x90\x93R`\x02\x83\x01\x80T\x92\x93\x92\x91\x84\x01\x91a%\xEB\x90aI\xAEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta&\x17\x90aI\xAEV[\x80\x15a&bW\x80`\x1F\x10a&9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a&bV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a&EW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta&{\x90aI\xAEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta&\xA7\x90aI\xAEV[\x80\x15a&\xF2W\x80`\x1F\x10a&\xC9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a&\xF2V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a&\xD5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a%\x96V[PPPP\x90P\x91\x90PV[``_a'!\x83a6\x81V[`\x01\x01\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a'?Wa'?aA\x91V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a'iW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[_\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a'sWP\x93\x92PPPV[_a'\xADa(gV[\x80T_\x90\x81R`\x0E\x82\x01` R`@\x81 T\x91\x92P`\xFF\x90\x91\x16\x90`\x01\x82`\x03\x81\x11\x15a'\xDCWa'\xDCaF|V[\x14\x80a'\xF9WP`\x02\x82`\x03\x81\x11\x15a'\xF7Wa'\xF7aF|V[\x14[\x90P_`\x01`\x0C\x85\x01T_\x90\x81R`\x0F\x86\x01` R`@\x90 T`\xFF\x16`\x02\x81\x11\x15a('Wa('aF|V[\x14\x90P\x81\x80a(3WP\x80[\x15a(aW\x83T`\x0C\x85\x01T`@Qc\x03\xC0\x10_`\xE1\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x08nV[PPPPV[\x7F\x80\xF3XZ\xF8h\x06\xC5wC\x03\xB0l\x1E\xE6@\xAA\x83\xB6\xEF>E\xDFI\xBB&\xC8RE\0\xC2\0\x90V[_\x80a(\x95a(gV[\x90P\x80`\x0C\x01_\x81Ta(\xA7\x90aI\xF5V[\x91\x82\x90UP_\x81\x81R`\x0F\x83\x01` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16`\x01\x17\x90U`\x10\x90\x94\x01\x90R\x91\x90\x91 \x92\x90\x92UP\x90V[_a(\xE5a(gV[\x90Pa(\xF3\x86\x85\x85\x85a0\xD9V[_\x86\x81R`\x0E\x82\x01` R`@\x90 \x80T`\xFF\x19\x16`\x03\x17\x90U`\x0B\x81\x01\x86\x90U`\x0C\x81\x01\x85\x90Ua)%\x85\x87a/uV[PPPPPPV[a)6\x81a6\x16V[a)VW`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x08nV[PV[_a)ba(gV[_\x92\x83R`\x01\x01` RP`@\x90 T\x15\x15\x90V[_\x80a)\x81a(gV[_\x84\x81R`\x0E\x91\x90\x91\x01` R`@\x90 T`\xFF\x16`\x03\x81\x11\x15a)\xA7Wa)\xA7aF|V[\x14\x15\x92\x91PPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a*5WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a*)_\x80Q` aQy\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a*SW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*\xA5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*\xC9\x91\x90aF:V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a)VW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x08nV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a+VWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra+S\x91\x81\x01\x90aNbV[`\x01[a+~W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x08nV[_\x80Q` aQy\x839\x81Q\x91R\x81\x14a+\xAEW`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x08nV[a+\xB8\x83\x83a7XV[PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a*SW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x82R`\x0E\x81RmProtocolConfig`\x90\x1B` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\x01\x81R`1`\xF8\x1B\x90\x82\x01R\x81Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81\x83\x01R\x7F\xA3\xDE\x18\x80\xCF\x08>\x83\x18\xB7zye\xD0-\xD9v^\x85\xA4\x8EA\x8ADc\xAFz\rW\xB4\xB3\xEE\x81\x84\x01R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xC0\x82\x01\x84R\x80Q\x90\x83\x01 a\x19\x01`\xF0\x1B`\xE0\x83\x01R`\xE2\x82\x01Ra\x01\x02\x80\x82\x01\x94\x90\x94R\x82Q\x80\x82\x03\x90\x94\x01\x84Ra\x01\"\x01\x90\x91R\x81Q\x91\x01 \x90V[_\x80_\x80a-\x18\x86\x86a7\xADV[\x92P\x92P\x92Pa-(\x82\x82a7\xF6V[P\x90\x94\x93PPPPV[_\x80\x80\x89\x15\x80a-@WP\x87\x15[\x15a-aW`@Qc\x18\x85Zy`\xE2\x1B\x81R`\x04\x81\x01\x8D\x90R`$\x01a\x08nV[a-m\x8B\x8B\x8B\x8Ba8\xAEV[\x91Pa-\xEC\x7Fl\xF4}\x8F[\xEDs\xAD*\x8C`\xAD\xBD\xD5\xB5U\xD6e5}\xB7Xf*\xBDI\xEA\x97\x7F\x80\xD1\x9A\x8Ea-\x9Ca(gV[`\r\x01T\x8F\x86\x8A\x8A`@Qa-\xB2\x92\x91\x90aI\xE6V[`@\x80Q\x91\x82\x90\x03\x82 ` \x83\x01\x97\x90\x97R\x81\x01\x94\x90\x94R``\x84\x01\x92\x90\x92R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x81\x01\x91\x90\x91R`\xE0\x01a\x10\x19V[\x90Pa.-\x81\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa-\n\x92PPPV[\x92Pa.7a(gV[_\x8E\x81R`\x03\x91\x90\x91\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x90\x91R\x90 T`\xFF\x16a.\x8FW`@Qc\xA3\xF4\xAF\xEB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x8D\x90R`D\x01a\x08nV[\x9AP\x9AP\x9A\x97PPPPPPPPV[_\x80a.\xA9a(gV[_\x87\x81R`\x12\x82\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x89\x16\x84R\x90\x91R\x90 T\x90\x91P`\xFF\x16\x15a/\x03W`@Qc$\xA0\xBB\x1B`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01a\x08nV[_\x86\x81R`\x12\x82\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x88\x16\x84R\x82R\x80\x83 \x80T`\xFF\x19\x16`\x01\x17\x90U\x88\x83R`\x13\x84\x01\x82R\x80\x83 \x86\x84R\x90\x91R\x81 \x80T\x82\x90a/R\x90aI\xF5V[\x91\x82\x90UP_\x96\x87R`\x05\x90\x92\x01` RP`@\x90\x94 T\x90\x93\x14\x94\x93PPPPV[_a/~a(gV[_\x84\x81R`\x0F\x82\x01` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16`\x02\x17\x90U`\x10\x84\x01\x90\x91R\x90 \x92\x90\x92UP`\r\x01UV[_\x80a/\xB9a(gV[\x90Pa0\x02a/\xC7\x86a,\x06V[\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa-\n\x92PPPV[`\x0B\x82\x01T_\x90\x81R`\x03\x83\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T\x90\x92P`\xFF\x16a0`W`@Qc\x1A\xC5\x16I`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01a\x08nV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R` \x88\x90R`@\x90 T`\xFF\x16\x15a0\xABW`@Qc\x9D'\x99\xE9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01a\x08nV[P`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R` \x96\x90\x96R`@\x90\x95 \x80T`\xFF\x19\x16`\x01\x17\x90UP\x92\x93\x92PPPV[_\x82\x90\x03a0\xF9W`@Qb\x1A25`\xE6\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x82\x11\x15a1%W`@Qc\x02\xD4\xE4\xEF`\xE3\x1B\x81R`\x04\x81\x01\x83\x90R`\xFF`$\x82\x01R`D\x01a\x08nV[`@\x80Q\x80\x82\x01\x90\x91R`\x10\x81Ro8:\xB164\xB1\xA22\xB1\xB9<\xB8:4\xB7\xB7`\x81\x1B` \x82\x01Ra1X\x90\x825\x84a<\x8BV[a1\x90`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m:\xB9\xB2\xB9\"2\xB1\xB9<\xB8:4\xB7\xB7`\x91\x1B\x81RP\x82` \x015\x85\x85\x90Pa<\x8BV[`@\x80Q\x80\x82\x01\x82R`\x06\x81Re5\xB6\xB9\xA3\xB2\xB7`\xD1\x1B` \x82\x01Ra1\xB9\x91\x83\x015\x84a<\x8BV[`@\x80Q\x80\x82\x01\x90\x91R`\x03\x81Rbmpc`\xE8\x1B` \x82\x01Ra1\xE2\x90``\x83\x015\x84a<\x8BV[_a1\xEBa(gV[\x80T\x90\x91P\x85\x11a2\x1CW\x80T`@Qc\xEF\xD5_g`\xE0\x1B\x81Ra\x08n\x91\x87\x91`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[\x84\x81U_[\x83\x81\x10\x15a5\xCAW6\x85\x85\x83\x81\x81\x10a2<Wa2<aJvV[\x90P` \x02\x81\x01\x90a2N\x91\x90aNyV[`@\x80Q`\x80\x81\x01\x90\x91R\x90\x91P_\x90\x80a2l` \x85\x01\x85a?\x9BV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x01` \x81\x01\x90a2\x8D\x91\x90a?\x9BV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a2\xA8`@\x85\x01\x85aN\x97V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x90\x82RP` \x01a2\xEE``\x85\x01\x85aN\x97V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x91RP\x80Q\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16a3NW`@QcB3@%`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16a3yW`@Qc-\xEC\xCFM`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x88\x81R`\x02\x85\x01` \x90\x81R`@\x80\x83 \x84Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15a3\xCDW\x80Q`@Qc\r\x18\xC4\xFF`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01a\x08nV[_\x88\x81R`\x03\x85\x01` \x90\x81R`@\x80\x83 \x84\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15a4&W` \x81\x01Q`@Qc\xF5\x1A\xF6\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01a\x08nV[_\x88\x81R`\x01\x85\x81\x01` \x90\x81R`@\x80\x84 \x80T\x80\x85\x01\x82U\x90\x85R\x93\x82\x90 \x85Q`\x04\x90\x95\x02\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x17\x82U\x92\x86\x01Q\x93\x81\x01\x80T\x90\x93\x16\x93\x90\x94\x16\x92\x90\x92\x17\x90U\x82\x01Q\x82\x91\x90`\x02\x82\x01\x90a4\x96\x90\x82aO$V[P``\x82\x01Q`\x03\x82\x01\x90a4\xAB\x90\x82aO$V[PPP_\x88\x81R`\x02\x80\x86\x01` \x90\x81R`@\x80\x84 \x85Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x86R\x90\x83R\x81\x85 \x80T`\xFF\x19\x90\x81\x16`\x01\x90\x81\x17\x90\x92U\x8E\x87R`\x03\x8B\x01\x85R\x83\x87 \x88\x86\x01\x80Q\x85\x16\x89R\x90\x86R\x84\x88 \x80T\x90\x92\x16\x83\x17\x90\x91U\x8E\x87R`\x04\x8B\x01\x85R\x83\x87 \x88Q\x84\x16\x88R\x90\x94R\x94\x82\x90 \x86Q\x81T\x90\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x82U\x93Q\x95\x81\x01\x80T\x96\x90\x92\x16\x95\x90\x93\x16\x94\x90\x94\x17\x90\x93U\x91\x83\x01Q\x83\x92\x91\x82\x01\x90a5d\x90\x82aO$V[P``\x82\x01Q`\x03\x82\x01\x90a5y\x90\x82aO$V[PPP_\x88\x81R`\x05\x85\x01` \x90\x81R`@\x82 \x92\x81\x01Q\x83T`\x01\x80\x82\x01\x86U\x94\x84R\x91\x90\x92 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x91\x90\x91\x01\x90Pa2!V[P_\x94\x85R`\x06\x81\x01` \x90\x81R`@\x80\x87 \x845\x90U`\x07\x83\x01\x82R\x80\x87 \x82\x85\x015\x90U`\x08\x83\x01\x82R\x80\x87 \x81\x85\x015\x90U`\t\x90\x92\x01\x90R\x90\x93 ``\x90\x93\x015\x90\x92UPPV[_`\x03a6!a(gV[_\x84\x81R`\x0E\x91\x90\x91\x01` R`@\x90 T`\xFF\x16`\x03\x81\x11\x15a6GWa6GaF|V[\x14\x92\x91PPV[_a6Wa(gV[_\x92\x83R`\x0F\x81\x01` \x90\x81R`@\x80\x85 \x80T`\xFF\x19\x16\x90U`\x10\x90\x92\x01\x90R\x82 \x91\x90\x91UPV[_\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a6\xBFWr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a6\xEBWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a7\tWf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a7!Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a75Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a7GW`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\rzW`\x01\x01\x92\x91PPV[a7a\x82a<\xD4V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a7\xA5Wa+\xB8\x82\x82a=7V[a\x0Eca=\xA9V[_\x80_\x83Q`A\x03a7\xE4W` \x84\x01Q`@\x85\x01Q``\x86\x01Q_\x1Aa7\xD6\x88\x82\x85\x85a=\xC8V[\x95P\x95P\x95PPPPa7\xEFV[PP\x81Q_\x91P`\x02\x90[\x92P\x92P\x92V[_\x82`\x03\x81\x11\x15a8\tWa8\taF|V[\x03a8\x12WPPV[`\x01\x82`\x03\x81\x11\x15a8&Wa8&aF|V[\x03a8DW`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a8XWa8XaF|V[\x03a8yW`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x08nV[`\x03\x82`\x03\x81\x11\x15a8\x8DWa8\x8DaF|V[\x03a\x0EcW`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x08nV[_\x80\x84`\x01`\x01`@\x1B\x03\x81\x11\x15a8\xC8Wa8\xC8aA\x91V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a8\xF1W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x85\x81\x10\x15a;8W6_\x88\x88\x84\x81\x81\x10a9\x12Wa9\x12aJvV[\x90P` \x02\x81\x01\x90a9$\x91\x90aO\xDFV[a92\x90`@\x81\x01\x90aO\xF3V[\x90\x92P\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a9PWa9PaA\x91V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a9yW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x82\x81\x10\x15a:kW\x7F\xDD\xD1\x08w.j8\x99\xFE\xB0M\x14\x8A\xE9\x15\xCB\xE3\xEB^\xBD &\x88\x08\x03\x99\xE9\x92\x1A\xC3ak\x84\x84\x83\x81\x81\x10a9\xB9Wa9\xB9aJvV[\x90P` \x02\x81\x01\x90a9\xCB\x91\x90aP8V[a9\xD9\x90` \x81\x01\x90aPLV[\x85\x85\x84\x81\x81\x10a9\xEBWa9\xEBaJvV[\x90P` \x02\x81\x01\x90a9\xFD\x91\x90aP8V[a:\x0B\x90` \x81\x01\x90aN\x97V[`@Qa:\x19\x92\x91\x90aI\xE6V[`@Q\x90\x81\x90\x03\x81 a:0\x93\x92\x91` \x01aPeV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x82\x81Q\x81\x10a:XWa:XaJvV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a9~V[P\x89\x89\x85\x81\x81\x10a:~Wa:~aJvV[\x90P` \x02\x81\x01\x90a:\x90\x91\x90aO\xDFV[5\x8A\x8A\x86\x81\x81\x10a:\xA3Wa:\xA3aJvV[\x90P` \x02\x81\x01\x90a:\xB5\x91\x90aO\xDFV[` \x015\x82`@Q` \x01a:\xCA\x91\x90aP\x87V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01\x94\x90\x94R\x81\x01\x91\x90\x91R``\x81\x01\x91\x90\x91R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85\x85\x81Q\x81\x10a;\"Wa;\"aJvV[` \x90\x81\x02\x91\x90\x91\x01\x01RPPP`\x01\x01a8\xF6V[P_\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a;RWa;RaA\x91V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a;{W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84\x81\x10\x15a<UW\x85\x85\x82\x81\x81\x10a;\x9AWa;\x9AaJvV[\x90P` \x02\x81\x01\x90a;\xAC\x91\x90aO\xDFV[5\x86\x86\x83\x81\x81\x10a;\xBFWa;\xBFaJvV[\x90P` \x02\x81\x01\x90a;\xD1\x91\x90aO\xDFV[` \x015\x87\x87\x84\x81\x81\x10a;\xE7Wa;\xE7aJvV[\x90P` \x02\x81\x01\x90a;\xF9\x91\x90aO\xDFV[a<\x07\x90`@\x81\x01\x90aN\x97V[`@Q` \x01a<\x1A\x94\x93\x92\x91\x90aP\xBCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x82\x81Q\x81\x10a<BWa<BaJvV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a;\x80V[P\x81\x81`@Q` \x01a<i\x92\x91\x90aQ\x1FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x92PPP\x94\x93PPPPV[\x81_\x03a<\xADW\x82`@Qc\x1B_\xDB\x07`\xE1\x1B\x81R`\x04\x01a\x08n\x91\x90a?eV[\x80\x82\x11\x15a+\xB8W\x82\x82\x82`@Qc\xCA\xA8\x14\xA3`\xE0\x1B\x81R`\x04\x01a\x08n\x93\x92\x91\x90aQCV[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a=\tW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x08nV[_\x80Q` aQy\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``_\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa=S\x91\x90aQgV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a=\x8BW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a=\x90V[``\x91P[P\x91P\x91Pa=\xA0\x85\x83\x83a>\x90V[\x95\x94PPPPPV[4\x15a*SW`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a>\x01WP_\x91P`\x03\x90P\x82a>\x86V[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a>RW=_\x80>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a>}WP_\x92P`\x01\x91P\x82\x90Pa>\x86V[\x92P_\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[``\x82a>\xA5Wa>\xA0\x82a>\xEFV[a>\xE8V[\x81Q\x15\x80\x15a>\xBCWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a>\xE5W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x08nV[P\x80[\x93\x92PPPV[\x80Q\x15a>\xFFW\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x83\x81\x10\x15a?2W\x81\x81\x01Q\x83\x82\x01R` \x01a?\x1AV[PP_\x91\x01RV[_\x81Q\x80\x84Ra?Q\x81` \x86\x01` \x86\x01a?\x18V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R_a>\xE8` \x83\x01\x84a?:V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a)VW_\x80\xFD[\x805a?\x96\x81a?wV[\x91\x90PV[_` \x82\x84\x03\x12\x15a?\xABW_\x80\xFD[\x815a>\xE8\x81a?wV[_\x80\x83`\x1F\x84\x01\x12a?\xC6W_\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a?\xDCW_\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a?\xF6W_\x80\xFD[\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12a@\rW_\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a@#W_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a?\xF6W_\x80\xFD[_\x80_\x80_\x80_\x87\x89\x03`\xE0\x81\x12\x15a@QW_\x80\xFD[\x885`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a@gW_\x80\xFD[a@s\x8C\x83\x8D\x01a?\xB6V[\x90\x9AP\x98P\x88\x91P`\x80`\x1F\x19\x84\x01\x12\x15a@\x8CW_\x80\xFD[` \x8B\x01\x97P`\xA0\x8B\x015\x92P\x80\x83\x11\x15a@\xA5W_\x80\xFD[a@\xB1\x8C\x84\x8D\x01a?\xFDV[\x90\x97P\x95P`\xC0\x8B\x015\x92P\x86\x91P\x80\x83\x11\x15a@\xCCW_\x80\xFD[PPa@\xDA\x8A\x82\x8B\x01a?\xB6V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[_` \x82\x84\x03\x12\x15a@\xFDW_\x80\xFD[P5\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15aA\x15W_\x80\xFD[\x825\x91P` \x83\x015aA'\x81a?wV[\x80\x91PP\x92P\x92\x90PV[_`\x01\x80`\xA0\x1B\x03\x80\x83Q\x16\x84R\x80` \x84\x01Q\x16` \x85\x01RP`@\x82\x01Q`\x80`@\x85\x01RaAf`\x80\x85\x01\x82a?:V[\x90P``\x83\x01Q\x84\x82\x03``\x86\x01Ra=\xA0\x82\x82a?:V[` \x81R_a>\xE8` \x83\x01\x84aA2V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x80`@\x83\x85\x03\x12\x15aA\xB6W_\x80\xFD[\x825aA\xC1\x81a?wV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aA\xDCW_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aA\xEFW_\x80\xFD[\x815\x81\x81\x11\x15aB\x01WaB\x01aA\x91V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15aB)WaB)aA\x91V[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15aBAW_\x80\xFD[\x82` \x86\x01` \x83\x017_` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aB\xA2W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aB}V[P\x90\x96\x95PPPPPPV[_\x80_\x80_``\x86\x88\x03\x12\x15aB\xC2W_\x80\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aB\xDFW_\x80\xFD[aB\xEB\x89\x83\x8A\x01a?\xFDV[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15aC\x03W_\x80\xFD[PaC\x10\x88\x82\x89\x01a?\xFDV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_\x80_\x80_\x80_\x80_`\xA0\x8A\x8C\x03\x12\x15aC9W_\x80\xFD[\x895\x98P` \x8A\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aCVW_\x80\xFD[aCb\x8D\x83\x8E\x01a?\xB6V[\x90\x9AP\x98P`@\x8C\x015\x91P\x80\x82\x11\x15aCzW_\x80\xFD[aC\x86\x8D\x83\x8E\x01a?\xB6V[\x90\x98P\x96P``\x8C\x015\x91P\x80\x82\x11\x15aC\x9EW_\x80\xFD[aC\xAA\x8D\x83\x8E\x01a?\xFDV[\x90\x96P\x94P`\x80\x8C\x015\x91P\x80\x82\x11\x15aC\xC2W_\x80\xFD[PaC\xCF\x8C\x82\x8D\x01a?\xFDV[\x91P\x80\x93PP\x80\x91PP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[_\x80_\x80_\x80_`\x80\x88\x8A\x03\x12\x15aC\xFCW_\x80\xFD[\x875\x96P` \x88\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aD\x19W_\x80\xFD[aD%\x8B\x83\x8C\x01a?\xB6V[\x90\x98P\x96P`@\x8A\x015\x91P\x80\x82\x11\x15aD=W_\x80\xFD[aDI\x8B\x83\x8C\x01a?\xFDV[\x90\x96P\x94P``\x8A\x015\x91P\x80\x82\x11\x15aDaW_\x80\xFD[Pa@\xDA\x8A\x82\x8B\x01a?\xFDV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a?\x96W_\x80\xFD[_\x80_\x80_\x80`\x80\x87\x89\x03\x12\x15aD\x99W_\x80\xFD[\x865\x95P` \x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aD\xB6W_\x80\xFD[aD\xC2\x8A\x83\x8B\x01a?\xFDV[\x90\x97P\x95P`@\x89\x015\x91P\x80\x82\x11\x15aD\xDAW_\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12aD\xEDW_\x80\xFD[\x815\x81\x81\x11\x15aD\xFBW_\x80\xFD[\x8A` ``\x83\x02\x85\x01\x01\x11\x15aE\x0FW_\x80\xFD[` \x83\x01\x95P\x80\x94PPPPaE'``\x88\x01aDnV[\x90P\x92\x95P\x92\x95P\x92\x95V[_\x80`@\x83\x85\x03\x12\x15aEDW_\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[_\x82\x82Q\x80\x85R` \x80\x86\x01\x95P` \x82`\x05\x1B\x84\x01\x01` \x86\x01_[\x84\x81\x10\x15aE\x9EW`\x1F\x19\x86\x84\x03\x01\x89RaE\x8C\x83\x83QaA2V[\x98\x84\x01\x98\x92P\x90\x83\x01\x90`\x01\x01aEpV[P\x90\x97\x96PPPPPPPV[` \x81R_a>\xE8` \x83\x01\x84aESV[_\x85QaE\xCE\x81\x84` \x8A\x01a?\x18V[a\x10;`\xF1\x1B\x90\x83\x01\x90\x81R\x85QaE\xED\x81`\x02\x84\x01` \x8A\x01a?\x18V[\x80\x82\x01\x91PP`\x17`\xF9\x1B\x80`\x02\x83\x01R\x85QaF\x11\x81`\x03\x85\x01` \x8A\x01a?\x18V[`\x03\x92\x01\x91\x82\x01R\x83QaF,\x81`\x04\x84\x01` \x88\x01a?\x18V[\x01`\x04\x01\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15aFJW_\x80\xFD[\x81Qa>\xE8\x81a?wV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\rzWa\rzaFUV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\rzWa\rzaFUV[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aF\xB8W_\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aF\xD6W_\x80\xFD[\x806\x03\x82\x13\x15a?\xF6W_\x80\xFD[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x805`\x03\x81\x90\x0B\x81\x14a?\x96W_\x80\xFD[_\x825`^\x19\x836\x03\x01\x81\x12aG1W_\x80\xFD[\x90\x91\x01\x92\x91PPV[_\x83\x83\x85R` \x80\x86\x01\x95P\x80\x85`\x05\x1B\x83\x01\x01\x84_[\x87\x81\x10\x15aE\x9EW\x84\x83\x03`\x1F\x19\x01\x89RaGl\x82\x88aG\x1DV[``aGx\x82\x83aF\xA3V[\x82\x87RaG\x88\x83\x88\x01\x82\x84aF\xE4V[\x92PPPaG\x98\x86\x83\x01\x83aF\xA3V[\x86\x83\x03\x88\x88\x01RaG\xAA\x83\x82\x84aF\xE4V[\x92PPP`@aG\xBC\x81\x84\x01\x84aF\xA3V[\x93P\x86\x83\x03\x82\x88\x01RaG\xD0\x83\x85\x83aF\xE4V[\x9C\x88\x01\x9C\x96PPP\x92\x85\x01\x92PP`\x01\x01aGQV[`\xE0\x80\x82R\x81\x01\x87\x90R_a\x01\0\x80\x83\x01`\x05\x8A\x90\x1B\x84\x01\x82\x01\x8B\x84[\x8C\x81\x10\x15aIFW\x86\x83\x03`\xFF\x19\x01\x84R\x8156\x8F\x90\x03`\xFE\x19\x01\x81\x12aH(W_\x80\xFD[\x8E\x01aHD\x84aH7\x83a?\x8BV[`\x01`\x01`\xA0\x1B\x03\x16\x90RV[` aHQ\x81\x83\x01a?\x8BV[`\x01`\x01`\xA0\x1B\x03\x16\x81\x86\x01R`@aHl\x83\x82\x01\x84aF\xA3V[\x89\x83\x89\x01RaH~\x8A\x89\x01\x82\x84aF\xE4V[\x92PPP``aH\x90\x81\x85\x01\x85aF\xA3V[\x88\x84\x03\x83\x8A\x01RaH\xA2\x84\x82\x84aF\xE4V[\x93PPPP`\x80aH\xB4\x81\x85\x01aG\x0CV[aH\xC2\x82\x89\x01\x82`\x03\x0B\x90RV[PP`\xA0aH\xD2\x81\x85\x01\x85aF\xA3V[\x88\x84\x03\x83\x8A\x01RaH\xE4\x84\x82\x84aF\xE4V[\x93PPPP`\xC0aH\xF7\x81\x85\x01\x85aF\xA3V[\x88\x84\x03\x83\x8A\x01RaI\t\x84\x82\x84aF\xE4V[\x93PPPPaI\x1B`\xE0\x84\x01\x84aF\xA3V[\x93P\x86\x82\x03`\xE0\x88\x01RaI0\x82\x85\x83aF\xE4V[\x97\x83\x01\x97\x96PPP\x92\x90\x92\x01\x91P`\x01\x01aH\x03V[PPaIv` \x86\x01\x8B\x805\x82R` \x81\x015` \x83\x01R`@\x81\x015`@\x83\x01R``\x81\x015``\x83\x01RPPV[\x84\x81\x03`\xA0\x86\x01RaI\x89\x81\x89\x8BaF\xE4V[\x92PPP\x82\x81\x03`\xC0\x84\x01RaI\xA0\x81\x85\x87aG:V[\x9A\x99PPPPPPPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80aI\xC2W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aI\xE0WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[_`\x01\x82\x01aJ\x06WaJ\x06aFUV[P`\x01\x01\x90V[`@\x81R_aJ `@\x83\x01\x86\x88aF\xE4V[\x82\x81\x03` \x84\x01RaJ3\x81\x85\x87aF\xE4V[\x97\x96PPPPPPPV[\x85\x81R``` \x82\x01R_aJW``\x83\x01\x86\x88aF\xE4V[\x82\x81\x03`@\x84\x01RaJj\x81\x85\x87aF\xE4V[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x805`\x04\x81\x10a?\x96W_\x80\xFD[`\x04\x81\x10aJ\xB4WcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90RV[_\x83\x83\x85R` \x80\x86\x01\x95P\x80\x85`\x05\x1B\x83\x01\x01\x84_[\x87\x81\x10\x15aE\x9EW\x84\x83\x03`\x1F\x19\x01\x89RaJ\xEA\x82\x88aG\x1DV[``\x815\x85R\x85\x82\x015\x86\x86\x01R`@aK\x06\x81\x84\x01\x84aF\xA3V[\x93P\x82\x82\x88\x01RaK\x1A\x83\x88\x01\x85\x83aF\xE4V[\x9C\x88\x01\x9C\x96PPP\x92\x85\x01\x92PP`\x01\x01aJ\xCFV[_\x82\x82Q\x80\x85R` \x80\x86\x01\x95P` \x82`\x05\x1B\x84\x01\x01` \x86\x01_[\x84\x81\x10\x15aE\x9EW`\x1F\x19\x86\x84\x03\x01\x89RaKi\x83\x83Qa?:V[\x98\x84\x01\x98\x92P\x90\x83\x01\x90`\x01\x01aKMV[``\x80\x82R\x81\x81\x01\x86\x90R_\x90`\x80\x80\x84\x01`\x05\x89\x90\x1B\x85\x01\x82\x01\x8A\x85[\x8B\x81\x10\x15aL\xB1W`\x7F\x19\x80\x89\x85\x03\x01\x85RaK\xB5\x83\x8FaG\x1DV[\x87\x85\x01\x815\x86R` \x80\x83\x015\x81\x88\x01R`@\x80\x84\x015`\x1E\x19\x856\x03\x01\x81\x12aK\xDDW_\x80\xFD[\x90\x93\x01\x92\x835\x82\x85\x01`\x01`\x01`@\x1B\x03\x82\x11\x15aK\xF9W_\x80\xFD[\x81`\x05\x1B\x806\x03\x82\x13\x15aL\x0BW_\x80\xFD[\x8A\x84\x01\x8E\x90R\x94\x82\x90R\x93\x89\x01\x8B\x01\x93_\x8C\x8B\x016\x88\x90\x03`^\x19\x01[\x84\x83\x10\x15aL\x95W\x89\x8D\x89\x03\x01\x82R\x835\x81\x81\x12aLDW_\x80\xFD[\x89\x01\x87\x81\x01aL[\x8AaLV\x83aJ\x8AV[aJ\x98V[aLg\x88\x83\x01\x82aF\xA3V[\x92P\x90P\x87\x89\x8B\x01RaL}\x88\x8B\x01\x83\x83aF\xE4V[\x99PPP\x92\x86\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x86\x01\x90aL(V[PPP\x99\x83\x01\x99P\x92\x97PP\x94\x90\x94\x01\x93PPP`\x01\x01aK\x99V[PP\x85\x81\x03` \x87\x01RaL\xC6\x81\x89\x8BaJ\xB8V[\x93PPPP\x82\x81\x03`@\x84\x01RaJj\x81\x85aK0V[_`\x01`\x01`\xFB\x1B\x03\x83\x11\x15aL\xF1W_\x80\xFD[\x82`\x05\x1B\x80\x85\x847\x91\x90\x91\x01\x93\x92PPPV[``\x80\x82R\x81\x01\x86\x90R_`\x01`\x01`\xFB\x1B\x03\x87\x11\x15aM\"W_\x80\xFD[\x86`\x05\x1B\x80\x89`\x80\x85\x017\x82\x01\x82\x81\x03`\x80\x90\x81\x01` \x85\x01RaMI\x90\x82\x01\x87\x89aF\xE4V[\x90P\x82\x81\x03`@\x84\x01RaM^\x81\x85\x87aF\xE4V[\x99\x98PPPPPPPPPV[`\xA0\x81R_aM}`\xA0\x83\x01\x85aESV[\x90Pa>\xE8` \x83\x01\x84\x805\x82R` \x81\x015` \x83\x01R`@\x81\x015`@\x83\x01R``\x81\x015``\x83\x01RPPV[_` \x82\x84\x03\x12\x15aM\xBDW_\x80\xFD[a>\xE8\x82aDnV[_``\x80\x83RaM\xDA``\x84\x01\x88\x8AaF\xE4V[\x83\x81\x03` \x85\x81\x01\x91\x90\x91R\x86\x82R\x87\x91\x81\x01_[\x88\x81\x10\x15aNAW`\x01`\x01`@\x1B\x03\x80aN\t\x86aDnV[\x16\x83R\x80aN\x18\x85\x87\x01aDnV[\x16\x84\x84\x01R`@\x81aN+\x82\x88\x01aDnV[\x16\x90\x84\x01RP\x92\x84\x01\x92\x90\x84\x01\x90`\x01\x01aM\xEFV[P\x80\x94PPPPP`\x01`\x01`@\x1B\x03\x83\x16`@\x83\x01R\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15aNrW_\x80\xFD[PQ\x91\x90PV[_\x825`\xFE\x19\x836\x03\x01\x81\x12aN\x8DW_\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aN\xACW_\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aN\xC5W_\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a?\xF6W_\x80\xFD[`\x1F\x82\x11\x15a+\xB8W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15aN\xFEWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15aO\x1DW_\x81U`\x01\x01aO\nV[PPPPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aO=WaO=aA\x91V[aOQ\x81aOK\x84TaI\xAEV[\x84aN\xD9V[` \x80`\x1F\x83\x11`\x01\x81\x14aO\x84W_\x84\x15aOmWP\x85\x83\x01Q[_\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua)%V[_\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15aO\xB2W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01aO\x93V[P\x85\x82\x10\x15aO\xCFW\x87\x85\x01Q_\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[_\x825`^\x19\x836\x03\x01\x81\x12aN\x8DW_\x80\xFD[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aP\x08W_\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aP!W_\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a?\xF6W_\x80\xFD[_\x825`>\x19\x836\x03\x01\x81\x12aN\x8DW_\x80\xFD[_` \x82\x84\x03\x12\x15aP\\W_\x80\xFD[a>\xE8\x82aJ\x8AV[\x83\x81R``\x81\x01aPy` \x83\x01\x85aJ\x98V[\x82`@\x83\x01R\x94\x93PPPPV[\x81Q_\x90\x82\x90` \x80\x86\x01\x84[\x83\x81\x10\x15aP\xB0W\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01aP\x94V[P\x92\x96\x95PPPPPPV[\x84\x81R\x83` \x82\x01R```@\x82\x01R_aP\xDB``\x83\x01\x84\x86aF\xE4V[\x96\x95PPPPPPV[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15aQ\x14W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aP\xF8V[P\x94\x95\x94PPPPPV[`@\x81R_aQ1`@\x83\x01\x85aP\xE5V[\x82\x81\x03` \x84\x01Ra=\xA0\x81\x85aP\xE5V[``\x81R_aQU``\x83\x01\x86a?:V[` \x83\x01\x94\x90\x94RP`@\x01R\x91\x90PV[_\x82QaN\x8D\x81\x84` \x87\x01a?\x18V\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405260043610610212575f3560e01c8063756e86521161011e578063bf9b16c8116100a8578063c999a8b41161006d578063c999a8b4146105dc578063ccbf8199146105fb578063cceac0191461061a578063ee7d52d114610639578063f9c670c314610658575f80fd5b8063bf9b16c81461054c578063c0ae64f71461056b578063c2580a2d1461058a578063c2b42986146105a9578063c3aaaa5a146105bd575f80fd5b8063976f3eb9116100ee578063976f3eb9146104c15780639d1b1be1146104d5578063a642ba0f146104e9578063ad3cb1cc14610508578063b4722bc414610538575f80fd5b8063756e8652146104505780637eaac8f21461046f5780639447cfd414610483578063976c98b5146104a2575f80fd5b806341ad069c1161019f57806352d1902d1161016f57806352d1902d146103a95780635bff76d9146103bd57806365b394af146103e95780636ec0f23f1461041257806371d76dae14610431575f80fd5b806341ad069c1461033957806346c5bbbd1461035857806347e82295146103775780634f1ef28614610396575f80fd5b8063221cdd4e116101e5578063221cdd4e1461029957806326cf5def146102b8578063281e8bfe146102da5780632a388998146102f957806331ff41c81461030d575f80fd5b80630d8e6e2c14610216578063123abb28146102405780631ce3f9bc14610256578063203d01141461026a575b5f80fd5b348015610221575f80fd5b5061022a610684565b6040516102379190613f65565b60405180910390f35b34801561024b575f80fd5b506102546106f0565b005b348015610261575f80fd5b506102546107cb565b348015610275575f80fd5b50610289610284366004613f9b565b6108f5565b6040519015158152602001610237565b3480156102a4575f80fd5b506102546102b336600461403a565b610935565b3480156102c3575f80fd5b506102cc610b4b565b604051908152602001610237565b3480156102e5575f80fd5b506102cc6102f43660046140ed565b610b71565b348015610304575f80fd5b506102cc610b96565b348015610318575f80fd5b5061032c610327366004614104565b610bbc565b604051610237919061417f565b348015610344575f80fd5b506102cc6103533660046140ed565b610d80565b348015610363575f80fd5b50610289610372366004614104565b610dc5565b348015610382575f80fd5b506102cc6103913660046140ed565b610e23565b6102546103a43660046141a5565b610e48565b3480156103b4575f80fd5b506102cc610e67565b3480156103c8575f80fd5b506103dc6103d73660046140ed565b610e82565b6040516102379190614262565b3480156103f4575f80fd5b506103fd610f05565b60408051928352602083019190915201610237565b34801561041d575f80fd5b5061025461042c3660046142ae565b610f25565b34801561043c575f80fd5b5061025461044b366004614321565b611386565b34801561045b575f80fd5b5061025461046a3660046143e6565b611626565b34801561047a575f80fd5b506103dc611795565b34801561048e575f80fd5b5061028961049d366004614104565b611816565b3480156104ad575f80fd5b506102546104bc36600461403a565b611854565b3480156104cc575f80fd5b506102cc611c04565b3480156104e0575f80fd5b506102cc611c18565b3480156104f4575f80fd5b506102546105033660046142ae565b611c29565b348015610513575f80fd5b5061022a604051806040016040528060058152602001640352e302e360dc1b81525081565b348015610543575f80fd5b506102cc611d7c565b348015610557575f80fd5b506102896105663660046140ed565b611da2565b348015610576575f80fd5b506102546105853660046140ed565b611dac565b348015610595575f80fd5b506102546105a43660046140ed565b611f30565b3480156105b4575f80fd5b506102cc612125565b3480156105c8575f80fd5b506102cc6105d73660046140ed565b61214b565b3480156105e7575f80fd5b506103fd6105f63660046140ed565b612170565b348015610606575f80fd5b50610254610615366004614484565b6121d7565b348015610625575f80fd5b50610289610634366004614533565b6124e3565b348015610644575f80fd5b506102896106533660046140ed565b612548565b348015610663575f80fd5b506106776106723660046140ed565b612552565b60405161023791906145ab565b60606040518060400160405280600e81526020016d50726f746f636f6c436f6e66696760901b8152506106b65f612715565b6106c06004612715565b6106c95f612715565b6040516020016106dc94939291906145bd565b604051602081830303815290604052905090565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00805460059190600160401b900460ff1680610739575080546001600160401b03808416911610155b156107575760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b03831617600160401b1781556107806127a4565b805460ff60401b191681556040516001600160401b03831681527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a15050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561081b573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061083f919061463a565b6001600160a01b0316336001600160a01b0316146108775760405163021bfda160e41b81523360048201526024015b60405180910390fd5b61087f6127a4565b5f610888612867565b600b8101549091505f61089a8261288b565b905080827f15aaaf475ef407543f5164f57dcf57f7f93816f55bae77ca09efc445ba40eef78486600d01546001436108d29190614669565b6040805193845260208401929092529082015260600160405180910390a3505050565b5f806108ff612867565b600b8101545f9081526003909101602090815260408083206001600160a01b039096168352949052929092205460ff1692915050565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00546001600160401b03166001600160401b031660011461098957604051636f4f731f60e01b815260040160405180910390fd5b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00805460059190600160401b900460ff16806109d2575080546001600160401b03808416911610155b156109f05760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b03831617600160401b1781555f610a1a612867565b90505f610a2c600760f81b6001614690565b9050610a4981610a41600160fb1b6001614690565b8d8d8d6128dc565b60405180604001604052804381526020018c8c8c8c8c8c8c604051602001610a7797969594939291906147e6565b60408051601f1981840301815291815281516020928301209092525f848152601786018252919091208251815591015160019091015560f86007901b817f204d6b80121154cd87d99cf54c639a3dd0a53b3084277098de972ebdd34c6be98d8d8d8d8d8d8d604051610aef97969594939291906147e6565b60405180910390a35050805460ff60401b191681556040516001600160401b03831681527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a1505050505050505050565b5f80610b55612867565b600b8101545f9081526009909101602052604090205492915050565b5f610b7b8261292d565b610b83612867565b5f92835260070160205250604090205490565b5f80610ba0612867565b600b8101545f9081526006909101602052604090205492915050565b604080516080810182525f8082526020820152606091810182905281810191909152610be783612959565b610c07576040516377ddbe8160e01b81526004810184905260240161086e565b610c0f612867565b5f848152600491909101602090815260408083206001600160a01b03808716855290835292819020815160808101835281548516815260018201549094169284019290925260028201805491840191610c67906149ae565b80601f0160208091040260200160405190810160405280929190818152602001828054610c93906149ae565b8015610cde5780601f10610cb557610100808354040283529160200191610cde565b820191905f5260205f20905b815481529060010190602001808311610cc157829003601f168201915b50505050508152602001600382018054610cf7906149ae565b80601f0160208091040260200160405190810160405280929190818152602001828054610d23906149ae565b8015610d6e5780601f10610d4557610100808354040283529160200191610d6e565b820191905f5260205f20905b815481529060010190602001808311610d5157829003601f168201915b50505050508152505090505b92915050565b5f610d8a82612977565b610daa576040516377ddbe8160e01b81526004810183905260240161086e565b610db2612867565b5f92835260080160205250604090205490565b5f610dcf83612977565b610def576040516377ddbe8160e01b81526004810184905260240161086e565b610df7612867565b5f938452600201602090815260408085206001600160a01b039490941685529290525090205460ff1690565b5f610e2d8261292d565b610e35612867565b5f92835260090160205250604090205490565b610e506129af565b610e5982612a55565b610e638282612afc565b5050565b5f610e70612bbd565b505f8051602061517983398151915290565b6060610e8d8261292d565b610e95612867565b6005015f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020018280548015610ef957602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311610edb575b50505050509050919050565b5f805f610f10612867565b905080600b0154925080600d01549150509091565b5f610f2e612867565b90505f7f98bc32bb4045abd79f66305d47272679087ff12d8a03152ce14a0954b7709c00905060015f888152600e8401602052604090205460ff166003811115610f7a57610f7a61467c565b14610f9b57604051633586efa160e01b81526004810188905260240161086e565b600b8201545f88815260208390526040808220549051611034917f8a7d3190a514cdb5d57c841cb7382e85ad21b892b7fdfd3b87ab000925d9d5019185918d91610fe8908c908c906149e6565b6040805191829003822060208301969096528101939093526060830191909152608082015260a081019190915260c0015b60405160208183030381529060405280519060200120612c06565b90505f611076828a8a8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250612d0a92505050565b90505f856003015f8581526020019081526020015f205f836001600160a01b03166001600160a01b031681526020019081526020015f205f9054906101000a900460ff1690505f866003015f8d81526020019081526020015f205f846001600160a01b03166001600160a01b031681526020019081526020015f205f9054906101000a900460ff1690508115801561110c575080155b1561113c57604051631703bf1d60e31b81526001600160a01b0384166004820152602481018d905260440161086e565b5f8c815260018701602090815260408083206001600160a01b038716845290915290205460ff161561119357604051630c4b0b9960e31b81526001600160a01b0384166004820152602481018d905260440161086e565b5f8c81526001878101602090815260408084206001600160a01b0388168552909152909120805460ff1916909117905581156111f6575f8c81526002870160209081526040808320878452909152812080549091906111f1906149f5565b909155505b8015611229575f8c8152600387016020908152604080832087845290915281208054909190611224906149f5565b909155505b826001600160a01b03168c7f4254d948402ba059c02f9feecfca09b40c0b39026a344a159e3a7679807a83d28d8d8d8d6040516112699493929190614a0d565b60405180910390a35f85815260098801602090815260408083205460018b019092528220546112989190614669565b5f8e815260018a01602090815260408083205460038c0183528184208a8552909252909120549192501480156112f757505f81116112d75760016112d9565b805b5f8e8152600289016020908152604080832089845290915290205410155b15611377575f8d8152600e890160205260408120805460ff1916600217905561131f8e61288b565b9050808e7f15aaaf475ef407543f5164f57dcf57f7f93816f55bae77ca09efc445ba40eef7898c600d01546001436113579190614669565b6040805193845260208401929092529082015260600160405180910390a3505b50505050505050505050505050565b5f61138f612867565b905060015f8b8152600f8301602052604090205460ff1660028111156113b7576113b761467c565b146113d857604051637995bbcf60e01b8152600481018b905260240161086e565b5f8a81526010820160205260408120549080806113fd848f8f8f8f8f8f8f8f8f612d32565b9250925092505f6114108f868685612e9f565b9050836001600160a01b03168f7f7faef745af79a3af23d4199ce0366975cc2dd7149384d631337d133b226b5be4858d8d8d8d604051611454959493929190614a3e565b60405180910390a38061146c5750505050505061161b565b5050505f828152600e840160205260409020805460ff1916600317905550600b820181905561149b8b82612f75565b5f818152600183016020526040812080549091906001600160401b038111156114c6576114c6614191565b6040519080825280602002602001820160405280156114f957816020015b60608152602001906001900390816114e45790505b5090505f5b82548110156115d45782818154811061151957611519614a76565b905f5260205f2090600402016003018054611533906149ae565b80601f016020809104026020016040519081016040528092919081815260200182805461155f906149ae565b80156115aa5780601f10611581576101008083540402835291602001916115aa565b820191905f5260205f20905b81548152906001019060200180831161158d57829003601f168201915b50505050508282815181106115c1576115c1614a76565b60209081029190910101526001016114fe565b508c837f6c6eb35881f805f284be59b840b4ea990f96b13dd9e7192c4581e1bc81c0af988e8e8e8e8760405161160e959493929190614b7b565b60405180910390a3505050505b505050505050505050565b61162e612867565b5f888152600a91909101602052604090205460ff1661166357604051632b73d9b760e01b81526004810188905260240161086e565b5f8781527f98bc32bb4045abd79f66305d47272679087ff12d8a03152ce14a0954b7709c0460209081526040808320905161173d928b917fc687b63525ef20d95442d56bd140c9da6c10c6e23f482486c246c05236ceef9c9183916116cc918e918e9101614cdd565b6040516020818303038152906040528051906020012088886040516116f29291906149e6565b6040519081900381206117209493929160200193845260208401929092526040830152606082015260800190565b604051602081830303815290604052805190602001208888612faf565b9050806001600160a01b0316887fea1640f34deada2f815def0e112e6cd249533849a0fd53e9293ba34d7c4688cb89898989898960405161178396959493929190614d04565b60405180910390a35050505050505050565b60605f6117a0612867565b9050806005015f82600b015481526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561180b57602002820191905f5260205f20905b81546001600160a01b031681526001909101906020018083116117ed575b505050505091505090565b5f6118208361292d565b611828612867565b5f938452600301602090815260408085206001600160a01b039490941685529290525090205460ff1690565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156118a4573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118c8919061463a565b6001600160a01b0316336001600160a01b0316146118fb5760405163021bfda160e41b815233600482015260240161086e565b6119036127a4565b5f61190c612867565b600b8101548154919250905f90611924906001614690565b9050611932818b8b8b6130d9565b5f818152600e840160209081526040808320805460ff191660019081179091558601825280832080548251818502810185019093528083529192909190849084015b82821015611ae8575f848152602090819020604080516080810182526004860290920180546001600160a01b03908116845260018201541693830193909352600283018054929392918401916119c9906149ae565b80601f01602080910402602001604051908101604052809291908181526020018280546119f5906149ae565b8015611a405780601f10611a1757610100808354040283529160200191611a40565b820191905f5260205f20905b815481529060010190602001808311611a2357829003601f168201915b50505050508152602001600382018054611a59906149ae565b80601f0160208091040260200160405190810160405280929190818152602001828054611a85906149ae565b8015611ad05780601f10611aa757610100808354040283529160200191611ad0565b820191905f5260205f20905b815481529060010190602001808311611ab357829003601f168201915b50505050508152505081526020019060010190611974565b5050505090508089604051602001611b01929190614d6b565b60405160208183030381529060405280519060200120611b3e7f98bc32bb4045abd79f66305d47272679087ff12d8a03152ce14a0954b7709c0090565b5f84815260209182526040908190209290925581518083018352438152915182820191611b79918f918f918f918f918f918f918f91016147e6565b60408051601f1981840301815291815281516020928301209092525f8581526017880182528290208351815592015160019092019190915551839083907f204d6b80121154cd87d99cf54c639a3dd0a53b3084277098de972ebdd34c6be990611bef908f908f908f908f908f908f908f906147e6565b60405180910390a35050505050505050505050565b5f80611c0e612867565b600b015492915050565b5f80611c22612867565b5492915050565b5f8581527f98bc32bb4045abd79f66305d47272679087ff12d8a03152ce14a0954b7709c0660205260409020547f98bc32bb4045abd79f66305d47272679087ff12d8a03152ce14a0954b7709c009060ff16611c9a57604051623d8b4d60e41b81526004810187905260240161086e565b5f611d29826005015f8981526020019081526020015f20887f4af2c2967c6c1fa8706c2504bb0cad843f8d0fac79f31f2d0206e747f049fca28a8888604051611ce49291906149e6565b604051908190038120611d0c9392916020019283526020830191909152604082015260600190565b604051602081830303815290604052805190602001208989612faf565b9050806001600160a01b0316877fee1b8f29d516d563f41f2a3b75c200752feb512e6256eb561d17a4e61ee6335e88888888604051611d6b9493929190614a0d565b60405180910390a350505050505050565b5f80611d86612867565b600b8101545f9081526008909101602052604090205492915050565b5f610d7a82613616565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611dfc573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e20919061463a565b6001600160a01b0316336001600160a01b031614611e535760405163021bfda160e41b815233600482015260240161086e565b5f611e5c612867565b905080600b01548203611e855760405163030f5d5560e31b81526004810183905260240161086e565b611e8e82612977565b611eae576040516377ddbe8160e01b81526004810183905260240161086e565b5f828152600a820160209081526040808320805460ff19908116600117909155600e8501835281842080549091169055600c8401548084526010850190925290912054839003611f0157611f018161364e565b60405183907fda075d09198d207e3a918d4b8dfc87df2d60a00be703fd39eaac90962da0b7f0905f90a2505050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611f80573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611fa4919061463a565b6001600160a01b0316336001600160a01b031614611fd75760405163021bfda160e41b815233600482015260240161086e565b5f611fe0612867565b905080600d015482036120095760405163f0e781c160e01b81526004810183905260240161086e565b5f828152600f8201602052604081205460ff169060028260028111156120315761203161467c565b1490505f60018360028111156120495761204961467c565b148015612088575060035f8681526010860160209081526040808320548352600e880190915290205460ff1660038111156120865761208661467c565b145b905081158015612096575080155b156120b757604051637995bbcf60e01b81526004810186905260240161086e565b6120c08561364e565b5f8581527f98bc32bb4045abd79f66305d47272679087ff12d8a03152ce14a0954b7709c066020526040808220805460ff191660011790555186917f03436013075f8d1abb1781dbcaf418fc76fc797d1ab51c38664c1a51f3cd57f991a25050505050565b5f8061212f612867565b600b8101545f9081526007909101602052604090205492915050565b5f6121558261292d565b61215d612867565b5f92835260060160205250604090205490565b5f8061217b83612959565b61219b576040516377ddbe8160e01b81526004810184905260240161086e565b5f6121a4612867565b5f948552601701602090815260409485902085518087019096528054808752600190910154959091018590529492505050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612227573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061224b919061463a565b6001600160a01b0316336001600160a01b03161461227e5760405163021bfda160e41b815233600482015260240161086e565b855f0361229e57604051630992f7ad60e01b815260040160405180910390fd5b5f8490036122bf5760405163b548914760e01b815260040160405180910390fd5b5f8290036122e057604051632f94141160e21b815260040160405180910390fd5b806001600160401b03165f03612309576040516302fa7d2960e31b815260040160405180910390fd5b5f5b8281101561249a573684848381811061232657612326614a76565b60600291909101915061233e90506020820182614dad565b6001600160401b03165f0361236657604051633212217560e21b815260040160405180910390fd5b6123766060820160408301614dad565b6001600160401b031661238f6040830160208401614dad565b6001600160401b03161115612400576123ab6020820182614dad565b6123bb6040830160208401614dad565b6123cb6060840160408501614dad565b60405163790cee0760e11b81526001600160401b0393841660048201529183166024830152909116604482015260640161086e565b5f5b82811015612490576124176020830183614dad565b6001600160401b031686868381811061243257612432614a76565b6124489260206060909202019081019150614dad565b6001600160401b031603612488576124636020830183614dad565b6040516306c67e4760e41b81526001600160401b03909116600482015260240161086e565b600101612402565b505060010161230b565b50857fc42f1ecac8d4881ef9fd335ca98ee7254a436b92ba874a609e50a7d30c487b8d86868686866040516124d3959493929190614dc6565b60405180910390a2505050505050565b5f806124ed612867565b905060025f848152600f8301602052604090205460ff1660028111156125155761251561467c565b14801561253057505f83815260108201602052604090205484145b8015612540575061254084613616565b949350505050565b5f610d7a82612977565b606061255d8261292d565b612565612867565b6001015f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020015f905b8282101561270a575f848152602090819020604080516080810182526004860290920180546001600160a01b03908116845260018201541693830193909352600283018054929392918401916125eb906149ae565b80601f0160208091040260200160405190810160405280929190818152602001828054612617906149ae565b80156126625780601f1061263957610100808354040283529160200191612662565b820191905f5260205f20905b81548152906001019060200180831161264557829003601f168201915b5050505050815260200160038201805461267b906149ae565b80601f01602080910402602001604051908101604052809291908181526020018280546126a7906149ae565b80156126f25780601f106126c9576101008083540402835291602001916126f2565b820191905f5260205f20905b8154815290600101906020018083116126d557829003601f168201915b50505050508152505081526020019060010190612596565b505050509050919050565b60605f61272183613681565b60010190505f816001600160401b0381111561273f5761273f614191565b6040519080825280601f01601f191660200182016040528015612769576020820181803683370190505b5090508181016020015b5f19016f181899199a1a9b1b9c1cb0b131b232b360811b600a86061a8153600a850494508461277357509392505050565b5f6127ad612867565b80545f908152600e8201602052604081205491925060ff9091169060018260038111156127dc576127dc61467c565b14806127f9575060028260038111156127f7576127f761467c565b145b90505f6001600c8501545f908152600f8601602052604090205460ff1660028111156128275761282761467c565b14905081806128335750805b15612861578354600c8501546040516303c0105f60e11b81526004810192909252602482015260440161086e565b50505050565b7f80f3585af86806c5774303b06c1ee640aa83b6ef3e45df49bb26c8524500c20090565b5f80612895612867565b905080600c015f81546128a7906149f5565b91829055505f818152600f830160209081526040808320805460ff191660011790556010909401905291909120929092555090565b5f6128e5612867565b90506128f3868585856130d9565b5f868152600e820160205260409020805460ff19166003179055600b8101869055600c81018590556129258587612f75565b505050505050565b61293681613616565b612956576040516377ddbe8160e01b81526004810182905260240161086e565b50565b5f612962612867565b5f928352600101602052506040902054151590565b5f80612981612867565b5f848152600e91909101602052604090205460ff1660038111156129a7576129a761467c565b141592915050565b306001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161480612a3557507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316612a295f80516020615179833981519152546001600160a01b031690565b6001600160a01b031614155b15612a535760405163703e46dd60e11b815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612aa5573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612ac9919061463a565b6001600160a01b0316336001600160a01b0316146129565760405163021bfda160e41b815233600482015260240161086e565b816001600160a01b03166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa925050508015612b56575060408051601f3d908101601f19168201909252612b5391810190614e62565b60015b612b7e57604051634c9c8ce360e01b81526001600160a01b038316600482015260240161086e565b5f805160206151798339815191528114612bae57604051632a87526960e21b81526004810182905260240161086e565b612bb88383613758565b505050565b306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614612a535760405163703e46dd60e11b815260040160405180910390fd5b604080518082018252600e81526d50726f746f636f6c436f6e66696760901b6020918201528151808301835260018152603160f81b9082015281517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f818301527fa3de1880cf083e8318b77a7965d02dd9765e85a48e418a4463af7a0d57b4b3ee818401527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc660608201524660808201523060a0808301919091528351808303909101815260c08201845280519083012061190160f01b60e083015260e2820152610102808201949094528251808203909401845261012201909152815191012090565b5f805f80612d1886866137ad565b925092509250612d2882826137f6565b5090949350505050565b5f8080891580612d40575087155b15612d61576040516318855a7960e21b8152600481018d905260240161086e565b612d6d8b8b8b8b6138ae565b9150612dec7f6cf47d8f5bed73ad2a8c60adbdd5b555d665357db758662abd49ea977f80d19a8e612d9c612867565b600d01548f868a8a604051612db29291906149e6565b6040805191829003822060208301979097528101949094526060840192909252608083015260a082015260c081019190915260e001611019565b9050612e2d8188888080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250612d0a92505050565b9250612e37612867565b5f8e8152600391909101602090815260408083206001600160a01b038716845290915290205460ff16612e8f5760405163a3f4afeb60e01b81526001600160a01b0384166004820152602481018d905260440161086e565b9a509a509a975050505050505050565b5f80612ea9612867565b5f87815260128201602090815260408083206001600160a01b038916845290915290205490915060ff1615612f03576040516324a0bb1b60e11b81526001600160a01b03851660048201526024810187905260440161086e565b5f86815260128201602090815260408083206001600160a01b03881684528252808320805460ff19166001179055888352601384018252808320868452909152812080548290612f52906149f5565b91829055505f968752600590920160205250604090942054909314949350505050565b5f612f7e612867565b5f848152600f820160209081526040808320805460ff191660021790556010840190915290209290925550600d0155565b5f80612fb9612867565b9050613002612fc786612c06565b85858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250612d0a92505050565b600b8201545f90815260038301602090815260408083206001600160a01b038516845290915290205490925060ff1661306057604051631ac5164960e31b81526001600160a01b03831660048201526024810187905260440161086e565b6001600160a01b0382165f9081526020889052604090205460ff16156130ab57604051639d2799e960e01b81526001600160a01b03831660048201526024810187905260440161086e565b506001600160a01b0381165f9081526020969096526040909520805460ff1916600117905550929392505050565b5f8290036130f957604051621a323560e61b815260040160405180910390fd5b60ff821115613125576040516302d4e4ef60e31b81526004810183905260ff602482015260440161086e565b60408051808201909152601081526f383ab13634b1a232b1b93cb83a34b7b760811b602082015261315890823584613c8b565b6131906040518060400160405280600e81526020016d3ab9b2b92232b1b93cb83a34b7b760911b815250826020013585859050613c8b565b604080518082018252600681526535b6b9a3b2b760d11b60208201526131b99183013584613c8b565b6040805180820190915260038152626d706360e81b60208201526131e290606083013584613c8b565b5f6131eb612867565b8054909150851161321c57805460405163efd55f6760e01b815261086e918791600401918252602082015260400190565b8481555f5b838110156135ca573685858381811061323c5761323c614a76565b905060200281019061324e9190614e79565b60408051608081019091529091505f908061326c6020850185613f9b565b6001600160a01b0316815260200183602001602081019061328d9190613f9b565b6001600160a01b031681526020016132a86040850185614e97565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152505050908252506020016132ee6060850185614e97565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250505091525080519091506001600160a01b031661334e57604051634233402560e11b815260040160405180910390fd5b60208101516001600160a01b031661337957604051632deccf4d60e01b815260040160405180910390fd5b5f888152600285016020908152604080832084516001600160a01b0316845290915290205460ff16156133cd578051604051630d18c4ff60e41b81526001600160a01b03909116600482015260240161086e565b5f8881526003850160209081526040808320848301516001600160a01b0316845290915290205460ff161561342657602081015160405163f51af6bb60e01b81526001600160a01b03909116600482015260240161086e565b5f8881526001858101602090815260408084208054808501825590855293829020855160049095020180546001600160a01b03199081166001600160a01b039687161782559286015193810180549093169390941692909217905582015182919060028201906134969082614f24565b50606082015160038201906134ab9082614f24565b5050505f88815260028086016020908152604080842085516001600160a01b039081168652908352818520805460ff1990811660019081179092558e875260038b018552838720888601805185168952908652848820805490921683179091558e875260048b01855283872088518416885290945294829020865181549083166001600160a01b031991821617825593519581018054969092169590931694909417909355918301518392918201906135649082614f24565b50606082015160038201906135799082614f24565b5050505f8881526005850160209081526040822092810151835460018082018655948452919092200180546001600160a01b0319166001600160a01b03909216919091179055919091019050613221565b505f948552600681016020908152604080872084359055600783018252808720828501359055600883018252808720818501359055600990920190529093206060909301359092555050565b5f6003613621612867565b5f848152600e91909101602052604090205460ff1660038111156136475761364761467c565b1492915050565b5f613657612867565b5f928352600f810160209081526040808520805460ff191690556010909201905282209190915550565b5f8072184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b83106136bf5772184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b830492506040015b6d04ee2d6d415b85acef810000000083106136eb576d04ee2d6d415b85acef8100000000830492506020015b662386f26fc10000831061370957662386f26fc10000830492506010015b6305f5e1008310613721576305f5e100830492506008015b612710831061373557612710830492506004015b60648310613747576064830492506002015b600a8310610d7a5760010192915050565b61376182613cd4565b6040516001600160a01b038316907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a28051156137a557612bb88282613d37565b610e63613da9565b5f805f83516041036137e4576020840151604085015160608601515f1a6137d688828585613dc8565b9550955095505050506137ef565b505081515f91506002905b9250925092565b5f8260038111156138095761380961467c565b03613812575050565b60018260038111156138265761382661467c565b036138445760405163f645eedf60e01b815260040160405180910390fd5b60028260038111156138585761385861467c565b036138795760405163fce698f760e01b81526004810182905260240161086e565b600382600381111561388d5761388d61467c565b03610e63576040516335e2f38360e21b81526004810182905260240161086e565b5f80846001600160401b038111156138c8576138c8614191565b6040519080825280602002602001820160405280156138f1578160200160208202803683370190505b5090505f5b85811015613b3857365f88888481811061391257613912614a76565b90506020028101906139249190614fdf565b613932906040810190614ff3565b90925090505f816001600160401b0381111561395057613950614191565b604051908082528060200260200182016040528015613979578160200160208202803683370190505b5090505f5b82811015613a6b577fddd108772e6a3899feb04d148ae915cbe3eb5ebd202688080399e9921ac3616b8484838181106139b9576139b9614a76565b90506020028101906139cb9190615038565b6139d990602081019061504c565b8585848181106139eb576139eb614a76565b90506020028101906139fd9190615038565b613a0b906020810190614e97565b604051613a199291906149e6565b604051908190038120613a30939291602001615065565b60405160208183030381529060405280519060200120828281518110613a5857613a58614a76565b602090810291909101015260010161397e565b50898985818110613a7e57613a7e614a76565b9050602002810190613a909190614fdf565b358a8a86818110613aa357613aa3614a76565b9050602002810190613ab59190614fdf565b6020013582604051602001613aca9190615087565b60408051601f19818403018152828252805160209182012090830194909452810191909152606081019190915260800160405160208183030381529060405280519060200120858581518110613b2257613b22614a76565b60209081029190910101525050506001016138f6565b505f836001600160401b03811115613b5257613b52614191565b604051908082528060200260200182016040528015613b7b578160200160208202803683370190505b5090505f5b84811015613c5557858582818110613b9a57613b9a614a76565b9050602002810190613bac9190614fdf565b35868683818110613bbf57613bbf614a76565b9050602002810190613bd19190614fdf565b60200135878784818110613be757613be7614a76565b9050602002810190613bf99190614fdf565b613c07906040810190614e97565b604051602001613c1a94939291906150bc565b60405160208183030381529060405280519060200120828281518110613c4257613c42614a76565b6020908102919091010152600101613b80565b508181604051602001613c6992919061511f565b6040516020818303038152906040528051906020012092505050949350505050565b815f03613cad5782604051631b5fdb0760e11b815260040161086e9190613f65565b80821115612bb85782828260405163caa814a360e01b815260040161086e93929190615143565b806001600160a01b03163b5f03613d0957604051634c9c8ce360e01b81526001600160a01b038216600482015260240161086e565b5f8051602061517983398151915280546001600160a01b0319166001600160a01b0392909216919091179055565b60605f80846001600160a01b031684604051613d539190615167565b5f60405180830381855af49150503d805f8114613d8b576040519150601f19603f3d011682016040523d82523d5f602084013e613d90565b606091505b5091509150613da0858383613e90565b95945050505050565b3415612a535760405163b398979f60e01b815260040160405180910390fd5b5f80807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0841115613e0157505f91506003905082613e86565b604080515f808252602082018084528a905260ff891692820192909252606081018790526080810186905260019060a0016020604051602081039080840390855afa158015613e52573d5f803e3d5ffd5b5050604051601f1901519150506001600160a01b038116613e7d57505f925060019150829050613e86565b92505f91508190505b9450945094915050565b606082613ea557613ea082613eef565b613ee8565b8151158015613ebc57506001600160a01b0384163b155b15613ee557604051639996b31560e01b81526001600160a01b038516600482015260240161086e565b50805b9392505050565b805115613eff5780518082602001fd5b60405163d6bda27560e01b815260040160405180910390fd5b5f5b83811015613f32578181015183820152602001613f1a565b50505f910152565b5f8151808452613f51816020860160208601613f18565b601f01601f19169290920160200192915050565b602081525f613ee86020830184613f3a565b6001600160a01b0381168114612956575f80fd5b8035613f9681613f77565b919050565b5f60208284031215613fab575f80fd5b8135613ee881613f77565b5f8083601f840112613fc6575f80fd5b5081356001600160401b03811115613fdc575f80fd5b6020830191508360208260051b8501011115613ff6575f80fd5b9250929050565b5f8083601f84011261400d575f80fd5b5081356001600160401b03811115614023575f80fd5b602083019150836020828501011115613ff6575f80fd5b5f805f805f805f87890360e0811215614051575f80fd5b88356001600160401b0380821115614067575f80fd5b6140738c838d01613fb6565b909a5098508891506080601f198401121561408c575f80fd5b60208b01975060a08b01359250808311156140a5575f80fd5b6140b18c848d01613ffd565b909750955060c08b01359250869150808311156140cc575f80fd5b50506140da8a828b01613fb6565b989b979a50959850939692959293505050565b5f602082840312156140fd575f80fd5b5035919050565b5f8060408385031215614115575f80fd5b82359150602083013561412781613f77565b809150509250929050565b5f60018060a01b03808351168452806020840151166020850152506040820151608060408501526141666080850182613f3a565b905060608301518482036060860152613da08282613f3a565b602081525f613ee86020830184614132565b634e487b7160e01b5f52604160045260245ffd5b5f80604083850312156141b6575f80fd5b82356141c181613f77565b915060208301356001600160401b03808211156141dc575f80fd5b818501915085601f8301126141ef575f80fd5b81358181111561420157614201614191565b604051601f8201601f19908116603f0116810190838211818310171561422957614229614191565b81604052828152886020848701011115614241575f80fd5b826020860160208301375f6020848301015280955050505050509250929050565b602080825282518282018190525f9190848201906040850190845b818110156142a25783516001600160a01b03168352928401929184019160010161427d565b50909695505050505050565b5f805f805f606086880312156142c2575f80fd5b8535945060208601356001600160401b03808211156142df575f80fd5b6142eb89838a01613ffd565b90965094506040880135915080821115614303575f80fd5b5061431088828901613ffd565b969995985093965092949392505050565b5f805f805f805f805f60a08a8c031215614339575f80fd5b8935985060208a01356001600160401b0380821115614356575f80fd5b6143628d838e01613fb6565b909a50985060408c013591508082111561437a575f80fd5b6143868d838e01613fb6565b909850965060608c013591508082111561439e575f80fd5b6143aa8d838e01613ffd565b909650945060808c01359150808211156143c2575f80fd5b506143cf8c828d01613ffd565b915080935050809150509295985092959850929598565b5f805f805f805f6080888a0312156143fc575f80fd5b8735965060208801356001600160401b0380821115614419575f80fd5b6144258b838c01613fb6565b909850965060408a013591508082111561443d575f80fd5b6144498b838c01613ffd565b909650945060608a0135915080821115614461575f80fd5b506140da8a828b01613ffd565b80356001600160401b0381168114613f96575f80fd5b5f805f805f8060808789031215614499575f80fd5b8635955060208701356001600160401b03808211156144b6575f80fd5b6144c28a838b01613ffd565b909750955060408901359150808211156144da575f80fd5b818901915089601f8301126144ed575f80fd5b8135818111156144fb575f80fd5b8a602060608302850101111561450f575f80fd5b6020830195508094505050506145276060880161446e565b90509295509295509295565b5f8060408385031215614544575f80fd5b50508035926020909101359150565b5f8282518085526020808601955060208260051b840101602086015f5b8481101561459e57601f1986840301895261458c838351614132565b98840198925090830190600101614570565b5090979650505050505050565b602081525f613ee86020830184614553565b5f85516145ce818460208a01613f18565b61103b60f11b90830190815285516145ed816002840160208a01613f18565b808201915050601760f91b8060028301528551614611816003850160208a01613f18565b6003920191820152835161462c816004840160208801613f18565b016004019695505050505050565b5f6020828403121561464a575f80fd5b8151613ee881613f77565b634e487b7160e01b5f52601160045260245ffd5b81810381811115610d7a57610d7a614655565b634e487b7160e01b5f52602160045260245ffd5b80820180821115610d7a57610d7a614655565b5f808335601e198436030181126146b8575f80fd5b83016020810192503590506001600160401b038111156146d6575f80fd5b803603821315613ff6575f80fd5b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b8035600381900b8114613f96575f80fd5b5f8235605e19833603018112614731575f80fd5b90910192915050565b5f8383855260208086019550808560051b830101845f5b8781101561459e57848303601f1901895261476c828861471d565b606061477882836146a3565b82875261478883880182846146e4565b92505050614798868301836146a3565b868303888801526147aa8382846146e4565b9250505060406147bc818401846146a3565b9350868303828801526147d08385836146e4565b9c88019c96505050928501925050600101614751565b60e080825281018790525f61010080830160058a901b840182018b845b8c8110156149465786830360ff190184528135368f900360fe19018112614828575f80fd5b8e016148448461483783613f8b565b6001600160a01b03169052565b6020614851818301613f8b565b6001600160a01b031681860152604061486c838201846146a3565b898389015261487e8a890182846146e4565b925050506060614890818501856146a3565b888403838a01526148a28482846146e4565b935050505060806148b481850161470c565b6148c28289018260030b9052565b505060a06148d2818501856146a3565b888403838a01526148e48482846146e4565b935050505060c06148f7818501856146a3565b888403838a01526149098482846146e4565b935050505061491b60e08401846146a3565b935086820360e08801526149308285836146e4565b9783019796505050929092019150600101614803565b5050614976602086018b803582526020810135602083015260408101356040830152606081013560608301525050565b84810360a086015261498981898b6146e4565b9250505082810360c08401526149a081858761473a565b9a9950505050505050505050565b600181811c908216806149c257607f821691505b6020821081036149e057634e487b7160e01b5f52602260045260245ffd5b50919050565b818382375f9101908152919050565b5f60018201614a0657614a06614655565b5060010190565b604081525f614a206040830186886146e4565b8281036020840152614a338185876146e4565b979650505050505050565b858152606060208201525f614a576060830186886146e4565b8281036040840152614a6a8185876146e4565b98975050505050505050565b634e487b7160e01b5f52603260045260245ffd5b803560048110613f96575f80fd5b60048110614ab457634e487b7160e01b5f52602160045260245ffd5b9052565b5f8383855260208086019550808560051b830101845f5b8781101561459e57848303601f19018952614aea828861471d565b60608135855285820135868601526040614b06818401846146a3565b93508282880152614b1a83880185836146e4565b9c88019c96505050928501925050600101614acf565b5f8282518085526020808601955060208260051b840101602086015f5b8481101561459e57601f19868403018952614b69838351613f3a565b98840198925090830190600101614b4d565b60608082528181018690525f906080808401600589901b850182018a855b8b811015614cb157607f1980898503018552614bb5838f61471d565b8785018135865260208083013581880152604080840135601e19853603018112614bdd575f80fd5b9093019283358285016001600160401b03821115614bf9575f80fd5b8160051b803603821315614c0b575f80fd5b8a84018e9052948290529389018b01935f8c8b0136889003605e19015b84831015614c9557898d89030182528335818112614c44575f80fd5b8901878101614c5b8a614c5683614a8a565b614a98565b614c67888301826146a3565b9250905087898b0152614c7d888b0183836146e4565b99505050928601926001929092019190860190614c28565b5050509983019950929750509490940193505050600101614b99565b50508581036020870152614cc681898b614ab8565b93505050508281036040840152614a6a8185614b30565b5f6001600160fb1b03831115614cf1575f80fd5b8260051b80858437919091019392505050565b606080825281018690525f6001600160fb1b03871115614d22575f80fd5b8660051b80896080850137820182810360809081016020850152614d4990820187896146e4565b90508281036040840152614d5e8185876146e4565b9998505050505050505050565b60a081525f614d7d60a0830185614553565b9050613ee86020830184803582526020810135602083015260408101356040830152606081013560608301525050565b5f60208284031215614dbd575f80fd5b613ee88261446e565b5f6060808352614dda60608401888a6146e4565b838103602085810191909152868252879181015f5b88811015614e41576001600160401b0380614e098661446e565b16835280614e1885870161446e565b1684840152604081614e2b82880161446e565b1690840152509284019290840190600101614def565b50809450505050506001600160401b03831660408301529695505050505050565b5f60208284031215614e72575f80fd5b5051919050565b5f823560fe19833603018112614e8d575f80fd5b9190910192915050565b5f808335601e19843603018112614eac575f80fd5b8301803591506001600160401b03821115614ec5575f80fd5b602001915036819003821315613ff6575f80fd5b601f821115612bb857805f5260205f20601f840160051c81016020851015614efe5750805b601f840160051c820191505b81811015614f1d575f8155600101614f0a565b5050505050565b81516001600160401b03811115614f3d57614f3d614191565b614f5181614f4b84546149ae565b84614ed9565b602080601f831160018114614f84575f8415614f6d5750858301515b5f19600386901b1c1916600185901b178555612925565b5f85815260208120601f198616915b82811015614fb257888601518255948401946001909101908401614f93565b5085821015614fcf57878501515f19600388901b60f8161c191681555b5050505050600190811b01905550565b5f8235605e19833603018112614e8d575f80fd5b5f808335601e19843603018112615008575f80fd5b8301803591506001600160401b03821115615021575f80fd5b6020019150600581901b3603821315613ff6575f80fd5b5f8235603e19833603018112614e8d575f80fd5b5f6020828403121561505c575f80fd5b613ee882614a8a565b838152606081016150796020830185614a98565b826040830152949350505050565b81515f9082906020808601845b838110156150b057815185529382019390820190600101615094565b50929695505050505050565b848152836020820152606060408201525f6150db6060830184866146e4565b9695505050505050565b5f815180845260208085019450602084015f5b83811015615114578151875295820195908201906001016150f8565b509495945050505050565b604081525f61513160408301856150e5565b8281036020840152613da081856150e5565b606081525f6151556060830186613f3a565b60208301949094525060400152919050565b5f8251614e8d818460208701613f1856fe360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x02\x12W_5`\xE0\x1C\x80cun\x86R\x11a\x01\x1EW\x80c\xBF\x9B\x16\xC8\x11a\0\xA8W\x80c\xC9\x99\xA8\xB4\x11a\0mW\x80c\xC9\x99\xA8\xB4\x14a\x05\xDCW\x80c\xCC\xBF\x81\x99\x14a\x05\xFBW\x80c\xCC\xEA\xC0\x19\x14a\x06\x1AW\x80c\xEE}R\xD1\x14a\x069W\x80c\xF9\xC6p\xC3\x14a\x06XW_\x80\xFD[\x80c\xBF\x9B\x16\xC8\x14a\x05LW\x80c\xC0\xAEd\xF7\x14a\x05kW\x80c\xC2X\n-\x14a\x05\x8AW\x80c\xC2\xB4)\x86\x14a\x05\xA9W\x80c\xC3\xAA\xAAZ\x14a\x05\xBDW_\x80\xFD[\x80c\x97o>\xB9\x11a\0\xEEW\x80c\x97o>\xB9\x14a\x04\xC1W\x80c\x9D\x1B\x1B\xE1\x14a\x04\xD5W\x80c\xA6B\xBA\x0F\x14a\x04\xE9W\x80c\xAD<\xB1\xCC\x14a\x05\x08W\x80c\xB4r+\xC4\x14a\x058W_\x80\xFD[\x80cun\x86R\x14a\x04PW\x80c~\xAA\xC8\xF2\x14a\x04oW\x80c\x94G\xCF\xD4\x14a\x04\x83W\x80c\x97l\x98\xB5\x14a\x04\xA2W_\x80\xFD[\x80cA\xAD\x06\x9C\x11a\x01\x9FW\x80cR\xD1\x90-\x11a\x01oW\x80cR\xD1\x90-\x14a\x03\xA9W\x80c[\xFFv\xD9\x14a\x03\xBDW\x80ce\xB3\x94\xAF\x14a\x03\xE9W\x80cn\xC0\xF2?\x14a\x04\x12W\x80cq\xD7m\xAE\x14a\x041W_\x80\xFD[\x80cA\xAD\x06\x9C\x14a\x039W\x80cF\xC5\xBB\xBD\x14a\x03XW\x80cG\xE8\"\x95\x14a\x03wW\x80cO\x1E\xF2\x86\x14a\x03\x96W_\x80\xFD[\x80c\"\x1C\xDDN\x11a\x01\xE5W\x80c\"\x1C\xDDN\x14a\x02\x99W\x80c&\xCF]\xEF\x14a\x02\xB8W\x80c(\x1E\x8B\xFE\x14a\x02\xDAW\x80c*8\x89\x98\x14a\x02\xF9W\x80c1\xFFA\xC8\x14a\x03\rW_\x80\xFD[\x80c\r\x8En,\x14a\x02\x16W\x80c\x12:\xBB(\x14a\x02@W\x80c\x1C\xE3\xF9\xBC\x14a\x02VW\x80c =\x01\x14\x14a\x02jW[_\x80\xFD[4\x80\x15a\x02!W_\x80\xFD[Pa\x02*a\x06\x84V[`@Qa\x027\x91\x90a?eV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02KW_\x80\xFD[Pa\x02Ta\x06\xF0V[\0[4\x80\x15a\x02aW_\x80\xFD[Pa\x02Ta\x07\xCBV[4\x80\x15a\x02uW_\x80\xFD[Pa\x02\x89a\x02\x846`\x04a?\x9BV[a\x08\xF5V[`@Q\x90\x15\x15\x81R` \x01a\x027V[4\x80\x15a\x02\xA4W_\x80\xFD[Pa\x02Ta\x02\xB36`\x04a@:V[a\t5V[4\x80\x15a\x02\xC3W_\x80\xFD[Pa\x02\xCCa\x0BKV[`@Q\x90\x81R` \x01a\x027V[4\x80\x15a\x02\xE5W_\x80\xFD[Pa\x02\xCCa\x02\xF46`\x04a@\xEDV[a\x0BqV[4\x80\x15a\x03\x04W_\x80\xFD[Pa\x02\xCCa\x0B\x96V[4\x80\x15a\x03\x18W_\x80\xFD[Pa\x03,a\x03'6`\x04aA\x04V[a\x0B\xBCV[`@Qa\x027\x91\x90aA\x7FV[4\x80\x15a\x03DW_\x80\xFD[Pa\x02\xCCa\x03S6`\x04a@\xEDV[a\r\x80V[4\x80\x15a\x03cW_\x80\xFD[Pa\x02\x89a\x03r6`\x04aA\x04V[a\r\xC5V[4\x80\x15a\x03\x82W_\x80\xFD[Pa\x02\xCCa\x03\x916`\x04a@\xEDV[a\x0E#V[a\x02Ta\x03\xA46`\x04aA\xA5V[a\x0EHV[4\x80\x15a\x03\xB4W_\x80\xFD[Pa\x02\xCCa\x0EgV[4\x80\x15a\x03\xC8W_\x80\xFD[Pa\x03\xDCa\x03\xD76`\x04a@\xEDV[a\x0E\x82V[`@Qa\x027\x91\x90aBbV[4\x80\x15a\x03\xF4W_\x80\xFD[Pa\x03\xFDa\x0F\x05V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x027V[4\x80\x15a\x04\x1DW_\x80\xFD[Pa\x02Ta\x04,6`\x04aB\xAEV[a\x0F%V[4\x80\x15a\x04<W_\x80\xFD[Pa\x02Ta\x04K6`\x04aC!V[a\x13\x86V[4\x80\x15a\x04[W_\x80\xFD[Pa\x02Ta\x04j6`\x04aC\xE6V[a\x16&V[4\x80\x15a\x04zW_\x80\xFD[Pa\x03\xDCa\x17\x95V[4\x80\x15a\x04\x8EW_\x80\xFD[Pa\x02\x89a\x04\x9D6`\x04aA\x04V[a\x18\x16V[4\x80\x15a\x04\xADW_\x80\xFD[Pa\x02Ta\x04\xBC6`\x04a@:V[a\x18TV[4\x80\x15a\x04\xCCW_\x80\xFD[Pa\x02\xCCa\x1C\x04V[4\x80\x15a\x04\xE0W_\x80\xFD[Pa\x02\xCCa\x1C\x18V[4\x80\x15a\x04\xF4W_\x80\xFD[Pa\x02Ta\x05\x036`\x04aB\xAEV[a\x1C)V[4\x80\x15a\x05\x13W_\x80\xFD[Pa\x02*`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[4\x80\x15a\x05CW_\x80\xFD[Pa\x02\xCCa\x1D|V[4\x80\x15a\x05WW_\x80\xFD[Pa\x02\x89a\x05f6`\x04a@\xEDV[a\x1D\xA2V[4\x80\x15a\x05vW_\x80\xFD[Pa\x02Ta\x05\x856`\x04a@\xEDV[a\x1D\xACV[4\x80\x15a\x05\x95W_\x80\xFD[Pa\x02Ta\x05\xA46`\x04a@\xEDV[a\x1F0V[4\x80\x15a\x05\xB4W_\x80\xFD[Pa\x02\xCCa!%V[4\x80\x15a\x05\xC8W_\x80\xFD[Pa\x02\xCCa\x05\xD76`\x04a@\xEDV[a!KV[4\x80\x15a\x05\xE7W_\x80\xFD[Pa\x03\xFDa\x05\xF66`\x04a@\xEDV[a!pV[4\x80\x15a\x06\x06W_\x80\xFD[Pa\x02Ta\x06\x156`\x04aD\x84V[a!\xD7V[4\x80\x15a\x06%W_\x80\xFD[Pa\x02\x89a\x0646`\x04aE3V[a$\xE3V[4\x80\x15a\x06DW_\x80\xFD[Pa\x02\x89a\x06S6`\x04a@\xEDV[a%HV[4\x80\x15a\x06cW_\x80\xFD[Pa\x06wa\x06r6`\x04a@\xEDV[a%RV[`@Qa\x027\x91\x90aE\xABV[```@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mProtocolConfig`\x90\x1B\x81RPa\x06\xB6_a'\x15V[a\x06\xC0`\x04a'\x15V[a\x06\xC9_a'\x15V[`@Q` \x01a\x06\xDC\x94\x93\x92\x91\x90aE\xBDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x05\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\x079WP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a\x07WW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x17`\x01`@\x1B\x17\x81Ua\x07\x80a'\xA4V[\x80T`\xFF`@\x1B\x19\x16\x81U`@Q`\x01`\x01`@\x1B\x03\x83\x16\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x1BW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08?\x91\x90aF:V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x08wW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\x08\x7Fa'\xA4V[_a\x08\x88a(gV[`\x0B\x81\x01T\x90\x91P_a\x08\x9A\x82a(\x8BV[\x90P\x80\x82\x7F\x15\xAA\xAFG^\xF4\x07T?Qd\xF5}\xCFW\xF7\xF98\x16\xF5[\xAEw\xCA\t\xEF\xC4E\xBA@\xEE\xF7\x84\x86`\r\x01T`\x01Ca\x08\xD2\x91\x90aFiV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01`@Q\x80\x91\x03\x90\xA3PPPV[_\x80a\x08\xFFa(gV[`\x0B\x81\x01T_\x90\x81R`\x03\x90\x91\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x90\x96\x16\x83R\x94\x90R\x92\x90\x92 T`\xFF\x16\x92\x91PPV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x16`\x01\x14a\t\x89W`@QcoOs\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x05\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\t\xD2WP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a\t\xF0W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x17`\x01`@\x1B\x17\x81U_a\n\x1Aa(gV[\x90P_a\n,`\x07`\xF8\x1B`\x01aF\x90V[\x90Pa\nI\x81a\nA`\x01`\xFB\x1B`\x01aF\x90V[\x8D\x8D\x8Da(\xDCV[`@Q\x80`@\x01`@R\x80C\x81R` \x01\x8C\x8C\x8C\x8C\x8C\x8C\x8C`@Q` \x01a\nw\x97\x96\x95\x94\x93\x92\x91\x90aG\xE6V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x90\x92R_\x84\x81R`\x17\x86\x01\x82R\x91\x90\x91 \x82Q\x81U\x91\x01Q`\x01\x90\x91\x01U`\xF8`\x07\x90\x1B\x81\x7F Mk\x80\x12\x11T\xCD\x87\xD9\x9C\xF5Lc\x9A=\xD0\xA5;0\x84'p\x98\xDE\x97.\xBD\xD3Lk\xE9\x8D\x8D\x8D\x8D\x8D\x8D\x8D`@Qa\n\xEF\x97\x96\x95\x94\x93\x92\x91\x90aG\xE6V[`@Q\x80\x91\x03\x90\xA3PP\x80T`\xFF`@\x1B\x19\x16\x81U`@Q`\x01`\x01`@\x1B\x03\x83\x16\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPPPPV[_\x80a\x0BUa(gV[`\x0B\x81\x01T_\x90\x81R`\t\x90\x91\x01` R`@\x90 T\x92\x91PPV[_a\x0B{\x82a)-V[a\x0B\x83a(gV[_\x92\x83R`\x07\x01` RP`@\x90 T\x90V[_\x80a\x0B\xA0a(gV[`\x0B\x81\x01T_\x90\x81R`\x06\x90\x91\x01` R`@\x90 T\x92\x91PPV[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01R``\x91\x81\x01\x82\x90R\x81\x81\x01\x91\x90\x91Ra\x0B\xE7\x83a)YV[a\x0C\x07W`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x08nV[a\x0C\x0Fa(gV[_\x84\x81R`\x04\x91\x90\x91\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x85R\x90\x83R\x92\x81\x90 \x81Q`\x80\x81\x01\x83R\x81T\x85\x16\x81R`\x01\x82\x01T\x90\x94\x16\x92\x84\x01\x92\x90\x92R`\x02\x82\x01\x80T\x91\x84\x01\x91a\x0Cg\x90aI\xAEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0C\x93\x90aI\xAEV[\x80\x15a\x0C\xDEW\x80`\x1F\x10a\x0C\xB5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\xDEV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0C\xC1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\x0C\xF7\x90aI\xAEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r#\x90aI\xAEV[\x80\x15a\rnW\x80`\x1F\x10a\rEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\rnV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\rQW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P[\x92\x91PPV[_a\r\x8A\x82a)wV[a\r\xAAW`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x08nV[a\r\xB2a(gV[_\x92\x83R`\x08\x01` RP`@\x90 T\x90V[_a\r\xCF\x83a)wV[a\r\xEFW`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x08nV[a\r\xF7a(gV[_\x93\x84R`\x02\x01` \x90\x81R`@\x80\x85 `\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x85R\x92\x90RP\x90 T`\xFF\x16\x90V[_a\x0E-\x82a)-V[a\x0E5a(gV[_\x92\x83R`\t\x01` RP`@\x90 T\x90V[a\x0EPa)\xAFV[a\x0EY\x82a*UV[a\x0Ec\x82\x82a*\xFCV[PPV[_a\x0Epa+\xBDV[P_\x80Q` aQy\x839\x81Q\x91R\x90V[``a\x0E\x8D\x82a)-V[a\x0E\x95a(gV[`\x05\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0E\xF9W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x0E\xDBW[PPPPP\x90P\x91\x90PV[_\x80_a\x0F\x10a(gV[\x90P\x80`\x0B\x01T\x92P\x80`\r\x01T\x91PP\x90\x91V[_a\x0F.a(gV[\x90P_\x7F\x98\xBC2\xBB@E\xAB\xD7\x9Ff0]G'&y\x08\x7F\xF1-\x8A\x03\x15,\xE1J\tT\xB7p\x9C\0\x90P`\x01_\x88\x81R`\x0E\x84\x01` R`@\x90 T`\xFF\x16`\x03\x81\x11\x15a\x0FzWa\x0FzaF|V[\x14a\x0F\x9BW`@Qc5\x86\xEF\xA1`\xE0\x1B\x81R`\x04\x81\x01\x88\x90R`$\x01a\x08nV[`\x0B\x82\x01T_\x88\x81R` \x83\x90R`@\x80\x82 T\x90Qa\x104\x91\x7F\x8A}1\x90\xA5\x14\xCD\xB5\xD5|\x84\x1C\xB78.\x85\xAD!\xB8\x92\xB7\xFD\xFD;\x87\xAB\0\t%\xD9\xD5\x01\x91\x85\x91\x8D\x91a\x0F\xE8\x90\x8C\x90\x8C\x90aI\xE6V[`@\x80Q\x91\x82\x90\x03\x82 ` \x83\x01\x96\x90\x96R\x81\x01\x93\x90\x93R``\x83\x01\x91\x90\x91R`\x80\x82\x01R`\xA0\x81\x01\x91\x90\x91R`\xC0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a,\x06V[\x90P_a\x10v\x82\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa-\n\x92PPPV[\x90P_\x85`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x83`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P_\x86`\x03\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _\x84`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x81\x15\x80\x15a\x11\x0CWP\x80\x15[\x15a\x11<W`@Qc\x17\x03\xBF\x1D`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x8D\x90R`D\x01a\x08nV[_\x8C\x81R`\x01\x87\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15a\x11\x93W`@Qc\x0CK\x0B\x99`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x8D\x90R`D\x01a\x08nV[_\x8C\x81R`\x01\x87\x81\x01` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x88\x16\x85R\x90\x91R\x90\x91 \x80T`\xFF\x19\x16\x90\x91\x17\x90U\x81\x15a\x11\xF6W_\x8C\x81R`\x02\x87\x01` \x90\x81R`@\x80\x83 \x87\x84R\x90\x91R\x81 \x80T\x90\x91\x90a\x11\xF1\x90aI\xF5V[\x90\x91UP[\x80\x15a\x12)W_\x8C\x81R`\x03\x87\x01` \x90\x81R`@\x80\x83 \x87\x84R\x90\x91R\x81 \x80T\x90\x91\x90a\x12$\x90aI\xF5V[\x90\x91UP[\x82`\x01`\x01`\xA0\x1B\x03\x16\x8C\x7FBT\xD9H@+\xA0Y\xC0/\x9F\xEE\xCF\xCA\t\xB4\x0C\x0B9\x02j4J\x15\x9E:vy\x80z\x83\xD2\x8D\x8D\x8D\x8D`@Qa\x12i\x94\x93\x92\x91\x90aJ\rV[`@Q\x80\x91\x03\x90\xA3_\x85\x81R`\t\x88\x01` \x90\x81R`@\x80\x83 T`\x01\x8B\x01\x90\x92R\x82 Ta\x12\x98\x91\x90aFiV[_\x8E\x81R`\x01\x8A\x01` \x90\x81R`@\x80\x83 T`\x03\x8C\x01\x83R\x81\x84 \x8A\x85R\x90\x92R\x90\x91 T\x91\x92P\x14\x80\x15a\x12\xF7WP_\x81\x11a\x12\xD7W`\x01a\x12\xD9V[\x80[_\x8E\x81R`\x02\x89\x01` \x90\x81R`@\x80\x83 \x89\x84R\x90\x91R\x90 T\x10\x15[\x15a\x13wW_\x8D\x81R`\x0E\x89\x01` R`@\x81 \x80T`\xFF\x19\x16`\x02\x17\x90Ua\x13\x1F\x8Ea(\x8BV[\x90P\x80\x8E\x7F\x15\xAA\xAFG^\xF4\x07T?Qd\xF5}\xCFW\xF7\xF98\x16\xF5[\xAEw\xCA\t\xEF\xC4E\xBA@\xEE\xF7\x89\x8C`\r\x01T`\x01Ca\x13W\x91\x90aFiV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01`@Q\x80\x91\x03\x90\xA3P[PPPPPPPPPPPPPV[_a\x13\x8Fa(gV[\x90P`\x01_\x8B\x81R`\x0F\x83\x01` R`@\x90 T`\xFF\x16`\x02\x81\x11\x15a\x13\xB7Wa\x13\xB7aF|V[\x14a\x13\xD8W`@Qcy\x95\xBB\xCF`\xE0\x1B\x81R`\x04\x81\x01\x8B\x90R`$\x01a\x08nV[_\x8A\x81R`\x10\x82\x01` R`@\x81 T\x90\x80\x80a\x13\xFD\x84\x8F\x8F\x8F\x8F\x8F\x8F\x8F\x8F\x8Fa-2V[\x92P\x92P\x92P_a\x14\x10\x8F\x86\x86\x85a.\x9FV[\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16\x8F\x7F\x7F\xAE\xF7E\xAFy\xA3\xAF#\xD4\x19\x9C\xE06iu\xCC-\xD7\x14\x93\x84\xD613}\x13;\"k[\xE4\x85\x8D\x8D\x8D\x8D`@Qa\x14T\x95\x94\x93\x92\x91\x90aJ>V[`@Q\x80\x91\x03\x90\xA3\x80a\x14lWPPPPPPa\x16\x1BV[PPP_\x82\x81R`\x0E\x84\x01` R`@\x90 \x80T`\xFF\x19\x16`\x03\x17\x90UP`\x0B\x82\x01\x81\x90Ua\x14\x9B\x8B\x82a/uV[_\x81\x81R`\x01\x83\x01` R`@\x81 \x80T\x90\x91\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xC6Wa\x14\xC6aA\x91V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14\xF9W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x14\xE4W\x90P[P\x90P_[\x82T\x81\x10\x15a\x15\xD4W\x82\x81\x81T\x81\x10a\x15\x19Wa\x15\x19aJvV[\x90_R` _ \x90`\x04\x02\x01`\x03\x01\x80Ta\x153\x90aI\xAEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15_\x90aI\xAEV[\x80\x15a\x15\xAAW\x80`\x1F\x10a\x15\x81Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\xAAV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\x8DW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x82\x82\x81Q\x81\x10a\x15\xC1Wa\x15\xC1aJvV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x14\xFEV[P\x8C\x83\x7Fln\xB3X\x81\xF8\x05\xF2\x84\xBEY\xB8@\xB4\xEA\x99\x0F\x96\xB1=\xD9\xE7\x19,E\x81\xE1\xBC\x81\xC0\xAF\x98\x8E\x8E\x8E\x8E\x87`@Qa\x16\x0E\x95\x94\x93\x92\x91\x90aK{V[`@Q\x80\x91\x03\x90\xA3PPPP[PPPPPPPPPV[a\x16.a(gV[_\x88\x81R`\n\x91\x90\x91\x01` R`@\x90 T`\xFF\x16a\x16cW`@Qc+s\xD9\xB7`\xE0\x1B\x81R`\x04\x81\x01\x88\x90R`$\x01a\x08nV[_\x87\x81R\x7F\x98\xBC2\xBB@E\xAB\xD7\x9Ff0]G'&y\x08\x7F\xF1-\x8A\x03\x15,\xE1J\tT\xB7p\x9C\x04` \x90\x81R`@\x80\x83 \x90Qa\x17=\x92\x8B\x91\x7F\xC6\x87\xB65%\xEF \xD9TB\xD5k\xD1@\xC9\xDAl\x10\xC6\xE2?H$\x86\xC2F\xC0R6\xCE\xEF\x9C\x91\x83\x91a\x16\xCC\x91\x8E\x91\x8E\x91\x01aL\xDDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x88\x88`@Qa\x16\xF2\x92\x91\x90aI\xE6V[`@Q\x90\x81\x90\x03\x81 a\x17 \x94\x93\x92\x91` \x01\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x88\x88a/\xAFV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x88\x7F\xEA\x16@\xF3M\xEA\xDA/\x81]\xEF\x0E\x11.l\xD2IS8I\xA0\xFDS\xE9);\xA3M|F\x88\xCB\x89\x89\x89\x89\x89\x89`@Qa\x17\x83\x96\x95\x94\x93\x92\x91\x90aM\x04V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPV[``_a\x17\xA0a(gV[\x90P\x80`\x05\x01_\x82`\x0B\x01T\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x18\x0BW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x17\xEDW[PPPPP\x91PP\x90V[_a\x18 \x83a)-V[a\x18(a(gV[_\x93\x84R`\x03\x01` \x90\x81R`@\x80\x85 `\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x85R\x92\x90RP\x90 T`\xFF\x16\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xA4W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xC8\x91\x90aF:V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18\xFBW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x08nV[a\x19\x03a'\xA4V[_a\x19\x0Ca(gV[`\x0B\x81\x01T\x81T\x91\x92P\x90_\x90a\x19$\x90`\x01aF\x90V[\x90Pa\x192\x81\x8B\x8B\x8Ba0\xD9V[_\x81\x81R`\x0E\x84\x01` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x86\x01\x82R\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R\x91\x92\x90\x91\x90\x84\x90\x84\x01[\x82\x82\x10\x15a\x1A\xE8W_\x84\x81R` \x90\x81\x90 `@\x80Q`\x80\x81\x01\x82R`\x04\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x84R`\x01\x82\x01T\x16\x93\x83\x01\x93\x90\x93R`\x02\x83\x01\x80T\x92\x93\x92\x91\x84\x01\x91a\x19\xC9\x90aI\xAEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x19\xF5\x90aI\xAEV[\x80\x15a\x1A@W\x80`\x1F\x10a\x1A\x17Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1A@V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1A#W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\x1AY\x90aI\xAEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1A\x85\x90aI\xAEV[\x80\x15a\x1A\xD0W\x80`\x1F\x10a\x1A\xA7Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1A\xD0V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1A\xB3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x19tV[PPPP\x90P\x80\x89`@Q` \x01a\x1B\x01\x92\x91\x90aMkV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x1B>\x7F\x98\xBC2\xBB@E\xAB\xD7\x9Ff0]G'&y\x08\x7F\xF1-\x8A\x03\x15,\xE1J\tT\xB7p\x9C\0\x90V[_\x84\x81R` \x91\x82R`@\x90\x81\x90 \x92\x90\x92U\x81Q\x80\x83\x01\x83RC\x81R\x91Q\x82\x82\x01\x91a\x1By\x91\x8F\x91\x8F\x91\x8F\x91\x8F\x91\x8F\x91\x8F\x91\x8F\x91\x01aG\xE6V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x90\x92R_\x85\x81R`\x17\x88\x01\x82R\x82\x90 \x83Q\x81U\x92\x01Q`\x01\x90\x92\x01\x91\x90\x91UQ\x83\x90\x83\x90\x7F Mk\x80\x12\x11T\xCD\x87\xD9\x9C\xF5Lc\x9A=\xD0\xA5;0\x84'p\x98\xDE\x97.\xBD\xD3Lk\xE9\x90a\x1B\xEF\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90aG\xE6V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPPPPV[_\x80a\x1C\x0Ea(gV[`\x0B\x01T\x92\x91PPV[_\x80a\x1C\"a(gV[T\x92\x91PPV[_\x85\x81R\x7F\x98\xBC2\xBB@E\xAB\xD7\x9Ff0]G'&y\x08\x7F\xF1-\x8A\x03\x15,\xE1J\tT\xB7p\x9C\x06` R`@\x90 T\x7F\x98\xBC2\xBB@E\xAB\xD7\x9Ff0]G'&y\x08\x7F\xF1-\x8A\x03\x15,\xE1J\tT\xB7p\x9C\0\x90`\xFF\x16a\x1C\x9AW`@Qb=\x8BM`\xE4\x1B\x81R`\x04\x81\x01\x87\x90R`$\x01a\x08nV[_a\x1D)\x82`\x05\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x88\x7FJ\xF2\xC2\x96|l\x1F\xA8pl%\x04\xBB\x0C\xAD\x84?\x8D\x0F\xACy\xF3\x1F-\x02\x06\xE7G\xF0I\xFC\xA2\x8A\x88\x88`@Qa\x1C\xE4\x92\x91\x90aI\xE6V[`@Q\x90\x81\x90\x03\x81 a\x1D\x0C\x93\x92\x91` \x01\x92\x83R` \x83\x01\x91\x90\x91R`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x89\x89a/\xAFV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x87\x7F\xEE\x1B\x8F)\xD5\x16\xD5c\xF4\x1F*;u\xC2\0u/\xEBQ.bV\xEBV\x1D\x17\xA4\xE6\x1E\xE63^\x88\x88\x88\x88`@Qa\x1Dk\x94\x93\x92\x91\x90aJ\rV[`@Q\x80\x91\x03\x90\xA3PPPPPPPV[_\x80a\x1D\x86a(gV[`\x0B\x81\x01T_\x90\x81R`\x08\x90\x91\x01` R`@\x90 T\x92\x91PPV[_a\rz\x82a6\x16V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xFCW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E \x91\x90aF:V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1ESW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x08nV[_a\x1E\\a(gV[\x90P\x80`\x0B\x01T\x82\x03a\x1E\x85W`@Qc\x03\x0F]U`\xE3\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x08nV[a\x1E\x8E\x82a)wV[a\x1E\xAEW`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x08nV[_\x82\x81R`\n\x82\x01` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x90\x81\x16`\x01\x17\x90\x91U`\x0E\x85\x01\x83R\x81\x84 \x80T\x90\x91\x16\x90U`\x0C\x84\x01T\x80\x84R`\x10\x85\x01\x90\x92R\x90\x91 T\x83\x90\x03a\x1F\x01Wa\x1F\x01\x81a6NV[`@Q\x83\x90\x7F\xDA\x07]\t\x19\x8D ~:\x91\x8DK\x8D\xFC\x87\xDF-`\xA0\x0B\xE7\x03\xFD9\xEA\xAC\x90\x96-\xA0\xB7\xF0\x90_\x90\xA2PPPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\x80W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xA4\x91\x90aF:V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1F\xD7W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x08nV[_a\x1F\xE0a(gV[\x90P\x80`\r\x01T\x82\x03a \tW`@Qc\xF0\xE7\x81\xC1`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x08nV[_\x82\x81R`\x0F\x82\x01` R`@\x81 T`\xFF\x16\x90`\x02\x82`\x02\x81\x11\x15a 1Wa 1aF|V[\x14\x90P_`\x01\x83`\x02\x81\x11\x15a IWa IaF|V[\x14\x80\x15a \x88WP`\x03_\x86\x81R`\x10\x86\x01` \x90\x81R`@\x80\x83 T\x83R`\x0E\x88\x01\x90\x91R\x90 T`\xFF\x16`\x03\x81\x11\x15a \x86Wa \x86aF|V[\x14[\x90P\x81\x15\x80\x15a \x96WP\x80\x15[\x15a \xB7W`@Qcy\x95\xBB\xCF`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R`$\x01a\x08nV[a \xC0\x85a6NV[_\x85\x81R\x7F\x98\xBC2\xBB@E\xAB\xD7\x9Ff0]G'&y\x08\x7F\xF1-\x8A\x03\x15,\xE1J\tT\xB7p\x9C\x06` R`@\x80\x82 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x86\x91\x7F\x03C`\x13\x07_\x8D\x1A\xBB\x17\x81\xDB\xCA\xF4\x18\xFCv\xFCy}\x1A\xB5\x1C8fL\x1AQ\xF3\xCDW\xF9\x91\xA2PPPPPV[_\x80a!/a(gV[`\x0B\x81\x01T_\x90\x81R`\x07\x90\x91\x01` R`@\x90 T\x92\x91PPV[_a!U\x82a)-V[a!]a(gV[_\x92\x83R`\x06\x01` RP`@\x90 T\x90V[_\x80a!{\x83a)YV[a!\x9BW`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x08nV[_a!\xA4a(gV[_\x94\x85R`\x17\x01` \x90\x81R`@\x94\x85\x90 \x85Q\x80\x87\x01\x90\x96R\x80T\x80\x87R`\x01\x90\x91\x01T\x95\x90\x91\x01\x85\x90R\x94\x92PPPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"'W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"K\x91\x90aF:V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\"~W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x08nV[\x85_\x03a\"\x9EW`@Qc\t\x92\xF7\xAD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x90\x03a\"\xBFW`@Qc\xB5H\x91G`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x82\x90\x03a\"\xE0W`@Qc/\x94\x14\x11`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`@\x1B\x03\x16_\x03a#\tW`@Qc\x02\xFA})`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x82\x81\x10\x15a$\x9AW6\x84\x84\x83\x81\x81\x10a#&Wa#&aJvV[``\x02\x91\x90\x91\x01\x91Pa#>\x90P` \x82\x01\x82aM\xADV[`\x01`\x01`@\x1B\x03\x16_\x03a#fW`@Qc2\x12!u`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a#v``\x82\x01`@\x83\x01aM\xADV[`\x01`\x01`@\x1B\x03\x16a#\x8F`@\x83\x01` \x84\x01aM\xADV[`\x01`\x01`@\x1B\x03\x16\x11\x15a$\0Wa#\xAB` \x82\x01\x82aM\xADV[a#\xBB`@\x83\x01` \x84\x01aM\xADV[a#\xCB``\x84\x01`@\x85\x01aM\xADV[`@Qcy\x0C\xEE\x07`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x93\x84\x16`\x04\x82\x01R\x91\x83\x16`$\x83\x01R\x90\x91\x16`D\x82\x01R`d\x01a\x08nV[_[\x82\x81\x10\x15a$\x90Wa$\x17` \x83\x01\x83aM\xADV[`\x01`\x01`@\x1B\x03\x16\x86\x86\x83\x81\x81\x10a$2Wa$2aJvV[a$H\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91PaM\xADV[`\x01`\x01`@\x1B\x03\x16\x03a$\x88Wa$c` \x83\x01\x83aM\xADV[`@Qc\x06\xC6~G`\xE4\x1B\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01a\x08nV[`\x01\x01a$\x02V[PP`\x01\x01a#\x0BV[P\x85\x7F\xC4/\x1E\xCA\xC8\xD4\x88\x1E\xF9\xFD3\\\xA9\x8E\xE7%JCk\x92\xBA\x87J`\x9EP\xA7\xD3\x0CH{\x8D\x86\x86\x86\x86\x86`@Qa$\xD3\x95\x94\x93\x92\x91\x90aM\xC6V[`@Q\x80\x91\x03\x90\xA2PPPPPPV[_\x80a$\xEDa(gV[\x90P`\x02_\x84\x81R`\x0F\x83\x01` R`@\x90 T`\xFF\x16`\x02\x81\x11\x15a%\x15Wa%\x15aF|V[\x14\x80\x15a%0WP_\x83\x81R`\x10\x82\x01` R`@\x90 T\x84\x14[\x80\x15a%@WPa%@\x84a6\x16V[\x94\x93PPPPV[_a\rz\x82a)wV[``a%]\x82a)-V[a%ea(gV[`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a'\nW_\x84\x81R` \x90\x81\x90 `@\x80Q`\x80\x81\x01\x82R`\x04\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x84R`\x01\x82\x01T\x16\x93\x83\x01\x93\x90\x93R`\x02\x83\x01\x80T\x92\x93\x92\x91\x84\x01\x91a%\xEB\x90aI\xAEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta&\x17\x90aI\xAEV[\x80\x15a&bW\x80`\x1F\x10a&9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a&bV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a&EW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta&{\x90aI\xAEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta&\xA7\x90aI\xAEV[\x80\x15a&\xF2W\x80`\x1F\x10a&\xC9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a&\xF2V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a&\xD5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a%\x96V[PPPP\x90P\x91\x90PV[``_a'!\x83a6\x81V[`\x01\x01\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a'?Wa'?aA\x91V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a'iW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[_\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a'sWP\x93\x92PPPV[_a'\xADa(gV[\x80T_\x90\x81R`\x0E\x82\x01` R`@\x81 T\x91\x92P`\xFF\x90\x91\x16\x90`\x01\x82`\x03\x81\x11\x15a'\xDCWa'\xDCaF|V[\x14\x80a'\xF9WP`\x02\x82`\x03\x81\x11\x15a'\xF7Wa'\xF7aF|V[\x14[\x90P_`\x01`\x0C\x85\x01T_\x90\x81R`\x0F\x86\x01` R`@\x90 T`\xFF\x16`\x02\x81\x11\x15a('Wa('aF|V[\x14\x90P\x81\x80a(3WP\x80[\x15a(aW\x83T`\x0C\x85\x01T`@Qc\x03\xC0\x10_`\xE1\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x08nV[PPPPV[\x7F\x80\xF3XZ\xF8h\x06\xC5wC\x03\xB0l\x1E\xE6@\xAA\x83\xB6\xEF>E\xDFI\xBB&\xC8RE\0\xC2\0\x90V[_\x80a(\x95a(gV[\x90P\x80`\x0C\x01_\x81Ta(\xA7\x90aI\xF5V[\x91\x82\x90UP_\x81\x81R`\x0F\x83\x01` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16`\x01\x17\x90U`\x10\x90\x94\x01\x90R\x91\x90\x91 \x92\x90\x92UP\x90V[_a(\xE5a(gV[\x90Pa(\xF3\x86\x85\x85\x85a0\xD9V[_\x86\x81R`\x0E\x82\x01` R`@\x90 \x80T`\xFF\x19\x16`\x03\x17\x90U`\x0B\x81\x01\x86\x90U`\x0C\x81\x01\x85\x90Ua)%\x85\x87a/uV[PPPPPPV[a)6\x81a6\x16V[a)VW`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x08nV[PV[_a)ba(gV[_\x92\x83R`\x01\x01` RP`@\x90 T\x15\x15\x90V[_\x80a)\x81a(gV[_\x84\x81R`\x0E\x91\x90\x91\x01` R`@\x90 T`\xFF\x16`\x03\x81\x11\x15a)\xA7Wa)\xA7aF|V[\x14\x15\x92\x91PPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a*5WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a*)_\x80Q` aQy\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a*SW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*\xA5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*\xC9\x91\x90aF:V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a)VW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x08nV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a+VWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra+S\x91\x81\x01\x90aNbV[`\x01[a+~W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x08nV[_\x80Q` aQy\x839\x81Q\x91R\x81\x14a+\xAEW`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x08nV[a+\xB8\x83\x83a7XV[PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a*SW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x82R`\x0E\x81RmProtocolConfig`\x90\x1B` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\x01\x81R`1`\xF8\x1B\x90\x82\x01R\x81Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81\x83\x01R\x7F\xA3\xDE\x18\x80\xCF\x08>\x83\x18\xB7zye\xD0-\xD9v^\x85\xA4\x8EA\x8ADc\xAFz\rW\xB4\xB3\xEE\x81\x84\x01R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xC0\x82\x01\x84R\x80Q\x90\x83\x01 a\x19\x01`\xF0\x1B`\xE0\x83\x01R`\xE2\x82\x01Ra\x01\x02\x80\x82\x01\x94\x90\x94R\x82Q\x80\x82\x03\x90\x94\x01\x84Ra\x01\"\x01\x90\x91R\x81Q\x91\x01 \x90V[_\x80_\x80a-\x18\x86\x86a7\xADV[\x92P\x92P\x92Pa-(\x82\x82a7\xF6V[P\x90\x94\x93PPPPV[_\x80\x80\x89\x15\x80a-@WP\x87\x15[\x15a-aW`@Qc\x18\x85Zy`\xE2\x1B\x81R`\x04\x81\x01\x8D\x90R`$\x01a\x08nV[a-m\x8B\x8B\x8B\x8Ba8\xAEV[\x91Pa-\xEC\x7Fl\xF4}\x8F[\xEDs\xAD*\x8C`\xAD\xBD\xD5\xB5U\xD6e5}\xB7Xf*\xBDI\xEA\x97\x7F\x80\xD1\x9A\x8Ea-\x9Ca(gV[`\r\x01T\x8F\x86\x8A\x8A`@Qa-\xB2\x92\x91\x90aI\xE6V[`@\x80Q\x91\x82\x90\x03\x82 ` \x83\x01\x97\x90\x97R\x81\x01\x94\x90\x94R``\x84\x01\x92\x90\x92R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x81\x01\x91\x90\x91R`\xE0\x01a\x10\x19V[\x90Pa.-\x81\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa-\n\x92PPPV[\x92Pa.7a(gV[_\x8E\x81R`\x03\x91\x90\x91\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x90\x91R\x90 T`\xFF\x16a.\x8FW`@Qc\xA3\xF4\xAF\xEB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x8D\x90R`D\x01a\x08nV[\x9AP\x9AP\x9A\x97PPPPPPPPV[_\x80a.\xA9a(gV[_\x87\x81R`\x12\x82\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x89\x16\x84R\x90\x91R\x90 T\x90\x91P`\xFF\x16\x15a/\x03W`@Qc$\xA0\xBB\x1B`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01a\x08nV[_\x86\x81R`\x12\x82\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x88\x16\x84R\x82R\x80\x83 \x80T`\xFF\x19\x16`\x01\x17\x90U\x88\x83R`\x13\x84\x01\x82R\x80\x83 \x86\x84R\x90\x91R\x81 \x80T\x82\x90a/R\x90aI\xF5V[\x91\x82\x90UP_\x96\x87R`\x05\x90\x92\x01` RP`@\x90\x94 T\x90\x93\x14\x94\x93PPPPV[_a/~a(gV[_\x84\x81R`\x0F\x82\x01` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16`\x02\x17\x90U`\x10\x84\x01\x90\x91R\x90 \x92\x90\x92UP`\r\x01UV[_\x80a/\xB9a(gV[\x90Pa0\x02a/\xC7\x86a,\x06V[\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa-\n\x92PPPV[`\x0B\x82\x01T_\x90\x81R`\x03\x83\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T\x90\x92P`\xFF\x16a0`W`@Qc\x1A\xC5\x16I`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01a\x08nV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R` \x88\x90R`@\x90 T`\xFF\x16\x15a0\xABW`@Qc\x9D'\x99\xE9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01a\x08nV[P`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R` \x96\x90\x96R`@\x90\x95 \x80T`\xFF\x19\x16`\x01\x17\x90UP\x92\x93\x92PPPV[_\x82\x90\x03a0\xF9W`@Qb\x1A25`\xE6\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x82\x11\x15a1%W`@Qc\x02\xD4\xE4\xEF`\xE3\x1B\x81R`\x04\x81\x01\x83\x90R`\xFF`$\x82\x01R`D\x01a\x08nV[`@\x80Q\x80\x82\x01\x90\x91R`\x10\x81Ro8:\xB164\xB1\xA22\xB1\xB9<\xB8:4\xB7\xB7`\x81\x1B` \x82\x01Ra1X\x90\x825\x84a<\x8BV[a1\x90`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m:\xB9\xB2\xB9\"2\xB1\xB9<\xB8:4\xB7\xB7`\x91\x1B\x81RP\x82` \x015\x85\x85\x90Pa<\x8BV[`@\x80Q\x80\x82\x01\x82R`\x06\x81Re5\xB6\xB9\xA3\xB2\xB7`\xD1\x1B` \x82\x01Ra1\xB9\x91\x83\x015\x84a<\x8BV[`@\x80Q\x80\x82\x01\x90\x91R`\x03\x81Rbmpc`\xE8\x1B` \x82\x01Ra1\xE2\x90``\x83\x015\x84a<\x8BV[_a1\xEBa(gV[\x80T\x90\x91P\x85\x11a2\x1CW\x80T`@Qc\xEF\xD5_g`\xE0\x1B\x81Ra\x08n\x91\x87\x91`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[\x84\x81U_[\x83\x81\x10\x15a5\xCAW6\x85\x85\x83\x81\x81\x10a2<Wa2<aJvV[\x90P` \x02\x81\x01\x90a2N\x91\x90aNyV[`@\x80Q`\x80\x81\x01\x90\x91R\x90\x91P_\x90\x80a2l` \x85\x01\x85a?\x9BV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x01` \x81\x01\x90a2\x8D\x91\x90a?\x9BV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a2\xA8`@\x85\x01\x85aN\x97V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x90\x82RP` \x01a2\xEE``\x85\x01\x85aN\x97V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x91RP\x80Q\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16a3NW`@QcB3@%`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16a3yW`@Qc-\xEC\xCFM`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x88\x81R`\x02\x85\x01` \x90\x81R`@\x80\x83 \x84Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15a3\xCDW\x80Q`@Qc\r\x18\xC4\xFF`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01a\x08nV[_\x88\x81R`\x03\x85\x01` \x90\x81R`@\x80\x83 \x84\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15a4&W` \x81\x01Q`@Qc\xF5\x1A\xF6\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01a\x08nV[_\x88\x81R`\x01\x85\x81\x01` \x90\x81R`@\x80\x84 \x80T\x80\x85\x01\x82U\x90\x85R\x93\x82\x90 \x85Q`\x04\x90\x95\x02\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x17\x82U\x92\x86\x01Q\x93\x81\x01\x80T\x90\x93\x16\x93\x90\x94\x16\x92\x90\x92\x17\x90U\x82\x01Q\x82\x91\x90`\x02\x82\x01\x90a4\x96\x90\x82aO$V[P``\x82\x01Q`\x03\x82\x01\x90a4\xAB\x90\x82aO$V[PPP_\x88\x81R`\x02\x80\x86\x01` \x90\x81R`@\x80\x84 \x85Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x86R\x90\x83R\x81\x85 \x80T`\xFF\x19\x90\x81\x16`\x01\x90\x81\x17\x90\x92U\x8E\x87R`\x03\x8B\x01\x85R\x83\x87 \x88\x86\x01\x80Q\x85\x16\x89R\x90\x86R\x84\x88 \x80T\x90\x92\x16\x83\x17\x90\x91U\x8E\x87R`\x04\x8B\x01\x85R\x83\x87 \x88Q\x84\x16\x88R\x90\x94R\x94\x82\x90 \x86Q\x81T\x90\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x82U\x93Q\x95\x81\x01\x80T\x96\x90\x92\x16\x95\x90\x93\x16\x94\x90\x94\x17\x90\x93U\x91\x83\x01Q\x83\x92\x91\x82\x01\x90a5d\x90\x82aO$V[P``\x82\x01Q`\x03\x82\x01\x90a5y\x90\x82aO$V[PPP_\x88\x81R`\x05\x85\x01` \x90\x81R`@\x82 \x92\x81\x01Q\x83T`\x01\x80\x82\x01\x86U\x94\x84R\x91\x90\x92 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x91\x90\x91\x01\x90Pa2!V[P_\x94\x85R`\x06\x81\x01` \x90\x81R`@\x80\x87 \x845\x90U`\x07\x83\x01\x82R\x80\x87 \x82\x85\x015\x90U`\x08\x83\x01\x82R\x80\x87 \x81\x85\x015\x90U`\t\x90\x92\x01\x90R\x90\x93 ``\x90\x93\x015\x90\x92UPPV[_`\x03a6!a(gV[_\x84\x81R`\x0E\x91\x90\x91\x01` R`@\x90 T`\xFF\x16`\x03\x81\x11\x15a6GWa6GaF|V[\x14\x92\x91PPV[_a6Wa(gV[_\x92\x83R`\x0F\x81\x01` \x90\x81R`@\x80\x85 \x80T`\xFF\x19\x16\x90U`\x10\x90\x92\x01\x90R\x82 \x91\x90\x91UPV[_\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a6\xBFWr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a6\xEBWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a7\tWf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a7!Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a75Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a7GW`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\rzW`\x01\x01\x92\x91PPV[a7a\x82a<\xD4V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a7\xA5Wa+\xB8\x82\x82a=7V[a\x0Eca=\xA9V[_\x80_\x83Q`A\x03a7\xE4W` \x84\x01Q`@\x85\x01Q``\x86\x01Q_\x1Aa7\xD6\x88\x82\x85\x85a=\xC8V[\x95P\x95P\x95PPPPa7\xEFV[PP\x81Q_\x91P`\x02\x90[\x92P\x92P\x92V[_\x82`\x03\x81\x11\x15a8\tWa8\taF|V[\x03a8\x12WPPV[`\x01\x82`\x03\x81\x11\x15a8&Wa8&aF|V[\x03a8DW`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a8XWa8XaF|V[\x03a8yW`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x08nV[`\x03\x82`\x03\x81\x11\x15a8\x8DWa8\x8DaF|V[\x03a\x0EcW`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x08nV[_\x80\x84`\x01`\x01`@\x1B\x03\x81\x11\x15a8\xC8Wa8\xC8aA\x91V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a8\xF1W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x85\x81\x10\x15a;8W6_\x88\x88\x84\x81\x81\x10a9\x12Wa9\x12aJvV[\x90P` \x02\x81\x01\x90a9$\x91\x90aO\xDFV[a92\x90`@\x81\x01\x90aO\xF3V[\x90\x92P\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a9PWa9PaA\x91V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a9yW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x82\x81\x10\x15a:kW\x7F\xDD\xD1\x08w.j8\x99\xFE\xB0M\x14\x8A\xE9\x15\xCB\xE3\xEB^\xBD &\x88\x08\x03\x99\xE9\x92\x1A\xC3ak\x84\x84\x83\x81\x81\x10a9\xB9Wa9\xB9aJvV[\x90P` \x02\x81\x01\x90a9\xCB\x91\x90aP8V[a9\xD9\x90` \x81\x01\x90aPLV[\x85\x85\x84\x81\x81\x10a9\xEBWa9\xEBaJvV[\x90P` \x02\x81\x01\x90a9\xFD\x91\x90aP8V[a:\x0B\x90` \x81\x01\x90aN\x97V[`@Qa:\x19\x92\x91\x90aI\xE6V[`@Q\x90\x81\x90\x03\x81 a:0\x93\x92\x91` \x01aPeV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x82\x81Q\x81\x10a:XWa:XaJvV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a9~V[P\x89\x89\x85\x81\x81\x10a:~Wa:~aJvV[\x90P` \x02\x81\x01\x90a:\x90\x91\x90aO\xDFV[5\x8A\x8A\x86\x81\x81\x10a:\xA3Wa:\xA3aJvV[\x90P` \x02\x81\x01\x90a:\xB5\x91\x90aO\xDFV[` \x015\x82`@Q` \x01a:\xCA\x91\x90aP\x87V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01\x94\x90\x94R\x81\x01\x91\x90\x91R``\x81\x01\x91\x90\x91R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85\x85\x81Q\x81\x10a;\"Wa;\"aJvV[` \x90\x81\x02\x91\x90\x91\x01\x01RPPP`\x01\x01a8\xF6V[P_\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a;RWa;RaA\x91V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a;{W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84\x81\x10\x15a<UW\x85\x85\x82\x81\x81\x10a;\x9AWa;\x9AaJvV[\x90P` \x02\x81\x01\x90a;\xAC\x91\x90aO\xDFV[5\x86\x86\x83\x81\x81\x10a;\xBFWa;\xBFaJvV[\x90P` \x02\x81\x01\x90a;\xD1\x91\x90aO\xDFV[` \x015\x87\x87\x84\x81\x81\x10a;\xE7Wa;\xE7aJvV[\x90P` \x02\x81\x01\x90a;\xF9\x91\x90aO\xDFV[a<\x07\x90`@\x81\x01\x90aN\x97V[`@Q` \x01a<\x1A\x94\x93\x92\x91\x90aP\xBCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x82\x81Q\x81\x10a<BWa<BaJvV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a;\x80V[P\x81\x81`@Q` \x01a<i\x92\x91\x90aQ\x1FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x92PPP\x94\x93PPPPV[\x81_\x03a<\xADW\x82`@Qc\x1B_\xDB\x07`\xE1\x1B\x81R`\x04\x01a\x08n\x91\x90a?eV[\x80\x82\x11\x15a+\xB8W\x82\x82\x82`@Qc\xCA\xA8\x14\xA3`\xE0\x1B\x81R`\x04\x01a\x08n\x93\x92\x91\x90aQCV[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a=\tW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x08nV[_\x80Q` aQy\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``_\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa=S\x91\x90aQgV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a=\x8BW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a=\x90V[``\x91P[P\x91P\x91Pa=\xA0\x85\x83\x83a>\x90V[\x95\x94PPPPPV[4\x15a*SW`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a>\x01WP_\x91P`\x03\x90P\x82a>\x86V[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a>RW=_\x80>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a>}WP_\x92P`\x01\x91P\x82\x90Pa>\x86V[\x92P_\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[``\x82a>\xA5Wa>\xA0\x82a>\xEFV[a>\xE8V[\x81Q\x15\x80\x15a>\xBCWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a>\xE5W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x08nV[P\x80[\x93\x92PPPV[\x80Q\x15a>\xFFW\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x83\x81\x10\x15a?2W\x81\x81\x01Q\x83\x82\x01R` \x01a?\x1AV[PP_\x91\x01RV[_\x81Q\x80\x84Ra?Q\x81` \x86\x01` \x86\x01a?\x18V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R_a>\xE8` \x83\x01\x84a?:V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a)VW_\x80\xFD[\x805a?\x96\x81a?wV[\x91\x90PV[_` \x82\x84\x03\x12\x15a?\xABW_\x80\xFD[\x815a>\xE8\x81a?wV[_\x80\x83`\x1F\x84\x01\x12a?\xC6W_\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a?\xDCW_\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a?\xF6W_\x80\xFD[\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12a@\rW_\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a@#W_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a?\xF6W_\x80\xFD[_\x80_\x80_\x80_\x87\x89\x03`\xE0\x81\x12\x15a@QW_\x80\xFD[\x885`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a@gW_\x80\xFD[a@s\x8C\x83\x8D\x01a?\xB6V[\x90\x9AP\x98P\x88\x91P`\x80`\x1F\x19\x84\x01\x12\x15a@\x8CW_\x80\xFD[` \x8B\x01\x97P`\xA0\x8B\x015\x92P\x80\x83\x11\x15a@\xA5W_\x80\xFD[a@\xB1\x8C\x84\x8D\x01a?\xFDV[\x90\x97P\x95P`\xC0\x8B\x015\x92P\x86\x91P\x80\x83\x11\x15a@\xCCW_\x80\xFD[PPa@\xDA\x8A\x82\x8B\x01a?\xB6V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[_` \x82\x84\x03\x12\x15a@\xFDW_\x80\xFD[P5\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15aA\x15W_\x80\xFD[\x825\x91P` \x83\x015aA'\x81a?wV[\x80\x91PP\x92P\x92\x90PV[_`\x01\x80`\xA0\x1B\x03\x80\x83Q\x16\x84R\x80` \x84\x01Q\x16` \x85\x01RP`@\x82\x01Q`\x80`@\x85\x01RaAf`\x80\x85\x01\x82a?:V[\x90P``\x83\x01Q\x84\x82\x03``\x86\x01Ra=\xA0\x82\x82a?:V[` \x81R_a>\xE8` \x83\x01\x84aA2V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x80`@\x83\x85\x03\x12\x15aA\xB6W_\x80\xFD[\x825aA\xC1\x81a?wV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aA\xDCW_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aA\xEFW_\x80\xFD[\x815\x81\x81\x11\x15aB\x01WaB\x01aA\x91V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15aB)WaB)aA\x91V[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15aBAW_\x80\xFD[\x82` \x86\x01` \x83\x017_` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aB\xA2W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aB}V[P\x90\x96\x95PPPPPPV[_\x80_\x80_``\x86\x88\x03\x12\x15aB\xC2W_\x80\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aB\xDFW_\x80\xFD[aB\xEB\x89\x83\x8A\x01a?\xFDV[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15aC\x03W_\x80\xFD[PaC\x10\x88\x82\x89\x01a?\xFDV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_\x80_\x80_\x80_\x80_`\xA0\x8A\x8C\x03\x12\x15aC9W_\x80\xFD[\x895\x98P` \x8A\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aCVW_\x80\xFD[aCb\x8D\x83\x8E\x01a?\xB6V[\x90\x9AP\x98P`@\x8C\x015\x91P\x80\x82\x11\x15aCzW_\x80\xFD[aC\x86\x8D\x83\x8E\x01a?\xB6V[\x90\x98P\x96P``\x8C\x015\x91P\x80\x82\x11\x15aC\x9EW_\x80\xFD[aC\xAA\x8D\x83\x8E\x01a?\xFDV[\x90\x96P\x94P`\x80\x8C\x015\x91P\x80\x82\x11\x15aC\xC2W_\x80\xFD[PaC\xCF\x8C\x82\x8D\x01a?\xFDV[\x91P\x80\x93PP\x80\x91PP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[_\x80_\x80_\x80_`\x80\x88\x8A\x03\x12\x15aC\xFCW_\x80\xFD[\x875\x96P` \x88\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aD\x19W_\x80\xFD[aD%\x8B\x83\x8C\x01a?\xB6V[\x90\x98P\x96P`@\x8A\x015\x91P\x80\x82\x11\x15aD=W_\x80\xFD[aDI\x8B\x83\x8C\x01a?\xFDV[\x90\x96P\x94P``\x8A\x015\x91P\x80\x82\x11\x15aDaW_\x80\xFD[Pa@\xDA\x8A\x82\x8B\x01a?\xFDV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a?\x96W_\x80\xFD[_\x80_\x80_\x80`\x80\x87\x89\x03\x12\x15aD\x99W_\x80\xFD[\x865\x95P` \x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aD\xB6W_\x80\xFD[aD\xC2\x8A\x83\x8B\x01a?\xFDV[\x90\x97P\x95P`@\x89\x015\x91P\x80\x82\x11\x15aD\xDAW_\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12aD\xEDW_\x80\xFD[\x815\x81\x81\x11\x15aD\xFBW_\x80\xFD[\x8A` ``\x83\x02\x85\x01\x01\x11\x15aE\x0FW_\x80\xFD[` \x83\x01\x95P\x80\x94PPPPaE'``\x88\x01aDnV[\x90P\x92\x95P\x92\x95P\x92\x95V[_\x80`@\x83\x85\x03\x12\x15aEDW_\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[_\x82\x82Q\x80\x85R` \x80\x86\x01\x95P` \x82`\x05\x1B\x84\x01\x01` \x86\x01_[\x84\x81\x10\x15aE\x9EW`\x1F\x19\x86\x84\x03\x01\x89RaE\x8C\x83\x83QaA2V[\x98\x84\x01\x98\x92P\x90\x83\x01\x90`\x01\x01aEpV[P\x90\x97\x96PPPPPPPV[` \x81R_a>\xE8` \x83\x01\x84aESV[_\x85QaE\xCE\x81\x84` \x8A\x01a?\x18V[a\x10;`\xF1\x1B\x90\x83\x01\x90\x81R\x85QaE\xED\x81`\x02\x84\x01` \x8A\x01a?\x18V[\x80\x82\x01\x91PP`\x17`\xF9\x1B\x80`\x02\x83\x01R\x85QaF\x11\x81`\x03\x85\x01` \x8A\x01a?\x18V[`\x03\x92\x01\x91\x82\x01R\x83QaF,\x81`\x04\x84\x01` \x88\x01a?\x18V[\x01`\x04\x01\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15aFJW_\x80\xFD[\x81Qa>\xE8\x81a?wV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\rzWa\rzaFUV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\rzWa\rzaFUV[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aF\xB8W_\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aF\xD6W_\x80\xFD[\x806\x03\x82\x13\x15a?\xF6W_\x80\xFD[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x805`\x03\x81\x90\x0B\x81\x14a?\x96W_\x80\xFD[_\x825`^\x19\x836\x03\x01\x81\x12aG1W_\x80\xFD[\x90\x91\x01\x92\x91PPV[_\x83\x83\x85R` \x80\x86\x01\x95P\x80\x85`\x05\x1B\x83\x01\x01\x84_[\x87\x81\x10\x15aE\x9EW\x84\x83\x03`\x1F\x19\x01\x89RaGl\x82\x88aG\x1DV[``aGx\x82\x83aF\xA3V[\x82\x87RaG\x88\x83\x88\x01\x82\x84aF\xE4V[\x92PPPaG\x98\x86\x83\x01\x83aF\xA3V[\x86\x83\x03\x88\x88\x01RaG\xAA\x83\x82\x84aF\xE4V[\x92PPP`@aG\xBC\x81\x84\x01\x84aF\xA3V[\x93P\x86\x83\x03\x82\x88\x01RaG\xD0\x83\x85\x83aF\xE4V[\x9C\x88\x01\x9C\x96PPP\x92\x85\x01\x92PP`\x01\x01aGQV[`\xE0\x80\x82R\x81\x01\x87\x90R_a\x01\0\x80\x83\x01`\x05\x8A\x90\x1B\x84\x01\x82\x01\x8B\x84[\x8C\x81\x10\x15aIFW\x86\x83\x03`\xFF\x19\x01\x84R\x8156\x8F\x90\x03`\xFE\x19\x01\x81\x12aH(W_\x80\xFD[\x8E\x01aHD\x84aH7\x83a?\x8BV[`\x01`\x01`\xA0\x1B\x03\x16\x90RV[` aHQ\x81\x83\x01a?\x8BV[`\x01`\x01`\xA0\x1B\x03\x16\x81\x86\x01R`@aHl\x83\x82\x01\x84aF\xA3V[\x89\x83\x89\x01RaH~\x8A\x89\x01\x82\x84aF\xE4V[\x92PPP``aH\x90\x81\x85\x01\x85aF\xA3V[\x88\x84\x03\x83\x8A\x01RaH\xA2\x84\x82\x84aF\xE4V[\x93PPPP`\x80aH\xB4\x81\x85\x01aG\x0CV[aH\xC2\x82\x89\x01\x82`\x03\x0B\x90RV[PP`\xA0aH\xD2\x81\x85\x01\x85aF\xA3V[\x88\x84\x03\x83\x8A\x01RaH\xE4\x84\x82\x84aF\xE4V[\x93PPPP`\xC0aH\xF7\x81\x85\x01\x85aF\xA3V[\x88\x84\x03\x83\x8A\x01RaI\t\x84\x82\x84aF\xE4V[\x93PPPPaI\x1B`\xE0\x84\x01\x84aF\xA3V[\x93P\x86\x82\x03`\xE0\x88\x01RaI0\x82\x85\x83aF\xE4V[\x97\x83\x01\x97\x96PPP\x92\x90\x92\x01\x91P`\x01\x01aH\x03V[PPaIv` \x86\x01\x8B\x805\x82R` \x81\x015` \x83\x01R`@\x81\x015`@\x83\x01R``\x81\x015``\x83\x01RPPV[\x84\x81\x03`\xA0\x86\x01RaI\x89\x81\x89\x8BaF\xE4V[\x92PPP\x82\x81\x03`\xC0\x84\x01RaI\xA0\x81\x85\x87aG:V[\x9A\x99PPPPPPPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80aI\xC2W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aI\xE0WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[_`\x01\x82\x01aJ\x06WaJ\x06aFUV[P`\x01\x01\x90V[`@\x81R_aJ `@\x83\x01\x86\x88aF\xE4V[\x82\x81\x03` \x84\x01RaJ3\x81\x85\x87aF\xE4V[\x97\x96PPPPPPPV[\x85\x81R``` \x82\x01R_aJW``\x83\x01\x86\x88aF\xE4V[\x82\x81\x03`@\x84\x01RaJj\x81\x85\x87aF\xE4V[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x805`\x04\x81\x10a?\x96W_\x80\xFD[`\x04\x81\x10aJ\xB4WcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90RV[_\x83\x83\x85R` \x80\x86\x01\x95P\x80\x85`\x05\x1B\x83\x01\x01\x84_[\x87\x81\x10\x15aE\x9EW\x84\x83\x03`\x1F\x19\x01\x89RaJ\xEA\x82\x88aG\x1DV[``\x815\x85R\x85\x82\x015\x86\x86\x01R`@aK\x06\x81\x84\x01\x84aF\xA3V[\x93P\x82\x82\x88\x01RaK\x1A\x83\x88\x01\x85\x83aF\xE4V[\x9C\x88\x01\x9C\x96PPP\x92\x85\x01\x92PP`\x01\x01aJ\xCFV[_\x82\x82Q\x80\x85R` \x80\x86\x01\x95P` \x82`\x05\x1B\x84\x01\x01` \x86\x01_[\x84\x81\x10\x15aE\x9EW`\x1F\x19\x86\x84\x03\x01\x89RaKi\x83\x83Qa?:V[\x98\x84\x01\x98\x92P\x90\x83\x01\x90`\x01\x01aKMV[``\x80\x82R\x81\x81\x01\x86\x90R_\x90`\x80\x80\x84\x01`\x05\x89\x90\x1B\x85\x01\x82\x01\x8A\x85[\x8B\x81\x10\x15aL\xB1W`\x7F\x19\x80\x89\x85\x03\x01\x85RaK\xB5\x83\x8FaG\x1DV[\x87\x85\x01\x815\x86R` \x80\x83\x015\x81\x88\x01R`@\x80\x84\x015`\x1E\x19\x856\x03\x01\x81\x12aK\xDDW_\x80\xFD[\x90\x93\x01\x92\x835\x82\x85\x01`\x01`\x01`@\x1B\x03\x82\x11\x15aK\xF9W_\x80\xFD[\x81`\x05\x1B\x806\x03\x82\x13\x15aL\x0BW_\x80\xFD[\x8A\x84\x01\x8E\x90R\x94\x82\x90R\x93\x89\x01\x8B\x01\x93_\x8C\x8B\x016\x88\x90\x03`^\x19\x01[\x84\x83\x10\x15aL\x95W\x89\x8D\x89\x03\x01\x82R\x835\x81\x81\x12aLDW_\x80\xFD[\x89\x01\x87\x81\x01aL[\x8AaLV\x83aJ\x8AV[aJ\x98V[aLg\x88\x83\x01\x82aF\xA3V[\x92P\x90P\x87\x89\x8B\x01RaL}\x88\x8B\x01\x83\x83aF\xE4V[\x99PPP\x92\x86\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x86\x01\x90aL(V[PPP\x99\x83\x01\x99P\x92\x97PP\x94\x90\x94\x01\x93PPP`\x01\x01aK\x99V[PP\x85\x81\x03` \x87\x01RaL\xC6\x81\x89\x8BaJ\xB8V[\x93PPPP\x82\x81\x03`@\x84\x01RaJj\x81\x85aK0V[_`\x01`\x01`\xFB\x1B\x03\x83\x11\x15aL\xF1W_\x80\xFD[\x82`\x05\x1B\x80\x85\x847\x91\x90\x91\x01\x93\x92PPPV[``\x80\x82R\x81\x01\x86\x90R_`\x01`\x01`\xFB\x1B\x03\x87\x11\x15aM\"W_\x80\xFD[\x86`\x05\x1B\x80\x89`\x80\x85\x017\x82\x01\x82\x81\x03`\x80\x90\x81\x01` \x85\x01RaMI\x90\x82\x01\x87\x89aF\xE4V[\x90P\x82\x81\x03`@\x84\x01RaM^\x81\x85\x87aF\xE4V[\x99\x98PPPPPPPPPV[`\xA0\x81R_aM}`\xA0\x83\x01\x85aESV[\x90Pa>\xE8` \x83\x01\x84\x805\x82R` \x81\x015` \x83\x01R`@\x81\x015`@\x83\x01R``\x81\x015``\x83\x01RPPV[_` \x82\x84\x03\x12\x15aM\xBDW_\x80\xFD[a>\xE8\x82aDnV[_``\x80\x83RaM\xDA``\x84\x01\x88\x8AaF\xE4V[\x83\x81\x03` \x85\x81\x01\x91\x90\x91R\x86\x82R\x87\x91\x81\x01_[\x88\x81\x10\x15aNAW`\x01`\x01`@\x1B\x03\x80aN\t\x86aDnV[\x16\x83R\x80aN\x18\x85\x87\x01aDnV[\x16\x84\x84\x01R`@\x81aN+\x82\x88\x01aDnV[\x16\x90\x84\x01RP\x92\x84\x01\x92\x90\x84\x01\x90`\x01\x01aM\xEFV[P\x80\x94PPPPP`\x01`\x01`@\x1B\x03\x83\x16`@\x83\x01R\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15aNrW_\x80\xFD[PQ\x91\x90PV[_\x825`\xFE\x19\x836\x03\x01\x81\x12aN\x8DW_\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aN\xACW_\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aN\xC5W_\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a?\xF6W_\x80\xFD[`\x1F\x82\x11\x15a+\xB8W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15aN\xFEWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15aO\x1DW_\x81U`\x01\x01aO\nV[PPPPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aO=WaO=aA\x91V[aOQ\x81aOK\x84TaI\xAEV[\x84aN\xD9V[` \x80`\x1F\x83\x11`\x01\x81\x14aO\x84W_\x84\x15aOmWP\x85\x83\x01Q[_\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua)%V[_\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15aO\xB2W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01aO\x93V[P\x85\x82\x10\x15aO\xCFW\x87\x85\x01Q_\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[_\x825`^\x19\x836\x03\x01\x81\x12aN\x8DW_\x80\xFD[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aP\x08W_\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aP!W_\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a?\xF6W_\x80\xFD[_\x825`>\x19\x836\x03\x01\x81\x12aN\x8DW_\x80\xFD[_` \x82\x84\x03\x12\x15aP\\W_\x80\xFD[a>\xE8\x82aJ\x8AV[\x83\x81R``\x81\x01aPy` \x83\x01\x85aJ\x98V[\x82`@\x83\x01R\x94\x93PPPPV[\x81Q_\x90\x82\x90` \x80\x86\x01\x84[\x83\x81\x10\x15aP\xB0W\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01aP\x94V[P\x92\x96\x95PPPPPPV[\x84\x81R\x83` \x82\x01R```@\x82\x01R_aP\xDB``\x83\x01\x84\x86aF\xE4V[\x96\x95PPPPPPV[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15aQ\x14W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aP\xF8V[P\x94\x95\x94PPPPPV[`@\x81R_aQ1`@\x83\x01\x85aP\xE5V[\x82\x81\x03` \x84\x01Ra=\xA0\x81\x85aP\xE5V[``\x81R_aQU``\x83\x01\x86a?:V[` \x83\x01\x94\x90\x94RP`@\x01R\x91\x90PV[_\x82QaN\x8D\x81\x84` \x87\x01a?\x18V\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct ChainUpgradeWindow { uint64 chainId; uint64 startBlock; uint64 endBlock; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ChainUpgradeWindow {
        #[allow(missing_docs)]
        pub chainId: u64,
        #[allow(missing_docs)]
        pub startBlock: u64,
        #[allow(missing_docs)]
        pub endBlock: u64,
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
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u64, u64, u64);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ChainUpgradeWindow> for UnderlyingRustTuple<'_> {
            fn from(value: ChainUpgradeWindow) -> Self {
                (value.chainId, value.startBlock, value.endBlock)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ChainUpgradeWindow {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    chainId: tuple.0,
                    startBlock: tuple.1,
                    endBlock: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ChainUpgradeWindow {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ChainUpgradeWindow {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.chainId),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.startBlock),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.endBlock),
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
        impl alloy_sol_types::SolType for ChainUpgradeWindow {
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
        impl alloy_sol_types::SolStruct for ChainUpgradeWindow {
            const NAME: &'static str = "ChainUpgradeWindow";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ChainUpgradeWindow(uint64 chainId,uint64 startBlock,uint64 endBlock)",
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
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.chainId)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.startBlock)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.endBlock)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ChainUpgradeWindow {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.chainId,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.startBlock,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.endBlock,
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
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.chainId,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.startBlock,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.endBlock,
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
struct KmsNode { address txSenderAddress; address signerAddress; string ipAddress; string storageUrl; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsNode {
        #[allow(missing_docs)]
        pub txSenderAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub signerAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub ipAddress: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub storageUrl: alloy::sol_types::private::String,
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
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::String,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::String,
            alloy::sol_types::private::String,
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
        impl ::core::convert::From<KmsNode> for UnderlyingRustTuple<'_> {
            fn from(value: KmsNode) -> Self {
                (
                    value.txSenderAddress,
                    value.signerAddress,
                    value.ipAddress,
                    value.storageUrl,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KmsNode {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    txSenderAddress: tuple.0,
                    signerAddress: tuple.1,
                    ipAddress: tuple.2,
                    storageUrl: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for KmsNode {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for KmsNode {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.txSenderAddress,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.signerAddress,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.ipAddress,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.storageUrl,
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
        impl alloy_sol_types::SolType for KmsNode {
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
        impl alloy_sol_types::SolStruct for KmsNode {
            const NAME: &'static str = "KmsNode";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "KmsNode(address txSenderAddress,address signerAddress,string ipAddress,string storageUrl)",
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
                            &self.txSenderAddress,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signerAddress,
                        )
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.ipAddress,
                        )
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.storageUrl,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for KmsNode {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.txSenderAddress,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signerAddress,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.ipAddress,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.storageUrl,
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
                    &rust.txSenderAddress,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signerAddress,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ipAddress,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.storageUrl,
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
struct KmsNodeParams { address txSenderAddress; address signerAddress; string ipAddress; string storageUrl; int32 partyId; string mpcIdentity; bytes caCert; string storagePrefix; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsNodeParams {
        #[allow(missing_docs)]
        pub txSenderAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub signerAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub ipAddress: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub storageUrl: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub partyId: i32,
        #[allow(missing_docs)]
        pub mpcIdentity: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub caCert: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub storagePrefix: alloy::sol_types::private::String,
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
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::Int<32>,
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::String,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::String,
            alloy::sol_types::private::String,
            i32,
            alloy::sol_types::private::String,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::String,
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
        impl ::core::convert::From<KmsNodeParams> for UnderlyingRustTuple<'_> {
            fn from(value: KmsNodeParams) -> Self {
                (
                    value.txSenderAddress,
                    value.signerAddress,
                    value.ipAddress,
                    value.storageUrl,
                    value.partyId,
                    value.mpcIdentity,
                    value.caCert,
                    value.storagePrefix,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KmsNodeParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    txSenderAddress: tuple.0,
                    signerAddress: tuple.1,
                    ipAddress: tuple.2,
                    storageUrl: tuple.3,
                    partyId: tuple.4,
                    mpcIdentity: tuple.5,
                    caCert: tuple.6,
                    storagePrefix: tuple.7,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for KmsNodeParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for KmsNodeParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.txSenderAddress,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.signerAddress,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.ipAddress,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.storageUrl,
                    ),
                    <alloy::sol_types::sol_data::Int<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.partyId),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.mpcIdentity,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.caCert,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.storagePrefix,
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
        impl alloy_sol_types::SolType for KmsNodeParams {
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
        impl alloy_sol_types::SolStruct for KmsNodeParams {
            const NAME: &'static str = "KmsNodeParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "KmsNodeParams(address txSenderAddress,address signerAddress,string ipAddress,string storageUrl,int32 partyId,string mpcIdentity,bytes caCert,string storagePrefix)",
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
                            &self.txSenderAddress,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signerAddress,
                        )
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.ipAddress,
                        )
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.storageUrl,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.partyId)
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.mpcIdentity,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.caCert,
                        )
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.storagePrefix,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for KmsNodeParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.txSenderAddress,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signerAddress,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.ipAddress,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.storageUrl,
                    )
                    + <alloy::sol_types::sol_data::Int<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.partyId,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.mpcIdentity,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.caCert,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.storagePrefix,
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
                    &rust.txSenderAddress,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signerAddress,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ipAddress,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.storageUrl,
                    out,
                );
                <alloy::sol_types::sol_data::Int<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.partyId,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.mpcIdentity,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.caCert,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.storagePrefix,
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
struct KmsThresholds { uint256 publicDecryption; uint256 userDecryption; uint256 kmsGen; uint256 mpc; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsThresholds {
        #[allow(missing_docs)]
        pub publicDecryption: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub userDecryption: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub kmsGen: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub mpc: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<KmsThresholds> for UnderlyingRustTuple<'_> {
            fn from(value: KmsThresholds) -> Self {
                (value.publicDecryption, value.userDecryption, value.kmsGen, value.mpc)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KmsThresholds {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    publicDecryption: tuple.0,
                    userDecryption: tuple.1,
                    kmsGen: tuple.2,
                    mpc: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for KmsThresholds {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for KmsThresholds {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.publicDecryption),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.userDecryption),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsGen),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.mpc),
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
        impl alloy_sol_types::SolType for KmsThresholds {
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
        impl alloy_sol_types::SolStruct for KmsThresholds {
            const NAME: &'static str = "KmsThresholds";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "KmsThresholds(uint256 publicDecryption,uint256 userDecryption,uint256 kmsGen,uint256 mpc)",
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
                            &self.publicDecryption,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.userDecryption,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.kmsGen)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.mpc)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for KmsThresholds {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.publicDecryption,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.userDecryption,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.kmsGen,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.mpc)
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
                    &rust.publicDecryption,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.userDecryption,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.kmsGen,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.mpc, out);
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
struct PcrValues { bytes pcr0; bytes pcr1; bytes pcr2; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PcrValues {
        #[allow(missing_docs)]
        pub pcr0: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub pcr1: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub pcr2: alloy::sol_types::private::Bytes,
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
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<PcrValues> for UnderlyingRustTuple<'_> {
            fn from(value: PcrValues) -> Self {
                (value.pcr0, value.pcr1, value.pcr2)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PcrValues {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    pcr0: tuple.0,
                    pcr1: tuple.1,
                    pcr2: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for PcrValues {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for PcrValues {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.pcr0,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.pcr1,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.pcr2,
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
        impl alloy_sol_types::SolType for PcrValues {
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
        impl alloy_sol_types::SolStruct for PcrValues {
            const NAME: &'static str = "PcrValues";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "PcrValues(bytes pcr0,bytes pcr1,bytes pcr2)",
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.pcr0,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.pcr1,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.pcr2,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for PcrValues {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.pcr0,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.pcr1,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.pcr2,
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
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.pcr0,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.pcr1,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.pcr2,
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
    /**Custom error with signature `DuplicateChainId(uint64)` and selector `0x6c67e470`.
```solidity
error DuplicateChainId(uint64 chainId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DuplicateChainId {
        #[allow(missing_docs)]
        pub chainId: u64,
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
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u64,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<DuplicateChainId> for UnderlyingRustTuple<'_> {
            fn from(value: DuplicateChainId) -> Self {
                (value.chainId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DuplicateChainId {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { chainId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DuplicateChainId {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DuplicateChainId(uint64)";
            const SELECTOR: [u8; 4] = [108u8, 103u8, 228u8, 112u8];
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
                        64,
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
    /**Custom error with signature `EmptyChainUpgradeWindows()` and selector `0xbe505044`.
```solidity
error EmptyChainUpgradeWindows();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyChainUpgradeWindows;
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
        impl ::core::convert::From<EmptyChainUpgradeWindows>
        for UnderlyingRustTuple<'_> {
            fn from(value: EmptyChainUpgradeWindows) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for EmptyChainUpgradeWindows {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyChainUpgradeWindows {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyChainUpgradeWindows()";
            const SELECTOR: [u8; 4] = [190u8, 80u8, 80u8, 68u8];
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
    /**Custom error with signature `EmptyEpochActivationResults(uint256)` and selector `0x621569e4`.
```solidity
error EmptyEpochActivationResults(uint256 epochId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyEpochActivationResults {
        #[allow(missing_docs)]
        pub epochId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<EmptyEpochActivationResults>
        for UnderlyingRustTuple<'_> {
            fn from(value: EmptyEpochActivationResults) -> Self {
                (value.epochId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for EmptyEpochActivationResults {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { epochId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyEpochActivationResults {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyEpochActivationResults(uint256)";
            const SELECTOR: [u8; 4] = [98u8, 21u8, 105u8, 228u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.epochId),
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
    /**Custom error with signature `EmptyKmsNodes()` and selector `0x068c8d40`.
```solidity
error EmptyKmsNodes();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyKmsNodes;
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
        impl ::core::convert::From<EmptyKmsNodes> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyKmsNodes) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyKmsNodes {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyKmsNodes {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyKmsNodes()";
            const SELECTOR: [u8; 4] = [6u8, 140u8, 141u8, 64u8];
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
    /**Custom error with signature `EmptySoftwareVersion()` and selector `0xb5489147`.
```solidity
error EmptySoftwareVersion();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptySoftwareVersion;
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
        impl ::core::convert::From<EmptySoftwareVersion> for UnderlyingRustTuple<'_> {
            fn from(value: EmptySoftwareVersion) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptySoftwareVersion {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptySoftwareVersion {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptySoftwareVersion()";
            const SELECTOR: [u8; 4] = [181u8, 72u8, 145u8, 71u8];
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
    /**Custom error with signature `EpochActivationAlreadyConfirmed(address,uint256)` and selector `0x49417636`.
```solidity
error EpochActivationAlreadyConfirmed(address signer, uint256 epochId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EpochActivationAlreadyConfirmed {
        #[allow(missing_docs)]
        pub signer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub epochId: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<EpochActivationAlreadyConfirmed>
        for UnderlyingRustTuple<'_> {
            fn from(value: EpochActivationAlreadyConfirmed) -> Self {
                (value.signer, value.epochId)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for EpochActivationAlreadyConfirmed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signer: tuple.0,
                    epochId: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EpochActivationAlreadyConfirmed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EpochActivationAlreadyConfirmed(address,uint256)";
            const SELECTOR: [u8; 4] = [73u8, 65u8, 118u8, 54u8];
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
                        &self.signer,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.epochId),
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
    /**Custom error with signature `EpochActivationUnauthorized(address,uint256)` and selector `0xa3f4afeb`.
```solidity
error EpochActivationUnauthorized(address signer, uint256 epochId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EpochActivationUnauthorized {
        #[allow(missing_docs)]
        pub signer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub epochId: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<EpochActivationUnauthorized>
        for UnderlyingRustTuple<'_> {
            fn from(value: EpochActivationUnauthorized) -> Self {
                (value.signer, value.epochId)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for EpochActivationUnauthorized {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signer: tuple.0,
                    epochId: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EpochActivationUnauthorized {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EpochActivationUnauthorized(address,uint256)";
            const SELECTOR: [u8; 4] = [163u8, 244u8, 175u8, 235u8];
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
                        &self.signer,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.epochId),
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
    /**Custom error with signature `InvalidBlockWindow(uint64,uint64,uint64)` and selector `0xf219dc0e`.
```solidity
error InvalidBlockWindow(uint64 chainId, uint64 startBlock, uint64 endBlock);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidBlockWindow {
        #[allow(missing_docs)]
        pub chainId: u64,
        #[allow(missing_docs)]
        pub startBlock: u64,
        #[allow(missing_docs)]
        pub endBlock: u64,
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
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u64, u64, u64);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidBlockWindow> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidBlockWindow) -> Self {
                (value.chainId, value.startBlock, value.endBlock)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidBlockWindow {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    chainId: tuple.0,
                    startBlock: tuple.1,
                    endBlock: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidBlockWindow {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidBlockWindow(uint64,uint64,uint64)";
            const SELECTOR: [u8; 4] = [242u8, 25u8, 220u8, 14u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.chainId),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.startBlock),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.endBlock),
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
    /**Custom error with signature `InvalidHighThreshold(string,uint256,uint256)` and selector `0xcaa814a3`.
```solidity
error InvalidHighThreshold(string thresholdName, uint256 threshold, uint256 nodeCount);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidHighThreshold {
        #[allow(missing_docs)]
        pub thresholdName: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub threshold: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub nodeCount: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::String,
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
        impl ::core::convert::From<InvalidHighThreshold> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidHighThreshold) -> Self {
                (value.thresholdName, value.threshold, value.nodeCount)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidHighThreshold {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    thresholdName: tuple.0,
                    threshold: tuple.1,
                    nodeCount: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidHighThreshold {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidHighThreshold(string,uint256,uint256)";
            const SELECTOR: [u8; 4] = [202u8, 168u8, 20u8, 163u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.thresholdName,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.threshold),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nodeCount),
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
    /**Custom error with signature `InvalidKmsContext(uint256)` and selector `0x77ddbe81`.
```solidity
error InvalidKmsContext(uint256 kmsContextId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidKmsContext {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<InvalidKmsContext> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidKmsContext) -> Self {
                (value.kmsContextId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidKmsContext {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { kmsContextId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidKmsContext {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidKmsContext(uint256)";
            const SELECTOR: [u8; 4] = [119u8, 221u8, 190u8, 129u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsContextId),
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
    /**Custom error with signature `InvalidKmsEpoch(uint256)` and selector `0x7995bbcf`.
```solidity
error InvalidKmsEpoch(uint256 epochId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidKmsEpoch {
        #[allow(missing_docs)]
        pub epochId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<InvalidKmsEpoch> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidKmsEpoch) -> Self {
                (value.epochId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidKmsEpoch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { epochId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidKmsEpoch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidKmsEpoch(uint256)";
            const SELECTOR: [u8; 4] = [121u8, 149u8, 187u8, 207u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.epochId),
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
    /**Custom error with signature `InvalidNullThreshold(string)` and selector `0x36bfb60e`.
```solidity
error InvalidNullThreshold(string thresholdName);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidNullThreshold {
        #[allow(missing_docs)]
        pub thresholdName: alloy::sol_types::private::String,
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
        impl ::core::convert::From<InvalidNullThreshold> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidNullThreshold) -> Self {
                (value.thresholdName,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidNullThreshold {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { thresholdName: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidNullThreshold {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidNullThreshold(string)";
            const SELECTOR: [u8; 4] = [54u8, 191u8, 182u8, 14u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.thresholdName,
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
    /**Custom error with signature `InvalidProposalId()` and selector `0x0992f7ad`.
```solidity
error InvalidProposalId();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidProposalId;
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
        impl ::core::convert::From<InvalidProposalId> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidProposalId) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidProposalId {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidProposalId {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidProposalId()";
            const SELECTOR: [u8; 4] = [9u8, 146u8, 247u8, 173u8];
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
    /**Custom error with signature `KmsContextCreationAlreadyConfirmed(address,uint256)` and selector `0x62585cc8`.
```solidity
error KmsContextCreationAlreadyConfirmed(address signer, uint256 kmsContextId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsContextCreationAlreadyConfirmed {
        #[allow(missing_docs)]
        pub signer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<KmsContextCreationAlreadyConfirmed>
        for UnderlyingRustTuple<'_> {
            fn from(value: KmsContextCreationAlreadyConfirmed) -> Self {
                (value.signer, value.kmsContextId)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for KmsContextCreationAlreadyConfirmed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signer: tuple.0,
                    kmsContextId: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KmsContextCreationAlreadyConfirmed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KmsContextCreationAlreadyConfirmed(address,uint256)";
            const SELECTOR: [u8; 4] = [98u8, 88u8, 92u8, 200u8];
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
                        &self.signer,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsContextId),
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
    /**Custom error with signature `KmsContextCreationUnauthorized(address,uint256)` and selector `0xb81df8e8`.
```solidity
error KmsContextCreationUnauthorized(address signer, uint256 kmsContextId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsContextCreationUnauthorized {
        #[allow(missing_docs)]
        pub signer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<KmsContextCreationUnauthorized>
        for UnderlyingRustTuple<'_> {
            fn from(value: KmsContextCreationUnauthorized) -> Self {
                (value.signer, value.kmsContextId)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for KmsContextCreationUnauthorized {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signer: tuple.0,
                    kmsContextId: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KmsContextCreationUnauthorized {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KmsContextCreationUnauthorized(address,uint256)";
            const SELECTOR: [u8; 4] = [184u8, 29u8, 248u8, 232u8];
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
                        &self.signer,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsContextId),
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
    /**Custom error with signature `KmsContextNotDestroyed(uint256)` and selector `0x2b73d9b7`.
```solidity
error KmsContextNotDestroyed(uint256 kmsContextId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsContextNotDestroyed {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<KmsContextNotDestroyed> for UnderlyingRustTuple<'_> {
            fn from(value: KmsContextNotDestroyed) -> Self {
                (value.kmsContextId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KmsContextNotDestroyed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { kmsContextId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KmsContextNotDestroyed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KmsContextNotDestroyed(uint256)";
            const SELECTOR: [u8; 4] = [43u8, 115u8, 217u8, 183u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsContextId),
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
    /**Custom error with signature `KmsContextNotPending(uint256)` and selector `0x3586efa1`.
```solidity
error KmsContextNotPending(uint256 kmsContextId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsContextNotPending {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<KmsContextNotPending> for UnderlyingRustTuple<'_> {
            fn from(value: KmsContextNotPending) -> Self {
                (value.kmsContextId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KmsContextNotPending {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { kmsContextId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KmsContextNotPending {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KmsContextNotPending(uint256)";
            const SELECTOR: [u8; 4] = [53u8, 134u8, 239u8, 161u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsContextId),
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
    /**Custom error with signature `KmsDestructionAlreadyConfirmed(address,uint256)` and selector `0x9d2799e9`.
```solidity
error KmsDestructionAlreadyConfirmed(address signer, uint256 destroyedId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsDestructionAlreadyConfirmed {
        #[allow(missing_docs)]
        pub signer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub destroyedId: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<KmsDestructionAlreadyConfirmed>
        for UnderlyingRustTuple<'_> {
            fn from(value: KmsDestructionAlreadyConfirmed) -> Self {
                (value.signer, value.destroyedId)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for KmsDestructionAlreadyConfirmed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signer: tuple.0,
                    destroyedId: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KmsDestructionAlreadyConfirmed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KmsDestructionAlreadyConfirmed(address,uint256)";
            const SELECTOR: [u8; 4] = [157u8, 39u8, 153u8, 233u8];
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
                        &self.signer,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.destroyedId),
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
    /**Custom error with signature `KmsDestructionUnauthorized(address,uint256)` and selector `0xd628b248`.
```solidity
error KmsDestructionUnauthorized(address signer, uint256 destroyedId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsDestructionUnauthorized {
        #[allow(missing_docs)]
        pub signer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub destroyedId: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<KmsDestructionUnauthorized>
        for UnderlyingRustTuple<'_> {
            fn from(value: KmsDestructionUnauthorized) -> Self {
                (value.signer, value.destroyedId)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for KmsDestructionUnauthorized {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signer: tuple.0,
                    destroyedId: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KmsDestructionUnauthorized {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KmsDestructionUnauthorized(address,uint256)";
            const SELECTOR: [u8; 4] = [214u8, 40u8, 178u8, 72u8];
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
                        &self.signer,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.destroyedId),
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
    /**Custom error with signature `KmsEpochNotDestroyed(uint256)` and selector `0x03d8b4d0`.
```solidity
error KmsEpochNotDestroyed(uint256 epochId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsEpochNotDestroyed {
        #[allow(missing_docs)]
        pub epochId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<KmsEpochNotDestroyed> for UnderlyingRustTuple<'_> {
            fn from(value: KmsEpochNotDestroyed) -> Self {
                (value.epochId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KmsEpochNotDestroyed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { epochId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KmsEpochNotDestroyed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KmsEpochNotDestroyed(uint256)";
            const SELECTOR: [u8; 4] = [3u8, 216u8, 180u8, 208u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.epochId),
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
    /**Custom error with signature `KmsLifecycleOperationInFlight(uint256,uint256)` and selector `0x078020be`.
```solidity
error KmsLifecycleOperationInFlight(uint256 kmsContextId, uint256 epochId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsLifecycleOperationInFlight {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub epochId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<KmsLifecycleOperationInFlight>
        for UnderlyingRustTuple<'_> {
            fn from(value: KmsLifecycleOperationInFlight) -> Self {
                (value.kmsContextId, value.epochId)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for KmsLifecycleOperationInFlight {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    kmsContextId: tuple.0,
                    epochId: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KmsLifecycleOperationInFlight {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KmsLifecycleOperationInFlight(uint256,uint256)";
            const SELECTOR: [u8; 4] = [7u8, 128u8, 32u8, 190u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsContextId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.epochId),
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
    /**Custom error with signature `KmsNodeNullSigner()` and selector `0x2deccf4d`.
```solidity
error KmsNodeNullSigner();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsNodeNullSigner;
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
        impl ::core::convert::From<KmsNodeNullSigner> for UnderlyingRustTuple<'_> {
            fn from(value: KmsNodeNullSigner) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KmsNodeNullSigner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KmsNodeNullSigner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KmsNodeNullSigner()";
            const SELECTOR: [u8; 4] = [45u8, 236u8, 207u8, 77u8];
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
    /**Custom error with signature `KmsNodeNullTxSender()` and selector `0x8466804a`.
```solidity
error KmsNodeNullTxSender();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsNodeNullTxSender;
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
        impl ::core::convert::From<KmsNodeNullTxSender> for UnderlyingRustTuple<'_> {
            fn from(value: KmsNodeNullTxSender) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KmsNodeNullTxSender {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KmsNodeNullTxSender {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KmsNodeNullTxSender()";
            const SELECTOR: [u8; 4] = [132u8, 102u8, 128u8, 74u8];
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
    /**Custom error with signature `KmsSignerAlreadyRegistered(address)` and selector `0xf51af6bb`.
```solidity
error KmsSignerAlreadyRegistered(address signer);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsSignerAlreadyRegistered {
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
        impl ::core::convert::From<KmsSignerAlreadyRegistered>
        for UnderlyingRustTuple<'_> {
            fn from(value: KmsSignerAlreadyRegistered) -> Self {
                (value.signer,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for KmsSignerAlreadyRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { signer: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KmsSignerAlreadyRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KmsSignerAlreadyRegistered(address)";
            const SELECTOR: [u8; 4] = [245u8, 26u8, 246u8, 187u8];
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
    /**Custom error with signature `KmsSignerSetExceedsProofFormatLimit(uint256,uint256)` and selector `0x16a72778`.
```solidity
error KmsSignerSetExceedsProofFormatLimit(uint256 signerCount, uint256 maxAllowed);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsSignerSetExceedsProofFormatLimit {
        #[allow(missing_docs)]
        pub signerCount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub maxAllowed: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<KmsSignerSetExceedsProofFormatLimit>
        for UnderlyingRustTuple<'_> {
            fn from(value: KmsSignerSetExceedsProofFormatLimit) -> Self {
                (value.signerCount, value.maxAllowed)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for KmsSignerSetExceedsProofFormatLimit {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signerCount: tuple.0,
                    maxAllowed: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KmsSignerSetExceedsProofFormatLimit {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KmsSignerSetExceedsProofFormatLimit(uint256,uint256)";
            const SELECTOR: [u8; 4] = [22u8, 167u8, 39u8, 120u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.signerCount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxAllowed),
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
    /**Custom error with signature `KmsTxSenderAlreadyRegistered(address)` and selector `0xd18c4ff0`.
```solidity
error KmsTxSenderAlreadyRegistered(address txSender);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsTxSenderAlreadyRegistered {
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
        impl ::core::convert::From<KmsTxSenderAlreadyRegistered>
        for UnderlyingRustTuple<'_> {
            fn from(value: KmsTxSenderAlreadyRegistered) -> Self {
                (value.txSender,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for KmsTxSenderAlreadyRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { txSender: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KmsTxSenderAlreadyRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KmsTxSenderAlreadyRegistered(address)";
            const SELECTOR: [u8; 4] = [209u8, 140u8, 79u8, 240u8];
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
    /**Custom error with signature `LatestActiveKmsContextCannotBeDestroyed(uint256)` and selector `0x187aeaa8`.
```solidity
error LatestActiveKmsContextCannotBeDestroyed(uint256 kmsContextId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LatestActiveKmsContextCannotBeDestroyed {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<LatestActiveKmsContextCannotBeDestroyed>
        for UnderlyingRustTuple<'_> {
            fn from(value: LatestActiveKmsContextCannotBeDestroyed) -> Self {
                (value.kmsContextId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for LatestActiveKmsContextCannotBeDestroyed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { kmsContextId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LatestActiveKmsContextCannotBeDestroyed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LatestActiveKmsContextCannotBeDestroyed(uint256)";
            const SELECTOR: [u8; 4] = [24u8, 122u8, 234u8, 168u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsContextId),
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
    /**Custom error with signature `LatestActiveKmsEpochCannotBeDestroyed(uint256)` and selector `0xf0e781c1`.
```solidity
error LatestActiveKmsEpochCannotBeDestroyed(uint256 epochId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LatestActiveKmsEpochCannotBeDestroyed {
        #[allow(missing_docs)]
        pub epochId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<LatestActiveKmsEpochCannotBeDestroyed>
        for UnderlyingRustTuple<'_> {
            fn from(value: LatestActiveKmsEpochCannotBeDestroyed) -> Self {
                (value.epochId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for LatestActiveKmsEpochCannotBeDestroyed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { epochId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LatestActiveKmsEpochCannotBeDestroyed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LatestActiveKmsEpochCannotBeDestroyed(uint256)";
            const SELECTOR: [u8; 4] = [240u8, 231u8, 129u8, 193u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.epochId),
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
    /**Custom error with signature `NonIncreasingKmsContextId(uint256,uint256)` and selector `0xefd55f67`.
```solidity
error NonIncreasingKmsContextId(uint256 contextId, uint256 latestActiveKmsContextId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NonIncreasingKmsContextId {
        #[allow(missing_docs)]
        pub contextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub latestActiveKmsContextId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<NonIncreasingKmsContextId>
        for UnderlyingRustTuple<'_> {
            fn from(value: NonIncreasingKmsContextId) -> Self {
                (value.contextId, value.latestActiveKmsContextId)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for NonIncreasingKmsContextId {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    contextId: tuple.0,
                    latestActiveKmsContextId: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NonIncreasingKmsContextId {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NonIncreasingKmsContextId(uint256,uint256)";
            const SELECTOR: [u8; 4] = [239u8, 213u8, 95u8, 103u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.contextId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.latestActiveKmsContextId,
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
    /**Custom error with signature `ZeroChainId()` and selector `0xc84885d4`.
```solidity
error ZeroChainId();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroChainId;
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
        impl ::core::convert::From<ZeroChainId> for UnderlyingRustTuple<'_> {
            fn from(value: ZeroChainId) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ZeroChainId {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroChainId {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroChainId()";
            const SELECTOR: [u8; 4] = [200u8, 72u8, 133u8, 212u8];
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
    /**Custom error with signature `ZeroGwStartBlock()` and selector `0x17d3e948`.
```solidity
error ZeroGwStartBlock();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroGwStartBlock;
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
        impl ::core::convert::From<ZeroGwStartBlock> for UnderlyingRustTuple<'_> {
            fn from(value: ZeroGwStartBlock) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ZeroGwStartBlock {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroGwStartBlock {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroGwStartBlock()";
            const SELECTOR: [u8; 4] = [23u8, 211u8, 233u8, 72u8];
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
    #[derive()]
    /**Event with signature `ActivateEpoch(uint256,uint256,(uint256,uint256,(uint8,bytes)[])[],(uint256,uint256,bytes)[],string[])` and selector `0x6c6eb35881f805f284be59b840b4ea990f96b13dd9e7192c4581e1bc81c0af98`.
```solidity
event ActivateEpoch(uint256 indexed kmsContextId, uint256 indexed epochId, IProtocolConfig.EpochKeyResult[] keys, IProtocolConfig.EpochCrsResult[] crsList, string[] kmsNodeStorageUrls);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ActivateEpoch {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub epochId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub keys: alloy::sol_types::private::Vec<
            <IProtocolConfig::EpochKeyResult as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub crsList: alloy::sol_types::private::Vec<
            <IProtocolConfig::EpochCrsResult as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub kmsNodeStorageUrls: alloy::sol_types::private::Vec<
            alloy::sol_types::private::String,
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
        impl alloy_sol_types::SolEvent for ActivateEpoch {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<IProtocolConfig::EpochKeyResult>,
                alloy::sol_types::sol_data::Array<IProtocolConfig::EpochCrsResult>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "ActivateEpoch(uint256,uint256,(uint256,uint256,(uint8,bytes)[])[],(uint256,uint256,bytes)[],string[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                108u8, 110u8, 179u8, 88u8, 129u8, 248u8, 5u8, 242u8, 132u8, 190u8, 89u8,
                184u8, 64u8, 180u8, 234u8, 153u8, 15u8, 150u8, 177u8, 61u8, 217u8, 231u8,
                25u8, 44u8, 69u8, 129u8, 225u8, 188u8, 129u8, 192u8, 175u8, 152u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    kmsContextId: topics.1,
                    epochId: topics.2,
                    keys: data.0,
                    crsList: data.1,
                    kmsNodeStorageUrls: data.2,
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
                        IProtocolConfig::EpochKeyResult,
                    > as alloy_sol_types::SolType>::tokenize(&self.keys),
                    <alloy::sol_types::sol_data::Array<
                        IProtocolConfig::EpochCrsResult,
                    > as alloy_sol_types::SolType>::tokenize(&self.crsList),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsNodeStorageUrls),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.kmsContextId.clone(),
                    self.epochId.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.kmsContextId);
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.epochId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ActivateEpoch {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ActivateEpoch> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ActivateEpoch) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `CoprocessorUpgradeProposed(uint256,string,(uint64,uint64,uint64)[],uint64)` and selector `0xc42f1ecac8d4881ef9fd335ca98ee7254a436b92ba874a609e50a7d30c487b8d`.
```solidity
event CoprocessorUpgradeProposed(uint256 indexed proposalId, string softwareVersion, ChainUpgradeWindow[] chainUpgradeWindows, uint64 gwStartBlock);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct CoprocessorUpgradeProposed {
        #[allow(missing_docs)]
        pub proposalId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub softwareVersion: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub chainUpgradeWindows: alloy::sol_types::private::Vec<
            <ChainUpgradeWindow as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub gwStartBlock: u64,
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
        impl alloy_sol_types::SolEvent for CoprocessorUpgradeProposed {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<ChainUpgradeWindow>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "CoprocessorUpgradeProposed(uint256,string,(uint64,uint64,uint64)[],uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                196u8, 47u8, 30u8, 202u8, 200u8, 212u8, 136u8, 30u8, 249u8, 253u8, 51u8,
                92u8, 169u8, 142u8, 231u8, 37u8, 74u8, 67u8, 107u8, 146u8, 186u8, 135u8,
                74u8, 96u8, 158u8, 80u8, 167u8, 211u8, 12u8, 72u8, 123u8, 141u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    proposalId: topics.1,
                    softwareVersion: data.0,
                    chainUpgradeWindows: data.1,
                    gwStartBlock: data.2,
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.softwareVersion,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        ChainUpgradeWindow,
                    > as alloy_sol_types::SolType>::tokenize(&self.chainUpgradeWindows),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.gwStartBlock),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.proposalId.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.proposalId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for CoprocessorUpgradeProposed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&CoprocessorUpgradeProposed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &CoprocessorUpgradeProposed,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `EpochActivationConfirmation(uint256,address,bytes32,bytes,bytes)` and selector `0x7faef745af79a3af23d4199ce0366975cc2dd7149384d631337d133b226b5be4`.
```solidity
event EpochActivationConfirmation(uint256 indexed epochId, address indexed signer, bytes32 epochMaterialHash, bytes signature, bytes extraData);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EpochActivationConfirmation {
        #[allow(missing_docs)]
        pub epochId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub signer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub epochMaterialHash: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for EpochActivationConfirmation {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "EpochActivationConfirmation(uint256,address,bytes32,bytes,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                127u8, 174u8, 247u8, 69u8, 175u8, 121u8, 163u8, 175u8, 35u8, 212u8, 25u8,
                156u8, 224u8, 54u8, 105u8, 117u8, 204u8, 45u8, 215u8, 20u8, 147u8, 132u8,
                214u8, 49u8, 51u8, 125u8, 19u8, 59u8, 34u8, 107u8, 91u8, 228u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    epochId: topics.1,
                    signer: topics.2,
                    epochMaterialHash: data.0,
                    signature: data.1,
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.epochMaterialHash),
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
                (Self::SIGNATURE_HASH.into(), self.epochId.clone(), self.signer.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.epochId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.signer,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for EpochActivationConfirmation {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EpochActivationConfirmation> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &EpochActivationConfirmation,
            ) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `KmsContextCreationConfirmation(uint256,address,bytes,bytes)` and selector `0x4254d948402ba059c02f9feecfca09b40c0b39026a344a159e3a7679807a83d2`.
```solidity
event KmsContextCreationConfirmation(uint256 indexed kmsContextId, address indexed signer, bytes signature, bytes extraData);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct KmsContextCreationConfirmation {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub signer: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for KmsContextCreationConfirmation {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "KmsContextCreationConfirmation(uint256,address,bytes,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                66u8, 84u8, 217u8, 72u8, 64u8, 43u8, 160u8, 89u8, 192u8, 47u8, 159u8,
                238u8, 207u8, 202u8, 9u8, 180u8, 12u8, 11u8, 57u8, 2u8, 106u8, 52u8,
                74u8, 21u8, 158u8, 58u8, 118u8, 121u8, 128u8, 122u8, 131u8, 210u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    kmsContextId: topics.1,
                    signer: topics.2,
                    signature: data.0,
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
                (
                    Self::SIGNATURE_HASH.into(),
                    self.kmsContextId.clone(),
                    self.signer.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.kmsContextId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.signer,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for KmsContextCreationConfirmation {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&KmsContextCreationConfirmation>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &KmsContextCreationConfirmation,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `KmsContextDestroyed(uint256)` and selector `0xda075d09198d207e3a918d4b8dfc87df2d60a00be703fd39eaac90962da0b7f0`.
```solidity
event KmsContextDestroyed(uint256 indexed kmsContextId);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct KmsContextDestroyed {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for KmsContextDestroyed {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "KmsContextDestroyed(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                218u8, 7u8, 93u8, 9u8, 25u8, 141u8, 32u8, 126u8, 58u8, 145u8, 141u8,
                75u8, 141u8, 252u8, 135u8, 223u8, 45u8, 96u8, 160u8, 11u8, 231u8, 3u8,
                253u8, 57u8, 234u8, 172u8, 144u8, 150u8, 45u8, 160u8, 183u8, 240u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { kmsContextId: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.kmsContextId.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.kmsContextId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for KmsContextDestroyed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&KmsContextDestroyed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &KmsContextDestroyed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `KmsContextDestructionConfirmation(uint256,uint256[],address,bytes,bytes)` and selector `0xea1640f34deada2f815def0e112e6cd249533849a0fd53e9293ba34d7c4688cb`.
```solidity
event KmsContextDestructionConfirmation(uint256 indexed destroyedContextId, uint256[] destroyedEpochIds, address indexed signer, bytes signature, bytes extraData);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct KmsContextDestructionConfirmation {
        #[allow(missing_docs)]
        pub destroyedContextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub destroyedEpochIds: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub signer: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for KmsContextDestructionConfirmation {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "KmsContextDestructionConfirmation(uint256,uint256[],address,bytes,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                234u8, 22u8, 64u8, 243u8, 77u8, 234u8, 218u8, 47u8, 129u8, 93u8, 239u8,
                14u8, 17u8, 46u8, 108u8, 210u8, 73u8, 83u8, 56u8, 73u8, 160u8, 253u8,
                83u8, 233u8, 41u8, 59u8, 163u8, 77u8, 124u8, 70u8, 136u8, 203u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    destroyedContextId: topics.1,
                    destroyedEpochIds: data.0,
                    signer: topics.2,
                    signature: data.1,
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
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.destroyedEpochIds),
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
                (
                    Self::SIGNATURE_HASH.into(),
                    self.destroyedContextId.clone(),
                    self.signer.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(
                    &self.destroyedContextId,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.signer,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData
        for KmsContextDestructionConfirmation {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&KmsContextDestructionConfirmation>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &KmsContextDestructionConfirmation,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `KmsEpochDestroyed(uint256)` and selector `0x03436013075f8d1abb1781dbcaf418fc76fc797d1ab51c38664c1a51f3cd57f9`.
```solidity
event KmsEpochDestroyed(uint256 indexed epochId);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct KmsEpochDestroyed {
        #[allow(missing_docs)]
        pub epochId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for KmsEpochDestroyed {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "KmsEpochDestroyed(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                3u8, 67u8, 96u8, 19u8, 7u8, 95u8, 141u8, 26u8, 187u8, 23u8, 129u8, 219u8,
                202u8, 244u8, 24u8, 252u8, 118u8, 252u8, 121u8, 125u8, 26u8, 181u8, 28u8,
                56u8, 102u8, 76u8, 26u8, 81u8, 243u8, 205u8, 87u8, 249u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { epochId: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.epochId.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.epochId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for KmsEpochDestroyed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&KmsEpochDestroyed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &KmsEpochDestroyed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `KmsEpochDestructionConfirmation(uint256,address,bytes,bytes)` and selector `0xee1b8f29d516d563f41f2a3b75c200752feb512e6256eb561d17a4e61ee6335e`.
```solidity
event KmsEpochDestructionConfirmation(uint256 indexed destroyedEpochId, address indexed signer, bytes signature, bytes extraData);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct KmsEpochDestructionConfirmation {
        #[allow(missing_docs)]
        pub destroyedEpochId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub signer: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for KmsEpochDestructionConfirmation {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "KmsEpochDestructionConfirmation(uint256,address,bytes,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                238u8, 27u8, 143u8, 41u8, 213u8, 22u8, 213u8, 99u8, 244u8, 31u8, 42u8,
                59u8, 117u8, 194u8, 0u8, 117u8, 47u8, 235u8, 81u8, 46u8, 98u8, 86u8,
                235u8, 86u8, 29u8, 23u8, 164u8, 230u8, 30u8, 230u8, 51u8, 94u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    destroyedEpochId: topics.1,
                    signer: topics.2,
                    signature: data.0,
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
                (
                    Self::SIGNATURE_HASH.into(),
                    self.destroyedEpochId.clone(),
                    self.signer.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.destroyedEpochId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.signer,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for KmsEpochDestructionConfirmation {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&KmsEpochDestructionConfirmation>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &KmsEpochDestructionConfirmation,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `NewKmsContext(uint256,uint256,(address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[])` and selector `0x204d6b80121154cd87d99cf54c639a3dd0a53b3084277098de972ebdd34c6be9`.
```solidity
event NewKmsContext(uint256 indexed contextId, uint256 indexed previousContextId, KmsNodeParams[] kmsNodeParams, KmsThresholds thresholds, string softwareVersion, PcrValues[] pcrValues);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct NewKmsContext {
        #[allow(missing_docs)]
        pub contextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub previousContextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub kmsNodeParams: alloy::sol_types::private::Vec<
            <KmsNodeParams as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub thresholds: <KmsThresholds as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub softwareVersion: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub pcrValues: alloy::sol_types::private::Vec<
            <PcrValues as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for NewKmsContext {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<KmsNodeParams>,
                KmsThresholds,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<PcrValues>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "NewKmsContext(uint256,uint256,(address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                32u8, 77u8, 107u8, 128u8, 18u8, 17u8, 84u8, 205u8, 135u8, 217u8, 156u8,
                245u8, 76u8, 99u8, 154u8, 61u8, 208u8, 165u8, 59u8, 48u8, 132u8, 39u8,
                112u8, 152u8, 222u8, 151u8, 46u8, 189u8, 211u8, 76u8, 107u8, 233u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    contextId: topics.1,
                    previousContextId: topics.2,
                    kmsNodeParams: data.0,
                    thresholds: data.1,
                    softwareVersion: data.2,
                    pcrValues: data.3,
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
                        KmsNodeParams,
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsNodeParams),
                    <KmsThresholds as alloy_sol_types::SolType>::tokenize(
                        &self.thresholds,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.softwareVersion,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        PcrValues,
                    > as alloy_sol_types::SolType>::tokenize(&self.pcrValues),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.contextId.clone(),
                    self.previousContextId.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.contextId);
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.previousContextId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for NewKmsContext {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NewKmsContext> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NewKmsContext) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `NewKmsEpoch(uint256,uint256,uint256,uint256,uint256)` and selector `0x15aaaf475ef407543f5164f57dcf57f7f93816f55bae77ca09efc445ba40eef7`.
```solidity
event NewKmsEpoch(uint256 indexed kmsContextId, uint256 indexed epochId, uint256 previousContextId, uint256 previousEpochId, uint256 materialBlockNumber);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct NewKmsEpoch {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub epochId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub previousContextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub previousEpochId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub materialBlockNumber: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for NewKmsEpoch {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "NewKmsEpoch(uint256,uint256,uint256,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                21u8, 170u8, 175u8, 71u8, 94u8, 244u8, 7u8, 84u8, 63u8, 81u8, 100u8,
                245u8, 125u8, 207u8, 87u8, 247u8, 249u8, 56u8, 22u8, 245u8, 91u8, 174u8,
                119u8, 202u8, 9u8, 239u8, 196u8, 69u8, 186u8, 64u8, 238u8, 247u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    kmsContextId: topics.1,
                    epochId: topics.2,
                    previousContextId: data.0,
                    previousEpochId: data.1,
                    materialBlockNumber: data.2,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.previousContextId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.previousEpochId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.materialBlockNumber),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.kmsContextId.clone(),
                    self.epochId.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.kmsContextId);
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.epochId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for NewKmsEpoch {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NewKmsEpoch> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NewKmsEpoch) -> alloy_sol_types::private::LogData {
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
    #[derive()]
    /**Function with signature `confirmEpochActivation(uint256,(uint256,uint256,(uint8,bytes)[])[],(uint256,uint256,bytes)[],bytes,bytes)` and selector `0x71d76dae`.
```solidity
function confirmEpochActivation(uint256 epochId, IProtocolConfig.EpochKeyResult[] memory keys, IProtocolConfig.EpochCrsResult[] memory crsList, bytes memory signature, bytes memory extraData) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct confirmEpochActivationCall {
        #[allow(missing_docs)]
        pub epochId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub keys: alloy::sol_types::private::Vec<
            <IProtocolConfig::EpochKeyResult as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub crsList: alloy::sol_types::private::Vec<
            <IProtocolConfig::EpochCrsResult as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub extraData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`confirmEpochActivation(uint256,(uint256,uint256,(uint8,bytes)[])[],(uint256,uint256,bytes)[],bytes,bytes)`](confirmEpochActivationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct confirmEpochActivationReturn {}
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
                alloy::sol_types::sol_data::Array<IProtocolConfig::EpochKeyResult>,
                alloy::sol_types::sol_data::Array<IProtocolConfig::EpochCrsResult>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    <IProtocolConfig::EpochKeyResult as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Vec<
                    <IProtocolConfig::EpochCrsResult as alloy::sol_types::SolType>::RustType,
                >,
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
            impl ::core::convert::From<confirmEpochActivationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: confirmEpochActivationCall) -> Self {
                    (
                        value.epochId,
                        value.keys,
                        value.crsList,
                        value.signature,
                        value.extraData,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for confirmEpochActivationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        epochId: tuple.0,
                        keys: tuple.1,
                        crsList: tuple.2,
                        signature: tuple.3,
                        extraData: tuple.4,
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
            impl ::core::convert::From<confirmEpochActivationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: confirmEpochActivationReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for confirmEpochActivationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl confirmEpochActivationReturn {
            fn _tokenize(
                &self,
            ) -> <confirmEpochActivationCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for confirmEpochActivationCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IProtocolConfig::EpochKeyResult>,
                alloy::sol_types::sol_data::Array<IProtocolConfig::EpochCrsResult>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = confirmEpochActivationReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "confirmEpochActivation(uint256,(uint256,uint256,(uint8,bytes)[])[],(uint256,uint256,bytes)[],bytes,bytes)";
            const SELECTOR: [u8; 4] = [113u8, 215u8, 109u8, 174u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.epochId),
                    <alloy::sol_types::sol_data::Array<
                        IProtocolConfig::EpochKeyResult,
                    > as alloy_sol_types::SolType>::tokenize(&self.keys),
                    <alloy::sol_types::sol_data::Array<
                        IProtocolConfig::EpochCrsResult,
                    > as alloy_sol_types::SolType>::tokenize(&self.crsList),
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
                confirmEpochActivationReturn::_tokenize(ret)
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
    /**Function with signature `confirmKmsContextCreation(uint256,bytes,bytes)` and selector `0x6ec0f23f`.
```solidity
function confirmKmsContextCreation(uint256 kmsContextId, bytes memory signature, bytes memory extraData) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct confirmKmsContextCreationCall {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub extraData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`confirmKmsContextCreation(uint256,bytes,bytes)`](confirmKmsContextCreationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct confirmKmsContextCreationReturn {}
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
            impl ::core::convert::From<confirmKmsContextCreationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: confirmKmsContextCreationCall) -> Self {
                    (value.kmsContextId, value.signature, value.extraData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for confirmKmsContextCreationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        kmsContextId: tuple.0,
                        signature: tuple.1,
                        extraData: tuple.2,
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
            impl ::core::convert::From<confirmKmsContextCreationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: confirmKmsContextCreationReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for confirmKmsContextCreationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl confirmKmsContextCreationReturn {
            fn _tokenize(
                &self,
            ) -> <confirmKmsContextCreationCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for confirmKmsContextCreationCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = confirmKmsContextCreationReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "confirmKmsContextCreation(uint256,bytes,bytes)";
            const SELECTOR: [u8; 4] = [110u8, 192u8, 242u8, 63u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsContextId),
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
                confirmKmsContextCreationReturn::_tokenize(ret)
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
    /**Function with signature `confirmKmsContextDestruction(uint256,uint256[],bytes,bytes)` and selector `0x756e8652`.
```solidity
function confirmKmsContextDestruction(uint256 destroyedContextId, uint256[] memory destroyedEpochIds, bytes memory signature, bytes memory extraData) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct confirmKmsContextDestructionCall {
        #[allow(missing_docs)]
        pub destroyedContextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub destroyedEpochIds: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub extraData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`confirmKmsContextDestruction(uint256,uint256[],bytes,bytes)`](confirmKmsContextDestructionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct confirmKmsContextDestructionReturn {}
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
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
            impl ::core::convert::From<confirmKmsContextDestructionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: confirmKmsContextDestructionCall) -> Self {
                    (
                        value.destroyedContextId,
                        value.destroyedEpochIds,
                        value.signature,
                        value.extraData,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for confirmKmsContextDestructionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        destroyedContextId: tuple.0,
                        destroyedEpochIds: tuple.1,
                        signature: tuple.2,
                        extraData: tuple.3,
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
            impl ::core::convert::From<confirmKmsContextDestructionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: confirmKmsContextDestructionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for confirmKmsContextDestructionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl confirmKmsContextDestructionReturn {
            fn _tokenize(
                &self,
            ) -> <confirmKmsContextDestructionCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for confirmKmsContextDestructionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = confirmKmsContextDestructionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "confirmKmsContextDestruction(uint256,uint256[],bytes,bytes)";
            const SELECTOR: [u8; 4] = [117u8, 110u8, 134u8, 82u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.destroyedContextId),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.destroyedEpochIds),
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
                confirmKmsContextDestructionReturn::_tokenize(ret)
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
    /**Function with signature `confirmKmsEpochDestruction(uint256,bytes,bytes)` and selector `0xa642ba0f`.
```solidity
function confirmKmsEpochDestruction(uint256 destroyedEpochId, bytes memory signature, bytes memory extraData) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct confirmKmsEpochDestructionCall {
        #[allow(missing_docs)]
        pub destroyedEpochId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub extraData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`confirmKmsEpochDestruction(uint256,bytes,bytes)`](confirmKmsEpochDestructionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct confirmKmsEpochDestructionReturn {}
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
            impl ::core::convert::From<confirmKmsEpochDestructionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: confirmKmsEpochDestructionCall) -> Self {
                    (value.destroyedEpochId, value.signature, value.extraData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for confirmKmsEpochDestructionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        destroyedEpochId: tuple.0,
                        signature: tuple.1,
                        extraData: tuple.2,
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
            impl ::core::convert::From<confirmKmsEpochDestructionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: confirmKmsEpochDestructionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for confirmKmsEpochDestructionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl confirmKmsEpochDestructionReturn {
            fn _tokenize(
                &self,
            ) -> <confirmKmsEpochDestructionCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for confirmKmsEpochDestructionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = confirmKmsEpochDestructionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "confirmKmsEpochDestruction(uint256,bytes,bytes)";
            const SELECTOR: [u8; 4] = [166u8, 66u8, 186u8, 15u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.destroyedEpochId),
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
                confirmKmsEpochDestructionReturn::_tokenize(ret)
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
    /**Function with signature `defineNewEpochForCurrentKmsContext()` and selector `0x1ce3f9bc`.
```solidity
function defineNewEpochForCurrentKmsContext() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct defineNewEpochForCurrentKmsContextCall;
    ///Container type for the return parameters of the [`defineNewEpochForCurrentKmsContext()`](defineNewEpochForCurrentKmsContextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct defineNewEpochForCurrentKmsContextReturn {}
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
            impl ::core::convert::From<defineNewEpochForCurrentKmsContextCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: defineNewEpochForCurrentKmsContextCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for defineNewEpochForCurrentKmsContextCall {
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
            impl ::core::convert::From<defineNewEpochForCurrentKmsContextReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: defineNewEpochForCurrentKmsContextReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for defineNewEpochForCurrentKmsContextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl defineNewEpochForCurrentKmsContextReturn {
            fn _tokenize(
                &self,
            ) -> <defineNewEpochForCurrentKmsContextCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for defineNewEpochForCurrentKmsContextCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = defineNewEpochForCurrentKmsContextReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "defineNewEpochForCurrentKmsContext()";
            const SELECTOR: [u8; 4] = [28u8, 227u8, 249u8, 188u8];
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
                defineNewEpochForCurrentKmsContextReturn::_tokenize(ret)
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
    /**Function with signature `defineNewKmsContextAndEpoch((address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[])` and selector `0x976c98b5`.
```solidity
function defineNewKmsContextAndEpoch(KmsNodeParams[] memory kmsNodeParams, KmsThresholds memory thresholds, string memory softwareVersion, PcrValues[] memory pcrValues) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct defineNewKmsContextAndEpochCall {
        #[allow(missing_docs)]
        pub kmsNodeParams: alloy::sol_types::private::Vec<
            <KmsNodeParams as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub thresholds: <KmsThresholds as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub softwareVersion: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub pcrValues: alloy::sol_types::private::Vec<
            <PcrValues as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`defineNewKmsContextAndEpoch((address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[])`](defineNewKmsContextAndEpochCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct defineNewKmsContextAndEpochReturn {}
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
                alloy::sol_types::sol_data::Array<KmsNodeParams>,
                KmsThresholds,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<PcrValues>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <KmsNodeParams as alloy::sol_types::SolType>::RustType,
                >,
                <KmsThresholds as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::String,
                alloy::sol_types::private::Vec<
                    <PcrValues as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<defineNewKmsContextAndEpochCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: defineNewKmsContextAndEpochCall) -> Self {
                    (
                        value.kmsNodeParams,
                        value.thresholds,
                        value.softwareVersion,
                        value.pcrValues,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for defineNewKmsContextAndEpochCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        kmsNodeParams: tuple.0,
                        thresholds: tuple.1,
                        softwareVersion: tuple.2,
                        pcrValues: tuple.3,
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
            impl ::core::convert::From<defineNewKmsContextAndEpochReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: defineNewKmsContextAndEpochReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for defineNewKmsContextAndEpochReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl defineNewKmsContextAndEpochReturn {
            fn _tokenize(
                &self,
            ) -> <defineNewKmsContextAndEpochCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for defineNewKmsContextAndEpochCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<KmsNodeParams>,
                KmsThresholds,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<PcrValues>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = defineNewKmsContextAndEpochReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "defineNewKmsContextAndEpoch((address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[])";
            const SELECTOR: [u8; 4] = [151u8, 108u8, 152u8, 181u8];
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
                        KmsNodeParams,
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsNodeParams),
                    <KmsThresholds as alloy_sol_types::SolType>::tokenize(
                        &self.thresholds,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.softwareVersion,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        PcrValues,
                    > as alloy_sol_types::SolType>::tokenize(&self.pcrValues),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                defineNewKmsContextAndEpochReturn::_tokenize(ret)
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
    /**Function with signature `destroyKmsContext(uint256)` and selector `0xc0ae64f7`.
```solidity
function destroyKmsContext(uint256 kmsContextId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct destroyKmsContextCall {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`destroyKmsContext(uint256)`](destroyKmsContextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct destroyKmsContextReturn {}
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
            impl ::core::convert::From<destroyKmsContextCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: destroyKmsContextCall) -> Self {
                    (value.kmsContextId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for destroyKmsContextCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { kmsContextId: tuple.0 }
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
            impl ::core::convert::From<destroyKmsContextReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: destroyKmsContextReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for destroyKmsContextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl destroyKmsContextReturn {
            fn _tokenize(
                &self,
            ) -> <destroyKmsContextCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for destroyKmsContextCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = destroyKmsContextReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "destroyKmsContext(uint256)";
            const SELECTOR: [u8; 4] = [192u8, 174u8, 100u8, 247u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsContextId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                destroyKmsContextReturn::_tokenize(ret)
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
    /**Function with signature `destroyKmsEpoch(uint256)` and selector `0xc2580a2d`.
```solidity
function destroyKmsEpoch(uint256 epochId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct destroyKmsEpochCall {
        #[allow(missing_docs)]
        pub epochId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`destroyKmsEpoch(uint256)`](destroyKmsEpochCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct destroyKmsEpochReturn {}
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
            impl ::core::convert::From<destroyKmsEpochCall> for UnderlyingRustTuple<'_> {
                fn from(value: destroyKmsEpochCall) -> Self {
                    (value.epochId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for destroyKmsEpochCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { epochId: tuple.0 }
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
            impl ::core::convert::From<destroyKmsEpochReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: destroyKmsEpochReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for destroyKmsEpochReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl destroyKmsEpochReturn {
            fn _tokenize(
                &self,
            ) -> <destroyKmsEpochCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for destroyKmsEpochCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = destroyKmsEpochReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "destroyKmsEpoch(uint256)";
            const SELECTOR: [u8; 4] = [194u8, 88u8, 10u8, 45u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.epochId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                destroyKmsEpochReturn::_tokenize(ret)
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
    /**Function with signature `getCurrentKmsContextAndEpoch()` and selector `0x65b394af`.
```solidity
function getCurrentKmsContextAndEpoch() external view returns (uint256 contextId, uint256 epochId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentKmsContextAndEpochCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCurrentKmsContextAndEpoch()`](getCurrentKmsContextAndEpochCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentKmsContextAndEpochReturn {
        #[allow(missing_docs)]
        pub contextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub epochId: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getCurrentKmsContextAndEpochCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentKmsContextAndEpochCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentKmsContextAndEpochCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<getCurrentKmsContextAndEpochReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentKmsContextAndEpochReturn) -> Self {
                    (value.contextId, value.epochId)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentKmsContextAndEpochReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        contextId: tuple.0,
                        epochId: tuple.1,
                    }
                }
            }
        }
        impl getCurrentKmsContextAndEpochReturn {
            fn _tokenize(
                &self,
            ) -> <getCurrentKmsContextAndEpochCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.contextId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.epochId),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentKmsContextAndEpochCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCurrentKmsContextAndEpochReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCurrentKmsContextAndEpoch()";
            const SELECTOR: [u8; 4] = [101u8, 179u8, 148u8, 175u8];
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
                getCurrentKmsContextAndEpochReturn::_tokenize(ret)
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
    /**Function with signature `getCurrentKmsContextId()` and selector `0x976f3eb9`.
```solidity
function getCurrentKmsContextId() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentKmsContextIdCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCurrentKmsContextId()`](getCurrentKmsContextIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentKmsContextIdReturn {
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
            impl ::core::convert::From<getCurrentKmsContextIdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentKmsContextIdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentKmsContextIdCall {
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
            impl ::core::convert::From<getCurrentKmsContextIdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentKmsContextIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentKmsContextIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentKmsContextIdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCurrentKmsContextId()";
            const SELECTOR: [u8; 4] = [151u8, 111u8, 62u8, 185u8];
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
                        let r: getCurrentKmsContextIdReturn = r.into();
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
                        let r: getCurrentKmsContextIdReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getCurrentKmsContextIdCounter()` and selector `0x9d1b1be1`.
```solidity
function getCurrentKmsContextIdCounter() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentKmsContextIdCounterCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCurrentKmsContextIdCounter()`](getCurrentKmsContextIdCounterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentKmsContextIdCounterReturn {
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
            impl ::core::convert::From<getCurrentKmsContextIdCounterCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentKmsContextIdCounterCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentKmsContextIdCounterCall {
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
            impl ::core::convert::From<getCurrentKmsContextIdCounterReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentKmsContextIdCounterReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentKmsContextIdCounterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentKmsContextIdCounterCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCurrentKmsContextIdCounter()";
            const SELECTOR: [u8; 4] = [157u8, 27u8, 27u8, 225u8];
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
                        let r: getCurrentKmsContextIdCounterReturn = r.into();
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
                        let r: getCurrentKmsContextIdCounterReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getKmsContextAnchor(uint256)` and selector `0xc999a8b4`.
```solidity
function getKmsContextAnchor(uint256 contextId) external view returns (uint256 emissionBlockNumber, bytes32 contextInfoHash);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKmsContextAnchorCall {
        #[allow(missing_docs)]
        pub contextId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getKmsContextAnchor(uint256)`](getKmsContextAnchorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKmsContextAnchorReturn {
        #[allow(missing_docs)]
        pub emissionBlockNumber: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub contextInfoHash: alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<getKmsContextAnchorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKmsContextAnchorCall) -> Self {
                    (value.contextId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKmsContextAnchorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { contextId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<getKmsContextAnchorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKmsContextAnchorReturn) -> Self {
                    (value.emissionBlockNumber, value.contextInfoHash)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKmsContextAnchorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        emissionBlockNumber: tuple.0,
                        contextInfoHash: tuple.1,
                    }
                }
            }
        }
        impl getKmsContextAnchorReturn {
            fn _tokenize(
                &self,
            ) -> <getKmsContextAnchorCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.emissionBlockNumber),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.contextInfoHash),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getKmsContextAnchorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getKmsContextAnchorReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getKmsContextAnchor(uint256)";
            const SELECTOR: [u8; 4] = [201u8, 153u8, 168u8, 180u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.contextId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                getKmsContextAnchorReturn::_tokenize(ret)
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
    /**Function with signature `getKmsGenThreshold()` and selector `0xb4722bc4`.
```solidity
function getKmsGenThreshold() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKmsGenThresholdCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getKmsGenThreshold()`](getKmsGenThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKmsGenThresholdReturn {
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
            impl ::core::convert::From<getKmsGenThresholdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKmsGenThresholdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKmsGenThresholdCall {
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
            impl ::core::convert::From<getKmsGenThresholdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKmsGenThresholdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKmsGenThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getKmsGenThresholdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getKmsGenThreshold()";
            const SELECTOR: [u8; 4] = [180u8, 114u8, 43u8, 196u8];
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
                        let r: getKmsGenThresholdReturn = r.into();
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
                        let r: getKmsGenThresholdReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getKmsGenThresholdForContext(uint256)` and selector `0x41ad069c`.
```solidity
function getKmsGenThresholdForContext(uint256 kmsContextId) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKmsGenThresholdForContextCall {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getKmsGenThresholdForContext(uint256)`](getKmsGenThresholdForContextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKmsGenThresholdForContextReturn {
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
            impl ::core::convert::From<getKmsGenThresholdForContextCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKmsGenThresholdForContextCall) -> Self {
                    (value.kmsContextId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKmsGenThresholdForContextCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { kmsContextId: tuple.0 }
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
            impl ::core::convert::From<getKmsGenThresholdForContextReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKmsGenThresholdForContextReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKmsGenThresholdForContextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getKmsGenThresholdForContextCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getKmsGenThresholdForContext(uint256)";
            const SELECTOR: [u8; 4] = [65u8, 173u8, 6u8, 156u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsContextId),
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
                        let r: getKmsGenThresholdForContextReturn = r.into();
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
                        let r: getKmsGenThresholdForContextReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getKmsNodeForContext(uint256,address)` and selector `0x31ff41c8`.
```solidity
function getKmsNodeForContext(uint256 kmsContextId, address txSender) external view returns (KmsNode memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKmsNodeForContextCall {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub txSender: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getKmsNodeForContext(uint256,address)`](getKmsNodeForContextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKmsNodeForContextReturn {
        #[allow(missing_docs)]
        pub _0: <KmsNode as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getKmsNodeForContextCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKmsNodeForContextCall) -> Self {
                    (value.kmsContextId, value.txSender)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKmsNodeForContextCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        kmsContextId: tuple.0,
                        txSender: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (KmsNode,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <KmsNode as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getKmsNodeForContextReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKmsNodeForContextReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKmsNodeForContextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getKmsNodeForContextCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <KmsNode as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (KmsNode,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getKmsNodeForContext(uint256,address)";
            const SELECTOR: [u8; 4] = [49u8, 255u8, 65u8, 200u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsContextId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.txSender,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<KmsNode as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getKmsNodeForContextReturn = r.into();
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
                        let r: getKmsNodeForContextReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getKmsNodesForContext(uint256)` and selector `0xf9c670c3`.
```solidity
function getKmsNodesForContext(uint256 kmsContextId) external view returns (KmsNode[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKmsNodesForContextCall {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getKmsNodesForContext(uint256)`](getKmsNodesForContextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKmsNodesForContextReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            <KmsNode as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getKmsNodesForContextCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKmsNodesForContextCall) -> Self {
                    (value.kmsContextId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKmsNodesForContextCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { kmsContextId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Array<KmsNode>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <KmsNode as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getKmsNodesForContextReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKmsNodesForContextReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKmsNodesForContextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getKmsNodesForContextCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                <KmsNode as alloy::sol_types::SolType>::RustType,
            >;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Array<KmsNode>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getKmsNodesForContext(uint256)";
            const SELECTOR: [u8; 4] = [249u8, 198u8, 112u8, 195u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsContextId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        KmsNode,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getKmsNodesForContextReturn = r.into();
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
                        let r: getKmsNodesForContextReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getKmsSigners()` and selector `0x7eaac8f2`.
```solidity
function getKmsSigners() external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKmsSignersCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getKmsSigners()`](getKmsSignersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKmsSignersReturn {
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
            impl ::core::convert::From<getKmsSignersCall> for UnderlyingRustTuple<'_> {
                fn from(value: getKmsSignersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getKmsSignersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<getKmsSignersReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getKmsSignersReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getKmsSignersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getKmsSignersCall {
            type Parameters<'a> = ();
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
            const SIGNATURE: &'static str = "getKmsSigners()";
            const SELECTOR: [u8; 4] = [126u8, 170u8, 200u8, 242u8];
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
                        let r: getKmsSignersReturn = r.into();
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
                        let r: getKmsSignersReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getKmsSignersForContext(uint256)` and selector `0x5bff76d9`.
```solidity
function getKmsSignersForContext(uint256 kmsContextId) external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKmsSignersForContextCall {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getKmsSignersForContext(uint256)`](getKmsSignersForContextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKmsSignersForContextReturn {
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
            impl ::core::convert::From<getKmsSignersForContextCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKmsSignersForContextCall) -> Self {
                    (value.kmsContextId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKmsSignersForContextCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { kmsContextId: tuple.0 }
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
            impl ::core::convert::From<getKmsSignersForContextReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKmsSignersForContextReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKmsSignersForContextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getKmsSignersForContextCall {
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
            const SIGNATURE: &'static str = "getKmsSignersForContext(uint256)";
            const SELECTOR: [u8; 4] = [91u8, 255u8, 118u8, 217u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsContextId),
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
                        let r: getKmsSignersForContextReturn = r.into();
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
                        let r: getKmsSignersForContextReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getMpcThreshold()` and selector `0x26cf5def`.
```solidity
function getMpcThreshold() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMpcThresholdCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getMpcThreshold()`](getMpcThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMpcThresholdReturn {
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
            impl ::core::convert::From<getMpcThresholdCall> for UnderlyingRustTuple<'_> {
                fn from(value: getMpcThresholdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMpcThresholdCall {
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
            impl ::core::convert::From<getMpcThresholdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMpcThresholdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMpcThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getMpcThresholdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getMpcThreshold()";
            const SELECTOR: [u8; 4] = [38u8, 207u8, 93u8, 239u8];
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
                        let r: getMpcThresholdReturn = r.into();
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
                        let r: getMpcThresholdReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getMpcThresholdForContext(uint256)` and selector `0x47e82295`.
```solidity
function getMpcThresholdForContext(uint256 kmsContextId) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMpcThresholdForContextCall {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getMpcThresholdForContext(uint256)`](getMpcThresholdForContextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMpcThresholdForContextReturn {
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
            impl ::core::convert::From<getMpcThresholdForContextCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMpcThresholdForContextCall) -> Self {
                    (value.kmsContextId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMpcThresholdForContextCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { kmsContextId: tuple.0 }
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
            impl ::core::convert::From<getMpcThresholdForContextReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMpcThresholdForContextReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMpcThresholdForContextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getMpcThresholdForContextCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getMpcThresholdForContext(uint256)";
            const SELECTOR: [u8; 4] = [71u8, 232u8, 34u8, 149u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsContextId),
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
                        let r: getMpcThresholdForContextReturn = r.into();
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
                        let r: getMpcThresholdForContextReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getPublicDecryptionThreshold()` and selector `0x2a388998`.
```solidity
function getPublicDecryptionThreshold() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPublicDecryptionThresholdCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getPublicDecryptionThreshold()`](getPublicDecryptionThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPublicDecryptionThresholdReturn {
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
            impl ::core::convert::From<getPublicDecryptionThresholdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPublicDecryptionThresholdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPublicDecryptionThresholdCall {
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
            impl ::core::convert::From<getPublicDecryptionThresholdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPublicDecryptionThresholdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPublicDecryptionThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPublicDecryptionThresholdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPublicDecryptionThreshold()";
            const SELECTOR: [u8; 4] = [42u8, 56u8, 137u8, 152u8];
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
                        let r: getPublicDecryptionThresholdReturn = r.into();
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
                        let r: getPublicDecryptionThresholdReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getPublicDecryptionThresholdForContext(uint256)` and selector `0xc3aaaa5a`.
```solidity
function getPublicDecryptionThresholdForContext(uint256 kmsContextId) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPublicDecryptionThresholdForContextCall {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getPublicDecryptionThresholdForContext(uint256)`](getPublicDecryptionThresholdForContextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPublicDecryptionThresholdForContextReturn {
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
            impl ::core::convert::From<getPublicDecryptionThresholdForContextCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPublicDecryptionThresholdForContextCall) -> Self {
                    (value.kmsContextId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPublicDecryptionThresholdForContextCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { kmsContextId: tuple.0 }
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
            impl ::core::convert::From<getPublicDecryptionThresholdForContextReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPublicDecryptionThresholdForContextReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPublicDecryptionThresholdForContextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPublicDecryptionThresholdForContextCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPublicDecryptionThresholdForContext(uint256)";
            const SELECTOR: [u8; 4] = [195u8, 170u8, 170u8, 90u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsContextId),
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
                        let r: getPublicDecryptionThresholdForContextReturn = r.into();
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
                        let r: getPublicDecryptionThresholdForContextReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getUserDecryptionThreshold()` and selector `0xc2b42986`.
```solidity
function getUserDecryptionThreshold() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getUserDecryptionThresholdCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getUserDecryptionThreshold()`](getUserDecryptionThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getUserDecryptionThresholdReturn {
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
            impl ::core::convert::From<getUserDecryptionThresholdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getUserDecryptionThresholdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getUserDecryptionThresholdCall {
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
            impl ::core::convert::From<getUserDecryptionThresholdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getUserDecryptionThresholdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getUserDecryptionThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getUserDecryptionThresholdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getUserDecryptionThreshold()";
            const SELECTOR: [u8; 4] = [194u8, 180u8, 41u8, 134u8];
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
                        let r: getUserDecryptionThresholdReturn = r.into();
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
                        let r: getUserDecryptionThresholdReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getUserDecryptionThresholdForContext(uint256)` and selector `0x281e8bfe`.
```solidity
function getUserDecryptionThresholdForContext(uint256 kmsContextId) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getUserDecryptionThresholdForContextCall {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getUserDecryptionThresholdForContext(uint256)`](getUserDecryptionThresholdForContextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getUserDecryptionThresholdForContextReturn {
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
            impl ::core::convert::From<getUserDecryptionThresholdForContextCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getUserDecryptionThresholdForContextCall) -> Self {
                    (value.kmsContextId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getUserDecryptionThresholdForContextCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { kmsContextId: tuple.0 }
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
            impl ::core::convert::From<getUserDecryptionThresholdForContextReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getUserDecryptionThresholdForContextReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getUserDecryptionThresholdForContextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getUserDecryptionThresholdForContextCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getUserDecryptionThresholdForContext(uint256)";
            const SELECTOR: [u8; 4] = [40u8, 30u8, 139u8, 254u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsContextId),
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
                        let r: getUserDecryptionThresholdForContextReturn = r.into();
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
                        let r: getUserDecryptionThresholdForContextReturn = r.into();
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
    /**Function with signature `initializeFromEmptyProxy((address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[])` and selector `0x221cdd4e`.
```solidity
function initializeFromEmptyProxy(KmsNodeParams[] memory initialKmsNodeParams, KmsThresholds memory initialThresholds, string memory softwareVersion, PcrValues[] memory pcrValues) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeFromEmptyProxyCall {
        #[allow(missing_docs)]
        pub initialKmsNodeParams: alloy::sol_types::private::Vec<
            <KmsNodeParams as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub initialThresholds: <KmsThresholds as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub softwareVersion: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub pcrValues: alloy::sol_types::private::Vec<
            <PcrValues as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`initializeFromEmptyProxy((address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[])`](initializeFromEmptyProxyCall) function.
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<KmsNodeParams>,
                KmsThresholds,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<PcrValues>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <KmsNodeParams as alloy::sol_types::SolType>::RustType,
                >,
                <KmsThresholds as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::String,
                alloy::sol_types::private::Vec<
                    <PcrValues as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<initializeFromEmptyProxyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializeFromEmptyProxyCall) -> Self {
                    (
                        value.initialKmsNodeParams,
                        value.initialThresholds,
                        value.softwareVersion,
                        value.pcrValues,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeFromEmptyProxyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        initialKmsNodeParams: tuple.0,
                        initialThresholds: tuple.1,
                        softwareVersion: tuple.2,
                        pcrValues: tuple.3,
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
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<KmsNodeParams>,
                KmsThresholds,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<PcrValues>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeFromEmptyProxyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initializeFromEmptyProxy((address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[])";
            const SELECTOR: [u8; 4] = [34u8, 28u8, 221u8, 78u8];
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
                        KmsNodeParams,
                    > as alloy_sol_types::SolType>::tokenize(&self.initialKmsNodeParams),
                    <KmsThresholds as alloy_sol_types::SolType>::tokenize(
                        &self.initialThresholds,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.softwareVersion,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        PcrValues,
                    > as alloy_sol_types::SolType>::tokenize(&self.pcrValues),
                )
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
    /**Function with signature `isKmsSigner(address)` and selector `0x203d0114`.
```solidity
function isKmsSigner(address signer) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isKmsSignerCall {
        #[allow(missing_docs)]
        pub signer: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isKmsSigner(address)`](isKmsSignerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isKmsSignerReturn {
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
            impl ::core::convert::From<isKmsSignerCall> for UnderlyingRustTuple<'_> {
                fn from(value: isKmsSignerCall) -> Self {
                    (value.signer,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isKmsSignerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { signer: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<isKmsSignerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isKmsSignerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isKmsSignerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isKmsSignerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isKmsSigner(address)";
            const SELECTOR: [u8; 4] = [32u8, 61u8, 1u8, 20u8];
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
                        &self.signer,
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
                        let r: isKmsSignerReturn = r.into();
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
                        let r: isKmsSignerReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isKmsSignerForContext(uint256,address)` and selector `0x9447cfd4`.
```solidity
function isKmsSignerForContext(uint256 kmsContextId, address signer) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isKmsSignerForContextCall {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub signer: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isKmsSignerForContext(uint256,address)`](isKmsSignerForContextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isKmsSignerForContextReturn {
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
            impl ::core::convert::From<isKmsSignerForContextCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isKmsSignerForContextCall) -> Self {
                    (value.kmsContextId, value.signer)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isKmsSignerForContextCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        kmsContextId: tuple.0,
                        signer: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<isKmsSignerForContextReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isKmsSignerForContextReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isKmsSignerForContextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isKmsSignerForContextCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isKmsSignerForContext(uint256,address)";
            const SELECTOR: [u8; 4] = [148u8, 71u8, 207u8, 212u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsContextId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.signer,
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
                        let r: isKmsSignerForContextReturn = r.into();
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
                        let r: isKmsSignerForContextReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isKmsTxSenderForContext(uint256,address)` and selector `0x46c5bbbd`.
```solidity
function isKmsTxSenderForContext(uint256 kmsContextId, address txSender) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isKmsTxSenderForContextCall {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub txSender: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isKmsTxSenderForContext(uint256,address)`](isKmsTxSenderForContextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isKmsTxSenderForContextReturn {
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
            impl ::core::convert::From<isKmsTxSenderForContextCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isKmsTxSenderForContextCall) -> Self {
                    (value.kmsContextId, value.txSender)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isKmsTxSenderForContextCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        kmsContextId: tuple.0,
                        txSender: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<isKmsTxSenderForContextReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isKmsTxSenderForContextReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isKmsTxSenderForContextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isKmsTxSenderForContextCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isKmsTxSenderForContext(uint256,address)";
            const SELECTOR: [u8; 4] = [70u8, 197u8, 187u8, 189u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsContextId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.txSender,
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
                        let r: isKmsTxSenderForContextReturn = r.into();
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
                        let r: isKmsTxSenderForContextReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isLiveKmsContext(uint256)` and selector `0xee7d52d1`.
```solidity
function isLiveKmsContext(uint256 kmsContextId) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isLiveKmsContextCall {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isLiveKmsContext(uint256)`](isLiveKmsContextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isLiveKmsContextReturn {
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
            impl ::core::convert::From<isLiveKmsContextCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isLiveKmsContextCall) -> Self {
                    (value.kmsContextId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isLiveKmsContextCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { kmsContextId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<isLiveKmsContextReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isLiveKmsContextReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isLiveKmsContextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isLiveKmsContextCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isLiveKmsContext(uint256)";
            const SELECTOR: [u8; 4] = [238u8, 125u8, 82u8, 209u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsContextId),
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
                        let r: isLiveKmsContextReturn = r.into();
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
                        let r: isLiveKmsContextReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isValidEpochForContext(uint256,uint256)` and selector `0xcceac019`.
```solidity
function isValidEpochForContext(uint256 kmsContextId, uint256 epochId) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidEpochForContextCall {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub epochId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isValidEpochForContext(uint256,uint256)`](isValidEpochForContextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidEpochForContextReturn {
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<isValidEpochForContextCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isValidEpochForContextCall) -> Self {
                    (value.kmsContextId, value.epochId)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isValidEpochForContextCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        kmsContextId: tuple.0,
                        epochId: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<isValidEpochForContextReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isValidEpochForContextReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isValidEpochForContextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isValidEpochForContextCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isValidEpochForContext(uint256,uint256)";
            const SELECTOR: [u8; 4] = [204u8, 234u8, 192u8, 25u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsContextId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.epochId),
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
                        let r: isValidEpochForContextReturn = r.into();
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
                        let r: isValidEpochForContextReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isValidKmsContext(uint256)` and selector `0xbf9b16c8`.
```solidity
function isValidKmsContext(uint256 kmsContextId) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidKmsContextCall {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isValidKmsContext(uint256)`](isValidKmsContextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidKmsContextReturn {
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
            impl ::core::convert::From<isValidKmsContextCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isValidKmsContextCall) -> Self {
                    (value.kmsContextId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isValidKmsContextCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { kmsContextId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<isValidKmsContextReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isValidKmsContextReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isValidKmsContextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isValidKmsContextCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isValidKmsContext(uint256)";
            const SELECTOR: [u8; 4] = [191u8, 155u8, 22u8, 200u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsContextId),
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
                        let r: isValidKmsContextReturn = r.into();
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
                        let r: isValidKmsContextReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `proposeCoprocessorUpgrade(uint256,string,(uint64,uint64,uint64)[],uint64)` and selector `0xccbf8199`.
```solidity
function proposeCoprocessorUpgrade(uint256 proposalId, string memory softwareVersion, ChainUpgradeWindow[] memory chainUpgradeWindows, uint64 gwStartBlock) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposeCoprocessorUpgradeCall {
        #[allow(missing_docs)]
        pub proposalId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub softwareVersion: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub chainUpgradeWindows: alloy::sol_types::private::Vec<
            <ChainUpgradeWindow as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub gwStartBlock: u64,
    }
    ///Container type for the return parameters of the [`proposeCoprocessorUpgrade(uint256,string,(uint64,uint64,uint64)[],uint64)`](proposeCoprocessorUpgradeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposeCoprocessorUpgradeReturn {}
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
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<ChainUpgradeWindow>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::String,
                alloy::sol_types::private::Vec<
                    <ChainUpgradeWindow as alloy::sol_types::SolType>::RustType,
                >,
                u64,
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
            impl ::core::convert::From<proposeCoprocessorUpgradeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposeCoprocessorUpgradeCall) -> Self {
                    (
                        value.proposalId,
                        value.softwareVersion,
                        value.chainUpgradeWindows,
                        value.gwStartBlock,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposeCoprocessorUpgradeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        proposalId: tuple.0,
                        softwareVersion: tuple.1,
                        chainUpgradeWindows: tuple.2,
                        gwStartBlock: tuple.3,
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
            impl ::core::convert::From<proposeCoprocessorUpgradeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposeCoprocessorUpgradeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposeCoprocessorUpgradeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl proposeCoprocessorUpgradeReturn {
            fn _tokenize(
                &self,
            ) -> <proposeCoprocessorUpgradeCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposeCoprocessorUpgradeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<ChainUpgradeWindow>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = proposeCoprocessorUpgradeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proposeCoprocessorUpgrade(uint256,string,(uint64,uint64,uint64)[],uint64)";
            const SELECTOR: [u8; 4] = [204u8, 191u8, 129u8, 153u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.proposalId),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.softwareVersion,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        ChainUpgradeWindow,
                    > as alloy_sol_types::SolType>::tokenize(&self.chainUpgradeWindows),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.gwStartBlock),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                proposeCoprocessorUpgradeReturn::_tokenize(ret)
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
    ///Container for all the [`ProtocolConfig`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum ProtocolConfigCalls {
        #[allow(missing_docs)]
        UPGRADE_INTERFACE_VERSION(UPGRADE_INTERFACE_VERSIONCall),
        #[allow(missing_docs)]
        confirmEpochActivation(confirmEpochActivationCall),
        #[allow(missing_docs)]
        confirmKmsContextCreation(confirmKmsContextCreationCall),
        #[allow(missing_docs)]
        confirmKmsContextDestruction(confirmKmsContextDestructionCall),
        #[allow(missing_docs)]
        confirmKmsEpochDestruction(confirmKmsEpochDestructionCall),
        #[allow(missing_docs)]
        defineNewEpochForCurrentKmsContext(defineNewEpochForCurrentKmsContextCall),
        #[allow(missing_docs)]
        defineNewKmsContextAndEpoch(defineNewKmsContextAndEpochCall),
        #[allow(missing_docs)]
        destroyKmsContext(destroyKmsContextCall),
        #[allow(missing_docs)]
        destroyKmsEpoch(destroyKmsEpochCall),
        #[allow(missing_docs)]
        getCurrentKmsContextAndEpoch(getCurrentKmsContextAndEpochCall),
        #[allow(missing_docs)]
        getCurrentKmsContextId(getCurrentKmsContextIdCall),
        #[allow(missing_docs)]
        getCurrentKmsContextIdCounter(getCurrentKmsContextIdCounterCall),
        #[allow(missing_docs)]
        getKmsContextAnchor(getKmsContextAnchorCall),
        #[allow(missing_docs)]
        getKmsGenThreshold(getKmsGenThresholdCall),
        #[allow(missing_docs)]
        getKmsGenThresholdForContext(getKmsGenThresholdForContextCall),
        #[allow(missing_docs)]
        getKmsNodeForContext(getKmsNodeForContextCall),
        #[allow(missing_docs)]
        getKmsNodesForContext(getKmsNodesForContextCall),
        #[allow(missing_docs)]
        getKmsSigners(getKmsSignersCall),
        #[allow(missing_docs)]
        getKmsSignersForContext(getKmsSignersForContextCall),
        #[allow(missing_docs)]
        getMpcThreshold(getMpcThresholdCall),
        #[allow(missing_docs)]
        getMpcThresholdForContext(getMpcThresholdForContextCall),
        #[allow(missing_docs)]
        getPublicDecryptionThreshold(getPublicDecryptionThresholdCall),
        #[allow(missing_docs)]
        getPublicDecryptionThresholdForContext(
            getPublicDecryptionThresholdForContextCall,
        ),
        #[allow(missing_docs)]
        getUserDecryptionThreshold(getUserDecryptionThresholdCall),
        #[allow(missing_docs)]
        getUserDecryptionThresholdForContext(getUserDecryptionThresholdForContextCall),
        #[allow(missing_docs)]
        getVersion(getVersionCall),
        #[allow(missing_docs)]
        initializeFromEmptyProxy(initializeFromEmptyProxyCall),
        #[allow(missing_docs)]
        isKmsSigner(isKmsSignerCall),
        #[allow(missing_docs)]
        isKmsSignerForContext(isKmsSignerForContextCall),
        #[allow(missing_docs)]
        isKmsTxSenderForContext(isKmsTxSenderForContextCall),
        #[allow(missing_docs)]
        isLiveKmsContext(isLiveKmsContextCall),
        #[allow(missing_docs)]
        isValidEpochForContext(isValidEpochForContextCall),
        #[allow(missing_docs)]
        isValidKmsContext(isValidKmsContextCall),
        #[allow(missing_docs)]
        proposeCoprocessorUpgrade(proposeCoprocessorUpgradeCall),
        #[allow(missing_docs)]
        proxiableUUID(proxiableUUIDCall),
        #[allow(missing_docs)]
        reinitializeV4(reinitializeV4Call),
        #[allow(missing_docs)]
        upgradeToAndCall(upgradeToAndCallCall),
    }
    impl ProtocolConfigCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [13u8, 142u8, 110u8, 44u8],
            [18u8, 58u8, 187u8, 40u8],
            [28u8, 227u8, 249u8, 188u8],
            [32u8, 61u8, 1u8, 20u8],
            [34u8, 28u8, 221u8, 78u8],
            [38u8, 207u8, 93u8, 239u8],
            [40u8, 30u8, 139u8, 254u8],
            [42u8, 56u8, 137u8, 152u8],
            [49u8, 255u8, 65u8, 200u8],
            [65u8, 173u8, 6u8, 156u8],
            [70u8, 197u8, 187u8, 189u8],
            [71u8, 232u8, 34u8, 149u8],
            [79u8, 30u8, 242u8, 134u8],
            [82u8, 209u8, 144u8, 45u8],
            [91u8, 255u8, 118u8, 217u8],
            [101u8, 179u8, 148u8, 175u8],
            [110u8, 192u8, 242u8, 63u8],
            [113u8, 215u8, 109u8, 174u8],
            [117u8, 110u8, 134u8, 82u8],
            [126u8, 170u8, 200u8, 242u8],
            [148u8, 71u8, 207u8, 212u8],
            [151u8, 108u8, 152u8, 181u8],
            [151u8, 111u8, 62u8, 185u8],
            [157u8, 27u8, 27u8, 225u8],
            [166u8, 66u8, 186u8, 15u8],
            [173u8, 60u8, 177u8, 204u8],
            [180u8, 114u8, 43u8, 196u8],
            [191u8, 155u8, 22u8, 200u8],
            [192u8, 174u8, 100u8, 247u8],
            [194u8, 88u8, 10u8, 45u8],
            [194u8, 180u8, 41u8, 134u8],
            [195u8, 170u8, 170u8, 90u8],
            [201u8, 153u8, 168u8, 180u8],
            [204u8, 191u8, 129u8, 153u8],
            [204u8, 234u8, 192u8, 25u8],
            [238u8, 125u8, 82u8, 209u8],
            [249u8, 198u8, 112u8, 195u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(getVersion),
            ::core::stringify!(reinitializeV4),
            ::core::stringify!(defineNewEpochForCurrentKmsContext),
            ::core::stringify!(isKmsSigner),
            ::core::stringify!(initializeFromEmptyProxy),
            ::core::stringify!(getMpcThreshold),
            ::core::stringify!(getUserDecryptionThresholdForContext),
            ::core::stringify!(getPublicDecryptionThreshold),
            ::core::stringify!(getKmsNodeForContext),
            ::core::stringify!(getKmsGenThresholdForContext),
            ::core::stringify!(isKmsTxSenderForContext),
            ::core::stringify!(getMpcThresholdForContext),
            ::core::stringify!(upgradeToAndCall),
            ::core::stringify!(proxiableUUID),
            ::core::stringify!(getKmsSignersForContext),
            ::core::stringify!(getCurrentKmsContextAndEpoch),
            ::core::stringify!(confirmKmsContextCreation),
            ::core::stringify!(confirmEpochActivation),
            ::core::stringify!(confirmKmsContextDestruction),
            ::core::stringify!(getKmsSigners),
            ::core::stringify!(isKmsSignerForContext),
            ::core::stringify!(defineNewKmsContextAndEpoch),
            ::core::stringify!(getCurrentKmsContextId),
            ::core::stringify!(getCurrentKmsContextIdCounter),
            ::core::stringify!(confirmKmsEpochDestruction),
            ::core::stringify!(UPGRADE_INTERFACE_VERSION),
            ::core::stringify!(getKmsGenThreshold),
            ::core::stringify!(isValidKmsContext),
            ::core::stringify!(destroyKmsContext),
            ::core::stringify!(destroyKmsEpoch),
            ::core::stringify!(getUserDecryptionThreshold),
            ::core::stringify!(getPublicDecryptionThresholdForContext),
            ::core::stringify!(getKmsContextAnchor),
            ::core::stringify!(proposeCoprocessorUpgrade),
            ::core::stringify!(isValidEpochForContext),
            ::core::stringify!(isLiveKmsContext),
            ::core::stringify!(getKmsNodesForContext),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <getVersionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <reinitializeV4Call as alloy_sol_types::SolCall>::SIGNATURE,
            <defineNewEpochForCurrentKmsContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isKmsSignerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getMpcThresholdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getUserDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getPublicDecryptionThresholdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getKmsNodeForContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getKmsGenThresholdForContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isKmsTxSenderForContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getMpcThresholdForContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <upgradeToAndCallCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proxiableUUIDCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getKmsSignersForContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getCurrentKmsContextAndEpochCall as alloy_sol_types::SolCall>::SIGNATURE,
            <confirmKmsContextCreationCall as alloy_sol_types::SolCall>::SIGNATURE,
            <confirmEpochActivationCall as alloy_sol_types::SolCall>::SIGNATURE,
            <confirmKmsContextDestructionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getKmsSignersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isKmsSignerForContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <defineNewKmsContextAndEpochCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getCurrentKmsContextIdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getCurrentKmsContextIdCounterCall as alloy_sol_types::SolCall>::SIGNATURE,
            <confirmKmsEpochDestructionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getKmsGenThresholdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isValidKmsContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <destroyKmsContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <destroyKmsEpochCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getUserDecryptionThresholdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getPublicDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getKmsContextAnchorCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proposeCoprocessorUpgradeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isValidEpochForContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isLiveKmsContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getKmsNodesForContextCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for ProtocolConfigCalls {
        const NAME: &'static str = "ProtocolConfigCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 37usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::UPGRADE_INTERFACE_VERSION(_) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::confirmEpochActivation(_) => {
                    <confirmEpochActivationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::confirmKmsContextCreation(_) => {
                    <confirmKmsContextCreationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::confirmKmsContextDestruction(_) => {
                    <confirmKmsContextDestructionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::confirmKmsEpochDestruction(_) => {
                    <confirmKmsEpochDestructionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::defineNewEpochForCurrentKmsContext(_) => {
                    <defineNewEpochForCurrentKmsContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::defineNewKmsContextAndEpoch(_) => {
                    <defineNewKmsContextAndEpochCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::destroyKmsContext(_) => {
                    <destroyKmsContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::destroyKmsEpoch(_) => {
                    <destroyKmsEpochCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentKmsContextAndEpoch(_) => {
                    <getCurrentKmsContextAndEpochCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentKmsContextId(_) => {
                    <getCurrentKmsContextIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentKmsContextIdCounter(_) => {
                    <getCurrentKmsContextIdCounterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKmsContextAnchor(_) => {
                    <getKmsContextAnchorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKmsGenThreshold(_) => {
                    <getKmsGenThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKmsGenThresholdForContext(_) => {
                    <getKmsGenThresholdForContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKmsNodeForContext(_) => {
                    <getKmsNodeForContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKmsNodesForContext(_) => {
                    <getKmsNodesForContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKmsSigners(_) => {
                    <getKmsSignersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKmsSignersForContext(_) => {
                    <getKmsSignersForContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getMpcThreshold(_) => {
                    <getMpcThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getMpcThresholdForContext(_) => {
                    <getMpcThresholdForContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPublicDecryptionThreshold(_) => {
                    <getPublicDecryptionThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPublicDecryptionThresholdForContext(_) => {
                    <getPublicDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getUserDecryptionThreshold(_) => {
                    <getUserDecryptionThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getUserDecryptionThresholdForContext(_) => {
                    <getUserDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getVersion(_) => {
                    <getVersionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initializeFromEmptyProxy(_) => {
                    <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isKmsSigner(_) => {
                    <isKmsSignerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isKmsSignerForContext(_) => {
                    <isKmsSignerForContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isKmsTxSenderForContext(_) => {
                    <isKmsTxSenderForContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isLiveKmsContext(_) => {
                    <isLiveKmsContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isValidEpochForContext(_) => {
                    <isValidEpochForContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isValidKmsContext(_) => {
                    <isValidKmsContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proposeCoprocessorUpgrade(_) => {
                    <proposeCoprocessorUpgradeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proxiableUUID(_) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::SELECTOR
                }
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
            ) -> alloy_sol_types::Result<ProtocolConfigCalls>] = &[
                {
                    fn getVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getVersionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::getVersion)
                    }
                    getVersion
                },
                {
                    fn reinitializeV4(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <reinitializeV4Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::reinitializeV4)
                    }
                    reinitializeV4
                },
                {
                    fn defineNewEpochForCurrentKmsContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <defineNewEpochForCurrentKmsContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::defineNewEpochForCurrentKmsContext)
                    }
                    defineNewEpochForCurrentKmsContext
                },
                {
                    fn isKmsSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <isKmsSignerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::isKmsSigner)
                    }
                    isKmsSigner
                },
                {
                    fn initializeFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::initializeFromEmptyProxy)
                    }
                    initializeFromEmptyProxy
                },
                {
                    fn getMpcThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getMpcThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::getMpcThreshold)
                    }
                    getMpcThreshold
                },
                {
                    fn getUserDecryptionThresholdForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getUserDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigCalls::getUserDecryptionThresholdForContext,
                            )
                    }
                    getUserDecryptionThresholdForContext
                },
                {
                    fn getPublicDecryptionThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getPublicDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::getPublicDecryptionThreshold)
                    }
                    getPublicDecryptionThreshold
                },
                {
                    fn getKmsNodeForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getKmsNodeForContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::getKmsNodeForContext)
                    }
                    getKmsNodeForContext
                },
                {
                    fn getKmsGenThresholdForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getKmsGenThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::getKmsGenThresholdForContext)
                    }
                    getKmsGenThresholdForContext
                },
                {
                    fn isKmsTxSenderForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <isKmsTxSenderForContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::isKmsTxSenderForContext)
                    }
                    isKmsTxSenderForContext
                },
                {
                    fn getMpcThresholdForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getMpcThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::getMpcThresholdForContext)
                    }
                    getMpcThresholdForContext
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn getKmsSignersForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getKmsSignersForContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::getKmsSignersForContext)
                    }
                    getKmsSignersForContext
                },
                {
                    fn getCurrentKmsContextAndEpoch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getCurrentKmsContextAndEpochCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::getCurrentKmsContextAndEpoch)
                    }
                    getCurrentKmsContextAndEpoch
                },
                {
                    fn confirmKmsContextCreation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <confirmKmsContextCreationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::confirmKmsContextCreation)
                    }
                    confirmKmsContextCreation
                },
                {
                    fn confirmEpochActivation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <confirmEpochActivationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::confirmEpochActivation)
                    }
                    confirmEpochActivation
                },
                {
                    fn confirmKmsContextDestruction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <confirmKmsContextDestructionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::confirmKmsContextDestruction)
                    }
                    confirmKmsContextDestruction
                },
                {
                    fn getKmsSigners(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getKmsSignersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::getKmsSigners)
                    }
                    getKmsSigners
                },
                {
                    fn isKmsSignerForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <isKmsSignerForContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::isKmsSignerForContext)
                    }
                    isKmsSignerForContext
                },
                {
                    fn defineNewKmsContextAndEpoch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <defineNewKmsContextAndEpochCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::defineNewKmsContextAndEpoch)
                    }
                    defineNewKmsContextAndEpoch
                },
                {
                    fn getCurrentKmsContextId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getCurrentKmsContextIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::getCurrentKmsContextId)
                    }
                    getCurrentKmsContextId
                },
                {
                    fn getCurrentKmsContextIdCounter(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getCurrentKmsContextIdCounterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::getCurrentKmsContextIdCounter)
                    }
                    getCurrentKmsContextIdCounter
                },
                {
                    fn confirmKmsEpochDestruction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <confirmKmsEpochDestructionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::confirmKmsEpochDestruction)
                    }
                    confirmKmsEpochDestruction
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn getKmsGenThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getKmsGenThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::getKmsGenThreshold)
                    }
                    getKmsGenThreshold
                },
                {
                    fn isValidKmsContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <isValidKmsContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::isValidKmsContext)
                    }
                    isValidKmsContext
                },
                {
                    fn destroyKmsContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <destroyKmsContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::destroyKmsContext)
                    }
                    destroyKmsContext
                },
                {
                    fn destroyKmsEpoch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <destroyKmsEpochCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::destroyKmsEpoch)
                    }
                    destroyKmsEpoch
                },
                {
                    fn getUserDecryptionThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getUserDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::getUserDecryptionThreshold)
                    }
                    getUserDecryptionThreshold
                },
                {
                    fn getPublicDecryptionThresholdForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getPublicDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigCalls::getPublicDecryptionThresholdForContext,
                            )
                    }
                    getPublicDecryptionThresholdForContext
                },
                {
                    fn getKmsContextAnchor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getKmsContextAnchorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::getKmsContextAnchor)
                    }
                    getKmsContextAnchor
                },
                {
                    fn proposeCoprocessorUpgrade(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <proposeCoprocessorUpgradeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::proposeCoprocessorUpgrade)
                    }
                    proposeCoprocessorUpgrade
                },
                {
                    fn isValidEpochForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <isValidEpochForContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::isValidEpochForContext)
                    }
                    isValidEpochForContext
                },
                {
                    fn isLiveKmsContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <isLiveKmsContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::isLiveKmsContext)
                    }
                    isLiveKmsContext
                },
                {
                    fn getKmsNodesForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getKmsNodesForContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::getKmsNodesForContext)
                    }
                    getKmsNodesForContext
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
            ) -> alloy_sol_types::Result<ProtocolConfigCalls>] = &[
                {
                    fn getVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getVersionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::getVersion)
                    }
                    getVersion
                },
                {
                    fn reinitializeV4(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <reinitializeV4Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::reinitializeV4)
                    }
                    reinitializeV4
                },
                {
                    fn defineNewEpochForCurrentKmsContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <defineNewEpochForCurrentKmsContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::defineNewEpochForCurrentKmsContext)
                    }
                    defineNewEpochForCurrentKmsContext
                },
                {
                    fn isKmsSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <isKmsSignerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::isKmsSigner)
                    }
                    isKmsSigner
                },
                {
                    fn initializeFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::initializeFromEmptyProxy)
                    }
                    initializeFromEmptyProxy
                },
                {
                    fn getMpcThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getMpcThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::getMpcThreshold)
                    }
                    getMpcThreshold
                },
                {
                    fn getUserDecryptionThresholdForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getUserDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigCalls::getUserDecryptionThresholdForContext,
                            )
                    }
                    getUserDecryptionThresholdForContext
                },
                {
                    fn getPublicDecryptionThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getPublicDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::getPublicDecryptionThreshold)
                    }
                    getPublicDecryptionThreshold
                },
                {
                    fn getKmsNodeForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getKmsNodeForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::getKmsNodeForContext)
                    }
                    getKmsNodeForContext
                },
                {
                    fn getKmsGenThresholdForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getKmsGenThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::getKmsGenThresholdForContext)
                    }
                    getKmsGenThresholdForContext
                },
                {
                    fn isKmsTxSenderForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <isKmsTxSenderForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::isKmsTxSenderForContext)
                    }
                    isKmsTxSenderForContext
                },
                {
                    fn getMpcThresholdForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getMpcThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::getMpcThresholdForContext)
                    }
                    getMpcThresholdForContext
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn getKmsSignersForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getKmsSignersForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::getKmsSignersForContext)
                    }
                    getKmsSignersForContext
                },
                {
                    fn getCurrentKmsContextAndEpoch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getCurrentKmsContextAndEpochCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::getCurrentKmsContextAndEpoch)
                    }
                    getCurrentKmsContextAndEpoch
                },
                {
                    fn confirmKmsContextCreation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <confirmKmsContextCreationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::confirmKmsContextCreation)
                    }
                    confirmKmsContextCreation
                },
                {
                    fn confirmEpochActivation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <confirmEpochActivationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::confirmEpochActivation)
                    }
                    confirmEpochActivation
                },
                {
                    fn confirmKmsContextDestruction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <confirmKmsContextDestructionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::confirmKmsContextDestruction)
                    }
                    confirmKmsContextDestruction
                },
                {
                    fn getKmsSigners(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getKmsSignersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::getKmsSigners)
                    }
                    getKmsSigners
                },
                {
                    fn isKmsSignerForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <isKmsSignerForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::isKmsSignerForContext)
                    }
                    isKmsSignerForContext
                },
                {
                    fn defineNewKmsContextAndEpoch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <defineNewKmsContextAndEpochCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::defineNewKmsContextAndEpoch)
                    }
                    defineNewKmsContextAndEpoch
                },
                {
                    fn getCurrentKmsContextId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getCurrentKmsContextIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::getCurrentKmsContextId)
                    }
                    getCurrentKmsContextId
                },
                {
                    fn getCurrentKmsContextIdCounter(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getCurrentKmsContextIdCounterCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::getCurrentKmsContextIdCounter)
                    }
                    getCurrentKmsContextIdCounter
                },
                {
                    fn confirmKmsEpochDestruction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <confirmKmsEpochDestructionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::confirmKmsEpochDestruction)
                    }
                    confirmKmsEpochDestruction
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn getKmsGenThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getKmsGenThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::getKmsGenThreshold)
                    }
                    getKmsGenThreshold
                },
                {
                    fn isValidKmsContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <isValidKmsContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::isValidKmsContext)
                    }
                    isValidKmsContext
                },
                {
                    fn destroyKmsContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <destroyKmsContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::destroyKmsContext)
                    }
                    destroyKmsContext
                },
                {
                    fn destroyKmsEpoch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <destroyKmsEpochCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::destroyKmsEpoch)
                    }
                    destroyKmsEpoch
                },
                {
                    fn getUserDecryptionThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getUserDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::getUserDecryptionThreshold)
                    }
                    getUserDecryptionThreshold
                },
                {
                    fn getPublicDecryptionThresholdForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getPublicDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigCalls::getPublicDecryptionThresholdForContext,
                            )
                    }
                    getPublicDecryptionThresholdForContext
                },
                {
                    fn getKmsContextAnchor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getKmsContextAnchorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::getKmsContextAnchor)
                    }
                    getKmsContextAnchor
                },
                {
                    fn proposeCoprocessorUpgrade(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <proposeCoprocessorUpgradeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::proposeCoprocessorUpgrade)
                    }
                    proposeCoprocessorUpgrade
                },
                {
                    fn isValidEpochForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <isValidEpochForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::isValidEpochForContext)
                    }
                    isValidEpochForContext
                },
                {
                    fn isLiveKmsContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <isLiveKmsContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::isLiveKmsContext)
                    }
                    isLiveKmsContext
                },
                {
                    fn getKmsNodesForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getKmsNodesForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::getKmsNodesForContext)
                    }
                    getKmsNodesForContext
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
                Self::confirmEpochActivation(inner) => {
                    <confirmEpochActivationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::confirmKmsContextCreation(inner) => {
                    <confirmKmsContextCreationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::confirmKmsContextDestruction(inner) => {
                    <confirmKmsContextDestructionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::confirmKmsEpochDestruction(inner) => {
                    <confirmKmsEpochDestructionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::defineNewEpochForCurrentKmsContext(inner) => {
                    <defineNewEpochForCurrentKmsContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::defineNewKmsContextAndEpoch(inner) => {
                    <defineNewKmsContextAndEpochCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::destroyKmsContext(inner) => {
                    <destroyKmsContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::destroyKmsEpoch(inner) => {
                    <destroyKmsEpochCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCurrentKmsContextAndEpoch(inner) => {
                    <getCurrentKmsContextAndEpochCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCurrentKmsContextId(inner) => {
                    <getCurrentKmsContextIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCurrentKmsContextIdCounter(inner) => {
                    <getCurrentKmsContextIdCounterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getKmsContextAnchor(inner) => {
                    <getKmsContextAnchorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getKmsGenThreshold(inner) => {
                    <getKmsGenThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getKmsGenThresholdForContext(inner) => {
                    <getKmsGenThresholdForContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getKmsNodeForContext(inner) => {
                    <getKmsNodeForContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getKmsNodesForContext(inner) => {
                    <getKmsNodesForContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getKmsSigners(inner) => {
                    <getKmsSignersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getKmsSignersForContext(inner) => {
                    <getKmsSignersForContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getMpcThreshold(inner) => {
                    <getMpcThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getMpcThresholdForContext(inner) => {
                    <getMpcThresholdForContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getPublicDecryptionThreshold(inner) => {
                    <getPublicDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getPublicDecryptionThresholdForContext(inner) => {
                    <getPublicDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getUserDecryptionThreshold(inner) => {
                    <getUserDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getUserDecryptionThresholdForContext(inner) => {
                    <getUserDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::isKmsSigner(inner) => {
                    <isKmsSignerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isKmsSignerForContext(inner) => {
                    <isKmsSignerForContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isKmsTxSenderForContext(inner) => {
                    <isKmsTxSenderForContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isLiveKmsContext(inner) => {
                    <isLiveKmsContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isValidEpochForContext(inner) => {
                    <isValidEpochForContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isValidKmsContext(inner) => {
                    <isValidKmsContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proposeCoprocessorUpgrade(inner) => {
                    <proposeCoprocessorUpgradeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proxiableUUID(inner) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
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
                Self::confirmEpochActivation(inner) => {
                    <confirmEpochActivationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::confirmKmsContextCreation(inner) => {
                    <confirmKmsContextCreationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::confirmKmsContextDestruction(inner) => {
                    <confirmKmsContextDestructionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::confirmKmsEpochDestruction(inner) => {
                    <confirmKmsEpochDestructionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::defineNewEpochForCurrentKmsContext(inner) => {
                    <defineNewEpochForCurrentKmsContextCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::defineNewKmsContextAndEpoch(inner) => {
                    <defineNewKmsContextAndEpochCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::destroyKmsContext(inner) => {
                    <destroyKmsContextCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::destroyKmsEpoch(inner) => {
                    <destroyKmsEpochCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCurrentKmsContextAndEpoch(inner) => {
                    <getCurrentKmsContextAndEpochCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCurrentKmsContextId(inner) => {
                    <getCurrentKmsContextIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCurrentKmsContextIdCounter(inner) => {
                    <getCurrentKmsContextIdCounterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getKmsContextAnchor(inner) => {
                    <getKmsContextAnchorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getKmsGenThreshold(inner) => {
                    <getKmsGenThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getKmsGenThresholdForContext(inner) => {
                    <getKmsGenThresholdForContextCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getKmsNodeForContext(inner) => {
                    <getKmsNodeForContextCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getKmsNodesForContext(inner) => {
                    <getKmsNodesForContextCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getKmsSigners(inner) => {
                    <getKmsSignersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getKmsSignersForContext(inner) => {
                    <getKmsSignersForContextCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getMpcThreshold(inner) => {
                    <getMpcThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getMpcThresholdForContext(inner) => {
                    <getMpcThresholdForContextCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getPublicDecryptionThreshold(inner) => {
                    <getPublicDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getPublicDecryptionThresholdForContext(inner) => {
                    <getPublicDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getUserDecryptionThreshold(inner) => {
                    <getUserDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getUserDecryptionThresholdForContext(inner) => {
                    <getUserDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::isKmsSigner(inner) => {
                    <isKmsSignerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isKmsSignerForContext(inner) => {
                    <isKmsSignerForContextCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isKmsTxSenderForContext(inner) => {
                    <isKmsTxSenderForContextCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isLiveKmsContext(inner) => {
                    <isLiveKmsContextCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isValidEpochForContext(inner) => {
                    <isValidEpochForContextCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isValidKmsContext(inner) => {
                    <isValidKmsContextCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proposeCoprocessorUpgrade(inner) => {
                    <proposeCoprocessorUpgradeCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`ProtocolConfig`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum ProtocolConfigErrors {
        #[allow(missing_docs)]
        AddressEmptyCode(AddressEmptyCode),
        #[allow(missing_docs)]
        DuplicateChainId(DuplicateChainId),
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
        EmptyChainUpgradeWindows(EmptyChainUpgradeWindows),
        #[allow(missing_docs)]
        EmptyEpochActivationResults(EmptyEpochActivationResults),
        #[allow(missing_docs)]
        EmptyKmsNodes(EmptyKmsNodes),
        #[allow(missing_docs)]
        EmptySoftwareVersion(EmptySoftwareVersion),
        #[allow(missing_docs)]
        EpochActivationAlreadyConfirmed(EpochActivationAlreadyConfirmed),
        #[allow(missing_docs)]
        EpochActivationUnauthorized(EpochActivationUnauthorized),
        #[allow(missing_docs)]
        FailedCall(FailedCall),
        #[allow(missing_docs)]
        InvalidBlockWindow(InvalidBlockWindow),
        #[allow(missing_docs)]
        InvalidHighThreshold(InvalidHighThreshold),
        #[allow(missing_docs)]
        InvalidInitialization(InvalidInitialization),
        #[allow(missing_docs)]
        InvalidKmsContext(InvalidKmsContext),
        #[allow(missing_docs)]
        InvalidKmsEpoch(InvalidKmsEpoch),
        #[allow(missing_docs)]
        InvalidNullThreshold(InvalidNullThreshold),
        #[allow(missing_docs)]
        InvalidProposalId(InvalidProposalId),
        #[allow(missing_docs)]
        KmsContextCreationAlreadyConfirmed(KmsContextCreationAlreadyConfirmed),
        #[allow(missing_docs)]
        KmsContextCreationUnauthorized(KmsContextCreationUnauthorized),
        #[allow(missing_docs)]
        KmsContextNotDestroyed(KmsContextNotDestroyed),
        #[allow(missing_docs)]
        KmsContextNotPending(KmsContextNotPending),
        #[allow(missing_docs)]
        KmsDestructionAlreadyConfirmed(KmsDestructionAlreadyConfirmed),
        #[allow(missing_docs)]
        KmsDestructionUnauthorized(KmsDestructionUnauthorized),
        #[allow(missing_docs)]
        KmsEpochNotDestroyed(KmsEpochNotDestroyed),
        #[allow(missing_docs)]
        KmsLifecycleOperationInFlight(KmsLifecycleOperationInFlight),
        #[allow(missing_docs)]
        KmsNodeNullSigner(KmsNodeNullSigner),
        #[allow(missing_docs)]
        KmsNodeNullTxSender(KmsNodeNullTxSender),
        #[allow(missing_docs)]
        KmsSignerAlreadyRegistered(KmsSignerAlreadyRegistered),
        #[allow(missing_docs)]
        KmsSignerSetExceedsProofFormatLimit(KmsSignerSetExceedsProofFormatLimit),
        #[allow(missing_docs)]
        KmsTxSenderAlreadyRegistered(KmsTxSenderAlreadyRegistered),
        #[allow(missing_docs)]
        LatestActiveKmsContextCannotBeDestroyed(LatestActiveKmsContextCannotBeDestroyed),
        #[allow(missing_docs)]
        LatestActiveKmsEpochCannotBeDestroyed(LatestActiveKmsEpochCannotBeDestroyed),
        #[allow(missing_docs)]
        NonIncreasingKmsContextId(NonIncreasingKmsContextId),
        #[allow(missing_docs)]
        NotHostOwner(NotHostOwner),
        #[allow(missing_docs)]
        NotInitializing(NotInitializing),
        #[allow(missing_docs)]
        NotInitializingFromEmptyProxy(NotInitializingFromEmptyProxy),
        #[allow(missing_docs)]
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        #[allow(missing_docs)]
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
        #[allow(missing_docs)]
        ZeroChainId(ZeroChainId),
        #[allow(missing_docs)]
        ZeroGwStartBlock(ZeroGwStartBlock),
    }
    impl ProtocolConfigErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [3u8, 216u8, 180u8, 208u8],
            [6u8, 140u8, 141u8, 64u8],
            [7u8, 128u8, 32u8, 190u8],
            [9u8, 146u8, 247u8, 173u8],
            [22u8, 167u8, 39u8, 120u8],
            [23u8, 211u8, 233u8, 72u8],
            [24u8, 122u8, 234u8, 168u8],
            [33u8, 191u8, 218u8, 16u8],
            [43u8, 115u8, 217u8, 183u8],
            [45u8, 236u8, 207u8, 77u8],
            [53u8, 134u8, 239u8, 161u8],
            [54u8, 191u8, 182u8, 14u8],
            [73u8, 65u8, 118u8, 54u8],
            [76u8, 156u8, 140u8, 227u8],
            [98u8, 21u8, 105u8, 228u8],
            [98u8, 88u8, 92u8, 200u8],
            [108u8, 103u8, 228u8, 112u8],
            [111u8, 79u8, 115u8, 31u8],
            [119u8, 221u8, 190u8, 129u8],
            [121u8, 149u8, 187u8, 207u8],
            [132u8, 102u8, 128u8, 74u8],
            [153u8, 150u8, 179u8, 21u8],
            [157u8, 39u8, 153u8, 233u8],
            [163u8, 244u8, 175u8, 235u8],
            [170u8, 29u8, 73u8, 164u8],
            [179u8, 152u8, 151u8, 159u8],
            [181u8, 72u8, 145u8, 71u8],
            [184u8, 29u8, 248u8, 232u8],
            [190u8, 80u8, 80u8, 68u8],
            [200u8, 72u8, 133u8, 212u8],
            [202u8, 168u8, 20u8, 163u8],
            [209u8, 140u8, 79u8, 240u8],
            [214u8, 40u8, 178u8, 72u8],
            [214u8, 189u8, 162u8, 117u8],
            [215u8, 139u8, 206u8, 12u8],
            [215u8, 230u8, 188u8, 248u8],
            [224u8, 124u8, 141u8, 186u8],
            [239u8, 213u8, 95u8, 103u8],
            [240u8, 231u8, 129u8, 193u8],
            [242u8, 25u8, 220u8, 14u8],
            [245u8, 26u8, 246u8, 187u8],
            [246u8, 69u8, 238u8, 223u8],
            [249u8, 46u8, 232u8, 169u8],
            [252u8, 230u8, 152u8, 247u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(KmsEpochNotDestroyed),
            ::core::stringify!(EmptyKmsNodes),
            ::core::stringify!(KmsLifecycleOperationInFlight),
            ::core::stringify!(InvalidProposalId),
            ::core::stringify!(KmsSignerSetExceedsProofFormatLimit),
            ::core::stringify!(ZeroGwStartBlock),
            ::core::stringify!(LatestActiveKmsContextCannotBeDestroyed),
            ::core::stringify!(NotHostOwner),
            ::core::stringify!(KmsContextNotDestroyed),
            ::core::stringify!(KmsNodeNullSigner),
            ::core::stringify!(KmsContextNotPending),
            ::core::stringify!(InvalidNullThreshold),
            ::core::stringify!(EpochActivationAlreadyConfirmed),
            ::core::stringify!(ERC1967InvalidImplementation),
            ::core::stringify!(EmptyEpochActivationResults),
            ::core::stringify!(KmsContextCreationAlreadyConfirmed),
            ::core::stringify!(DuplicateChainId),
            ::core::stringify!(NotInitializingFromEmptyProxy),
            ::core::stringify!(InvalidKmsContext),
            ::core::stringify!(InvalidKmsEpoch),
            ::core::stringify!(KmsNodeNullTxSender),
            ::core::stringify!(AddressEmptyCode),
            ::core::stringify!(KmsDestructionAlreadyConfirmed),
            ::core::stringify!(EpochActivationUnauthorized),
            ::core::stringify!(UUPSUnsupportedProxiableUUID),
            ::core::stringify!(ERC1967NonPayable),
            ::core::stringify!(EmptySoftwareVersion),
            ::core::stringify!(KmsContextCreationUnauthorized),
            ::core::stringify!(EmptyChainUpgradeWindows),
            ::core::stringify!(ZeroChainId),
            ::core::stringify!(InvalidHighThreshold),
            ::core::stringify!(KmsTxSenderAlreadyRegistered),
            ::core::stringify!(KmsDestructionUnauthorized),
            ::core::stringify!(FailedCall),
            ::core::stringify!(ECDSAInvalidSignatureS),
            ::core::stringify!(NotInitializing),
            ::core::stringify!(UUPSUnauthorizedCallContext),
            ::core::stringify!(NonIncreasingKmsContextId),
            ::core::stringify!(LatestActiveKmsEpochCannotBeDestroyed),
            ::core::stringify!(InvalidBlockWindow),
            ::core::stringify!(KmsSignerAlreadyRegistered),
            ::core::stringify!(ECDSAInvalidSignature),
            ::core::stringify!(InvalidInitialization),
            ::core::stringify!(ECDSAInvalidSignatureLength),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <KmsEpochNotDestroyed as alloy_sol_types::SolError>::SIGNATURE,
            <EmptyKmsNodes as alloy_sol_types::SolError>::SIGNATURE,
            <KmsLifecycleOperationInFlight as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidProposalId as alloy_sol_types::SolError>::SIGNATURE,
            <KmsSignerSetExceedsProofFormatLimit as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroGwStartBlock as alloy_sol_types::SolError>::SIGNATURE,
            <LatestActiveKmsContextCannotBeDestroyed as alloy_sol_types::SolError>::SIGNATURE,
            <NotHostOwner as alloy_sol_types::SolError>::SIGNATURE,
            <KmsContextNotDestroyed as alloy_sol_types::SolError>::SIGNATURE,
            <KmsNodeNullSigner as alloy_sol_types::SolError>::SIGNATURE,
            <KmsContextNotPending as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidNullThreshold as alloy_sol_types::SolError>::SIGNATURE,
            <EpochActivationAlreadyConfirmed as alloy_sol_types::SolError>::SIGNATURE,
            <ERC1967InvalidImplementation as alloy_sol_types::SolError>::SIGNATURE,
            <EmptyEpochActivationResults as alloy_sol_types::SolError>::SIGNATURE,
            <KmsContextCreationAlreadyConfirmed as alloy_sol_types::SolError>::SIGNATURE,
            <DuplicateChainId as alloy_sol_types::SolError>::SIGNATURE,
            <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidKmsContext as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidKmsEpoch as alloy_sol_types::SolError>::SIGNATURE,
            <KmsNodeNullTxSender as alloy_sol_types::SolError>::SIGNATURE,
            <AddressEmptyCode as alloy_sol_types::SolError>::SIGNATURE,
            <KmsDestructionAlreadyConfirmed as alloy_sol_types::SolError>::SIGNATURE,
            <EpochActivationUnauthorized as alloy_sol_types::SolError>::SIGNATURE,
            <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::SIGNATURE,
            <ERC1967NonPayable as alloy_sol_types::SolError>::SIGNATURE,
            <EmptySoftwareVersion as alloy_sol_types::SolError>::SIGNATURE,
            <KmsContextCreationUnauthorized as alloy_sol_types::SolError>::SIGNATURE,
            <EmptyChainUpgradeWindows as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroChainId as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidHighThreshold as alloy_sol_types::SolError>::SIGNATURE,
            <KmsTxSenderAlreadyRegistered as alloy_sol_types::SolError>::SIGNATURE,
            <KmsDestructionUnauthorized as alloy_sol_types::SolError>::SIGNATURE,
            <FailedCall as alloy_sol_types::SolError>::SIGNATURE,
            <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::SIGNATURE,
            <NotInitializing as alloy_sol_types::SolError>::SIGNATURE,
            <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::SIGNATURE,
            <NonIncreasingKmsContextId as alloy_sol_types::SolError>::SIGNATURE,
            <LatestActiveKmsEpochCannotBeDestroyed as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidBlockWindow as alloy_sol_types::SolError>::SIGNATURE,
            <KmsSignerAlreadyRegistered as alloy_sol_types::SolError>::SIGNATURE,
            <ECDSAInvalidSignature as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidInitialization as alloy_sol_types::SolError>::SIGNATURE,
            <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for ProtocolConfigErrors {
        const NAME: &'static str = "ProtocolConfigErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 44usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AddressEmptyCode(_) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DuplicateChainId(_) => {
                    <DuplicateChainId as alloy_sol_types::SolError>::SELECTOR
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
                Self::EmptyChainUpgradeWindows(_) => {
                    <EmptyChainUpgradeWindows as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyEpochActivationResults(_) => {
                    <EmptyEpochActivationResults as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyKmsNodes(_) => {
                    <EmptyKmsNodes as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptySoftwareVersion(_) => {
                    <EmptySoftwareVersion as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EpochActivationAlreadyConfirmed(_) => {
                    <EpochActivationAlreadyConfirmed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EpochActivationUnauthorized(_) => {
                    <EpochActivationUnauthorized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FailedCall(_) => {
                    <FailedCall as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidBlockWindow(_) => {
                    <InvalidBlockWindow as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidHighThreshold(_) => {
                    <InvalidHighThreshold as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidInitialization(_) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidKmsContext(_) => {
                    <InvalidKmsContext as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidKmsEpoch(_) => {
                    <InvalidKmsEpoch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidNullThreshold(_) => {
                    <InvalidNullThreshold as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidProposalId(_) => {
                    <InvalidProposalId as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsContextCreationAlreadyConfirmed(_) => {
                    <KmsContextCreationAlreadyConfirmed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsContextCreationUnauthorized(_) => {
                    <KmsContextCreationUnauthorized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsContextNotDestroyed(_) => {
                    <KmsContextNotDestroyed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsContextNotPending(_) => {
                    <KmsContextNotPending as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsDestructionAlreadyConfirmed(_) => {
                    <KmsDestructionAlreadyConfirmed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsDestructionUnauthorized(_) => {
                    <KmsDestructionUnauthorized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsEpochNotDestroyed(_) => {
                    <KmsEpochNotDestroyed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsLifecycleOperationInFlight(_) => {
                    <KmsLifecycleOperationInFlight as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsNodeNullSigner(_) => {
                    <KmsNodeNullSigner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsNodeNullTxSender(_) => {
                    <KmsNodeNullTxSender as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsSignerAlreadyRegistered(_) => {
                    <KmsSignerAlreadyRegistered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsSignerSetExceedsProofFormatLimit(_) => {
                    <KmsSignerSetExceedsProofFormatLimit as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsTxSenderAlreadyRegistered(_) => {
                    <KmsTxSenderAlreadyRegistered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LatestActiveKmsContextCannotBeDestroyed(_) => {
                    <LatestActiveKmsContextCannotBeDestroyed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LatestActiveKmsEpochCannotBeDestroyed(_) => {
                    <LatestActiveKmsEpochCannotBeDestroyed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NonIncreasingKmsContextId(_) => {
                    <NonIncreasingKmsContextId as alloy_sol_types::SolError>::SELECTOR
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
                Self::UUPSUnauthorizedCallContext(_) => {
                    <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UUPSUnsupportedProxiableUUID(_) => {
                    <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroChainId(_) => {
                    <ZeroChainId as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroGwStartBlock(_) => {
                    <ZeroGwStartBlock as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<ProtocolConfigErrors>] = &[
                {
                    fn KmsEpochNotDestroyed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsEpochNotDestroyed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::KmsEpochNotDestroyed)
                    }
                    KmsEpochNotDestroyed
                },
                {
                    fn EmptyKmsNodes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <EmptyKmsNodes as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::EmptyKmsNodes)
                    }
                    EmptyKmsNodes
                },
                {
                    fn KmsLifecycleOperationInFlight(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsLifecycleOperationInFlight as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::KmsLifecycleOperationInFlight)
                    }
                    KmsLifecycleOperationInFlight
                },
                {
                    fn InvalidProposalId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <InvalidProposalId as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::InvalidProposalId)
                    }
                    InvalidProposalId
                },
                {
                    fn KmsSignerSetExceedsProofFormatLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsSignerSetExceedsProofFormatLimit as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigErrors::KmsSignerSetExceedsProofFormatLimit,
                            )
                    }
                    KmsSignerSetExceedsProofFormatLimit
                },
                {
                    fn ZeroGwStartBlock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <ZeroGwStartBlock as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::ZeroGwStartBlock)
                    }
                    ZeroGwStartBlock
                },
                {
                    fn LatestActiveKmsContextCannotBeDestroyed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <LatestActiveKmsContextCannotBeDestroyed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigErrors::LatestActiveKmsContextCannotBeDestroyed,
                            )
                    }
                    LatestActiveKmsContextCannotBeDestroyed
                },
                {
                    fn NotHostOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <NotHostOwner as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ProtocolConfigErrors::NotHostOwner)
                    }
                    NotHostOwner
                },
                {
                    fn KmsContextNotDestroyed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsContextNotDestroyed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::KmsContextNotDestroyed)
                    }
                    KmsContextNotDestroyed
                },
                {
                    fn KmsNodeNullSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsNodeNullSigner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::KmsNodeNullSigner)
                    }
                    KmsNodeNullSigner
                },
                {
                    fn KmsContextNotPending(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsContextNotPending as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::KmsContextNotPending)
                    }
                    KmsContextNotPending
                },
                {
                    fn InvalidNullThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <InvalidNullThreshold as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::InvalidNullThreshold)
                    }
                    InvalidNullThreshold
                },
                {
                    fn EpochActivationAlreadyConfirmed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <EpochActivationAlreadyConfirmed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::EpochActivationAlreadyConfirmed)
                    }
                    EpochActivationAlreadyConfirmed
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn EmptyEpochActivationResults(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <EmptyEpochActivationResults as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::EmptyEpochActivationResults)
                    }
                    EmptyEpochActivationResults
                },
                {
                    fn KmsContextCreationAlreadyConfirmed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsContextCreationAlreadyConfirmed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigErrors::KmsContextCreationAlreadyConfirmed,
                            )
                    }
                    KmsContextCreationAlreadyConfirmed
                },
                {
                    fn DuplicateChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <DuplicateChainId as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::DuplicateChainId)
                    }
                    DuplicateChainId
                },
                {
                    fn NotInitializingFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::NotInitializingFromEmptyProxy)
                    }
                    NotInitializingFromEmptyProxy
                },
                {
                    fn InvalidKmsContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <InvalidKmsContext as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::InvalidKmsContext)
                    }
                    InvalidKmsContext
                },
                {
                    fn InvalidKmsEpoch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <InvalidKmsEpoch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::InvalidKmsEpoch)
                    }
                    InvalidKmsEpoch
                },
                {
                    fn KmsNodeNullTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsNodeNullTxSender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::KmsNodeNullTxSender)
                    }
                    KmsNodeNullTxSender
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn KmsDestructionAlreadyConfirmed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsDestructionAlreadyConfirmed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::KmsDestructionAlreadyConfirmed)
                    }
                    KmsDestructionAlreadyConfirmed
                },
                {
                    fn EpochActivationUnauthorized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <EpochActivationUnauthorized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::EpochActivationUnauthorized)
                    }
                    EpochActivationUnauthorized
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn EmptySoftwareVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <EmptySoftwareVersion as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::EmptySoftwareVersion)
                    }
                    EmptySoftwareVersion
                },
                {
                    fn KmsContextCreationUnauthorized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsContextCreationUnauthorized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::KmsContextCreationUnauthorized)
                    }
                    KmsContextCreationUnauthorized
                },
                {
                    fn EmptyChainUpgradeWindows(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <EmptyChainUpgradeWindows as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::EmptyChainUpgradeWindows)
                    }
                    EmptyChainUpgradeWindows
                },
                {
                    fn ZeroChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <ZeroChainId as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ProtocolConfigErrors::ZeroChainId)
                    }
                    ZeroChainId
                },
                {
                    fn InvalidHighThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <InvalidHighThreshold as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::InvalidHighThreshold)
                    }
                    InvalidHighThreshold
                },
                {
                    fn KmsTxSenderAlreadyRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsTxSenderAlreadyRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::KmsTxSenderAlreadyRegistered)
                    }
                    KmsTxSenderAlreadyRegistered
                },
                {
                    fn KmsDestructionUnauthorized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsDestructionUnauthorized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::KmsDestructionUnauthorized)
                    }
                    KmsDestructionUnauthorized
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ProtocolConfigErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn NonIncreasingKmsContextId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <NonIncreasingKmsContextId as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::NonIncreasingKmsContextId)
                    }
                    NonIncreasingKmsContextId
                },
                {
                    fn LatestActiveKmsEpochCannotBeDestroyed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <LatestActiveKmsEpochCannotBeDestroyed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigErrors::LatestActiveKmsEpochCannotBeDestroyed,
                            )
                    }
                    LatestActiveKmsEpochCannotBeDestroyed
                },
                {
                    fn InvalidBlockWindow(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <InvalidBlockWindow as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::InvalidBlockWindow)
                    }
                    InvalidBlockWindow
                },
                {
                    fn KmsSignerAlreadyRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsSignerAlreadyRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::KmsSignerAlreadyRegistered)
                    }
                    KmsSignerAlreadyRegistered
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::InvalidInitialization)
                    }
                    InvalidInitialization
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::ECDSAInvalidSignatureLength)
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
            ) -> alloy_sol_types::Result<ProtocolConfigErrors>] = &[
                {
                    fn KmsEpochNotDestroyed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsEpochNotDestroyed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::KmsEpochNotDestroyed)
                    }
                    KmsEpochNotDestroyed
                },
                {
                    fn EmptyKmsNodes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <EmptyKmsNodes as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::EmptyKmsNodes)
                    }
                    EmptyKmsNodes
                },
                {
                    fn KmsLifecycleOperationInFlight(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsLifecycleOperationInFlight as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::KmsLifecycleOperationInFlight)
                    }
                    KmsLifecycleOperationInFlight
                },
                {
                    fn InvalidProposalId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <InvalidProposalId as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::InvalidProposalId)
                    }
                    InvalidProposalId
                },
                {
                    fn KmsSignerSetExceedsProofFormatLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsSignerSetExceedsProofFormatLimit as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigErrors::KmsSignerSetExceedsProofFormatLimit,
                            )
                    }
                    KmsSignerSetExceedsProofFormatLimit
                },
                {
                    fn ZeroGwStartBlock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <ZeroGwStartBlock as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::ZeroGwStartBlock)
                    }
                    ZeroGwStartBlock
                },
                {
                    fn LatestActiveKmsContextCannotBeDestroyed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <LatestActiveKmsContextCannotBeDestroyed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigErrors::LatestActiveKmsContextCannotBeDestroyed,
                            )
                    }
                    LatestActiveKmsContextCannotBeDestroyed
                },
                {
                    fn NotHostOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <NotHostOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::NotHostOwner)
                    }
                    NotHostOwner
                },
                {
                    fn KmsContextNotDestroyed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsContextNotDestroyed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::KmsContextNotDestroyed)
                    }
                    KmsContextNotDestroyed
                },
                {
                    fn KmsNodeNullSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsNodeNullSigner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::KmsNodeNullSigner)
                    }
                    KmsNodeNullSigner
                },
                {
                    fn KmsContextNotPending(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsContextNotPending as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::KmsContextNotPending)
                    }
                    KmsContextNotPending
                },
                {
                    fn InvalidNullThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <InvalidNullThreshold as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::InvalidNullThreshold)
                    }
                    InvalidNullThreshold
                },
                {
                    fn EpochActivationAlreadyConfirmed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <EpochActivationAlreadyConfirmed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::EpochActivationAlreadyConfirmed)
                    }
                    EpochActivationAlreadyConfirmed
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn EmptyEpochActivationResults(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <EmptyEpochActivationResults as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::EmptyEpochActivationResults)
                    }
                    EmptyEpochActivationResults
                },
                {
                    fn KmsContextCreationAlreadyConfirmed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsContextCreationAlreadyConfirmed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigErrors::KmsContextCreationAlreadyConfirmed,
                            )
                    }
                    KmsContextCreationAlreadyConfirmed
                },
                {
                    fn DuplicateChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <DuplicateChainId as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::DuplicateChainId)
                    }
                    DuplicateChainId
                },
                {
                    fn NotInitializingFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::NotInitializingFromEmptyProxy)
                    }
                    NotInitializingFromEmptyProxy
                },
                {
                    fn InvalidKmsContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <InvalidKmsContext as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::InvalidKmsContext)
                    }
                    InvalidKmsContext
                },
                {
                    fn InvalidKmsEpoch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <InvalidKmsEpoch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::InvalidKmsEpoch)
                    }
                    InvalidKmsEpoch
                },
                {
                    fn KmsNodeNullTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsNodeNullTxSender as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::KmsNodeNullTxSender)
                    }
                    KmsNodeNullTxSender
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn KmsDestructionAlreadyConfirmed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsDestructionAlreadyConfirmed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::KmsDestructionAlreadyConfirmed)
                    }
                    KmsDestructionAlreadyConfirmed
                },
                {
                    fn EpochActivationUnauthorized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <EpochActivationUnauthorized as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::EpochActivationUnauthorized)
                    }
                    EpochActivationUnauthorized
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn EmptySoftwareVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <EmptySoftwareVersion as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::EmptySoftwareVersion)
                    }
                    EmptySoftwareVersion
                },
                {
                    fn KmsContextCreationUnauthorized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsContextCreationUnauthorized as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::KmsContextCreationUnauthorized)
                    }
                    KmsContextCreationUnauthorized
                },
                {
                    fn EmptyChainUpgradeWindows(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <EmptyChainUpgradeWindows as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::EmptyChainUpgradeWindows)
                    }
                    EmptyChainUpgradeWindows
                },
                {
                    fn ZeroChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <ZeroChainId as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::ZeroChainId)
                    }
                    ZeroChainId
                },
                {
                    fn InvalidHighThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <InvalidHighThreshold as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::InvalidHighThreshold)
                    }
                    InvalidHighThreshold
                },
                {
                    fn KmsTxSenderAlreadyRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsTxSenderAlreadyRegistered as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::KmsTxSenderAlreadyRegistered)
                    }
                    KmsTxSenderAlreadyRegistered
                },
                {
                    fn KmsDestructionUnauthorized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsDestructionUnauthorized as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::KmsDestructionUnauthorized)
                    }
                    KmsDestructionUnauthorized
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn NonIncreasingKmsContextId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <NonIncreasingKmsContextId as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::NonIncreasingKmsContextId)
                    }
                    NonIncreasingKmsContextId
                },
                {
                    fn LatestActiveKmsEpochCannotBeDestroyed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <LatestActiveKmsEpochCannotBeDestroyed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigErrors::LatestActiveKmsEpochCannotBeDestroyed,
                            )
                    }
                    LatestActiveKmsEpochCannotBeDestroyed
                },
                {
                    fn InvalidBlockWindow(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <InvalidBlockWindow as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::InvalidBlockWindow)
                    }
                    InvalidBlockWindow
                },
                {
                    fn KmsSignerAlreadyRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsSignerAlreadyRegistered as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::KmsSignerAlreadyRegistered)
                    }
                    KmsSignerAlreadyRegistered
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::InvalidInitialization)
                    }
                    InvalidInitialization
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::ECDSAInvalidSignatureLength)
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
                Self::DuplicateChainId(inner) => {
                    <DuplicateChainId as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::EmptyChainUpgradeWindows(inner) => {
                    <EmptyChainUpgradeWindows as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EmptyEpochActivationResults(inner) => {
                    <EmptyEpochActivationResults as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EmptyKmsNodes(inner) => {
                    <EmptyKmsNodes as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::EmptySoftwareVersion(inner) => {
                    <EmptySoftwareVersion as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EpochActivationAlreadyConfirmed(inner) => {
                    <EpochActivationAlreadyConfirmed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EpochActivationUnauthorized(inner) => {
                    <EpochActivationUnauthorized as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidBlockWindow(inner) => {
                    <InvalidBlockWindow as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidHighThreshold(inner) => {
                    <InvalidHighThreshold as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidInitialization(inner) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidKmsContext(inner) => {
                    <InvalidKmsContext as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidKmsEpoch(inner) => {
                    <InvalidKmsEpoch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidNullThreshold(inner) => {
                    <InvalidNullThreshold as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidProposalId(inner) => {
                    <InvalidProposalId as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KmsContextCreationAlreadyConfirmed(inner) => {
                    <KmsContextCreationAlreadyConfirmed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KmsContextCreationUnauthorized(inner) => {
                    <KmsContextCreationUnauthorized as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KmsContextNotDestroyed(inner) => {
                    <KmsContextNotDestroyed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KmsContextNotPending(inner) => {
                    <KmsContextNotPending as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KmsDestructionAlreadyConfirmed(inner) => {
                    <KmsDestructionAlreadyConfirmed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KmsDestructionUnauthorized(inner) => {
                    <KmsDestructionUnauthorized as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KmsEpochNotDestroyed(inner) => {
                    <KmsEpochNotDestroyed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KmsLifecycleOperationInFlight(inner) => {
                    <KmsLifecycleOperationInFlight as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KmsNodeNullSigner(inner) => {
                    <KmsNodeNullSigner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KmsNodeNullTxSender(inner) => {
                    <KmsNodeNullTxSender as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KmsSignerAlreadyRegistered(inner) => {
                    <KmsSignerAlreadyRegistered as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KmsSignerSetExceedsProofFormatLimit(inner) => {
                    <KmsSignerSetExceedsProofFormatLimit as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KmsTxSenderAlreadyRegistered(inner) => {
                    <KmsTxSenderAlreadyRegistered as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LatestActiveKmsContextCannotBeDestroyed(inner) => {
                    <LatestActiveKmsContextCannotBeDestroyed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LatestActiveKmsEpochCannotBeDestroyed(inner) => {
                    <LatestActiveKmsEpochCannotBeDestroyed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NonIncreasingKmsContextId(inner) => {
                    <NonIncreasingKmsContextId as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::ZeroChainId(inner) => {
                    <ZeroChainId as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ZeroGwStartBlock(inner) => {
                    <ZeroGwStartBlock as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::DuplicateChainId(inner) => {
                    <DuplicateChainId as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::EmptyChainUpgradeWindows(inner) => {
                    <EmptyChainUpgradeWindows as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EmptyEpochActivationResults(inner) => {
                    <EmptyEpochActivationResults as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EmptyKmsNodes(inner) => {
                    <EmptyKmsNodes as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EmptySoftwareVersion(inner) => {
                    <EmptySoftwareVersion as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EpochActivationAlreadyConfirmed(inner) => {
                    <EpochActivationAlreadyConfirmed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EpochActivationUnauthorized(inner) => {
                    <EpochActivationUnauthorized as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidBlockWindow(inner) => {
                    <InvalidBlockWindow as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidHighThreshold(inner) => {
                    <InvalidHighThreshold as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidKmsContext(inner) => {
                    <InvalidKmsContext as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidKmsEpoch(inner) => {
                    <InvalidKmsEpoch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidNullThreshold(inner) => {
                    <InvalidNullThreshold as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidProposalId(inner) => {
                    <InvalidProposalId as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KmsContextCreationAlreadyConfirmed(inner) => {
                    <KmsContextCreationAlreadyConfirmed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KmsContextCreationUnauthorized(inner) => {
                    <KmsContextCreationUnauthorized as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KmsContextNotDestroyed(inner) => {
                    <KmsContextNotDestroyed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KmsContextNotPending(inner) => {
                    <KmsContextNotPending as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KmsDestructionAlreadyConfirmed(inner) => {
                    <KmsDestructionAlreadyConfirmed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KmsDestructionUnauthorized(inner) => {
                    <KmsDestructionUnauthorized as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KmsEpochNotDestroyed(inner) => {
                    <KmsEpochNotDestroyed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KmsLifecycleOperationInFlight(inner) => {
                    <KmsLifecycleOperationInFlight as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KmsNodeNullSigner(inner) => {
                    <KmsNodeNullSigner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KmsNodeNullTxSender(inner) => {
                    <KmsNodeNullTxSender as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KmsSignerAlreadyRegistered(inner) => {
                    <KmsSignerAlreadyRegistered as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KmsSignerSetExceedsProofFormatLimit(inner) => {
                    <KmsSignerSetExceedsProofFormatLimit as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KmsTxSenderAlreadyRegistered(inner) => {
                    <KmsTxSenderAlreadyRegistered as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LatestActiveKmsContextCannotBeDestroyed(inner) => {
                    <LatestActiveKmsContextCannotBeDestroyed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LatestActiveKmsEpochCannotBeDestroyed(inner) => {
                    <LatestActiveKmsEpochCannotBeDestroyed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NonIncreasingKmsContextId(inner) => {
                    <NonIncreasingKmsContextId as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::ZeroChainId(inner) => {
                    <ZeroChainId as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ZeroGwStartBlock(inner) => {
                    <ZeroGwStartBlock as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`ProtocolConfig`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum ProtocolConfigEvents {
        #[allow(missing_docs)]
        ActivateEpoch(ActivateEpoch),
        #[allow(missing_docs)]
        CoprocessorUpgradeProposed(CoprocessorUpgradeProposed),
        #[allow(missing_docs)]
        EpochActivationConfirmation(EpochActivationConfirmation),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        KmsContextCreationConfirmation(KmsContextCreationConfirmation),
        #[allow(missing_docs)]
        KmsContextDestroyed(KmsContextDestroyed),
        #[allow(missing_docs)]
        KmsContextDestructionConfirmation(KmsContextDestructionConfirmation),
        #[allow(missing_docs)]
        KmsEpochDestroyed(KmsEpochDestroyed),
        #[allow(missing_docs)]
        KmsEpochDestructionConfirmation(KmsEpochDestructionConfirmation),
        #[allow(missing_docs)]
        NewKmsContext(NewKmsContext),
        #[allow(missing_docs)]
        NewKmsEpoch(NewKmsEpoch),
        #[allow(missing_docs)]
        Upgraded(Upgraded),
    }
    impl ProtocolConfigEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                3u8, 67u8, 96u8, 19u8, 7u8, 95u8, 141u8, 26u8, 187u8, 23u8, 129u8, 219u8,
                202u8, 244u8, 24u8, 252u8, 118u8, 252u8, 121u8, 125u8, 26u8, 181u8, 28u8,
                56u8, 102u8, 76u8, 26u8, 81u8, 243u8, 205u8, 87u8, 249u8,
            ],
            [
                21u8, 170u8, 175u8, 71u8, 94u8, 244u8, 7u8, 84u8, 63u8, 81u8, 100u8,
                245u8, 125u8, 207u8, 87u8, 247u8, 249u8, 56u8, 22u8, 245u8, 91u8, 174u8,
                119u8, 202u8, 9u8, 239u8, 196u8, 69u8, 186u8, 64u8, 238u8, 247u8,
            ],
            [
                32u8, 77u8, 107u8, 128u8, 18u8, 17u8, 84u8, 205u8, 135u8, 217u8, 156u8,
                245u8, 76u8, 99u8, 154u8, 61u8, 208u8, 165u8, 59u8, 48u8, 132u8, 39u8,
                112u8, 152u8, 222u8, 151u8, 46u8, 189u8, 211u8, 76u8, 107u8, 233u8,
            ],
            [
                66u8, 84u8, 217u8, 72u8, 64u8, 43u8, 160u8, 89u8, 192u8, 47u8, 159u8,
                238u8, 207u8, 202u8, 9u8, 180u8, 12u8, 11u8, 57u8, 2u8, 106u8, 52u8,
                74u8, 21u8, 158u8, 58u8, 118u8, 121u8, 128u8, 122u8, 131u8, 210u8,
            ],
            [
                108u8, 110u8, 179u8, 88u8, 129u8, 248u8, 5u8, 242u8, 132u8, 190u8, 89u8,
                184u8, 64u8, 180u8, 234u8, 153u8, 15u8, 150u8, 177u8, 61u8, 217u8, 231u8,
                25u8, 44u8, 69u8, 129u8, 225u8, 188u8, 129u8, 192u8, 175u8, 152u8,
            ],
            [
                127u8, 174u8, 247u8, 69u8, 175u8, 121u8, 163u8, 175u8, 35u8, 212u8, 25u8,
                156u8, 224u8, 54u8, 105u8, 117u8, 204u8, 45u8, 215u8, 20u8, 147u8, 132u8,
                214u8, 49u8, 51u8, 125u8, 19u8, 59u8, 34u8, 107u8, 91u8, 228u8,
            ],
            [
                188u8, 124u8, 215u8, 90u8, 32u8, 238u8, 39u8, 253u8, 154u8, 222u8, 186u8,
                179u8, 32u8, 65u8, 247u8, 85u8, 33u8, 77u8, 188u8, 107u8, 255u8, 169u8,
                12u8, 192u8, 34u8, 91u8, 57u8, 218u8, 46u8, 92u8, 45u8, 59u8,
            ],
            [
                196u8, 47u8, 30u8, 202u8, 200u8, 212u8, 136u8, 30u8, 249u8, 253u8, 51u8,
                92u8, 169u8, 142u8, 231u8, 37u8, 74u8, 67u8, 107u8, 146u8, 186u8, 135u8,
                74u8, 96u8, 158u8, 80u8, 167u8, 211u8, 12u8, 72u8, 123u8, 141u8,
            ],
            [
                199u8, 245u8, 5u8, 178u8, 243u8, 113u8, 174u8, 33u8, 117u8, 238u8, 73u8,
                19u8, 244u8, 73u8, 158u8, 31u8, 38u8, 51u8, 167u8, 181u8, 147u8, 99u8,
                33u8, 238u8, 209u8, 205u8, 174u8, 182u8, 17u8, 81u8, 129u8, 210u8,
            ],
            [
                218u8, 7u8, 93u8, 9u8, 25u8, 141u8, 32u8, 126u8, 58u8, 145u8, 141u8,
                75u8, 141u8, 252u8, 135u8, 223u8, 45u8, 96u8, 160u8, 11u8, 231u8, 3u8,
                253u8, 57u8, 234u8, 172u8, 144u8, 150u8, 45u8, 160u8, 183u8, 240u8,
            ],
            [
                234u8, 22u8, 64u8, 243u8, 77u8, 234u8, 218u8, 47u8, 129u8, 93u8, 239u8,
                14u8, 17u8, 46u8, 108u8, 210u8, 73u8, 83u8, 56u8, 73u8, 160u8, 253u8,
                83u8, 233u8, 41u8, 59u8, 163u8, 77u8, 124u8, 70u8, 136u8, 203u8,
            ],
            [
                238u8, 27u8, 143u8, 41u8, 213u8, 22u8, 213u8, 99u8, 244u8, 31u8, 42u8,
                59u8, 117u8, 194u8, 0u8, 117u8, 47u8, 235u8, 81u8, 46u8, 98u8, 86u8,
                235u8, 86u8, 29u8, 23u8, 164u8, 230u8, 30u8, 230u8, 51u8, 94u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(KmsEpochDestroyed),
            ::core::stringify!(NewKmsEpoch),
            ::core::stringify!(NewKmsContext),
            ::core::stringify!(KmsContextCreationConfirmation),
            ::core::stringify!(ActivateEpoch),
            ::core::stringify!(EpochActivationConfirmation),
            ::core::stringify!(Upgraded),
            ::core::stringify!(CoprocessorUpgradeProposed),
            ::core::stringify!(Initialized),
            ::core::stringify!(KmsContextDestroyed),
            ::core::stringify!(KmsContextDestructionConfirmation),
            ::core::stringify!(KmsEpochDestructionConfirmation),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <KmsEpochDestroyed as alloy_sol_types::SolEvent>::SIGNATURE,
            <NewKmsEpoch as alloy_sol_types::SolEvent>::SIGNATURE,
            <NewKmsContext as alloy_sol_types::SolEvent>::SIGNATURE,
            <KmsContextCreationConfirmation as alloy_sol_types::SolEvent>::SIGNATURE,
            <ActivateEpoch as alloy_sol_types::SolEvent>::SIGNATURE,
            <EpochActivationConfirmation as alloy_sol_types::SolEvent>::SIGNATURE,
            <Upgraded as alloy_sol_types::SolEvent>::SIGNATURE,
            <CoprocessorUpgradeProposed as alloy_sol_types::SolEvent>::SIGNATURE,
            <Initialized as alloy_sol_types::SolEvent>::SIGNATURE,
            <KmsContextDestroyed as alloy_sol_types::SolEvent>::SIGNATURE,
            <KmsContextDestructionConfirmation as alloy_sol_types::SolEvent>::SIGNATURE,
            <KmsEpochDestructionConfirmation as alloy_sol_types::SolEvent>::SIGNATURE,
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
    impl alloy_sol_types::SolEventInterface for ProtocolConfigEvents {
        const NAME: &'static str = "ProtocolConfigEvents";
        const COUNT: usize = 12usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<ActivateEpoch as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ActivateEpoch as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ActivateEpoch)
                }
                Some(
                    <CoprocessorUpgradeProposed as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <CoprocessorUpgradeProposed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::CoprocessorUpgradeProposed)
                }
                Some(
                    <EpochActivationConfirmation as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <EpochActivationConfirmation as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::EpochActivationConfirmation)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::Initialized)
                }
                Some(
                    <KmsContextCreationConfirmation as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <KmsContextCreationConfirmation as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::KmsContextCreationConfirmation)
                }
                Some(
                    <KmsContextDestroyed as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <KmsContextDestroyed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::KmsContextDestroyed)
                }
                Some(
                    <KmsContextDestructionConfirmation as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <KmsContextDestructionConfirmation as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::KmsContextDestructionConfirmation)
                }
                Some(
                    <KmsEpochDestroyed as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <KmsEpochDestroyed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::KmsEpochDestroyed)
                }
                Some(
                    <KmsEpochDestructionConfirmation as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <KmsEpochDestructionConfirmation as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::KmsEpochDestructionConfirmation)
                }
                Some(<NewKmsContext as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NewKmsContext as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::NewKmsContext)
                }
                Some(<NewKmsEpoch as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NewKmsEpoch as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::NewKmsEpoch)
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
    impl alloy_sol_types::private::IntoLogData for ProtocolConfigEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ActivateEpoch(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::CoprocessorUpgradeProposed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EpochActivationConfirmation(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::KmsContextCreationConfirmation(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::KmsContextDestroyed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::KmsContextDestructionConfirmation(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::KmsEpochDestroyed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::KmsEpochDestructionConfirmation(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NewKmsContext(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NewKmsEpoch(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ActivateEpoch(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::CoprocessorUpgradeProposed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EpochActivationConfirmation(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::KmsContextCreationConfirmation(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::KmsContextDestroyed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::KmsContextDestructionConfirmation(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::KmsEpochDestroyed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::KmsEpochDestructionConfirmation(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NewKmsContext(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NewKmsEpoch(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ProtocolConfig`](self) contract instance.

See the [wrapper's documentation](`ProtocolConfigInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> ProtocolConfigInstance<P, N> {
        ProtocolConfigInstance::<P, N>::new(address, __provider)
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
        Output = alloy_contract::Result<ProtocolConfigInstance<P, N>>,
    > {
        ProtocolConfigInstance::<P, N>::deploy(__provider)
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
        ProtocolConfigInstance::<P, N>::deploy_builder(__provider)
    }
    /**A [`ProtocolConfig`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ProtocolConfig`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ProtocolConfigInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for ProtocolConfigInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ProtocolConfigInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ProtocolConfigInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`ProtocolConfig`](self) contract instance.

See the [wrapper's documentation](`ProtocolConfigInstance`) for more details.*/
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
        ) -> alloy_contract::Result<ProtocolConfigInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> ProtocolConfigInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ProtocolConfigInstance<P, N> {
            ProtocolConfigInstance {
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
    > ProtocolConfigInstance<P, N> {
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
        ///Creates a new call builder for the [`confirmEpochActivation`] function.
        pub fn confirmEpochActivation(
            &self,
            epochId: alloy::sol_types::private::primitives::aliases::U256,
            keys: alloy::sol_types::private::Vec<
                <IProtocolConfig::EpochKeyResult as alloy::sol_types::SolType>::RustType,
            >,
            crsList: alloy::sol_types::private::Vec<
                <IProtocolConfig::EpochCrsResult as alloy::sol_types::SolType>::RustType,
            >,
            signature: alloy::sol_types::private::Bytes,
            extraData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, confirmEpochActivationCall, N> {
            self.call_builder(
                &confirmEpochActivationCall {
                    epochId,
                    keys,
                    crsList,
                    signature,
                    extraData,
                },
            )
        }
        ///Creates a new call builder for the [`confirmKmsContextCreation`] function.
        pub fn confirmKmsContextCreation(
            &self,
            kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
            signature: alloy::sol_types::private::Bytes,
            extraData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, confirmKmsContextCreationCall, N> {
            self.call_builder(
                &confirmKmsContextCreationCall {
                    kmsContextId,
                    signature,
                    extraData,
                },
            )
        }
        ///Creates a new call builder for the [`confirmKmsContextDestruction`] function.
        pub fn confirmKmsContextDestruction(
            &self,
            destroyedContextId: alloy::sol_types::private::primitives::aliases::U256,
            destroyedEpochIds: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            signature: alloy::sol_types::private::Bytes,
            extraData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, confirmKmsContextDestructionCall, N> {
            self.call_builder(
                &confirmKmsContextDestructionCall {
                    destroyedContextId,
                    destroyedEpochIds,
                    signature,
                    extraData,
                },
            )
        }
        ///Creates a new call builder for the [`confirmKmsEpochDestruction`] function.
        pub fn confirmKmsEpochDestruction(
            &self,
            destroyedEpochId: alloy::sol_types::private::primitives::aliases::U256,
            signature: alloy::sol_types::private::Bytes,
            extraData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, confirmKmsEpochDestructionCall, N> {
            self.call_builder(
                &confirmKmsEpochDestructionCall {
                    destroyedEpochId,
                    signature,
                    extraData,
                },
            )
        }
        ///Creates a new call builder for the [`defineNewEpochForCurrentKmsContext`] function.
        pub fn defineNewEpochForCurrentKmsContext(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            defineNewEpochForCurrentKmsContextCall,
            N,
        > {
            self.call_builder(&defineNewEpochForCurrentKmsContextCall)
        }
        ///Creates a new call builder for the [`defineNewKmsContextAndEpoch`] function.
        pub fn defineNewKmsContextAndEpoch(
            &self,
            kmsNodeParams: alloy::sol_types::private::Vec<
                <KmsNodeParams as alloy::sol_types::SolType>::RustType,
            >,
            thresholds: <KmsThresholds as alloy::sol_types::SolType>::RustType,
            softwareVersion: alloy::sol_types::private::String,
            pcrValues: alloy::sol_types::private::Vec<
                <PcrValues as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, defineNewKmsContextAndEpochCall, N> {
            self.call_builder(
                &defineNewKmsContextAndEpochCall {
                    kmsNodeParams,
                    thresholds,
                    softwareVersion,
                    pcrValues,
                },
            )
        }
        ///Creates a new call builder for the [`destroyKmsContext`] function.
        pub fn destroyKmsContext(
            &self,
            kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, destroyKmsContextCall, N> {
            self.call_builder(
                &destroyKmsContextCall {
                    kmsContextId,
                },
            )
        }
        ///Creates a new call builder for the [`destroyKmsEpoch`] function.
        pub fn destroyKmsEpoch(
            &self,
            epochId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, destroyKmsEpochCall, N> {
            self.call_builder(&destroyKmsEpochCall { epochId })
        }
        ///Creates a new call builder for the [`getCurrentKmsContextAndEpoch`] function.
        pub fn getCurrentKmsContextAndEpoch(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getCurrentKmsContextAndEpochCall, N> {
            self.call_builder(&getCurrentKmsContextAndEpochCall)
        }
        ///Creates a new call builder for the [`getCurrentKmsContextId`] function.
        pub fn getCurrentKmsContextId(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getCurrentKmsContextIdCall, N> {
            self.call_builder(&getCurrentKmsContextIdCall)
        }
        ///Creates a new call builder for the [`getCurrentKmsContextIdCounter`] function.
        pub fn getCurrentKmsContextIdCounter(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getCurrentKmsContextIdCounterCall, N> {
            self.call_builder(&getCurrentKmsContextIdCounterCall)
        }
        ///Creates a new call builder for the [`getKmsContextAnchor`] function.
        pub fn getKmsContextAnchor(
            &self,
            contextId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getKmsContextAnchorCall, N> {
            self.call_builder(
                &getKmsContextAnchorCall {
                    contextId,
                },
            )
        }
        ///Creates a new call builder for the [`getKmsGenThreshold`] function.
        pub fn getKmsGenThreshold(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getKmsGenThresholdCall, N> {
            self.call_builder(&getKmsGenThresholdCall)
        }
        ///Creates a new call builder for the [`getKmsGenThresholdForContext`] function.
        pub fn getKmsGenThresholdForContext(
            &self,
            kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getKmsGenThresholdForContextCall, N> {
            self.call_builder(
                &getKmsGenThresholdForContextCall {
                    kmsContextId,
                },
            )
        }
        ///Creates a new call builder for the [`getKmsNodeForContext`] function.
        pub fn getKmsNodeForContext(
            &self,
            kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
            txSender: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, getKmsNodeForContextCall, N> {
            self.call_builder(
                &getKmsNodeForContextCall {
                    kmsContextId,
                    txSender,
                },
            )
        }
        ///Creates a new call builder for the [`getKmsNodesForContext`] function.
        pub fn getKmsNodesForContext(
            &self,
            kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getKmsNodesForContextCall, N> {
            self.call_builder(
                &getKmsNodesForContextCall {
                    kmsContextId,
                },
            )
        }
        ///Creates a new call builder for the [`getKmsSigners`] function.
        pub fn getKmsSigners(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getKmsSignersCall, N> {
            self.call_builder(&getKmsSignersCall)
        }
        ///Creates a new call builder for the [`getKmsSignersForContext`] function.
        pub fn getKmsSignersForContext(
            &self,
            kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getKmsSignersForContextCall, N> {
            self.call_builder(
                &getKmsSignersForContextCall {
                    kmsContextId,
                },
            )
        }
        ///Creates a new call builder for the [`getMpcThreshold`] function.
        pub fn getMpcThreshold(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getMpcThresholdCall, N> {
            self.call_builder(&getMpcThresholdCall)
        }
        ///Creates a new call builder for the [`getMpcThresholdForContext`] function.
        pub fn getMpcThresholdForContext(
            &self,
            kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getMpcThresholdForContextCall, N> {
            self.call_builder(
                &getMpcThresholdForContextCall {
                    kmsContextId,
                },
            )
        }
        ///Creates a new call builder for the [`getPublicDecryptionThreshold`] function.
        pub fn getPublicDecryptionThreshold(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getPublicDecryptionThresholdCall, N> {
            self.call_builder(&getPublicDecryptionThresholdCall)
        }
        ///Creates a new call builder for the [`getPublicDecryptionThresholdForContext`] function.
        pub fn getPublicDecryptionThresholdForContext(
            &self,
            kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            getPublicDecryptionThresholdForContextCall,
            N,
        > {
            self.call_builder(
                &getPublicDecryptionThresholdForContextCall {
                    kmsContextId,
                },
            )
        }
        ///Creates a new call builder for the [`getUserDecryptionThreshold`] function.
        pub fn getUserDecryptionThreshold(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getUserDecryptionThresholdCall, N> {
            self.call_builder(&getUserDecryptionThresholdCall)
        }
        ///Creates a new call builder for the [`getUserDecryptionThresholdForContext`] function.
        pub fn getUserDecryptionThresholdForContext(
            &self,
            kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            getUserDecryptionThresholdForContextCall,
            N,
        > {
            self.call_builder(
                &getUserDecryptionThresholdForContextCall {
                    kmsContextId,
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
            initialKmsNodeParams: alloy::sol_types::private::Vec<
                <KmsNodeParams as alloy::sol_types::SolType>::RustType,
            >,
            initialThresholds: <KmsThresholds as alloy::sol_types::SolType>::RustType,
            softwareVersion: alloy::sol_types::private::String,
            pcrValues: alloy::sol_types::private::Vec<
                <PcrValues as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, initializeFromEmptyProxyCall, N> {
            self.call_builder(
                &initializeFromEmptyProxyCall {
                    initialKmsNodeParams,
                    initialThresholds,
                    softwareVersion,
                    pcrValues,
                },
            )
        }
        ///Creates a new call builder for the [`isKmsSigner`] function.
        pub fn isKmsSigner(
            &self,
            signer: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, isKmsSignerCall, N> {
            self.call_builder(&isKmsSignerCall { signer })
        }
        ///Creates a new call builder for the [`isKmsSignerForContext`] function.
        pub fn isKmsSignerForContext(
            &self,
            kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
            signer: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, isKmsSignerForContextCall, N> {
            self.call_builder(
                &isKmsSignerForContextCall {
                    kmsContextId,
                    signer,
                },
            )
        }
        ///Creates a new call builder for the [`isKmsTxSenderForContext`] function.
        pub fn isKmsTxSenderForContext(
            &self,
            kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
            txSender: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, isKmsTxSenderForContextCall, N> {
            self.call_builder(
                &isKmsTxSenderForContextCall {
                    kmsContextId,
                    txSender,
                },
            )
        }
        ///Creates a new call builder for the [`isLiveKmsContext`] function.
        pub fn isLiveKmsContext(
            &self,
            kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, isLiveKmsContextCall, N> {
            self.call_builder(
                &isLiveKmsContextCall {
                    kmsContextId,
                },
            )
        }
        ///Creates a new call builder for the [`isValidEpochForContext`] function.
        pub fn isValidEpochForContext(
            &self,
            kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
            epochId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, isValidEpochForContextCall, N> {
            self.call_builder(
                &isValidEpochForContextCall {
                    kmsContextId,
                    epochId,
                },
            )
        }
        ///Creates a new call builder for the [`isValidKmsContext`] function.
        pub fn isValidKmsContext(
            &self,
            kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, isValidKmsContextCall, N> {
            self.call_builder(
                &isValidKmsContextCall {
                    kmsContextId,
                },
            )
        }
        ///Creates a new call builder for the [`proposeCoprocessorUpgrade`] function.
        pub fn proposeCoprocessorUpgrade(
            &self,
            proposalId: alloy::sol_types::private::primitives::aliases::U256,
            softwareVersion: alloy::sol_types::private::String,
            chainUpgradeWindows: alloy::sol_types::private::Vec<
                <ChainUpgradeWindow as alloy::sol_types::SolType>::RustType,
            >,
            gwStartBlock: u64,
        ) -> alloy_contract::SolCallBuilder<&P, proposeCoprocessorUpgradeCall, N> {
            self.call_builder(
                &proposeCoprocessorUpgradeCall {
                    proposalId,
                    softwareVersion,
                    chainUpgradeWindows,
                    gwStartBlock,
                },
            )
        }
        ///Creates a new call builder for the [`proxiableUUID`] function.
        pub fn proxiableUUID(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, proxiableUUIDCall, N> {
            self.call_builder(&proxiableUUIDCall)
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
    > ProtocolConfigInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`ActivateEpoch`] event.
        pub fn ActivateEpoch_filter(
            &self,
        ) -> alloy_contract::Event<&P, ActivateEpoch, N> {
            self.event_filter::<ActivateEpoch>()
        }
        ///Creates a new event filter for the [`CoprocessorUpgradeProposed`] event.
        pub fn CoprocessorUpgradeProposed_filter(
            &self,
        ) -> alloy_contract::Event<&P, CoprocessorUpgradeProposed, N> {
            self.event_filter::<CoprocessorUpgradeProposed>()
        }
        ///Creates a new event filter for the [`EpochActivationConfirmation`] event.
        pub fn EpochActivationConfirmation_filter(
            &self,
        ) -> alloy_contract::Event<&P, EpochActivationConfirmation, N> {
            self.event_filter::<EpochActivationConfirmation>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<&P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`KmsContextCreationConfirmation`] event.
        pub fn KmsContextCreationConfirmation_filter(
            &self,
        ) -> alloy_contract::Event<&P, KmsContextCreationConfirmation, N> {
            self.event_filter::<KmsContextCreationConfirmation>()
        }
        ///Creates a new event filter for the [`KmsContextDestroyed`] event.
        pub fn KmsContextDestroyed_filter(
            &self,
        ) -> alloy_contract::Event<&P, KmsContextDestroyed, N> {
            self.event_filter::<KmsContextDestroyed>()
        }
        ///Creates a new event filter for the [`KmsContextDestructionConfirmation`] event.
        pub fn KmsContextDestructionConfirmation_filter(
            &self,
        ) -> alloy_contract::Event<&P, KmsContextDestructionConfirmation, N> {
            self.event_filter::<KmsContextDestructionConfirmation>()
        }
        ///Creates a new event filter for the [`KmsEpochDestroyed`] event.
        pub fn KmsEpochDestroyed_filter(
            &self,
        ) -> alloy_contract::Event<&P, KmsEpochDestroyed, N> {
            self.event_filter::<KmsEpochDestroyed>()
        }
        ///Creates a new event filter for the [`KmsEpochDestructionConfirmation`] event.
        pub fn KmsEpochDestructionConfirmation_filter(
            &self,
        ) -> alloy_contract::Event<&P, KmsEpochDestructionConfirmation, N> {
            self.event_filter::<KmsEpochDestructionConfirmation>()
        }
        ///Creates a new event filter for the [`NewKmsContext`] event.
        pub fn NewKmsContext_filter(
            &self,
        ) -> alloy_contract::Event<&P, NewKmsContext, N> {
            self.event_filter::<NewKmsContext>()
        }
        ///Creates a new event filter for the [`NewKmsEpoch`] event.
        pub fn NewKmsEpoch_filter(&self) -> alloy_contract::Event<&P, NewKmsEpoch, N> {
            self.event_filter::<NewKmsEpoch>()
        }
        ///Creates a new event filter for the [`Upgraded`] event.
        pub fn Upgraded_filter(&self) -> alloy_contract::Event<&P, Upgraded, N> {
            self.event_filter::<Upgraded>()
        }
    }
}
