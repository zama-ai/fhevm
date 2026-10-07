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
    struct EpochCrsResult { uint256 crsId; uint256 maxBitLength; bytes crsDigest; bytes signature; }
    struct EpochKeyResult { uint256 prepKeygenId; uint256 keyId; IKMSGeneration.KeyDigest[] keyDigests; bytes signature; }
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
struct EpochCrsResult { uint256 crsId; uint256 maxBitLength; bytes crsDigest; bytes signature; }
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<EpochCrsResult> for UnderlyingRustTuple<'_> {
            fn from(value: EpochCrsResult) -> Self {
                (value.crsId, value.maxBitLength, value.crsDigest, value.signature)
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
                    signature: tuple.3,
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
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
                    "EpochCrsResult(uint256 crsId,uint256 maxBitLength,bytes crsDigest,bytes signature)",
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signature,
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
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
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
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signature,
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
struct EpochKeyResult { uint256 prepKeygenId; uint256 keyId; IKMSGeneration.KeyDigest[] keyDigests; bytes signature; }
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Array<IKMSGeneration::KeyDigest>,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<EpochKeyResult> for UnderlyingRustTuple<'_> {
            fn from(value: EpochKeyResult) -> Self {
                (value.prepKeygenId, value.keyId, value.keyDigests, value.signature)
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
                    signature: tuple.3,
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
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
                    "EpochKeyResult(uint256 prepKeygenId,uint256 keyId,IKMSGeneration.KeyDigest[] keyDigests,bytes signature)",
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signature,
                        )
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
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
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
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signature,
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
        bytes signature;
    }
    struct EpochKeyResult {
        uint256 prepKeygenId;
        uint256 keyId;
        IKMSGeneration.KeyDigest[] keyDigests;
        bytes signature;
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
    error EmptyEpochActivationAttestation(uint256 epochId);
    error EmptyKmsNodes();
    error EmptySoftwareVersion();
    error EpochActivationAlreadyConfirmed(address signer, uint256 epochId);
    error EpochActivationSignerDoesNotMatchTxSender(address signer, address txSender);
    error EpochActivationUnauthorized(address caller, uint256 epochId);
    error FailedCall();
    error InvalidBlockWindow(uint64 chainId, uint64 startBlock, uint64 endBlock);
    error InvalidHighThreshold(string thresholdName, uint256 threshold, uint256 nodeCount);
    error InvalidInitialization();
    error InvalidKmsContext(uint256 kmsContextId);
    error InvalidKmsEpoch(uint256 epochId);
    error InvalidNullThreshold(string thresholdName);
    error InvalidProposalId();
    error KmsContextCreationAlreadyConfirmed(address txSender, uint256 kmsContextId);
    error KmsContextCreationUnauthorized(address caller, uint256 kmsContextId);
    error KmsContextNotPending(uint256 kmsContextId);
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
    event EpochActivationConfirmation(uint256 indexed epochId, address indexed signer, bytes32 dataHash);
    event Initialized(uint64 version);
    event KmsContextCreationConfirmation(uint256 indexed kmsContextId, address indexed txSender, bool isPreviousTxSender, bool isNewTxSender);
    event KmsContextDestroyed(uint256 indexed kmsContextId);
    event KmsEpochDestroyed(uint256 indexed epochId);
    event NewKmsContext(uint256 indexed contextId, uint256 indexed previousContextId, KmsNodeParams[] kmsNodeParams, KmsThresholds thresholds, string softwareVersion, PcrValues[] pcrValues);
    event NewKmsEpoch(uint256 indexed kmsContextId, uint256 indexed epochId, uint256 previousContextId, uint256 previousEpochId, uint256 materialBlockNumber);
    event Upgraded(address indexed implementation);

    constructor();

    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function confirmEpochActivation(uint256 epochId, IProtocolConfig.EpochKeyResult[] memory keys, IProtocolConfig.EpochCrsResult[] memory crsList) external;
    function confirmKmsContextCreation(uint256 kmsContextId) external;
    function defineNewEpochForCurrentKmsContext() external;
    function defineNewKmsContextAndEpoch(KmsNodeParams[] memory kmsNodeParams, KmsThresholds memory thresholds, string memory softwareVersion, PcrValues[] memory pcrValues) external;
    function destroyKmsContext(uint256 kmsContextId) external;
    function destroyKmsEpoch(uint256 epochId) external;
    function getContextCreationPreviousTxSenderThreshold(uint256 kmsContextId) external view returns (uint256);
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
    function reinitializeV3() external;
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
          },
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
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
          },
          {
            "name": "signature",
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
    "name": "confirmKmsContextCreation",
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
    "name": "getContextCreationPreviousTxSenderThreshold",
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
    "name": "reinitializeV3",
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
          },
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
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
          },
          {
            "name": "signature",
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
        "name": "dataHash",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
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
        "name": "txSender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "isPreviousTxSender",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
      },
      {
        "name": "isNewTxSender",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
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
    "name": "EmptyEpochActivationAttestation",
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
    "name": "EpochActivationSignerDoesNotMatchTxSender",
    "inputs": [
      {
        "name": "signer",
        "type": "address",
        "internalType": "address"
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
    "name": "EpochActivationUnauthorized",
    "inputs": [
      {
        "name": "caller",
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
        "name": "txSender",
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
        "name": "caller",
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
    ///0x60a06040523060805234801562000014575f80fd5b506200001f62000025565b620000d9565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00805468010000000000000000900460ff1615620000765760405163f92ee8a960e01b815260040160405180910390fd5b80546001600160401b0390811614620000d65780546001600160401b0319166001600160401b0390811782556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a15b50565b608051614996620001005f395f8181612c7701528181612ca00152612e8501526149965ff3fe608060405260043610610207575f3560e01c80639447cfd411610113578063c2580a2d1161009d578063ccbf81991161006d578063ccbf8199146105b2578063cceac019146105d1578063d9be2de4146105f0578063ee7d52d11461060f578063f9c670c31461062e575f80fd5b8063c2580a2d14610541578063c2b4298614610560578063c3aaaa5a14610574578063c999a8b414610593575f80fd5b8063ad3cb1cc116100e3578063ad3cb1cc146104ab578063b4722bc4146104db578063bac22bb8146104ef578063bf9b16c814610503578063c0ae64f714610522575f80fd5b80639447cfd414610445578063976c98b514610464578063976f3eb9146104835780639d1b1be114610497575f80fd5b806341ad069c116101945780634f1ef286116101645780634f1ef286146103b557806352d1902d146103c85780635bff76d9146103dc57806365b394af146104085780637eaac8f214610431575f80fd5b806341ad069c1461033957806346c5bbbd1461035857806347e82295146103775780634cb950e114610396575f80fd5b806326cf5def116101da57806326cf5def14610299578063281e8bfe146102bb5780632a388998146102da57806331ff41c8146102ee5780633e6316ea1461031a575f80fd5b80630d8e6e2c1461020b5780631ce3f9bc14610235578063203d01141461024b578063221cdd4e1461027a575b5f80fd5b348015610216575f80fd5b5061021f61065a565b60405161022c9190613991565b60405180910390f35b348015610240575f80fd5b506102496106c6565b005b348015610256575f80fd5b5061026a6102653660046139c7565b6107f0565b604051901515815260200161022c565b348015610285575f80fd5b50610249610294366004613a66565b610830565b3480156102a4575f80fd5b506102ad610a46565b60405190815260200161022c565b3480156102c6575f80fd5b506102ad6102d5366004613b19565b610a6c565b3480156102e5575f80fd5b506102ad610a91565b3480156102f9575f80fd5b5061030d610308366004613b30565b610ab7565b60405161022c9190613bab565b348015610325575f80fd5b506102ad610334366004613b19565b610c7b565b348015610344575f80fd5b506102ad610353366004613b19565b610c97565b348015610363575f80fd5b5061026a610372366004613b30565b610cdc565b348015610382575f80fd5b506102ad610391366004613b19565b610d3a565b3480156103a1575f80fd5b506102496103b0366004613bbd565b610d5f565b6102496103c3366004613c44565b611503565b3480156103d3575f80fd5b506102ad611522565b3480156103e7575f80fd5b506103fb6103f6366004613b19565b61153d565b60405161022c9190613d01565b348015610413575f80fd5b5061041c6115c0565b6040805192835260208301919091520161022c565b34801561043c575f80fd5b506103fb6115e0565b348015610450575f80fd5b5061026a61045f366004613b30565b611661565b34801561046f575f80fd5b5061024961047e366004613a66565b61169f565b34801561048e575f80fd5b506102ad61189f565b3480156104a2575f80fd5b506102ad6118b3565b3480156104b6575f80fd5b5061021f604051806040016040528060058152602001640352e302e360dc1b81525081565b3480156104e6575f80fd5b506102ad6118c4565b3480156104fa575f80fd5b506102496118ea565b34801561050e575f80fd5b5061026a61051d366004613b19565b6119b2565b34801561052d575f80fd5b5061024961053c366004613b19565b6119bc565b34801561054c575f80fd5b5061024961055b366004613b19565b611b6a565b34801561056b575f80fd5b506102ad611d2b565b34801561057f575f80fd5b506102ad61058e366004613b19565b611d51565b34801561059e575f80fd5b5061041c6105ad366004613b19565b611d76565b3480156105bd575f80fd5b506102496105cc366004613d63565b611ddd565b3480156105dc575f80fd5b5061026a6105eb366004613e12565b6120e9565b3480156105fb575f80fd5b5061024961060a366004613b19565b61214e565b34801561061a575f80fd5b5061026a610629366004613b19565b6123d7565b348015610639575f80fd5b5061064d610648366004613b19565b6123e1565b60405161022c9190613e32565b60606040518060400160405280600e81526020016d50726f746f636f6c436f6e66696760901b81525061068c5f6125a4565b61069660036125a4565b61069f5f6125a4565b6040516020016106b29493929190613e94565b604051602081830303815290604052905090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610716573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061073a9190613f11565b6001600160a01b0316336001600160a01b0316146107725760405163021bfda160e41b81523360048201526024015b60405180910390fd5b61077a612633565b5f6107836126f6565b600b8101549091505f6107958261271a565b905080827f15aaaf475ef407543f5164f57dcf57f7f93816f55bae77ca09efc445ba40eef78486600d01546001436107cd9190613f40565b6040805193845260208401929092529082015260600160405180910390a3505050565b5f806107fa6126f6565b600b8101545f9081526003909101602090815260408083206001600160a01b039096168352949052929092205460ff1692915050565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00546001600160401b03166001600160401b031660011461088457604051636f4f731f60e01b815260040160405180910390fd5b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00805460049190600160401b900460ff16806108cd575080546001600160401b03808416911610155b156108eb5760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b03831617600160401b1781555f6109156126f6565b90505f610927600760f81b6001613f67565b90506109448161093c600160fb1b6001613f67565b8d8d8d61276b565b60405180604001604052804381526020018c8c8c8c8c8c8c60405160200161097297969594939291906140b9565b60408051601f1981840301815291815281516020928301209092525f848152601786018252919091208251815591015160019091015560f86007901b817f204d6b80121154cd87d99cf54c639a3dd0a53b3084277098de972ebdd34c6be98d8d8d8d8d8d8d6040516109ea97969594939291906140b9565b60405180910390a35050805460ff60401b191681556040516001600160401b03831681527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a1505050505050505050565b5f80610a506126f6565b600b8101545f9081526009909101602052604090205492915050565b5f610a76826127bc565b610a7e6126f6565b5f92835260070160205250604090205490565b5f80610a9b6126f6565b600b8101545f9081526006909101602052604090205492915050565b604080516080810182525f8082526020820152606091810182905281810191909152610ae2836127e8565b610b02576040516377ddbe8160e01b815260048101849052602401610769565b610b0a6126f6565b5f848152600491909101602090815260408083206001600160a01b03808716855290835292819020815160808101835281548516815260018201549094169284019290925260028201805491840191610b6290614281565b80601f0160208091040260200160405190810160405280929190818152602001828054610b8e90614281565b8015610bd95780601f10610bb057610100808354040283529160200191610bd9565b820191905f5260205f20905b815481529060010190602001808311610bbc57829003601f168201915b50505050508152602001600382018054610bf290614281565b80601f0160208091040260200160405190810160405280929190818152602001828054610c1e90614281565b8015610c695780601f10610c4057610100808354040283529160200191610c69565b820191905f5260205f20905b815481529060010190602001808311610c4c57829003601f168201915b50505050508152505090505b92915050565b5f610c846126f6565b5f92835260140160205250604090205490565b5f610ca182612806565b610cc1576040516377ddbe8160e01b815260048101839052602401610769565b610cc96126f6565b5f92835260080160205250604090205490565b5f610ce683612806565b610d06576040516377ddbe8160e01b815260048101849052602401610769565b610d0e6126f6565b5f938452600201602090815260408085206001600160a01b039490941685529290525090205460ff1690565b5f610d44826127bc565b610d4c6126f6565b5f92835260090160205250604090205490565b5f610d686126f6565b905060015f878152600f8301602052604090205460ff166002811115610d9057610d90613f53565b14610db157604051637995bbcf60e01b815260048101879052602401610769565b5f8681526010820160209081526040808320548084526002850183528184203385529092529091205460ff16610e035760405163a3f4afeb60e01b815233600482015260248101889052604401610769565b841580610e0e575082155b15610e2f5760405163ce9440df60e01b815260048101889052602401610769565b5f81815260048301602090815260408083203384528252808320600101548151600160f91b938101939093526021830185905260418084018c90528251808503909101815260619093019091526001600160a01b0316919081886001600160401b03811115610ea057610ea0613c30565b604051908082528060200260200182016040528015610ec9578160200160208202803683370190505b5090505f5b89811015611057575f610f118c8c84818110610eec57610eec6142b9565b9050602002810190610efe91906142cd565b610f0c9060408101906142eb565b61283e565b90505f610f6b8d8d85818110610f2957610f296142b9565b9050602002810190610f3b91906142cd565b358e8e86818110610f4e57610f4e6142b9565b9050602002810190610f6091906142cd565b6020013584886129a4565b9050610fa987828f8f87818110610f8457610f846142b9565b9050602002810190610f9691906142cd565b610fa4906060810190614330565b612b20565b8c8c84818110610fbb57610fbb6142b9565b9050602002810190610fcd91906142cd565b358d8d85818110610fe057610fe06142b9565b9050602002810190610ff291906142cd565b602001358360405160200161101a939291909283526020830191909152604082015260600190565b60405160208183030381529060405280519060200120848481518110611042576110426142b9565b60209081029190910101525050600101610ece565b505f876001600160401b0381111561107157611071613c30565b60405190808252806020026020018201604052801561109a578160200160208202803683370190505b5090505f5b88811015611217575f6111308b8b848181106110bd576110bd6142b9565b90506020028101906110cf91906142cd565b358c8c858181106110e2576110e26142b9565b90506020028101906110f491906142cd565b602001358d8d8681811061110a5761110a6142b9565b905060200281019061111c91906142cd565b61112a906040810190614330565b89612ba5565b905061114987828d8d86818110610f8457610f846142b9565b8a8a8381811061115b5761115b6142b9565b905060200281019061116d91906142cd565b358b8b84818110611180576111806142b9565b905060200281019061119291906142cd565b602001358c8c858181106111a8576111a86142b9565b90506020028101906111ba91906142cd565b6111c8906040810190614330565b6040516020016111db9493929190614372565b60405160208183030381529060405280519060200120838381518110611203576112036142b9565b60209081029190910101525060010161109f565b50818160405160200161122b9291906143cb565b60408051601f1981840301815291815281516020928301205f8f815260128b0184528281206001600160a01b038a16825290935291205490945060ff1615925061129d915050576040516324a0bb1b60e11b81526001600160a01b0383166004820152602481018a9052604401610769565b5f89815260128501602090815260408083206001600160a01b03861684528252808320805460ff191660011790558b83526013870182528083208484529091528120805482906112ec906143ef565b9190508190559050826001600160a01b03168a7f7eda6f85e23b7b91c019b0570d02b663606ef9d74594f7e01fcfbdb0f4e954d58460405161133091815260200190565b60405180910390a35f84815260058601602052604090205481036114f7575f848152600e860160205260409020805460ff19166003179055600b85018490556113798a85612c32565b5f848152600186016020526040812080549091906001600160401b038111156113a4576113a4613c30565b6040519080825280602002602001820160405280156113d757816020015b60608152602001906001900390816113c25790505b5090505f5b82548110156114b2578281815481106113f7576113f76142b9565b905f5260205f209060040201600301805461141190614281565b80601f016020809104026020016040519081016040528092919081815260200182805461143d90614281565b80156114885780601f1061145f57610100808354040283529160200191611488565b820191905f5260205f20905b81548152906001019060200180831161146b57829003601f168201915b505050505082828151811061149f5761149f6142b9565b60209081029190910101526001016113dc565b508b867f1a547b42e72cd3dda04e6adccd2200276cfef01fe2138d07f3a7440f416d38bc8d8d8d8d876040516114ec9594939291906145c1565b60405180910390a350505b50505050505050505050565b61150b612c6c565b61151482612d12565b61151e8282612db9565b5050565b5f61152b612e7a565b505f8051602061497683398151915290565b6060611548826127bc565b6115506126f6565b6005015f8381526020019081526020015f208054806020026020016040519081016040528092919081815260200182805480156115b457602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311611596575b50505050509050919050565b5f805f6115cb6126f6565b905080600b0154925080600d01549150509091565b60605f6115eb6126f6565b9050806005015f82600b015481526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561165657602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311611638575b505050505091505090565b5f61166b836127bc565b6116736126f6565b5f938452600301602090815260408085206001600160a01b039490941685529290525090205460ff1690565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156116ef573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117139190613f11565b6001600160a01b0316336001600160a01b0316146117465760405163021bfda160e41b8152336004820152602401610769565b61174e612633565b5f6117576126f6565b600b8101548154919250905f9061176f906001613f67565b905061177d818b8b8b612ec3565b5f818152600e840160209081526040808320805460ff19166001908117909155858452600987018352818420549087019092528220546117bd9190613f40565b90505f81116117cd5760016117cf565b805b846014015f8481526020019081526020015f208190555060405180604001604052804381526020018c8c8c8c8c8c8c60405160200161181497969594939291906140b9565b60408051601f1981840301815291815281516020928301209092525f8581526017880182528290208351815592015160019092019190915551839083907f204d6b80121154cd87d99cf54c639a3dd0a53b3084277098de972ebdd34c6be99061188a908f908f908f908f908f908f908f906140b9565b60405180910390a35050505050505050505050565b5f806118a96126f6565b600b015492915050565b5f806118bd6126f6565b5492915050565b5f806118ce6126f6565b600b8101545f9081526008909101602052604090205492915050565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00805460049190600160401b900460ff1680611933575080546001600160401b03808416911610155b156119515760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b038316908117600160401b1760ff60401b191682556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a15050565b5f610c7582613400565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611a0c573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a309190613f11565b6001600160a01b0316336001600160a01b031614611a635760405163021bfda160e41b8152336004820152602401610769565b5f611a6c6126f6565b905080600b01548203611a955760405163030f5d5560e31b815260048101839052602401610769565b611a9e82612806565b611abe576040516377ddbe8160e01b815260048101839052602401610769565b5f828152600a820160209081526040808320805460ff19908116600117909155600e8501835281842080549091169055600c8401548084526010850190925290912054839003611b1157611b1181613438565b5f8381526014830160209081526040808320839055601585018252808320839055601685019091528082208290555184917fda075d09198d207e3a918d4b8dfc87df2d60a00be703fd39eaac90962da0b7f091a2505050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611bba573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611bde9190613f11565b6001600160a01b0316336001600160a01b031614611c115760405163021bfda160e41b8152336004820152602401610769565b5f611c1a6126f6565b905080600d01548203611c435760405163f0e781c160e01b815260048101839052602401610769565b5f828152600f8201602052604081205460ff16906002826002811115611c6b57611c6b613f53565b1490505f6001836002811115611c8357611c83613f53565b148015611cc2575060035f8681526010860160209081526040808320548352600e880190915290205460ff166003811115611cc057611cc0613f53565b145b905081158015611cd0575080155b15611cf157604051637995bbcf60e01b815260048101869052602401610769565b611cfa85613438565b60405185907f03436013075f8d1abb1781dbcaf418fc76fc797d1ab51c38664c1a51f3cd57f9905f90a25050505050565b5f80611d356126f6565b600b8101545f9081526007909101602052604090205492915050565b5f611d5b826127bc565b611d636126f6565b5f92835260060160205250604090205490565b5f80611d81836127e8565b611da1576040516377ddbe8160e01b815260048101849052602401610769565b5f611daa6126f6565b5f948552601701602090815260409485902085518087019096528054808752600190910154959091018590529492505050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611e2d573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e519190613f11565b6001600160a01b0316336001600160a01b031614611e845760405163021bfda160e41b8152336004820152602401610769565b855f03611ea457604051630992f7ad60e01b815260040160405180910390fd5b5f849003611ec55760405163b548914760e01b815260040160405180910390fd5b5f829003611ee657604051632f94141160e21b815260040160405180910390fd5b806001600160401b03165f03611f0f576040516302fa7d2960e31b815260040160405180910390fd5b5f5b828110156120a05736848483818110611f2c57611f2c6142b9565b606002919091019150611f44905060208201826146ce565b6001600160401b03165f03611f6c57604051633212217560e21b815260040160405180910390fd5b611f7c60608201604083016146ce565b6001600160401b0316611f9560408301602084016146ce565b6001600160401b0316111561200657611fb160208201826146ce565b611fc160408301602084016146ce565b611fd160608401604085016146ce565b60405163790cee0760e11b81526001600160401b03938416600482015291831660248301529091166044820152606401610769565b5f5b828110156120965761201d60208301836146ce565b6001600160401b0316868683818110612038576120386142b9565b61204e92602060609092020190810191506146ce565b6001600160401b03160361208e5761206960208301836146ce565b6040516306c67e4760e41b81526001600160401b039091166004820152602401610769565b600101612008565b5050600101611f11565b50857fc42f1ecac8d4881ef9fd335ca98ee7254a436b92ba874a609e50a7d30c487b8d86868686866040516120d99594939291906146e7565b60405180910390a2505050505050565b5f806120f36126f6565b905060025f848152600f8301602052604090205460ff16600281111561211b5761211b613f53565b14801561213657505f83815260108201602052604090205484145b8015612146575061214684613400565b949350505050565b5f6121576126f6565b905060015f838152600e8301602052604090205460ff16600381111561217f5761217f613f53565b146121a057604051633586efa160e01b815260048101839052602401610769565b600b8101545f818152600283016020818152604080842033808652908352818520548886529383528185209085529091529091205460ff9182169116811580156121e8575080155b1561220f57604051631703bf1d60e31b815233600482015260248101869052604401610769565b5f858152601185016020908152604080832033845290915290205460ff161561225457604051630c4b0b9960e31b815233600482015260248101869052604401610769565b5f85815260118501602090815260408083203384529091529020805460ff1916600117905581156122a1575f8581526016850160205260408120805490919061229c906143ef565b909155505b80156122c9575f858152601585016020526040812080549091906122c4906143ef565b909155505b6040805183151581528215156020820152339187917fb79c48003695b6ebe555afa36fad071deeee75eb3718ad63de5621d35ba44b4f910160405180910390a35f858152600185016020908152604080832054601588019092529091205414801561235057505f858152601485016020908152604080832054601688019092529091205410155b156123d0575f858152600e850160205260408120805460ff191660021790556123788661271a565b905080867f15aaaf475ef407543f5164f57dcf57f7f93816f55bae77ca09efc445ba40eef78688600d01546001436123b09190613f40565b6040805193845260208401929092529082015260600160405180910390a3505b5050505050565b5f610c7582612806565b60606123ec826127bc565b6123f46126f6565b6001015f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020015f905b82821015612599575f848152602090819020604080516080810182526004860290920180546001600160a01b039081168452600182015416938301939093526002830180549293929184019161247a90614281565b80601f01602080910402602001604051908101604052809291908181526020018280546124a690614281565b80156124f15780601f106124c8576101008083540402835291602001916124f1565b820191905f5260205f20905b8154815290600101906020018083116124d457829003601f168201915b5050505050815260200160038201805461250a90614281565b80601f016020809104026020016040519081016040528092919081815260200182805461253690614281565b80156125815780601f1061255857610100808354040283529160200191612581565b820191905f5260205f20905b81548152906001019060200180831161256457829003601f168201915b50505050508152505081526020019060010190612425565b505050509050919050565b60605f6125b08361346b565b60010190505f816001600160401b038111156125ce576125ce613c30565b6040519080825280601f01601f1916602001820160405280156125f8576020820181803683370190505b5090508181016020015b5f19016f181899199a1a9b1b9c1cb0b131b232b360811b600a86061a8153600a850494508461260257509392505050565b5f61263c6126f6565b80545f908152600e8201602052604081205491925060ff90911690600182600381111561266b5761266b613f53565b14806126885750600282600381111561268657612686613f53565b145b90505f6001600c8501545f908152600f8601602052604090205460ff1660028111156126b6576126b6613f53565b14905081806126c25750805b156126f0578354600c8501546040516303c0105f60e11b815260048101929092526024820152604401610769565b50505050565b7f80f3585af86806c5774303b06c1ee640aa83b6ef3e45df49bb26c8524500c20090565b5f806127246126f6565b905080600c015f8154612736906143ef565b91829055505f818152600f830160209081526040808320805460ff191660011790556010909401905291909120929092555090565b5f6127746126f6565b905061278286858585612ec3565b5f868152600e820160205260409020805460ff19166003179055600b8101869055600c81018590556127b48587612c32565b505050505050565b6127c581613400565b6127e5576040516377ddbe8160e01b815260048101829052602401610769565b50565b5f6127f16126f6565b5f928352600101602052506040902054151590565b5f806128106126f6565b5f848152600e91909101602052604090205460ff16600381111561283657612836613f53565b141592915050565b5f80826001600160401b0381111561285857612858613c30565b604051908082528060200260200182016040528015612881578160200160208202803683370190505b5090505f5b83811015612973577fddd108772e6a3899feb04d148ae915cbe3eb5ebd202688080399e9921ac3616b8585838181106128c1576128c16142b9565b90506020028101906128d39190614783565b6128e1906020810190614797565b8686848181106128f3576128f36142b9565b90506020028101906129059190614783565b612913906020810190614330565b6040516129219291906147b0565b6040519081900381206129389392916020016147bf565b60405160208183030381529060405280519060200120828281518110612960576129606142b9565b6020908102919091010152600101612886565b508060405160200161298591906147e1565b6040516020818303038152906040528051906020012091505092915050565b8051602080830191909120604080517fbd14835bb4ae13c78ecb88ded2c3370325f39e6006eb94ff45e95f98e4c85a2a938101939093528201869052606082018590526080820184905260a08201525f90612b179060c0015b60405160208183030381529060405280519060200120604080518082018252600e81526d50726f746f636f6c436f6e66696760901b6020918201528151808301835260018152603160f81b9082015281517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f818301527fa3de1880cf083e8318b77a7965d02dd9765e85a48e418a4463af7a0d57b4b3ee818401527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc660608201524660808201523060a0808301919091528351808303909101815260c08201845280519083012061190160f01b60e083015260e2820152610102808201949094528251808203909401845261012201909152815191012090565b95945050505050565b5f612b608484848080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525061354292505050565b9050846001600160a01b0316816001600160a01b0316146123d0576040516378b9ada360e11b81526001600160a01b0382166004820152336024820152604401610769565b5f612c287fa264b318e95080300a3f06a6656a8e7fe24f9903f0e6bcca307efbe39c4c4e0987878787604051602001612bdf9291906147b0565b60408051601f19818403018152828252805160209182012089518a83012091840196909652908201939093526060810191909152608081019290925260a082015260c0016129fd565b9695505050505050565b5f612c3b6126f6565b5f848152600f820160209081526040808320805460ff191660021790556010840190915290209290925550600d0155565b306001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161480612cf257507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316612ce65f80516020614976833981519152546001600160a01b031690565b6001600160a01b031614155b15612d105760405163703e46dd60e11b815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612d62573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612d869190613f11565b6001600160a01b0316336001600160a01b0316146127e55760405163021bfda160e41b8152336004820152602401610769565b816001600160a01b03166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa925050508015612e13575060408051601f3d908101601f19168201909252612e1091810190614816565b60015b612e3b57604051634c9c8ce360e01b81526001600160a01b0383166004820152602401610769565b5f805160206149768339815191528114612e6b57604051632a87526960e21b815260048101829052602401610769565b612e75838361356a565b505050565b306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614612d105760405163703e46dd60e11b815260040160405180910390fd5b5f829003612ee357604051621a323560e61b815260040160405180910390fd5b60ff821115612f0f576040516302d4e4ef60e31b81526004810183905260ff6024820152604401610769565b60408051808201909152601081526f383ab13634b1a232b1b93cb83a34b7b760811b6020820152612f42908235846135bf565b612f7a6040518060400160405280600e81526020016d3ab9b2b92232b1b93cb83a34b7b760911b8152508260200135858590506135bf565b604080518082018252600681526535b6b9a3b2b760d11b6020820152612fa391830135846135bf565b6040805180820190915260038152626d706360e81b6020820152612fcc906060830135846135bf565b5f612fd56126f6565b8054909150851161300657805460405163efd55f6760e01b8152610769918791600401918252602082015260400190565b8481555f5b838110156133b45736858583818110613026576130266142b9565b9050602002810190613038919061482d565b60408051608081019091529091505f908061305660208501856139c7565b6001600160a01b0316815260200183602001602081019061307791906139c7565b6001600160a01b031681526020016130926040850185614330565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152505050908252506020016130d86060850185614330565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250505091525080519091506001600160a01b031661313857604051634233402560e11b815260040160405180910390fd5b60208101516001600160a01b031661316357604051632deccf4d60e01b815260040160405180910390fd5b5f888152600285016020908152604080832084516001600160a01b0316845290915290205460ff16156131b7578051604051630d18c4ff60e41b81526001600160a01b039091166004820152602401610769565b5f8881526003850160209081526040808320848301516001600160a01b0316845290915290205460ff161561321057602081015160405163f51af6bb60e01b81526001600160a01b039091166004820152602401610769565b5f8881526001858101602090815260408084208054808501825590855293829020855160049095020180546001600160a01b03199081166001600160a01b039687161782559286015193810180549093169390941692909217905582015182919060028201906132809082614885565b50606082015160038201906132959082614885565b5050505f88815260028086016020908152604080842085516001600160a01b039081168652908352818520805460ff1990811660019081179092558e875260038b018552838720888601805185168952908652848820805490921683179091558e875260048b01855283872088518416885290945294829020865181549083166001600160a01b0319918216178255935195810180549690921695909316949094179093559183015183929182019061334e9082614885565b50606082015160038201906133639082614885565b5050505f8881526005850160209081526040822092810151835460018082018655948452919092200180546001600160a01b0319166001600160a01b0390921691909117905591909101905061300b565b505f948552600681016020908152604080872084359055600783018252808720828501359055600883018252808720818501359055600990920190529093206060909301359092555050565b5f600361340b6126f6565b5f848152600e91909101602052604090205460ff16600381111561343157613431613f53565b1492915050565b5f6134416126f6565b5f928352600f810160209081526040808520805460ff191690556010909201905282209190915550565b5f8072184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b83106134a95772184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b830492506040015b6d04ee2d6d415b85acef810000000083106134d5576d04ee2d6d415b85acef8100000000830492506020015b662386f26fc1000083106134f357662386f26fc10000830492506010015b6305f5e100831061350b576305f5e100830492506008015b612710831061351f57612710830492506004015b60648310613531576064830492506002015b600a8310610c755760010192915050565b5f805f806135508686613608565b9250925092506135608282613651565b5090949350505050565b61357382613709565b6040516001600160a01b038316907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a28051156135b757612e75828261376c565b61151e6137d5565b815f036135e15782604051631b5fdb0760e11b81526004016107699190613991565b80821115612e755782828260405163caa814a360e01b815260040161076993929190614940565b5f805f835160410361363f576020840151604085015160608601515f1a613631888285856137f4565b95509550955050505061364a565b505081515f91506002905b9250925092565b5f82600381111561366457613664613f53565b0361366d575050565b600182600381111561368157613681613f53565b0361369f5760405163f645eedf60e01b815260040160405180910390fd5b60028260038111156136b3576136b3613f53565b036136d45760405163fce698f760e01b815260048101829052602401610769565b60038260038111156136e8576136e8613f53565b0361151e576040516335e2f38360e21b815260048101829052602401610769565b806001600160a01b03163b5f0361373e57604051634c9c8ce360e01b81526001600160a01b0382166004820152602401610769565b5f8051602061497683398151915280546001600160a01b0319166001600160a01b0392909216919091179055565b60605f80846001600160a01b0316846040516137889190614964565b5f60405180830381855af49150503d805f81146137c0576040519150601f19603f3d011682016040523d82523d5f602084013e6137c5565b606091505b5091509150612b178583836138bc565b3415612d105760405163b398979f60e01b815260040160405180910390fd5b5f80807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084111561382d57505f915060039050826138b2565b604080515f808252602082018084528a905260ff891692820192909252606081018790526080810186905260019060a0016020604051602081039080840390855afa15801561387e573d5f803e3d5ffd5b5050604051601f1901519150506001600160a01b0381166138a957505f9250600191508290506138b2565b92505f91508190505b9450945094915050565b6060826138d1576138cc8261391b565b613914565b81511580156138e857506001600160a01b0384163b155b1561391157604051639996b31560e01b81526001600160a01b0385166004820152602401610769565b50805b9392505050565b80511561392b5780518082602001fd5b60405163d6bda27560e01b815260040160405180910390fd5b5f5b8381101561395e578181015183820152602001613946565b50505f910152565b5f815180845261397d816020860160208601613944565b601f01601f19169290920160200192915050565b602081525f6139146020830184613966565b6001600160a01b03811681146127e5575f80fd5b80356139c2816139a3565b919050565b5f602082840312156139d7575f80fd5b8135613914816139a3565b5f8083601f8401126139f2575f80fd5b5081356001600160401b03811115613a08575f80fd5b6020830191508360208260051b8501011115613a22575f80fd5b9250929050565b5f8083601f840112613a39575f80fd5b5081356001600160401b03811115613a4f575f80fd5b602083019150836020828501011115613a22575f80fd5b5f805f805f805f87890360e0811215613a7d575f80fd5b88356001600160401b0380821115613a93575f80fd5b613a9f8c838d016139e2565b909a5098508891506080601f1984011215613ab8575f80fd5b60208b01975060a08b0135925080831115613ad1575f80fd5b613add8c848d01613a29565b909750955060c08b0135925086915080831115613af8575f80fd5b5050613b068a828b016139e2565b989b979a50959850939692959293505050565b5f60208284031215613b29575f80fd5b5035919050565b5f8060408385031215613b41575f80fd5b823591506020830135613b53816139a3565b809150509250929050565b5f60018060a01b0380835116845280602084015116602085015250604082015160806040850152613b926080850182613966565b905060608301518482036060860152612b178282613966565b602081525f6139146020830184613b5e565b5f805f805f60608688031215613bd1575f80fd5b8535945060208601356001600160401b0380821115613bee575f80fd5b613bfa89838a016139e2565b90965094506040880135915080821115613c12575f80fd5b50613c1f888289016139e2565b969995985093965092949392505050565b634e487b7160e01b5f52604160045260245ffd5b5f8060408385031215613c55575f80fd5b8235613c60816139a3565b915060208301356001600160401b0380821115613c7b575f80fd5b818501915085601f830112613c8e575f80fd5b813581811115613ca057613ca0613c30565b604051601f8201601f19908116603f01168101908382118183101715613cc857613cc8613c30565b81604052828152886020848701011115613ce0575f80fd5b826020860160208301375f6020848301015280955050505050509250929050565b602080825282518282018190525f9190848201906040850190845b81811015613d415783516001600160a01b031683529284019291840191600101613d1c565b50909695505050505050565b80356001600160401b03811681146139c2575f80fd5b5f805f805f8060808789031215613d78575f80fd5b8635955060208701356001600160401b0380821115613d95575f80fd5b613da18a838b01613a29565b90975095506040890135915080821115613db9575f80fd5b818901915089601f830112613dcc575f80fd5b813581811115613dda575f80fd5b8a6020606083028501011115613dee575f80fd5b602083019550809450505050613e0660608801613d4d565b90509295509295509295565b5f8060408385031215613e23575f80fd5b50508035926020909101359150565b5f60208083016020845280855180835260408601915060408160051b8701019250602087015f5b82811015613e8757603f19888603018452613e75858351613b5e565b94509285019290850190600101613e59565b5092979650505050505050565b5f8551613ea5818460208a01613944565b61103b60f11b9083019081528551613ec4816002840160208a01613944565b808201915050601760f91b8060028301528551613ee8816003850160208a01613944565b60039201918201528351613f03816004840160208801613944565b016004019695505050505050565b5f60208284031215613f21575f80fd5b8151613914816139a3565b634e487b7160e01b5f52601160045260245ffd5b81810381811115610c7557610c75613f2c565b634e487b7160e01b5f52602160045260245ffd5b80820180821115610c7557610c75613f2c565b5f808335601e19843603018112613f8f575f80fd5b83016020810192503590506001600160401b03811115613fad575f80fd5b803603821315613a22575f80fd5b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b8035600381900b81146139c2575f80fd5b5f8383855260208086019550808560051b830101845f5b878110156140ac57848303601f19018952813536889003605e19018112614030575f80fd5b8701606061403e8280613f7a565b82875261404e8388018284613fbb565b9250505061405e86830183613f7a565b86830388880152614070838284613fbb565b92505050604061408281840184613f7a565b935086830382880152614096838583613fbb565b9c88019c9650505092850192505060010161400b565b5090979650505050505050565b60e080825281018790525f61010080830160058a901b840182018b845b8c8110156142195786830360ff190184528135368f900360fe190181126140fb575f80fd5b8e016141178461410a836139b7565b6001600160a01b03169052565b60206141248183016139b7565b6001600160a01b031681860152604061413f83820184613f7a565b89838901526141518a89018284613fbb565b92505050606061416381850185613f7a565b888403838a0152614175848284613fbb565b93505050506080614187818501613fe3565b6141958289018260030b9052565b505060a06141a581850185613f7a565b888403838a01526141b7848284613fbb565b935050505060c06141ca81850185613f7a565b888403838a01526141dc848284613fbb565b93505050506141ee60e0840184613f7a565b935086820360e0880152614203828583613fbb565b97830197965050509290920191506001016140d6565b5050614249602086018b803582526020810135602083015260408101356040830152606081013560608301525050565b84810360a086015261425c81898b613fbb565b9250505082810360c0840152614273818587613ff4565b9a9950505050505050505050565b600181811c9082168061429557607f821691505b6020821081036142b357634e487b7160e01b5f52602260045260245ffd5b50919050565b634e487b7160e01b5f52603260045260245ffd5b5f8235607e198336030181126142e1575f80fd5b9190910192915050565b5f808335601e19843603018112614300575f80fd5b8301803591506001600160401b03821115614319575f80fd5b6020019150600581901b3603821315613a22575f80fd5b5f808335601e19843603018112614345575f80fd5b8301803591506001600160401b0382111561435e575f80fd5b602001915036819003821315613a22575f80fd5b848152836020820152606060408201525f612c28606083018486613fbb565b5f815180845260208085019450602084015f5b838110156143c0578151875295820195908201906001016143a4565b509495945050505050565b604081525f6143dd6040830185614391565b8281036020840152612b178185614391565b5f6001820161440057614400613f2c565b5060010190565b8035600481106139c2575f80fd5b6004811061443157634e487b7160e01b5f52602160045260245ffd5b9052565b5f8383855260208086019550808560051b830101845f5b878110156140ac57848303601f19018952813536889003603e19018112614471575f80fd5b870160406144878561448284614407565b614415565b61449386830183613f7a565b925081878701526144a78287018483613fbb565b9b87019b95505050918401915060010161444c565b5f8235607e198336030181126144d0575f80fd5b90910192915050565b5f8383855260208086019550808560051b830101845f5b878110156140ac57848303601f1901895261450b82886144bc565b6080813585528582013586860152604061452781840184613f7a565b83838901526145398489018284613fbb565b9350505050606061454c81840184613f7a565b935086830382880152614560838583613fbb565b9c88019c965050509285019250506001016144f0565b5f8282518085526020808601955060208260051b840101602086015f5b848110156140ac57601f198684030189526145af838351613966565b98840198925090830190600101614593565b60608082528181018690525f906080808401600589811b860183018b865b8c81101561469557888303607f190185526145fa828f6144bc565b8035845260208082013581860152604080830135601e1984360301811261461f575f80fd5b830182810190356001600160401b03811115614639575f80fd5b80891b3603821315614649575f80fd5b8a8389015261465b8b89018284614435565b9250505061466b8a840184613f7a565b93508682038b88015261467f828583613fbb565b98830198965050509290920191506001016145df565b505086810360208801526146aa818a8c6144d9565b94505050505082810360408401526146c28185614576565b98975050505050505050565b5f602082840312156146de575f80fd5b61391482613d4d565b5f60608083526146fb60608401888a613fbb565b838103602085810191909152868252879181015f5b88811015614762576001600160401b038061472a86613d4d565b16835280614739858701613d4d565b168484015260408161474c828801613d4d565b1690840152509284019290840190600101614710565b50809450505050506001600160401b03831660408301529695505050505050565b5f8235603e198336030181126142e1575f80fd5b5f602082840312156147a7575f80fd5b61391482614407565b818382375f9101908152919050565b838152606081016147d36020830185614415565b826040830152949350505050565b81515f9082906020808601845b8381101561480a578151855293820193908201906001016147ee565b50929695505050505050565b5f60208284031215614826575f80fd5b5051919050565b5f823560fe198336030181126142e1575f80fd5b601f821115612e7557805f5260205f20601f840160051c810160208510156148665750805b601f840160051c820191505b818110156123d0575f8155600101614872565b81516001600160401b0381111561489e5761489e613c30565b6148b2816148ac8454614281565b84614841565b602080601f8311600181146148e5575f84156148ce5750858301515b5f19600386901b1c1916600185901b1785556127b4565b5f85815260208120601f198616915b82811015614913578886015182559484019460019091019084016148f4565b508582101561493057878501515f19600388901b60f8161c191681555b5050505050600190811b01905550565b606081525f6149526060830186613966565b60208301949094525060400152919050565b5f82516142e181846020870161394456fe360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R0`\x80R4\x80\x15b\0\0\x14W_\x80\xFD[Pb\0\0\x1Fb\0\0%V[b\0\0\xD9V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80Th\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15b\0\0vW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80T`\x01`\x01`@\x1B\x03\x90\x81\x16\x14b\0\0\xD6W\x80T`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x81\x17\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PV[`\x80QaI\x96b\0\x01\0_9_\x81\x81a,w\x01R\x81\x81a,\xA0\x01Ra.\x85\x01RaI\x96_\xF3\xFE`\x80`@R`\x046\x10a\x02\x07W_5`\xE0\x1C\x80c\x94G\xCF\xD4\x11a\x01\x13W\x80c\xC2X\n-\x11a\0\x9DW\x80c\xCC\xBF\x81\x99\x11a\0mW\x80c\xCC\xBF\x81\x99\x14a\x05\xB2W\x80c\xCC\xEA\xC0\x19\x14a\x05\xD1W\x80c\xD9\xBE-\xE4\x14a\x05\xF0W\x80c\xEE}R\xD1\x14a\x06\x0FW\x80c\xF9\xC6p\xC3\x14a\x06.W_\x80\xFD[\x80c\xC2X\n-\x14a\x05AW\x80c\xC2\xB4)\x86\x14a\x05`W\x80c\xC3\xAA\xAAZ\x14a\x05tW\x80c\xC9\x99\xA8\xB4\x14a\x05\x93W_\x80\xFD[\x80c\xAD<\xB1\xCC\x11a\0\xE3W\x80c\xAD<\xB1\xCC\x14a\x04\xABW\x80c\xB4r+\xC4\x14a\x04\xDBW\x80c\xBA\xC2+\xB8\x14a\x04\xEFW\x80c\xBF\x9B\x16\xC8\x14a\x05\x03W\x80c\xC0\xAEd\xF7\x14a\x05\"W_\x80\xFD[\x80c\x94G\xCF\xD4\x14a\x04EW\x80c\x97l\x98\xB5\x14a\x04dW\x80c\x97o>\xB9\x14a\x04\x83W\x80c\x9D\x1B\x1B\xE1\x14a\x04\x97W_\x80\xFD[\x80cA\xAD\x06\x9C\x11a\x01\x94W\x80cO\x1E\xF2\x86\x11a\x01dW\x80cO\x1E\xF2\x86\x14a\x03\xB5W\x80cR\xD1\x90-\x14a\x03\xC8W\x80c[\xFFv\xD9\x14a\x03\xDCW\x80ce\xB3\x94\xAF\x14a\x04\x08W\x80c~\xAA\xC8\xF2\x14a\x041W_\x80\xFD[\x80cA\xAD\x06\x9C\x14a\x039W\x80cF\xC5\xBB\xBD\x14a\x03XW\x80cG\xE8\"\x95\x14a\x03wW\x80cL\xB9P\xE1\x14a\x03\x96W_\x80\xFD[\x80c&\xCF]\xEF\x11a\x01\xDAW\x80c&\xCF]\xEF\x14a\x02\x99W\x80c(\x1E\x8B\xFE\x14a\x02\xBBW\x80c*8\x89\x98\x14a\x02\xDAW\x80c1\xFFA\xC8\x14a\x02\xEEW\x80c>c\x16\xEA\x14a\x03\x1AW_\x80\xFD[\x80c\r\x8En,\x14a\x02\x0BW\x80c\x1C\xE3\xF9\xBC\x14a\x025W\x80c =\x01\x14\x14a\x02KW\x80c\"\x1C\xDDN\x14a\x02zW[_\x80\xFD[4\x80\x15a\x02\x16W_\x80\xFD[Pa\x02\x1Fa\x06ZV[`@Qa\x02,\x91\x90a9\x91V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02@W_\x80\xFD[Pa\x02Ia\x06\xC6V[\0[4\x80\x15a\x02VW_\x80\xFD[Pa\x02ja\x02e6`\x04a9\xC7V[a\x07\xF0V[`@Q\x90\x15\x15\x81R` \x01a\x02,V[4\x80\x15a\x02\x85W_\x80\xFD[Pa\x02Ia\x02\x946`\x04a:fV[a\x080V[4\x80\x15a\x02\xA4W_\x80\xFD[Pa\x02\xADa\nFV[`@Q\x90\x81R` \x01a\x02,V[4\x80\x15a\x02\xC6W_\x80\xFD[Pa\x02\xADa\x02\xD56`\x04a;\x19V[a\nlV[4\x80\x15a\x02\xE5W_\x80\xFD[Pa\x02\xADa\n\x91V[4\x80\x15a\x02\xF9W_\x80\xFD[Pa\x03\ra\x03\x086`\x04a;0V[a\n\xB7V[`@Qa\x02,\x91\x90a;\xABV[4\x80\x15a\x03%W_\x80\xFD[Pa\x02\xADa\x0346`\x04a;\x19V[a\x0C{V[4\x80\x15a\x03DW_\x80\xFD[Pa\x02\xADa\x03S6`\x04a;\x19V[a\x0C\x97V[4\x80\x15a\x03cW_\x80\xFD[Pa\x02ja\x03r6`\x04a;0V[a\x0C\xDCV[4\x80\x15a\x03\x82W_\x80\xFD[Pa\x02\xADa\x03\x916`\x04a;\x19V[a\r:V[4\x80\x15a\x03\xA1W_\x80\xFD[Pa\x02Ia\x03\xB06`\x04a;\xBDV[a\r_V[a\x02Ia\x03\xC36`\x04a<DV[a\x15\x03V[4\x80\x15a\x03\xD3W_\x80\xFD[Pa\x02\xADa\x15\"V[4\x80\x15a\x03\xE7W_\x80\xFD[Pa\x03\xFBa\x03\xF66`\x04a;\x19V[a\x15=V[`@Qa\x02,\x91\x90a=\x01V[4\x80\x15a\x04\x13W_\x80\xFD[Pa\x04\x1Ca\x15\xC0V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02,V[4\x80\x15a\x04<W_\x80\xFD[Pa\x03\xFBa\x15\xE0V[4\x80\x15a\x04PW_\x80\xFD[Pa\x02ja\x04_6`\x04a;0V[a\x16aV[4\x80\x15a\x04oW_\x80\xFD[Pa\x02Ia\x04~6`\x04a:fV[a\x16\x9FV[4\x80\x15a\x04\x8EW_\x80\xFD[Pa\x02\xADa\x18\x9FV[4\x80\x15a\x04\xA2W_\x80\xFD[Pa\x02\xADa\x18\xB3V[4\x80\x15a\x04\xB6W_\x80\xFD[Pa\x02\x1F`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[4\x80\x15a\x04\xE6W_\x80\xFD[Pa\x02\xADa\x18\xC4V[4\x80\x15a\x04\xFAW_\x80\xFD[Pa\x02Ia\x18\xEAV[4\x80\x15a\x05\x0EW_\x80\xFD[Pa\x02ja\x05\x1D6`\x04a;\x19V[a\x19\xB2V[4\x80\x15a\x05-W_\x80\xFD[Pa\x02Ia\x05<6`\x04a;\x19V[a\x19\xBCV[4\x80\x15a\x05LW_\x80\xFD[Pa\x02Ia\x05[6`\x04a;\x19V[a\x1BjV[4\x80\x15a\x05kW_\x80\xFD[Pa\x02\xADa\x1D+V[4\x80\x15a\x05\x7FW_\x80\xFD[Pa\x02\xADa\x05\x8E6`\x04a;\x19V[a\x1DQV[4\x80\x15a\x05\x9EW_\x80\xFD[Pa\x04\x1Ca\x05\xAD6`\x04a;\x19V[a\x1DvV[4\x80\x15a\x05\xBDW_\x80\xFD[Pa\x02Ia\x05\xCC6`\x04a=cV[a\x1D\xDDV[4\x80\x15a\x05\xDCW_\x80\xFD[Pa\x02ja\x05\xEB6`\x04a>\x12V[a \xE9V[4\x80\x15a\x05\xFBW_\x80\xFD[Pa\x02Ia\x06\n6`\x04a;\x19V[a!NV[4\x80\x15a\x06\x1AW_\x80\xFD[Pa\x02ja\x06)6`\x04a;\x19V[a#\xD7V[4\x80\x15a\x069W_\x80\xFD[Pa\x06Ma\x06H6`\x04a;\x19V[a#\xE1V[`@Qa\x02,\x91\x90a>2V[```@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mProtocolConfig`\x90\x1B\x81RPa\x06\x8C_a%\xA4V[a\x06\x96`\x03a%\xA4V[a\x06\x9F_a%\xA4V[`@Q` \x01a\x06\xB2\x94\x93\x92\x91\x90a>\x94V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x16W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07:\x91\x90a?\x11V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07rW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\x07za&3V[_a\x07\x83a&\xF6V[`\x0B\x81\x01T\x90\x91P_a\x07\x95\x82a'\x1AV[\x90P\x80\x82\x7F\x15\xAA\xAFG^\xF4\x07T?Qd\xF5}\xCFW\xF7\xF98\x16\xF5[\xAEw\xCA\t\xEF\xC4E\xBA@\xEE\xF7\x84\x86`\r\x01T`\x01Ca\x07\xCD\x91\x90a?@V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01`@Q\x80\x91\x03\x90\xA3PPPV[_\x80a\x07\xFAa&\xF6V[`\x0B\x81\x01T_\x90\x81R`\x03\x90\x91\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x90\x96\x16\x83R\x94\x90R\x92\x90\x92 T`\xFF\x16\x92\x91PPV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x16`\x01\x14a\x08\x84W`@QcoOs\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x04\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\x08\xCDWP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a\x08\xEBW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x17`\x01`@\x1B\x17\x81U_a\t\x15a&\xF6V[\x90P_a\t'`\x07`\xF8\x1B`\x01a?gV[\x90Pa\tD\x81a\t<`\x01`\xFB\x1B`\x01a?gV[\x8D\x8D\x8Da'kV[`@Q\x80`@\x01`@R\x80C\x81R` \x01\x8C\x8C\x8C\x8C\x8C\x8C\x8C`@Q` \x01a\tr\x97\x96\x95\x94\x93\x92\x91\x90a@\xB9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x90\x92R_\x84\x81R`\x17\x86\x01\x82R\x91\x90\x91 \x82Q\x81U\x91\x01Q`\x01\x90\x91\x01U`\xF8`\x07\x90\x1B\x81\x7F Mk\x80\x12\x11T\xCD\x87\xD9\x9C\xF5Lc\x9A=\xD0\xA5;0\x84'p\x98\xDE\x97.\xBD\xD3Lk\xE9\x8D\x8D\x8D\x8D\x8D\x8D\x8D`@Qa\t\xEA\x97\x96\x95\x94\x93\x92\x91\x90a@\xB9V[`@Q\x80\x91\x03\x90\xA3PP\x80T`\xFF`@\x1B\x19\x16\x81U`@Q`\x01`\x01`@\x1B\x03\x83\x16\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPPPPV[_\x80a\nPa&\xF6V[`\x0B\x81\x01T_\x90\x81R`\t\x90\x91\x01` R`@\x90 T\x92\x91PPV[_a\nv\x82a'\xBCV[a\n~a&\xF6V[_\x92\x83R`\x07\x01` RP`@\x90 T\x90V[_\x80a\n\x9Ba&\xF6V[`\x0B\x81\x01T_\x90\x81R`\x06\x90\x91\x01` R`@\x90 T\x92\x91PPV[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01R``\x91\x81\x01\x82\x90R\x81\x81\x01\x91\x90\x91Ra\n\xE2\x83a'\xE8V[a\x0B\x02W`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x07iV[a\x0B\na&\xF6V[_\x84\x81R`\x04\x91\x90\x91\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x85R\x90\x83R\x92\x81\x90 \x81Q`\x80\x81\x01\x83R\x81T\x85\x16\x81R`\x01\x82\x01T\x90\x94\x16\x92\x84\x01\x92\x90\x92R`\x02\x82\x01\x80T\x91\x84\x01\x91a\x0Bb\x90aB\x81V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\x8E\x90aB\x81V[\x80\x15a\x0B\xD9W\x80`\x1F\x10a\x0B\xB0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xD9V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xBCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\x0B\xF2\x90aB\x81V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0C\x1E\x90aB\x81V[\x80\x15a\x0CiW\x80`\x1F\x10a\x0C@Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0CiV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0CLW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P[\x92\x91PPV[_a\x0C\x84a&\xF6V[_\x92\x83R`\x14\x01` RP`@\x90 T\x90V[_a\x0C\xA1\x82a(\x06V[a\x0C\xC1W`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07iV[a\x0C\xC9a&\xF6V[_\x92\x83R`\x08\x01` RP`@\x90 T\x90V[_a\x0C\xE6\x83a(\x06V[a\r\x06W`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x07iV[a\r\x0Ea&\xF6V[_\x93\x84R`\x02\x01` \x90\x81R`@\x80\x85 `\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x85R\x92\x90RP\x90 T`\xFF\x16\x90V[_a\rD\x82a'\xBCV[a\rLa&\xF6V[_\x92\x83R`\t\x01` RP`@\x90 T\x90V[_a\rha&\xF6V[\x90P`\x01_\x87\x81R`\x0F\x83\x01` R`@\x90 T`\xFF\x16`\x02\x81\x11\x15a\r\x90Wa\r\x90a?SV[\x14a\r\xB1W`@Qcy\x95\xBB\xCF`\xE0\x1B\x81R`\x04\x81\x01\x87\x90R`$\x01a\x07iV[_\x86\x81R`\x10\x82\x01` \x90\x81R`@\x80\x83 T\x80\x84R`\x02\x85\x01\x83R\x81\x84 3\x85R\x90\x92R\x90\x91 T`\xFF\x16a\x0E\x03W`@Qc\xA3\xF4\xAF\xEB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x88\x90R`D\x01a\x07iV[\x84\x15\x80a\x0E\x0EWP\x82\x15[\x15a\x0E/W`@Qc\xCE\x94@\xDF`\xE0\x1B\x81R`\x04\x81\x01\x88\x90R`$\x01a\x07iV[_\x81\x81R`\x04\x83\x01` \x90\x81R`@\x80\x83 3\x84R\x82R\x80\x83 `\x01\x01T\x81Q`\x01`\xF9\x1B\x93\x81\x01\x93\x90\x93R`!\x83\x01\x85\x90R`A\x80\x84\x01\x8C\x90R\x82Q\x80\x85\x03\x90\x91\x01\x81R`a\x90\x93\x01\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16\x91\x90\x81\x88`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E\xA0Wa\x0E\xA0a<0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\xC9W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x89\x81\x10\x15a\x10WW_a\x0F\x11\x8C\x8C\x84\x81\x81\x10a\x0E\xECWa\x0E\xECaB\xB9V[\x90P` \x02\x81\x01\x90a\x0E\xFE\x91\x90aB\xCDV[a\x0F\x0C\x90`@\x81\x01\x90aB\xEBV[a(>V[\x90P_a\x0Fk\x8D\x8D\x85\x81\x81\x10a\x0F)Wa\x0F)aB\xB9V[\x90P` \x02\x81\x01\x90a\x0F;\x91\x90aB\xCDV[5\x8E\x8E\x86\x81\x81\x10a\x0FNWa\x0FNaB\xB9V[\x90P` \x02\x81\x01\x90a\x0F`\x91\x90aB\xCDV[` \x015\x84\x88a)\xA4V[\x90Pa\x0F\xA9\x87\x82\x8F\x8F\x87\x81\x81\x10a\x0F\x84Wa\x0F\x84aB\xB9V[\x90P` \x02\x81\x01\x90a\x0F\x96\x91\x90aB\xCDV[a\x0F\xA4\x90``\x81\x01\x90aC0V[a+ V[\x8C\x8C\x84\x81\x81\x10a\x0F\xBBWa\x0F\xBBaB\xB9V[\x90P` \x02\x81\x01\x90a\x0F\xCD\x91\x90aB\xCDV[5\x8D\x8D\x85\x81\x81\x10a\x0F\xE0Wa\x0F\xE0aB\xB9V[\x90P` \x02\x81\x01\x90a\x0F\xF2\x91\x90aB\xCDV[` \x015\x83`@Q` \x01a\x10\x1A\x93\x92\x91\x90\x92\x83R` \x83\x01\x91\x90\x91R`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84\x84\x81Q\x81\x10a\x10BWa\x10BaB\xB9V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x0E\xCEV[P_\x87`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10qWa\x10qa<0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\x9AW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x88\x81\x10\x15a\x12\x17W_a\x110\x8B\x8B\x84\x81\x81\x10a\x10\xBDWa\x10\xBDaB\xB9V[\x90P` \x02\x81\x01\x90a\x10\xCF\x91\x90aB\xCDV[5\x8C\x8C\x85\x81\x81\x10a\x10\xE2Wa\x10\xE2aB\xB9V[\x90P` \x02\x81\x01\x90a\x10\xF4\x91\x90aB\xCDV[` \x015\x8D\x8D\x86\x81\x81\x10a\x11\nWa\x11\naB\xB9V[\x90P` \x02\x81\x01\x90a\x11\x1C\x91\x90aB\xCDV[a\x11*\x90`@\x81\x01\x90aC0V[\x89a+\xA5V[\x90Pa\x11I\x87\x82\x8D\x8D\x86\x81\x81\x10a\x0F\x84Wa\x0F\x84aB\xB9V[\x8A\x8A\x83\x81\x81\x10a\x11[Wa\x11[aB\xB9V[\x90P` \x02\x81\x01\x90a\x11m\x91\x90aB\xCDV[5\x8B\x8B\x84\x81\x81\x10a\x11\x80Wa\x11\x80aB\xB9V[\x90P` \x02\x81\x01\x90a\x11\x92\x91\x90aB\xCDV[` \x015\x8C\x8C\x85\x81\x81\x10a\x11\xA8Wa\x11\xA8aB\xB9V[\x90P` \x02\x81\x01\x90a\x11\xBA\x91\x90aB\xCDV[a\x11\xC8\x90`@\x81\x01\x90aC0V[`@Q` \x01a\x11\xDB\x94\x93\x92\x91\x90aCrV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83\x83\x81Q\x81\x10a\x12\x03Wa\x12\x03aB\xB9V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x10\x9FV[P\x81\x81`@Q` \x01a\x12+\x92\x91\x90aC\xCBV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 _\x8F\x81R`\x12\x8B\x01\x84R\x82\x81 `\x01`\x01`\xA0\x1B\x03\x8A\x16\x82R\x90\x93R\x91 T\x90\x94P`\xFF\x16\x15\x92Pa\x12\x9D\x91PPW`@Qc$\xA0\xBB\x1B`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x81\x01\x8A\x90R`D\x01a\x07iV[_\x89\x81R`\x12\x85\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x82R\x80\x83 \x80T`\xFF\x19\x16`\x01\x17\x90U\x8B\x83R`\x13\x87\x01\x82R\x80\x83 \x84\x84R\x90\x91R\x81 \x80T\x82\x90a\x12\xEC\x90aC\xEFV[\x91\x90P\x81\x90U\x90P\x82`\x01`\x01`\xA0\x1B\x03\x16\x8A\x7F~\xDAo\x85\xE2;{\x91\xC0\x19\xB0W\r\x02\xB6c`n\xF9\xD7E\x94\xF7\xE0\x1F\xCF\xBD\xB0\xF4\xE9T\xD5\x84`@Qa\x130\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3_\x84\x81R`\x05\x86\x01` R`@\x90 T\x81\x03a\x14\xF7W_\x84\x81R`\x0E\x86\x01` R`@\x90 \x80T`\xFF\x19\x16`\x03\x17\x90U`\x0B\x85\x01\x84\x90Ua\x13y\x8A\x85a,2V[_\x84\x81R`\x01\x86\x01` R`@\x81 \x80T\x90\x91\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13\xA4Wa\x13\xA4a<0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13\xD7W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x13\xC2W\x90P[P\x90P_[\x82T\x81\x10\x15a\x14\xB2W\x82\x81\x81T\x81\x10a\x13\xF7Wa\x13\xF7aB\xB9V[\x90_R` _ \x90`\x04\x02\x01`\x03\x01\x80Ta\x14\x11\x90aB\x81V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14=\x90aB\x81V[\x80\x15a\x14\x88W\x80`\x1F\x10a\x14_Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x14\x88V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x14kW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x82\x82\x81Q\x81\x10a\x14\x9FWa\x14\x9FaB\xB9V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x13\xDCV[P\x8B\x86\x7F\x1AT{B\xE7,\xD3\xDD\xA0Nj\xDC\xCD\"\0'l\xFE\xF0\x1F\xE2\x13\x8D\x07\xF3\xA7D\x0FAm8\xBC\x8D\x8D\x8D\x8D\x87`@Qa\x14\xEC\x95\x94\x93\x92\x91\x90aE\xC1V[`@Q\x80\x91\x03\x90\xA3PP[PPPPPPPPPPV[a\x15\x0Ba,lV[a\x15\x14\x82a-\x12V[a\x15\x1E\x82\x82a-\xB9V[PPV[_a\x15+a.zV[P_\x80Q` aIv\x839\x81Q\x91R\x90V[``a\x15H\x82a'\xBCV[a\x15Pa&\xF6V[`\x05\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x15\xB4W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x15\x96W[PPPPP\x90P\x91\x90PV[_\x80_a\x15\xCBa&\xF6V[\x90P\x80`\x0B\x01T\x92P\x80`\r\x01T\x91PP\x90\x91V[``_a\x15\xEBa&\xF6V[\x90P\x80`\x05\x01_\x82`\x0B\x01T\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x16VW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x168W[PPPPP\x91PP\x90V[_a\x16k\x83a'\xBCV[a\x16sa&\xF6V[_\x93\x84R`\x03\x01` \x90\x81R`@\x80\x85 `\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x85R\x92\x90RP\x90 T`\xFF\x16\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xEFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x13\x91\x90a?\x11V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x17FW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07iV[a\x17Na&3V[_a\x17Wa&\xF6V[`\x0B\x81\x01T\x81T\x91\x92P\x90_\x90a\x17o\x90`\x01a?gV[\x90Pa\x17}\x81\x8B\x8B\x8Ba.\xC3V[_\x81\x81R`\x0E\x84\x01` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x85\x84R`\t\x87\x01\x83R\x81\x84 T\x90\x87\x01\x90\x92R\x82 Ta\x17\xBD\x91\x90a?@V[\x90P_\x81\x11a\x17\xCDW`\x01a\x17\xCFV[\x80[\x84`\x14\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP`@Q\x80`@\x01`@R\x80C\x81R` \x01\x8C\x8C\x8C\x8C\x8C\x8C\x8C`@Q` \x01a\x18\x14\x97\x96\x95\x94\x93\x92\x91\x90a@\xB9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x90\x92R_\x85\x81R`\x17\x88\x01\x82R\x82\x90 \x83Q\x81U\x92\x01Q`\x01\x90\x92\x01\x91\x90\x91UQ\x83\x90\x83\x90\x7F Mk\x80\x12\x11T\xCD\x87\xD9\x9C\xF5Lc\x9A=\xD0\xA5;0\x84'p\x98\xDE\x97.\xBD\xD3Lk\xE9\x90a\x18\x8A\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90a@\xB9V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPPPPV[_\x80a\x18\xA9a&\xF6V[`\x0B\x01T\x92\x91PPV[_\x80a\x18\xBDa&\xF6V[T\x92\x91PPV[_\x80a\x18\xCEa&\xF6V[`\x0B\x81\x01T_\x90\x81R`\x08\x90\x91\x01` R`@\x90 T\x92\x91PPV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x04\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\x193WP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a\x19QW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x90\x81\x17`\x01`@\x1B\x17`\xFF`@\x1B\x19\x16\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[_a\x0Cu\x82a4\0V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\x0CW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A0\x91\x90a?\x11V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1AcW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07iV[_a\x1Ala&\xF6V[\x90P\x80`\x0B\x01T\x82\x03a\x1A\x95W`@Qc\x03\x0F]U`\xE3\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07iV[a\x1A\x9E\x82a(\x06V[a\x1A\xBEW`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07iV[_\x82\x81R`\n\x82\x01` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x90\x81\x16`\x01\x17\x90\x91U`\x0E\x85\x01\x83R\x81\x84 \x80T\x90\x91\x16\x90U`\x0C\x84\x01T\x80\x84R`\x10\x85\x01\x90\x92R\x90\x91 T\x83\x90\x03a\x1B\x11Wa\x1B\x11\x81a48V[_\x83\x81R`\x14\x83\x01` \x90\x81R`@\x80\x83 \x83\x90U`\x15\x85\x01\x82R\x80\x83 \x83\x90U`\x16\x85\x01\x90\x91R\x80\x82 \x82\x90UQ\x84\x91\x7F\xDA\x07]\t\x19\x8D ~:\x91\x8DK\x8D\xFC\x87\xDF-`\xA0\x0B\xE7\x03\xFD9\xEA\xAC\x90\x96-\xA0\xB7\xF0\x91\xA2PPPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xBAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xDE\x91\x90a?\x11V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1C\x11W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07iV[_a\x1C\x1Aa&\xF6V[\x90P\x80`\r\x01T\x82\x03a\x1CCW`@Qc\xF0\xE7\x81\xC1`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07iV[_\x82\x81R`\x0F\x82\x01` R`@\x81 T`\xFF\x16\x90`\x02\x82`\x02\x81\x11\x15a\x1CkWa\x1Cka?SV[\x14\x90P_`\x01\x83`\x02\x81\x11\x15a\x1C\x83Wa\x1C\x83a?SV[\x14\x80\x15a\x1C\xC2WP`\x03_\x86\x81R`\x10\x86\x01` \x90\x81R`@\x80\x83 T\x83R`\x0E\x88\x01\x90\x91R\x90 T`\xFF\x16`\x03\x81\x11\x15a\x1C\xC0Wa\x1C\xC0a?SV[\x14[\x90P\x81\x15\x80\x15a\x1C\xD0WP\x80\x15[\x15a\x1C\xF1W`@Qcy\x95\xBB\xCF`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R`$\x01a\x07iV[a\x1C\xFA\x85a48V[`@Q\x85\x90\x7F\x03C`\x13\x07_\x8D\x1A\xBB\x17\x81\xDB\xCA\xF4\x18\xFCv\xFCy}\x1A\xB5\x1C8fL\x1AQ\xF3\xCDW\xF9\x90_\x90\xA2PPPPPV[_\x80a\x1D5a&\xF6V[`\x0B\x81\x01T_\x90\x81R`\x07\x90\x91\x01` R`@\x90 T\x92\x91PPV[_a\x1D[\x82a'\xBCV[a\x1Dca&\xF6V[_\x92\x83R`\x06\x01` RP`@\x90 T\x90V[_\x80a\x1D\x81\x83a'\xE8V[a\x1D\xA1W`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x07iV[_a\x1D\xAAa&\xF6V[_\x94\x85R`\x17\x01` \x90\x81R`@\x94\x85\x90 \x85Q\x80\x87\x01\x90\x96R\x80T\x80\x87R`\x01\x90\x91\x01T\x95\x90\x91\x01\x85\x90R\x94\x92PPPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E-W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1EQ\x91\x90a?\x11V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1E\x84W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07iV[\x85_\x03a\x1E\xA4W`@Qc\t\x92\xF7\xAD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x90\x03a\x1E\xC5W`@Qc\xB5H\x91G`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x82\x90\x03a\x1E\xE6W`@Qc/\x94\x14\x11`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`@\x1B\x03\x16_\x03a\x1F\x0FW`@Qc\x02\xFA})`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x82\x81\x10\x15a \xA0W6\x84\x84\x83\x81\x81\x10a\x1F,Wa\x1F,aB\xB9V[``\x02\x91\x90\x91\x01\x91Pa\x1FD\x90P` \x82\x01\x82aF\xCEV[`\x01`\x01`@\x1B\x03\x16_\x03a\x1FlW`@Qc2\x12!u`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1F|``\x82\x01`@\x83\x01aF\xCEV[`\x01`\x01`@\x1B\x03\x16a\x1F\x95`@\x83\x01` \x84\x01aF\xCEV[`\x01`\x01`@\x1B\x03\x16\x11\x15a \x06Wa\x1F\xB1` \x82\x01\x82aF\xCEV[a\x1F\xC1`@\x83\x01` \x84\x01aF\xCEV[a\x1F\xD1``\x84\x01`@\x85\x01aF\xCEV[`@Qcy\x0C\xEE\x07`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x93\x84\x16`\x04\x82\x01R\x91\x83\x16`$\x83\x01R\x90\x91\x16`D\x82\x01R`d\x01a\x07iV[_[\x82\x81\x10\x15a \x96Wa \x1D` \x83\x01\x83aF\xCEV[`\x01`\x01`@\x1B\x03\x16\x86\x86\x83\x81\x81\x10a 8Wa 8aB\xB9V[a N\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91PaF\xCEV[`\x01`\x01`@\x1B\x03\x16\x03a \x8EWa i` \x83\x01\x83aF\xCEV[`@Qc\x06\xC6~G`\xE4\x1B\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01a\x07iV[`\x01\x01a \x08V[PP`\x01\x01a\x1F\x11V[P\x85\x7F\xC4/\x1E\xCA\xC8\xD4\x88\x1E\xF9\xFD3\\\xA9\x8E\xE7%JCk\x92\xBA\x87J`\x9EP\xA7\xD3\x0CH{\x8D\x86\x86\x86\x86\x86`@Qa \xD9\x95\x94\x93\x92\x91\x90aF\xE7V[`@Q\x80\x91\x03\x90\xA2PPPPPPV[_\x80a \xF3a&\xF6V[\x90P`\x02_\x84\x81R`\x0F\x83\x01` R`@\x90 T`\xFF\x16`\x02\x81\x11\x15a!\x1BWa!\x1Ba?SV[\x14\x80\x15a!6WP_\x83\x81R`\x10\x82\x01` R`@\x90 T\x84\x14[\x80\x15a!FWPa!F\x84a4\0V[\x94\x93PPPPV[_a!Wa&\xF6V[\x90P`\x01_\x83\x81R`\x0E\x83\x01` R`@\x90 T`\xFF\x16`\x03\x81\x11\x15a!\x7FWa!\x7Fa?SV[\x14a!\xA0W`@Qc5\x86\xEF\xA1`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07iV[`\x0B\x81\x01T_\x81\x81R`\x02\x83\x01` \x81\x81R`@\x80\x84 3\x80\x86R\x90\x83R\x81\x85 T\x88\x86R\x93\x83R\x81\x85 \x90\x85R\x90\x91R\x90\x91 T`\xFF\x91\x82\x16\x91\x16\x81\x15\x80\x15a!\xE8WP\x80\x15[\x15a\"\x0FW`@Qc\x17\x03\xBF\x1D`\xE3\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01a\x07iV[_\x85\x81R`\x11\x85\x01` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T`\xFF\x16\x15a\"TW`@Qc\x0CK\x0B\x99`\xE3\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01a\x07iV[_\x85\x81R`\x11\x85\x01` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U\x81\x15a\"\xA1W_\x85\x81R`\x16\x85\x01` R`@\x81 \x80T\x90\x91\x90a\"\x9C\x90aC\xEFV[\x90\x91UP[\x80\x15a\"\xC9W_\x85\x81R`\x15\x85\x01` R`@\x81 \x80T\x90\x91\x90a\"\xC4\x90aC\xEFV[\x90\x91UP[`@\x80Q\x83\x15\x15\x81R\x82\x15\x15` \x82\x01R3\x91\x87\x91\x7F\xB7\x9CH\x006\x95\xB6\xEB\xE5U\xAF\xA3o\xAD\x07\x1D\xEE\xEEu\xEB7\x18\xADc\xDEV!\xD3[\xA4KO\x91\x01`@Q\x80\x91\x03\x90\xA3_\x85\x81R`\x01\x85\x01` \x90\x81R`@\x80\x83 T`\x15\x88\x01\x90\x92R\x90\x91 T\x14\x80\x15a#PWP_\x85\x81R`\x14\x85\x01` \x90\x81R`@\x80\x83 T`\x16\x88\x01\x90\x92R\x90\x91 T\x10\x15[\x15a#\xD0W_\x85\x81R`\x0E\x85\x01` R`@\x81 \x80T`\xFF\x19\x16`\x02\x17\x90Ua#x\x86a'\x1AV[\x90P\x80\x86\x7F\x15\xAA\xAFG^\xF4\x07T?Qd\xF5}\xCFW\xF7\xF98\x16\xF5[\xAEw\xCA\t\xEF\xC4E\xBA@\xEE\xF7\x86\x88`\r\x01T`\x01Ca#\xB0\x91\x90a?@V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01`@Q\x80\x91\x03\x90\xA3P[PPPPPV[_a\x0Cu\x82a(\x06V[``a#\xEC\x82a'\xBCV[a#\xF4a&\xF6V[`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a%\x99W_\x84\x81R` \x90\x81\x90 `@\x80Q`\x80\x81\x01\x82R`\x04\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x84R`\x01\x82\x01T\x16\x93\x83\x01\x93\x90\x93R`\x02\x83\x01\x80T\x92\x93\x92\x91\x84\x01\x91a$z\x90aB\x81V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta$\xA6\x90aB\x81V[\x80\x15a$\xF1W\x80`\x1F\x10a$\xC8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a$\xF1V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a$\xD4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta%\n\x90aB\x81V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta%6\x90aB\x81V[\x80\x15a%\x81W\x80`\x1F\x10a%XWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a%\x81V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a%dW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a$%V[PPPP\x90P\x91\x90PV[``_a%\xB0\x83a4kV[`\x01\x01\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a%\xCEWa%\xCEa<0V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a%\xF8W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[_\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a&\x02WP\x93\x92PPPV[_a&<a&\xF6V[\x80T_\x90\x81R`\x0E\x82\x01` R`@\x81 T\x91\x92P`\xFF\x90\x91\x16\x90`\x01\x82`\x03\x81\x11\x15a&kWa&ka?SV[\x14\x80a&\x88WP`\x02\x82`\x03\x81\x11\x15a&\x86Wa&\x86a?SV[\x14[\x90P_`\x01`\x0C\x85\x01T_\x90\x81R`\x0F\x86\x01` R`@\x90 T`\xFF\x16`\x02\x81\x11\x15a&\xB6Wa&\xB6a?SV[\x14\x90P\x81\x80a&\xC2WP\x80[\x15a&\xF0W\x83T`\x0C\x85\x01T`@Qc\x03\xC0\x10_`\xE1\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x07iV[PPPPV[\x7F\x80\xF3XZ\xF8h\x06\xC5wC\x03\xB0l\x1E\xE6@\xAA\x83\xB6\xEF>E\xDFI\xBB&\xC8RE\0\xC2\0\x90V[_\x80a'$a&\xF6V[\x90P\x80`\x0C\x01_\x81Ta'6\x90aC\xEFV[\x91\x82\x90UP_\x81\x81R`\x0F\x83\x01` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16`\x01\x17\x90U`\x10\x90\x94\x01\x90R\x91\x90\x91 \x92\x90\x92UP\x90V[_a'ta&\xF6V[\x90Pa'\x82\x86\x85\x85\x85a.\xC3V[_\x86\x81R`\x0E\x82\x01` R`@\x90 \x80T`\xFF\x19\x16`\x03\x17\x90U`\x0B\x81\x01\x86\x90U`\x0C\x81\x01\x85\x90Ua'\xB4\x85\x87a,2V[PPPPPPV[a'\xC5\x81a4\0V[a'\xE5W`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07iV[PV[_a'\xF1a&\xF6V[_\x92\x83R`\x01\x01` RP`@\x90 T\x15\x15\x90V[_\x80a(\x10a&\xF6V[_\x84\x81R`\x0E\x91\x90\x91\x01` R`@\x90 T`\xFF\x16`\x03\x81\x11\x15a(6Wa(6a?SV[\x14\x15\x92\x91PPV[_\x80\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a(XWa(Xa<0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a(\x81W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15a)sW\x7F\xDD\xD1\x08w.j8\x99\xFE\xB0M\x14\x8A\xE9\x15\xCB\xE3\xEB^\xBD &\x88\x08\x03\x99\xE9\x92\x1A\xC3ak\x85\x85\x83\x81\x81\x10a(\xC1Wa(\xC1aB\xB9V[\x90P` \x02\x81\x01\x90a(\xD3\x91\x90aG\x83V[a(\xE1\x90` \x81\x01\x90aG\x97V[\x86\x86\x84\x81\x81\x10a(\xF3Wa(\xF3aB\xB9V[\x90P` \x02\x81\x01\x90a)\x05\x91\x90aG\x83V[a)\x13\x90` \x81\x01\x90aC0V[`@Qa)!\x92\x91\x90aG\xB0V[`@Q\x90\x81\x90\x03\x81 a)8\x93\x92\x91` \x01aG\xBFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x82\x81Q\x81\x10a)`Wa)`aB\xB9V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a(\x86V[P\x80`@Q` \x01a)\x85\x91\x90aG\xE1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x92\x91PPV[\x80Q` \x80\x83\x01\x91\x90\x91 `@\x80Q\x7F\xBD\x14\x83[\xB4\xAE\x13\xC7\x8E\xCB\x88\xDE\xD2\xC37\x03%\xF3\x9E`\x06\xEB\x94\xFFE\xE9_\x98\xE4\xC8Z*\x93\x81\x01\x93\x90\x93R\x82\x01\x86\x90R``\x82\x01\x85\x90R`\x80\x82\x01\x84\x90R`\xA0\x82\x01R_\x90a+\x17\x90`\xC0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@\x80Q\x80\x82\x01\x82R`\x0E\x81RmProtocolConfig`\x90\x1B` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\x01\x81R`1`\xF8\x1B\x90\x82\x01R\x81Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81\x83\x01R\x7F\xA3\xDE\x18\x80\xCF\x08>\x83\x18\xB7zye\xD0-\xD9v^\x85\xA4\x8EA\x8ADc\xAFz\rW\xB4\xB3\xEE\x81\x84\x01R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xC0\x82\x01\x84R\x80Q\x90\x83\x01 a\x19\x01`\xF0\x1B`\xE0\x83\x01R`\xE2\x82\x01Ra\x01\x02\x80\x82\x01\x94\x90\x94R\x82Q\x80\x82\x03\x90\x94\x01\x84Ra\x01\"\x01\x90\x91R\x81Q\x91\x01 \x90V[\x95\x94PPPPPV[_a+`\x84\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa5B\x92PPPV[\x90P\x84`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a#\xD0W`@Qcx\xB9\xAD\xA3`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R3`$\x82\x01R`D\x01a\x07iV[_a,(\x7F\xA2d\xB3\x18\xE9P\x800\n?\x06\xA6ej\x8E\x7F\xE2O\x99\x03\xF0\xE6\xBC\xCA0~\xFB\xE3\x9CLN\t\x87\x87\x87\x87`@Q` \x01a+\xDF\x92\x91\x90aG\xB0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x89Q\x8A\x83\x01 \x91\x84\x01\x96\x90\x96R\x90\x82\x01\x93\x90\x93R``\x81\x01\x91\x90\x91R`\x80\x81\x01\x92\x90\x92R`\xA0\x82\x01R`\xC0\x01a)\xFDV[\x96\x95PPPPPPV[_a,;a&\xF6V[_\x84\x81R`\x0F\x82\x01` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16`\x02\x17\x90U`\x10\x84\x01\x90\x91R\x90 \x92\x90\x92UP`\r\x01UV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a,\xF2WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a,\xE6_\x80Q` aIv\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a-\x10W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-bW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\x86\x91\x90a?\x11V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a'\xE5W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07iV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a.\x13WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra.\x10\x91\x81\x01\x90aH\x16V[`\x01[a.;W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x07iV[_\x80Q` aIv\x839\x81Q\x91R\x81\x14a.kW`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07iV[a.u\x83\x83a5jV[PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a-\x10W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x82\x90\x03a.\xE3W`@Qb\x1A25`\xE6\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x82\x11\x15a/\x0FW`@Qc\x02\xD4\xE4\xEF`\xE3\x1B\x81R`\x04\x81\x01\x83\x90R`\xFF`$\x82\x01R`D\x01a\x07iV[`@\x80Q\x80\x82\x01\x90\x91R`\x10\x81Ro8:\xB164\xB1\xA22\xB1\xB9<\xB8:4\xB7\xB7`\x81\x1B` \x82\x01Ra/B\x90\x825\x84a5\xBFV[a/z`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m:\xB9\xB2\xB9\"2\xB1\xB9<\xB8:4\xB7\xB7`\x91\x1B\x81RP\x82` \x015\x85\x85\x90Pa5\xBFV[`@\x80Q\x80\x82\x01\x82R`\x06\x81Re5\xB6\xB9\xA3\xB2\xB7`\xD1\x1B` \x82\x01Ra/\xA3\x91\x83\x015\x84a5\xBFV[`@\x80Q\x80\x82\x01\x90\x91R`\x03\x81Rbmpc`\xE8\x1B` \x82\x01Ra/\xCC\x90``\x83\x015\x84a5\xBFV[_a/\xD5a&\xF6V[\x80T\x90\x91P\x85\x11a0\x06W\x80T`@Qc\xEF\xD5_g`\xE0\x1B\x81Ra\x07i\x91\x87\x91`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[\x84\x81U_[\x83\x81\x10\x15a3\xB4W6\x85\x85\x83\x81\x81\x10a0&Wa0&aB\xB9V[\x90P` \x02\x81\x01\x90a08\x91\x90aH-V[`@\x80Q`\x80\x81\x01\x90\x91R\x90\x91P_\x90\x80a0V` \x85\x01\x85a9\xC7V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x01` \x81\x01\x90a0w\x91\x90a9\xC7V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a0\x92`@\x85\x01\x85aC0V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x90\x82RP` \x01a0\xD8``\x85\x01\x85aC0V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x91RP\x80Q\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16a18W`@QcB3@%`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16a1cW`@Qc-\xEC\xCFM`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x88\x81R`\x02\x85\x01` \x90\x81R`@\x80\x83 \x84Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15a1\xB7W\x80Q`@Qc\r\x18\xC4\xFF`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01a\x07iV[_\x88\x81R`\x03\x85\x01` \x90\x81R`@\x80\x83 \x84\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15a2\x10W` \x81\x01Q`@Qc\xF5\x1A\xF6\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01a\x07iV[_\x88\x81R`\x01\x85\x81\x01` \x90\x81R`@\x80\x84 \x80T\x80\x85\x01\x82U\x90\x85R\x93\x82\x90 \x85Q`\x04\x90\x95\x02\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x17\x82U\x92\x86\x01Q\x93\x81\x01\x80T\x90\x93\x16\x93\x90\x94\x16\x92\x90\x92\x17\x90U\x82\x01Q\x82\x91\x90`\x02\x82\x01\x90a2\x80\x90\x82aH\x85V[P``\x82\x01Q`\x03\x82\x01\x90a2\x95\x90\x82aH\x85V[PPP_\x88\x81R`\x02\x80\x86\x01` \x90\x81R`@\x80\x84 \x85Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x86R\x90\x83R\x81\x85 \x80T`\xFF\x19\x90\x81\x16`\x01\x90\x81\x17\x90\x92U\x8E\x87R`\x03\x8B\x01\x85R\x83\x87 \x88\x86\x01\x80Q\x85\x16\x89R\x90\x86R\x84\x88 \x80T\x90\x92\x16\x83\x17\x90\x91U\x8E\x87R`\x04\x8B\x01\x85R\x83\x87 \x88Q\x84\x16\x88R\x90\x94R\x94\x82\x90 \x86Q\x81T\x90\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x82U\x93Q\x95\x81\x01\x80T\x96\x90\x92\x16\x95\x90\x93\x16\x94\x90\x94\x17\x90\x93U\x91\x83\x01Q\x83\x92\x91\x82\x01\x90a3N\x90\x82aH\x85V[P``\x82\x01Q`\x03\x82\x01\x90a3c\x90\x82aH\x85V[PPP_\x88\x81R`\x05\x85\x01` \x90\x81R`@\x82 \x92\x81\x01Q\x83T`\x01\x80\x82\x01\x86U\x94\x84R\x91\x90\x92 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x91\x90\x91\x01\x90Pa0\x0BV[P_\x94\x85R`\x06\x81\x01` \x90\x81R`@\x80\x87 \x845\x90U`\x07\x83\x01\x82R\x80\x87 \x82\x85\x015\x90U`\x08\x83\x01\x82R\x80\x87 \x81\x85\x015\x90U`\t\x90\x92\x01\x90R\x90\x93 ``\x90\x93\x015\x90\x92UPPV[_`\x03a4\x0Ba&\xF6V[_\x84\x81R`\x0E\x91\x90\x91\x01` R`@\x90 T`\xFF\x16`\x03\x81\x11\x15a41Wa41a?SV[\x14\x92\x91PPV[_a4Aa&\xF6V[_\x92\x83R`\x0F\x81\x01` \x90\x81R`@\x80\x85 \x80T`\xFF\x19\x16\x90U`\x10\x90\x92\x01\x90R\x82 \x91\x90\x91UPV[_\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a4\xA9Wr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a4\xD5Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a4\xF3Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a5\x0BWc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a5\x1FWa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a51W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x0CuW`\x01\x01\x92\x91PPV[_\x80_\x80a5P\x86\x86a6\x08V[\x92P\x92P\x92Pa5`\x82\x82a6QV[P\x90\x94\x93PPPPV[a5s\x82a7\tV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a5\xB7Wa.u\x82\x82a7lV[a\x15\x1Ea7\xD5V[\x81_\x03a5\xE1W\x82`@Qc\x1B_\xDB\x07`\xE1\x1B\x81R`\x04\x01a\x07i\x91\x90a9\x91V[\x80\x82\x11\x15a.uW\x82\x82\x82`@Qc\xCA\xA8\x14\xA3`\xE0\x1B\x81R`\x04\x01a\x07i\x93\x92\x91\x90aI@V[_\x80_\x83Q`A\x03a6?W` \x84\x01Q`@\x85\x01Q``\x86\x01Q_\x1Aa61\x88\x82\x85\x85a7\xF4V[\x95P\x95P\x95PPPPa6JV[PP\x81Q_\x91P`\x02\x90[\x92P\x92P\x92V[_\x82`\x03\x81\x11\x15a6dWa6da?SV[\x03a6mWPPV[`\x01\x82`\x03\x81\x11\x15a6\x81Wa6\x81a?SV[\x03a6\x9FW`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a6\xB3Wa6\xB3a?SV[\x03a6\xD4W`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07iV[`\x03\x82`\x03\x81\x11\x15a6\xE8Wa6\xE8a?SV[\x03a\x15\x1EW`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07iV[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a7>W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x07iV[_\x80Q` aIv\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``_\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa7\x88\x91\x90aIdV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a7\xC0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a7\xC5V[``\x91P[P\x91P\x91Pa+\x17\x85\x83\x83a8\xBCV[4\x15a-\x10W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a8-WP_\x91P`\x03\x90P\x82a8\xB2V[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a8~W=_\x80>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a8\xA9WP_\x92P`\x01\x91P\x82\x90Pa8\xB2V[\x92P_\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[``\x82a8\xD1Wa8\xCC\x82a9\x1BV[a9\x14V[\x81Q\x15\x80\x15a8\xE8WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a9\x11W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x07iV[P\x80[\x93\x92PPPV[\x80Q\x15a9+W\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x83\x81\x10\x15a9^W\x81\x81\x01Q\x83\x82\x01R` \x01a9FV[PP_\x91\x01RV[_\x81Q\x80\x84Ra9}\x81` \x86\x01` \x86\x01a9DV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R_a9\x14` \x83\x01\x84a9fV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a'\xE5W_\x80\xFD[\x805a9\xC2\x81a9\xA3V[\x91\x90PV[_` \x82\x84\x03\x12\x15a9\xD7W_\x80\xFD[\x815a9\x14\x81a9\xA3V[_\x80\x83`\x1F\x84\x01\x12a9\xF2W_\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a:\x08W_\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a:\"W_\x80\xFD[\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12a:9W_\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a:OW_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a:\"W_\x80\xFD[_\x80_\x80_\x80_\x87\x89\x03`\xE0\x81\x12\x15a:}W_\x80\xFD[\x885`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a:\x93W_\x80\xFD[a:\x9F\x8C\x83\x8D\x01a9\xE2V[\x90\x9AP\x98P\x88\x91P`\x80`\x1F\x19\x84\x01\x12\x15a:\xB8W_\x80\xFD[` \x8B\x01\x97P`\xA0\x8B\x015\x92P\x80\x83\x11\x15a:\xD1W_\x80\xFD[a:\xDD\x8C\x84\x8D\x01a:)V[\x90\x97P\x95P`\xC0\x8B\x015\x92P\x86\x91P\x80\x83\x11\x15a:\xF8W_\x80\xFD[PPa;\x06\x8A\x82\x8B\x01a9\xE2V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[_` \x82\x84\x03\x12\x15a;)W_\x80\xFD[P5\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a;AW_\x80\xFD[\x825\x91P` \x83\x015a;S\x81a9\xA3V[\x80\x91PP\x92P\x92\x90PV[_`\x01\x80`\xA0\x1B\x03\x80\x83Q\x16\x84R\x80` \x84\x01Q\x16` \x85\x01RP`@\x82\x01Q`\x80`@\x85\x01Ra;\x92`\x80\x85\x01\x82a9fV[\x90P``\x83\x01Q\x84\x82\x03``\x86\x01Ra+\x17\x82\x82a9fV[` \x81R_a9\x14` \x83\x01\x84a;^V[_\x80_\x80_``\x86\x88\x03\x12\x15a;\xD1W_\x80\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a;\xEEW_\x80\xFD[a;\xFA\x89\x83\x8A\x01a9\xE2V[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15a<\x12W_\x80\xFD[Pa<\x1F\x88\x82\x89\x01a9\xE2V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x80`@\x83\x85\x03\x12\x15a<UW_\x80\xFD[\x825a<`\x81a9\xA3V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a<{W_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a<\x8EW_\x80\xFD[\x815\x81\x81\x11\x15a<\xA0Wa<\xA0a<0V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a<\xC8Wa<\xC8a<0V[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a<\xE0W_\x80\xFD[\x82` \x86\x01` \x83\x017_` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a=AW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a=\x1CV[P\x90\x96\x95PPPPPPV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a9\xC2W_\x80\xFD[_\x80_\x80_\x80`\x80\x87\x89\x03\x12\x15a=xW_\x80\xFD[\x865\x95P` \x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a=\x95W_\x80\xFD[a=\xA1\x8A\x83\x8B\x01a:)V[\x90\x97P\x95P`@\x89\x015\x91P\x80\x82\x11\x15a=\xB9W_\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12a=\xCCW_\x80\xFD[\x815\x81\x81\x11\x15a=\xDAW_\x80\xFD[\x8A` ``\x83\x02\x85\x01\x01\x11\x15a=\xEEW_\x80\xFD[` \x83\x01\x95P\x80\x94PPPPa>\x06``\x88\x01a=MV[\x90P\x92\x95P\x92\x95P\x92\x95V[_\x80`@\x83\x85\x03\x12\x15a>#W_\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[_` \x80\x83\x01` \x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P` \x87\x01_[\x82\x81\x10\x15a>\x87W`?\x19\x88\x86\x03\x01\x84Ra>u\x85\x83Qa;^V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a>YV[P\x92\x97\x96PPPPPPPV[_\x85Qa>\xA5\x81\x84` \x8A\x01a9DV[a\x10;`\xF1\x1B\x90\x83\x01\x90\x81R\x85Qa>\xC4\x81`\x02\x84\x01` \x8A\x01a9DV[\x80\x82\x01\x91PP`\x17`\xF9\x1B\x80`\x02\x83\x01R\x85Qa>\xE8\x81`\x03\x85\x01` \x8A\x01a9DV[`\x03\x92\x01\x91\x82\x01R\x83Qa?\x03\x81`\x04\x84\x01` \x88\x01a9DV[\x01`\x04\x01\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a?!W_\x80\xFD[\x81Qa9\x14\x81a9\xA3V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0CuWa\x0Cua?,V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0CuWa\x0Cua?,V[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a?\x8FW_\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a?\xADW_\x80\xFD[\x806\x03\x82\x13\x15a:\"W_\x80\xFD[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x805`\x03\x81\x90\x0B\x81\x14a9\xC2W_\x80\xFD[_\x83\x83\x85R` \x80\x86\x01\x95P\x80\x85`\x05\x1B\x83\x01\x01\x84_[\x87\x81\x10\x15a@\xACW\x84\x83\x03`\x1F\x19\x01\x89R\x8156\x88\x90\x03`^\x19\x01\x81\x12a@0W_\x80\xFD[\x87\x01``a@>\x82\x80a?zV[\x82\x87Ra@N\x83\x88\x01\x82\x84a?\xBBV[\x92PPPa@^\x86\x83\x01\x83a?zV[\x86\x83\x03\x88\x88\x01Ra@p\x83\x82\x84a?\xBBV[\x92PPP`@a@\x82\x81\x84\x01\x84a?zV[\x93P\x86\x83\x03\x82\x88\x01Ra@\x96\x83\x85\x83a?\xBBV[\x9C\x88\x01\x9C\x96PPP\x92\x85\x01\x92PP`\x01\x01a@\x0BV[P\x90\x97\x96PPPPPPPV[`\xE0\x80\x82R\x81\x01\x87\x90R_a\x01\0\x80\x83\x01`\x05\x8A\x90\x1B\x84\x01\x82\x01\x8B\x84[\x8C\x81\x10\x15aB\x19W\x86\x83\x03`\xFF\x19\x01\x84R\x8156\x8F\x90\x03`\xFE\x19\x01\x81\x12a@\xFBW_\x80\xFD[\x8E\x01aA\x17\x84aA\n\x83a9\xB7V[`\x01`\x01`\xA0\x1B\x03\x16\x90RV[` aA$\x81\x83\x01a9\xB7V[`\x01`\x01`\xA0\x1B\x03\x16\x81\x86\x01R`@aA?\x83\x82\x01\x84a?zV[\x89\x83\x89\x01RaAQ\x8A\x89\x01\x82\x84a?\xBBV[\x92PPP``aAc\x81\x85\x01\x85a?zV[\x88\x84\x03\x83\x8A\x01RaAu\x84\x82\x84a?\xBBV[\x93PPPP`\x80aA\x87\x81\x85\x01a?\xE3V[aA\x95\x82\x89\x01\x82`\x03\x0B\x90RV[PP`\xA0aA\xA5\x81\x85\x01\x85a?zV[\x88\x84\x03\x83\x8A\x01RaA\xB7\x84\x82\x84a?\xBBV[\x93PPPP`\xC0aA\xCA\x81\x85\x01\x85a?zV[\x88\x84\x03\x83\x8A\x01RaA\xDC\x84\x82\x84a?\xBBV[\x93PPPPaA\xEE`\xE0\x84\x01\x84a?zV[\x93P\x86\x82\x03`\xE0\x88\x01RaB\x03\x82\x85\x83a?\xBBV[\x97\x83\x01\x97\x96PPP\x92\x90\x92\x01\x91P`\x01\x01a@\xD6V[PPaBI` \x86\x01\x8B\x805\x82R` \x81\x015` \x83\x01R`@\x81\x015`@\x83\x01R``\x81\x015``\x83\x01RPPV[\x84\x81\x03`\xA0\x86\x01RaB\\\x81\x89\x8Ba?\xBBV[\x92PPP\x82\x81\x03`\xC0\x84\x01RaBs\x81\x85\x87a?\xF4V[\x9A\x99PPPPPPPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80aB\x95W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aB\xB3WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`~\x19\x836\x03\x01\x81\x12aB\xE1W_\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aC\0W_\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aC\x19W_\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a:\"W_\x80\xFD[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aCEW_\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aC^W_\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a:\"W_\x80\xFD[\x84\x81R\x83` \x82\x01R```@\x82\x01R_a,(``\x83\x01\x84\x86a?\xBBV[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15aC\xC0W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aC\xA4V[P\x94\x95\x94PPPPPV[`@\x81R_aC\xDD`@\x83\x01\x85aC\x91V[\x82\x81\x03` \x84\x01Ra+\x17\x81\x85aC\x91V[_`\x01\x82\x01aD\0WaD\0a?,V[P`\x01\x01\x90V[\x805`\x04\x81\x10a9\xC2W_\x80\xFD[`\x04\x81\x10aD1WcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90RV[_\x83\x83\x85R` \x80\x86\x01\x95P\x80\x85`\x05\x1B\x83\x01\x01\x84_[\x87\x81\x10\x15a@\xACW\x84\x83\x03`\x1F\x19\x01\x89R\x8156\x88\x90\x03`>\x19\x01\x81\x12aDqW_\x80\xFD[\x87\x01`@aD\x87\x85aD\x82\x84aD\x07V[aD\x15V[aD\x93\x86\x83\x01\x83a?zV[\x92P\x81\x87\x87\x01RaD\xA7\x82\x87\x01\x84\x83a?\xBBV[\x9B\x87\x01\x9B\x95PPP\x91\x84\x01\x91P`\x01\x01aDLV[_\x825`~\x19\x836\x03\x01\x81\x12aD\xD0W_\x80\xFD[\x90\x91\x01\x92\x91PPV[_\x83\x83\x85R` \x80\x86\x01\x95P\x80\x85`\x05\x1B\x83\x01\x01\x84_[\x87\x81\x10\x15a@\xACW\x84\x83\x03`\x1F\x19\x01\x89RaE\x0B\x82\x88aD\xBCV[`\x80\x815\x85R\x85\x82\x015\x86\x86\x01R`@aE'\x81\x84\x01\x84a?zV[\x83\x83\x89\x01RaE9\x84\x89\x01\x82\x84a?\xBBV[\x93PPPP``aEL\x81\x84\x01\x84a?zV[\x93P\x86\x83\x03\x82\x88\x01RaE`\x83\x85\x83a?\xBBV[\x9C\x88\x01\x9C\x96PPP\x92\x85\x01\x92PP`\x01\x01aD\xF0V[_\x82\x82Q\x80\x85R` \x80\x86\x01\x95P` \x82`\x05\x1B\x84\x01\x01` \x86\x01_[\x84\x81\x10\x15a@\xACW`\x1F\x19\x86\x84\x03\x01\x89RaE\xAF\x83\x83Qa9fV[\x98\x84\x01\x98\x92P\x90\x83\x01\x90`\x01\x01aE\x93V[``\x80\x82R\x81\x81\x01\x86\x90R_\x90`\x80\x80\x84\x01`\x05\x89\x81\x1B\x86\x01\x83\x01\x8B\x86[\x8C\x81\x10\x15aF\x95W\x88\x83\x03`\x7F\x19\x01\x85RaE\xFA\x82\x8FaD\xBCV[\x805\x84R` \x80\x82\x015\x81\x86\x01R`@\x80\x83\x015`\x1E\x19\x846\x03\x01\x81\x12aF\x1FW_\x80\xFD[\x83\x01\x82\x81\x01\x905`\x01`\x01`@\x1B\x03\x81\x11\x15aF9W_\x80\xFD[\x80\x89\x1B6\x03\x82\x13\x15aFIW_\x80\xFD[\x8A\x83\x89\x01RaF[\x8B\x89\x01\x82\x84aD5V[\x92PPPaFk\x8A\x84\x01\x84a?zV[\x93P\x86\x82\x03\x8B\x88\x01RaF\x7F\x82\x85\x83a?\xBBV[\x98\x83\x01\x98\x96PPP\x92\x90\x92\x01\x91P`\x01\x01aE\xDFV[PP\x86\x81\x03` \x88\x01RaF\xAA\x81\x8A\x8CaD\xD9V[\x94PPPPP\x82\x81\x03`@\x84\x01RaF\xC2\x81\x85aEvV[\x98\x97PPPPPPPPV[_` \x82\x84\x03\x12\x15aF\xDEW_\x80\xFD[a9\x14\x82a=MV[_``\x80\x83RaF\xFB``\x84\x01\x88\x8Aa?\xBBV[\x83\x81\x03` \x85\x81\x01\x91\x90\x91R\x86\x82R\x87\x91\x81\x01_[\x88\x81\x10\x15aGbW`\x01`\x01`@\x1B\x03\x80aG*\x86a=MV[\x16\x83R\x80aG9\x85\x87\x01a=MV[\x16\x84\x84\x01R`@\x81aGL\x82\x88\x01a=MV[\x16\x90\x84\x01RP\x92\x84\x01\x92\x90\x84\x01\x90`\x01\x01aG\x10V[P\x80\x94PPPPP`\x01`\x01`@\x1B\x03\x83\x16`@\x83\x01R\x96\x95PPPPPPV[_\x825`>\x19\x836\x03\x01\x81\x12aB\xE1W_\x80\xFD[_` \x82\x84\x03\x12\x15aG\xA7W_\x80\xFD[a9\x14\x82aD\x07V[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[\x83\x81R``\x81\x01aG\xD3` \x83\x01\x85aD\x15V[\x82`@\x83\x01R\x94\x93PPPPV[\x81Q_\x90\x82\x90` \x80\x86\x01\x84[\x83\x81\x10\x15aH\nW\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01aG\xEEV[P\x92\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15aH&W_\x80\xFD[PQ\x91\x90PV[_\x825`\xFE\x19\x836\x03\x01\x81\x12aB\xE1W_\x80\xFD[`\x1F\x82\x11\x15a.uW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15aHfWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a#\xD0W_\x81U`\x01\x01aHrV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aH\x9EWaH\x9Ea<0V[aH\xB2\x81aH\xAC\x84TaB\x81V[\x84aHAV[` \x80`\x1F\x83\x11`\x01\x81\x14aH\xE5W_\x84\x15aH\xCEWP\x85\x83\x01Q[_\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua'\xB4V[_\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15aI\x13W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01aH\xF4V[P\x85\x82\x10\x15aI0W\x87\x85\x01Q_\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[``\x81R_aIR``\x83\x01\x86a9fV[` \x83\x01\x94\x90\x94RP`@\x01R\x91\x90PV[_\x82QaB\xE1\x81\x84` \x87\x01a9DV\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405260043610610207575f3560e01c80639447cfd411610113578063c2580a2d1161009d578063ccbf81991161006d578063ccbf8199146105b2578063cceac019146105d1578063d9be2de4146105f0578063ee7d52d11461060f578063f9c670c31461062e575f80fd5b8063c2580a2d14610541578063c2b4298614610560578063c3aaaa5a14610574578063c999a8b414610593575f80fd5b8063ad3cb1cc116100e3578063ad3cb1cc146104ab578063b4722bc4146104db578063bac22bb8146104ef578063bf9b16c814610503578063c0ae64f714610522575f80fd5b80639447cfd414610445578063976c98b514610464578063976f3eb9146104835780639d1b1be114610497575f80fd5b806341ad069c116101945780634f1ef286116101645780634f1ef286146103b557806352d1902d146103c85780635bff76d9146103dc57806365b394af146104085780637eaac8f214610431575f80fd5b806341ad069c1461033957806346c5bbbd1461035857806347e82295146103775780634cb950e114610396575f80fd5b806326cf5def116101da57806326cf5def14610299578063281e8bfe146102bb5780632a388998146102da57806331ff41c8146102ee5780633e6316ea1461031a575f80fd5b80630d8e6e2c1461020b5780631ce3f9bc14610235578063203d01141461024b578063221cdd4e1461027a575b5f80fd5b348015610216575f80fd5b5061021f61065a565b60405161022c9190613991565b60405180910390f35b348015610240575f80fd5b506102496106c6565b005b348015610256575f80fd5b5061026a6102653660046139c7565b6107f0565b604051901515815260200161022c565b348015610285575f80fd5b50610249610294366004613a66565b610830565b3480156102a4575f80fd5b506102ad610a46565b60405190815260200161022c565b3480156102c6575f80fd5b506102ad6102d5366004613b19565b610a6c565b3480156102e5575f80fd5b506102ad610a91565b3480156102f9575f80fd5b5061030d610308366004613b30565b610ab7565b60405161022c9190613bab565b348015610325575f80fd5b506102ad610334366004613b19565b610c7b565b348015610344575f80fd5b506102ad610353366004613b19565b610c97565b348015610363575f80fd5b5061026a610372366004613b30565b610cdc565b348015610382575f80fd5b506102ad610391366004613b19565b610d3a565b3480156103a1575f80fd5b506102496103b0366004613bbd565b610d5f565b6102496103c3366004613c44565b611503565b3480156103d3575f80fd5b506102ad611522565b3480156103e7575f80fd5b506103fb6103f6366004613b19565b61153d565b60405161022c9190613d01565b348015610413575f80fd5b5061041c6115c0565b6040805192835260208301919091520161022c565b34801561043c575f80fd5b506103fb6115e0565b348015610450575f80fd5b5061026a61045f366004613b30565b611661565b34801561046f575f80fd5b5061024961047e366004613a66565b61169f565b34801561048e575f80fd5b506102ad61189f565b3480156104a2575f80fd5b506102ad6118b3565b3480156104b6575f80fd5b5061021f604051806040016040528060058152602001640352e302e360dc1b81525081565b3480156104e6575f80fd5b506102ad6118c4565b3480156104fa575f80fd5b506102496118ea565b34801561050e575f80fd5b5061026a61051d366004613b19565b6119b2565b34801561052d575f80fd5b5061024961053c366004613b19565b6119bc565b34801561054c575f80fd5b5061024961055b366004613b19565b611b6a565b34801561056b575f80fd5b506102ad611d2b565b34801561057f575f80fd5b506102ad61058e366004613b19565b611d51565b34801561059e575f80fd5b5061041c6105ad366004613b19565b611d76565b3480156105bd575f80fd5b506102496105cc366004613d63565b611ddd565b3480156105dc575f80fd5b5061026a6105eb366004613e12565b6120e9565b3480156105fb575f80fd5b5061024961060a366004613b19565b61214e565b34801561061a575f80fd5b5061026a610629366004613b19565b6123d7565b348015610639575f80fd5b5061064d610648366004613b19565b6123e1565b60405161022c9190613e32565b60606040518060400160405280600e81526020016d50726f746f636f6c436f6e66696760901b81525061068c5f6125a4565b61069660036125a4565b61069f5f6125a4565b6040516020016106b29493929190613e94565b604051602081830303815290604052905090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610716573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061073a9190613f11565b6001600160a01b0316336001600160a01b0316146107725760405163021bfda160e41b81523360048201526024015b60405180910390fd5b61077a612633565b5f6107836126f6565b600b8101549091505f6107958261271a565b905080827f15aaaf475ef407543f5164f57dcf57f7f93816f55bae77ca09efc445ba40eef78486600d01546001436107cd9190613f40565b6040805193845260208401929092529082015260600160405180910390a3505050565b5f806107fa6126f6565b600b8101545f9081526003909101602090815260408083206001600160a01b039096168352949052929092205460ff1692915050565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00546001600160401b03166001600160401b031660011461088457604051636f4f731f60e01b815260040160405180910390fd5b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00805460049190600160401b900460ff16806108cd575080546001600160401b03808416911610155b156108eb5760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b03831617600160401b1781555f6109156126f6565b90505f610927600760f81b6001613f67565b90506109448161093c600160fb1b6001613f67565b8d8d8d61276b565b60405180604001604052804381526020018c8c8c8c8c8c8c60405160200161097297969594939291906140b9565b60408051601f1981840301815291815281516020928301209092525f848152601786018252919091208251815591015160019091015560f86007901b817f204d6b80121154cd87d99cf54c639a3dd0a53b3084277098de972ebdd34c6be98d8d8d8d8d8d8d6040516109ea97969594939291906140b9565b60405180910390a35050805460ff60401b191681556040516001600160401b03831681527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a1505050505050505050565b5f80610a506126f6565b600b8101545f9081526009909101602052604090205492915050565b5f610a76826127bc565b610a7e6126f6565b5f92835260070160205250604090205490565b5f80610a9b6126f6565b600b8101545f9081526006909101602052604090205492915050565b604080516080810182525f8082526020820152606091810182905281810191909152610ae2836127e8565b610b02576040516377ddbe8160e01b815260048101849052602401610769565b610b0a6126f6565b5f848152600491909101602090815260408083206001600160a01b03808716855290835292819020815160808101835281548516815260018201549094169284019290925260028201805491840191610b6290614281565b80601f0160208091040260200160405190810160405280929190818152602001828054610b8e90614281565b8015610bd95780601f10610bb057610100808354040283529160200191610bd9565b820191905f5260205f20905b815481529060010190602001808311610bbc57829003601f168201915b50505050508152602001600382018054610bf290614281565b80601f0160208091040260200160405190810160405280929190818152602001828054610c1e90614281565b8015610c695780601f10610c4057610100808354040283529160200191610c69565b820191905f5260205f20905b815481529060010190602001808311610c4c57829003601f168201915b50505050508152505090505b92915050565b5f610c846126f6565b5f92835260140160205250604090205490565b5f610ca182612806565b610cc1576040516377ddbe8160e01b815260048101839052602401610769565b610cc96126f6565b5f92835260080160205250604090205490565b5f610ce683612806565b610d06576040516377ddbe8160e01b815260048101849052602401610769565b610d0e6126f6565b5f938452600201602090815260408085206001600160a01b039490941685529290525090205460ff1690565b5f610d44826127bc565b610d4c6126f6565b5f92835260090160205250604090205490565b5f610d686126f6565b905060015f878152600f8301602052604090205460ff166002811115610d9057610d90613f53565b14610db157604051637995bbcf60e01b815260048101879052602401610769565b5f8681526010820160209081526040808320548084526002850183528184203385529092529091205460ff16610e035760405163a3f4afeb60e01b815233600482015260248101889052604401610769565b841580610e0e575082155b15610e2f5760405163ce9440df60e01b815260048101889052602401610769565b5f81815260048301602090815260408083203384528252808320600101548151600160f91b938101939093526021830185905260418084018c90528251808503909101815260619093019091526001600160a01b0316919081886001600160401b03811115610ea057610ea0613c30565b604051908082528060200260200182016040528015610ec9578160200160208202803683370190505b5090505f5b89811015611057575f610f118c8c84818110610eec57610eec6142b9565b9050602002810190610efe91906142cd565b610f0c9060408101906142eb565b61283e565b90505f610f6b8d8d85818110610f2957610f296142b9565b9050602002810190610f3b91906142cd565b358e8e86818110610f4e57610f4e6142b9565b9050602002810190610f6091906142cd565b6020013584886129a4565b9050610fa987828f8f87818110610f8457610f846142b9565b9050602002810190610f9691906142cd565b610fa4906060810190614330565b612b20565b8c8c84818110610fbb57610fbb6142b9565b9050602002810190610fcd91906142cd565b358d8d85818110610fe057610fe06142b9565b9050602002810190610ff291906142cd565b602001358360405160200161101a939291909283526020830191909152604082015260600190565b60405160208183030381529060405280519060200120848481518110611042576110426142b9565b60209081029190910101525050600101610ece565b505f876001600160401b0381111561107157611071613c30565b60405190808252806020026020018201604052801561109a578160200160208202803683370190505b5090505f5b88811015611217575f6111308b8b848181106110bd576110bd6142b9565b90506020028101906110cf91906142cd565b358c8c858181106110e2576110e26142b9565b90506020028101906110f491906142cd565b602001358d8d8681811061110a5761110a6142b9565b905060200281019061111c91906142cd565b61112a906040810190614330565b89612ba5565b905061114987828d8d86818110610f8457610f846142b9565b8a8a8381811061115b5761115b6142b9565b905060200281019061116d91906142cd565b358b8b84818110611180576111806142b9565b905060200281019061119291906142cd565b602001358c8c858181106111a8576111a86142b9565b90506020028101906111ba91906142cd565b6111c8906040810190614330565b6040516020016111db9493929190614372565b60405160208183030381529060405280519060200120838381518110611203576112036142b9565b60209081029190910101525060010161109f565b50818160405160200161122b9291906143cb565b60408051601f1981840301815291815281516020928301205f8f815260128b0184528281206001600160a01b038a16825290935291205490945060ff1615925061129d915050576040516324a0bb1b60e11b81526001600160a01b0383166004820152602481018a9052604401610769565b5f89815260128501602090815260408083206001600160a01b03861684528252808320805460ff191660011790558b83526013870182528083208484529091528120805482906112ec906143ef565b9190508190559050826001600160a01b03168a7f7eda6f85e23b7b91c019b0570d02b663606ef9d74594f7e01fcfbdb0f4e954d58460405161133091815260200190565b60405180910390a35f84815260058601602052604090205481036114f7575f848152600e860160205260409020805460ff19166003179055600b85018490556113798a85612c32565b5f848152600186016020526040812080549091906001600160401b038111156113a4576113a4613c30565b6040519080825280602002602001820160405280156113d757816020015b60608152602001906001900390816113c25790505b5090505f5b82548110156114b2578281815481106113f7576113f76142b9565b905f5260205f209060040201600301805461141190614281565b80601f016020809104026020016040519081016040528092919081815260200182805461143d90614281565b80156114885780601f1061145f57610100808354040283529160200191611488565b820191905f5260205f20905b81548152906001019060200180831161146b57829003601f168201915b505050505082828151811061149f5761149f6142b9565b60209081029190910101526001016113dc565b508b867f1a547b42e72cd3dda04e6adccd2200276cfef01fe2138d07f3a7440f416d38bc8d8d8d8d876040516114ec9594939291906145c1565b60405180910390a350505b50505050505050505050565b61150b612c6c565b61151482612d12565b61151e8282612db9565b5050565b5f61152b612e7a565b505f8051602061497683398151915290565b6060611548826127bc565b6115506126f6565b6005015f8381526020019081526020015f208054806020026020016040519081016040528092919081815260200182805480156115b457602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311611596575b50505050509050919050565b5f805f6115cb6126f6565b905080600b0154925080600d01549150509091565b60605f6115eb6126f6565b9050806005015f82600b015481526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561165657602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311611638575b505050505091505090565b5f61166b836127bc565b6116736126f6565b5f938452600301602090815260408085206001600160a01b039490941685529290525090205460ff1690565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156116ef573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117139190613f11565b6001600160a01b0316336001600160a01b0316146117465760405163021bfda160e41b8152336004820152602401610769565b61174e612633565b5f6117576126f6565b600b8101548154919250905f9061176f906001613f67565b905061177d818b8b8b612ec3565b5f818152600e840160209081526040808320805460ff19166001908117909155858452600987018352818420549087019092528220546117bd9190613f40565b90505f81116117cd5760016117cf565b805b846014015f8481526020019081526020015f208190555060405180604001604052804381526020018c8c8c8c8c8c8c60405160200161181497969594939291906140b9565b60408051601f1981840301815291815281516020928301209092525f8581526017880182528290208351815592015160019092019190915551839083907f204d6b80121154cd87d99cf54c639a3dd0a53b3084277098de972ebdd34c6be99061188a908f908f908f908f908f908f908f906140b9565b60405180910390a35050505050505050505050565b5f806118a96126f6565b600b015492915050565b5f806118bd6126f6565b5492915050565b5f806118ce6126f6565b600b8101545f9081526008909101602052604090205492915050565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00805460049190600160401b900460ff1680611933575080546001600160401b03808416911610155b156119515760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b038316908117600160401b1760ff60401b191682556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a15050565b5f610c7582613400565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611a0c573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a309190613f11565b6001600160a01b0316336001600160a01b031614611a635760405163021bfda160e41b8152336004820152602401610769565b5f611a6c6126f6565b905080600b01548203611a955760405163030f5d5560e31b815260048101839052602401610769565b611a9e82612806565b611abe576040516377ddbe8160e01b815260048101839052602401610769565b5f828152600a820160209081526040808320805460ff19908116600117909155600e8501835281842080549091169055600c8401548084526010850190925290912054839003611b1157611b1181613438565b5f8381526014830160209081526040808320839055601585018252808320839055601685019091528082208290555184917fda075d09198d207e3a918d4b8dfc87df2d60a00be703fd39eaac90962da0b7f091a2505050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611bba573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611bde9190613f11565b6001600160a01b0316336001600160a01b031614611c115760405163021bfda160e41b8152336004820152602401610769565b5f611c1a6126f6565b905080600d01548203611c435760405163f0e781c160e01b815260048101839052602401610769565b5f828152600f8201602052604081205460ff16906002826002811115611c6b57611c6b613f53565b1490505f6001836002811115611c8357611c83613f53565b148015611cc2575060035f8681526010860160209081526040808320548352600e880190915290205460ff166003811115611cc057611cc0613f53565b145b905081158015611cd0575080155b15611cf157604051637995bbcf60e01b815260048101869052602401610769565b611cfa85613438565b60405185907f03436013075f8d1abb1781dbcaf418fc76fc797d1ab51c38664c1a51f3cd57f9905f90a25050505050565b5f80611d356126f6565b600b8101545f9081526007909101602052604090205492915050565b5f611d5b826127bc565b611d636126f6565b5f92835260060160205250604090205490565b5f80611d81836127e8565b611da1576040516377ddbe8160e01b815260048101849052602401610769565b5f611daa6126f6565b5f948552601701602090815260409485902085518087019096528054808752600190910154959091018590529492505050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611e2d573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e519190613f11565b6001600160a01b0316336001600160a01b031614611e845760405163021bfda160e41b8152336004820152602401610769565b855f03611ea457604051630992f7ad60e01b815260040160405180910390fd5b5f849003611ec55760405163b548914760e01b815260040160405180910390fd5b5f829003611ee657604051632f94141160e21b815260040160405180910390fd5b806001600160401b03165f03611f0f576040516302fa7d2960e31b815260040160405180910390fd5b5f5b828110156120a05736848483818110611f2c57611f2c6142b9565b606002919091019150611f44905060208201826146ce565b6001600160401b03165f03611f6c57604051633212217560e21b815260040160405180910390fd5b611f7c60608201604083016146ce565b6001600160401b0316611f9560408301602084016146ce565b6001600160401b0316111561200657611fb160208201826146ce565b611fc160408301602084016146ce565b611fd160608401604085016146ce565b60405163790cee0760e11b81526001600160401b03938416600482015291831660248301529091166044820152606401610769565b5f5b828110156120965761201d60208301836146ce565b6001600160401b0316868683818110612038576120386142b9565b61204e92602060609092020190810191506146ce565b6001600160401b03160361208e5761206960208301836146ce565b6040516306c67e4760e41b81526001600160401b039091166004820152602401610769565b600101612008565b5050600101611f11565b50857fc42f1ecac8d4881ef9fd335ca98ee7254a436b92ba874a609e50a7d30c487b8d86868686866040516120d99594939291906146e7565b60405180910390a2505050505050565b5f806120f36126f6565b905060025f848152600f8301602052604090205460ff16600281111561211b5761211b613f53565b14801561213657505f83815260108201602052604090205484145b8015612146575061214684613400565b949350505050565b5f6121576126f6565b905060015f838152600e8301602052604090205460ff16600381111561217f5761217f613f53565b146121a057604051633586efa160e01b815260048101839052602401610769565b600b8101545f818152600283016020818152604080842033808652908352818520548886529383528185209085529091529091205460ff9182169116811580156121e8575080155b1561220f57604051631703bf1d60e31b815233600482015260248101869052604401610769565b5f858152601185016020908152604080832033845290915290205460ff161561225457604051630c4b0b9960e31b815233600482015260248101869052604401610769565b5f85815260118501602090815260408083203384529091529020805460ff1916600117905581156122a1575f8581526016850160205260408120805490919061229c906143ef565b909155505b80156122c9575f858152601585016020526040812080549091906122c4906143ef565b909155505b6040805183151581528215156020820152339187917fb79c48003695b6ebe555afa36fad071deeee75eb3718ad63de5621d35ba44b4f910160405180910390a35f858152600185016020908152604080832054601588019092529091205414801561235057505f858152601485016020908152604080832054601688019092529091205410155b156123d0575f858152600e850160205260408120805460ff191660021790556123788661271a565b905080867f15aaaf475ef407543f5164f57dcf57f7f93816f55bae77ca09efc445ba40eef78688600d01546001436123b09190613f40565b6040805193845260208401929092529082015260600160405180910390a3505b5050505050565b5f610c7582612806565b60606123ec826127bc565b6123f46126f6565b6001015f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020015f905b82821015612599575f848152602090819020604080516080810182526004860290920180546001600160a01b039081168452600182015416938301939093526002830180549293929184019161247a90614281565b80601f01602080910402602001604051908101604052809291908181526020018280546124a690614281565b80156124f15780601f106124c8576101008083540402835291602001916124f1565b820191905f5260205f20905b8154815290600101906020018083116124d457829003601f168201915b5050505050815260200160038201805461250a90614281565b80601f016020809104026020016040519081016040528092919081815260200182805461253690614281565b80156125815780601f1061255857610100808354040283529160200191612581565b820191905f5260205f20905b81548152906001019060200180831161256457829003601f168201915b50505050508152505081526020019060010190612425565b505050509050919050565b60605f6125b08361346b565b60010190505f816001600160401b038111156125ce576125ce613c30565b6040519080825280601f01601f1916602001820160405280156125f8576020820181803683370190505b5090508181016020015b5f19016f181899199a1a9b1b9c1cb0b131b232b360811b600a86061a8153600a850494508461260257509392505050565b5f61263c6126f6565b80545f908152600e8201602052604081205491925060ff90911690600182600381111561266b5761266b613f53565b14806126885750600282600381111561268657612686613f53565b145b90505f6001600c8501545f908152600f8601602052604090205460ff1660028111156126b6576126b6613f53565b14905081806126c25750805b156126f0578354600c8501546040516303c0105f60e11b815260048101929092526024820152604401610769565b50505050565b7f80f3585af86806c5774303b06c1ee640aa83b6ef3e45df49bb26c8524500c20090565b5f806127246126f6565b905080600c015f8154612736906143ef565b91829055505f818152600f830160209081526040808320805460ff191660011790556010909401905291909120929092555090565b5f6127746126f6565b905061278286858585612ec3565b5f868152600e820160205260409020805460ff19166003179055600b8101869055600c81018590556127b48587612c32565b505050505050565b6127c581613400565b6127e5576040516377ddbe8160e01b815260048101829052602401610769565b50565b5f6127f16126f6565b5f928352600101602052506040902054151590565b5f806128106126f6565b5f848152600e91909101602052604090205460ff16600381111561283657612836613f53565b141592915050565b5f80826001600160401b0381111561285857612858613c30565b604051908082528060200260200182016040528015612881578160200160208202803683370190505b5090505f5b83811015612973577fddd108772e6a3899feb04d148ae915cbe3eb5ebd202688080399e9921ac3616b8585838181106128c1576128c16142b9565b90506020028101906128d39190614783565b6128e1906020810190614797565b8686848181106128f3576128f36142b9565b90506020028101906129059190614783565b612913906020810190614330565b6040516129219291906147b0565b6040519081900381206129389392916020016147bf565b60405160208183030381529060405280519060200120828281518110612960576129606142b9565b6020908102919091010152600101612886565b508060405160200161298591906147e1565b6040516020818303038152906040528051906020012091505092915050565b8051602080830191909120604080517fbd14835bb4ae13c78ecb88ded2c3370325f39e6006eb94ff45e95f98e4c85a2a938101939093528201869052606082018590526080820184905260a08201525f90612b179060c0015b60405160208183030381529060405280519060200120604080518082018252600e81526d50726f746f636f6c436f6e66696760901b6020918201528151808301835260018152603160f81b9082015281517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f818301527fa3de1880cf083e8318b77a7965d02dd9765e85a48e418a4463af7a0d57b4b3ee818401527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc660608201524660808201523060a0808301919091528351808303909101815260c08201845280519083012061190160f01b60e083015260e2820152610102808201949094528251808203909401845261012201909152815191012090565b95945050505050565b5f612b608484848080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525061354292505050565b9050846001600160a01b0316816001600160a01b0316146123d0576040516378b9ada360e11b81526001600160a01b0382166004820152336024820152604401610769565b5f612c287fa264b318e95080300a3f06a6656a8e7fe24f9903f0e6bcca307efbe39c4c4e0987878787604051602001612bdf9291906147b0565b60408051601f19818403018152828252805160209182012089518a83012091840196909652908201939093526060810191909152608081019290925260a082015260c0016129fd565b9695505050505050565b5f612c3b6126f6565b5f848152600f820160209081526040808320805460ff191660021790556010840190915290209290925550600d0155565b306001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161480612cf257507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316612ce65f80516020614976833981519152546001600160a01b031690565b6001600160a01b031614155b15612d105760405163703e46dd60e11b815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612d62573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612d869190613f11565b6001600160a01b0316336001600160a01b0316146127e55760405163021bfda160e41b8152336004820152602401610769565b816001600160a01b03166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa925050508015612e13575060408051601f3d908101601f19168201909252612e1091810190614816565b60015b612e3b57604051634c9c8ce360e01b81526001600160a01b0383166004820152602401610769565b5f805160206149768339815191528114612e6b57604051632a87526960e21b815260048101829052602401610769565b612e75838361356a565b505050565b306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614612d105760405163703e46dd60e11b815260040160405180910390fd5b5f829003612ee357604051621a323560e61b815260040160405180910390fd5b60ff821115612f0f576040516302d4e4ef60e31b81526004810183905260ff6024820152604401610769565b60408051808201909152601081526f383ab13634b1a232b1b93cb83a34b7b760811b6020820152612f42908235846135bf565b612f7a6040518060400160405280600e81526020016d3ab9b2b92232b1b93cb83a34b7b760911b8152508260200135858590506135bf565b604080518082018252600681526535b6b9a3b2b760d11b6020820152612fa391830135846135bf565b6040805180820190915260038152626d706360e81b6020820152612fcc906060830135846135bf565b5f612fd56126f6565b8054909150851161300657805460405163efd55f6760e01b8152610769918791600401918252602082015260400190565b8481555f5b838110156133b45736858583818110613026576130266142b9565b9050602002810190613038919061482d565b60408051608081019091529091505f908061305660208501856139c7565b6001600160a01b0316815260200183602001602081019061307791906139c7565b6001600160a01b031681526020016130926040850185614330565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152505050908252506020016130d86060850185614330565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250505091525080519091506001600160a01b031661313857604051634233402560e11b815260040160405180910390fd5b60208101516001600160a01b031661316357604051632deccf4d60e01b815260040160405180910390fd5b5f888152600285016020908152604080832084516001600160a01b0316845290915290205460ff16156131b7578051604051630d18c4ff60e41b81526001600160a01b039091166004820152602401610769565b5f8881526003850160209081526040808320848301516001600160a01b0316845290915290205460ff161561321057602081015160405163f51af6bb60e01b81526001600160a01b039091166004820152602401610769565b5f8881526001858101602090815260408084208054808501825590855293829020855160049095020180546001600160a01b03199081166001600160a01b039687161782559286015193810180549093169390941692909217905582015182919060028201906132809082614885565b50606082015160038201906132959082614885565b5050505f88815260028086016020908152604080842085516001600160a01b039081168652908352818520805460ff1990811660019081179092558e875260038b018552838720888601805185168952908652848820805490921683179091558e875260048b01855283872088518416885290945294829020865181549083166001600160a01b0319918216178255935195810180549690921695909316949094179093559183015183929182019061334e9082614885565b50606082015160038201906133639082614885565b5050505f8881526005850160209081526040822092810151835460018082018655948452919092200180546001600160a01b0319166001600160a01b0390921691909117905591909101905061300b565b505f948552600681016020908152604080872084359055600783018252808720828501359055600883018252808720818501359055600990920190529093206060909301359092555050565b5f600361340b6126f6565b5f848152600e91909101602052604090205460ff16600381111561343157613431613f53565b1492915050565b5f6134416126f6565b5f928352600f810160209081526040808520805460ff191690556010909201905282209190915550565b5f8072184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b83106134a95772184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b830492506040015b6d04ee2d6d415b85acef810000000083106134d5576d04ee2d6d415b85acef8100000000830492506020015b662386f26fc1000083106134f357662386f26fc10000830492506010015b6305f5e100831061350b576305f5e100830492506008015b612710831061351f57612710830492506004015b60648310613531576064830492506002015b600a8310610c755760010192915050565b5f805f806135508686613608565b9250925092506135608282613651565b5090949350505050565b61357382613709565b6040516001600160a01b038316907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a28051156135b757612e75828261376c565b61151e6137d5565b815f036135e15782604051631b5fdb0760e11b81526004016107699190613991565b80821115612e755782828260405163caa814a360e01b815260040161076993929190614940565b5f805f835160410361363f576020840151604085015160608601515f1a613631888285856137f4565b95509550955050505061364a565b505081515f91506002905b9250925092565b5f82600381111561366457613664613f53565b0361366d575050565b600182600381111561368157613681613f53565b0361369f5760405163f645eedf60e01b815260040160405180910390fd5b60028260038111156136b3576136b3613f53565b036136d45760405163fce698f760e01b815260048101829052602401610769565b60038260038111156136e8576136e8613f53565b0361151e576040516335e2f38360e21b815260048101829052602401610769565b806001600160a01b03163b5f0361373e57604051634c9c8ce360e01b81526001600160a01b0382166004820152602401610769565b5f8051602061497683398151915280546001600160a01b0319166001600160a01b0392909216919091179055565b60605f80846001600160a01b0316846040516137889190614964565b5f60405180830381855af49150503d805f81146137c0576040519150601f19603f3d011682016040523d82523d5f602084013e6137c5565b606091505b5091509150612b178583836138bc565b3415612d105760405163b398979f60e01b815260040160405180910390fd5b5f80807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084111561382d57505f915060039050826138b2565b604080515f808252602082018084528a905260ff891692820192909252606081018790526080810186905260019060a0016020604051602081039080840390855afa15801561387e573d5f803e3d5ffd5b5050604051601f1901519150506001600160a01b0381166138a957505f9250600191508290506138b2565b92505f91508190505b9450945094915050565b6060826138d1576138cc8261391b565b613914565b81511580156138e857506001600160a01b0384163b155b1561391157604051639996b31560e01b81526001600160a01b0385166004820152602401610769565b50805b9392505050565b80511561392b5780518082602001fd5b60405163d6bda27560e01b815260040160405180910390fd5b5f5b8381101561395e578181015183820152602001613946565b50505f910152565b5f815180845261397d816020860160208601613944565b601f01601f19169290920160200192915050565b602081525f6139146020830184613966565b6001600160a01b03811681146127e5575f80fd5b80356139c2816139a3565b919050565b5f602082840312156139d7575f80fd5b8135613914816139a3565b5f8083601f8401126139f2575f80fd5b5081356001600160401b03811115613a08575f80fd5b6020830191508360208260051b8501011115613a22575f80fd5b9250929050565b5f8083601f840112613a39575f80fd5b5081356001600160401b03811115613a4f575f80fd5b602083019150836020828501011115613a22575f80fd5b5f805f805f805f87890360e0811215613a7d575f80fd5b88356001600160401b0380821115613a93575f80fd5b613a9f8c838d016139e2565b909a5098508891506080601f1984011215613ab8575f80fd5b60208b01975060a08b0135925080831115613ad1575f80fd5b613add8c848d01613a29565b909750955060c08b0135925086915080831115613af8575f80fd5b5050613b068a828b016139e2565b989b979a50959850939692959293505050565b5f60208284031215613b29575f80fd5b5035919050565b5f8060408385031215613b41575f80fd5b823591506020830135613b53816139a3565b809150509250929050565b5f60018060a01b0380835116845280602084015116602085015250604082015160806040850152613b926080850182613966565b905060608301518482036060860152612b178282613966565b602081525f6139146020830184613b5e565b5f805f805f60608688031215613bd1575f80fd5b8535945060208601356001600160401b0380821115613bee575f80fd5b613bfa89838a016139e2565b90965094506040880135915080821115613c12575f80fd5b50613c1f888289016139e2565b969995985093965092949392505050565b634e487b7160e01b5f52604160045260245ffd5b5f8060408385031215613c55575f80fd5b8235613c60816139a3565b915060208301356001600160401b0380821115613c7b575f80fd5b818501915085601f830112613c8e575f80fd5b813581811115613ca057613ca0613c30565b604051601f8201601f19908116603f01168101908382118183101715613cc857613cc8613c30565b81604052828152886020848701011115613ce0575f80fd5b826020860160208301375f6020848301015280955050505050509250929050565b602080825282518282018190525f9190848201906040850190845b81811015613d415783516001600160a01b031683529284019291840191600101613d1c565b50909695505050505050565b80356001600160401b03811681146139c2575f80fd5b5f805f805f8060808789031215613d78575f80fd5b8635955060208701356001600160401b0380821115613d95575f80fd5b613da18a838b01613a29565b90975095506040890135915080821115613db9575f80fd5b818901915089601f830112613dcc575f80fd5b813581811115613dda575f80fd5b8a6020606083028501011115613dee575f80fd5b602083019550809450505050613e0660608801613d4d565b90509295509295509295565b5f8060408385031215613e23575f80fd5b50508035926020909101359150565b5f60208083016020845280855180835260408601915060408160051b8701019250602087015f5b82811015613e8757603f19888603018452613e75858351613b5e565b94509285019290850190600101613e59565b5092979650505050505050565b5f8551613ea5818460208a01613944565b61103b60f11b9083019081528551613ec4816002840160208a01613944565b808201915050601760f91b8060028301528551613ee8816003850160208a01613944565b60039201918201528351613f03816004840160208801613944565b016004019695505050505050565b5f60208284031215613f21575f80fd5b8151613914816139a3565b634e487b7160e01b5f52601160045260245ffd5b81810381811115610c7557610c75613f2c565b634e487b7160e01b5f52602160045260245ffd5b80820180821115610c7557610c75613f2c565b5f808335601e19843603018112613f8f575f80fd5b83016020810192503590506001600160401b03811115613fad575f80fd5b803603821315613a22575f80fd5b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b8035600381900b81146139c2575f80fd5b5f8383855260208086019550808560051b830101845f5b878110156140ac57848303601f19018952813536889003605e19018112614030575f80fd5b8701606061403e8280613f7a565b82875261404e8388018284613fbb565b9250505061405e86830183613f7a565b86830388880152614070838284613fbb565b92505050604061408281840184613f7a565b935086830382880152614096838583613fbb565b9c88019c9650505092850192505060010161400b565b5090979650505050505050565b60e080825281018790525f61010080830160058a901b840182018b845b8c8110156142195786830360ff190184528135368f900360fe190181126140fb575f80fd5b8e016141178461410a836139b7565b6001600160a01b03169052565b60206141248183016139b7565b6001600160a01b031681860152604061413f83820184613f7a565b89838901526141518a89018284613fbb565b92505050606061416381850185613f7a565b888403838a0152614175848284613fbb565b93505050506080614187818501613fe3565b6141958289018260030b9052565b505060a06141a581850185613f7a565b888403838a01526141b7848284613fbb565b935050505060c06141ca81850185613f7a565b888403838a01526141dc848284613fbb565b93505050506141ee60e0840184613f7a565b935086820360e0880152614203828583613fbb565b97830197965050509290920191506001016140d6565b5050614249602086018b803582526020810135602083015260408101356040830152606081013560608301525050565b84810360a086015261425c81898b613fbb565b9250505082810360c0840152614273818587613ff4565b9a9950505050505050505050565b600181811c9082168061429557607f821691505b6020821081036142b357634e487b7160e01b5f52602260045260245ffd5b50919050565b634e487b7160e01b5f52603260045260245ffd5b5f8235607e198336030181126142e1575f80fd5b9190910192915050565b5f808335601e19843603018112614300575f80fd5b8301803591506001600160401b03821115614319575f80fd5b6020019150600581901b3603821315613a22575f80fd5b5f808335601e19843603018112614345575f80fd5b8301803591506001600160401b0382111561435e575f80fd5b602001915036819003821315613a22575f80fd5b848152836020820152606060408201525f612c28606083018486613fbb565b5f815180845260208085019450602084015f5b838110156143c0578151875295820195908201906001016143a4565b509495945050505050565b604081525f6143dd6040830185614391565b8281036020840152612b178185614391565b5f6001820161440057614400613f2c565b5060010190565b8035600481106139c2575f80fd5b6004811061443157634e487b7160e01b5f52602160045260245ffd5b9052565b5f8383855260208086019550808560051b830101845f5b878110156140ac57848303601f19018952813536889003603e19018112614471575f80fd5b870160406144878561448284614407565b614415565b61449386830183613f7a565b925081878701526144a78287018483613fbb565b9b87019b95505050918401915060010161444c565b5f8235607e198336030181126144d0575f80fd5b90910192915050565b5f8383855260208086019550808560051b830101845f5b878110156140ac57848303601f1901895261450b82886144bc565b6080813585528582013586860152604061452781840184613f7a565b83838901526145398489018284613fbb565b9350505050606061454c81840184613f7a565b935086830382880152614560838583613fbb565b9c88019c965050509285019250506001016144f0565b5f8282518085526020808601955060208260051b840101602086015f5b848110156140ac57601f198684030189526145af838351613966565b98840198925090830190600101614593565b60608082528181018690525f906080808401600589811b860183018b865b8c81101561469557888303607f190185526145fa828f6144bc565b8035845260208082013581860152604080830135601e1984360301811261461f575f80fd5b830182810190356001600160401b03811115614639575f80fd5b80891b3603821315614649575f80fd5b8a8389015261465b8b89018284614435565b9250505061466b8a840184613f7a565b93508682038b88015261467f828583613fbb565b98830198965050509290920191506001016145df565b505086810360208801526146aa818a8c6144d9565b94505050505082810360408401526146c28185614576565b98975050505050505050565b5f602082840312156146de575f80fd5b61391482613d4d565b5f60608083526146fb60608401888a613fbb565b838103602085810191909152868252879181015f5b88811015614762576001600160401b038061472a86613d4d565b16835280614739858701613d4d565b168484015260408161474c828801613d4d565b1690840152509284019290840190600101614710565b50809450505050506001600160401b03831660408301529695505050505050565b5f8235603e198336030181126142e1575f80fd5b5f602082840312156147a7575f80fd5b61391482614407565b818382375f9101908152919050565b838152606081016147d36020830185614415565b826040830152949350505050565b81515f9082906020808601845b8381101561480a578151855293820193908201906001016147ee565b50929695505050505050565b5f60208284031215614826575f80fd5b5051919050565b5f823560fe198336030181126142e1575f80fd5b601f821115612e7557805f5260205f20601f840160051c810160208510156148665750805b601f840160051c820191505b818110156123d0575f8155600101614872565b81516001600160401b0381111561489e5761489e613c30565b6148b2816148ac8454614281565b84614841565b602080601f8311600181146148e5575f84156148ce5750858301515b5f19600386901b1c1916600185901b1785556127b4565b5f85815260208120601f198616915b82811015614913578886015182559484019460019091019084016148f4565b508582101561493057878501515f19600388901b60f8161c191681555b5050505050600190811b01905550565b606081525f6149526060830186613966565b60208301949094525060400152919050565b5f82516142e181846020870161394456fe360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x02\x07W_5`\xE0\x1C\x80c\x94G\xCF\xD4\x11a\x01\x13W\x80c\xC2X\n-\x11a\0\x9DW\x80c\xCC\xBF\x81\x99\x11a\0mW\x80c\xCC\xBF\x81\x99\x14a\x05\xB2W\x80c\xCC\xEA\xC0\x19\x14a\x05\xD1W\x80c\xD9\xBE-\xE4\x14a\x05\xF0W\x80c\xEE}R\xD1\x14a\x06\x0FW\x80c\xF9\xC6p\xC3\x14a\x06.W_\x80\xFD[\x80c\xC2X\n-\x14a\x05AW\x80c\xC2\xB4)\x86\x14a\x05`W\x80c\xC3\xAA\xAAZ\x14a\x05tW\x80c\xC9\x99\xA8\xB4\x14a\x05\x93W_\x80\xFD[\x80c\xAD<\xB1\xCC\x11a\0\xE3W\x80c\xAD<\xB1\xCC\x14a\x04\xABW\x80c\xB4r+\xC4\x14a\x04\xDBW\x80c\xBA\xC2+\xB8\x14a\x04\xEFW\x80c\xBF\x9B\x16\xC8\x14a\x05\x03W\x80c\xC0\xAEd\xF7\x14a\x05\"W_\x80\xFD[\x80c\x94G\xCF\xD4\x14a\x04EW\x80c\x97l\x98\xB5\x14a\x04dW\x80c\x97o>\xB9\x14a\x04\x83W\x80c\x9D\x1B\x1B\xE1\x14a\x04\x97W_\x80\xFD[\x80cA\xAD\x06\x9C\x11a\x01\x94W\x80cO\x1E\xF2\x86\x11a\x01dW\x80cO\x1E\xF2\x86\x14a\x03\xB5W\x80cR\xD1\x90-\x14a\x03\xC8W\x80c[\xFFv\xD9\x14a\x03\xDCW\x80ce\xB3\x94\xAF\x14a\x04\x08W\x80c~\xAA\xC8\xF2\x14a\x041W_\x80\xFD[\x80cA\xAD\x06\x9C\x14a\x039W\x80cF\xC5\xBB\xBD\x14a\x03XW\x80cG\xE8\"\x95\x14a\x03wW\x80cL\xB9P\xE1\x14a\x03\x96W_\x80\xFD[\x80c&\xCF]\xEF\x11a\x01\xDAW\x80c&\xCF]\xEF\x14a\x02\x99W\x80c(\x1E\x8B\xFE\x14a\x02\xBBW\x80c*8\x89\x98\x14a\x02\xDAW\x80c1\xFFA\xC8\x14a\x02\xEEW\x80c>c\x16\xEA\x14a\x03\x1AW_\x80\xFD[\x80c\r\x8En,\x14a\x02\x0BW\x80c\x1C\xE3\xF9\xBC\x14a\x025W\x80c =\x01\x14\x14a\x02KW\x80c\"\x1C\xDDN\x14a\x02zW[_\x80\xFD[4\x80\x15a\x02\x16W_\x80\xFD[Pa\x02\x1Fa\x06ZV[`@Qa\x02,\x91\x90a9\x91V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02@W_\x80\xFD[Pa\x02Ia\x06\xC6V[\0[4\x80\x15a\x02VW_\x80\xFD[Pa\x02ja\x02e6`\x04a9\xC7V[a\x07\xF0V[`@Q\x90\x15\x15\x81R` \x01a\x02,V[4\x80\x15a\x02\x85W_\x80\xFD[Pa\x02Ia\x02\x946`\x04a:fV[a\x080V[4\x80\x15a\x02\xA4W_\x80\xFD[Pa\x02\xADa\nFV[`@Q\x90\x81R` \x01a\x02,V[4\x80\x15a\x02\xC6W_\x80\xFD[Pa\x02\xADa\x02\xD56`\x04a;\x19V[a\nlV[4\x80\x15a\x02\xE5W_\x80\xFD[Pa\x02\xADa\n\x91V[4\x80\x15a\x02\xF9W_\x80\xFD[Pa\x03\ra\x03\x086`\x04a;0V[a\n\xB7V[`@Qa\x02,\x91\x90a;\xABV[4\x80\x15a\x03%W_\x80\xFD[Pa\x02\xADa\x0346`\x04a;\x19V[a\x0C{V[4\x80\x15a\x03DW_\x80\xFD[Pa\x02\xADa\x03S6`\x04a;\x19V[a\x0C\x97V[4\x80\x15a\x03cW_\x80\xFD[Pa\x02ja\x03r6`\x04a;0V[a\x0C\xDCV[4\x80\x15a\x03\x82W_\x80\xFD[Pa\x02\xADa\x03\x916`\x04a;\x19V[a\r:V[4\x80\x15a\x03\xA1W_\x80\xFD[Pa\x02Ia\x03\xB06`\x04a;\xBDV[a\r_V[a\x02Ia\x03\xC36`\x04a<DV[a\x15\x03V[4\x80\x15a\x03\xD3W_\x80\xFD[Pa\x02\xADa\x15\"V[4\x80\x15a\x03\xE7W_\x80\xFD[Pa\x03\xFBa\x03\xF66`\x04a;\x19V[a\x15=V[`@Qa\x02,\x91\x90a=\x01V[4\x80\x15a\x04\x13W_\x80\xFD[Pa\x04\x1Ca\x15\xC0V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02,V[4\x80\x15a\x04<W_\x80\xFD[Pa\x03\xFBa\x15\xE0V[4\x80\x15a\x04PW_\x80\xFD[Pa\x02ja\x04_6`\x04a;0V[a\x16aV[4\x80\x15a\x04oW_\x80\xFD[Pa\x02Ia\x04~6`\x04a:fV[a\x16\x9FV[4\x80\x15a\x04\x8EW_\x80\xFD[Pa\x02\xADa\x18\x9FV[4\x80\x15a\x04\xA2W_\x80\xFD[Pa\x02\xADa\x18\xB3V[4\x80\x15a\x04\xB6W_\x80\xFD[Pa\x02\x1F`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[4\x80\x15a\x04\xE6W_\x80\xFD[Pa\x02\xADa\x18\xC4V[4\x80\x15a\x04\xFAW_\x80\xFD[Pa\x02Ia\x18\xEAV[4\x80\x15a\x05\x0EW_\x80\xFD[Pa\x02ja\x05\x1D6`\x04a;\x19V[a\x19\xB2V[4\x80\x15a\x05-W_\x80\xFD[Pa\x02Ia\x05<6`\x04a;\x19V[a\x19\xBCV[4\x80\x15a\x05LW_\x80\xFD[Pa\x02Ia\x05[6`\x04a;\x19V[a\x1BjV[4\x80\x15a\x05kW_\x80\xFD[Pa\x02\xADa\x1D+V[4\x80\x15a\x05\x7FW_\x80\xFD[Pa\x02\xADa\x05\x8E6`\x04a;\x19V[a\x1DQV[4\x80\x15a\x05\x9EW_\x80\xFD[Pa\x04\x1Ca\x05\xAD6`\x04a;\x19V[a\x1DvV[4\x80\x15a\x05\xBDW_\x80\xFD[Pa\x02Ia\x05\xCC6`\x04a=cV[a\x1D\xDDV[4\x80\x15a\x05\xDCW_\x80\xFD[Pa\x02ja\x05\xEB6`\x04a>\x12V[a \xE9V[4\x80\x15a\x05\xFBW_\x80\xFD[Pa\x02Ia\x06\n6`\x04a;\x19V[a!NV[4\x80\x15a\x06\x1AW_\x80\xFD[Pa\x02ja\x06)6`\x04a;\x19V[a#\xD7V[4\x80\x15a\x069W_\x80\xFD[Pa\x06Ma\x06H6`\x04a;\x19V[a#\xE1V[`@Qa\x02,\x91\x90a>2V[```@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mProtocolConfig`\x90\x1B\x81RPa\x06\x8C_a%\xA4V[a\x06\x96`\x03a%\xA4V[a\x06\x9F_a%\xA4V[`@Q` \x01a\x06\xB2\x94\x93\x92\x91\x90a>\x94V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x16W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07:\x91\x90a?\x11V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07rW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\x07za&3V[_a\x07\x83a&\xF6V[`\x0B\x81\x01T\x90\x91P_a\x07\x95\x82a'\x1AV[\x90P\x80\x82\x7F\x15\xAA\xAFG^\xF4\x07T?Qd\xF5}\xCFW\xF7\xF98\x16\xF5[\xAEw\xCA\t\xEF\xC4E\xBA@\xEE\xF7\x84\x86`\r\x01T`\x01Ca\x07\xCD\x91\x90a?@V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01`@Q\x80\x91\x03\x90\xA3PPPV[_\x80a\x07\xFAa&\xF6V[`\x0B\x81\x01T_\x90\x81R`\x03\x90\x91\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x90\x96\x16\x83R\x94\x90R\x92\x90\x92 T`\xFF\x16\x92\x91PPV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x16`\x01\x14a\x08\x84W`@QcoOs\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x04\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\x08\xCDWP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a\x08\xEBW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x17`\x01`@\x1B\x17\x81U_a\t\x15a&\xF6V[\x90P_a\t'`\x07`\xF8\x1B`\x01a?gV[\x90Pa\tD\x81a\t<`\x01`\xFB\x1B`\x01a?gV[\x8D\x8D\x8Da'kV[`@Q\x80`@\x01`@R\x80C\x81R` \x01\x8C\x8C\x8C\x8C\x8C\x8C\x8C`@Q` \x01a\tr\x97\x96\x95\x94\x93\x92\x91\x90a@\xB9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x90\x92R_\x84\x81R`\x17\x86\x01\x82R\x91\x90\x91 \x82Q\x81U\x91\x01Q`\x01\x90\x91\x01U`\xF8`\x07\x90\x1B\x81\x7F Mk\x80\x12\x11T\xCD\x87\xD9\x9C\xF5Lc\x9A=\xD0\xA5;0\x84'p\x98\xDE\x97.\xBD\xD3Lk\xE9\x8D\x8D\x8D\x8D\x8D\x8D\x8D`@Qa\t\xEA\x97\x96\x95\x94\x93\x92\x91\x90a@\xB9V[`@Q\x80\x91\x03\x90\xA3PP\x80T`\xFF`@\x1B\x19\x16\x81U`@Q`\x01`\x01`@\x1B\x03\x83\x16\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPPPPV[_\x80a\nPa&\xF6V[`\x0B\x81\x01T_\x90\x81R`\t\x90\x91\x01` R`@\x90 T\x92\x91PPV[_a\nv\x82a'\xBCV[a\n~a&\xF6V[_\x92\x83R`\x07\x01` RP`@\x90 T\x90V[_\x80a\n\x9Ba&\xF6V[`\x0B\x81\x01T_\x90\x81R`\x06\x90\x91\x01` R`@\x90 T\x92\x91PPV[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01R``\x91\x81\x01\x82\x90R\x81\x81\x01\x91\x90\x91Ra\n\xE2\x83a'\xE8V[a\x0B\x02W`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x07iV[a\x0B\na&\xF6V[_\x84\x81R`\x04\x91\x90\x91\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x85R\x90\x83R\x92\x81\x90 \x81Q`\x80\x81\x01\x83R\x81T\x85\x16\x81R`\x01\x82\x01T\x90\x94\x16\x92\x84\x01\x92\x90\x92R`\x02\x82\x01\x80T\x91\x84\x01\x91a\x0Bb\x90aB\x81V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\x8E\x90aB\x81V[\x80\x15a\x0B\xD9W\x80`\x1F\x10a\x0B\xB0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xD9V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xBCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\x0B\xF2\x90aB\x81V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0C\x1E\x90aB\x81V[\x80\x15a\x0CiW\x80`\x1F\x10a\x0C@Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0CiV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0CLW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P[\x92\x91PPV[_a\x0C\x84a&\xF6V[_\x92\x83R`\x14\x01` RP`@\x90 T\x90V[_a\x0C\xA1\x82a(\x06V[a\x0C\xC1W`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07iV[a\x0C\xC9a&\xF6V[_\x92\x83R`\x08\x01` RP`@\x90 T\x90V[_a\x0C\xE6\x83a(\x06V[a\r\x06W`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x07iV[a\r\x0Ea&\xF6V[_\x93\x84R`\x02\x01` \x90\x81R`@\x80\x85 `\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x85R\x92\x90RP\x90 T`\xFF\x16\x90V[_a\rD\x82a'\xBCV[a\rLa&\xF6V[_\x92\x83R`\t\x01` RP`@\x90 T\x90V[_a\rha&\xF6V[\x90P`\x01_\x87\x81R`\x0F\x83\x01` R`@\x90 T`\xFF\x16`\x02\x81\x11\x15a\r\x90Wa\r\x90a?SV[\x14a\r\xB1W`@Qcy\x95\xBB\xCF`\xE0\x1B\x81R`\x04\x81\x01\x87\x90R`$\x01a\x07iV[_\x86\x81R`\x10\x82\x01` \x90\x81R`@\x80\x83 T\x80\x84R`\x02\x85\x01\x83R\x81\x84 3\x85R\x90\x92R\x90\x91 T`\xFF\x16a\x0E\x03W`@Qc\xA3\xF4\xAF\xEB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x88\x90R`D\x01a\x07iV[\x84\x15\x80a\x0E\x0EWP\x82\x15[\x15a\x0E/W`@Qc\xCE\x94@\xDF`\xE0\x1B\x81R`\x04\x81\x01\x88\x90R`$\x01a\x07iV[_\x81\x81R`\x04\x83\x01` \x90\x81R`@\x80\x83 3\x84R\x82R\x80\x83 `\x01\x01T\x81Q`\x01`\xF9\x1B\x93\x81\x01\x93\x90\x93R`!\x83\x01\x85\x90R`A\x80\x84\x01\x8C\x90R\x82Q\x80\x85\x03\x90\x91\x01\x81R`a\x90\x93\x01\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16\x91\x90\x81\x88`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E\xA0Wa\x0E\xA0a<0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\xC9W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x89\x81\x10\x15a\x10WW_a\x0F\x11\x8C\x8C\x84\x81\x81\x10a\x0E\xECWa\x0E\xECaB\xB9V[\x90P` \x02\x81\x01\x90a\x0E\xFE\x91\x90aB\xCDV[a\x0F\x0C\x90`@\x81\x01\x90aB\xEBV[a(>V[\x90P_a\x0Fk\x8D\x8D\x85\x81\x81\x10a\x0F)Wa\x0F)aB\xB9V[\x90P` \x02\x81\x01\x90a\x0F;\x91\x90aB\xCDV[5\x8E\x8E\x86\x81\x81\x10a\x0FNWa\x0FNaB\xB9V[\x90P` \x02\x81\x01\x90a\x0F`\x91\x90aB\xCDV[` \x015\x84\x88a)\xA4V[\x90Pa\x0F\xA9\x87\x82\x8F\x8F\x87\x81\x81\x10a\x0F\x84Wa\x0F\x84aB\xB9V[\x90P` \x02\x81\x01\x90a\x0F\x96\x91\x90aB\xCDV[a\x0F\xA4\x90``\x81\x01\x90aC0V[a+ V[\x8C\x8C\x84\x81\x81\x10a\x0F\xBBWa\x0F\xBBaB\xB9V[\x90P` \x02\x81\x01\x90a\x0F\xCD\x91\x90aB\xCDV[5\x8D\x8D\x85\x81\x81\x10a\x0F\xE0Wa\x0F\xE0aB\xB9V[\x90P` \x02\x81\x01\x90a\x0F\xF2\x91\x90aB\xCDV[` \x015\x83`@Q` \x01a\x10\x1A\x93\x92\x91\x90\x92\x83R` \x83\x01\x91\x90\x91R`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84\x84\x81Q\x81\x10a\x10BWa\x10BaB\xB9V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x0E\xCEV[P_\x87`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10qWa\x10qa<0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\x9AW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x88\x81\x10\x15a\x12\x17W_a\x110\x8B\x8B\x84\x81\x81\x10a\x10\xBDWa\x10\xBDaB\xB9V[\x90P` \x02\x81\x01\x90a\x10\xCF\x91\x90aB\xCDV[5\x8C\x8C\x85\x81\x81\x10a\x10\xE2Wa\x10\xE2aB\xB9V[\x90P` \x02\x81\x01\x90a\x10\xF4\x91\x90aB\xCDV[` \x015\x8D\x8D\x86\x81\x81\x10a\x11\nWa\x11\naB\xB9V[\x90P` \x02\x81\x01\x90a\x11\x1C\x91\x90aB\xCDV[a\x11*\x90`@\x81\x01\x90aC0V[\x89a+\xA5V[\x90Pa\x11I\x87\x82\x8D\x8D\x86\x81\x81\x10a\x0F\x84Wa\x0F\x84aB\xB9V[\x8A\x8A\x83\x81\x81\x10a\x11[Wa\x11[aB\xB9V[\x90P` \x02\x81\x01\x90a\x11m\x91\x90aB\xCDV[5\x8B\x8B\x84\x81\x81\x10a\x11\x80Wa\x11\x80aB\xB9V[\x90P` \x02\x81\x01\x90a\x11\x92\x91\x90aB\xCDV[` \x015\x8C\x8C\x85\x81\x81\x10a\x11\xA8Wa\x11\xA8aB\xB9V[\x90P` \x02\x81\x01\x90a\x11\xBA\x91\x90aB\xCDV[a\x11\xC8\x90`@\x81\x01\x90aC0V[`@Q` \x01a\x11\xDB\x94\x93\x92\x91\x90aCrV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83\x83\x81Q\x81\x10a\x12\x03Wa\x12\x03aB\xB9V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x10\x9FV[P\x81\x81`@Q` \x01a\x12+\x92\x91\x90aC\xCBV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 _\x8F\x81R`\x12\x8B\x01\x84R\x82\x81 `\x01`\x01`\xA0\x1B\x03\x8A\x16\x82R\x90\x93R\x91 T\x90\x94P`\xFF\x16\x15\x92Pa\x12\x9D\x91PPW`@Qc$\xA0\xBB\x1B`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x81\x01\x8A\x90R`D\x01a\x07iV[_\x89\x81R`\x12\x85\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x82R\x80\x83 \x80T`\xFF\x19\x16`\x01\x17\x90U\x8B\x83R`\x13\x87\x01\x82R\x80\x83 \x84\x84R\x90\x91R\x81 \x80T\x82\x90a\x12\xEC\x90aC\xEFV[\x91\x90P\x81\x90U\x90P\x82`\x01`\x01`\xA0\x1B\x03\x16\x8A\x7F~\xDAo\x85\xE2;{\x91\xC0\x19\xB0W\r\x02\xB6c`n\xF9\xD7E\x94\xF7\xE0\x1F\xCF\xBD\xB0\xF4\xE9T\xD5\x84`@Qa\x130\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3_\x84\x81R`\x05\x86\x01` R`@\x90 T\x81\x03a\x14\xF7W_\x84\x81R`\x0E\x86\x01` R`@\x90 \x80T`\xFF\x19\x16`\x03\x17\x90U`\x0B\x85\x01\x84\x90Ua\x13y\x8A\x85a,2V[_\x84\x81R`\x01\x86\x01` R`@\x81 \x80T\x90\x91\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13\xA4Wa\x13\xA4a<0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13\xD7W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x13\xC2W\x90P[P\x90P_[\x82T\x81\x10\x15a\x14\xB2W\x82\x81\x81T\x81\x10a\x13\xF7Wa\x13\xF7aB\xB9V[\x90_R` _ \x90`\x04\x02\x01`\x03\x01\x80Ta\x14\x11\x90aB\x81V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14=\x90aB\x81V[\x80\x15a\x14\x88W\x80`\x1F\x10a\x14_Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x14\x88V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x14kW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x82\x82\x81Q\x81\x10a\x14\x9FWa\x14\x9FaB\xB9V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x13\xDCV[P\x8B\x86\x7F\x1AT{B\xE7,\xD3\xDD\xA0Nj\xDC\xCD\"\0'l\xFE\xF0\x1F\xE2\x13\x8D\x07\xF3\xA7D\x0FAm8\xBC\x8D\x8D\x8D\x8D\x87`@Qa\x14\xEC\x95\x94\x93\x92\x91\x90aE\xC1V[`@Q\x80\x91\x03\x90\xA3PP[PPPPPPPPPPV[a\x15\x0Ba,lV[a\x15\x14\x82a-\x12V[a\x15\x1E\x82\x82a-\xB9V[PPV[_a\x15+a.zV[P_\x80Q` aIv\x839\x81Q\x91R\x90V[``a\x15H\x82a'\xBCV[a\x15Pa&\xF6V[`\x05\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x15\xB4W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x15\x96W[PPPPP\x90P\x91\x90PV[_\x80_a\x15\xCBa&\xF6V[\x90P\x80`\x0B\x01T\x92P\x80`\r\x01T\x91PP\x90\x91V[``_a\x15\xEBa&\xF6V[\x90P\x80`\x05\x01_\x82`\x0B\x01T\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x16VW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x168W[PPPPP\x91PP\x90V[_a\x16k\x83a'\xBCV[a\x16sa&\xF6V[_\x93\x84R`\x03\x01` \x90\x81R`@\x80\x85 `\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x85R\x92\x90RP\x90 T`\xFF\x16\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xEFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x13\x91\x90a?\x11V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x17FW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07iV[a\x17Na&3V[_a\x17Wa&\xF6V[`\x0B\x81\x01T\x81T\x91\x92P\x90_\x90a\x17o\x90`\x01a?gV[\x90Pa\x17}\x81\x8B\x8B\x8Ba.\xC3V[_\x81\x81R`\x0E\x84\x01` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x85\x84R`\t\x87\x01\x83R\x81\x84 T\x90\x87\x01\x90\x92R\x82 Ta\x17\xBD\x91\x90a?@V[\x90P_\x81\x11a\x17\xCDW`\x01a\x17\xCFV[\x80[\x84`\x14\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP`@Q\x80`@\x01`@R\x80C\x81R` \x01\x8C\x8C\x8C\x8C\x8C\x8C\x8C`@Q` \x01a\x18\x14\x97\x96\x95\x94\x93\x92\x91\x90a@\xB9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x90\x92R_\x85\x81R`\x17\x88\x01\x82R\x82\x90 \x83Q\x81U\x92\x01Q`\x01\x90\x92\x01\x91\x90\x91UQ\x83\x90\x83\x90\x7F Mk\x80\x12\x11T\xCD\x87\xD9\x9C\xF5Lc\x9A=\xD0\xA5;0\x84'p\x98\xDE\x97.\xBD\xD3Lk\xE9\x90a\x18\x8A\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90a@\xB9V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPPPPV[_\x80a\x18\xA9a&\xF6V[`\x0B\x01T\x92\x91PPV[_\x80a\x18\xBDa&\xF6V[T\x92\x91PPV[_\x80a\x18\xCEa&\xF6V[`\x0B\x81\x01T_\x90\x81R`\x08\x90\x91\x01` R`@\x90 T\x92\x91PPV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x04\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\x193WP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a\x19QW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x90\x81\x17`\x01`@\x1B\x17`\xFF`@\x1B\x19\x16\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[_a\x0Cu\x82a4\0V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\x0CW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A0\x91\x90a?\x11V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1AcW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07iV[_a\x1Ala&\xF6V[\x90P\x80`\x0B\x01T\x82\x03a\x1A\x95W`@Qc\x03\x0F]U`\xE3\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07iV[a\x1A\x9E\x82a(\x06V[a\x1A\xBEW`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07iV[_\x82\x81R`\n\x82\x01` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x90\x81\x16`\x01\x17\x90\x91U`\x0E\x85\x01\x83R\x81\x84 \x80T\x90\x91\x16\x90U`\x0C\x84\x01T\x80\x84R`\x10\x85\x01\x90\x92R\x90\x91 T\x83\x90\x03a\x1B\x11Wa\x1B\x11\x81a48V[_\x83\x81R`\x14\x83\x01` \x90\x81R`@\x80\x83 \x83\x90U`\x15\x85\x01\x82R\x80\x83 \x83\x90U`\x16\x85\x01\x90\x91R\x80\x82 \x82\x90UQ\x84\x91\x7F\xDA\x07]\t\x19\x8D ~:\x91\x8DK\x8D\xFC\x87\xDF-`\xA0\x0B\xE7\x03\xFD9\xEA\xAC\x90\x96-\xA0\xB7\xF0\x91\xA2PPPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xBAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xDE\x91\x90a?\x11V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1C\x11W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07iV[_a\x1C\x1Aa&\xF6V[\x90P\x80`\r\x01T\x82\x03a\x1CCW`@Qc\xF0\xE7\x81\xC1`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07iV[_\x82\x81R`\x0F\x82\x01` R`@\x81 T`\xFF\x16\x90`\x02\x82`\x02\x81\x11\x15a\x1CkWa\x1Cka?SV[\x14\x90P_`\x01\x83`\x02\x81\x11\x15a\x1C\x83Wa\x1C\x83a?SV[\x14\x80\x15a\x1C\xC2WP`\x03_\x86\x81R`\x10\x86\x01` \x90\x81R`@\x80\x83 T\x83R`\x0E\x88\x01\x90\x91R\x90 T`\xFF\x16`\x03\x81\x11\x15a\x1C\xC0Wa\x1C\xC0a?SV[\x14[\x90P\x81\x15\x80\x15a\x1C\xD0WP\x80\x15[\x15a\x1C\xF1W`@Qcy\x95\xBB\xCF`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R`$\x01a\x07iV[a\x1C\xFA\x85a48V[`@Q\x85\x90\x7F\x03C`\x13\x07_\x8D\x1A\xBB\x17\x81\xDB\xCA\xF4\x18\xFCv\xFCy}\x1A\xB5\x1C8fL\x1AQ\xF3\xCDW\xF9\x90_\x90\xA2PPPPPV[_\x80a\x1D5a&\xF6V[`\x0B\x81\x01T_\x90\x81R`\x07\x90\x91\x01` R`@\x90 T\x92\x91PPV[_a\x1D[\x82a'\xBCV[a\x1Dca&\xF6V[_\x92\x83R`\x06\x01` RP`@\x90 T\x90V[_\x80a\x1D\x81\x83a'\xE8V[a\x1D\xA1W`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x07iV[_a\x1D\xAAa&\xF6V[_\x94\x85R`\x17\x01` \x90\x81R`@\x94\x85\x90 \x85Q\x80\x87\x01\x90\x96R\x80T\x80\x87R`\x01\x90\x91\x01T\x95\x90\x91\x01\x85\x90R\x94\x92PPPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E-W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1EQ\x91\x90a?\x11V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1E\x84W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07iV[\x85_\x03a\x1E\xA4W`@Qc\t\x92\xF7\xAD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x90\x03a\x1E\xC5W`@Qc\xB5H\x91G`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x82\x90\x03a\x1E\xE6W`@Qc/\x94\x14\x11`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`@\x1B\x03\x16_\x03a\x1F\x0FW`@Qc\x02\xFA})`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x82\x81\x10\x15a \xA0W6\x84\x84\x83\x81\x81\x10a\x1F,Wa\x1F,aB\xB9V[``\x02\x91\x90\x91\x01\x91Pa\x1FD\x90P` \x82\x01\x82aF\xCEV[`\x01`\x01`@\x1B\x03\x16_\x03a\x1FlW`@Qc2\x12!u`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1F|``\x82\x01`@\x83\x01aF\xCEV[`\x01`\x01`@\x1B\x03\x16a\x1F\x95`@\x83\x01` \x84\x01aF\xCEV[`\x01`\x01`@\x1B\x03\x16\x11\x15a \x06Wa\x1F\xB1` \x82\x01\x82aF\xCEV[a\x1F\xC1`@\x83\x01` \x84\x01aF\xCEV[a\x1F\xD1``\x84\x01`@\x85\x01aF\xCEV[`@Qcy\x0C\xEE\x07`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x93\x84\x16`\x04\x82\x01R\x91\x83\x16`$\x83\x01R\x90\x91\x16`D\x82\x01R`d\x01a\x07iV[_[\x82\x81\x10\x15a \x96Wa \x1D` \x83\x01\x83aF\xCEV[`\x01`\x01`@\x1B\x03\x16\x86\x86\x83\x81\x81\x10a 8Wa 8aB\xB9V[a N\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91PaF\xCEV[`\x01`\x01`@\x1B\x03\x16\x03a \x8EWa i` \x83\x01\x83aF\xCEV[`@Qc\x06\xC6~G`\xE4\x1B\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01a\x07iV[`\x01\x01a \x08V[PP`\x01\x01a\x1F\x11V[P\x85\x7F\xC4/\x1E\xCA\xC8\xD4\x88\x1E\xF9\xFD3\\\xA9\x8E\xE7%JCk\x92\xBA\x87J`\x9EP\xA7\xD3\x0CH{\x8D\x86\x86\x86\x86\x86`@Qa \xD9\x95\x94\x93\x92\x91\x90aF\xE7V[`@Q\x80\x91\x03\x90\xA2PPPPPPV[_\x80a \xF3a&\xF6V[\x90P`\x02_\x84\x81R`\x0F\x83\x01` R`@\x90 T`\xFF\x16`\x02\x81\x11\x15a!\x1BWa!\x1Ba?SV[\x14\x80\x15a!6WP_\x83\x81R`\x10\x82\x01` R`@\x90 T\x84\x14[\x80\x15a!FWPa!F\x84a4\0V[\x94\x93PPPPV[_a!Wa&\xF6V[\x90P`\x01_\x83\x81R`\x0E\x83\x01` R`@\x90 T`\xFF\x16`\x03\x81\x11\x15a!\x7FWa!\x7Fa?SV[\x14a!\xA0W`@Qc5\x86\xEF\xA1`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07iV[`\x0B\x81\x01T_\x81\x81R`\x02\x83\x01` \x81\x81R`@\x80\x84 3\x80\x86R\x90\x83R\x81\x85 T\x88\x86R\x93\x83R\x81\x85 \x90\x85R\x90\x91R\x90\x91 T`\xFF\x91\x82\x16\x91\x16\x81\x15\x80\x15a!\xE8WP\x80\x15[\x15a\"\x0FW`@Qc\x17\x03\xBF\x1D`\xE3\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01a\x07iV[_\x85\x81R`\x11\x85\x01` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T`\xFF\x16\x15a\"TW`@Qc\x0CK\x0B\x99`\xE3\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01a\x07iV[_\x85\x81R`\x11\x85\x01` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U\x81\x15a\"\xA1W_\x85\x81R`\x16\x85\x01` R`@\x81 \x80T\x90\x91\x90a\"\x9C\x90aC\xEFV[\x90\x91UP[\x80\x15a\"\xC9W_\x85\x81R`\x15\x85\x01` R`@\x81 \x80T\x90\x91\x90a\"\xC4\x90aC\xEFV[\x90\x91UP[`@\x80Q\x83\x15\x15\x81R\x82\x15\x15` \x82\x01R3\x91\x87\x91\x7F\xB7\x9CH\x006\x95\xB6\xEB\xE5U\xAF\xA3o\xAD\x07\x1D\xEE\xEEu\xEB7\x18\xADc\xDEV!\xD3[\xA4KO\x91\x01`@Q\x80\x91\x03\x90\xA3_\x85\x81R`\x01\x85\x01` \x90\x81R`@\x80\x83 T`\x15\x88\x01\x90\x92R\x90\x91 T\x14\x80\x15a#PWP_\x85\x81R`\x14\x85\x01` \x90\x81R`@\x80\x83 T`\x16\x88\x01\x90\x92R\x90\x91 T\x10\x15[\x15a#\xD0W_\x85\x81R`\x0E\x85\x01` R`@\x81 \x80T`\xFF\x19\x16`\x02\x17\x90Ua#x\x86a'\x1AV[\x90P\x80\x86\x7F\x15\xAA\xAFG^\xF4\x07T?Qd\xF5}\xCFW\xF7\xF98\x16\xF5[\xAEw\xCA\t\xEF\xC4E\xBA@\xEE\xF7\x86\x88`\r\x01T`\x01Ca#\xB0\x91\x90a?@V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01`@Q\x80\x91\x03\x90\xA3P[PPPPPV[_a\x0Cu\x82a(\x06V[``a#\xEC\x82a'\xBCV[a#\xF4a&\xF6V[`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a%\x99W_\x84\x81R` \x90\x81\x90 `@\x80Q`\x80\x81\x01\x82R`\x04\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x84R`\x01\x82\x01T\x16\x93\x83\x01\x93\x90\x93R`\x02\x83\x01\x80T\x92\x93\x92\x91\x84\x01\x91a$z\x90aB\x81V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta$\xA6\x90aB\x81V[\x80\x15a$\xF1W\x80`\x1F\x10a$\xC8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a$\xF1V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a$\xD4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta%\n\x90aB\x81V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta%6\x90aB\x81V[\x80\x15a%\x81W\x80`\x1F\x10a%XWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a%\x81V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a%dW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a$%V[PPPP\x90P\x91\x90PV[``_a%\xB0\x83a4kV[`\x01\x01\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a%\xCEWa%\xCEa<0V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a%\xF8W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[_\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a&\x02WP\x93\x92PPPV[_a&<a&\xF6V[\x80T_\x90\x81R`\x0E\x82\x01` R`@\x81 T\x91\x92P`\xFF\x90\x91\x16\x90`\x01\x82`\x03\x81\x11\x15a&kWa&ka?SV[\x14\x80a&\x88WP`\x02\x82`\x03\x81\x11\x15a&\x86Wa&\x86a?SV[\x14[\x90P_`\x01`\x0C\x85\x01T_\x90\x81R`\x0F\x86\x01` R`@\x90 T`\xFF\x16`\x02\x81\x11\x15a&\xB6Wa&\xB6a?SV[\x14\x90P\x81\x80a&\xC2WP\x80[\x15a&\xF0W\x83T`\x0C\x85\x01T`@Qc\x03\xC0\x10_`\xE1\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x07iV[PPPPV[\x7F\x80\xF3XZ\xF8h\x06\xC5wC\x03\xB0l\x1E\xE6@\xAA\x83\xB6\xEF>E\xDFI\xBB&\xC8RE\0\xC2\0\x90V[_\x80a'$a&\xF6V[\x90P\x80`\x0C\x01_\x81Ta'6\x90aC\xEFV[\x91\x82\x90UP_\x81\x81R`\x0F\x83\x01` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16`\x01\x17\x90U`\x10\x90\x94\x01\x90R\x91\x90\x91 \x92\x90\x92UP\x90V[_a'ta&\xF6V[\x90Pa'\x82\x86\x85\x85\x85a.\xC3V[_\x86\x81R`\x0E\x82\x01` R`@\x90 \x80T`\xFF\x19\x16`\x03\x17\x90U`\x0B\x81\x01\x86\x90U`\x0C\x81\x01\x85\x90Ua'\xB4\x85\x87a,2V[PPPPPPV[a'\xC5\x81a4\0V[a'\xE5W`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07iV[PV[_a'\xF1a&\xF6V[_\x92\x83R`\x01\x01` RP`@\x90 T\x15\x15\x90V[_\x80a(\x10a&\xF6V[_\x84\x81R`\x0E\x91\x90\x91\x01` R`@\x90 T`\xFF\x16`\x03\x81\x11\x15a(6Wa(6a?SV[\x14\x15\x92\x91PPV[_\x80\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a(XWa(Xa<0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a(\x81W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15a)sW\x7F\xDD\xD1\x08w.j8\x99\xFE\xB0M\x14\x8A\xE9\x15\xCB\xE3\xEB^\xBD &\x88\x08\x03\x99\xE9\x92\x1A\xC3ak\x85\x85\x83\x81\x81\x10a(\xC1Wa(\xC1aB\xB9V[\x90P` \x02\x81\x01\x90a(\xD3\x91\x90aG\x83V[a(\xE1\x90` \x81\x01\x90aG\x97V[\x86\x86\x84\x81\x81\x10a(\xF3Wa(\xF3aB\xB9V[\x90P` \x02\x81\x01\x90a)\x05\x91\x90aG\x83V[a)\x13\x90` \x81\x01\x90aC0V[`@Qa)!\x92\x91\x90aG\xB0V[`@Q\x90\x81\x90\x03\x81 a)8\x93\x92\x91` \x01aG\xBFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x82\x81Q\x81\x10a)`Wa)`aB\xB9V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a(\x86V[P\x80`@Q` \x01a)\x85\x91\x90aG\xE1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x92\x91PPV[\x80Q` \x80\x83\x01\x91\x90\x91 `@\x80Q\x7F\xBD\x14\x83[\xB4\xAE\x13\xC7\x8E\xCB\x88\xDE\xD2\xC37\x03%\xF3\x9E`\x06\xEB\x94\xFFE\xE9_\x98\xE4\xC8Z*\x93\x81\x01\x93\x90\x93R\x82\x01\x86\x90R``\x82\x01\x85\x90R`\x80\x82\x01\x84\x90R`\xA0\x82\x01R_\x90a+\x17\x90`\xC0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@\x80Q\x80\x82\x01\x82R`\x0E\x81RmProtocolConfig`\x90\x1B` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\x01\x81R`1`\xF8\x1B\x90\x82\x01R\x81Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81\x83\x01R\x7F\xA3\xDE\x18\x80\xCF\x08>\x83\x18\xB7zye\xD0-\xD9v^\x85\xA4\x8EA\x8ADc\xAFz\rW\xB4\xB3\xEE\x81\x84\x01R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xC0\x82\x01\x84R\x80Q\x90\x83\x01 a\x19\x01`\xF0\x1B`\xE0\x83\x01R`\xE2\x82\x01Ra\x01\x02\x80\x82\x01\x94\x90\x94R\x82Q\x80\x82\x03\x90\x94\x01\x84Ra\x01\"\x01\x90\x91R\x81Q\x91\x01 \x90V[\x95\x94PPPPPV[_a+`\x84\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa5B\x92PPPV[\x90P\x84`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a#\xD0W`@Qcx\xB9\xAD\xA3`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R3`$\x82\x01R`D\x01a\x07iV[_a,(\x7F\xA2d\xB3\x18\xE9P\x800\n?\x06\xA6ej\x8E\x7F\xE2O\x99\x03\xF0\xE6\xBC\xCA0~\xFB\xE3\x9CLN\t\x87\x87\x87\x87`@Q` \x01a+\xDF\x92\x91\x90aG\xB0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x89Q\x8A\x83\x01 \x91\x84\x01\x96\x90\x96R\x90\x82\x01\x93\x90\x93R``\x81\x01\x91\x90\x91R`\x80\x81\x01\x92\x90\x92R`\xA0\x82\x01R`\xC0\x01a)\xFDV[\x96\x95PPPPPPV[_a,;a&\xF6V[_\x84\x81R`\x0F\x82\x01` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16`\x02\x17\x90U`\x10\x84\x01\x90\x91R\x90 \x92\x90\x92UP`\r\x01UV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a,\xF2WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a,\xE6_\x80Q` aIv\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a-\x10W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-bW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\x86\x91\x90a?\x11V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a'\xE5W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07iV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a.\x13WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra.\x10\x91\x81\x01\x90aH\x16V[`\x01[a.;W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x07iV[_\x80Q` aIv\x839\x81Q\x91R\x81\x14a.kW`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07iV[a.u\x83\x83a5jV[PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a-\x10W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x82\x90\x03a.\xE3W`@Qb\x1A25`\xE6\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x82\x11\x15a/\x0FW`@Qc\x02\xD4\xE4\xEF`\xE3\x1B\x81R`\x04\x81\x01\x83\x90R`\xFF`$\x82\x01R`D\x01a\x07iV[`@\x80Q\x80\x82\x01\x90\x91R`\x10\x81Ro8:\xB164\xB1\xA22\xB1\xB9<\xB8:4\xB7\xB7`\x81\x1B` \x82\x01Ra/B\x90\x825\x84a5\xBFV[a/z`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m:\xB9\xB2\xB9\"2\xB1\xB9<\xB8:4\xB7\xB7`\x91\x1B\x81RP\x82` \x015\x85\x85\x90Pa5\xBFV[`@\x80Q\x80\x82\x01\x82R`\x06\x81Re5\xB6\xB9\xA3\xB2\xB7`\xD1\x1B` \x82\x01Ra/\xA3\x91\x83\x015\x84a5\xBFV[`@\x80Q\x80\x82\x01\x90\x91R`\x03\x81Rbmpc`\xE8\x1B` \x82\x01Ra/\xCC\x90``\x83\x015\x84a5\xBFV[_a/\xD5a&\xF6V[\x80T\x90\x91P\x85\x11a0\x06W\x80T`@Qc\xEF\xD5_g`\xE0\x1B\x81Ra\x07i\x91\x87\x91`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[\x84\x81U_[\x83\x81\x10\x15a3\xB4W6\x85\x85\x83\x81\x81\x10a0&Wa0&aB\xB9V[\x90P` \x02\x81\x01\x90a08\x91\x90aH-V[`@\x80Q`\x80\x81\x01\x90\x91R\x90\x91P_\x90\x80a0V` \x85\x01\x85a9\xC7V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x01` \x81\x01\x90a0w\x91\x90a9\xC7V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a0\x92`@\x85\x01\x85aC0V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x90\x82RP` \x01a0\xD8``\x85\x01\x85aC0V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x91RP\x80Q\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16a18W`@QcB3@%`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16a1cW`@Qc-\xEC\xCFM`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x88\x81R`\x02\x85\x01` \x90\x81R`@\x80\x83 \x84Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15a1\xB7W\x80Q`@Qc\r\x18\xC4\xFF`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01a\x07iV[_\x88\x81R`\x03\x85\x01` \x90\x81R`@\x80\x83 \x84\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15a2\x10W` \x81\x01Q`@Qc\xF5\x1A\xF6\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01a\x07iV[_\x88\x81R`\x01\x85\x81\x01` \x90\x81R`@\x80\x84 \x80T\x80\x85\x01\x82U\x90\x85R\x93\x82\x90 \x85Q`\x04\x90\x95\x02\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x17\x82U\x92\x86\x01Q\x93\x81\x01\x80T\x90\x93\x16\x93\x90\x94\x16\x92\x90\x92\x17\x90U\x82\x01Q\x82\x91\x90`\x02\x82\x01\x90a2\x80\x90\x82aH\x85V[P``\x82\x01Q`\x03\x82\x01\x90a2\x95\x90\x82aH\x85V[PPP_\x88\x81R`\x02\x80\x86\x01` \x90\x81R`@\x80\x84 \x85Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x86R\x90\x83R\x81\x85 \x80T`\xFF\x19\x90\x81\x16`\x01\x90\x81\x17\x90\x92U\x8E\x87R`\x03\x8B\x01\x85R\x83\x87 \x88\x86\x01\x80Q\x85\x16\x89R\x90\x86R\x84\x88 \x80T\x90\x92\x16\x83\x17\x90\x91U\x8E\x87R`\x04\x8B\x01\x85R\x83\x87 \x88Q\x84\x16\x88R\x90\x94R\x94\x82\x90 \x86Q\x81T\x90\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x82U\x93Q\x95\x81\x01\x80T\x96\x90\x92\x16\x95\x90\x93\x16\x94\x90\x94\x17\x90\x93U\x91\x83\x01Q\x83\x92\x91\x82\x01\x90a3N\x90\x82aH\x85V[P``\x82\x01Q`\x03\x82\x01\x90a3c\x90\x82aH\x85V[PPP_\x88\x81R`\x05\x85\x01` \x90\x81R`@\x82 \x92\x81\x01Q\x83T`\x01\x80\x82\x01\x86U\x94\x84R\x91\x90\x92 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x91\x90\x91\x01\x90Pa0\x0BV[P_\x94\x85R`\x06\x81\x01` \x90\x81R`@\x80\x87 \x845\x90U`\x07\x83\x01\x82R\x80\x87 \x82\x85\x015\x90U`\x08\x83\x01\x82R\x80\x87 \x81\x85\x015\x90U`\t\x90\x92\x01\x90R\x90\x93 ``\x90\x93\x015\x90\x92UPPV[_`\x03a4\x0Ba&\xF6V[_\x84\x81R`\x0E\x91\x90\x91\x01` R`@\x90 T`\xFF\x16`\x03\x81\x11\x15a41Wa41a?SV[\x14\x92\x91PPV[_a4Aa&\xF6V[_\x92\x83R`\x0F\x81\x01` \x90\x81R`@\x80\x85 \x80T`\xFF\x19\x16\x90U`\x10\x90\x92\x01\x90R\x82 \x91\x90\x91UPV[_\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a4\xA9Wr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a4\xD5Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a4\xF3Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a5\x0BWc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a5\x1FWa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a51W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x0CuW`\x01\x01\x92\x91PPV[_\x80_\x80a5P\x86\x86a6\x08V[\x92P\x92P\x92Pa5`\x82\x82a6QV[P\x90\x94\x93PPPPV[a5s\x82a7\tV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a5\xB7Wa.u\x82\x82a7lV[a\x15\x1Ea7\xD5V[\x81_\x03a5\xE1W\x82`@Qc\x1B_\xDB\x07`\xE1\x1B\x81R`\x04\x01a\x07i\x91\x90a9\x91V[\x80\x82\x11\x15a.uW\x82\x82\x82`@Qc\xCA\xA8\x14\xA3`\xE0\x1B\x81R`\x04\x01a\x07i\x93\x92\x91\x90aI@V[_\x80_\x83Q`A\x03a6?W` \x84\x01Q`@\x85\x01Q``\x86\x01Q_\x1Aa61\x88\x82\x85\x85a7\xF4V[\x95P\x95P\x95PPPPa6JV[PP\x81Q_\x91P`\x02\x90[\x92P\x92P\x92V[_\x82`\x03\x81\x11\x15a6dWa6da?SV[\x03a6mWPPV[`\x01\x82`\x03\x81\x11\x15a6\x81Wa6\x81a?SV[\x03a6\x9FW`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a6\xB3Wa6\xB3a?SV[\x03a6\xD4W`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07iV[`\x03\x82`\x03\x81\x11\x15a6\xE8Wa6\xE8a?SV[\x03a\x15\x1EW`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07iV[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a7>W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x07iV[_\x80Q` aIv\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``_\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa7\x88\x91\x90aIdV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a7\xC0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a7\xC5V[``\x91P[P\x91P\x91Pa+\x17\x85\x83\x83a8\xBCV[4\x15a-\x10W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a8-WP_\x91P`\x03\x90P\x82a8\xB2V[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a8~W=_\x80>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a8\xA9WP_\x92P`\x01\x91P\x82\x90Pa8\xB2V[\x92P_\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[``\x82a8\xD1Wa8\xCC\x82a9\x1BV[a9\x14V[\x81Q\x15\x80\x15a8\xE8WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a9\x11W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x07iV[P\x80[\x93\x92PPPV[\x80Q\x15a9+W\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x83\x81\x10\x15a9^W\x81\x81\x01Q\x83\x82\x01R` \x01a9FV[PP_\x91\x01RV[_\x81Q\x80\x84Ra9}\x81` \x86\x01` \x86\x01a9DV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R_a9\x14` \x83\x01\x84a9fV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a'\xE5W_\x80\xFD[\x805a9\xC2\x81a9\xA3V[\x91\x90PV[_` \x82\x84\x03\x12\x15a9\xD7W_\x80\xFD[\x815a9\x14\x81a9\xA3V[_\x80\x83`\x1F\x84\x01\x12a9\xF2W_\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a:\x08W_\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a:\"W_\x80\xFD[\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12a:9W_\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a:OW_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a:\"W_\x80\xFD[_\x80_\x80_\x80_\x87\x89\x03`\xE0\x81\x12\x15a:}W_\x80\xFD[\x885`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a:\x93W_\x80\xFD[a:\x9F\x8C\x83\x8D\x01a9\xE2V[\x90\x9AP\x98P\x88\x91P`\x80`\x1F\x19\x84\x01\x12\x15a:\xB8W_\x80\xFD[` \x8B\x01\x97P`\xA0\x8B\x015\x92P\x80\x83\x11\x15a:\xD1W_\x80\xFD[a:\xDD\x8C\x84\x8D\x01a:)V[\x90\x97P\x95P`\xC0\x8B\x015\x92P\x86\x91P\x80\x83\x11\x15a:\xF8W_\x80\xFD[PPa;\x06\x8A\x82\x8B\x01a9\xE2V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[_` \x82\x84\x03\x12\x15a;)W_\x80\xFD[P5\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a;AW_\x80\xFD[\x825\x91P` \x83\x015a;S\x81a9\xA3V[\x80\x91PP\x92P\x92\x90PV[_`\x01\x80`\xA0\x1B\x03\x80\x83Q\x16\x84R\x80` \x84\x01Q\x16` \x85\x01RP`@\x82\x01Q`\x80`@\x85\x01Ra;\x92`\x80\x85\x01\x82a9fV[\x90P``\x83\x01Q\x84\x82\x03``\x86\x01Ra+\x17\x82\x82a9fV[` \x81R_a9\x14` \x83\x01\x84a;^V[_\x80_\x80_``\x86\x88\x03\x12\x15a;\xD1W_\x80\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a;\xEEW_\x80\xFD[a;\xFA\x89\x83\x8A\x01a9\xE2V[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15a<\x12W_\x80\xFD[Pa<\x1F\x88\x82\x89\x01a9\xE2V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x80`@\x83\x85\x03\x12\x15a<UW_\x80\xFD[\x825a<`\x81a9\xA3V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a<{W_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a<\x8EW_\x80\xFD[\x815\x81\x81\x11\x15a<\xA0Wa<\xA0a<0V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a<\xC8Wa<\xC8a<0V[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a<\xE0W_\x80\xFD[\x82` \x86\x01` \x83\x017_` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a=AW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a=\x1CV[P\x90\x96\x95PPPPPPV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a9\xC2W_\x80\xFD[_\x80_\x80_\x80`\x80\x87\x89\x03\x12\x15a=xW_\x80\xFD[\x865\x95P` \x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a=\x95W_\x80\xFD[a=\xA1\x8A\x83\x8B\x01a:)V[\x90\x97P\x95P`@\x89\x015\x91P\x80\x82\x11\x15a=\xB9W_\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12a=\xCCW_\x80\xFD[\x815\x81\x81\x11\x15a=\xDAW_\x80\xFD[\x8A` ``\x83\x02\x85\x01\x01\x11\x15a=\xEEW_\x80\xFD[` \x83\x01\x95P\x80\x94PPPPa>\x06``\x88\x01a=MV[\x90P\x92\x95P\x92\x95P\x92\x95V[_\x80`@\x83\x85\x03\x12\x15a>#W_\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[_` \x80\x83\x01` \x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P` \x87\x01_[\x82\x81\x10\x15a>\x87W`?\x19\x88\x86\x03\x01\x84Ra>u\x85\x83Qa;^V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a>YV[P\x92\x97\x96PPPPPPPV[_\x85Qa>\xA5\x81\x84` \x8A\x01a9DV[a\x10;`\xF1\x1B\x90\x83\x01\x90\x81R\x85Qa>\xC4\x81`\x02\x84\x01` \x8A\x01a9DV[\x80\x82\x01\x91PP`\x17`\xF9\x1B\x80`\x02\x83\x01R\x85Qa>\xE8\x81`\x03\x85\x01` \x8A\x01a9DV[`\x03\x92\x01\x91\x82\x01R\x83Qa?\x03\x81`\x04\x84\x01` \x88\x01a9DV[\x01`\x04\x01\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a?!W_\x80\xFD[\x81Qa9\x14\x81a9\xA3V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0CuWa\x0Cua?,V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0CuWa\x0Cua?,V[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a?\x8FW_\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a?\xADW_\x80\xFD[\x806\x03\x82\x13\x15a:\"W_\x80\xFD[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x805`\x03\x81\x90\x0B\x81\x14a9\xC2W_\x80\xFD[_\x83\x83\x85R` \x80\x86\x01\x95P\x80\x85`\x05\x1B\x83\x01\x01\x84_[\x87\x81\x10\x15a@\xACW\x84\x83\x03`\x1F\x19\x01\x89R\x8156\x88\x90\x03`^\x19\x01\x81\x12a@0W_\x80\xFD[\x87\x01``a@>\x82\x80a?zV[\x82\x87Ra@N\x83\x88\x01\x82\x84a?\xBBV[\x92PPPa@^\x86\x83\x01\x83a?zV[\x86\x83\x03\x88\x88\x01Ra@p\x83\x82\x84a?\xBBV[\x92PPP`@a@\x82\x81\x84\x01\x84a?zV[\x93P\x86\x83\x03\x82\x88\x01Ra@\x96\x83\x85\x83a?\xBBV[\x9C\x88\x01\x9C\x96PPP\x92\x85\x01\x92PP`\x01\x01a@\x0BV[P\x90\x97\x96PPPPPPPV[`\xE0\x80\x82R\x81\x01\x87\x90R_a\x01\0\x80\x83\x01`\x05\x8A\x90\x1B\x84\x01\x82\x01\x8B\x84[\x8C\x81\x10\x15aB\x19W\x86\x83\x03`\xFF\x19\x01\x84R\x8156\x8F\x90\x03`\xFE\x19\x01\x81\x12a@\xFBW_\x80\xFD[\x8E\x01aA\x17\x84aA\n\x83a9\xB7V[`\x01`\x01`\xA0\x1B\x03\x16\x90RV[` aA$\x81\x83\x01a9\xB7V[`\x01`\x01`\xA0\x1B\x03\x16\x81\x86\x01R`@aA?\x83\x82\x01\x84a?zV[\x89\x83\x89\x01RaAQ\x8A\x89\x01\x82\x84a?\xBBV[\x92PPP``aAc\x81\x85\x01\x85a?zV[\x88\x84\x03\x83\x8A\x01RaAu\x84\x82\x84a?\xBBV[\x93PPPP`\x80aA\x87\x81\x85\x01a?\xE3V[aA\x95\x82\x89\x01\x82`\x03\x0B\x90RV[PP`\xA0aA\xA5\x81\x85\x01\x85a?zV[\x88\x84\x03\x83\x8A\x01RaA\xB7\x84\x82\x84a?\xBBV[\x93PPPP`\xC0aA\xCA\x81\x85\x01\x85a?zV[\x88\x84\x03\x83\x8A\x01RaA\xDC\x84\x82\x84a?\xBBV[\x93PPPPaA\xEE`\xE0\x84\x01\x84a?zV[\x93P\x86\x82\x03`\xE0\x88\x01RaB\x03\x82\x85\x83a?\xBBV[\x97\x83\x01\x97\x96PPP\x92\x90\x92\x01\x91P`\x01\x01a@\xD6V[PPaBI` \x86\x01\x8B\x805\x82R` \x81\x015` \x83\x01R`@\x81\x015`@\x83\x01R``\x81\x015``\x83\x01RPPV[\x84\x81\x03`\xA0\x86\x01RaB\\\x81\x89\x8Ba?\xBBV[\x92PPP\x82\x81\x03`\xC0\x84\x01RaBs\x81\x85\x87a?\xF4V[\x9A\x99PPPPPPPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80aB\x95W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aB\xB3WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`~\x19\x836\x03\x01\x81\x12aB\xE1W_\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aC\0W_\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aC\x19W_\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a:\"W_\x80\xFD[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aCEW_\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aC^W_\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a:\"W_\x80\xFD[\x84\x81R\x83` \x82\x01R```@\x82\x01R_a,(``\x83\x01\x84\x86a?\xBBV[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15aC\xC0W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aC\xA4V[P\x94\x95\x94PPPPPV[`@\x81R_aC\xDD`@\x83\x01\x85aC\x91V[\x82\x81\x03` \x84\x01Ra+\x17\x81\x85aC\x91V[_`\x01\x82\x01aD\0WaD\0a?,V[P`\x01\x01\x90V[\x805`\x04\x81\x10a9\xC2W_\x80\xFD[`\x04\x81\x10aD1WcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90RV[_\x83\x83\x85R` \x80\x86\x01\x95P\x80\x85`\x05\x1B\x83\x01\x01\x84_[\x87\x81\x10\x15a@\xACW\x84\x83\x03`\x1F\x19\x01\x89R\x8156\x88\x90\x03`>\x19\x01\x81\x12aDqW_\x80\xFD[\x87\x01`@aD\x87\x85aD\x82\x84aD\x07V[aD\x15V[aD\x93\x86\x83\x01\x83a?zV[\x92P\x81\x87\x87\x01RaD\xA7\x82\x87\x01\x84\x83a?\xBBV[\x9B\x87\x01\x9B\x95PPP\x91\x84\x01\x91P`\x01\x01aDLV[_\x825`~\x19\x836\x03\x01\x81\x12aD\xD0W_\x80\xFD[\x90\x91\x01\x92\x91PPV[_\x83\x83\x85R` \x80\x86\x01\x95P\x80\x85`\x05\x1B\x83\x01\x01\x84_[\x87\x81\x10\x15a@\xACW\x84\x83\x03`\x1F\x19\x01\x89RaE\x0B\x82\x88aD\xBCV[`\x80\x815\x85R\x85\x82\x015\x86\x86\x01R`@aE'\x81\x84\x01\x84a?zV[\x83\x83\x89\x01RaE9\x84\x89\x01\x82\x84a?\xBBV[\x93PPPP``aEL\x81\x84\x01\x84a?zV[\x93P\x86\x83\x03\x82\x88\x01RaE`\x83\x85\x83a?\xBBV[\x9C\x88\x01\x9C\x96PPP\x92\x85\x01\x92PP`\x01\x01aD\xF0V[_\x82\x82Q\x80\x85R` \x80\x86\x01\x95P` \x82`\x05\x1B\x84\x01\x01` \x86\x01_[\x84\x81\x10\x15a@\xACW`\x1F\x19\x86\x84\x03\x01\x89RaE\xAF\x83\x83Qa9fV[\x98\x84\x01\x98\x92P\x90\x83\x01\x90`\x01\x01aE\x93V[``\x80\x82R\x81\x81\x01\x86\x90R_\x90`\x80\x80\x84\x01`\x05\x89\x81\x1B\x86\x01\x83\x01\x8B\x86[\x8C\x81\x10\x15aF\x95W\x88\x83\x03`\x7F\x19\x01\x85RaE\xFA\x82\x8FaD\xBCV[\x805\x84R` \x80\x82\x015\x81\x86\x01R`@\x80\x83\x015`\x1E\x19\x846\x03\x01\x81\x12aF\x1FW_\x80\xFD[\x83\x01\x82\x81\x01\x905`\x01`\x01`@\x1B\x03\x81\x11\x15aF9W_\x80\xFD[\x80\x89\x1B6\x03\x82\x13\x15aFIW_\x80\xFD[\x8A\x83\x89\x01RaF[\x8B\x89\x01\x82\x84aD5V[\x92PPPaFk\x8A\x84\x01\x84a?zV[\x93P\x86\x82\x03\x8B\x88\x01RaF\x7F\x82\x85\x83a?\xBBV[\x98\x83\x01\x98\x96PPP\x92\x90\x92\x01\x91P`\x01\x01aE\xDFV[PP\x86\x81\x03` \x88\x01RaF\xAA\x81\x8A\x8CaD\xD9V[\x94PPPPP\x82\x81\x03`@\x84\x01RaF\xC2\x81\x85aEvV[\x98\x97PPPPPPPPV[_` \x82\x84\x03\x12\x15aF\xDEW_\x80\xFD[a9\x14\x82a=MV[_``\x80\x83RaF\xFB``\x84\x01\x88\x8Aa?\xBBV[\x83\x81\x03` \x85\x81\x01\x91\x90\x91R\x86\x82R\x87\x91\x81\x01_[\x88\x81\x10\x15aGbW`\x01`\x01`@\x1B\x03\x80aG*\x86a=MV[\x16\x83R\x80aG9\x85\x87\x01a=MV[\x16\x84\x84\x01R`@\x81aGL\x82\x88\x01a=MV[\x16\x90\x84\x01RP\x92\x84\x01\x92\x90\x84\x01\x90`\x01\x01aG\x10V[P\x80\x94PPPPP`\x01`\x01`@\x1B\x03\x83\x16`@\x83\x01R\x96\x95PPPPPPV[_\x825`>\x19\x836\x03\x01\x81\x12aB\xE1W_\x80\xFD[_` \x82\x84\x03\x12\x15aG\xA7W_\x80\xFD[a9\x14\x82aD\x07V[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[\x83\x81R``\x81\x01aG\xD3` \x83\x01\x85aD\x15V[\x82`@\x83\x01R\x94\x93PPPPV[\x81Q_\x90\x82\x90` \x80\x86\x01\x84[\x83\x81\x10\x15aH\nW\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01aG\xEEV[P\x92\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15aH&W_\x80\xFD[PQ\x91\x90PV[_\x825`\xFE\x19\x836\x03\x01\x81\x12aB\xE1W_\x80\xFD[`\x1F\x82\x11\x15a.uW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15aHfWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a#\xD0W_\x81U`\x01\x01aHrV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aH\x9EWaH\x9Ea<0V[aH\xB2\x81aH\xAC\x84TaB\x81V[\x84aHAV[` \x80`\x1F\x83\x11`\x01\x81\x14aH\xE5W_\x84\x15aH\xCEWP\x85\x83\x01Q[_\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua'\xB4V[_\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15aI\x13W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01aH\xF4V[P\x85\x82\x10\x15aI0W\x87\x85\x01Q_\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[``\x81R_aIR``\x83\x01\x86a9fV[` \x83\x01\x94\x90\x94RP`@\x01R\x91\x90PV[_\x82QaB\xE1\x81\x84` \x87\x01a9DV\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC",
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
    /**Custom error with signature `EmptyEpochActivationAttestation(uint256)` and selector `0xce9440df`.
```solidity
error EmptyEpochActivationAttestation(uint256 epochId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyEpochActivationAttestation {
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
        impl ::core::convert::From<EmptyEpochActivationAttestation>
        for UnderlyingRustTuple<'_> {
            fn from(value: EmptyEpochActivationAttestation) -> Self {
                (value.epochId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for EmptyEpochActivationAttestation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { epochId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyEpochActivationAttestation {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyEpochActivationAttestation(uint256)";
            const SELECTOR: [u8; 4] = [206u8, 148u8, 64u8, 223u8];
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
    /**Custom error with signature `EpochActivationSignerDoesNotMatchTxSender(address,address)` and selector `0xf1735b46`.
```solidity
error EpochActivationSignerDoesNotMatchTxSender(address signer, address txSender);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EpochActivationSignerDoesNotMatchTxSender {
        #[allow(missing_docs)]
        pub signer: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<EpochActivationSignerDoesNotMatchTxSender>
        for UnderlyingRustTuple<'_> {
            fn from(value: EpochActivationSignerDoesNotMatchTxSender) -> Self {
                (value.signer, value.txSender)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for EpochActivationSignerDoesNotMatchTxSender {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signer: tuple.0,
                    txSender: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EpochActivationSignerDoesNotMatchTxSender {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EpochActivationSignerDoesNotMatchTxSender(address,address)";
            const SELECTOR: [u8; 4] = [241u8, 115u8, 91u8, 70u8];
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
    /**Custom error with signature `EpochActivationUnauthorized(address,uint256)` and selector `0xa3f4afeb`.
```solidity
error EpochActivationUnauthorized(address caller, uint256 epochId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EpochActivationUnauthorized {
        #[allow(missing_docs)]
        pub caller: alloy::sol_types::private::Address,
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
                (value.caller, value.epochId)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for EpochActivationUnauthorized {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    caller: tuple.0,
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
                        &self.caller,
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
error KmsContextCreationAlreadyConfirmed(address txSender, uint256 kmsContextId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsContextCreationAlreadyConfirmed {
        #[allow(missing_docs)]
        pub txSender: alloy::sol_types::private::Address,
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
                (value.txSender, value.kmsContextId)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for KmsContextCreationAlreadyConfirmed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    txSender: tuple.0,
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
                        &self.txSender,
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
error KmsContextCreationUnauthorized(address caller, uint256 kmsContextId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsContextCreationUnauthorized {
        #[allow(missing_docs)]
        pub caller: alloy::sol_types::private::Address,
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
                (value.caller, value.kmsContextId)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for KmsContextCreationUnauthorized {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    caller: tuple.0,
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
                        &self.caller,
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
    /**Event with signature `ActivateEpoch(uint256,uint256,(uint256,uint256,(uint8,bytes)[],bytes)[],(uint256,uint256,bytes,bytes)[],string[])` and selector `0x1a547b42e72cd3dda04e6adccd2200276cfef01fe2138d07f3a7440f416d38bc`.
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
            const SIGNATURE: &'static str = "ActivateEpoch(uint256,uint256,(uint256,uint256,(uint8,bytes)[],bytes)[],(uint256,uint256,bytes,bytes)[],string[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                26u8, 84u8, 123u8, 66u8, 231u8, 44u8, 211u8, 221u8, 160u8, 78u8, 106u8,
                220u8, 205u8, 34u8, 0u8, 39u8, 108u8, 254u8, 240u8, 31u8, 226u8, 19u8,
                141u8, 7u8, 243u8, 167u8, 68u8, 15u8, 65u8, 109u8, 56u8, 188u8,
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
    /**Event with signature `EpochActivationConfirmation(uint256,address,bytes32)` and selector `0x7eda6f85e23b7b91c019b0570d02b663606ef9d74594f7e01fcfbdb0f4e954d5`.
```solidity
event EpochActivationConfirmation(uint256 indexed epochId, address indexed signer, bytes32 dataHash);
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
        pub dataHash: alloy::sol_types::private::FixedBytes<32>,
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
            type DataTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "EpochActivationConfirmation(uint256,address,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                126u8, 218u8, 111u8, 133u8, 226u8, 59u8, 123u8, 145u8, 192u8, 25u8,
                176u8, 87u8, 13u8, 2u8, 182u8, 99u8, 96u8, 110u8, 249u8, 215u8, 69u8,
                148u8, 247u8, 224u8, 31u8, 207u8, 189u8, 176u8, 244u8, 233u8, 84u8, 213u8,
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
                    dataHash: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.dataHash),
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
    /**Event with signature `KmsContextCreationConfirmation(uint256,address,bool,bool)` and selector `0xb79c48003695b6ebe555afa36fad071deeee75eb3718ad63de5621d35ba44b4f`.
```solidity
event KmsContextCreationConfirmation(uint256 indexed kmsContextId, address indexed txSender, bool isPreviousTxSender, bool isNewTxSender);
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
        pub txSender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub isPreviousTxSender: bool,
        #[allow(missing_docs)]
        pub isNewTxSender: bool,
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
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bool,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "KmsContextCreationConfirmation(uint256,address,bool,bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                183u8, 156u8, 72u8, 0u8, 54u8, 149u8, 182u8, 235u8, 229u8, 85u8, 175u8,
                163u8, 111u8, 173u8, 7u8, 29u8, 238u8, 238u8, 117u8, 235u8, 55u8, 24u8,
                173u8, 99u8, 222u8, 86u8, 33u8, 211u8, 91u8, 164u8, 75u8, 79u8,
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
                    txSender: topics.2,
                    isPreviousTxSender: data.0,
                    isNewTxSender: data.1,
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
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.isPreviousTxSender,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.isNewTxSender,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.kmsContextId.clone(),
                    self.txSender.clone(),
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
                    &self.txSender,
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
    /**Function with signature `confirmEpochActivation(uint256,(uint256,uint256,(uint8,bytes)[],bytes)[],(uint256,uint256,bytes,bytes)[])` and selector `0x4cb950e1`.
```solidity
function confirmEpochActivation(uint256 epochId, IProtocolConfig.EpochKeyResult[] memory keys, IProtocolConfig.EpochCrsResult[] memory crsList) external;
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
    }
    ///Container type for the return parameters of the [`confirmEpochActivation(uint256,(uint256,uint256,(uint8,bytes)[],bytes)[],(uint256,uint256,bytes,bytes)[])`](confirmEpochActivationCall) function.
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
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
                    (value.epochId, value.keys, value.crsList)
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
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = confirmEpochActivationReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "confirmEpochActivation(uint256,(uint256,uint256,(uint8,bytes)[],bytes)[],(uint256,uint256,bytes,bytes)[])";
            const SELECTOR: [u8; 4] = [76u8, 185u8, 80u8, 225u8];
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
    /**Function with signature `confirmKmsContextCreation(uint256)` and selector `0xd9be2de4`.
```solidity
function confirmKmsContextCreation(uint256 kmsContextId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct confirmKmsContextCreationCall {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`confirmKmsContextCreation(uint256)`](confirmKmsContextCreationCall) function.
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
            impl ::core::convert::From<confirmKmsContextCreationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: confirmKmsContextCreationCall) -> Self {
                    (value.kmsContextId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for confirmKmsContextCreationCall {
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
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = confirmKmsContextCreationReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "confirmKmsContextCreation(uint256)";
            const SELECTOR: [u8; 4] = [217u8, 190u8, 45u8, 228u8];
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
    /**Function with signature `getContextCreationPreviousTxSenderThreshold(uint256)` and selector `0x3e6316ea`.
```solidity
function getContextCreationPreviousTxSenderThreshold(uint256 kmsContextId) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getContextCreationPreviousTxSenderThresholdCall {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getContextCreationPreviousTxSenderThreshold(uint256)`](getContextCreationPreviousTxSenderThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getContextCreationPreviousTxSenderThresholdReturn {
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
            impl ::core::convert::From<getContextCreationPreviousTxSenderThresholdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getContextCreationPreviousTxSenderThresholdCall) -> Self {
                    (value.kmsContextId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getContextCreationPreviousTxSenderThresholdCall {
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
            impl ::core::convert::From<getContextCreationPreviousTxSenderThresholdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(
                    value: getContextCreationPreviousTxSenderThresholdReturn,
                ) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getContextCreationPreviousTxSenderThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for getContextCreationPreviousTxSenderThresholdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getContextCreationPreviousTxSenderThreshold(uint256)";
            const SELECTOR: [u8; 4] = [62u8, 99u8, 22u8, 234u8];
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
                        let r: getContextCreationPreviousTxSenderThresholdReturn = r
                            .into();
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
                        let r: getContextCreationPreviousTxSenderThresholdReturn = r
                            .into();
                        r._0
                    })
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
    /**Function with signature `reinitializeV3()` and selector `0xbac22bb8`.
```solidity
function reinitializeV3() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reinitializeV3Call;
    ///Container type for the return parameters of the [`reinitializeV3()`](reinitializeV3Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reinitializeV3Return {}
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
            impl ::core::convert::From<reinitializeV3Call> for UnderlyingRustTuple<'_> {
                fn from(value: reinitializeV3Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for reinitializeV3Call {
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
            impl ::core::convert::From<reinitializeV3Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: reinitializeV3Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for reinitializeV3Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl reinitializeV3Return {
            fn _tokenize(
                &self,
            ) -> <reinitializeV3Call as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for reinitializeV3Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = reinitializeV3Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "reinitializeV3()";
            const SELECTOR: [u8; 4] = [186u8, 194u8, 43u8, 184u8];
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
                reinitializeV3Return::_tokenize(ret)
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
        defineNewEpochForCurrentKmsContext(defineNewEpochForCurrentKmsContextCall),
        #[allow(missing_docs)]
        defineNewKmsContextAndEpoch(defineNewKmsContextAndEpochCall),
        #[allow(missing_docs)]
        destroyKmsContext(destroyKmsContextCall),
        #[allow(missing_docs)]
        destroyKmsEpoch(destroyKmsEpochCall),
        #[allow(missing_docs)]
        getContextCreationPreviousTxSenderThreshold(
            getContextCreationPreviousTxSenderThresholdCall,
        ),
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
        reinitializeV3(reinitializeV3Call),
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
            [28u8, 227u8, 249u8, 188u8],
            [32u8, 61u8, 1u8, 20u8],
            [34u8, 28u8, 221u8, 78u8],
            [38u8, 207u8, 93u8, 239u8],
            [40u8, 30u8, 139u8, 254u8],
            [42u8, 56u8, 137u8, 152u8],
            [49u8, 255u8, 65u8, 200u8],
            [62u8, 99u8, 22u8, 234u8],
            [65u8, 173u8, 6u8, 156u8],
            [70u8, 197u8, 187u8, 189u8],
            [71u8, 232u8, 34u8, 149u8],
            [76u8, 185u8, 80u8, 225u8],
            [79u8, 30u8, 242u8, 134u8],
            [82u8, 209u8, 144u8, 45u8],
            [91u8, 255u8, 118u8, 217u8],
            [101u8, 179u8, 148u8, 175u8],
            [126u8, 170u8, 200u8, 242u8],
            [148u8, 71u8, 207u8, 212u8],
            [151u8, 108u8, 152u8, 181u8],
            [151u8, 111u8, 62u8, 185u8],
            [157u8, 27u8, 27u8, 225u8],
            [173u8, 60u8, 177u8, 204u8],
            [180u8, 114u8, 43u8, 196u8],
            [186u8, 194u8, 43u8, 184u8],
            [191u8, 155u8, 22u8, 200u8],
            [192u8, 174u8, 100u8, 247u8],
            [194u8, 88u8, 10u8, 45u8],
            [194u8, 180u8, 41u8, 134u8],
            [195u8, 170u8, 170u8, 90u8],
            [201u8, 153u8, 168u8, 180u8],
            [204u8, 191u8, 129u8, 153u8],
            [204u8, 234u8, 192u8, 25u8],
            [217u8, 190u8, 45u8, 228u8],
            [238u8, 125u8, 82u8, 209u8],
            [249u8, 198u8, 112u8, 195u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(getVersion),
            ::core::stringify!(defineNewEpochForCurrentKmsContext),
            ::core::stringify!(isKmsSigner),
            ::core::stringify!(initializeFromEmptyProxy),
            ::core::stringify!(getMpcThreshold),
            ::core::stringify!(getUserDecryptionThresholdForContext),
            ::core::stringify!(getPublicDecryptionThreshold),
            ::core::stringify!(getKmsNodeForContext),
            ::core::stringify!(getContextCreationPreviousTxSenderThreshold),
            ::core::stringify!(getKmsGenThresholdForContext),
            ::core::stringify!(isKmsTxSenderForContext),
            ::core::stringify!(getMpcThresholdForContext),
            ::core::stringify!(confirmEpochActivation),
            ::core::stringify!(upgradeToAndCall),
            ::core::stringify!(proxiableUUID),
            ::core::stringify!(getKmsSignersForContext),
            ::core::stringify!(getCurrentKmsContextAndEpoch),
            ::core::stringify!(getKmsSigners),
            ::core::stringify!(isKmsSignerForContext),
            ::core::stringify!(defineNewKmsContextAndEpoch),
            ::core::stringify!(getCurrentKmsContextId),
            ::core::stringify!(getCurrentKmsContextIdCounter),
            ::core::stringify!(UPGRADE_INTERFACE_VERSION),
            ::core::stringify!(getKmsGenThreshold),
            ::core::stringify!(reinitializeV3),
            ::core::stringify!(isValidKmsContext),
            ::core::stringify!(destroyKmsContext),
            ::core::stringify!(destroyKmsEpoch),
            ::core::stringify!(getUserDecryptionThreshold),
            ::core::stringify!(getPublicDecryptionThresholdForContext),
            ::core::stringify!(getKmsContextAnchor),
            ::core::stringify!(proposeCoprocessorUpgrade),
            ::core::stringify!(isValidEpochForContext),
            ::core::stringify!(confirmKmsContextCreation),
            ::core::stringify!(isLiveKmsContext),
            ::core::stringify!(getKmsNodesForContext),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <getVersionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <defineNewEpochForCurrentKmsContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isKmsSignerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getMpcThresholdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getUserDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getPublicDecryptionThresholdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getKmsNodeForContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getContextCreationPreviousTxSenderThresholdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getKmsGenThresholdForContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isKmsTxSenderForContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getMpcThresholdForContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <confirmEpochActivationCall as alloy_sol_types::SolCall>::SIGNATURE,
            <upgradeToAndCallCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proxiableUUIDCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getKmsSignersForContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getCurrentKmsContextAndEpochCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getKmsSignersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isKmsSignerForContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <defineNewKmsContextAndEpochCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getCurrentKmsContextIdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getCurrentKmsContextIdCounterCall as alloy_sol_types::SolCall>::SIGNATURE,
            <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getKmsGenThresholdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <reinitializeV3Call as alloy_sol_types::SolCall>::SIGNATURE,
            <isValidKmsContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <destroyKmsContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <destroyKmsEpochCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getUserDecryptionThresholdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getPublicDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getKmsContextAnchorCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proposeCoprocessorUpgradeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isValidEpochForContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <confirmKmsContextCreationCall as alloy_sol_types::SolCall>::SIGNATURE,
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
        const COUNT: usize = 36usize;
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
                Self::getContextCreationPreviousTxSenderThreshold(_) => {
                    <getContextCreationPreviousTxSenderThresholdCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::reinitializeV3(_) => {
                    <reinitializeV3Call as alloy_sol_types::SolCall>::SELECTOR
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
                    fn getContextCreationPreviousTxSenderThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getContextCreationPreviousTxSenderThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigCalls::getContextCreationPreviousTxSenderThreshold,
                            )
                    }
                    getContextCreationPreviousTxSenderThreshold
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
                    fn reinitializeV3(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <reinitializeV3Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::reinitializeV3)
                    }
                    reinitializeV3
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
                    fn getContextCreationPreviousTxSenderThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getContextCreationPreviousTxSenderThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigCalls::getContextCreationPreviousTxSenderThreshold,
                            )
                    }
                    getContextCreationPreviousTxSenderThreshold
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
                    fn reinitializeV3(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <reinitializeV3Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::reinitializeV3)
                    }
                    reinitializeV3
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
                Self::getContextCreationPreviousTxSenderThreshold(inner) => {
                    <getContextCreationPreviousTxSenderThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::reinitializeV3(inner) => {
                    <reinitializeV3Call as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::getContextCreationPreviousTxSenderThreshold(inner) => {
                    <getContextCreationPreviousTxSenderThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::reinitializeV3(inner) => {
                    <reinitializeV3Call as alloy_sol_types::SolCall>::abi_encode_raw(
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
        EmptyEpochActivationAttestation(EmptyEpochActivationAttestation),
        #[allow(missing_docs)]
        EmptyKmsNodes(EmptyKmsNodes),
        #[allow(missing_docs)]
        EmptySoftwareVersion(EmptySoftwareVersion),
        #[allow(missing_docs)]
        EpochActivationAlreadyConfirmed(EpochActivationAlreadyConfirmed),
        #[allow(missing_docs)]
        EpochActivationSignerDoesNotMatchTxSender(
            EpochActivationSignerDoesNotMatchTxSender,
        ),
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
        KmsContextNotPending(KmsContextNotPending),
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
            [6u8, 140u8, 141u8, 64u8],
            [7u8, 128u8, 32u8, 190u8],
            [9u8, 146u8, 247u8, 173u8],
            [22u8, 167u8, 39u8, 120u8],
            [23u8, 211u8, 233u8, 72u8],
            [24u8, 122u8, 234u8, 168u8],
            [33u8, 191u8, 218u8, 16u8],
            [45u8, 236u8, 207u8, 77u8],
            [53u8, 134u8, 239u8, 161u8],
            [54u8, 191u8, 182u8, 14u8],
            [73u8, 65u8, 118u8, 54u8],
            [76u8, 156u8, 140u8, 227u8],
            [98u8, 88u8, 92u8, 200u8],
            [108u8, 103u8, 228u8, 112u8],
            [111u8, 79u8, 115u8, 31u8],
            [119u8, 221u8, 190u8, 129u8],
            [121u8, 149u8, 187u8, 207u8],
            [132u8, 102u8, 128u8, 74u8],
            [153u8, 150u8, 179u8, 21u8],
            [163u8, 244u8, 175u8, 235u8],
            [170u8, 29u8, 73u8, 164u8],
            [179u8, 152u8, 151u8, 159u8],
            [181u8, 72u8, 145u8, 71u8],
            [184u8, 29u8, 248u8, 232u8],
            [190u8, 80u8, 80u8, 68u8],
            [200u8, 72u8, 133u8, 212u8],
            [202u8, 168u8, 20u8, 163u8],
            [206u8, 148u8, 64u8, 223u8],
            [209u8, 140u8, 79u8, 240u8],
            [214u8, 189u8, 162u8, 117u8],
            [215u8, 139u8, 206u8, 12u8],
            [215u8, 230u8, 188u8, 248u8],
            [224u8, 124u8, 141u8, 186u8],
            [239u8, 213u8, 95u8, 103u8],
            [240u8, 231u8, 129u8, 193u8],
            [241u8, 115u8, 91u8, 70u8],
            [242u8, 25u8, 220u8, 14u8],
            [245u8, 26u8, 246u8, 187u8],
            [246u8, 69u8, 238u8, 223u8],
            [249u8, 46u8, 232u8, 169u8],
            [252u8, 230u8, 152u8, 247u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(EmptyKmsNodes),
            ::core::stringify!(KmsLifecycleOperationInFlight),
            ::core::stringify!(InvalidProposalId),
            ::core::stringify!(KmsSignerSetExceedsProofFormatLimit),
            ::core::stringify!(ZeroGwStartBlock),
            ::core::stringify!(LatestActiveKmsContextCannotBeDestroyed),
            ::core::stringify!(NotHostOwner),
            ::core::stringify!(KmsNodeNullSigner),
            ::core::stringify!(KmsContextNotPending),
            ::core::stringify!(InvalidNullThreshold),
            ::core::stringify!(EpochActivationAlreadyConfirmed),
            ::core::stringify!(ERC1967InvalidImplementation),
            ::core::stringify!(KmsContextCreationAlreadyConfirmed),
            ::core::stringify!(DuplicateChainId),
            ::core::stringify!(NotInitializingFromEmptyProxy),
            ::core::stringify!(InvalidKmsContext),
            ::core::stringify!(InvalidKmsEpoch),
            ::core::stringify!(KmsNodeNullTxSender),
            ::core::stringify!(AddressEmptyCode),
            ::core::stringify!(EpochActivationUnauthorized),
            ::core::stringify!(UUPSUnsupportedProxiableUUID),
            ::core::stringify!(ERC1967NonPayable),
            ::core::stringify!(EmptySoftwareVersion),
            ::core::stringify!(KmsContextCreationUnauthorized),
            ::core::stringify!(EmptyChainUpgradeWindows),
            ::core::stringify!(ZeroChainId),
            ::core::stringify!(InvalidHighThreshold),
            ::core::stringify!(EmptyEpochActivationAttestation),
            ::core::stringify!(KmsTxSenderAlreadyRegistered),
            ::core::stringify!(FailedCall),
            ::core::stringify!(ECDSAInvalidSignatureS),
            ::core::stringify!(NotInitializing),
            ::core::stringify!(UUPSUnauthorizedCallContext),
            ::core::stringify!(NonIncreasingKmsContextId),
            ::core::stringify!(LatestActiveKmsEpochCannotBeDestroyed),
            ::core::stringify!(EpochActivationSignerDoesNotMatchTxSender),
            ::core::stringify!(InvalidBlockWindow),
            ::core::stringify!(KmsSignerAlreadyRegistered),
            ::core::stringify!(ECDSAInvalidSignature),
            ::core::stringify!(InvalidInitialization),
            ::core::stringify!(ECDSAInvalidSignatureLength),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <EmptyKmsNodes as alloy_sol_types::SolError>::SIGNATURE,
            <KmsLifecycleOperationInFlight as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidProposalId as alloy_sol_types::SolError>::SIGNATURE,
            <KmsSignerSetExceedsProofFormatLimit as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroGwStartBlock as alloy_sol_types::SolError>::SIGNATURE,
            <LatestActiveKmsContextCannotBeDestroyed as alloy_sol_types::SolError>::SIGNATURE,
            <NotHostOwner as alloy_sol_types::SolError>::SIGNATURE,
            <KmsNodeNullSigner as alloy_sol_types::SolError>::SIGNATURE,
            <KmsContextNotPending as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidNullThreshold as alloy_sol_types::SolError>::SIGNATURE,
            <EpochActivationAlreadyConfirmed as alloy_sol_types::SolError>::SIGNATURE,
            <ERC1967InvalidImplementation as alloy_sol_types::SolError>::SIGNATURE,
            <KmsContextCreationAlreadyConfirmed as alloy_sol_types::SolError>::SIGNATURE,
            <DuplicateChainId as alloy_sol_types::SolError>::SIGNATURE,
            <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidKmsContext as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidKmsEpoch as alloy_sol_types::SolError>::SIGNATURE,
            <KmsNodeNullTxSender as alloy_sol_types::SolError>::SIGNATURE,
            <AddressEmptyCode as alloy_sol_types::SolError>::SIGNATURE,
            <EpochActivationUnauthorized as alloy_sol_types::SolError>::SIGNATURE,
            <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::SIGNATURE,
            <ERC1967NonPayable as alloy_sol_types::SolError>::SIGNATURE,
            <EmptySoftwareVersion as alloy_sol_types::SolError>::SIGNATURE,
            <KmsContextCreationUnauthorized as alloy_sol_types::SolError>::SIGNATURE,
            <EmptyChainUpgradeWindows as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroChainId as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidHighThreshold as alloy_sol_types::SolError>::SIGNATURE,
            <EmptyEpochActivationAttestation as alloy_sol_types::SolError>::SIGNATURE,
            <KmsTxSenderAlreadyRegistered as alloy_sol_types::SolError>::SIGNATURE,
            <FailedCall as alloy_sol_types::SolError>::SIGNATURE,
            <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::SIGNATURE,
            <NotInitializing as alloy_sol_types::SolError>::SIGNATURE,
            <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::SIGNATURE,
            <NonIncreasingKmsContextId as alloy_sol_types::SolError>::SIGNATURE,
            <LatestActiveKmsEpochCannotBeDestroyed as alloy_sol_types::SolError>::SIGNATURE,
            <EpochActivationSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::SIGNATURE,
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
        const COUNT: usize = 41usize;
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
                Self::EmptyEpochActivationAttestation(_) => {
                    <EmptyEpochActivationAttestation as alloy_sol_types::SolError>::SELECTOR
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
                Self::EpochActivationSignerDoesNotMatchTxSender(_) => {
                    <EpochActivationSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::SELECTOR
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
                Self::KmsContextNotPending(_) => {
                    <KmsContextNotPending as alloy_sol_types::SolError>::SELECTOR
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
                    fn EmptyEpochActivationAttestation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <EmptyEpochActivationAttestation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::EmptyEpochActivationAttestation)
                    }
                    EmptyEpochActivationAttestation
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
                    fn EpochActivationSignerDoesNotMatchTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <EpochActivationSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigErrors::EpochActivationSignerDoesNotMatchTxSender,
                            )
                    }
                    EpochActivationSignerDoesNotMatchTxSender
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
                    fn EmptyEpochActivationAttestation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <EmptyEpochActivationAttestation as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::EmptyEpochActivationAttestation)
                    }
                    EmptyEpochActivationAttestation
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
                    fn EpochActivationSignerDoesNotMatchTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <EpochActivationSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigErrors::EpochActivationSignerDoesNotMatchTxSender,
                            )
                    }
                    EpochActivationSignerDoesNotMatchTxSender
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
                Self::EmptyEpochActivationAttestation(inner) => {
                    <EmptyEpochActivationAttestation as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::EpochActivationSignerDoesNotMatchTxSender(inner) => {
                    <EpochActivationSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::KmsContextNotPending(inner) => {
                    <KmsContextNotPending as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::EmptyEpochActivationAttestation(inner) => {
                    <EmptyEpochActivationAttestation as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::EpochActivationSignerDoesNotMatchTxSender(inner) => {
                    <EpochActivationSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::KmsContextNotPending(inner) => {
                    <KmsContextNotPending as alloy_sol_types::SolError>::abi_encode_raw(
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
        KmsEpochDestroyed(KmsEpochDestroyed),
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
                26u8, 84u8, 123u8, 66u8, 231u8, 44u8, 211u8, 221u8, 160u8, 78u8, 106u8,
                220u8, 205u8, 34u8, 0u8, 39u8, 108u8, 254u8, 240u8, 31u8, 226u8, 19u8,
                141u8, 7u8, 243u8, 167u8, 68u8, 15u8, 65u8, 109u8, 56u8, 188u8,
            ],
            [
                32u8, 77u8, 107u8, 128u8, 18u8, 17u8, 84u8, 205u8, 135u8, 217u8, 156u8,
                245u8, 76u8, 99u8, 154u8, 61u8, 208u8, 165u8, 59u8, 48u8, 132u8, 39u8,
                112u8, 152u8, 222u8, 151u8, 46u8, 189u8, 211u8, 76u8, 107u8, 233u8,
            ],
            [
                126u8, 218u8, 111u8, 133u8, 226u8, 59u8, 123u8, 145u8, 192u8, 25u8,
                176u8, 87u8, 13u8, 2u8, 182u8, 99u8, 96u8, 110u8, 249u8, 215u8, 69u8,
                148u8, 247u8, 224u8, 31u8, 207u8, 189u8, 176u8, 244u8, 233u8, 84u8, 213u8,
            ],
            [
                183u8, 156u8, 72u8, 0u8, 54u8, 149u8, 182u8, 235u8, 229u8, 85u8, 175u8,
                163u8, 111u8, 173u8, 7u8, 29u8, 238u8, 238u8, 117u8, 235u8, 55u8, 24u8,
                173u8, 99u8, 222u8, 86u8, 33u8, 211u8, 91u8, 164u8, 75u8, 79u8,
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
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(KmsEpochDestroyed),
            ::core::stringify!(NewKmsEpoch),
            ::core::stringify!(ActivateEpoch),
            ::core::stringify!(NewKmsContext),
            ::core::stringify!(EpochActivationConfirmation),
            ::core::stringify!(KmsContextCreationConfirmation),
            ::core::stringify!(Upgraded),
            ::core::stringify!(CoprocessorUpgradeProposed),
            ::core::stringify!(Initialized),
            ::core::stringify!(KmsContextDestroyed),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <KmsEpochDestroyed as alloy_sol_types::SolEvent>::SIGNATURE,
            <NewKmsEpoch as alloy_sol_types::SolEvent>::SIGNATURE,
            <ActivateEpoch as alloy_sol_types::SolEvent>::SIGNATURE,
            <NewKmsContext as alloy_sol_types::SolEvent>::SIGNATURE,
            <EpochActivationConfirmation as alloy_sol_types::SolEvent>::SIGNATURE,
            <KmsContextCreationConfirmation as alloy_sol_types::SolEvent>::SIGNATURE,
            <Upgraded as alloy_sol_types::SolEvent>::SIGNATURE,
            <CoprocessorUpgradeProposed as alloy_sol_types::SolEvent>::SIGNATURE,
            <Initialized as alloy_sol_types::SolEvent>::SIGNATURE,
            <KmsContextDestroyed as alloy_sol_types::SolEvent>::SIGNATURE,
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
        const COUNT: usize = 10usize;
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
                    <KmsEpochDestroyed as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <KmsEpochDestroyed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::KmsEpochDestroyed)
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
                Self::KmsEpochDestroyed(inner) => {
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
                Self::KmsEpochDestroyed(inner) => {
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
        ) -> alloy_contract::SolCallBuilder<&P, confirmEpochActivationCall, N> {
            self.call_builder(
                &confirmEpochActivationCall {
                    epochId,
                    keys,
                    crsList,
                },
            )
        }
        ///Creates a new call builder for the [`confirmKmsContextCreation`] function.
        pub fn confirmKmsContextCreation(
            &self,
            kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, confirmKmsContextCreationCall, N> {
            self.call_builder(
                &confirmKmsContextCreationCall {
                    kmsContextId,
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
        ///Creates a new call builder for the [`getContextCreationPreviousTxSenderThreshold`] function.
        pub fn getContextCreationPreviousTxSenderThreshold(
            &self,
            kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            getContextCreationPreviousTxSenderThresholdCall,
            N,
        > {
            self.call_builder(
                &getContextCreationPreviousTxSenderThresholdCall {
                    kmsContextId,
                },
            )
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
        ///Creates a new call builder for the [`reinitializeV3`] function.
        pub fn reinitializeV3(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, reinitializeV3Call, N> {
            self.call_builder(&reinitializeV3Call)
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
        ///Creates a new event filter for the [`KmsEpochDestroyed`] event.
        pub fn KmsEpochDestroyed_filter(
            &self,
        ) -> alloy_contract::Event<&P, KmsEpochDestroyed, N> {
            self.event_filter::<KmsEpochDestroyed>()
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
