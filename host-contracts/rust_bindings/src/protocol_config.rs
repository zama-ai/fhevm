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
///Module containing a contract's types and functions.
/**

```solidity
library IProtocolConfig {
    struct EpochCrsResult { uint256 crsId; uint256 maxBitLength; bytes crsDigest; bytes signature; }
    struct EpochKeyResult { uint256 prepKeygenId; uint256 keyId; IKMSGeneration.KeyDigest[] keyDigests; bytes signature; }
    struct KmsThresholds { uint256 publicDecryption; uint256 userDecryption; uint256 kmsGen; uint256 mpc; }
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
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IProtocolConfig`](self) contract instance.

See the [wrapper's documentation](`IProtocolConfigInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IProtocolConfigInstance<P, N> {
        IProtocolConfigInstance::<P, N>::new(address, provider)
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
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IProtocolConfigInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`IProtocolConfig`](self) contract instance.

See the [wrapper's documentation](`IProtocolConfigInstance`) for more details.*/
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
    #[automatically_derived]
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
    #[automatically_derived]
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
    struct KmsThresholds {
        uint256 publicDecryption;
        uint256 userDecryption;
        uint256 kmsGen;
        uint256 mpc;
    }
}

interface ProtocolConfig {
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
    struct PcrValues {
        bytes pcr0;
        bytes pcr1;
        bytes pcr2;
    }

    error AddressEmptyCode(address target);
    error CurrentKmsContextCannotBeDestroyed(uint256 kmsContextId);
    error ECDSAInvalidSignature();
    error ECDSAInvalidSignatureLength(uint256 length);
    error ECDSAInvalidSignatureS(bytes32 s);
    error ERC1967InvalidImplementation(address implementation);
    error ERC1967NonPayable();
    error EmptyKmsNodes();
    error EpochActivationAlreadyConfirmed(address signer, uint256 epochId);
    error EpochActivationSignerDoesNotMatchTxSender(address signer, address txSender);
    error EpochActivationUnauthorized(address caller, uint256 epochId);
    error EpochNotUnderActiveContext(uint256 epochId, uint256 contextId);
    error FailedCall();
    error InvalidEpoch(uint256 epochId);
    error InvalidHighThreshold(string thresholdName, uint256 threshold, uint256 nodeCount);
    error InvalidInitialization();
    error InvalidKmsContext(uint256 kmsContextId);
    error InvalidNullThreshold(string thresholdName);
    error KmsContextCreationAlreadyConfirmed(address txSender, uint256 kmsContextId);
    error KmsContextCreationUnauthorized(address caller, uint256 kmsContextId);
    error KmsContextNotCreated(uint256 kmsContextId);
    error KmsContextNotPending(uint256 kmsContextId);
    error KmsNodeNullSigner();
    error KmsNodeNullTxSender();
    error KmsSignerAlreadyRegistered(address signer);
    error KmsSignerSetExceedsProofFormatLimit(uint256 signerCount, uint256 maxAllowed);
    error KmsTxSenderAlreadyRegistered(address txSender);
    error NonIncreasingEpochId(uint256 epochId, uint256 currentEpochId);
    error NonIncreasingKmsContextId(uint256 contextId, uint256 latestActiveKmsContextId);
    error NotHostOwner(address sender);
    error NotInitializing();
    error NotInitializingFromEmptyProxy();
    error ThresholdExceedsProofFormatLimit(string thresholdName, uint256 threshold, uint256 maxAllowed);
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);

    event ActivateEpoch(uint256 indexed kmsContextId, uint256 indexed epochId, IProtocolConfig.EpochKeyResult[] keys, IProtocolConfig.EpochCrsResult[] crsList, string[] kmsNodeStorageUrls);
    event EpochActivationConfirmation(uint256 indexed epochId, address indexed signer, bytes32 dataHash);
    event Initialized(uint64 version);
    event KmsContextCreationConfirmation(uint256 indexed kmsContextId, address indexed txSender, bool isPreviousTxSender, bool isNewTxSender);
    event KmsContextDestroyed(uint256 indexed kmsContextId);
    event KmsGenThresholdUpdated(uint256 indexed kmsContextId, uint256 threshold);
    event MirrorKmsContextAndEpoch(uint256 indexed contextId, uint256 indexed epochId, KmsNodeParams[] kmsNodeParams, IProtocolConfig.KmsThresholds thresholds, string softwareVersion, PcrValues[] pcrValues);
    event MirrorKmsEpoch(uint256 indexed contextId, uint256 indexed epochId);
    event MpcThresholdUpdated(uint256 indexed kmsContextId, uint256 threshold);
    event NewKmsContext(uint256 indexed contextId, uint256 indexed previousContextId, KmsNodeParams[] kmsNodeParams, IProtocolConfig.KmsThresholds thresholds, string softwareVersion, PcrValues[] pcrValues);
    event NewKmsEpoch(uint256 indexed kmsContextId, uint256 indexed epochId, uint256 previousContextId, uint256 previousEpochId, uint256 materialBlockNumber);
    event PendingContextAborted(uint256 indexed kmsContextId);
    event PendingEpochAborted(uint256 indexed kmsContextId, uint256 indexed epochId);
    event PublicDecryptionThresholdUpdated(uint256 indexed kmsContextId, uint256 threshold);
    event Upgraded(address indexed implementation);
    event UserDecryptionThresholdUpdated(uint256 indexed kmsContextId, uint256 threshold);

    constructor();

    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function abortPendingContext(uint256 kmsContextId) external;
    function abortPendingEpoch(uint256 epochId) external;
    function confirmEpochActivation(uint256 epochId, IProtocolConfig.EpochKeyResult[] memory keys, IProtocolConfig.EpochCrsResult[] memory crsList) external;
    function confirmKmsContextCreation(uint256 kmsContextId) external;
    function defineNewEpochForCurrentKmsContext() external;
    function defineNewKmsContextAndEpoch(KmsNodeParams[] memory kmsNodeParams, IProtocolConfig.KmsThresholds memory thresholds, string memory softwareVersion, PcrValues[] memory pcrValues) external;
    function destroyKmsContext(uint256 kmsContextId) external;
    function getCurrentKmsContextAndEpoch() external view returns (uint256 contextId, uint256 epochId);
    function getCurrentKmsContextId() external view returns (uint256);
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
    function initializeFromCanonical(uint256 canonicalContextId, uint256 canonicalEpochId, KmsNodeParams[] memory canonicalKmsNodeParams, IProtocolConfig.KmsThresholds memory canonicalThresholds) external;
    function initializeFromEmptyProxy(KmsNodeParams[] memory initialKmsNodeParams, IProtocolConfig.KmsThresholds memory initialThresholds, string memory softwareVersion, PcrValues[] memory pcrValues) external;
    function isKmsSigner(address signer) external view returns (bool);
    function isKmsSignerForContext(uint256 kmsContextId, address signer) external view returns (bool);
    function isKmsTxSenderForContext(uint256 kmsContextId, address txSender) external view returns (bool);
    function isValidEpochForContext(uint256 kmsContextId, uint256 epochId) external view returns (bool);
    function isValidKmsContext(uint256 kmsContextId) external view returns (bool);
    function mirrorKmsContextAndEpoch(uint256 contextId, uint256 epochId, KmsNodeParams[] memory kmsNodeParams, IProtocolConfig.KmsThresholds memory thresholds, string memory softwareVersion, PcrValues[] memory pcrValues) external;
    function mirrorKmsEpoch(uint256 contextId, uint256 epochId) external;
    function proxiableUUID() external view returns (bytes32);
    function reinitializeV2(KmsNodeParams[] memory kmsNodeParams, IProtocolConfig.KmsThresholds memory thresholds, string memory softwareVersion, PcrValues[] memory pcrValues) external;
    function updateKmsGenThresholdForContext(uint256 kmsContextId, uint256 threshold) external;
    function updateMpcThresholdForContext(uint256 kmsContextId, uint256 threshold) external;
    function updatePublicDecryptionThresholdForContext(uint256 kmsContextId, uint256 threshold) external;
    function updateUserDecryptionThresholdForContext(uint256 kmsContextId, uint256 threshold) external;
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
    "name": "abortPendingContext",
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
    "name": "abortPendingEpoch",
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
        "internalType": "struct IProtocolConfig.KmsThresholds",
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
    "name": "initializeFromCanonical",
    "inputs": [
      {
        "name": "canonicalContextId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "canonicalEpochId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "canonicalKmsNodeParams",
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
        "name": "canonicalThresholds",
        "type": "tuple",
        "internalType": "struct IProtocolConfig.KmsThresholds",
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
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
        "internalType": "struct IProtocolConfig.KmsThresholds",
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
    "name": "mirrorKmsContextAndEpoch",
    "inputs": [
      {
        "name": "contextId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "epochId",
        "type": "uint256",
        "internalType": "uint256"
      },
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
        "internalType": "struct IProtocolConfig.KmsThresholds",
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
    "name": "mirrorKmsEpoch",
    "inputs": [
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
        "internalType": "struct IProtocolConfig.KmsThresholds",
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
    "name": "updateKmsGenThresholdForContext",
    "inputs": [
      {
        "name": "kmsContextId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "threshold",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateMpcThresholdForContext",
    "inputs": [
      {
        "name": "kmsContextId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "threshold",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updatePublicDecryptionThresholdForContext",
    "inputs": [
      {
        "name": "kmsContextId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "threshold",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateUserDecryptionThresholdForContext",
    "inputs": [
      {
        "name": "kmsContextId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "threshold",
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
    "name": "KmsGenThresholdUpdated",
    "inputs": [
      {
        "name": "kmsContextId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "threshold",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "MirrorKmsContextAndEpoch",
    "inputs": [
      {
        "name": "contextId",
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
        "internalType": "struct IProtocolConfig.KmsThresholds",
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
    "name": "MirrorKmsEpoch",
    "inputs": [
      {
        "name": "contextId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
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
    "name": "MpcThresholdUpdated",
    "inputs": [
      {
        "name": "kmsContextId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "threshold",
        "type": "uint256",
        "indexed": false,
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
        "internalType": "struct IProtocolConfig.KmsThresholds",
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
    "name": "PendingContextAborted",
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
    "name": "PendingEpochAborted",
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
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "PublicDecryptionThresholdUpdated",
    "inputs": [
      {
        "name": "kmsContextId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "threshold",
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
    "type": "event",
    "name": "UserDecryptionThresholdUpdated",
    "inputs": [
      {
        "name": "kmsContextId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "threshold",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
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
    "name": "CurrentKmsContextCannotBeDestroyed",
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
    "name": "EmptyKmsNodes",
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
    "name": "EpochNotUnderActiveContext",
    "inputs": [
      {
        "name": "epochId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "contextId",
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
    "name": "InvalidEpoch",
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
    "name": "KmsContextNotCreated",
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
    "name": "NonIncreasingEpochId",
    "inputs": [
      {
        "name": "epochId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "currentEpochId",
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
    "name": "ThresholdExceedsProofFormatLimit",
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
        "name": "maxAllowed",
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
pub mod ProtocolConfig {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a06040523060805234801562000014575f80fd5b506200001f62000025565b620000d9565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00805468010000000000000000900460ff1615620000765760405163f92ee8a960e01b815260040160405180910390fd5b80546001600160401b0390811614620000d65780546001600160401b0319166001600160401b0390811782556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a15b50565b60805161537c620001005f395f818161350a015281816135330152613713015261537c5ff3fe608060405260043610610233575f3560e01c806377d38e2411610129578063b4722bc4116100a8578063c3aaaa5a1161006d578063c3aaaa5a14610670578063c999a8b41461068f578063cceac019146106ae578063d9be2de4146106cd578063f9c670c3146106ec575f80fd5b8063b4722bc4146105eb578063bc4d07c2146105ff578063bf9b16c81461061e578063c0ae64f71461063d578063c2b429861461065c575f80fd5b8063976c98b5116100ee578063976c98b51461054a578063976f3eb914610569578063ad3cb1cc1461057d578063b0b461c4146105ad578063b181cda7146105cc575f80fd5b806377d38e24146104ba5780637eaac8f2146104d95780638aeac229146104ed5780638e97cb601461050c5780639447cfd41461052b575f80fd5b806331ff41c8116101b55780634cb950e11161017a5780634cb950e11461041f5780634f1ef2861461043e57806352d1902d146104515780635bff76d91461046557806365b394af14610491575f80fd5b806331ff41c8146103775780633b56159e146103a357806341ad069c146103c257806346c5bbbd146103e157806347e8229514610400575f80fd5b806320a4eb39116101fb57806320a4eb39146102e4578063221cdd4e1461030357806326cf5def14610322578063281e8bfe146103445780632a38899814610363575f80fd5b806306834d1d146102375780630d8e6e2c1461025857806316d4eb6f146102825780631ce3f9bc146102a1578063203d0114146102b5575b5f80fd5b348015610242575f80fd5b506102566102513660046141b0565b610718565b005b348015610263575f80fd5b5061026c610874565b604051610279919061421d565b60405180910390f35b34801561028d575f80fd5b5061025661029c36600461428c565b6108e0565b3480156102ac575f80fd5b50610256610a61565b3480156102c0575f80fd5b506102d46102cf366004614313565b610b7e565b6040519015158152602001610279565b3480156102ef575f80fd5b506102566102fe36600461432e565b610bbe565b34801561030e575f80fd5b5061025661031d366004614382565b610cee565b34801561032d575f80fd5b50610336610ee6565b604051908152602001610279565b34801561034f575f80fd5b5061033661035e36600461432e565b610f0c565b34801561036e575f80fd5b50610336610f31565b348015610382575f80fd5b50610396610391366004614428565b610f57565b60405161027991906144a3565b3480156103ae575f80fd5b506102566103bd36600461432e565b61111b565b3480156103cd575f80fd5b506103366103dc36600461432e565b61128c565b3480156103ec575f80fd5b506102d46103fb366004614428565b6112d1565b34801561040b575f80fd5b5061033661041a36600461432e565b61132f565b34801561042a575f80fd5b506102566104393660046144b5565b611354565b61025661044c366004614600565b611b3c565b34801561045c575f80fd5b50610336611b5b565b348015610470575f80fd5b5061048461047f36600461432e565b611b76565b604051610279919061464c565b34801561049c575f80fd5b506104a5611bf9565b60408051928352602083019190915201610279565b3480156104c5575f80fd5b506102566104d43660046141b0565b611c19565b3480156104e4575f80fd5b50610484611d56565b3480156104f8575f80fd5b50610256610507366004614382565b611dd7565b348015610517575f80fd5b506102566105263660046141b0565b611f9f565b348015610536575f80fd5b506102d4610545366004614428565b6120fd565b348015610555575f80fd5b50610256610564366004614382565b61213b565b348015610574575f80fd5b50610336612336565b348015610588575f80fd5b5061026c604051806040016040528060058152602001640352e302e360dc1b81525081565b3480156105b8575f80fd5b506102566105c73660046141b0565b61234a565b3480156105d7575f80fd5b506102566105e63660046141b0565b61248a565b3480156105f6575f80fd5b506103366125d2565b34801561060a575f80fd5b50610256610619366004614698565b6125f8565b348015610629575f80fd5b506102d461063836600461432e565b61275c565b348015610648575f80fd5b5061025661065736600461432e565b612766565b348015610667575f80fd5b5061033661289f565b34801561067b575f80fd5b5061033661068a36600461432e565b6128c5565b34801561069a575f80fd5b506104a56106a936600461432e565b6128ea565b3480156106b9575f80fd5b506102d46106c83660046141b0565b612951565b3480156106d8575f80fd5b506102566106e736600461432e565b6129a6565b3480156106f7575f80fd5b5061070b61070636600461432e565b612bed565b6040516102799190614754565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610768573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061078c91906147b6565b6001600160a01b0316336001600160a01b0316146107c45760405163021bfda160e41b81523360048201526024015b60405180910390fd5b5f6107cd612db0565b90506107d883612dd4565b6108216040518060400160405280601081526020016f383ab13634b1a232b1b93cb83a34b7b760811b81525083836001015f8781526020019081526020015f2080549050612e00565b5f838152600682016020526040908190208390555183907fd571bf833e41553bbe260e00b3af7a0e91aafd6cdc238a803aa9ac0e73efed65906108679085815260200190565b60405180910390a2505050565b60606040518060400160405280600e81526020016d50726f746f636f6c436f6e66696760901b8152506108a65f612e77565b6108b06002612e77565b6108b95f612e77565b6040516020016108cc94939291906147d1565b604051602081830303815290604052905090565b5f8051602061535c833981519152546001600160401b03166001600160401b031660011461092157604051636f4f731f60e01b815260040160405180910390fd5b5f8051602061535c833981519152805460039190600160401b900460ff1680610957575080546001600160401b03808416911610155b156109755760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b03831617600160401b17815560f860076109a6911b6001614876565b8710156109c9576040516377ddbe8160e01b8152600481018890526024016107bb565b6109d8600160fb1b6001614876565b8610156109fb5760405163a225656d60e01b8152600481018790526024016107bb565b610a108787610a0a878961489a565b86612f06565b50805460ff60401b191681556040516001600160401b03831681527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a150505050505050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610ab1573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ad591906147b6565b6001600160a01b0316336001600160a01b031614610b085760405163021bfda160e41b81523360048201526024016107bb565b5f610b11612db0565b600b8101549091505f610b2382612f5a565b905080827f15aaaf475ef407543f5164f57dcf57f7f93816f55bae77ca09efc445ba40eef78486600d0154600143610b5b9190614a09565b6040805193845260208401929092529082015260600160405180910390a3505050565b5f80610b88612db0565b600b8101545f9081526003909101602090815260408083206001600160a01b039096168352949052929092205460ff1692915050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610c0e573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c3291906147b6565b6001600160a01b0316336001600160a01b031614610c655760405163021bfda160e41b81523360048201526024016107bb565b5f610c6e612db0565b905060015f838152600e8301602052604090205460ff166003811115610c9657610c9661484e565b14610cb757604051633586efa160e01b8152600481018390526024016107bb565b610cc082612fab565b60405182907f75e115b7f76bf21d0a2e42da9304d9c357b54c489e5af59ed3c70b7cd48335fc905f90a25050565b5f8051602061535c833981519152546001600160401b03166001600160401b0316600114610d2f57604051636f4f731f60e01b815260040160405180910390fd5b5f8051602061535c833981519152805460039190600160401b900460ff1680610d65575080546001600160401b03808416911610155b15610d835760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b03831617600160401b1781555f610dad612db0565b90505f610de1610dc2600760f81b6001614876565b610dd1600160fb1b6001614876565b610ddb8d8f61489a565b8c612f06565b905060405180604001604052804381526020018c8c8c8c8c8c8c604051602001610e119796959493929190614b4a565b60408051601f1981840301815291815281516020928301209092525f848152601786018252919091208251815591015160019091015560f86007901b817f204d6b80121154cd87d99cf54c639a3dd0a53b3084277098de972ebdd34c6be98d8d8d8d8d8d8d604051610e899796959493929190614b4a565b60405180910390a35050805460ff60401b191681556040516001600160401b03831681527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2906020015b60405180910390a1505050505050505050565b5f80610ef0612db0565b600b8101545f9081526009909101602052604090205492915050565b5f610f1682612dd4565b610f1e612db0565b5f92835260070160205250604090205490565b5f80610f3b612db0565b600b8101545f9081526006909101602052604090205492915050565b604080516080810182525f8082526020820152606091810182905281810191909152610f8283613066565b610fa2576040516377ddbe8160e01b8152600481018490526024016107bb565b610faa612db0565b5f848152600491909101602090815260408083206001600160a01b0380871685529083529281902081516080810183528154851681526001820154909416928401929092526002820180549184019161100290614d12565b80601f016020809104026020016040519081016040528092919081815260200182805461102e90614d12565b80156110795780601f1061105057610100808354040283529160200191611079565b820191905f5260205f20905b81548152906001019060200180831161105c57829003601f168201915b5050505050815260200160038201805461109290614d12565b80601f01602080910402602001604051908101604052809291908181526020018280546110be90614d12565b80156111095780601f106110e057610100808354040283529160200191611109565b820191905f5260205f20905b8154815290600101906020018083116110ec57829003601f168201915b50505050508152505090505b92915050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561116b573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061118f91906147b6565b6001600160a01b0316336001600160a01b0316146111c25760405163021bfda160e41b81523360048201526024016107bb565b5f6111cb612db0565b905060015f838152600f8301602052604090205460ff1660028111156111f3576111f361484e565b146112145760405163a225656d60e01b8152600481018390526024016107bb565b5f828152601082016020526040902054600b82015481146112525760405163a69d7d5b60e01b815260048101849052602481018290526044016107bb565b61125b8361309e565b604051839082907f6440aaea7b2480b82449c317aa5a9168df77eb69308ff8f7c3980a1ad848b7df905f90a3505050565b5f61129682613066565b6112b6576040516377ddbe8160e01b8152600481018390526024016107bb565b6112be612db0565b5f92835260080160205250604090205490565b5f6112db83613066565b6112fb576040516377ddbe8160e01b8152600481018490526024016107bb565b611303612db0565b5f938452600201602090815260408085206001600160a01b039490941685529290525090205460ff1690565b5f61133982612dd4565b611341612db0565b5f92835260090160205250604090205490565b5f61135d612db0565b905060015f878152600f8301602052604090205460ff1660028111156113855761138561484e565b146113a65760405163a225656d60e01b8152600481018790526024016107bb565b5f8681526010820160209081526040808320548084526002850183528184203385529092529091205460ff166113f85760405163a3f4afeb60e01b8152336004820152602481018890526044016107bb565b60015f828152600e8401602052604090205460ff16600381111561141e5761141e61484e565b0361143f57604051631962dcfb60e11b8152600481018290526024016107bb565b61144881613066565b611468576040516377ddbe8160e01b8152600481018290526024016107bb565b5f81815260048301602090815260408083203384528252808320600101548151600160f91b938101939093526021830185905260418084018c90528251808503909101815260619093019091526001600160a01b0316919081886001600160401b038111156114d9576114d9614528565b604051908082528060200260200182016040528015611502578160200160208202803683370190505b5090505f5b89811015611690575f61154a8c8c8481811061152557611525614d44565b90506020028101906115379190614d58565b611545906040810190614d76565b6130d1565b90505f6115a48d8d8581811061156257611562614d44565b90506020028101906115749190614d58565b358e8e8681811061158757611587614d44565b90506020028101906115999190614d58565b602001358488613237565b90506115e287828f8f878181106115bd576115bd614d44565b90506020028101906115cf9190614d58565b6115dd906060810190614dbb565b6133b3565b8c8c848181106115f4576115f4614d44565b90506020028101906116069190614d58565b358d8d8581811061161957611619614d44565b905060200281019061162b9190614d58565b6020013583604051602001611653939291909283526020830191909152604082015260600190565b6040516020818303038152906040528051906020012084848151811061167b5761167b614d44565b60209081029190910101525050600101611507565b505f876001600160401b038111156116aa576116aa614528565b6040519080825280602002602001820160405280156116d3578160200160208202803683370190505b5090505f5b88811015611850575f6117698b8b848181106116f6576116f6614d44565b90506020028101906117089190614d58565b358c8c8581811061171b5761171b614d44565b905060200281019061172d9190614d58565b602001358d8d8681811061174357611743614d44565b90506020028101906117559190614d58565b611763906040810190614dbb565b89613438565b905061178287828d8d868181106115bd576115bd614d44565b8a8a8381811061179457611794614d44565b90506020028101906117a69190614d58565b358b8b848181106117b9576117b9614d44565b90506020028101906117cb9190614d58565b602001358c8c858181106117e1576117e1614d44565b90506020028101906117f39190614d58565b611801906040810190614dbb565b6040516020016118149493929190614dfd565b6040516020818303038152906040528051906020012083838151811061183c5761183c614d44565b6020908102919091010152506001016116d8565b508181604051602001611864929190614e56565b60408051601f1981840301815291815281516020928301205f8f815260128b0184528281206001600160a01b038a16825290935291205490945060ff161592506118d6915050576040516324a0bb1b60e11b81526001600160a01b0383166004820152602481018a90526044016107bb565b5f89815260128501602090815260408083206001600160a01b03861684528252808320805460ff191660011790558b835260138701825280832084845290915281208054829061192590614e7a565b9190508190559050826001600160a01b03168a7f7eda6f85e23b7b91c019b0570d02b663606ef9d74594f7e01fcfbdb0f4e954d58460405161196991815260200190565b60405180910390a35f8481526005860160205260409020548103611b30575f848152600e860160205260409020805460ff19166003179055600b85018490556119b28a856134c5565b5f848152600186016020526040812080549091906001600160401b038111156119dd576119dd614528565b604051908082528060200260200182016040528015611a1057816020015b60608152602001906001900390816119fb5790505b5090505f5b8254811015611aeb57828181548110611a3057611a30614d44565b905f5260205f2090600402016003018054611a4a90614d12565b80601f0160208091040260200160405190810160405280929190818152602001828054611a7690614d12565b8015611ac15780601f10611a9857610100808354040283529160200191611ac1565b820191905f5260205f20905b815481529060010190602001808311611aa457829003601f168201915b5050505050828281518110611ad857611ad8614d44565b6020908102919091010152600101611a15565b508b867f1a547b42e72cd3dda04e6adccd2200276cfef01fe2138d07f3a7440f416d38bc8d8d8d8d87604051611b2595949392919061504c565b60405180910390a350505b50505050505050505050565b611b446134ff565b611b4d826135a5565b611b57828261364c565b5050565b5f611b64613708565b505f8051602061533c83398151915290565b6060611b8182612dd4565b611b89612db0565b6005015f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020018280548015611bed57602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311611bcf575b50505050509050919050565b5f805f611c04612db0565b905080600b0154925080600d01549150509091565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611c69573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c8d91906147b6565b6001600160a01b0316336001600160a01b031614611cc05760405163021bfda160e41b81523360048201526024016107bb565b5f611cc9612db0565b9050611cd483612dd4565b611d10604051806040016040528060038152602001626d706360e81b81525083836001015f8781526020019081526020015f2080549050612e00565b5f838152600982016020526040908190208390555183907f148f9c6cb77d12306b9f596534d14b7aae3e4f98a2dbe3cdb07ea4924c775f12906108679085815260200190565b60605f611d61612db0565b9050806005015f82600b015481526020019081526020015f20805480602002602001604051908101604052809291908181526020018280548015611dcc57602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311611dae575b505050505091505090565b5f8051602061535c833981519152805460039190600160401b900460ff1680611e0d575080546001600160401b03808416911610155b15611e2b5760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b03831617600160401b1781555f611e55612db0565b600160fb1b600c82019081558154600b83018190555f818152600e840160205260408120805460ff19166003179055825493945090929091908290611e9990614e7a565b91829055509050611eaa81836134c5565b60405180604001604052804381526020018d8d8d8d8d8d8d604051602001611ed89796959493929190614b4a565b60408051601f1981840301815291815281516020928301209092525f858152601787018252919091208251815591015160019091015560f86007901b827f204d6b80121154cd87d99cf54c639a3dd0a53b3084277098de972ebdd34c6be98e8e8e8e8e8e8e604051611f509796959493929190614b4a565b60405180910390a35050815460ff60401b19168255506040516001600160401b03831681527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d290602001610ed3565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611fef573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061201391906147b6565b6001600160a01b0316336001600160a01b0316146120465760405163021bfda160e41b81523360048201526024016107bb565b5f61204f612db0565b905080600b01548314158061206a575061206883613066565b155b1561208b576040516377ddbe8160e01b8152600481018490526024016107bb565b600c8101548083116120ba5760405163e8121f5160e01b815260048101849052602481018290526044016107bb565b600c82018390556120cb83856134c5565b604051839085907f0a1c24c2ba5e6e1b1a8585795e5b781e372aee1db686247dac7574c10fd735a6905f90a350505050565b5f61210783612dd4565b61210f612db0565b5f938452600301602090815260408085206001600160a01b039490941685529290525090205460ff1690565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561218b573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906121af91906147b6565b6001600160a01b0316336001600160a01b0316146121e25760405163021bfda160e41b81523360048201526024016107bb565b5f6121eb612db0565b600b8101549091505f6122076122018a8c61489a565b89613751565b5f818152600e850160205260409020805460ff19166001179055905061222c81612f5a565b505f828152600984016020908152604080832054600187019092528220546122549190614a09565b90505f8111612264576001612266565b805b846014015f8481526020019081526020015f208190555060405180604001604052804381526020018c8c8c8c8c8c8c6040516020016122ab9796959493929190614b4a565b60408051601f1981840301815291815281516020928301209092525f8581526017880182528290208351815592015160019092019190915551839083907f204d6b80121154cd87d99cf54c639a3dd0a53b3084277098de972ebdd34c6be990612321908f908f908f908f908f908f908f90614b4a565b60405180910390a35050505050505050505050565b5f80612340612db0565b600b015492915050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561239a573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906123be91906147b6565b6001600160a01b0316336001600160a01b0316146123f15760405163021bfda160e41b81523360048201526024016107bb565b5f6123fa612db0565b905061240583612dd4565b6124446040518060400160405280600681526020016535b6b9a3b2b760d11b81525083836001015f8781526020019081526020015f2080549050612e00565b5f838152600882016020526040908190208390555183907ff21cb37be709148aabebd278543e62d1b1e6a4477fb1cc43e069d3eeb8c87f90906108679085815260200190565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156124da573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906124fe91906147b6565b6001600160a01b0316336001600160a01b0316146125315760405163021bfda160e41b81523360048201526024016107bb565b5f61253a612db0565b905061254583612dd4565b61258c6040518060400160405280600e81526020016d3ab9b2b92232b1b93cb83a34b7b760911b81525083836001015f8781526020019081526020015f2080549050612e00565b5f838152600782016020526040908190208390555183907f90f1918493831c1b6133489743103384c5600eae796eb34c51ea4f2baafa4f94906108679085815260200190565b5f806125dc612db0565b600b8101545f9081526008909101602052604090205492915050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612648573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061266c91906147b6565b6001600160a01b0316336001600160a01b03161461269f5760405163021bfda160e41b81523360048201526024016107bb565b5f6126a8612db0565b600b810154909150808b116126da5760405163efd55f6760e01b8152600481018c9052602481018290526044016107bb565b600c820154808b116127095760405163e8121f5160e01b8152600481018c9052602481018290526044016107bb565b61271e8c8c6127188c8e61489a565b8b612f06565b508a8c7f2ac68f78f4ccde76b64906026d01ff3c42403eb7eef86fe788474a23267d64cf8c8c8c8c8c8c8c604051611b259796959493929190614b4a565b5f61111582613776565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156127b6573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906127da91906147b6565b6001600160a01b0316336001600160a01b03161461280d5760405163021bfda160e41b81523360048201526024016107bb565b5f612816612db0565b905080600b0154820361283f576040516322cafe7160e11b8152600481018390526024016107bb565b61284882613066565b612868576040516377ddbe8160e01b8152600481018390526024016107bb565b61287182612fab565b60405182907fda075d09198d207e3a918d4b8dfc87df2d60a00be703fd39eaac90962da0b7f0905f90a25050565b5f806128a9612db0565b600b8101545f9081526007909101602052604090205492915050565b5f6128cf82612dd4565b6128d7612db0565b5f92835260060160205250604090205490565b5f806128f5836137c0565b612915576040516377ddbe8160e01b8152600481018490526024016107bb565b5f61291e612db0565b5f948552601701602090815260409485902085518087019096528054808752600190910154959091018590529492505050565b5f8061295b612db0565b905060025f848152600f8301602052604090205460ff1660028111156129835761298361484e565b14801561299e57505f83815260108201602052604090205484145b949350505050565b5f6129af612db0565b905060015f838152600e8301602052604090205460ff1660038111156129d7576129d761484e565b146129f857604051633586efa160e01b8152600481018390526024016107bb565b600b8101545f818152600283016020818152604080842033808652908352818520548886529383528185209085529091529091205460ff918216911681158015612a40575080155b15612a6757604051631703bf1d60e31b8152336004820152602481018690526044016107bb565b5f858152601185016020908152604080832033845290915290205460ff1615612aac57604051630c4b0b9960e31b8152336004820152602481018690526044016107bb565b5f85815260118501602090815260408083203384529091529020805460ff191660011790558115612af9575f85815260168501602052604081208054909190612af490614e7a565b909155505b8015612b21575f85815260158501602052604081208054909190612b1c90614e7a565b909155505b6040805183151581528215156020820152339187917fb79c48003695b6ebe555afa36fad071deeee75eb3718ad63de5621d35ba44b4f910160405180910390a3612b6a85613807565b15612be6575f858152600e850160205260409020805460ff19166002179055600c840154600d850154819087907f15aaaf475ef407543f5164f57dcf57f7f93816f55bae77ca09efc445ba40eef7908790612bc6600143614a09565b6040805193845260208401929092529082015260600160405180910390a3505b5050505050565b6060612bf882612dd4565b612c00612db0565b6001015f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020015f905b82821015612da5575f848152602090819020604080516080810182526004860290920180546001600160a01b0390811684526001820154169383019390935260028301805492939291840191612c8690614d12565b80601f0160208091040260200160405190810160405280929190818152602001828054612cb290614d12565b8015612cfd5780601f10612cd457610100808354040283529160200191612cfd565b820191905f5260205f20905b815481529060010190602001808311612ce057829003601f168201915b50505050508152602001600382018054612d1690614d12565b80601f0160208091040260200160405190810160405280929190818152602001828054612d4290614d12565b8015612d8d5780601f10612d6457610100808354040283529160200191612d8d565b820191905f5260205f20905b815481529060010190602001808311612d7057829003601f168201915b50505050508152505081526020019060010190612c31565b505050509050919050565b7f80f3585af86806c5774303b06c1ee640aa83b6ef3e45df49bb26c8524500c20090565b612ddd81613776565b612dfd576040516377ddbe8160e01b8152600481018290526024016107bb565b50565b815f03612e225782604051631b5fdb0760e11b81526004016107bb919061421d565b60ff821115612e4b576040516322ba52db60e01b81526107bb908490849060ff90600401615159565b80821115612e725782828260405163caa814a360e01b81526004016107bb93929190615159565b505050565b60605f612e8383613861565b60010190505f816001600160401b03811115612ea157612ea1614528565b6040519080825280601f01601f191660200182016040528015612ecb576020820181803683370190505b5090508181016020015b5f19016f181899199a1a9b1b9c1cb0b131b232b360811b600a86061a8153600a8504945084612ed557509392505050565b5f80612f10612db0565b9050612f1d868585613938565b5f818152600e830160205260409020805460ff19166003179055600b8201819055600c82018690559150612f5185836134c5565b50949350505050565b5f80612f64612db0565b905080600c015f8154612f7690614e7a565b91829055505f818152600f830160209081526040808320805460ff191660011790556010909401905291909120929092555090565b5f612fb4612db0565b5f838152600a8201602090815260408083208054600160ff199182168117909255600e8601845282852080549091169055600c850154808552600f86019093529220549293509160ff16600281111561300f5761300f61484e565b14801561302a57505f81815260108301602052604090205483145b15613038576130388161309e565b505f918252601481016020908152604080842084905560158301825280842084905560169092019052812055565b5f80613070612db0565b905061307b836137c0565b801561309757505f838152600a8201602052604090205460ff16155b9392505050565b5f6130a7612db0565b5f928352600f810160209081526040808520805460ff191690556010909201905282209190915550565b5f80826001600160401b038111156130eb576130eb614528565b604051908082528060200260200182016040528015613114578160200160208202803683370190505b5090505f5b83811015613206577fddd108772e6a3899feb04d148ae915cbe3eb5ebd202688080399e9921ac3616b85858381811061315457613154614d44565b9050602002810190613166919061517d565b613174906020810190615191565b86868481811061318657613186614d44565b9050602002810190613198919061517d565b6131a6906020810190614dbb565b6040516131b49291906151aa565b6040519081900381206131cb9392916020016151b9565b604051602081830303815290604052805190602001208282815181106131f3576131f3614d44565b6020908102919091010152600101613119565b508060405160200161321891906151db565b6040516020818303038152906040528051906020012091505092915050565b8051602080830191909120604080517fbd14835bb4ae13c78ecb88ded2c3370325f39e6006eb94ff45e95f98e4c85a2a938101939093528201869052606082018590526080820184905260a08201525f906133aa9060c0015b60405160208183030381529060405280519060200120604080518082018252600e81526d50726f746f636f6c436f6e66696760901b6020918201528151808301835260018152603160f81b9082015281517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f818301527fa3de1880cf083e8318b77a7965d02dd9765e85a48e418a4463af7a0d57b4b3ee818401527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc660608201524660808201523060a0808301919091528351808303909101815260c08201845280519083012061190160f01b60e083015260e2820152610102808201949094528251808203909401845261012201909152815191012090565b95945050505050565b5f6133f38484848080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250613b5792505050565b9050846001600160a01b0316816001600160a01b031614612be6576040516378b9ada360e11b81526001600160a01b03821660048201523360248201526044016107bb565b5f6134bb7fa264b318e95080300a3f06a6656a8e7fe24f9903f0e6bcca307efbe39c4c4e09878787876040516020016134729291906151aa565b60408051601f19818403018152828252805160209182012089518a83012091840196909652908201939093526060810191909152608081019290925260a082015260c001613290565b9695505050505050565b5f6134ce612db0565b5f848152600f820160209081526040808320805460ff191660021790556010840190915290209290925550600d0155565b306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016148061358557507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166135795f8051602061533c833981519152546001600160a01b031690565b6001600160a01b031614155b156135a35760405163703e46dd60e11b815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156135f5573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061361991906147b6565b6001600160a01b0316336001600160a01b031614612dfd5760405163021bfda160e41b81523360048201526024016107bb565b816001600160a01b03166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa9250505080156136a6575060408051601f3d908101601f191682019092526136a391810190615210565b60015b6136ce57604051634c9c8ce360e01b81526001600160a01b03831660048201526024016107bb565b5f8051602061533c83398151915281146136fe57604051632a87526960e21b8152600481018290526024016107bb565b612e728383613b7f565b306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146135a35760405163703e46dd60e11b815260040160405180910390fd5b5f8061375b612db0565b805490915061299e9061376f906001614876565b8585613938565b5f80613780612db0565b905061378b83613066565b8015613097575060035f848152600e8301602052604090205460ff1660038111156137b8576137b861484e565b149392505050565b5f806137ca612db0565b90506137db600760f81b6001614876565b83101580156137eb575080548311155b801561309757505f928352600101602052506040902054151590565b5f80613811612db0565b5f848152600182016020908152604080832054601585019092529091205491925014801561309757505f838152601482016020908152604080832054601685019092529091205410159392505050565b5f8072184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b831061389f5772184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b830492506040015b6d04ee2d6d415b85acef810000000083106138cb576d04ee2d6d415b85acef8100000000830492506020015b662386f26fc1000083106138e957662386f26fc10000830492506010015b6305f5e1008310613901576305f5e100830492506008015b612710831061391557612710830492506004015b60648310613927576064830492506002015b600a83106111155760010192915050565b5f82515f0361395957604051621a323560e61b815260040160405180910390fd5b825160ff10156139895782516040516302d4e4ef60e31b8152600481019190915260ff60248201526044016107bb565b6139c06040518060400160405280601081526020016f383ab13634b1a232b1b93cb83a34b7b760811b815250835f01358551612e00565b6139f66040518060400160405280600e81526020016d3ab9b2b92232b1b93cb83a34b7b760911b81525083602001358551612e00565b613a246040518060400160405280600681526020016535b6b9a3b2b760d11b81525083604001358551612e00565b613a4f604051806040016040528060038152602001626d706360e81b81525083606001358551612e00565b5f613a58612db0565b80549091508511613a8957805460405163efd55f6760e01b81526107bb918791600401918252602082015260400190565b8481558491505f5b8451811015613b0b575f858281518110613aad57613aad614d44565b60200260200101519050613b02846040518060800160405280845f01516001600160a01b0316815260200184602001516001600160a01b03168152602001846040015181526020018460600151815250613bd4565b50600101613a91565b505f828152600682016020908152604080832086359055600784018252808320828701359055600884018252808320818701359055600990930190522060609092013590915592915050565b5f805f80613b658686613e77565b925092509250613b758282613ec0565b5090949350505050565b613b8882613f78565b6040516001600160a01b038316907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a2805115613bcc57612e728282613fdb565b611b57614044565b5f613bdd612db0565b82519091506001600160a01b0316613c0857604051634233402560e11b815260040160405180910390fd5b60208201516001600160a01b0316613c3357604051632deccf4d60e01b815260040160405180910390fd5b5f838152600282016020908152604080832085516001600160a01b0316845290915290205460ff1615613c87578151604051630d18c4ff60e41b81526001600160a01b0390911660048201526024016107bb565b5f8381526003820160209081526040808320858301516001600160a01b0316845290915290205460ff1615613ce057602082015160405163f51af6bb60e01b81526001600160a01b0390911660048201526024016107bb565b5f8381526001828101602090815260408084208054808501825590855293829020865160049095020180546001600160a01b03199081166001600160a01b03968716178255928701519381018054909316939094169290921790558301518391906002820190613d50908261526b565b5060608201516003820190613d65908261526b565b5050505f83815260028083016020908152604080842086516001600160a01b039081168652908352818520805460ff1990811660019081179092558987526003880185528387208986018051851689529086528488208054909216831790915589875260048801855283872089518416885290945294829020875181549083166001600160a01b03199182161782559351958101805496909216959093169490941790935591840151849291820190613e1e908261526b565b5060608201516003820190613e33908261526b565b5050505f92835260050160209081526040832091810151825460018101845592845292200180546001600160a01b0319166001600160a01b03909216919091179055565b5f805f8351604103613eae576020840151604085015160608601515f1a613ea088828585614063565b955095509550505050613eb9565b505081515f91506002905b9250925092565b5f826003811115613ed357613ed361484e565b03613edc575050565b6001826003811115613ef057613ef061484e565b03613f0e5760405163f645eedf60e01b815260040160405180910390fd5b6002826003811115613f2257613f2261484e565b03613f435760405163fce698f760e01b8152600481018290526024016107bb565b6003826003811115613f5757613f5761484e565b03611b57576040516335e2f38360e21b8152600481018290526024016107bb565b806001600160a01b03163b5f03613fad57604051634c9c8ce360e01b81526001600160a01b03821660048201526024016107bb565b5f8051602061533c83398151915280546001600160a01b0319166001600160a01b0392909216919091179055565b60605f80846001600160a01b031684604051613ff7919061532a565b5f60405180830381855af49150503d805f811461402f576040519150601f19603f3d011682016040523d82523d5f602084013e614034565b606091505b50915091506133aa85838361412b565b34156135a35760405163b398979f60e01b815260040160405180910390fd5b5f80807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084111561409c57505f91506003905082614121565b604080515f808252602082018084528a905260ff891692820192909252606081018790526080810186905260019060a0016020604051602081039080840390855afa1580156140ed573d5f803e3d5ffd5b5050604051601f1901519150506001600160a01b03811661411857505f925060019150829050614121565b92505f91508190505b9450945094915050565b6060826141405761413b82614187565b613097565b815115801561415757506001600160a01b0384163b155b1561418057604051639996b31560e01b81526001600160a01b03851660048201526024016107bb565b5092915050565b8051156141975780518082602001fd5b60405163d6bda27560e01b815260040160405180910390fd5b5f80604083850312156141c1575f80fd5b50508035926020909101359150565b5f5b838110156141ea5781810151838201526020016141d2565b50505f910152565b5f81518084526142098160208601602086016141d0565b601f01601f19169290920160200192915050565b602081525f61309760208301846141f2565b5f8083601f84011261423f575f80fd5b5081356001600160401b03811115614255575f80fd5b6020830191508360208260051b850101111561426f575f80fd5b9250929050565b5f60808284031215614286575f80fd5b50919050565b5f805f805f60e086880312156142a0575f80fd5b853594506020860135935060408601356001600160401b038111156142c3575f80fd5b6142cf8882890161422f565b90945092506142e390508760608801614276565b90509295509295909350565b6001600160a01b0381168114612dfd575f80fd5b803561430e816142ef565b919050565b5f60208284031215614323575f80fd5b8135613097816142ef565b5f6020828403121561433e575f80fd5b5035919050565b5f8083601f840112614355575f80fd5b5081356001600160401b0381111561436b575f80fd5b60208301915083602082850101111561426f575f80fd5b5f805f805f805f60e0888a031215614398575f80fd5b87356001600160401b03808211156143ae575f80fd5b6143ba8b838c0161422f565b90995097508791506143cf8b60208c01614276565b965060a08a01359150808211156143e4575f80fd5b6143f08b838c01614345565b909650945060c08a0135915080821115614408575f80fd5b506144158a828b0161422f565b989b979a50959850939692959293505050565b5f8060408385031215614439575f80fd5b82359150602083013561444b816142ef565b809150509250929050565b5f60018060a01b038083511684528060208401511660208501525060408201516080604085015261448a60808501826141f2565b9050606083015184820360608601526133aa82826141f2565b602081525f6130976020830184614456565b5f805f805f606086880312156144c9575f80fd5b8535945060208601356001600160401b03808211156144e6575f80fd5b6144f289838a0161422f565b9096509450604088013591508082111561450a575f80fd5b506145178882890161422f565b969995985093965092949392505050565b634e487b7160e01b5f52604160045260245ffd5b60405161010081016001600160401b038111828210171561455f5761455f614528565b60405290565b604051601f8201601f191681016001600160401b038111828210171561458d5761458d614528565b604052919050565b5f82601f8301126145a4575f80fd5b81356001600160401b038111156145bd576145bd614528565b6145d0601f8201601f1916602001614565565b8181528460208386010111156145e4575f80fd5b816020850160208301375f918101602001919091529392505050565b5f8060408385031215614611575f80fd5b823561461c816142ef565b915060208301356001600160401b03811115614636575f80fd5b61464285828601614595565b9150509250929050565b602080825282518282018190525f9190848201906040850190845b8181101561468c5783516001600160a01b031683529284019291840191600101614667565b50909695505050505050565b5f805f805f805f805f6101208a8c0312156146b1575f80fd5b8935985060208a0135975060408a01356001600160401b03808211156146d5575f80fd5b6146e18d838e0161422f565b90995097508791506146f68d60608e01614276565b965060e08c013591508082111561470b575f80fd5b6147178d838e01614345565b90965094506101008c0135915080821115614730575f80fd5b5061473d8c828d0161422f565b915080935050809150509295985092959850929598565b5f60208083016020845280855180835260408601915060408160051b8701019250602087015f5b828110156147a957603f19888603018452614797858351614456565b9450928501929085019060010161477b565b5092979650505050505050565b5f602082840312156147c6575f80fd5b8151613097816142ef565b5f85516147e2818460208a016141d0565b61103b60f11b9083019081528551614801816002840160208a016141d0565b808201915050601760f91b8060028301528551614825816003850160208a016141d0565b600392019182015283516148408160048401602088016141d0565b016004019695505050505050565b634e487b7160e01b5f52602160045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b8082018082111561111557611115614862565b8035600381900b811461430e575f80fd5b5f6001600160401b03808411156148b3576148b3614528565b8360051b60206148c4818301614565565b8681529185019181810190368411156148db575f80fd5b865b848110156149fd578035868111156148f3575f80fd5b8801610100368290031215614906575f80fd5b61490e61453c565b61491782614303565b8152614924868301614303565b868201526040808301358981111561493a575f80fd5b61494636828601614595565b8284015250506060808301358981111561495e575f80fd5b61496a36828601614595565b828401525050608061497d818401614889565b9082015260a08281013589811115614993575f80fd5b61499f36828601614595565b82840152505060c080830135898111156149b7575f80fd5b6149c336828601614595565b82840152505060e080830135898111156149db575f80fd5b6149e736828601614595565b91830191909152508452509183019183016148dd565b50979650505050505050565b8181038181111561111557611115614862565b5f808335601e19843603018112614a31575f80fd5b83016020810192503590506001600160401b03811115614a4f575f80fd5b80360382131561426f575f80fd5b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b5f8383855260208086019550808560051b830101845f5b87811015614b3d57848303601f19018952813536889003605e19018112614ac1575f80fd5b87016060614acf8280614a1c565b828752614adf8388018284614a5d565b92505050614aef86830183614a1c565b86830388880152614b01838284614a5d565b925050506040614b1381840184614a1c565b935086830382880152614b27838583614a5d565b9c88019c96505050928501925050600101614a9c565b5090979650505050505050565b60e080825281018790525f61010080830160058a901b840182018b845b8c811015614caa5786830360ff190184528135368f900360fe19018112614b8c575f80fd5b8e01614ba884614b9b83614303565b6001600160a01b03169052565b6020614bb5818301614303565b6001600160a01b0316818601526040614bd083820184614a1c565b8983890152614be28a89018284614a5d565b925050506060614bf481850185614a1c565b888403838a0152614c06848284614a5d565b93505050506080614c18818501614889565b614c268289018260030b9052565b505060a0614c3681850185614a1c565b888403838a0152614c48848284614a5d565b935050505060c0614c5b81850185614a1c565b888403838a0152614c6d848284614a5d565b9350505050614c7f60e0840184614a1c565b935086820360e0880152614c94828583614a5d565b9783019796505050929092019150600101614b67565b5050614cda602086018b803582526020810135602083015260408101356040830152606081013560608301525050565b84810360a0860152614ced81898b614a5d565b9250505082810360c0840152614d04818587614a85565b9a9950505050505050505050565b600181811c90821680614d2657607f821691505b60208210810361428657634e487b7160e01b5f52602260045260245ffd5b634e487b7160e01b5f52603260045260245ffd5b5f8235607e19833603018112614d6c575f80fd5b9190910192915050565b5f808335601e19843603018112614d8b575f80fd5b8301803591506001600160401b03821115614da4575f80fd5b6020019150600581901b360382131561426f575f80fd5b5f808335601e19843603018112614dd0575f80fd5b8301803591506001600160401b03821115614de9575f80fd5b60200191503681900382131561426f575f80fd5b848152836020820152606060408201525f6134bb606083018486614a5d565b5f815180845260208085019450602084015f5b83811015614e4b57815187529582019590820190600101614e2f565b509495945050505050565b604081525f614e686040830185614e1c565b82810360208401526133aa8185614e1c565b5f60018201614e8b57614e8b614862565b5060010190565b80356002811061430e575f80fd5b60028110614ebc57634e487b7160e01b5f52602160045260245ffd5b9052565b5f8383855260208086019550808560051b830101845f5b87811015614b3d57848303601f19018952813536889003603e19018112614efc575f80fd5b87016040614f1285614f0d84614e92565b614ea0565b614f1e86830183614a1c565b92508187870152614f328287018483614a5d565b9b87019b955050509184019150600101614ed7565b5f8235607e19833603018112614f5b575f80fd5b90910192915050565b5f8383855260208086019550808560051b830101845f5b87811015614b3d57848303601f19018952614f968288614f47565b60808135855285820135868601526040614fb281840184614a1c565b8383890152614fc48489018284614a5d565b93505050506060614fd781840184614a1c565b935086830382880152614feb838583614a5d565b9c88019c96505050928501925050600101614f7b565b5f8282518085526020808601955060208260051b840101602086015f5b84811015614b3d57601f1986840301895261503a8383516141f2565b9884019892509083019060010161501e565b60608082528181018690525f906080808401600589811b860183018b865b8c81101561512057888303607f19018552615085828f614f47565b8035845260208082013581860152604080830135601e198436030181126150aa575f80fd5b830182810190356001600160401b038111156150c4575f80fd5b80891b36038213156150d4575f80fd5b8a838901526150e68b89018284614ec0565b925050506150f68a840184614a1c565b93508682038b88015261510a828583614a5d565b988301989650505092909201915060010161506a565b50508681036020880152615135818a8c614f64565b945050505050828103604084015261514d8185615001565b98975050505050505050565b606081525f61516b60608301866141f2565b60208301949094525060400152919050565b5f8235603e19833603018112614d6c575f80fd5b5f602082840312156151a1575f80fd5b61309782614e92565b818382375f9101908152919050565b838152606081016151cd6020830185614ea0565b826040830152949350505050565b81515f9082906020808601845b83811015615204578151855293820193908201906001016151e8565b50929695505050505050565b5f60208284031215615220575f80fd5b5051919050565b601f821115612e7257805f5260205f20601f840160051c8101602085101561524c5750805b601f840160051c820191505b81811015612be6575f8155600101615258565b81516001600160401b0381111561528457615284614528565b615298816152928454614d12565b84615227565b602080601f8311600181146152cb575f84156152b45750858301515b5f19600386901b1c1916600185901b178555615322565b5f85815260208120601f198616915b828110156152f9578886015182559484019460019091019084016152da565b508582101561531657878501515f19600388901b60f8161c191681555b505060018460011b0185555b505050505050565b5f8251614d6c8184602087016141d056fe360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbcf0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R0`\x80R4\x80\x15b\0\0\x14W_\x80\xFD[Pb\0\0\x1Fb\0\0%V[b\0\0\xD9V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80Th\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15b\0\0vW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80T`\x01`\x01`@\x1B\x03\x90\x81\x16\x14b\0\0\xD6W\x80T`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x81\x17\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PV[`\x80QaS|b\0\x01\0_9_\x81\x81a5\n\x01R\x81\x81a53\x01Ra7\x13\x01RaS|_\xF3\xFE`\x80`@R`\x046\x10a\x023W_5`\xE0\x1C\x80cw\xD3\x8E$\x11a\x01)W\x80c\xB4r+\xC4\x11a\0\xA8W\x80c\xC3\xAA\xAAZ\x11a\0mW\x80c\xC3\xAA\xAAZ\x14a\x06pW\x80c\xC9\x99\xA8\xB4\x14a\x06\x8FW\x80c\xCC\xEA\xC0\x19\x14a\x06\xAEW\x80c\xD9\xBE-\xE4\x14a\x06\xCDW\x80c\xF9\xC6p\xC3\x14a\x06\xECW_\x80\xFD[\x80c\xB4r+\xC4\x14a\x05\xEBW\x80c\xBCM\x07\xC2\x14a\x05\xFFW\x80c\xBF\x9B\x16\xC8\x14a\x06\x1EW\x80c\xC0\xAEd\xF7\x14a\x06=W\x80c\xC2\xB4)\x86\x14a\x06\\W_\x80\xFD[\x80c\x97l\x98\xB5\x11a\0\xEEW\x80c\x97l\x98\xB5\x14a\x05JW\x80c\x97o>\xB9\x14a\x05iW\x80c\xAD<\xB1\xCC\x14a\x05}W\x80c\xB0\xB4a\xC4\x14a\x05\xADW\x80c\xB1\x81\xCD\xA7\x14a\x05\xCCW_\x80\xFD[\x80cw\xD3\x8E$\x14a\x04\xBAW\x80c~\xAA\xC8\xF2\x14a\x04\xD9W\x80c\x8A\xEA\xC2)\x14a\x04\xEDW\x80c\x8E\x97\xCB`\x14a\x05\x0CW\x80c\x94G\xCF\xD4\x14a\x05+W_\x80\xFD[\x80c1\xFFA\xC8\x11a\x01\xB5W\x80cL\xB9P\xE1\x11a\x01zW\x80cL\xB9P\xE1\x14a\x04\x1FW\x80cO\x1E\xF2\x86\x14a\x04>W\x80cR\xD1\x90-\x14a\x04QW\x80c[\xFFv\xD9\x14a\x04eW\x80ce\xB3\x94\xAF\x14a\x04\x91W_\x80\xFD[\x80c1\xFFA\xC8\x14a\x03wW\x80c;V\x15\x9E\x14a\x03\xA3W\x80cA\xAD\x06\x9C\x14a\x03\xC2W\x80cF\xC5\xBB\xBD\x14a\x03\xE1W\x80cG\xE8\"\x95\x14a\x04\0W_\x80\xFD[\x80c \xA4\xEB9\x11a\x01\xFBW\x80c \xA4\xEB9\x14a\x02\xE4W\x80c\"\x1C\xDDN\x14a\x03\x03W\x80c&\xCF]\xEF\x14a\x03\"W\x80c(\x1E\x8B\xFE\x14a\x03DW\x80c*8\x89\x98\x14a\x03cW_\x80\xFD[\x80c\x06\x83M\x1D\x14a\x027W\x80c\r\x8En,\x14a\x02XW\x80c\x16\xD4\xEBo\x14a\x02\x82W\x80c\x1C\xE3\xF9\xBC\x14a\x02\xA1W\x80c =\x01\x14\x14a\x02\xB5W[_\x80\xFD[4\x80\x15a\x02BW_\x80\xFD[Pa\x02Va\x02Q6`\x04aA\xB0V[a\x07\x18V[\0[4\x80\x15a\x02cW_\x80\xFD[Pa\x02la\x08tV[`@Qa\x02y\x91\x90aB\x1DV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x8DW_\x80\xFD[Pa\x02Va\x02\x9C6`\x04aB\x8CV[a\x08\xE0V[4\x80\x15a\x02\xACW_\x80\xFD[Pa\x02Va\naV[4\x80\x15a\x02\xC0W_\x80\xFD[Pa\x02\xD4a\x02\xCF6`\x04aC\x13V[a\x0B~V[`@Q\x90\x15\x15\x81R` \x01a\x02yV[4\x80\x15a\x02\xEFW_\x80\xFD[Pa\x02Va\x02\xFE6`\x04aC.V[a\x0B\xBEV[4\x80\x15a\x03\x0EW_\x80\xFD[Pa\x02Va\x03\x1D6`\x04aC\x82V[a\x0C\xEEV[4\x80\x15a\x03-W_\x80\xFD[Pa\x036a\x0E\xE6V[`@Q\x90\x81R` \x01a\x02yV[4\x80\x15a\x03OW_\x80\xFD[Pa\x036a\x03^6`\x04aC.V[a\x0F\x0CV[4\x80\x15a\x03nW_\x80\xFD[Pa\x036a\x0F1V[4\x80\x15a\x03\x82W_\x80\xFD[Pa\x03\x96a\x03\x916`\x04aD(V[a\x0FWV[`@Qa\x02y\x91\x90aD\xA3V[4\x80\x15a\x03\xAEW_\x80\xFD[Pa\x02Va\x03\xBD6`\x04aC.V[a\x11\x1BV[4\x80\x15a\x03\xCDW_\x80\xFD[Pa\x036a\x03\xDC6`\x04aC.V[a\x12\x8CV[4\x80\x15a\x03\xECW_\x80\xFD[Pa\x02\xD4a\x03\xFB6`\x04aD(V[a\x12\xD1V[4\x80\x15a\x04\x0BW_\x80\xFD[Pa\x036a\x04\x1A6`\x04aC.V[a\x13/V[4\x80\x15a\x04*W_\x80\xFD[Pa\x02Va\x0496`\x04aD\xB5V[a\x13TV[a\x02Va\x04L6`\x04aF\0V[a\x1B<V[4\x80\x15a\x04\\W_\x80\xFD[Pa\x036a\x1B[V[4\x80\x15a\x04pW_\x80\xFD[Pa\x04\x84a\x04\x7F6`\x04aC.V[a\x1BvV[`@Qa\x02y\x91\x90aFLV[4\x80\x15a\x04\x9CW_\x80\xFD[Pa\x04\xA5a\x1B\xF9V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02yV[4\x80\x15a\x04\xC5W_\x80\xFD[Pa\x02Va\x04\xD46`\x04aA\xB0V[a\x1C\x19V[4\x80\x15a\x04\xE4W_\x80\xFD[Pa\x04\x84a\x1DVV[4\x80\x15a\x04\xF8W_\x80\xFD[Pa\x02Va\x05\x076`\x04aC\x82V[a\x1D\xD7V[4\x80\x15a\x05\x17W_\x80\xFD[Pa\x02Va\x05&6`\x04aA\xB0V[a\x1F\x9FV[4\x80\x15a\x056W_\x80\xFD[Pa\x02\xD4a\x05E6`\x04aD(V[a \xFDV[4\x80\x15a\x05UW_\x80\xFD[Pa\x02Va\x05d6`\x04aC\x82V[a!;V[4\x80\x15a\x05tW_\x80\xFD[Pa\x036a#6V[4\x80\x15a\x05\x88W_\x80\xFD[Pa\x02l`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[4\x80\x15a\x05\xB8W_\x80\xFD[Pa\x02Va\x05\xC76`\x04aA\xB0V[a#JV[4\x80\x15a\x05\xD7W_\x80\xFD[Pa\x02Va\x05\xE66`\x04aA\xB0V[a$\x8AV[4\x80\x15a\x05\xF6W_\x80\xFD[Pa\x036a%\xD2V[4\x80\x15a\x06\nW_\x80\xFD[Pa\x02Va\x06\x196`\x04aF\x98V[a%\xF8V[4\x80\x15a\x06)W_\x80\xFD[Pa\x02\xD4a\x0686`\x04aC.V[a'\\V[4\x80\x15a\x06HW_\x80\xFD[Pa\x02Va\x06W6`\x04aC.V[a'fV[4\x80\x15a\x06gW_\x80\xFD[Pa\x036a(\x9FV[4\x80\x15a\x06{W_\x80\xFD[Pa\x036a\x06\x8A6`\x04aC.V[a(\xC5V[4\x80\x15a\x06\x9AW_\x80\xFD[Pa\x04\xA5a\x06\xA96`\x04aC.V[a(\xEAV[4\x80\x15a\x06\xB9W_\x80\xFD[Pa\x02\xD4a\x06\xC86`\x04aA\xB0V[a)QV[4\x80\x15a\x06\xD8W_\x80\xFD[Pa\x02Va\x06\xE76`\x04aC.V[a)\xA6V[4\x80\x15a\x06\xF7W_\x80\xFD[Pa\x07\x0Ba\x07\x066`\x04aC.V[a+\xEDV[`@Qa\x02y\x91\x90aGTV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07hW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x8C\x91\x90aG\xB6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07\xC4W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[_a\x07\xCDa-\xB0V[\x90Pa\x07\xD8\x83a-\xD4V[a\x08!`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o8:\xB164\xB1\xA22\xB1\xB9<\xB8:4\xB7\xB7`\x81\x1B\x81RP\x83\x83`\x01\x01_\x87\x81R` \x01\x90\x81R` \x01_ \x80T\x90Pa.\0V[_\x83\x81R`\x06\x82\x01` R`@\x90\x81\x90 \x83\x90UQ\x83\x90\x7F\xD5q\xBF\x83>AU;\xBE&\x0E\0\xB3\xAFz\x0E\x91\xAA\xFDl\xDC#\x8A\x80:\xA9\xAC\x0Es\xEF\xEDe\x90a\x08g\x90\x85\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPV[```@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mProtocolConfig`\x90\x1B\x81RPa\x08\xA6_a.wV[a\x08\xB0`\x02a.wV[a\x08\xB9_a.wV[`@Q` \x01a\x08\xCC\x94\x93\x92\x91\x90aG\xD1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[_\x80Q` aS\\\x839\x81Q\x91RT`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x16`\x01\x14a\t!W`@QcoOs\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80Q` aS\\\x839\x81Q\x91R\x80T`\x03\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\tWWP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a\tuW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x17`\x01`@\x1B\x17\x81U`\xF8`\x07a\t\xA6\x91\x1B`\x01aHvV[\x87\x10\x15a\t\xC9W`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x88\x90R`$\x01a\x07\xBBV[a\t\xD8`\x01`\xFB\x1B`\x01aHvV[\x86\x10\x15a\t\xFBW`@Qc\xA2%em`\xE0\x1B\x81R`\x04\x81\x01\x87\x90R`$\x01a\x07\xBBV[a\n\x10\x87\x87a\n\n\x87\x89aH\x9AV[\x86a/\x06V[P\x80T`\xFF`@\x1B\x19\x16\x81U`@Q`\x01`\x01`@\x1B\x03\x83\x16\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xB1W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xD5\x91\x90aG\xB6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0B\x08W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\xBBV[_a\x0B\x11a-\xB0V[`\x0B\x81\x01T\x90\x91P_a\x0B#\x82a/ZV[\x90P\x80\x82\x7F\x15\xAA\xAFG^\xF4\x07T?Qd\xF5}\xCFW\xF7\xF98\x16\xF5[\xAEw\xCA\t\xEF\xC4E\xBA@\xEE\xF7\x84\x86`\r\x01T`\x01Ca\x0B[\x91\x90aJ\tV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01`@Q\x80\x91\x03\x90\xA3PPPV[_\x80a\x0B\x88a-\xB0V[`\x0B\x81\x01T_\x90\x81R`\x03\x90\x91\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x90\x96\x16\x83R\x94\x90R\x92\x90\x92 T`\xFF\x16\x92\x91PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x0EW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C2\x91\x90aG\xB6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0CeW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\xBBV[_a\x0Cna-\xB0V[\x90P`\x01_\x83\x81R`\x0E\x83\x01` R`@\x90 T`\xFF\x16`\x03\x81\x11\x15a\x0C\x96Wa\x0C\x96aHNV[\x14a\x0C\xB7W`@Qc5\x86\xEF\xA1`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07\xBBV[a\x0C\xC0\x82a/\xABV[`@Q\x82\x90\x7Fu\xE1\x15\xB7\xF7k\xF2\x1D\n.B\xDA\x93\x04\xD9\xC3W\xB5LH\x9EZ\xF5\x9E\xD3\xC7\x0B|\xD4\x835\xFC\x90_\x90\xA2PPV[_\x80Q` aS\\\x839\x81Q\x91RT`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x16`\x01\x14a\r/W`@QcoOs\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80Q` aS\\\x839\x81Q\x91R\x80T`\x03\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\reWP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a\r\x83W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x17`\x01`@\x1B\x17\x81U_a\r\xADa-\xB0V[\x90P_a\r\xE1a\r\xC2`\x07`\xF8\x1B`\x01aHvV[a\r\xD1`\x01`\xFB\x1B`\x01aHvV[a\r\xDB\x8D\x8FaH\x9AV[\x8Ca/\x06V[\x90P`@Q\x80`@\x01`@R\x80C\x81R` \x01\x8C\x8C\x8C\x8C\x8C\x8C\x8C`@Q` \x01a\x0E\x11\x97\x96\x95\x94\x93\x92\x91\x90aKJV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x90\x92R_\x84\x81R`\x17\x86\x01\x82R\x91\x90\x91 \x82Q\x81U\x91\x01Q`\x01\x90\x91\x01U`\xF8`\x07\x90\x1B\x81\x7F Mk\x80\x12\x11T\xCD\x87\xD9\x9C\xF5Lc\x9A=\xD0\xA5;0\x84'p\x98\xDE\x97.\xBD\xD3Lk\xE9\x8D\x8D\x8D\x8D\x8D\x8D\x8D`@Qa\x0E\x89\x97\x96\x95\x94\x93\x92\x91\x90aKJV[`@Q\x80\x91\x03\x90\xA3PP\x80T`\xFF`@\x1B\x19\x16\x81U`@Q`\x01`\x01`@\x1B\x03\x83\x16\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01[`@Q\x80\x91\x03\x90\xA1PPPPPPPPPV[_\x80a\x0E\xF0a-\xB0V[`\x0B\x81\x01T_\x90\x81R`\t\x90\x91\x01` R`@\x90 T\x92\x91PPV[_a\x0F\x16\x82a-\xD4V[a\x0F\x1Ea-\xB0V[_\x92\x83R`\x07\x01` RP`@\x90 T\x90V[_\x80a\x0F;a-\xB0V[`\x0B\x81\x01T_\x90\x81R`\x06\x90\x91\x01` R`@\x90 T\x92\x91PPV[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01R``\x91\x81\x01\x82\x90R\x81\x81\x01\x91\x90\x91Ra\x0F\x82\x83a0fV[a\x0F\xA2W`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x07\xBBV[a\x0F\xAAa-\xB0V[_\x84\x81R`\x04\x91\x90\x91\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x85R\x90\x83R\x92\x81\x90 \x81Q`\x80\x81\x01\x83R\x81T\x85\x16\x81R`\x01\x82\x01T\x90\x94\x16\x92\x84\x01\x92\x90\x92R`\x02\x82\x01\x80T\x91\x84\x01\x91a\x10\x02\x90aM\x12V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10.\x90aM\x12V[\x80\x15a\x10yW\x80`\x1F\x10a\x10PWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10yV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10\\W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\x10\x92\x90aM\x12V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10\xBE\x90aM\x12V[\x80\x15a\x11\tW\x80`\x1F\x10a\x10\xE0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x11\tV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10\xECW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P[\x92\x91PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11kW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x8F\x91\x90aG\xB6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x11\xC2W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\xBBV[_a\x11\xCBa-\xB0V[\x90P`\x01_\x83\x81R`\x0F\x83\x01` R`@\x90 T`\xFF\x16`\x02\x81\x11\x15a\x11\xF3Wa\x11\xF3aHNV[\x14a\x12\x14W`@Qc\xA2%em`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07\xBBV[_\x82\x81R`\x10\x82\x01` R`@\x90 T`\x0B\x82\x01T\x81\x14a\x12RW`@Qc\xA6\x9D}[`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x82\x90R`D\x01a\x07\xBBV[a\x12[\x83a0\x9EV[`@Q\x83\x90\x82\x90\x7Fd@\xAA\xEA{$\x80\xB8$I\xC3\x17\xAAZ\x91h\xDFw\xEBi0\x8F\xF8\xF7\xC3\x98\n\x1A\xD8H\xB7\xDF\x90_\x90\xA3PPPV[_a\x12\x96\x82a0fV[a\x12\xB6W`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07\xBBV[a\x12\xBEa-\xB0V[_\x92\x83R`\x08\x01` RP`@\x90 T\x90V[_a\x12\xDB\x83a0fV[a\x12\xFBW`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x07\xBBV[a\x13\x03a-\xB0V[_\x93\x84R`\x02\x01` \x90\x81R`@\x80\x85 `\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x85R\x92\x90RP\x90 T`\xFF\x16\x90V[_a\x139\x82a-\xD4V[a\x13Aa-\xB0V[_\x92\x83R`\t\x01` RP`@\x90 T\x90V[_a\x13]a-\xB0V[\x90P`\x01_\x87\x81R`\x0F\x83\x01` R`@\x90 T`\xFF\x16`\x02\x81\x11\x15a\x13\x85Wa\x13\x85aHNV[\x14a\x13\xA6W`@Qc\xA2%em`\xE0\x1B\x81R`\x04\x81\x01\x87\x90R`$\x01a\x07\xBBV[_\x86\x81R`\x10\x82\x01` \x90\x81R`@\x80\x83 T\x80\x84R`\x02\x85\x01\x83R\x81\x84 3\x85R\x90\x92R\x90\x91 T`\xFF\x16a\x13\xF8W`@Qc\xA3\xF4\xAF\xEB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x88\x90R`D\x01a\x07\xBBV[`\x01_\x82\x81R`\x0E\x84\x01` R`@\x90 T`\xFF\x16`\x03\x81\x11\x15a\x14\x1EWa\x14\x1EaHNV[\x03a\x14?W`@Qc\x19b\xDC\xFB`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\xBBV[a\x14H\x81a0fV[a\x14hW`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\xBBV[_\x81\x81R`\x04\x83\x01` \x90\x81R`@\x80\x83 3\x84R\x82R\x80\x83 `\x01\x01T\x81Q`\x01`\xF9\x1B\x93\x81\x01\x93\x90\x93R`!\x83\x01\x85\x90R`A\x80\x84\x01\x8C\x90R\x82Q\x80\x85\x03\x90\x91\x01\x81R`a\x90\x93\x01\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16\x91\x90\x81\x88`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xD9Wa\x14\xD9aE(V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x15\x02W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x89\x81\x10\x15a\x16\x90W_a\x15J\x8C\x8C\x84\x81\x81\x10a\x15%Wa\x15%aMDV[\x90P` \x02\x81\x01\x90a\x157\x91\x90aMXV[a\x15E\x90`@\x81\x01\x90aMvV[a0\xD1V[\x90P_a\x15\xA4\x8D\x8D\x85\x81\x81\x10a\x15bWa\x15baMDV[\x90P` \x02\x81\x01\x90a\x15t\x91\x90aMXV[5\x8E\x8E\x86\x81\x81\x10a\x15\x87Wa\x15\x87aMDV[\x90P` \x02\x81\x01\x90a\x15\x99\x91\x90aMXV[` \x015\x84\x88a27V[\x90Pa\x15\xE2\x87\x82\x8F\x8F\x87\x81\x81\x10a\x15\xBDWa\x15\xBDaMDV[\x90P` \x02\x81\x01\x90a\x15\xCF\x91\x90aMXV[a\x15\xDD\x90``\x81\x01\x90aM\xBBV[a3\xB3V[\x8C\x8C\x84\x81\x81\x10a\x15\xF4Wa\x15\xF4aMDV[\x90P` \x02\x81\x01\x90a\x16\x06\x91\x90aMXV[5\x8D\x8D\x85\x81\x81\x10a\x16\x19Wa\x16\x19aMDV[\x90P` \x02\x81\x01\x90a\x16+\x91\x90aMXV[` \x015\x83`@Q` \x01a\x16S\x93\x92\x91\x90\x92\x83R` \x83\x01\x91\x90\x91R`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84\x84\x81Q\x81\x10a\x16{Wa\x16{aMDV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x15\x07V[P_\x87`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xAAWa\x16\xAAaE(V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\xD3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x88\x81\x10\x15a\x18PW_a\x17i\x8B\x8B\x84\x81\x81\x10a\x16\xF6Wa\x16\xF6aMDV[\x90P` \x02\x81\x01\x90a\x17\x08\x91\x90aMXV[5\x8C\x8C\x85\x81\x81\x10a\x17\x1BWa\x17\x1BaMDV[\x90P` \x02\x81\x01\x90a\x17-\x91\x90aMXV[` \x015\x8D\x8D\x86\x81\x81\x10a\x17CWa\x17CaMDV[\x90P` \x02\x81\x01\x90a\x17U\x91\x90aMXV[a\x17c\x90`@\x81\x01\x90aM\xBBV[\x89a48V[\x90Pa\x17\x82\x87\x82\x8D\x8D\x86\x81\x81\x10a\x15\xBDWa\x15\xBDaMDV[\x8A\x8A\x83\x81\x81\x10a\x17\x94Wa\x17\x94aMDV[\x90P` \x02\x81\x01\x90a\x17\xA6\x91\x90aMXV[5\x8B\x8B\x84\x81\x81\x10a\x17\xB9Wa\x17\xB9aMDV[\x90P` \x02\x81\x01\x90a\x17\xCB\x91\x90aMXV[` \x015\x8C\x8C\x85\x81\x81\x10a\x17\xE1Wa\x17\xE1aMDV[\x90P` \x02\x81\x01\x90a\x17\xF3\x91\x90aMXV[a\x18\x01\x90`@\x81\x01\x90aM\xBBV[`@Q` \x01a\x18\x14\x94\x93\x92\x91\x90aM\xFDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83\x83\x81Q\x81\x10a\x18<Wa\x18<aMDV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x16\xD8V[P\x81\x81`@Q` \x01a\x18d\x92\x91\x90aNVV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 _\x8F\x81R`\x12\x8B\x01\x84R\x82\x81 `\x01`\x01`\xA0\x1B\x03\x8A\x16\x82R\x90\x93R\x91 T\x90\x94P`\xFF\x16\x15\x92Pa\x18\xD6\x91PPW`@Qc$\xA0\xBB\x1B`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x81\x01\x8A\x90R`D\x01a\x07\xBBV[_\x89\x81R`\x12\x85\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x82R\x80\x83 \x80T`\xFF\x19\x16`\x01\x17\x90U\x8B\x83R`\x13\x87\x01\x82R\x80\x83 \x84\x84R\x90\x91R\x81 \x80T\x82\x90a\x19%\x90aNzV[\x91\x90P\x81\x90U\x90P\x82`\x01`\x01`\xA0\x1B\x03\x16\x8A\x7F~\xDAo\x85\xE2;{\x91\xC0\x19\xB0W\r\x02\xB6c`n\xF9\xD7E\x94\xF7\xE0\x1F\xCF\xBD\xB0\xF4\xE9T\xD5\x84`@Qa\x19i\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3_\x84\x81R`\x05\x86\x01` R`@\x90 T\x81\x03a\x1B0W_\x84\x81R`\x0E\x86\x01` R`@\x90 \x80T`\xFF\x19\x16`\x03\x17\x90U`\x0B\x85\x01\x84\x90Ua\x19\xB2\x8A\x85a4\xC5V[_\x84\x81R`\x01\x86\x01` R`@\x81 \x80T\x90\x91\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xDDWa\x19\xDDaE(V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A\x10W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x19\xFBW\x90P[P\x90P_[\x82T\x81\x10\x15a\x1A\xEBW\x82\x81\x81T\x81\x10a\x1A0Wa\x1A0aMDV[\x90_R` _ \x90`\x04\x02\x01`\x03\x01\x80Ta\x1AJ\x90aM\x12V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1Av\x90aM\x12V[\x80\x15a\x1A\xC1W\x80`\x1F\x10a\x1A\x98Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1A\xC1V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1A\xA4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x82\x82\x81Q\x81\x10a\x1A\xD8Wa\x1A\xD8aMDV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1A\x15V[P\x8B\x86\x7F\x1AT{B\xE7,\xD3\xDD\xA0Nj\xDC\xCD\"\0'l\xFE\xF0\x1F\xE2\x13\x8D\x07\xF3\xA7D\x0FAm8\xBC\x8D\x8D\x8D\x8D\x87`@Qa\x1B%\x95\x94\x93\x92\x91\x90aPLV[`@Q\x80\x91\x03\x90\xA3PP[PPPPPPPPPPV[a\x1BDa4\xFFV[a\x1BM\x82a5\xA5V[a\x1BW\x82\x82a6LV[PPV[_a\x1Bda7\x08V[P_\x80Q` aS<\x839\x81Q\x91R\x90V[``a\x1B\x81\x82a-\xD4V[a\x1B\x89a-\xB0V[`\x05\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1B\xEDW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x1B\xCFW[PPPPP\x90P\x91\x90PV[_\x80_a\x1C\x04a-\xB0V[\x90P\x80`\x0B\x01T\x92P\x80`\r\x01T\x91PP\x90\x91V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CiW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x8D\x91\x90aG\xB6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1C\xC0W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\xBBV[_a\x1C\xC9a-\xB0V[\x90Pa\x1C\xD4\x83a-\xD4V[a\x1D\x10`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bmpc`\xE8\x1B\x81RP\x83\x83`\x01\x01_\x87\x81R` \x01\x90\x81R` \x01_ \x80T\x90Pa.\0V[_\x83\x81R`\t\x82\x01` R`@\x90\x81\x90 \x83\x90UQ\x83\x90\x7F\x14\x8F\x9Cl\xB7}\x120k\x9FYe4\xD1Kz\xAE>O\x98\xA2\xDB\xE3\xCD\xB0~\xA4\x92Lw_\x12\x90a\x08g\x90\x85\x81R` \x01\x90V[``_a\x1Daa-\xB0V[\x90P\x80`\x05\x01_\x82`\x0B\x01T\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1D\xCCW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x1D\xAEW[PPPPP\x91PP\x90V[_\x80Q` aS\\\x839\x81Q\x91R\x80T`\x03\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\x1E\rWP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a\x1E+W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x17`\x01`@\x1B\x17\x81U_a\x1EUa-\xB0V[`\x01`\xFB\x1B`\x0C\x82\x01\x90\x81U\x81T`\x0B\x83\x01\x81\x90U_\x81\x81R`\x0E\x84\x01` R`@\x81 \x80T`\xFF\x19\x16`\x03\x17\x90U\x82T\x93\x94P\x90\x92\x90\x91\x90\x82\x90a\x1E\x99\x90aNzV[\x91\x82\x90UP\x90Pa\x1E\xAA\x81\x83a4\xC5V[`@Q\x80`@\x01`@R\x80C\x81R` \x01\x8D\x8D\x8D\x8D\x8D\x8D\x8D`@Q` \x01a\x1E\xD8\x97\x96\x95\x94\x93\x92\x91\x90aKJV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x90\x92R_\x85\x81R`\x17\x87\x01\x82R\x91\x90\x91 \x82Q\x81U\x91\x01Q`\x01\x90\x91\x01U`\xF8`\x07\x90\x1B\x82\x7F Mk\x80\x12\x11T\xCD\x87\xD9\x9C\xF5Lc\x9A=\xD0\xA5;0\x84'p\x98\xDE\x97.\xBD\xD3Lk\xE9\x8E\x8E\x8E\x8E\x8E\x8E\x8E`@Qa\x1FP\x97\x96\x95\x94\x93\x92\x91\x90aKJV[`@Q\x80\x91\x03\x90\xA3PP\x81T`\xFF`@\x1B\x19\x16\x82UP`@Q`\x01`\x01`@\x1B\x03\x83\x16\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01a\x0E\xD3V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\xEFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \x13\x91\x90aG\xB6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a FW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\xBBV[_a Oa-\xB0V[\x90P\x80`\x0B\x01T\x83\x14\x15\x80a jWPa h\x83a0fV[\x15[\x15a \x8BW`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x07\xBBV[`\x0C\x81\x01T\x80\x83\x11a \xBAW`@Qc\xE8\x12\x1FQ`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x82\x90R`D\x01a\x07\xBBV[`\x0C\x82\x01\x83\x90Ua \xCB\x83\x85a4\xC5V[`@Q\x83\x90\x85\x90\x7F\n\x1C$\xC2\xBA^n\x1B\x1A\x85\x85y^[x\x1E7*\xEE\x1D\xB6\x86$}\xACut\xC1\x0F\xD75\xA6\x90_\x90\xA3PPPPV[_a!\x07\x83a-\xD4V[a!\x0Fa-\xB0V[_\x93\x84R`\x03\x01` \x90\x81R`@\x80\x85 `\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x85R\x92\x90RP\x90 T`\xFF\x16\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\x8BW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xAF\x91\x90aG\xB6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a!\xE2W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\xBBV[_a!\xEBa-\xB0V[`\x0B\x81\x01T\x90\x91P_a\"\x07a\"\x01\x8A\x8CaH\x9AV[\x89a7QV[_\x81\x81R`\x0E\x85\x01` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U\x90Pa\",\x81a/ZV[P_\x82\x81R`\t\x84\x01` \x90\x81R`@\x80\x83 T`\x01\x87\x01\x90\x92R\x82 Ta\"T\x91\x90aJ\tV[\x90P_\x81\x11a\"dW`\x01a\"fV[\x80[\x84`\x14\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP`@Q\x80`@\x01`@R\x80C\x81R` \x01\x8C\x8C\x8C\x8C\x8C\x8C\x8C`@Q` \x01a\"\xAB\x97\x96\x95\x94\x93\x92\x91\x90aKJV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x90\x92R_\x85\x81R`\x17\x88\x01\x82R\x82\x90 \x83Q\x81U\x92\x01Q`\x01\x90\x92\x01\x91\x90\x91UQ\x83\x90\x83\x90\x7F Mk\x80\x12\x11T\xCD\x87\xD9\x9C\xF5Lc\x9A=\xD0\xA5;0\x84'p\x98\xDE\x97.\xBD\xD3Lk\xE9\x90a#!\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90aKJV[`@Q\x80\x91\x03\x90\xA3PPPPPPPPPPPV[_\x80a#@a-\xB0V[`\x0B\x01T\x92\x91PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#\x9AW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\xBE\x91\x90aG\xB6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a#\xF1W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\xBBV[_a#\xFAa-\xB0V[\x90Pa$\x05\x83a-\xD4V[a$D`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e5\xB6\xB9\xA3\xB2\xB7`\xD1\x1B\x81RP\x83\x83`\x01\x01_\x87\x81R` \x01\x90\x81R` \x01_ \x80T\x90Pa.\0V[_\x83\x81R`\x08\x82\x01` R`@\x90\x81\x90 \x83\x90UQ\x83\x90\x7F\xF2\x1C\xB3{\xE7\t\x14\x8A\xAB\xEB\xD2xT>b\xD1\xB1\xE6\xA4G\x7F\xB1\xCCC\xE0i\xD3\xEE\xB8\xC8\x7F\x90\x90a\x08g\x90\x85\x81R` \x01\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\xDAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\xFE\x91\x90aG\xB6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a%1W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\xBBV[_a%:a-\xB0V[\x90Pa%E\x83a-\xD4V[a%\x8C`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m:\xB9\xB2\xB9\"2\xB1\xB9<\xB8:4\xB7\xB7`\x91\x1B\x81RP\x83\x83`\x01\x01_\x87\x81R` \x01\x90\x81R` \x01_ \x80T\x90Pa.\0V[_\x83\x81R`\x07\x82\x01` R`@\x90\x81\x90 \x83\x90UQ\x83\x90\x7F\x90\xF1\x91\x84\x93\x83\x1C\x1Ba3H\x97C\x103\x84\xC5`\x0E\xAEyn\xB3LQ\xEAO+\xAA\xFAO\x94\x90a\x08g\x90\x85\x81R` \x01\x90V[_\x80a%\xDCa-\xB0V[`\x0B\x81\x01T_\x90\x81R`\x08\x90\x91\x01` R`@\x90 T\x92\x91PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&HW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&l\x91\x90aG\xB6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a&\x9FW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\xBBV[_a&\xA8a-\xB0V[`\x0B\x81\x01T\x90\x91P\x80\x8B\x11a&\xDAW`@Qc\xEF\xD5_g`\xE0\x1B\x81R`\x04\x81\x01\x8C\x90R`$\x81\x01\x82\x90R`D\x01a\x07\xBBV[`\x0C\x82\x01T\x80\x8B\x11a'\tW`@Qc\xE8\x12\x1FQ`\xE0\x1B\x81R`\x04\x81\x01\x8C\x90R`$\x81\x01\x82\x90R`D\x01a\x07\xBBV[a'\x1E\x8C\x8Ca'\x18\x8C\x8EaH\x9AV[\x8Ba/\x06V[P\x8A\x8C\x7F*\xC6\x8Fx\xF4\xCC\xDEv\xB6I\x06\x02m\x01\xFF<B@>\xB7\xEE\xF8o\xE7\x88GJ#&}d\xCF\x8C\x8C\x8C\x8C\x8C\x8C\x8C`@Qa\x1B%\x97\x96\x95\x94\x93\x92\x91\x90aKJV[_a\x11\x15\x82a7vV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'\xB6W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\xDA\x91\x90aG\xB6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a(\rW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\xBBV[_a(\x16a-\xB0V[\x90P\x80`\x0B\x01T\x82\x03a(?W`@Qc\"\xCA\xFEq`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07\xBBV[a(H\x82a0fV[a(hW`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07\xBBV[a(q\x82a/\xABV[`@Q\x82\x90\x7F\xDA\x07]\t\x19\x8D ~:\x91\x8DK\x8D\xFC\x87\xDF-`\xA0\x0B\xE7\x03\xFD9\xEA\xAC\x90\x96-\xA0\xB7\xF0\x90_\x90\xA2PPV[_\x80a(\xA9a-\xB0V[`\x0B\x81\x01T_\x90\x81R`\x07\x90\x91\x01` R`@\x90 T\x92\x91PPV[_a(\xCF\x82a-\xD4V[a(\xD7a-\xB0V[_\x92\x83R`\x06\x01` RP`@\x90 T\x90V[_\x80a(\xF5\x83a7\xC0V[a)\x15W`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x07\xBBV[_a)\x1Ea-\xB0V[_\x94\x85R`\x17\x01` \x90\x81R`@\x94\x85\x90 \x85Q\x80\x87\x01\x90\x96R\x80T\x80\x87R`\x01\x90\x91\x01T\x95\x90\x91\x01\x85\x90R\x94\x92PPPV[_\x80a)[a-\xB0V[\x90P`\x02_\x84\x81R`\x0F\x83\x01` R`@\x90 T`\xFF\x16`\x02\x81\x11\x15a)\x83Wa)\x83aHNV[\x14\x80\x15a)\x9EWP_\x83\x81R`\x10\x82\x01` R`@\x90 T\x84\x14[\x94\x93PPPPV[_a)\xAFa-\xB0V[\x90P`\x01_\x83\x81R`\x0E\x83\x01` R`@\x90 T`\xFF\x16`\x03\x81\x11\x15a)\xD7Wa)\xD7aHNV[\x14a)\xF8W`@Qc5\x86\xEF\xA1`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07\xBBV[`\x0B\x81\x01T_\x81\x81R`\x02\x83\x01` \x81\x81R`@\x80\x84 3\x80\x86R\x90\x83R\x81\x85 T\x88\x86R\x93\x83R\x81\x85 \x90\x85R\x90\x91R\x90\x91 T`\xFF\x91\x82\x16\x91\x16\x81\x15\x80\x15a*@WP\x80\x15[\x15a*gW`@Qc\x17\x03\xBF\x1D`\xE3\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01a\x07\xBBV[_\x85\x81R`\x11\x85\x01` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T`\xFF\x16\x15a*\xACW`@Qc\x0CK\x0B\x99`\xE3\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01a\x07\xBBV[_\x85\x81R`\x11\x85\x01` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U\x81\x15a*\xF9W_\x85\x81R`\x16\x85\x01` R`@\x81 \x80T\x90\x91\x90a*\xF4\x90aNzV[\x90\x91UP[\x80\x15a+!W_\x85\x81R`\x15\x85\x01` R`@\x81 \x80T\x90\x91\x90a+\x1C\x90aNzV[\x90\x91UP[`@\x80Q\x83\x15\x15\x81R\x82\x15\x15` \x82\x01R3\x91\x87\x91\x7F\xB7\x9CH\x006\x95\xB6\xEB\xE5U\xAF\xA3o\xAD\x07\x1D\xEE\xEEu\xEB7\x18\xADc\xDEV!\xD3[\xA4KO\x91\x01`@Q\x80\x91\x03\x90\xA3a+j\x85a8\x07V[\x15a+\xE6W_\x85\x81R`\x0E\x85\x01` R`@\x90 \x80T`\xFF\x19\x16`\x02\x17\x90U`\x0C\x84\x01T`\r\x85\x01T\x81\x90\x87\x90\x7F\x15\xAA\xAFG^\xF4\x07T?Qd\xF5}\xCFW\xF7\xF98\x16\xF5[\xAEw\xCA\t\xEF\xC4E\xBA@\xEE\xF7\x90\x87\x90a+\xC6`\x01CaJ\tV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01`@Q\x80\x91\x03\x90\xA3P[PPPPPV[``a+\xF8\x82a-\xD4V[a,\0a-\xB0V[`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a-\xA5W_\x84\x81R` \x90\x81\x90 `@\x80Q`\x80\x81\x01\x82R`\x04\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x84R`\x01\x82\x01T\x16\x93\x83\x01\x93\x90\x93R`\x02\x83\x01\x80T\x92\x93\x92\x91\x84\x01\x91a,\x86\x90aM\x12V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta,\xB2\x90aM\x12V[\x80\x15a,\xFDW\x80`\x1F\x10a,\xD4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a,\xFDV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a,\xE0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta-\x16\x90aM\x12V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta-B\x90aM\x12V[\x80\x15a-\x8DW\x80`\x1F\x10a-dWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a-\x8DV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a-pW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a,1V[PPPP\x90P\x91\x90PV[\x7F\x80\xF3XZ\xF8h\x06\xC5wC\x03\xB0l\x1E\xE6@\xAA\x83\xB6\xEF>E\xDFI\xBB&\xC8RE\0\xC2\0\x90V[a-\xDD\x81a7vV[a-\xFDW`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\xBBV[PV[\x81_\x03a.\"W\x82`@Qc\x1B_\xDB\x07`\xE1\x1B\x81R`\x04\x01a\x07\xBB\x91\x90aB\x1DV[`\xFF\x82\x11\x15a.KW`@Qc\"\xBAR\xDB`\xE0\x1B\x81Ra\x07\xBB\x90\x84\x90\x84\x90`\xFF\x90`\x04\x01aQYV[\x80\x82\x11\x15a.rW\x82\x82\x82`@Qc\xCA\xA8\x14\xA3`\xE0\x1B\x81R`\x04\x01a\x07\xBB\x93\x92\x91\x90aQYV[PPPV[``_a.\x83\x83a8aV[`\x01\x01\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a.\xA1Wa.\xA1aE(V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a.\xCBW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[_\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a.\xD5WP\x93\x92PPPV[_\x80a/\x10a-\xB0V[\x90Pa/\x1D\x86\x85\x85a98V[_\x81\x81R`\x0E\x83\x01` R`@\x90 \x80T`\xFF\x19\x16`\x03\x17\x90U`\x0B\x82\x01\x81\x90U`\x0C\x82\x01\x86\x90U\x91Pa/Q\x85\x83a4\xC5V[P\x94\x93PPPPV[_\x80a/da-\xB0V[\x90P\x80`\x0C\x01_\x81Ta/v\x90aNzV[\x91\x82\x90UP_\x81\x81R`\x0F\x83\x01` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16`\x01\x17\x90U`\x10\x90\x94\x01\x90R\x91\x90\x91 \x92\x90\x92UP\x90V[_a/\xB4a-\xB0V[_\x83\x81R`\n\x82\x01` \x90\x81R`@\x80\x83 \x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x0E\x86\x01\x84R\x82\x85 \x80T\x90\x91\x16\x90U`\x0C\x85\x01T\x80\x85R`\x0F\x86\x01\x90\x93R\x92 T\x92\x93P\x91`\xFF\x16`\x02\x81\x11\x15a0\x0FWa0\x0FaHNV[\x14\x80\x15a0*WP_\x81\x81R`\x10\x83\x01` R`@\x90 T\x83\x14[\x15a08Wa08\x81a0\x9EV[P_\x91\x82R`\x14\x81\x01` \x90\x81R`@\x80\x84 \x84\x90U`\x15\x83\x01\x82R\x80\x84 \x84\x90U`\x16\x90\x92\x01\x90R\x81 UV[_\x80a0pa-\xB0V[\x90Pa0{\x83a7\xC0V[\x80\x15a0\x97WP_\x83\x81R`\n\x82\x01` R`@\x90 T`\xFF\x16\x15[\x93\x92PPPV[_a0\xA7a-\xB0V[_\x92\x83R`\x0F\x81\x01` \x90\x81R`@\x80\x85 \x80T`\xFF\x19\x16\x90U`\x10\x90\x92\x01\x90R\x82 \x91\x90\x91UPV[_\x80\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a0\xEBWa0\xEBaE(V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a1\x14W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15a2\x06W\x7F\xDD\xD1\x08w.j8\x99\xFE\xB0M\x14\x8A\xE9\x15\xCB\xE3\xEB^\xBD &\x88\x08\x03\x99\xE9\x92\x1A\xC3ak\x85\x85\x83\x81\x81\x10a1TWa1TaMDV[\x90P` \x02\x81\x01\x90a1f\x91\x90aQ}V[a1t\x90` \x81\x01\x90aQ\x91V[\x86\x86\x84\x81\x81\x10a1\x86Wa1\x86aMDV[\x90P` \x02\x81\x01\x90a1\x98\x91\x90aQ}V[a1\xA6\x90` \x81\x01\x90aM\xBBV[`@Qa1\xB4\x92\x91\x90aQ\xAAV[`@Q\x90\x81\x90\x03\x81 a1\xCB\x93\x92\x91` \x01aQ\xB9V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x82\x81Q\x81\x10a1\xF3Wa1\xF3aMDV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a1\x19V[P\x80`@Q` \x01a2\x18\x91\x90aQ\xDBV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x92\x91PPV[\x80Q` \x80\x83\x01\x91\x90\x91 `@\x80Q\x7F\xBD\x14\x83[\xB4\xAE\x13\xC7\x8E\xCB\x88\xDE\xD2\xC37\x03%\xF3\x9E`\x06\xEB\x94\xFFE\xE9_\x98\xE4\xC8Z*\x93\x81\x01\x93\x90\x93R\x82\x01\x86\x90R``\x82\x01\x85\x90R`\x80\x82\x01\x84\x90R`\xA0\x82\x01R_\x90a3\xAA\x90`\xC0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@\x80Q\x80\x82\x01\x82R`\x0E\x81RmProtocolConfig`\x90\x1B` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\x01\x81R`1`\xF8\x1B\x90\x82\x01R\x81Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81\x83\x01R\x7F\xA3\xDE\x18\x80\xCF\x08>\x83\x18\xB7zye\xD0-\xD9v^\x85\xA4\x8EA\x8ADc\xAFz\rW\xB4\xB3\xEE\x81\x84\x01R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xC0\x82\x01\x84R\x80Q\x90\x83\x01 a\x19\x01`\xF0\x1B`\xE0\x83\x01R`\xE2\x82\x01Ra\x01\x02\x80\x82\x01\x94\x90\x94R\x82Q\x80\x82\x03\x90\x94\x01\x84Ra\x01\"\x01\x90\x91R\x81Q\x91\x01 \x90V[\x95\x94PPPPPV[_a3\xF3\x84\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa;W\x92PPPV[\x90P\x84`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a+\xE6W`@Qcx\xB9\xAD\xA3`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R3`$\x82\x01R`D\x01a\x07\xBBV[_a4\xBB\x7F\xA2d\xB3\x18\xE9P\x800\n?\x06\xA6ej\x8E\x7F\xE2O\x99\x03\xF0\xE6\xBC\xCA0~\xFB\xE3\x9CLN\t\x87\x87\x87\x87`@Q` \x01a4r\x92\x91\x90aQ\xAAV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x89Q\x8A\x83\x01 \x91\x84\x01\x96\x90\x96R\x90\x82\x01\x93\x90\x93R``\x81\x01\x91\x90\x91R`\x80\x81\x01\x92\x90\x92R`\xA0\x82\x01R`\xC0\x01a2\x90V[\x96\x95PPPPPPV[_a4\xCEa-\xB0V[_\x84\x81R`\x0F\x82\x01` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16`\x02\x17\x90U`\x10\x84\x01\x90\x91R\x90 \x92\x90\x92UP`\r\x01UV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a5\x85WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a5y_\x80Q` aS<\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a5\xA3W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5\xF5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6\x19\x91\x90aG\xB6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a-\xFDW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\xBBV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a6\xA6WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra6\xA3\x91\x81\x01\x90aR\x10V[`\x01[a6\xCEW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x07\xBBV[_\x80Q` aS<\x839\x81Q\x91R\x81\x14a6\xFEW`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\xBBV[a.r\x83\x83a;\x7FV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a5\xA3W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80a7[a-\xB0V[\x80T\x90\x91Pa)\x9E\x90a7o\x90`\x01aHvV[\x85\x85a98V[_\x80a7\x80a-\xB0V[\x90Pa7\x8B\x83a0fV[\x80\x15a0\x97WP`\x03_\x84\x81R`\x0E\x83\x01` R`@\x90 T`\xFF\x16`\x03\x81\x11\x15a7\xB8Wa7\xB8aHNV[\x14\x93\x92PPPV[_\x80a7\xCAa-\xB0V[\x90Pa7\xDB`\x07`\xF8\x1B`\x01aHvV[\x83\x10\x15\x80\x15a7\xEBWP\x80T\x83\x11\x15[\x80\x15a0\x97WP_\x92\x83R`\x01\x01` RP`@\x90 T\x15\x15\x90V[_\x80a8\x11a-\xB0V[_\x84\x81R`\x01\x82\x01` \x90\x81R`@\x80\x83 T`\x15\x85\x01\x90\x92R\x90\x91 T\x91\x92P\x14\x80\x15a0\x97WP_\x83\x81R`\x14\x82\x01` \x90\x81R`@\x80\x83 T`\x16\x85\x01\x90\x92R\x90\x91 T\x10\x15\x93\x92PPPV[_\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a8\x9FWr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a8\xCBWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a8\xE9Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a9\x01Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a9\x15Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a9'W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x11\x15W`\x01\x01\x92\x91PPV[_\x82Q_\x03a9YW`@Qb\x1A25`\xE6\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82Q`\xFF\x10\x15a9\x89W\x82Q`@Qc\x02\xD4\xE4\xEF`\xE3\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\xFF`$\x82\x01R`D\x01a\x07\xBBV[a9\xC0`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o8:\xB164\xB1\xA22\xB1\xB9<\xB8:4\xB7\xB7`\x81\x1B\x81RP\x83_\x015\x85Qa.\0V[a9\xF6`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m:\xB9\xB2\xB9\"2\xB1\xB9<\xB8:4\xB7\xB7`\x91\x1B\x81RP\x83` \x015\x85Qa.\0V[a:$`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e5\xB6\xB9\xA3\xB2\xB7`\xD1\x1B\x81RP\x83`@\x015\x85Qa.\0V[a:O`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bmpc`\xE8\x1B\x81RP\x83``\x015\x85Qa.\0V[_a:Xa-\xB0V[\x80T\x90\x91P\x85\x11a:\x89W\x80T`@Qc\xEF\xD5_g`\xE0\x1B\x81Ra\x07\xBB\x91\x87\x91`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[\x84\x81U\x84\x91P_[\x84Q\x81\x10\x15a;\x0BW_\x85\x82\x81Q\x81\x10a:\xADWa:\xADaMDV[` \x02` \x01\x01Q\x90Pa;\x02\x84`@Q\x80`\x80\x01`@R\x80\x84_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84`@\x01Q\x81R` \x01\x84``\x01Q\x81RPa;\xD4V[P`\x01\x01a:\x91V[P_\x82\x81R`\x06\x82\x01` \x90\x81R`@\x80\x83 \x865\x90U`\x07\x84\x01\x82R\x80\x83 \x82\x87\x015\x90U`\x08\x84\x01\x82R\x80\x83 \x81\x87\x015\x90U`\t\x90\x93\x01\x90R ``\x90\x92\x015\x90\x91U\x92\x91PPV[_\x80_\x80a;e\x86\x86a>wV[\x92P\x92P\x92Pa;u\x82\x82a>\xC0V[P\x90\x94\x93PPPPV[a;\x88\x82a?xV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a;\xCCWa.r\x82\x82a?\xDBV[a\x1BWa@DV[_a;\xDDa-\xB0V[\x82Q\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16a<\x08W`@QcB3@%`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16a<3W`@Qc-\xEC\xCFM`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x83\x81R`\x02\x82\x01` \x90\x81R`@\x80\x83 \x85Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15a<\x87W\x81Q`@Qc\r\x18\xC4\xFF`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01a\x07\xBBV[_\x83\x81R`\x03\x82\x01` \x90\x81R`@\x80\x83 \x85\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15a<\xE0W` \x82\x01Q`@Qc\xF5\x1A\xF6\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01a\x07\xBBV[_\x83\x81R`\x01\x82\x81\x01` \x90\x81R`@\x80\x84 \x80T\x80\x85\x01\x82U\x90\x85R\x93\x82\x90 \x86Q`\x04\x90\x95\x02\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x17\x82U\x92\x87\x01Q\x93\x81\x01\x80T\x90\x93\x16\x93\x90\x94\x16\x92\x90\x92\x17\x90U\x83\x01Q\x83\x91\x90`\x02\x82\x01\x90a=P\x90\x82aRkV[P``\x82\x01Q`\x03\x82\x01\x90a=e\x90\x82aRkV[PPP_\x83\x81R`\x02\x80\x83\x01` \x90\x81R`@\x80\x84 \x86Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x86R\x90\x83R\x81\x85 \x80T`\xFF\x19\x90\x81\x16`\x01\x90\x81\x17\x90\x92U\x89\x87R`\x03\x88\x01\x85R\x83\x87 \x89\x86\x01\x80Q\x85\x16\x89R\x90\x86R\x84\x88 \x80T\x90\x92\x16\x83\x17\x90\x91U\x89\x87R`\x04\x88\x01\x85R\x83\x87 \x89Q\x84\x16\x88R\x90\x94R\x94\x82\x90 \x87Q\x81T\x90\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x82U\x93Q\x95\x81\x01\x80T\x96\x90\x92\x16\x95\x90\x93\x16\x94\x90\x94\x17\x90\x93U\x91\x84\x01Q\x84\x92\x91\x82\x01\x90a>\x1E\x90\x82aRkV[P``\x82\x01Q`\x03\x82\x01\x90a>3\x90\x82aRkV[PPP_\x92\x83R`\x05\x01` \x90\x81R`@\x83 \x91\x81\x01Q\x82T`\x01\x81\x01\x84U\x92\x84R\x92 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[_\x80_\x83Q`A\x03a>\xAEW` \x84\x01Q`@\x85\x01Q``\x86\x01Q_\x1Aa>\xA0\x88\x82\x85\x85a@cV[\x95P\x95P\x95PPPPa>\xB9V[PP\x81Q_\x91P`\x02\x90[\x92P\x92P\x92V[_\x82`\x03\x81\x11\x15a>\xD3Wa>\xD3aHNV[\x03a>\xDCWPPV[`\x01\x82`\x03\x81\x11\x15a>\xF0Wa>\xF0aHNV[\x03a?\x0EW`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a?\"Wa?\"aHNV[\x03a?CW`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\xBBV[`\x03\x82`\x03\x81\x11\x15a?WWa?WaHNV[\x03a\x1BWW`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\xBBV[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a?\xADW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x07\xBBV[_\x80Q` aS<\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``_\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa?\xF7\x91\x90aS*V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a@/W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a@4V[``\x91P[P\x91P\x91Pa3\xAA\x85\x83\x83aA+V[4\x15a5\xA3W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a@\x9CWP_\x91P`\x03\x90P\x82aA!V[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a@\xEDW=_\x80>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aA\x18WP_\x92P`\x01\x91P\x82\x90PaA!V[\x92P_\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[``\x82aA@WaA;\x82aA\x87V[a0\x97V[\x81Q\x15\x80\x15aAWWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15aA\x80W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x07\xBBV[P\x92\x91PPV[\x80Q\x15aA\x97W\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80`@\x83\x85\x03\x12\x15aA\xC1W_\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[_[\x83\x81\x10\x15aA\xEAW\x81\x81\x01Q\x83\x82\x01R` \x01aA\xD2V[PP_\x91\x01RV[_\x81Q\x80\x84RaB\t\x81` \x86\x01` \x86\x01aA\xD0V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R_a0\x97` \x83\x01\x84aA\xF2V[_\x80\x83`\x1F\x84\x01\x12aB?W_\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aBUW_\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aBoW_\x80\xFD[\x92P\x92\x90PV[_`\x80\x82\x84\x03\x12\x15aB\x86W_\x80\xFD[P\x91\x90PV[_\x80_\x80_`\xE0\x86\x88\x03\x12\x15aB\xA0W_\x80\xFD[\x855\x94P` \x86\x015\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aB\xC3W_\x80\xFD[aB\xCF\x88\x82\x89\x01aB/V[\x90\x94P\x92PaB\xE3\x90P\x87``\x88\x01aBvV[\x90P\x92\x95P\x92\x95\x90\x93PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a-\xFDW_\x80\xFD[\x805aC\x0E\x81aB\xEFV[\x91\x90PV[_` \x82\x84\x03\x12\x15aC#W_\x80\xFD[\x815a0\x97\x81aB\xEFV[_` \x82\x84\x03\x12\x15aC>W_\x80\xFD[P5\x91\x90PV[_\x80\x83`\x1F\x84\x01\x12aCUW_\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aCkW_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aBoW_\x80\xFD[_\x80_\x80_\x80_`\xE0\x88\x8A\x03\x12\x15aC\x98W_\x80\xFD[\x875`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aC\xAEW_\x80\xFD[aC\xBA\x8B\x83\x8C\x01aB/V[\x90\x99P\x97P\x87\x91PaC\xCF\x8B` \x8C\x01aBvV[\x96P`\xA0\x8A\x015\x91P\x80\x82\x11\x15aC\xE4W_\x80\xFD[aC\xF0\x8B\x83\x8C\x01aCEV[\x90\x96P\x94P`\xC0\x8A\x015\x91P\x80\x82\x11\x15aD\x08W_\x80\xFD[PaD\x15\x8A\x82\x8B\x01aB/V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[_\x80`@\x83\x85\x03\x12\x15aD9W_\x80\xFD[\x825\x91P` \x83\x015aDK\x81aB\xEFV[\x80\x91PP\x92P\x92\x90PV[_`\x01\x80`\xA0\x1B\x03\x80\x83Q\x16\x84R\x80` \x84\x01Q\x16` \x85\x01RP`@\x82\x01Q`\x80`@\x85\x01RaD\x8A`\x80\x85\x01\x82aA\xF2V[\x90P``\x83\x01Q\x84\x82\x03``\x86\x01Ra3\xAA\x82\x82aA\xF2V[` \x81R_a0\x97` \x83\x01\x84aDVV[_\x80_\x80_``\x86\x88\x03\x12\x15aD\xC9W_\x80\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aD\xE6W_\x80\xFD[aD\xF2\x89\x83\x8A\x01aB/V[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15aE\nW_\x80\xFD[PaE\x17\x88\x82\x89\x01aB/V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Qa\x01\0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aE_WaE_aE(V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aE\x8DWaE\x8DaE(V[`@R\x91\x90PV[_\x82`\x1F\x83\x01\x12aE\xA4W_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aE\xBDWaE\xBDaE(V[aE\xD0`\x1F\x82\x01`\x1F\x19\x16` \x01aEeV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aE\xE4W_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15aF\x11W_\x80\xFD[\x825aF\x1C\x81aB\xEFV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aF6W_\x80\xFD[aFB\x85\x82\x86\x01aE\x95V[\x91PP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aF\x8CW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aFgV[P\x90\x96\x95PPPPPPV[_\x80_\x80_\x80_\x80_a\x01 \x8A\x8C\x03\x12\x15aF\xB1W_\x80\xFD[\x895\x98P` \x8A\x015\x97P`@\x8A\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aF\xD5W_\x80\xFD[aF\xE1\x8D\x83\x8E\x01aB/V[\x90\x99P\x97P\x87\x91PaF\xF6\x8D``\x8E\x01aBvV[\x96P`\xE0\x8C\x015\x91P\x80\x82\x11\x15aG\x0BW_\x80\xFD[aG\x17\x8D\x83\x8E\x01aCEV[\x90\x96P\x94Pa\x01\0\x8C\x015\x91P\x80\x82\x11\x15aG0W_\x80\xFD[PaG=\x8C\x82\x8D\x01aB/V[\x91P\x80\x93PP\x80\x91PP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[_` \x80\x83\x01` \x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P` \x87\x01_[\x82\x81\x10\x15aG\xA9W`?\x19\x88\x86\x03\x01\x84RaG\x97\x85\x83QaDVV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01aG{V[P\x92\x97\x96PPPPPPPV[_` \x82\x84\x03\x12\x15aG\xC6W_\x80\xFD[\x81Qa0\x97\x81aB\xEFV[_\x85QaG\xE2\x81\x84` \x8A\x01aA\xD0V[a\x10;`\xF1\x1B\x90\x83\x01\x90\x81R\x85QaH\x01\x81`\x02\x84\x01` \x8A\x01aA\xD0V[\x80\x82\x01\x91PP`\x17`\xF9\x1B\x80`\x02\x83\x01R\x85QaH%\x81`\x03\x85\x01` \x8A\x01aA\xD0V[`\x03\x92\x01\x91\x82\x01R\x83QaH@\x81`\x04\x84\x01` \x88\x01aA\xD0V[\x01`\x04\x01\x96\x95PPPPPPV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x11\x15Wa\x11\x15aHbV[\x805`\x03\x81\x90\x0B\x81\x14aC\x0EW_\x80\xFD[_`\x01`\x01`@\x1B\x03\x80\x84\x11\x15aH\xB3WaH\xB3aE(V[\x83`\x05\x1B` aH\xC4\x81\x83\x01aEeV[\x86\x81R\x91\x85\x01\x91\x81\x81\x01\x906\x84\x11\x15aH\xDBW_\x80\xFD[\x86[\x84\x81\x10\x15aI\xFDW\x805\x86\x81\x11\x15aH\xF3W_\x80\xFD[\x88\x01a\x01\x006\x82\x90\x03\x12\x15aI\x06W_\x80\xFD[aI\x0EaE<V[aI\x17\x82aC\x03V[\x81RaI$\x86\x83\x01aC\x03V[\x86\x82\x01R`@\x80\x83\x015\x89\x81\x11\x15aI:W_\x80\xFD[aIF6\x82\x86\x01aE\x95V[\x82\x84\x01RPP``\x80\x83\x015\x89\x81\x11\x15aI^W_\x80\xFD[aIj6\x82\x86\x01aE\x95V[\x82\x84\x01RPP`\x80aI}\x81\x84\x01aH\x89V[\x90\x82\x01R`\xA0\x82\x81\x015\x89\x81\x11\x15aI\x93W_\x80\xFD[aI\x9F6\x82\x86\x01aE\x95V[\x82\x84\x01RPP`\xC0\x80\x83\x015\x89\x81\x11\x15aI\xB7W_\x80\xFD[aI\xC36\x82\x86\x01aE\x95V[\x82\x84\x01RPP`\xE0\x80\x83\x015\x89\x81\x11\x15aI\xDBW_\x80\xFD[aI\xE76\x82\x86\x01aE\x95V[\x91\x83\x01\x91\x90\x91RP\x84RP\x91\x83\x01\x91\x83\x01aH\xDDV[P\x97\x96PPPPPPPV[\x81\x81\x03\x81\x81\x11\x15a\x11\x15Wa\x11\x15aHbV[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aJ1W_\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aJOW_\x80\xFD[\x806\x03\x82\x13\x15aBoW_\x80\xFD[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[_\x83\x83\x85R` \x80\x86\x01\x95P\x80\x85`\x05\x1B\x83\x01\x01\x84_[\x87\x81\x10\x15aK=W\x84\x83\x03`\x1F\x19\x01\x89R\x8156\x88\x90\x03`^\x19\x01\x81\x12aJ\xC1W_\x80\xFD[\x87\x01``aJ\xCF\x82\x80aJ\x1CV[\x82\x87RaJ\xDF\x83\x88\x01\x82\x84aJ]V[\x92PPPaJ\xEF\x86\x83\x01\x83aJ\x1CV[\x86\x83\x03\x88\x88\x01RaK\x01\x83\x82\x84aJ]V[\x92PPP`@aK\x13\x81\x84\x01\x84aJ\x1CV[\x93P\x86\x83\x03\x82\x88\x01RaK'\x83\x85\x83aJ]V[\x9C\x88\x01\x9C\x96PPP\x92\x85\x01\x92PP`\x01\x01aJ\x9CV[P\x90\x97\x96PPPPPPPV[`\xE0\x80\x82R\x81\x01\x87\x90R_a\x01\0\x80\x83\x01`\x05\x8A\x90\x1B\x84\x01\x82\x01\x8B\x84[\x8C\x81\x10\x15aL\xAAW\x86\x83\x03`\xFF\x19\x01\x84R\x8156\x8F\x90\x03`\xFE\x19\x01\x81\x12aK\x8CW_\x80\xFD[\x8E\x01aK\xA8\x84aK\x9B\x83aC\x03V[`\x01`\x01`\xA0\x1B\x03\x16\x90RV[` aK\xB5\x81\x83\x01aC\x03V[`\x01`\x01`\xA0\x1B\x03\x16\x81\x86\x01R`@aK\xD0\x83\x82\x01\x84aJ\x1CV[\x89\x83\x89\x01RaK\xE2\x8A\x89\x01\x82\x84aJ]V[\x92PPP``aK\xF4\x81\x85\x01\x85aJ\x1CV[\x88\x84\x03\x83\x8A\x01RaL\x06\x84\x82\x84aJ]V[\x93PPPP`\x80aL\x18\x81\x85\x01aH\x89V[aL&\x82\x89\x01\x82`\x03\x0B\x90RV[PP`\xA0aL6\x81\x85\x01\x85aJ\x1CV[\x88\x84\x03\x83\x8A\x01RaLH\x84\x82\x84aJ]V[\x93PPPP`\xC0aL[\x81\x85\x01\x85aJ\x1CV[\x88\x84\x03\x83\x8A\x01RaLm\x84\x82\x84aJ]V[\x93PPPPaL\x7F`\xE0\x84\x01\x84aJ\x1CV[\x93P\x86\x82\x03`\xE0\x88\x01RaL\x94\x82\x85\x83aJ]V[\x97\x83\x01\x97\x96PPP\x92\x90\x92\x01\x91P`\x01\x01aKgV[PPaL\xDA` \x86\x01\x8B\x805\x82R` \x81\x015` \x83\x01R`@\x81\x015`@\x83\x01R``\x81\x015``\x83\x01RPPV[\x84\x81\x03`\xA0\x86\x01RaL\xED\x81\x89\x8BaJ]V[\x92PPP\x82\x81\x03`\xC0\x84\x01RaM\x04\x81\x85\x87aJ\x85V[\x9A\x99PPPPPPPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80aM&W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aB\x86WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`~\x19\x836\x03\x01\x81\x12aMlW_\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aM\x8BW_\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aM\xA4W_\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aBoW_\x80\xFD[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aM\xD0W_\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aM\xE9W_\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aBoW_\x80\xFD[\x84\x81R\x83` \x82\x01R```@\x82\x01R_a4\xBB``\x83\x01\x84\x86aJ]V[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15aNKW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aN/V[P\x94\x95\x94PPPPPV[`@\x81R_aNh`@\x83\x01\x85aN\x1CV[\x82\x81\x03` \x84\x01Ra3\xAA\x81\x85aN\x1CV[_`\x01\x82\x01aN\x8BWaN\x8BaHbV[P`\x01\x01\x90V[\x805`\x02\x81\x10aC\x0EW_\x80\xFD[`\x02\x81\x10aN\xBCWcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90RV[_\x83\x83\x85R` \x80\x86\x01\x95P\x80\x85`\x05\x1B\x83\x01\x01\x84_[\x87\x81\x10\x15aK=W\x84\x83\x03`\x1F\x19\x01\x89R\x8156\x88\x90\x03`>\x19\x01\x81\x12aN\xFCW_\x80\xFD[\x87\x01`@aO\x12\x85aO\r\x84aN\x92V[aN\xA0V[aO\x1E\x86\x83\x01\x83aJ\x1CV[\x92P\x81\x87\x87\x01RaO2\x82\x87\x01\x84\x83aJ]V[\x9B\x87\x01\x9B\x95PPP\x91\x84\x01\x91P`\x01\x01aN\xD7V[_\x825`~\x19\x836\x03\x01\x81\x12aO[W_\x80\xFD[\x90\x91\x01\x92\x91PPV[_\x83\x83\x85R` \x80\x86\x01\x95P\x80\x85`\x05\x1B\x83\x01\x01\x84_[\x87\x81\x10\x15aK=W\x84\x83\x03`\x1F\x19\x01\x89RaO\x96\x82\x88aOGV[`\x80\x815\x85R\x85\x82\x015\x86\x86\x01R`@aO\xB2\x81\x84\x01\x84aJ\x1CV[\x83\x83\x89\x01RaO\xC4\x84\x89\x01\x82\x84aJ]V[\x93PPPP``aO\xD7\x81\x84\x01\x84aJ\x1CV[\x93P\x86\x83\x03\x82\x88\x01RaO\xEB\x83\x85\x83aJ]V[\x9C\x88\x01\x9C\x96PPP\x92\x85\x01\x92PP`\x01\x01aO{V[_\x82\x82Q\x80\x85R` \x80\x86\x01\x95P` \x82`\x05\x1B\x84\x01\x01` \x86\x01_[\x84\x81\x10\x15aK=W`\x1F\x19\x86\x84\x03\x01\x89RaP:\x83\x83QaA\xF2V[\x98\x84\x01\x98\x92P\x90\x83\x01\x90`\x01\x01aP\x1EV[``\x80\x82R\x81\x81\x01\x86\x90R_\x90`\x80\x80\x84\x01`\x05\x89\x81\x1B\x86\x01\x83\x01\x8B\x86[\x8C\x81\x10\x15aQ W\x88\x83\x03`\x7F\x19\x01\x85RaP\x85\x82\x8FaOGV[\x805\x84R` \x80\x82\x015\x81\x86\x01R`@\x80\x83\x015`\x1E\x19\x846\x03\x01\x81\x12aP\xAAW_\x80\xFD[\x83\x01\x82\x81\x01\x905`\x01`\x01`@\x1B\x03\x81\x11\x15aP\xC4W_\x80\xFD[\x80\x89\x1B6\x03\x82\x13\x15aP\xD4W_\x80\xFD[\x8A\x83\x89\x01RaP\xE6\x8B\x89\x01\x82\x84aN\xC0V[\x92PPPaP\xF6\x8A\x84\x01\x84aJ\x1CV[\x93P\x86\x82\x03\x8B\x88\x01RaQ\n\x82\x85\x83aJ]V[\x98\x83\x01\x98\x96PPP\x92\x90\x92\x01\x91P`\x01\x01aPjV[PP\x86\x81\x03` \x88\x01RaQ5\x81\x8A\x8CaOdV[\x94PPPPP\x82\x81\x03`@\x84\x01RaQM\x81\x85aP\x01V[\x98\x97PPPPPPPPV[``\x81R_aQk``\x83\x01\x86aA\xF2V[` \x83\x01\x94\x90\x94RP`@\x01R\x91\x90PV[_\x825`>\x19\x836\x03\x01\x81\x12aMlW_\x80\xFD[_` \x82\x84\x03\x12\x15aQ\xA1W_\x80\xFD[a0\x97\x82aN\x92V[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[\x83\x81R``\x81\x01aQ\xCD` \x83\x01\x85aN\xA0V[\x82`@\x83\x01R\x94\x93PPPPV[\x81Q_\x90\x82\x90` \x80\x86\x01\x84[\x83\x81\x10\x15aR\x04W\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01aQ\xE8V[P\x92\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15aR W_\x80\xFD[PQ\x91\x90PV[`\x1F\x82\x11\x15a.rW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15aRLWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a+\xE6W_\x81U`\x01\x01aRXV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aR\x84WaR\x84aE(V[aR\x98\x81aR\x92\x84TaM\x12V[\x84aR'V[` \x80`\x1F\x83\x11`\x01\x81\x14aR\xCBW_\x84\x15aR\xB4WP\x85\x83\x01Q[_\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85UaS\"V[_\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15aR\xF9W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01aR\xDAV[P\x85\x82\x10\x15aS\x16W\x87\x85\x01Q_\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PP`\x01\x84`\x01\x1B\x01\x85U[PPPPPPV[_\x82QaMl\x81\x84` \x87\x01aA\xD0V\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405260043610610233575f3560e01c806377d38e2411610129578063b4722bc4116100a8578063c3aaaa5a1161006d578063c3aaaa5a14610670578063c999a8b41461068f578063cceac019146106ae578063d9be2de4146106cd578063f9c670c3146106ec575f80fd5b8063b4722bc4146105eb578063bc4d07c2146105ff578063bf9b16c81461061e578063c0ae64f71461063d578063c2b429861461065c575f80fd5b8063976c98b5116100ee578063976c98b51461054a578063976f3eb914610569578063ad3cb1cc1461057d578063b0b461c4146105ad578063b181cda7146105cc575f80fd5b806377d38e24146104ba5780637eaac8f2146104d95780638aeac229146104ed5780638e97cb601461050c5780639447cfd41461052b575f80fd5b806331ff41c8116101b55780634cb950e11161017a5780634cb950e11461041f5780634f1ef2861461043e57806352d1902d146104515780635bff76d91461046557806365b394af14610491575f80fd5b806331ff41c8146103775780633b56159e146103a357806341ad069c146103c257806346c5bbbd146103e157806347e8229514610400575f80fd5b806320a4eb39116101fb57806320a4eb39146102e4578063221cdd4e1461030357806326cf5def14610322578063281e8bfe146103445780632a38899814610363575f80fd5b806306834d1d146102375780630d8e6e2c1461025857806316d4eb6f146102825780631ce3f9bc146102a1578063203d0114146102b5575b5f80fd5b348015610242575f80fd5b506102566102513660046141b0565b610718565b005b348015610263575f80fd5b5061026c610874565b604051610279919061421d565b60405180910390f35b34801561028d575f80fd5b5061025661029c36600461428c565b6108e0565b3480156102ac575f80fd5b50610256610a61565b3480156102c0575f80fd5b506102d46102cf366004614313565b610b7e565b6040519015158152602001610279565b3480156102ef575f80fd5b506102566102fe36600461432e565b610bbe565b34801561030e575f80fd5b5061025661031d366004614382565b610cee565b34801561032d575f80fd5b50610336610ee6565b604051908152602001610279565b34801561034f575f80fd5b5061033661035e36600461432e565b610f0c565b34801561036e575f80fd5b50610336610f31565b348015610382575f80fd5b50610396610391366004614428565b610f57565b60405161027991906144a3565b3480156103ae575f80fd5b506102566103bd36600461432e565b61111b565b3480156103cd575f80fd5b506103366103dc36600461432e565b61128c565b3480156103ec575f80fd5b506102d46103fb366004614428565b6112d1565b34801561040b575f80fd5b5061033661041a36600461432e565b61132f565b34801561042a575f80fd5b506102566104393660046144b5565b611354565b61025661044c366004614600565b611b3c565b34801561045c575f80fd5b50610336611b5b565b348015610470575f80fd5b5061048461047f36600461432e565b611b76565b604051610279919061464c565b34801561049c575f80fd5b506104a5611bf9565b60408051928352602083019190915201610279565b3480156104c5575f80fd5b506102566104d43660046141b0565b611c19565b3480156104e4575f80fd5b50610484611d56565b3480156104f8575f80fd5b50610256610507366004614382565b611dd7565b348015610517575f80fd5b506102566105263660046141b0565b611f9f565b348015610536575f80fd5b506102d4610545366004614428565b6120fd565b348015610555575f80fd5b50610256610564366004614382565b61213b565b348015610574575f80fd5b50610336612336565b348015610588575f80fd5b5061026c604051806040016040528060058152602001640352e302e360dc1b81525081565b3480156105b8575f80fd5b506102566105c73660046141b0565b61234a565b3480156105d7575f80fd5b506102566105e63660046141b0565b61248a565b3480156105f6575f80fd5b506103366125d2565b34801561060a575f80fd5b50610256610619366004614698565b6125f8565b348015610629575f80fd5b506102d461063836600461432e565b61275c565b348015610648575f80fd5b5061025661065736600461432e565b612766565b348015610667575f80fd5b5061033661289f565b34801561067b575f80fd5b5061033661068a36600461432e565b6128c5565b34801561069a575f80fd5b506104a56106a936600461432e565b6128ea565b3480156106b9575f80fd5b506102d46106c83660046141b0565b612951565b3480156106d8575f80fd5b506102566106e736600461432e565b6129a6565b3480156106f7575f80fd5b5061070b61070636600461432e565b612bed565b6040516102799190614754565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610768573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061078c91906147b6565b6001600160a01b0316336001600160a01b0316146107c45760405163021bfda160e41b81523360048201526024015b60405180910390fd5b5f6107cd612db0565b90506107d883612dd4565b6108216040518060400160405280601081526020016f383ab13634b1a232b1b93cb83a34b7b760811b81525083836001015f8781526020019081526020015f2080549050612e00565b5f838152600682016020526040908190208390555183907fd571bf833e41553bbe260e00b3af7a0e91aafd6cdc238a803aa9ac0e73efed65906108679085815260200190565b60405180910390a2505050565b60606040518060400160405280600e81526020016d50726f746f636f6c436f6e66696760901b8152506108a65f612e77565b6108b06002612e77565b6108b95f612e77565b6040516020016108cc94939291906147d1565b604051602081830303815290604052905090565b5f8051602061535c833981519152546001600160401b03166001600160401b031660011461092157604051636f4f731f60e01b815260040160405180910390fd5b5f8051602061535c833981519152805460039190600160401b900460ff1680610957575080546001600160401b03808416911610155b156109755760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b03831617600160401b17815560f860076109a6911b6001614876565b8710156109c9576040516377ddbe8160e01b8152600481018890526024016107bb565b6109d8600160fb1b6001614876565b8610156109fb5760405163a225656d60e01b8152600481018790526024016107bb565b610a108787610a0a878961489a565b86612f06565b50805460ff60401b191681556040516001600160401b03831681527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a150505050505050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610ab1573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ad591906147b6565b6001600160a01b0316336001600160a01b031614610b085760405163021bfda160e41b81523360048201526024016107bb565b5f610b11612db0565b600b8101549091505f610b2382612f5a565b905080827f15aaaf475ef407543f5164f57dcf57f7f93816f55bae77ca09efc445ba40eef78486600d0154600143610b5b9190614a09565b6040805193845260208401929092529082015260600160405180910390a3505050565b5f80610b88612db0565b600b8101545f9081526003909101602090815260408083206001600160a01b039096168352949052929092205460ff1692915050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610c0e573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c3291906147b6565b6001600160a01b0316336001600160a01b031614610c655760405163021bfda160e41b81523360048201526024016107bb565b5f610c6e612db0565b905060015f838152600e8301602052604090205460ff166003811115610c9657610c9661484e565b14610cb757604051633586efa160e01b8152600481018390526024016107bb565b610cc082612fab565b60405182907f75e115b7f76bf21d0a2e42da9304d9c357b54c489e5af59ed3c70b7cd48335fc905f90a25050565b5f8051602061535c833981519152546001600160401b03166001600160401b0316600114610d2f57604051636f4f731f60e01b815260040160405180910390fd5b5f8051602061535c833981519152805460039190600160401b900460ff1680610d65575080546001600160401b03808416911610155b15610d835760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b03831617600160401b1781555f610dad612db0565b90505f610de1610dc2600760f81b6001614876565b610dd1600160fb1b6001614876565b610ddb8d8f61489a565b8c612f06565b905060405180604001604052804381526020018c8c8c8c8c8c8c604051602001610e119796959493929190614b4a565b60408051601f1981840301815291815281516020928301209092525f848152601786018252919091208251815591015160019091015560f86007901b817f204d6b80121154cd87d99cf54c639a3dd0a53b3084277098de972ebdd34c6be98d8d8d8d8d8d8d604051610e899796959493929190614b4a565b60405180910390a35050805460ff60401b191681556040516001600160401b03831681527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2906020015b60405180910390a1505050505050505050565b5f80610ef0612db0565b600b8101545f9081526009909101602052604090205492915050565b5f610f1682612dd4565b610f1e612db0565b5f92835260070160205250604090205490565b5f80610f3b612db0565b600b8101545f9081526006909101602052604090205492915050565b604080516080810182525f8082526020820152606091810182905281810191909152610f8283613066565b610fa2576040516377ddbe8160e01b8152600481018490526024016107bb565b610faa612db0565b5f848152600491909101602090815260408083206001600160a01b0380871685529083529281902081516080810183528154851681526001820154909416928401929092526002820180549184019161100290614d12565b80601f016020809104026020016040519081016040528092919081815260200182805461102e90614d12565b80156110795780601f1061105057610100808354040283529160200191611079565b820191905f5260205f20905b81548152906001019060200180831161105c57829003601f168201915b5050505050815260200160038201805461109290614d12565b80601f01602080910402602001604051908101604052809291908181526020018280546110be90614d12565b80156111095780601f106110e057610100808354040283529160200191611109565b820191905f5260205f20905b8154815290600101906020018083116110ec57829003601f168201915b50505050508152505090505b92915050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561116b573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061118f91906147b6565b6001600160a01b0316336001600160a01b0316146111c25760405163021bfda160e41b81523360048201526024016107bb565b5f6111cb612db0565b905060015f838152600f8301602052604090205460ff1660028111156111f3576111f361484e565b146112145760405163a225656d60e01b8152600481018390526024016107bb565b5f828152601082016020526040902054600b82015481146112525760405163a69d7d5b60e01b815260048101849052602481018290526044016107bb565b61125b8361309e565b604051839082907f6440aaea7b2480b82449c317aa5a9168df77eb69308ff8f7c3980a1ad848b7df905f90a3505050565b5f61129682613066565b6112b6576040516377ddbe8160e01b8152600481018390526024016107bb565b6112be612db0565b5f92835260080160205250604090205490565b5f6112db83613066565b6112fb576040516377ddbe8160e01b8152600481018490526024016107bb565b611303612db0565b5f938452600201602090815260408085206001600160a01b039490941685529290525090205460ff1690565b5f61133982612dd4565b611341612db0565b5f92835260090160205250604090205490565b5f61135d612db0565b905060015f878152600f8301602052604090205460ff1660028111156113855761138561484e565b146113a65760405163a225656d60e01b8152600481018790526024016107bb565b5f8681526010820160209081526040808320548084526002850183528184203385529092529091205460ff166113f85760405163a3f4afeb60e01b8152336004820152602481018890526044016107bb565b60015f828152600e8401602052604090205460ff16600381111561141e5761141e61484e565b0361143f57604051631962dcfb60e11b8152600481018290526024016107bb565b61144881613066565b611468576040516377ddbe8160e01b8152600481018290526024016107bb565b5f81815260048301602090815260408083203384528252808320600101548151600160f91b938101939093526021830185905260418084018c90528251808503909101815260619093019091526001600160a01b0316919081886001600160401b038111156114d9576114d9614528565b604051908082528060200260200182016040528015611502578160200160208202803683370190505b5090505f5b89811015611690575f61154a8c8c8481811061152557611525614d44565b90506020028101906115379190614d58565b611545906040810190614d76565b6130d1565b90505f6115a48d8d8581811061156257611562614d44565b90506020028101906115749190614d58565b358e8e8681811061158757611587614d44565b90506020028101906115999190614d58565b602001358488613237565b90506115e287828f8f878181106115bd576115bd614d44565b90506020028101906115cf9190614d58565b6115dd906060810190614dbb565b6133b3565b8c8c848181106115f4576115f4614d44565b90506020028101906116069190614d58565b358d8d8581811061161957611619614d44565b905060200281019061162b9190614d58565b6020013583604051602001611653939291909283526020830191909152604082015260600190565b6040516020818303038152906040528051906020012084848151811061167b5761167b614d44565b60209081029190910101525050600101611507565b505f876001600160401b038111156116aa576116aa614528565b6040519080825280602002602001820160405280156116d3578160200160208202803683370190505b5090505f5b88811015611850575f6117698b8b848181106116f6576116f6614d44565b90506020028101906117089190614d58565b358c8c8581811061171b5761171b614d44565b905060200281019061172d9190614d58565b602001358d8d8681811061174357611743614d44565b90506020028101906117559190614d58565b611763906040810190614dbb565b89613438565b905061178287828d8d868181106115bd576115bd614d44565b8a8a8381811061179457611794614d44565b90506020028101906117a69190614d58565b358b8b848181106117b9576117b9614d44565b90506020028101906117cb9190614d58565b602001358c8c858181106117e1576117e1614d44565b90506020028101906117f39190614d58565b611801906040810190614dbb565b6040516020016118149493929190614dfd565b6040516020818303038152906040528051906020012083838151811061183c5761183c614d44565b6020908102919091010152506001016116d8565b508181604051602001611864929190614e56565b60408051601f1981840301815291815281516020928301205f8f815260128b0184528281206001600160a01b038a16825290935291205490945060ff161592506118d6915050576040516324a0bb1b60e11b81526001600160a01b0383166004820152602481018a90526044016107bb565b5f89815260128501602090815260408083206001600160a01b03861684528252808320805460ff191660011790558b835260138701825280832084845290915281208054829061192590614e7a565b9190508190559050826001600160a01b03168a7f7eda6f85e23b7b91c019b0570d02b663606ef9d74594f7e01fcfbdb0f4e954d58460405161196991815260200190565b60405180910390a35f8481526005860160205260409020548103611b30575f848152600e860160205260409020805460ff19166003179055600b85018490556119b28a856134c5565b5f848152600186016020526040812080549091906001600160401b038111156119dd576119dd614528565b604051908082528060200260200182016040528015611a1057816020015b60608152602001906001900390816119fb5790505b5090505f5b8254811015611aeb57828181548110611a3057611a30614d44565b905f5260205f2090600402016003018054611a4a90614d12565b80601f0160208091040260200160405190810160405280929190818152602001828054611a7690614d12565b8015611ac15780601f10611a9857610100808354040283529160200191611ac1565b820191905f5260205f20905b815481529060010190602001808311611aa457829003601f168201915b5050505050828281518110611ad857611ad8614d44565b6020908102919091010152600101611a15565b508b867f1a547b42e72cd3dda04e6adccd2200276cfef01fe2138d07f3a7440f416d38bc8d8d8d8d87604051611b2595949392919061504c565b60405180910390a350505b50505050505050505050565b611b446134ff565b611b4d826135a5565b611b57828261364c565b5050565b5f611b64613708565b505f8051602061533c83398151915290565b6060611b8182612dd4565b611b89612db0565b6005015f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020018280548015611bed57602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311611bcf575b50505050509050919050565b5f805f611c04612db0565b905080600b0154925080600d01549150509091565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611c69573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c8d91906147b6565b6001600160a01b0316336001600160a01b031614611cc05760405163021bfda160e41b81523360048201526024016107bb565b5f611cc9612db0565b9050611cd483612dd4565b611d10604051806040016040528060038152602001626d706360e81b81525083836001015f8781526020019081526020015f2080549050612e00565b5f838152600982016020526040908190208390555183907f148f9c6cb77d12306b9f596534d14b7aae3e4f98a2dbe3cdb07ea4924c775f12906108679085815260200190565b60605f611d61612db0565b9050806005015f82600b015481526020019081526020015f20805480602002602001604051908101604052809291908181526020018280548015611dcc57602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311611dae575b505050505091505090565b5f8051602061535c833981519152805460039190600160401b900460ff1680611e0d575080546001600160401b03808416911610155b15611e2b5760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b03831617600160401b1781555f611e55612db0565b600160fb1b600c82019081558154600b83018190555f818152600e840160205260408120805460ff19166003179055825493945090929091908290611e9990614e7a565b91829055509050611eaa81836134c5565b60405180604001604052804381526020018d8d8d8d8d8d8d604051602001611ed89796959493929190614b4a565b60408051601f1981840301815291815281516020928301209092525f858152601787018252919091208251815591015160019091015560f86007901b827f204d6b80121154cd87d99cf54c639a3dd0a53b3084277098de972ebdd34c6be98e8e8e8e8e8e8e604051611f509796959493929190614b4a565b60405180910390a35050815460ff60401b19168255506040516001600160401b03831681527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d290602001610ed3565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611fef573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061201391906147b6565b6001600160a01b0316336001600160a01b0316146120465760405163021bfda160e41b81523360048201526024016107bb565b5f61204f612db0565b905080600b01548314158061206a575061206883613066565b155b1561208b576040516377ddbe8160e01b8152600481018490526024016107bb565b600c8101548083116120ba5760405163e8121f5160e01b815260048101849052602481018290526044016107bb565b600c82018390556120cb83856134c5565b604051839085907f0a1c24c2ba5e6e1b1a8585795e5b781e372aee1db686247dac7574c10fd735a6905f90a350505050565b5f61210783612dd4565b61210f612db0565b5f938452600301602090815260408085206001600160a01b039490941685529290525090205460ff1690565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561218b573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906121af91906147b6565b6001600160a01b0316336001600160a01b0316146121e25760405163021bfda160e41b81523360048201526024016107bb565b5f6121eb612db0565b600b8101549091505f6122076122018a8c61489a565b89613751565b5f818152600e850160205260409020805460ff19166001179055905061222c81612f5a565b505f828152600984016020908152604080832054600187019092528220546122549190614a09565b90505f8111612264576001612266565b805b846014015f8481526020019081526020015f208190555060405180604001604052804381526020018c8c8c8c8c8c8c6040516020016122ab9796959493929190614b4a565b60408051601f1981840301815291815281516020928301209092525f8581526017880182528290208351815592015160019092019190915551839083907f204d6b80121154cd87d99cf54c639a3dd0a53b3084277098de972ebdd34c6be990612321908f908f908f908f908f908f908f90614b4a565b60405180910390a35050505050505050505050565b5f80612340612db0565b600b015492915050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561239a573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906123be91906147b6565b6001600160a01b0316336001600160a01b0316146123f15760405163021bfda160e41b81523360048201526024016107bb565b5f6123fa612db0565b905061240583612dd4565b6124446040518060400160405280600681526020016535b6b9a3b2b760d11b81525083836001015f8781526020019081526020015f2080549050612e00565b5f838152600882016020526040908190208390555183907ff21cb37be709148aabebd278543e62d1b1e6a4477fb1cc43e069d3eeb8c87f90906108679085815260200190565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156124da573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906124fe91906147b6565b6001600160a01b0316336001600160a01b0316146125315760405163021bfda160e41b81523360048201526024016107bb565b5f61253a612db0565b905061254583612dd4565b61258c6040518060400160405280600e81526020016d3ab9b2b92232b1b93cb83a34b7b760911b81525083836001015f8781526020019081526020015f2080549050612e00565b5f838152600782016020526040908190208390555183907f90f1918493831c1b6133489743103384c5600eae796eb34c51ea4f2baafa4f94906108679085815260200190565b5f806125dc612db0565b600b8101545f9081526008909101602052604090205492915050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612648573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061266c91906147b6565b6001600160a01b0316336001600160a01b03161461269f5760405163021bfda160e41b81523360048201526024016107bb565b5f6126a8612db0565b600b810154909150808b116126da5760405163efd55f6760e01b8152600481018c9052602481018290526044016107bb565b600c820154808b116127095760405163e8121f5160e01b8152600481018c9052602481018290526044016107bb565b61271e8c8c6127188c8e61489a565b8b612f06565b508a8c7f2ac68f78f4ccde76b64906026d01ff3c42403eb7eef86fe788474a23267d64cf8c8c8c8c8c8c8c604051611b259796959493929190614b4a565b5f61111582613776565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156127b6573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906127da91906147b6565b6001600160a01b0316336001600160a01b03161461280d5760405163021bfda160e41b81523360048201526024016107bb565b5f612816612db0565b905080600b0154820361283f576040516322cafe7160e11b8152600481018390526024016107bb565b61284882613066565b612868576040516377ddbe8160e01b8152600481018390526024016107bb565b61287182612fab565b60405182907fda075d09198d207e3a918d4b8dfc87df2d60a00be703fd39eaac90962da0b7f0905f90a25050565b5f806128a9612db0565b600b8101545f9081526007909101602052604090205492915050565b5f6128cf82612dd4565b6128d7612db0565b5f92835260060160205250604090205490565b5f806128f5836137c0565b612915576040516377ddbe8160e01b8152600481018490526024016107bb565b5f61291e612db0565b5f948552601701602090815260409485902085518087019096528054808752600190910154959091018590529492505050565b5f8061295b612db0565b905060025f848152600f8301602052604090205460ff1660028111156129835761298361484e565b14801561299e57505f83815260108201602052604090205484145b949350505050565b5f6129af612db0565b905060015f838152600e8301602052604090205460ff1660038111156129d7576129d761484e565b146129f857604051633586efa160e01b8152600481018390526024016107bb565b600b8101545f818152600283016020818152604080842033808652908352818520548886529383528185209085529091529091205460ff918216911681158015612a40575080155b15612a6757604051631703bf1d60e31b8152336004820152602481018690526044016107bb565b5f858152601185016020908152604080832033845290915290205460ff1615612aac57604051630c4b0b9960e31b8152336004820152602481018690526044016107bb565b5f85815260118501602090815260408083203384529091529020805460ff191660011790558115612af9575f85815260168501602052604081208054909190612af490614e7a565b909155505b8015612b21575f85815260158501602052604081208054909190612b1c90614e7a565b909155505b6040805183151581528215156020820152339187917fb79c48003695b6ebe555afa36fad071deeee75eb3718ad63de5621d35ba44b4f910160405180910390a3612b6a85613807565b15612be6575f858152600e850160205260409020805460ff19166002179055600c840154600d850154819087907f15aaaf475ef407543f5164f57dcf57f7f93816f55bae77ca09efc445ba40eef7908790612bc6600143614a09565b6040805193845260208401929092529082015260600160405180910390a3505b5050505050565b6060612bf882612dd4565b612c00612db0565b6001015f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020015f905b82821015612da5575f848152602090819020604080516080810182526004860290920180546001600160a01b0390811684526001820154169383019390935260028301805492939291840191612c8690614d12565b80601f0160208091040260200160405190810160405280929190818152602001828054612cb290614d12565b8015612cfd5780601f10612cd457610100808354040283529160200191612cfd565b820191905f5260205f20905b815481529060010190602001808311612ce057829003601f168201915b50505050508152602001600382018054612d1690614d12565b80601f0160208091040260200160405190810160405280929190818152602001828054612d4290614d12565b8015612d8d5780601f10612d6457610100808354040283529160200191612d8d565b820191905f5260205f20905b815481529060010190602001808311612d7057829003601f168201915b50505050508152505081526020019060010190612c31565b505050509050919050565b7f80f3585af86806c5774303b06c1ee640aa83b6ef3e45df49bb26c8524500c20090565b612ddd81613776565b612dfd576040516377ddbe8160e01b8152600481018290526024016107bb565b50565b815f03612e225782604051631b5fdb0760e11b81526004016107bb919061421d565b60ff821115612e4b576040516322ba52db60e01b81526107bb908490849060ff90600401615159565b80821115612e725782828260405163caa814a360e01b81526004016107bb93929190615159565b505050565b60605f612e8383613861565b60010190505f816001600160401b03811115612ea157612ea1614528565b6040519080825280601f01601f191660200182016040528015612ecb576020820181803683370190505b5090508181016020015b5f19016f181899199a1a9b1b9c1cb0b131b232b360811b600a86061a8153600a8504945084612ed557509392505050565b5f80612f10612db0565b9050612f1d868585613938565b5f818152600e830160205260409020805460ff19166003179055600b8201819055600c82018690559150612f5185836134c5565b50949350505050565b5f80612f64612db0565b905080600c015f8154612f7690614e7a565b91829055505f818152600f830160209081526040808320805460ff191660011790556010909401905291909120929092555090565b5f612fb4612db0565b5f838152600a8201602090815260408083208054600160ff199182168117909255600e8601845282852080549091169055600c850154808552600f86019093529220549293509160ff16600281111561300f5761300f61484e565b14801561302a57505f81815260108301602052604090205483145b15613038576130388161309e565b505f918252601481016020908152604080842084905560158301825280842084905560169092019052812055565b5f80613070612db0565b905061307b836137c0565b801561309757505f838152600a8201602052604090205460ff16155b9392505050565b5f6130a7612db0565b5f928352600f810160209081526040808520805460ff191690556010909201905282209190915550565b5f80826001600160401b038111156130eb576130eb614528565b604051908082528060200260200182016040528015613114578160200160208202803683370190505b5090505f5b83811015613206577fddd108772e6a3899feb04d148ae915cbe3eb5ebd202688080399e9921ac3616b85858381811061315457613154614d44565b9050602002810190613166919061517d565b613174906020810190615191565b86868481811061318657613186614d44565b9050602002810190613198919061517d565b6131a6906020810190614dbb565b6040516131b49291906151aa565b6040519081900381206131cb9392916020016151b9565b604051602081830303815290604052805190602001208282815181106131f3576131f3614d44565b6020908102919091010152600101613119565b508060405160200161321891906151db565b6040516020818303038152906040528051906020012091505092915050565b8051602080830191909120604080517fbd14835bb4ae13c78ecb88ded2c3370325f39e6006eb94ff45e95f98e4c85a2a938101939093528201869052606082018590526080820184905260a08201525f906133aa9060c0015b60405160208183030381529060405280519060200120604080518082018252600e81526d50726f746f636f6c436f6e66696760901b6020918201528151808301835260018152603160f81b9082015281517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f818301527fa3de1880cf083e8318b77a7965d02dd9765e85a48e418a4463af7a0d57b4b3ee818401527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc660608201524660808201523060a0808301919091528351808303909101815260c08201845280519083012061190160f01b60e083015260e2820152610102808201949094528251808203909401845261012201909152815191012090565b95945050505050565b5f6133f38484848080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250613b5792505050565b9050846001600160a01b0316816001600160a01b031614612be6576040516378b9ada360e11b81526001600160a01b03821660048201523360248201526044016107bb565b5f6134bb7fa264b318e95080300a3f06a6656a8e7fe24f9903f0e6bcca307efbe39c4c4e09878787876040516020016134729291906151aa565b60408051601f19818403018152828252805160209182012089518a83012091840196909652908201939093526060810191909152608081019290925260a082015260c001613290565b9695505050505050565b5f6134ce612db0565b5f848152600f820160209081526040808320805460ff191660021790556010840190915290209290925550600d0155565b306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016148061358557507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166135795f8051602061533c833981519152546001600160a01b031690565b6001600160a01b031614155b156135a35760405163703e46dd60e11b815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156135f5573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061361991906147b6565b6001600160a01b0316336001600160a01b031614612dfd5760405163021bfda160e41b81523360048201526024016107bb565b816001600160a01b03166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa9250505080156136a6575060408051601f3d908101601f191682019092526136a391810190615210565b60015b6136ce57604051634c9c8ce360e01b81526001600160a01b03831660048201526024016107bb565b5f8051602061533c83398151915281146136fe57604051632a87526960e21b8152600481018290526024016107bb565b612e728383613b7f565b306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146135a35760405163703e46dd60e11b815260040160405180910390fd5b5f8061375b612db0565b805490915061299e9061376f906001614876565b8585613938565b5f80613780612db0565b905061378b83613066565b8015613097575060035f848152600e8301602052604090205460ff1660038111156137b8576137b861484e565b149392505050565b5f806137ca612db0565b90506137db600760f81b6001614876565b83101580156137eb575080548311155b801561309757505f928352600101602052506040902054151590565b5f80613811612db0565b5f848152600182016020908152604080832054601585019092529091205491925014801561309757505f838152601482016020908152604080832054601685019092529091205410159392505050565b5f8072184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b831061389f5772184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b830492506040015b6d04ee2d6d415b85acef810000000083106138cb576d04ee2d6d415b85acef8100000000830492506020015b662386f26fc1000083106138e957662386f26fc10000830492506010015b6305f5e1008310613901576305f5e100830492506008015b612710831061391557612710830492506004015b60648310613927576064830492506002015b600a83106111155760010192915050565b5f82515f0361395957604051621a323560e61b815260040160405180910390fd5b825160ff10156139895782516040516302d4e4ef60e31b8152600481019190915260ff60248201526044016107bb565b6139c06040518060400160405280601081526020016f383ab13634b1a232b1b93cb83a34b7b760811b815250835f01358551612e00565b6139f66040518060400160405280600e81526020016d3ab9b2b92232b1b93cb83a34b7b760911b81525083602001358551612e00565b613a246040518060400160405280600681526020016535b6b9a3b2b760d11b81525083604001358551612e00565b613a4f604051806040016040528060038152602001626d706360e81b81525083606001358551612e00565b5f613a58612db0565b80549091508511613a8957805460405163efd55f6760e01b81526107bb918791600401918252602082015260400190565b8481558491505f5b8451811015613b0b575f858281518110613aad57613aad614d44565b60200260200101519050613b02846040518060800160405280845f01516001600160a01b0316815260200184602001516001600160a01b03168152602001846040015181526020018460600151815250613bd4565b50600101613a91565b505f828152600682016020908152604080832086359055600784018252808320828701359055600884018252808320818701359055600990930190522060609092013590915592915050565b5f805f80613b658686613e77565b925092509250613b758282613ec0565b5090949350505050565b613b8882613f78565b6040516001600160a01b038316907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a2805115613bcc57612e728282613fdb565b611b57614044565b5f613bdd612db0565b82519091506001600160a01b0316613c0857604051634233402560e11b815260040160405180910390fd5b60208201516001600160a01b0316613c3357604051632deccf4d60e01b815260040160405180910390fd5b5f838152600282016020908152604080832085516001600160a01b0316845290915290205460ff1615613c87578151604051630d18c4ff60e41b81526001600160a01b0390911660048201526024016107bb565b5f8381526003820160209081526040808320858301516001600160a01b0316845290915290205460ff1615613ce057602082015160405163f51af6bb60e01b81526001600160a01b0390911660048201526024016107bb565b5f8381526001828101602090815260408084208054808501825590855293829020865160049095020180546001600160a01b03199081166001600160a01b03968716178255928701519381018054909316939094169290921790558301518391906002820190613d50908261526b565b5060608201516003820190613d65908261526b565b5050505f83815260028083016020908152604080842086516001600160a01b039081168652908352818520805460ff1990811660019081179092558987526003880185528387208986018051851689529086528488208054909216831790915589875260048801855283872089518416885290945294829020875181549083166001600160a01b03199182161782559351958101805496909216959093169490941790935591840151849291820190613e1e908261526b565b5060608201516003820190613e33908261526b565b5050505f92835260050160209081526040832091810151825460018101845592845292200180546001600160a01b0319166001600160a01b03909216919091179055565b5f805f8351604103613eae576020840151604085015160608601515f1a613ea088828585614063565b955095509550505050613eb9565b505081515f91506002905b9250925092565b5f826003811115613ed357613ed361484e565b03613edc575050565b6001826003811115613ef057613ef061484e565b03613f0e5760405163f645eedf60e01b815260040160405180910390fd5b6002826003811115613f2257613f2261484e565b03613f435760405163fce698f760e01b8152600481018290526024016107bb565b6003826003811115613f5757613f5761484e565b03611b57576040516335e2f38360e21b8152600481018290526024016107bb565b806001600160a01b03163b5f03613fad57604051634c9c8ce360e01b81526001600160a01b03821660048201526024016107bb565b5f8051602061533c83398151915280546001600160a01b0319166001600160a01b0392909216919091179055565b60605f80846001600160a01b031684604051613ff7919061532a565b5f60405180830381855af49150503d805f811461402f576040519150601f19603f3d011682016040523d82523d5f602084013e614034565b606091505b50915091506133aa85838361412b565b34156135a35760405163b398979f60e01b815260040160405180910390fd5b5f80807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084111561409c57505f91506003905082614121565b604080515f808252602082018084528a905260ff891692820192909252606081018790526080810186905260019060a0016020604051602081039080840390855afa1580156140ed573d5f803e3d5ffd5b5050604051601f1901519150506001600160a01b03811661411857505f925060019150829050614121565b92505f91508190505b9450945094915050565b6060826141405761413b82614187565b613097565b815115801561415757506001600160a01b0384163b155b1561418057604051639996b31560e01b81526001600160a01b03851660048201526024016107bb565b5092915050565b8051156141975780518082602001fd5b60405163d6bda27560e01b815260040160405180910390fd5b5f80604083850312156141c1575f80fd5b50508035926020909101359150565b5f5b838110156141ea5781810151838201526020016141d2565b50505f910152565b5f81518084526142098160208601602086016141d0565b601f01601f19169290920160200192915050565b602081525f61309760208301846141f2565b5f8083601f84011261423f575f80fd5b5081356001600160401b03811115614255575f80fd5b6020830191508360208260051b850101111561426f575f80fd5b9250929050565b5f60808284031215614286575f80fd5b50919050565b5f805f805f60e086880312156142a0575f80fd5b853594506020860135935060408601356001600160401b038111156142c3575f80fd5b6142cf8882890161422f565b90945092506142e390508760608801614276565b90509295509295909350565b6001600160a01b0381168114612dfd575f80fd5b803561430e816142ef565b919050565b5f60208284031215614323575f80fd5b8135613097816142ef565b5f6020828403121561433e575f80fd5b5035919050565b5f8083601f840112614355575f80fd5b5081356001600160401b0381111561436b575f80fd5b60208301915083602082850101111561426f575f80fd5b5f805f805f805f60e0888a031215614398575f80fd5b87356001600160401b03808211156143ae575f80fd5b6143ba8b838c0161422f565b90995097508791506143cf8b60208c01614276565b965060a08a01359150808211156143e4575f80fd5b6143f08b838c01614345565b909650945060c08a0135915080821115614408575f80fd5b506144158a828b0161422f565b989b979a50959850939692959293505050565b5f8060408385031215614439575f80fd5b82359150602083013561444b816142ef565b809150509250929050565b5f60018060a01b038083511684528060208401511660208501525060408201516080604085015261448a60808501826141f2565b9050606083015184820360608601526133aa82826141f2565b602081525f6130976020830184614456565b5f805f805f606086880312156144c9575f80fd5b8535945060208601356001600160401b03808211156144e6575f80fd5b6144f289838a0161422f565b9096509450604088013591508082111561450a575f80fd5b506145178882890161422f565b969995985093965092949392505050565b634e487b7160e01b5f52604160045260245ffd5b60405161010081016001600160401b038111828210171561455f5761455f614528565b60405290565b604051601f8201601f191681016001600160401b038111828210171561458d5761458d614528565b604052919050565b5f82601f8301126145a4575f80fd5b81356001600160401b038111156145bd576145bd614528565b6145d0601f8201601f1916602001614565565b8181528460208386010111156145e4575f80fd5b816020850160208301375f918101602001919091529392505050565b5f8060408385031215614611575f80fd5b823561461c816142ef565b915060208301356001600160401b03811115614636575f80fd5b61464285828601614595565b9150509250929050565b602080825282518282018190525f9190848201906040850190845b8181101561468c5783516001600160a01b031683529284019291840191600101614667565b50909695505050505050565b5f805f805f805f805f6101208a8c0312156146b1575f80fd5b8935985060208a0135975060408a01356001600160401b03808211156146d5575f80fd5b6146e18d838e0161422f565b90995097508791506146f68d60608e01614276565b965060e08c013591508082111561470b575f80fd5b6147178d838e01614345565b90965094506101008c0135915080821115614730575f80fd5b5061473d8c828d0161422f565b915080935050809150509295985092959850929598565b5f60208083016020845280855180835260408601915060408160051b8701019250602087015f5b828110156147a957603f19888603018452614797858351614456565b9450928501929085019060010161477b565b5092979650505050505050565b5f602082840312156147c6575f80fd5b8151613097816142ef565b5f85516147e2818460208a016141d0565b61103b60f11b9083019081528551614801816002840160208a016141d0565b808201915050601760f91b8060028301528551614825816003850160208a016141d0565b600392019182015283516148408160048401602088016141d0565b016004019695505050505050565b634e487b7160e01b5f52602160045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b8082018082111561111557611115614862565b8035600381900b811461430e575f80fd5b5f6001600160401b03808411156148b3576148b3614528565b8360051b60206148c4818301614565565b8681529185019181810190368411156148db575f80fd5b865b848110156149fd578035868111156148f3575f80fd5b8801610100368290031215614906575f80fd5b61490e61453c565b61491782614303565b8152614924868301614303565b868201526040808301358981111561493a575f80fd5b61494636828601614595565b8284015250506060808301358981111561495e575f80fd5b61496a36828601614595565b828401525050608061497d818401614889565b9082015260a08281013589811115614993575f80fd5b61499f36828601614595565b82840152505060c080830135898111156149b7575f80fd5b6149c336828601614595565b82840152505060e080830135898111156149db575f80fd5b6149e736828601614595565b91830191909152508452509183019183016148dd565b50979650505050505050565b8181038181111561111557611115614862565b5f808335601e19843603018112614a31575f80fd5b83016020810192503590506001600160401b03811115614a4f575f80fd5b80360382131561426f575f80fd5b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b5f8383855260208086019550808560051b830101845f5b87811015614b3d57848303601f19018952813536889003605e19018112614ac1575f80fd5b87016060614acf8280614a1c565b828752614adf8388018284614a5d565b92505050614aef86830183614a1c565b86830388880152614b01838284614a5d565b925050506040614b1381840184614a1c565b935086830382880152614b27838583614a5d565b9c88019c96505050928501925050600101614a9c565b5090979650505050505050565b60e080825281018790525f61010080830160058a901b840182018b845b8c811015614caa5786830360ff190184528135368f900360fe19018112614b8c575f80fd5b8e01614ba884614b9b83614303565b6001600160a01b03169052565b6020614bb5818301614303565b6001600160a01b0316818601526040614bd083820184614a1c565b8983890152614be28a89018284614a5d565b925050506060614bf481850185614a1c565b888403838a0152614c06848284614a5d565b93505050506080614c18818501614889565b614c268289018260030b9052565b505060a0614c3681850185614a1c565b888403838a0152614c48848284614a5d565b935050505060c0614c5b81850185614a1c565b888403838a0152614c6d848284614a5d565b9350505050614c7f60e0840184614a1c565b935086820360e0880152614c94828583614a5d565b9783019796505050929092019150600101614b67565b5050614cda602086018b803582526020810135602083015260408101356040830152606081013560608301525050565b84810360a0860152614ced81898b614a5d565b9250505082810360c0840152614d04818587614a85565b9a9950505050505050505050565b600181811c90821680614d2657607f821691505b60208210810361428657634e487b7160e01b5f52602260045260245ffd5b634e487b7160e01b5f52603260045260245ffd5b5f8235607e19833603018112614d6c575f80fd5b9190910192915050565b5f808335601e19843603018112614d8b575f80fd5b8301803591506001600160401b03821115614da4575f80fd5b6020019150600581901b360382131561426f575f80fd5b5f808335601e19843603018112614dd0575f80fd5b8301803591506001600160401b03821115614de9575f80fd5b60200191503681900382131561426f575f80fd5b848152836020820152606060408201525f6134bb606083018486614a5d565b5f815180845260208085019450602084015f5b83811015614e4b57815187529582019590820190600101614e2f565b509495945050505050565b604081525f614e686040830185614e1c565b82810360208401526133aa8185614e1c565b5f60018201614e8b57614e8b614862565b5060010190565b80356002811061430e575f80fd5b60028110614ebc57634e487b7160e01b5f52602160045260245ffd5b9052565b5f8383855260208086019550808560051b830101845f5b87811015614b3d57848303601f19018952813536889003603e19018112614efc575f80fd5b87016040614f1285614f0d84614e92565b614ea0565b614f1e86830183614a1c565b92508187870152614f328287018483614a5d565b9b87019b955050509184019150600101614ed7565b5f8235607e19833603018112614f5b575f80fd5b90910192915050565b5f8383855260208086019550808560051b830101845f5b87811015614b3d57848303601f19018952614f968288614f47565b60808135855285820135868601526040614fb281840184614a1c565b8383890152614fc48489018284614a5d565b93505050506060614fd781840184614a1c565b935086830382880152614feb838583614a5d565b9c88019c96505050928501925050600101614f7b565b5f8282518085526020808601955060208260051b840101602086015f5b84811015614b3d57601f1986840301895261503a8383516141f2565b9884019892509083019060010161501e565b60608082528181018690525f906080808401600589811b860183018b865b8c81101561512057888303607f19018552615085828f614f47565b8035845260208082013581860152604080830135601e198436030181126150aa575f80fd5b830182810190356001600160401b038111156150c4575f80fd5b80891b36038213156150d4575f80fd5b8a838901526150e68b89018284614ec0565b925050506150f68a840184614a1c565b93508682038b88015261510a828583614a5d565b988301989650505092909201915060010161506a565b50508681036020880152615135818a8c614f64565b945050505050828103604084015261514d8185615001565b98975050505050505050565b606081525f61516b60608301866141f2565b60208301949094525060400152919050565b5f8235603e19833603018112614d6c575f80fd5b5f602082840312156151a1575f80fd5b61309782614e92565b818382375f9101908152919050565b838152606081016151cd6020830185614ea0565b826040830152949350505050565b81515f9082906020808601845b83811015615204578151855293820193908201906001016151e8565b50929695505050505050565b5f60208284031215615220575f80fd5b5051919050565b601f821115612e7257805f5260205f20601f840160051c8101602085101561524c5750805b601f840160051c820191505b81811015612be6575f8155600101615258565b81516001600160401b0381111561528457615284614528565b615298816152928454614d12565b84615227565b602080601f8311600181146152cb575f84156152b45750858301515b5f19600386901b1c1916600185901b178555615322565b5f85815260208120601f198616915b828110156152f9578886015182559484019460019091019084016152da565b508582101561531657878501515f19600388901b60f8161c191681555b505060018460011b0185555b505050505050565b5f8251614d6c8184602087016141d056fe360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbcf0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x023W_5`\xE0\x1C\x80cw\xD3\x8E$\x11a\x01)W\x80c\xB4r+\xC4\x11a\0\xA8W\x80c\xC3\xAA\xAAZ\x11a\0mW\x80c\xC3\xAA\xAAZ\x14a\x06pW\x80c\xC9\x99\xA8\xB4\x14a\x06\x8FW\x80c\xCC\xEA\xC0\x19\x14a\x06\xAEW\x80c\xD9\xBE-\xE4\x14a\x06\xCDW\x80c\xF9\xC6p\xC3\x14a\x06\xECW_\x80\xFD[\x80c\xB4r+\xC4\x14a\x05\xEBW\x80c\xBCM\x07\xC2\x14a\x05\xFFW\x80c\xBF\x9B\x16\xC8\x14a\x06\x1EW\x80c\xC0\xAEd\xF7\x14a\x06=W\x80c\xC2\xB4)\x86\x14a\x06\\W_\x80\xFD[\x80c\x97l\x98\xB5\x11a\0\xEEW\x80c\x97l\x98\xB5\x14a\x05JW\x80c\x97o>\xB9\x14a\x05iW\x80c\xAD<\xB1\xCC\x14a\x05}W\x80c\xB0\xB4a\xC4\x14a\x05\xADW\x80c\xB1\x81\xCD\xA7\x14a\x05\xCCW_\x80\xFD[\x80cw\xD3\x8E$\x14a\x04\xBAW\x80c~\xAA\xC8\xF2\x14a\x04\xD9W\x80c\x8A\xEA\xC2)\x14a\x04\xEDW\x80c\x8E\x97\xCB`\x14a\x05\x0CW\x80c\x94G\xCF\xD4\x14a\x05+W_\x80\xFD[\x80c1\xFFA\xC8\x11a\x01\xB5W\x80cL\xB9P\xE1\x11a\x01zW\x80cL\xB9P\xE1\x14a\x04\x1FW\x80cO\x1E\xF2\x86\x14a\x04>W\x80cR\xD1\x90-\x14a\x04QW\x80c[\xFFv\xD9\x14a\x04eW\x80ce\xB3\x94\xAF\x14a\x04\x91W_\x80\xFD[\x80c1\xFFA\xC8\x14a\x03wW\x80c;V\x15\x9E\x14a\x03\xA3W\x80cA\xAD\x06\x9C\x14a\x03\xC2W\x80cF\xC5\xBB\xBD\x14a\x03\xE1W\x80cG\xE8\"\x95\x14a\x04\0W_\x80\xFD[\x80c \xA4\xEB9\x11a\x01\xFBW\x80c \xA4\xEB9\x14a\x02\xE4W\x80c\"\x1C\xDDN\x14a\x03\x03W\x80c&\xCF]\xEF\x14a\x03\"W\x80c(\x1E\x8B\xFE\x14a\x03DW\x80c*8\x89\x98\x14a\x03cW_\x80\xFD[\x80c\x06\x83M\x1D\x14a\x027W\x80c\r\x8En,\x14a\x02XW\x80c\x16\xD4\xEBo\x14a\x02\x82W\x80c\x1C\xE3\xF9\xBC\x14a\x02\xA1W\x80c =\x01\x14\x14a\x02\xB5W[_\x80\xFD[4\x80\x15a\x02BW_\x80\xFD[Pa\x02Va\x02Q6`\x04aA\xB0V[a\x07\x18V[\0[4\x80\x15a\x02cW_\x80\xFD[Pa\x02la\x08tV[`@Qa\x02y\x91\x90aB\x1DV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x8DW_\x80\xFD[Pa\x02Va\x02\x9C6`\x04aB\x8CV[a\x08\xE0V[4\x80\x15a\x02\xACW_\x80\xFD[Pa\x02Va\naV[4\x80\x15a\x02\xC0W_\x80\xFD[Pa\x02\xD4a\x02\xCF6`\x04aC\x13V[a\x0B~V[`@Q\x90\x15\x15\x81R` \x01a\x02yV[4\x80\x15a\x02\xEFW_\x80\xFD[Pa\x02Va\x02\xFE6`\x04aC.V[a\x0B\xBEV[4\x80\x15a\x03\x0EW_\x80\xFD[Pa\x02Va\x03\x1D6`\x04aC\x82V[a\x0C\xEEV[4\x80\x15a\x03-W_\x80\xFD[Pa\x036a\x0E\xE6V[`@Q\x90\x81R` \x01a\x02yV[4\x80\x15a\x03OW_\x80\xFD[Pa\x036a\x03^6`\x04aC.V[a\x0F\x0CV[4\x80\x15a\x03nW_\x80\xFD[Pa\x036a\x0F1V[4\x80\x15a\x03\x82W_\x80\xFD[Pa\x03\x96a\x03\x916`\x04aD(V[a\x0FWV[`@Qa\x02y\x91\x90aD\xA3V[4\x80\x15a\x03\xAEW_\x80\xFD[Pa\x02Va\x03\xBD6`\x04aC.V[a\x11\x1BV[4\x80\x15a\x03\xCDW_\x80\xFD[Pa\x036a\x03\xDC6`\x04aC.V[a\x12\x8CV[4\x80\x15a\x03\xECW_\x80\xFD[Pa\x02\xD4a\x03\xFB6`\x04aD(V[a\x12\xD1V[4\x80\x15a\x04\x0BW_\x80\xFD[Pa\x036a\x04\x1A6`\x04aC.V[a\x13/V[4\x80\x15a\x04*W_\x80\xFD[Pa\x02Va\x0496`\x04aD\xB5V[a\x13TV[a\x02Va\x04L6`\x04aF\0V[a\x1B<V[4\x80\x15a\x04\\W_\x80\xFD[Pa\x036a\x1B[V[4\x80\x15a\x04pW_\x80\xFD[Pa\x04\x84a\x04\x7F6`\x04aC.V[a\x1BvV[`@Qa\x02y\x91\x90aFLV[4\x80\x15a\x04\x9CW_\x80\xFD[Pa\x04\xA5a\x1B\xF9V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02yV[4\x80\x15a\x04\xC5W_\x80\xFD[Pa\x02Va\x04\xD46`\x04aA\xB0V[a\x1C\x19V[4\x80\x15a\x04\xE4W_\x80\xFD[Pa\x04\x84a\x1DVV[4\x80\x15a\x04\xF8W_\x80\xFD[Pa\x02Va\x05\x076`\x04aC\x82V[a\x1D\xD7V[4\x80\x15a\x05\x17W_\x80\xFD[Pa\x02Va\x05&6`\x04aA\xB0V[a\x1F\x9FV[4\x80\x15a\x056W_\x80\xFD[Pa\x02\xD4a\x05E6`\x04aD(V[a \xFDV[4\x80\x15a\x05UW_\x80\xFD[Pa\x02Va\x05d6`\x04aC\x82V[a!;V[4\x80\x15a\x05tW_\x80\xFD[Pa\x036a#6V[4\x80\x15a\x05\x88W_\x80\xFD[Pa\x02l`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[4\x80\x15a\x05\xB8W_\x80\xFD[Pa\x02Va\x05\xC76`\x04aA\xB0V[a#JV[4\x80\x15a\x05\xD7W_\x80\xFD[Pa\x02Va\x05\xE66`\x04aA\xB0V[a$\x8AV[4\x80\x15a\x05\xF6W_\x80\xFD[Pa\x036a%\xD2V[4\x80\x15a\x06\nW_\x80\xFD[Pa\x02Va\x06\x196`\x04aF\x98V[a%\xF8V[4\x80\x15a\x06)W_\x80\xFD[Pa\x02\xD4a\x0686`\x04aC.V[a'\\V[4\x80\x15a\x06HW_\x80\xFD[Pa\x02Va\x06W6`\x04aC.V[a'fV[4\x80\x15a\x06gW_\x80\xFD[Pa\x036a(\x9FV[4\x80\x15a\x06{W_\x80\xFD[Pa\x036a\x06\x8A6`\x04aC.V[a(\xC5V[4\x80\x15a\x06\x9AW_\x80\xFD[Pa\x04\xA5a\x06\xA96`\x04aC.V[a(\xEAV[4\x80\x15a\x06\xB9W_\x80\xFD[Pa\x02\xD4a\x06\xC86`\x04aA\xB0V[a)QV[4\x80\x15a\x06\xD8W_\x80\xFD[Pa\x02Va\x06\xE76`\x04aC.V[a)\xA6V[4\x80\x15a\x06\xF7W_\x80\xFD[Pa\x07\x0Ba\x07\x066`\x04aC.V[a+\xEDV[`@Qa\x02y\x91\x90aGTV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07hW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x8C\x91\x90aG\xB6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07\xC4W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[_a\x07\xCDa-\xB0V[\x90Pa\x07\xD8\x83a-\xD4V[a\x08!`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o8:\xB164\xB1\xA22\xB1\xB9<\xB8:4\xB7\xB7`\x81\x1B\x81RP\x83\x83`\x01\x01_\x87\x81R` \x01\x90\x81R` \x01_ \x80T\x90Pa.\0V[_\x83\x81R`\x06\x82\x01` R`@\x90\x81\x90 \x83\x90UQ\x83\x90\x7F\xD5q\xBF\x83>AU;\xBE&\x0E\0\xB3\xAFz\x0E\x91\xAA\xFDl\xDC#\x8A\x80:\xA9\xAC\x0Es\xEF\xEDe\x90a\x08g\x90\x85\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPV[```@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mProtocolConfig`\x90\x1B\x81RPa\x08\xA6_a.wV[a\x08\xB0`\x02a.wV[a\x08\xB9_a.wV[`@Q` \x01a\x08\xCC\x94\x93\x92\x91\x90aG\xD1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[_\x80Q` aS\\\x839\x81Q\x91RT`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x16`\x01\x14a\t!W`@QcoOs\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80Q` aS\\\x839\x81Q\x91R\x80T`\x03\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\tWWP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a\tuW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x17`\x01`@\x1B\x17\x81U`\xF8`\x07a\t\xA6\x91\x1B`\x01aHvV[\x87\x10\x15a\t\xC9W`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x88\x90R`$\x01a\x07\xBBV[a\t\xD8`\x01`\xFB\x1B`\x01aHvV[\x86\x10\x15a\t\xFBW`@Qc\xA2%em`\xE0\x1B\x81R`\x04\x81\x01\x87\x90R`$\x01a\x07\xBBV[a\n\x10\x87\x87a\n\n\x87\x89aH\x9AV[\x86a/\x06V[P\x80T`\xFF`@\x1B\x19\x16\x81U`@Q`\x01`\x01`@\x1B\x03\x83\x16\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xB1W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xD5\x91\x90aG\xB6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0B\x08W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\xBBV[_a\x0B\x11a-\xB0V[`\x0B\x81\x01T\x90\x91P_a\x0B#\x82a/ZV[\x90P\x80\x82\x7F\x15\xAA\xAFG^\xF4\x07T?Qd\xF5}\xCFW\xF7\xF98\x16\xF5[\xAEw\xCA\t\xEF\xC4E\xBA@\xEE\xF7\x84\x86`\r\x01T`\x01Ca\x0B[\x91\x90aJ\tV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01`@Q\x80\x91\x03\x90\xA3PPPV[_\x80a\x0B\x88a-\xB0V[`\x0B\x81\x01T_\x90\x81R`\x03\x90\x91\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x90\x96\x16\x83R\x94\x90R\x92\x90\x92 T`\xFF\x16\x92\x91PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x0EW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C2\x91\x90aG\xB6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0CeW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\xBBV[_a\x0Cna-\xB0V[\x90P`\x01_\x83\x81R`\x0E\x83\x01` R`@\x90 T`\xFF\x16`\x03\x81\x11\x15a\x0C\x96Wa\x0C\x96aHNV[\x14a\x0C\xB7W`@Qc5\x86\xEF\xA1`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07\xBBV[a\x0C\xC0\x82a/\xABV[`@Q\x82\x90\x7Fu\xE1\x15\xB7\xF7k\xF2\x1D\n.B\xDA\x93\x04\xD9\xC3W\xB5LH\x9EZ\xF5\x9E\xD3\xC7\x0B|\xD4\x835\xFC\x90_\x90\xA2PPV[_\x80Q` aS\\\x839\x81Q\x91RT`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x16`\x01\x14a\r/W`@QcoOs\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80Q` aS\\\x839\x81Q\x91R\x80T`\x03\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\reWP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a\r\x83W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x17`\x01`@\x1B\x17\x81U_a\r\xADa-\xB0V[\x90P_a\r\xE1a\r\xC2`\x07`\xF8\x1B`\x01aHvV[a\r\xD1`\x01`\xFB\x1B`\x01aHvV[a\r\xDB\x8D\x8FaH\x9AV[\x8Ca/\x06V[\x90P`@Q\x80`@\x01`@R\x80C\x81R` \x01\x8C\x8C\x8C\x8C\x8C\x8C\x8C`@Q` \x01a\x0E\x11\x97\x96\x95\x94\x93\x92\x91\x90aKJV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x90\x92R_\x84\x81R`\x17\x86\x01\x82R\x91\x90\x91 \x82Q\x81U\x91\x01Q`\x01\x90\x91\x01U`\xF8`\x07\x90\x1B\x81\x7F Mk\x80\x12\x11T\xCD\x87\xD9\x9C\xF5Lc\x9A=\xD0\xA5;0\x84'p\x98\xDE\x97.\xBD\xD3Lk\xE9\x8D\x8D\x8D\x8D\x8D\x8D\x8D`@Qa\x0E\x89\x97\x96\x95\x94\x93\x92\x91\x90aKJV[`@Q\x80\x91\x03\x90\xA3PP\x80T`\xFF`@\x1B\x19\x16\x81U`@Q`\x01`\x01`@\x1B\x03\x83\x16\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01[`@Q\x80\x91\x03\x90\xA1PPPPPPPPPV[_\x80a\x0E\xF0a-\xB0V[`\x0B\x81\x01T_\x90\x81R`\t\x90\x91\x01` R`@\x90 T\x92\x91PPV[_a\x0F\x16\x82a-\xD4V[a\x0F\x1Ea-\xB0V[_\x92\x83R`\x07\x01` RP`@\x90 T\x90V[_\x80a\x0F;a-\xB0V[`\x0B\x81\x01T_\x90\x81R`\x06\x90\x91\x01` R`@\x90 T\x92\x91PPV[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01R``\x91\x81\x01\x82\x90R\x81\x81\x01\x91\x90\x91Ra\x0F\x82\x83a0fV[a\x0F\xA2W`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x07\xBBV[a\x0F\xAAa-\xB0V[_\x84\x81R`\x04\x91\x90\x91\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x85R\x90\x83R\x92\x81\x90 \x81Q`\x80\x81\x01\x83R\x81T\x85\x16\x81R`\x01\x82\x01T\x90\x94\x16\x92\x84\x01\x92\x90\x92R`\x02\x82\x01\x80T\x91\x84\x01\x91a\x10\x02\x90aM\x12V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10.\x90aM\x12V[\x80\x15a\x10yW\x80`\x1F\x10a\x10PWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10yV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10\\W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\x10\x92\x90aM\x12V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10\xBE\x90aM\x12V[\x80\x15a\x11\tW\x80`\x1F\x10a\x10\xE0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x11\tV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10\xECW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P[\x92\x91PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11kW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x8F\x91\x90aG\xB6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x11\xC2W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\xBBV[_a\x11\xCBa-\xB0V[\x90P`\x01_\x83\x81R`\x0F\x83\x01` R`@\x90 T`\xFF\x16`\x02\x81\x11\x15a\x11\xF3Wa\x11\xF3aHNV[\x14a\x12\x14W`@Qc\xA2%em`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07\xBBV[_\x82\x81R`\x10\x82\x01` R`@\x90 T`\x0B\x82\x01T\x81\x14a\x12RW`@Qc\xA6\x9D}[`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x82\x90R`D\x01a\x07\xBBV[a\x12[\x83a0\x9EV[`@Q\x83\x90\x82\x90\x7Fd@\xAA\xEA{$\x80\xB8$I\xC3\x17\xAAZ\x91h\xDFw\xEBi0\x8F\xF8\xF7\xC3\x98\n\x1A\xD8H\xB7\xDF\x90_\x90\xA3PPPV[_a\x12\x96\x82a0fV[a\x12\xB6W`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07\xBBV[a\x12\xBEa-\xB0V[_\x92\x83R`\x08\x01` RP`@\x90 T\x90V[_a\x12\xDB\x83a0fV[a\x12\xFBW`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x07\xBBV[a\x13\x03a-\xB0V[_\x93\x84R`\x02\x01` \x90\x81R`@\x80\x85 `\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x85R\x92\x90RP\x90 T`\xFF\x16\x90V[_a\x139\x82a-\xD4V[a\x13Aa-\xB0V[_\x92\x83R`\t\x01` RP`@\x90 T\x90V[_a\x13]a-\xB0V[\x90P`\x01_\x87\x81R`\x0F\x83\x01` R`@\x90 T`\xFF\x16`\x02\x81\x11\x15a\x13\x85Wa\x13\x85aHNV[\x14a\x13\xA6W`@Qc\xA2%em`\xE0\x1B\x81R`\x04\x81\x01\x87\x90R`$\x01a\x07\xBBV[_\x86\x81R`\x10\x82\x01` \x90\x81R`@\x80\x83 T\x80\x84R`\x02\x85\x01\x83R\x81\x84 3\x85R\x90\x92R\x90\x91 T`\xFF\x16a\x13\xF8W`@Qc\xA3\xF4\xAF\xEB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x88\x90R`D\x01a\x07\xBBV[`\x01_\x82\x81R`\x0E\x84\x01` R`@\x90 T`\xFF\x16`\x03\x81\x11\x15a\x14\x1EWa\x14\x1EaHNV[\x03a\x14?W`@Qc\x19b\xDC\xFB`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\xBBV[a\x14H\x81a0fV[a\x14hW`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\xBBV[_\x81\x81R`\x04\x83\x01` \x90\x81R`@\x80\x83 3\x84R\x82R\x80\x83 `\x01\x01T\x81Q`\x01`\xF9\x1B\x93\x81\x01\x93\x90\x93R`!\x83\x01\x85\x90R`A\x80\x84\x01\x8C\x90R\x82Q\x80\x85\x03\x90\x91\x01\x81R`a\x90\x93\x01\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16\x91\x90\x81\x88`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xD9Wa\x14\xD9aE(V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x15\x02W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x89\x81\x10\x15a\x16\x90W_a\x15J\x8C\x8C\x84\x81\x81\x10a\x15%Wa\x15%aMDV[\x90P` \x02\x81\x01\x90a\x157\x91\x90aMXV[a\x15E\x90`@\x81\x01\x90aMvV[a0\xD1V[\x90P_a\x15\xA4\x8D\x8D\x85\x81\x81\x10a\x15bWa\x15baMDV[\x90P` \x02\x81\x01\x90a\x15t\x91\x90aMXV[5\x8E\x8E\x86\x81\x81\x10a\x15\x87Wa\x15\x87aMDV[\x90P` \x02\x81\x01\x90a\x15\x99\x91\x90aMXV[` \x015\x84\x88a27V[\x90Pa\x15\xE2\x87\x82\x8F\x8F\x87\x81\x81\x10a\x15\xBDWa\x15\xBDaMDV[\x90P` \x02\x81\x01\x90a\x15\xCF\x91\x90aMXV[a\x15\xDD\x90``\x81\x01\x90aM\xBBV[a3\xB3V[\x8C\x8C\x84\x81\x81\x10a\x15\xF4Wa\x15\xF4aMDV[\x90P` \x02\x81\x01\x90a\x16\x06\x91\x90aMXV[5\x8D\x8D\x85\x81\x81\x10a\x16\x19Wa\x16\x19aMDV[\x90P` \x02\x81\x01\x90a\x16+\x91\x90aMXV[` \x015\x83`@Q` \x01a\x16S\x93\x92\x91\x90\x92\x83R` \x83\x01\x91\x90\x91R`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84\x84\x81Q\x81\x10a\x16{Wa\x16{aMDV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x15\x07V[P_\x87`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xAAWa\x16\xAAaE(V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\xD3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x88\x81\x10\x15a\x18PW_a\x17i\x8B\x8B\x84\x81\x81\x10a\x16\xF6Wa\x16\xF6aMDV[\x90P` \x02\x81\x01\x90a\x17\x08\x91\x90aMXV[5\x8C\x8C\x85\x81\x81\x10a\x17\x1BWa\x17\x1BaMDV[\x90P` \x02\x81\x01\x90a\x17-\x91\x90aMXV[` \x015\x8D\x8D\x86\x81\x81\x10a\x17CWa\x17CaMDV[\x90P` \x02\x81\x01\x90a\x17U\x91\x90aMXV[a\x17c\x90`@\x81\x01\x90aM\xBBV[\x89a48V[\x90Pa\x17\x82\x87\x82\x8D\x8D\x86\x81\x81\x10a\x15\xBDWa\x15\xBDaMDV[\x8A\x8A\x83\x81\x81\x10a\x17\x94Wa\x17\x94aMDV[\x90P` \x02\x81\x01\x90a\x17\xA6\x91\x90aMXV[5\x8B\x8B\x84\x81\x81\x10a\x17\xB9Wa\x17\xB9aMDV[\x90P` \x02\x81\x01\x90a\x17\xCB\x91\x90aMXV[` \x015\x8C\x8C\x85\x81\x81\x10a\x17\xE1Wa\x17\xE1aMDV[\x90P` \x02\x81\x01\x90a\x17\xF3\x91\x90aMXV[a\x18\x01\x90`@\x81\x01\x90aM\xBBV[`@Q` \x01a\x18\x14\x94\x93\x92\x91\x90aM\xFDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83\x83\x81Q\x81\x10a\x18<Wa\x18<aMDV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x16\xD8V[P\x81\x81`@Q` \x01a\x18d\x92\x91\x90aNVV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 _\x8F\x81R`\x12\x8B\x01\x84R\x82\x81 `\x01`\x01`\xA0\x1B\x03\x8A\x16\x82R\x90\x93R\x91 T\x90\x94P`\xFF\x16\x15\x92Pa\x18\xD6\x91PPW`@Qc$\xA0\xBB\x1B`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x81\x01\x8A\x90R`D\x01a\x07\xBBV[_\x89\x81R`\x12\x85\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x82R\x80\x83 \x80T`\xFF\x19\x16`\x01\x17\x90U\x8B\x83R`\x13\x87\x01\x82R\x80\x83 \x84\x84R\x90\x91R\x81 \x80T\x82\x90a\x19%\x90aNzV[\x91\x90P\x81\x90U\x90P\x82`\x01`\x01`\xA0\x1B\x03\x16\x8A\x7F~\xDAo\x85\xE2;{\x91\xC0\x19\xB0W\r\x02\xB6c`n\xF9\xD7E\x94\xF7\xE0\x1F\xCF\xBD\xB0\xF4\xE9T\xD5\x84`@Qa\x19i\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3_\x84\x81R`\x05\x86\x01` R`@\x90 T\x81\x03a\x1B0W_\x84\x81R`\x0E\x86\x01` R`@\x90 \x80T`\xFF\x19\x16`\x03\x17\x90U`\x0B\x85\x01\x84\x90Ua\x19\xB2\x8A\x85a4\xC5V[_\x84\x81R`\x01\x86\x01` R`@\x81 \x80T\x90\x91\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xDDWa\x19\xDDaE(V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A\x10W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x19\xFBW\x90P[P\x90P_[\x82T\x81\x10\x15a\x1A\xEBW\x82\x81\x81T\x81\x10a\x1A0Wa\x1A0aMDV[\x90_R` _ \x90`\x04\x02\x01`\x03\x01\x80Ta\x1AJ\x90aM\x12V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1Av\x90aM\x12V[\x80\x15a\x1A\xC1W\x80`\x1F\x10a\x1A\x98Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1A\xC1V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1A\xA4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x82\x82\x81Q\x81\x10a\x1A\xD8Wa\x1A\xD8aMDV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1A\x15V[P\x8B\x86\x7F\x1AT{B\xE7,\xD3\xDD\xA0Nj\xDC\xCD\"\0'l\xFE\xF0\x1F\xE2\x13\x8D\x07\xF3\xA7D\x0FAm8\xBC\x8D\x8D\x8D\x8D\x87`@Qa\x1B%\x95\x94\x93\x92\x91\x90aPLV[`@Q\x80\x91\x03\x90\xA3PP[PPPPPPPPPPV[a\x1BDa4\xFFV[a\x1BM\x82a5\xA5V[a\x1BW\x82\x82a6LV[PPV[_a\x1Bda7\x08V[P_\x80Q` aS<\x839\x81Q\x91R\x90V[``a\x1B\x81\x82a-\xD4V[a\x1B\x89a-\xB0V[`\x05\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1B\xEDW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x1B\xCFW[PPPPP\x90P\x91\x90PV[_\x80_a\x1C\x04a-\xB0V[\x90P\x80`\x0B\x01T\x92P\x80`\r\x01T\x91PP\x90\x91V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CiW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x8D\x91\x90aG\xB6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1C\xC0W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\xBBV[_a\x1C\xC9a-\xB0V[\x90Pa\x1C\xD4\x83a-\xD4V[a\x1D\x10`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bmpc`\xE8\x1B\x81RP\x83\x83`\x01\x01_\x87\x81R` \x01\x90\x81R` \x01_ \x80T\x90Pa.\0V[_\x83\x81R`\t\x82\x01` R`@\x90\x81\x90 \x83\x90UQ\x83\x90\x7F\x14\x8F\x9Cl\xB7}\x120k\x9FYe4\xD1Kz\xAE>O\x98\xA2\xDB\xE3\xCD\xB0~\xA4\x92Lw_\x12\x90a\x08g\x90\x85\x81R` \x01\x90V[``_a\x1Daa-\xB0V[\x90P\x80`\x05\x01_\x82`\x0B\x01T\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1D\xCCW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x1D\xAEW[PPPPP\x91PP\x90V[_\x80Q` aS\\\x839\x81Q\x91R\x80T`\x03\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\x1E\rWP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a\x1E+W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x17`\x01`@\x1B\x17\x81U_a\x1EUa-\xB0V[`\x01`\xFB\x1B`\x0C\x82\x01\x90\x81U\x81T`\x0B\x83\x01\x81\x90U_\x81\x81R`\x0E\x84\x01` R`@\x81 \x80T`\xFF\x19\x16`\x03\x17\x90U\x82T\x93\x94P\x90\x92\x90\x91\x90\x82\x90a\x1E\x99\x90aNzV[\x91\x82\x90UP\x90Pa\x1E\xAA\x81\x83a4\xC5V[`@Q\x80`@\x01`@R\x80C\x81R` \x01\x8D\x8D\x8D\x8D\x8D\x8D\x8D`@Q` \x01a\x1E\xD8\x97\x96\x95\x94\x93\x92\x91\x90aKJV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x90\x92R_\x85\x81R`\x17\x87\x01\x82R\x91\x90\x91 \x82Q\x81U\x91\x01Q`\x01\x90\x91\x01U`\xF8`\x07\x90\x1B\x82\x7F Mk\x80\x12\x11T\xCD\x87\xD9\x9C\xF5Lc\x9A=\xD0\xA5;0\x84'p\x98\xDE\x97.\xBD\xD3Lk\xE9\x8E\x8E\x8E\x8E\x8E\x8E\x8E`@Qa\x1FP\x97\x96\x95\x94\x93\x92\x91\x90aKJV[`@Q\x80\x91\x03\x90\xA3PP\x81T`\xFF`@\x1B\x19\x16\x82UP`@Q`\x01`\x01`@\x1B\x03\x83\x16\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01a\x0E\xD3V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\xEFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \x13\x91\x90aG\xB6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a FW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\xBBV[_a Oa-\xB0V[\x90P\x80`\x0B\x01T\x83\x14\x15\x80a jWPa h\x83a0fV[\x15[\x15a \x8BW`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x07\xBBV[`\x0C\x81\x01T\x80\x83\x11a \xBAW`@Qc\xE8\x12\x1FQ`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x82\x90R`D\x01a\x07\xBBV[`\x0C\x82\x01\x83\x90Ua \xCB\x83\x85a4\xC5V[`@Q\x83\x90\x85\x90\x7F\n\x1C$\xC2\xBA^n\x1B\x1A\x85\x85y^[x\x1E7*\xEE\x1D\xB6\x86$}\xACut\xC1\x0F\xD75\xA6\x90_\x90\xA3PPPPV[_a!\x07\x83a-\xD4V[a!\x0Fa-\xB0V[_\x93\x84R`\x03\x01` \x90\x81R`@\x80\x85 `\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x85R\x92\x90RP\x90 T`\xFF\x16\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\x8BW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xAF\x91\x90aG\xB6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a!\xE2W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\xBBV[_a!\xEBa-\xB0V[`\x0B\x81\x01T\x90\x91P_a\"\x07a\"\x01\x8A\x8CaH\x9AV[\x89a7QV[_\x81\x81R`\x0E\x85\x01` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U\x90Pa\",\x81a/ZV[P_\x82\x81R`\t\x84\x01` \x90\x81R`@\x80\x83 T`\x01\x87\x01\x90\x92R\x82 Ta\"T\x91\x90aJ\tV[\x90P_\x81\x11a\"dW`\x01a\"fV[\x80[\x84`\x14\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP`@Q\x80`@\x01`@R\x80C\x81R` \x01\x8C\x8C\x8C\x8C\x8C\x8C\x8C`@Q` \x01a\"\xAB\x97\x96\x95\x94\x93\x92\x91\x90aKJV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x90\x92R_\x85\x81R`\x17\x88\x01\x82R\x82\x90 \x83Q\x81U\x92\x01Q`\x01\x90\x92\x01\x91\x90\x91UQ\x83\x90\x83\x90\x7F Mk\x80\x12\x11T\xCD\x87\xD9\x9C\xF5Lc\x9A=\xD0\xA5;0\x84'p\x98\xDE\x97.\xBD\xD3Lk\xE9\x90a#!\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90aKJV[`@Q\x80\x91\x03\x90\xA3PPPPPPPPPPPV[_\x80a#@a-\xB0V[`\x0B\x01T\x92\x91PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#\x9AW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\xBE\x91\x90aG\xB6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a#\xF1W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\xBBV[_a#\xFAa-\xB0V[\x90Pa$\x05\x83a-\xD4V[a$D`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e5\xB6\xB9\xA3\xB2\xB7`\xD1\x1B\x81RP\x83\x83`\x01\x01_\x87\x81R` \x01\x90\x81R` \x01_ \x80T\x90Pa.\0V[_\x83\x81R`\x08\x82\x01` R`@\x90\x81\x90 \x83\x90UQ\x83\x90\x7F\xF2\x1C\xB3{\xE7\t\x14\x8A\xAB\xEB\xD2xT>b\xD1\xB1\xE6\xA4G\x7F\xB1\xCCC\xE0i\xD3\xEE\xB8\xC8\x7F\x90\x90a\x08g\x90\x85\x81R` \x01\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\xDAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\xFE\x91\x90aG\xB6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a%1W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\xBBV[_a%:a-\xB0V[\x90Pa%E\x83a-\xD4V[a%\x8C`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m:\xB9\xB2\xB9\"2\xB1\xB9<\xB8:4\xB7\xB7`\x91\x1B\x81RP\x83\x83`\x01\x01_\x87\x81R` \x01\x90\x81R` \x01_ \x80T\x90Pa.\0V[_\x83\x81R`\x07\x82\x01` R`@\x90\x81\x90 \x83\x90UQ\x83\x90\x7F\x90\xF1\x91\x84\x93\x83\x1C\x1Ba3H\x97C\x103\x84\xC5`\x0E\xAEyn\xB3LQ\xEAO+\xAA\xFAO\x94\x90a\x08g\x90\x85\x81R` \x01\x90V[_\x80a%\xDCa-\xB0V[`\x0B\x81\x01T_\x90\x81R`\x08\x90\x91\x01` R`@\x90 T\x92\x91PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&HW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&l\x91\x90aG\xB6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a&\x9FW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\xBBV[_a&\xA8a-\xB0V[`\x0B\x81\x01T\x90\x91P\x80\x8B\x11a&\xDAW`@Qc\xEF\xD5_g`\xE0\x1B\x81R`\x04\x81\x01\x8C\x90R`$\x81\x01\x82\x90R`D\x01a\x07\xBBV[`\x0C\x82\x01T\x80\x8B\x11a'\tW`@Qc\xE8\x12\x1FQ`\xE0\x1B\x81R`\x04\x81\x01\x8C\x90R`$\x81\x01\x82\x90R`D\x01a\x07\xBBV[a'\x1E\x8C\x8Ca'\x18\x8C\x8EaH\x9AV[\x8Ba/\x06V[P\x8A\x8C\x7F*\xC6\x8Fx\xF4\xCC\xDEv\xB6I\x06\x02m\x01\xFF<B@>\xB7\xEE\xF8o\xE7\x88GJ#&}d\xCF\x8C\x8C\x8C\x8C\x8C\x8C\x8C`@Qa\x1B%\x97\x96\x95\x94\x93\x92\x91\x90aKJV[_a\x11\x15\x82a7vV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'\xB6W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\xDA\x91\x90aG\xB6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a(\rW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\xBBV[_a(\x16a-\xB0V[\x90P\x80`\x0B\x01T\x82\x03a(?W`@Qc\"\xCA\xFEq`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07\xBBV[a(H\x82a0fV[a(hW`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07\xBBV[a(q\x82a/\xABV[`@Q\x82\x90\x7F\xDA\x07]\t\x19\x8D ~:\x91\x8DK\x8D\xFC\x87\xDF-`\xA0\x0B\xE7\x03\xFD9\xEA\xAC\x90\x96-\xA0\xB7\xF0\x90_\x90\xA2PPV[_\x80a(\xA9a-\xB0V[`\x0B\x81\x01T_\x90\x81R`\x07\x90\x91\x01` R`@\x90 T\x92\x91PPV[_a(\xCF\x82a-\xD4V[a(\xD7a-\xB0V[_\x92\x83R`\x06\x01` RP`@\x90 T\x90V[_\x80a(\xF5\x83a7\xC0V[a)\x15W`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x07\xBBV[_a)\x1Ea-\xB0V[_\x94\x85R`\x17\x01` \x90\x81R`@\x94\x85\x90 \x85Q\x80\x87\x01\x90\x96R\x80T\x80\x87R`\x01\x90\x91\x01T\x95\x90\x91\x01\x85\x90R\x94\x92PPPV[_\x80a)[a-\xB0V[\x90P`\x02_\x84\x81R`\x0F\x83\x01` R`@\x90 T`\xFF\x16`\x02\x81\x11\x15a)\x83Wa)\x83aHNV[\x14\x80\x15a)\x9EWP_\x83\x81R`\x10\x82\x01` R`@\x90 T\x84\x14[\x94\x93PPPPV[_a)\xAFa-\xB0V[\x90P`\x01_\x83\x81R`\x0E\x83\x01` R`@\x90 T`\xFF\x16`\x03\x81\x11\x15a)\xD7Wa)\xD7aHNV[\x14a)\xF8W`@Qc5\x86\xEF\xA1`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07\xBBV[`\x0B\x81\x01T_\x81\x81R`\x02\x83\x01` \x81\x81R`@\x80\x84 3\x80\x86R\x90\x83R\x81\x85 T\x88\x86R\x93\x83R\x81\x85 \x90\x85R\x90\x91R\x90\x91 T`\xFF\x91\x82\x16\x91\x16\x81\x15\x80\x15a*@WP\x80\x15[\x15a*gW`@Qc\x17\x03\xBF\x1D`\xE3\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01a\x07\xBBV[_\x85\x81R`\x11\x85\x01` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T`\xFF\x16\x15a*\xACW`@Qc\x0CK\x0B\x99`\xE3\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01a\x07\xBBV[_\x85\x81R`\x11\x85\x01` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U\x81\x15a*\xF9W_\x85\x81R`\x16\x85\x01` R`@\x81 \x80T\x90\x91\x90a*\xF4\x90aNzV[\x90\x91UP[\x80\x15a+!W_\x85\x81R`\x15\x85\x01` R`@\x81 \x80T\x90\x91\x90a+\x1C\x90aNzV[\x90\x91UP[`@\x80Q\x83\x15\x15\x81R\x82\x15\x15` \x82\x01R3\x91\x87\x91\x7F\xB7\x9CH\x006\x95\xB6\xEB\xE5U\xAF\xA3o\xAD\x07\x1D\xEE\xEEu\xEB7\x18\xADc\xDEV!\xD3[\xA4KO\x91\x01`@Q\x80\x91\x03\x90\xA3a+j\x85a8\x07V[\x15a+\xE6W_\x85\x81R`\x0E\x85\x01` R`@\x90 \x80T`\xFF\x19\x16`\x02\x17\x90U`\x0C\x84\x01T`\r\x85\x01T\x81\x90\x87\x90\x7F\x15\xAA\xAFG^\xF4\x07T?Qd\xF5}\xCFW\xF7\xF98\x16\xF5[\xAEw\xCA\t\xEF\xC4E\xBA@\xEE\xF7\x90\x87\x90a+\xC6`\x01CaJ\tV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01`@Q\x80\x91\x03\x90\xA3P[PPPPPV[``a+\xF8\x82a-\xD4V[a,\0a-\xB0V[`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a-\xA5W_\x84\x81R` \x90\x81\x90 `@\x80Q`\x80\x81\x01\x82R`\x04\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x84R`\x01\x82\x01T\x16\x93\x83\x01\x93\x90\x93R`\x02\x83\x01\x80T\x92\x93\x92\x91\x84\x01\x91a,\x86\x90aM\x12V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta,\xB2\x90aM\x12V[\x80\x15a,\xFDW\x80`\x1F\x10a,\xD4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a,\xFDV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a,\xE0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta-\x16\x90aM\x12V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta-B\x90aM\x12V[\x80\x15a-\x8DW\x80`\x1F\x10a-dWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a-\x8DV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a-pW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a,1V[PPPP\x90P\x91\x90PV[\x7F\x80\xF3XZ\xF8h\x06\xC5wC\x03\xB0l\x1E\xE6@\xAA\x83\xB6\xEF>E\xDFI\xBB&\xC8RE\0\xC2\0\x90V[a-\xDD\x81a7vV[a-\xFDW`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\xBBV[PV[\x81_\x03a.\"W\x82`@Qc\x1B_\xDB\x07`\xE1\x1B\x81R`\x04\x01a\x07\xBB\x91\x90aB\x1DV[`\xFF\x82\x11\x15a.KW`@Qc\"\xBAR\xDB`\xE0\x1B\x81Ra\x07\xBB\x90\x84\x90\x84\x90`\xFF\x90`\x04\x01aQYV[\x80\x82\x11\x15a.rW\x82\x82\x82`@Qc\xCA\xA8\x14\xA3`\xE0\x1B\x81R`\x04\x01a\x07\xBB\x93\x92\x91\x90aQYV[PPPV[``_a.\x83\x83a8aV[`\x01\x01\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a.\xA1Wa.\xA1aE(V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a.\xCBW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[_\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a.\xD5WP\x93\x92PPPV[_\x80a/\x10a-\xB0V[\x90Pa/\x1D\x86\x85\x85a98V[_\x81\x81R`\x0E\x83\x01` R`@\x90 \x80T`\xFF\x19\x16`\x03\x17\x90U`\x0B\x82\x01\x81\x90U`\x0C\x82\x01\x86\x90U\x91Pa/Q\x85\x83a4\xC5V[P\x94\x93PPPPV[_\x80a/da-\xB0V[\x90P\x80`\x0C\x01_\x81Ta/v\x90aNzV[\x91\x82\x90UP_\x81\x81R`\x0F\x83\x01` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16`\x01\x17\x90U`\x10\x90\x94\x01\x90R\x91\x90\x91 \x92\x90\x92UP\x90V[_a/\xB4a-\xB0V[_\x83\x81R`\n\x82\x01` \x90\x81R`@\x80\x83 \x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x0E\x86\x01\x84R\x82\x85 \x80T\x90\x91\x16\x90U`\x0C\x85\x01T\x80\x85R`\x0F\x86\x01\x90\x93R\x92 T\x92\x93P\x91`\xFF\x16`\x02\x81\x11\x15a0\x0FWa0\x0FaHNV[\x14\x80\x15a0*WP_\x81\x81R`\x10\x83\x01` R`@\x90 T\x83\x14[\x15a08Wa08\x81a0\x9EV[P_\x91\x82R`\x14\x81\x01` \x90\x81R`@\x80\x84 \x84\x90U`\x15\x83\x01\x82R\x80\x84 \x84\x90U`\x16\x90\x92\x01\x90R\x81 UV[_\x80a0pa-\xB0V[\x90Pa0{\x83a7\xC0V[\x80\x15a0\x97WP_\x83\x81R`\n\x82\x01` R`@\x90 T`\xFF\x16\x15[\x93\x92PPPV[_a0\xA7a-\xB0V[_\x92\x83R`\x0F\x81\x01` \x90\x81R`@\x80\x85 \x80T`\xFF\x19\x16\x90U`\x10\x90\x92\x01\x90R\x82 \x91\x90\x91UPV[_\x80\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a0\xEBWa0\xEBaE(V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a1\x14W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15a2\x06W\x7F\xDD\xD1\x08w.j8\x99\xFE\xB0M\x14\x8A\xE9\x15\xCB\xE3\xEB^\xBD &\x88\x08\x03\x99\xE9\x92\x1A\xC3ak\x85\x85\x83\x81\x81\x10a1TWa1TaMDV[\x90P` \x02\x81\x01\x90a1f\x91\x90aQ}V[a1t\x90` \x81\x01\x90aQ\x91V[\x86\x86\x84\x81\x81\x10a1\x86Wa1\x86aMDV[\x90P` \x02\x81\x01\x90a1\x98\x91\x90aQ}V[a1\xA6\x90` \x81\x01\x90aM\xBBV[`@Qa1\xB4\x92\x91\x90aQ\xAAV[`@Q\x90\x81\x90\x03\x81 a1\xCB\x93\x92\x91` \x01aQ\xB9V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x82\x81Q\x81\x10a1\xF3Wa1\xF3aMDV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a1\x19V[P\x80`@Q` \x01a2\x18\x91\x90aQ\xDBV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x92\x91PPV[\x80Q` \x80\x83\x01\x91\x90\x91 `@\x80Q\x7F\xBD\x14\x83[\xB4\xAE\x13\xC7\x8E\xCB\x88\xDE\xD2\xC37\x03%\xF3\x9E`\x06\xEB\x94\xFFE\xE9_\x98\xE4\xC8Z*\x93\x81\x01\x93\x90\x93R\x82\x01\x86\x90R``\x82\x01\x85\x90R`\x80\x82\x01\x84\x90R`\xA0\x82\x01R_\x90a3\xAA\x90`\xC0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@\x80Q\x80\x82\x01\x82R`\x0E\x81RmProtocolConfig`\x90\x1B` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\x01\x81R`1`\xF8\x1B\x90\x82\x01R\x81Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81\x83\x01R\x7F\xA3\xDE\x18\x80\xCF\x08>\x83\x18\xB7zye\xD0-\xD9v^\x85\xA4\x8EA\x8ADc\xAFz\rW\xB4\xB3\xEE\x81\x84\x01R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xC0\x82\x01\x84R\x80Q\x90\x83\x01 a\x19\x01`\xF0\x1B`\xE0\x83\x01R`\xE2\x82\x01Ra\x01\x02\x80\x82\x01\x94\x90\x94R\x82Q\x80\x82\x03\x90\x94\x01\x84Ra\x01\"\x01\x90\x91R\x81Q\x91\x01 \x90V[\x95\x94PPPPPV[_a3\xF3\x84\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa;W\x92PPPV[\x90P\x84`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a+\xE6W`@Qcx\xB9\xAD\xA3`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R3`$\x82\x01R`D\x01a\x07\xBBV[_a4\xBB\x7F\xA2d\xB3\x18\xE9P\x800\n?\x06\xA6ej\x8E\x7F\xE2O\x99\x03\xF0\xE6\xBC\xCA0~\xFB\xE3\x9CLN\t\x87\x87\x87\x87`@Q` \x01a4r\x92\x91\x90aQ\xAAV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x89Q\x8A\x83\x01 \x91\x84\x01\x96\x90\x96R\x90\x82\x01\x93\x90\x93R``\x81\x01\x91\x90\x91R`\x80\x81\x01\x92\x90\x92R`\xA0\x82\x01R`\xC0\x01a2\x90V[\x96\x95PPPPPPV[_a4\xCEa-\xB0V[_\x84\x81R`\x0F\x82\x01` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16`\x02\x17\x90U`\x10\x84\x01\x90\x91R\x90 \x92\x90\x92UP`\r\x01UV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a5\x85WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a5y_\x80Q` aS<\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a5\xA3W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5\xF5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6\x19\x91\x90aG\xB6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a-\xFDW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\xBBV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a6\xA6WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra6\xA3\x91\x81\x01\x90aR\x10V[`\x01[a6\xCEW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x07\xBBV[_\x80Q` aS<\x839\x81Q\x91R\x81\x14a6\xFEW`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\xBBV[a.r\x83\x83a;\x7FV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a5\xA3W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80a7[a-\xB0V[\x80T\x90\x91Pa)\x9E\x90a7o\x90`\x01aHvV[\x85\x85a98V[_\x80a7\x80a-\xB0V[\x90Pa7\x8B\x83a0fV[\x80\x15a0\x97WP`\x03_\x84\x81R`\x0E\x83\x01` R`@\x90 T`\xFF\x16`\x03\x81\x11\x15a7\xB8Wa7\xB8aHNV[\x14\x93\x92PPPV[_\x80a7\xCAa-\xB0V[\x90Pa7\xDB`\x07`\xF8\x1B`\x01aHvV[\x83\x10\x15\x80\x15a7\xEBWP\x80T\x83\x11\x15[\x80\x15a0\x97WP_\x92\x83R`\x01\x01` RP`@\x90 T\x15\x15\x90V[_\x80a8\x11a-\xB0V[_\x84\x81R`\x01\x82\x01` \x90\x81R`@\x80\x83 T`\x15\x85\x01\x90\x92R\x90\x91 T\x91\x92P\x14\x80\x15a0\x97WP_\x83\x81R`\x14\x82\x01` \x90\x81R`@\x80\x83 T`\x16\x85\x01\x90\x92R\x90\x91 T\x10\x15\x93\x92PPPV[_\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a8\x9FWr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a8\xCBWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a8\xE9Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a9\x01Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a9\x15Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a9'W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x11\x15W`\x01\x01\x92\x91PPV[_\x82Q_\x03a9YW`@Qb\x1A25`\xE6\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82Q`\xFF\x10\x15a9\x89W\x82Q`@Qc\x02\xD4\xE4\xEF`\xE3\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\xFF`$\x82\x01R`D\x01a\x07\xBBV[a9\xC0`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o8:\xB164\xB1\xA22\xB1\xB9<\xB8:4\xB7\xB7`\x81\x1B\x81RP\x83_\x015\x85Qa.\0V[a9\xF6`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m:\xB9\xB2\xB9\"2\xB1\xB9<\xB8:4\xB7\xB7`\x91\x1B\x81RP\x83` \x015\x85Qa.\0V[a:$`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e5\xB6\xB9\xA3\xB2\xB7`\xD1\x1B\x81RP\x83`@\x015\x85Qa.\0V[a:O`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bmpc`\xE8\x1B\x81RP\x83``\x015\x85Qa.\0V[_a:Xa-\xB0V[\x80T\x90\x91P\x85\x11a:\x89W\x80T`@Qc\xEF\xD5_g`\xE0\x1B\x81Ra\x07\xBB\x91\x87\x91`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[\x84\x81U\x84\x91P_[\x84Q\x81\x10\x15a;\x0BW_\x85\x82\x81Q\x81\x10a:\xADWa:\xADaMDV[` \x02` \x01\x01Q\x90Pa;\x02\x84`@Q\x80`\x80\x01`@R\x80\x84_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84`@\x01Q\x81R` \x01\x84``\x01Q\x81RPa;\xD4V[P`\x01\x01a:\x91V[P_\x82\x81R`\x06\x82\x01` \x90\x81R`@\x80\x83 \x865\x90U`\x07\x84\x01\x82R\x80\x83 \x82\x87\x015\x90U`\x08\x84\x01\x82R\x80\x83 \x81\x87\x015\x90U`\t\x90\x93\x01\x90R ``\x90\x92\x015\x90\x91U\x92\x91PPV[_\x80_\x80a;e\x86\x86a>wV[\x92P\x92P\x92Pa;u\x82\x82a>\xC0V[P\x90\x94\x93PPPPV[a;\x88\x82a?xV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a;\xCCWa.r\x82\x82a?\xDBV[a\x1BWa@DV[_a;\xDDa-\xB0V[\x82Q\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16a<\x08W`@QcB3@%`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16a<3W`@Qc-\xEC\xCFM`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x83\x81R`\x02\x82\x01` \x90\x81R`@\x80\x83 \x85Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15a<\x87W\x81Q`@Qc\r\x18\xC4\xFF`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01a\x07\xBBV[_\x83\x81R`\x03\x82\x01` \x90\x81R`@\x80\x83 \x85\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15a<\xE0W` \x82\x01Q`@Qc\xF5\x1A\xF6\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01a\x07\xBBV[_\x83\x81R`\x01\x82\x81\x01` \x90\x81R`@\x80\x84 \x80T\x80\x85\x01\x82U\x90\x85R\x93\x82\x90 \x86Q`\x04\x90\x95\x02\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x17\x82U\x92\x87\x01Q\x93\x81\x01\x80T\x90\x93\x16\x93\x90\x94\x16\x92\x90\x92\x17\x90U\x83\x01Q\x83\x91\x90`\x02\x82\x01\x90a=P\x90\x82aRkV[P``\x82\x01Q`\x03\x82\x01\x90a=e\x90\x82aRkV[PPP_\x83\x81R`\x02\x80\x83\x01` \x90\x81R`@\x80\x84 \x86Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x86R\x90\x83R\x81\x85 \x80T`\xFF\x19\x90\x81\x16`\x01\x90\x81\x17\x90\x92U\x89\x87R`\x03\x88\x01\x85R\x83\x87 \x89\x86\x01\x80Q\x85\x16\x89R\x90\x86R\x84\x88 \x80T\x90\x92\x16\x83\x17\x90\x91U\x89\x87R`\x04\x88\x01\x85R\x83\x87 \x89Q\x84\x16\x88R\x90\x94R\x94\x82\x90 \x87Q\x81T\x90\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x82U\x93Q\x95\x81\x01\x80T\x96\x90\x92\x16\x95\x90\x93\x16\x94\x90\x94\x17\x90\x93U\x91\x84\x01Q\x84\x92\x91\x82\x01\x90a>\x1E\x90\x82aRkV[P``\x82\x01Q`\x03\x82\x01\x90a>3\x90\x82aRkV[PPP_\x92\x83R`\x05\x01` \x90\x81R`@\x83 \x91\x81\x01Q\x82T`\x01\x81\x01\x84U\x92\x84R\x92 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[_\x80_\x83Q`A\x03a>\xAEW` \x84\x01Q`@\x85\x01Q``\x86\x01Q_\x1Aa>\xA0\x88\x82\x85\x85a@cV[\x95P\x95P\x95PPPPa>\xB9V[PP\x81Q_\x91P`\x02\x90[\x92P\x92P\x92V[_\x82`\x03\x81\x11\x15a>\xD3Wa>\xD3aHNV[\x03a>\xDCWPPV[`\x01\x82`\x03\x81\x11\x15a>\xF0Wa>\xF0aHNV[\x03a?\x0EW`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a?\"Wa?\"aHNV[\x03a?CW`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\xBBV[`\x03\x82`\x03\x81\x11\x15a?WWa?WaHNV[\x03a\x1BWW`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\xBBV[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a?\xADW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x07\xBBV[_\x80Q` aS<\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``_\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa?\xF7\x91\x90aS*V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a@/W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a@4V[``\x91P[P\x91P\x91Pa3\xAA\x85\x83\x83aA+V[4\x15a5\xA3W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a@\x9CWP_\x91P`\x03\x90P\x82aA!V[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a@\xEDW=_\x80>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aA\x18WP_\x92P`\x01\x91P\x82\x90PaA!V[\x92P_\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[``\x82aA@WaA;\x82aA\x87V[a0\x97V[\x81Q\x15\x80\x15aAWWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15aA\x80W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x07\xBBV[P\x92\x91PPV[\x80Q\x15aA\x97W\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80`@\x83\x85\x03\x12\x15aA\xC1W_\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[_[\x83\x81\x10\x15aA\xEAW\x81\x81\x01Q\x83\x82\x01R` \x01aA\xD2V[PP_\x91\x01RV[_\x81Q\x80\x84RaB\t\x81` \x86\x01` \x86\x01aA\xD0V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R_a0\x97` \x83\x01\x84aA\xF2V[_\x80\x83`\x1F\x84\x01\x12aB?W_\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aBUW_\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aBoW_\x80\xFD[\x92P\x92\x90PV[_`\x80\x82\x84\x03\x12\x15aB\x86W_\x80\xFD[P\x91\x90PV[_\x80_\x80_`\xE0\x86\x88\x03\x12\x15aB\xA0W_\x80\xFD[\x855\x94P` \x86\x015\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aB\xC3W_\x80\xFD[aB\xCF\x88\x82\x89\x01aB/V[\x90\x94P\x92PaB\xE3\x90P\x87``\x88\x01aBvV[\x90P\x92\x95P\x92\x95\x90\x93PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a-\xFDW_\x80\xFD[\x805aC\x0E\x81aB\xEFV[\x91\x90PV[_` \x82\x84\x03\x12\x15aC#W_\x80\xFD[\x815a0\x97\x81aB\xEFV[_` \x82\x84\x03\x12\x15aC>W_\x80\xFD[P5\x91\x90PV[_\x80\x83`\x1F\x84\x01\x12aCUW_\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aCkW_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aBoW_\x80\xFD[_\x80_\x80_\x80_`\xE0\x88\x8A\x03\x12\x15aC\x98W_\x80\xFD[\x875`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aC\xAEW_\x80\xFD[aC\xBA\x8B\x83\x8C\x01aB/V[\x90\x99P\x97P\x87\x91PaC\xCF\x8B` \x8C\x01aBvV[\x96P`\xA0\x8A\x015\x91P\x80\x82\x11\x15aC\xE4W_\x80\xFD[aC\xF0\x8B\x83\x8C\x01aCEV[\x90\x96P\x94P`\xC0\x8A\x015\x91P\x80\x82\x11\x15aD\x08W_\x80\xFD[PaD\x15\x8A\x82\x8B\x01aB/V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[_\x80`@\x83\x85\x03\x12\x15aD9W_\x80\xFD[\x825\x91P` \x83\x015aDK\x81aB\xEFV[\x80\x91PP\x92P\x92\x90PV[_`\x01\x80`\xA0\x1B\x03\x80\x83Q\x16\x84R\x80` \x84\x01Q\x16` \x85\x01RP`@\x82\x01Q`\x80`@\x85\x01RaD\x8A`\x80\x85\x01\x82aA\xF2V[\x90P``\x83\x01Q\x84\x82\x03``\x86\x01Ra3\xAA\x82\x82aA\xF2V[` \x81R_a0\x97` \x83\x01\x84aDVV[_\x80_\x80_``\x86\x88\x03\x12\x15aD\xC9W_\x80\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aD\xE6W_\x80\xFD[aD\xF2\x89\x83\x8A\x01aB/V[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15aE\nW_\x80\xFD[PaE\x17\x88\x82\x89\x01aB/V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Qa\x01\0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aE_WaE_aE(V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aE\x8DWaE\x8DaE(V[`@R\x91\x90PV[_\x82`\x1F\x83\x01\x12aE\xA4W_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aE\xBDWaE\xBDaE(V[aE\xD0`\x1F\x82\x01`\x1F\x19\x16` \x01aEeV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aE\xE4W_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15aF\x11W_\x80\xFD[\x825aF\x1C\x81aB\xEFV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aF6W_\x80\xFD[aFB\x85\x82\x86\x01aE\x95V[\x91PP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aF\x8CW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aFgV[P\x90\x96\x95PPPPPPV[_\x80_\x80_\x80_\x80_a\x01 \x8A\x8C\x03\x12\x15aF\xB1W_\x80\xFD[\x895\x98P` \x8A\x015\x97P`@\x8A\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aF\xD5W_\x80\xFD[aF\xE1\x8D\x83\x8E\x01aB/V[\x90\x99P\x97P\x87\x91PaF\xF6\x8D``\x8E\x01aBvV[\x96P`\xE0\x8C\x015\x91P\x80\x82\x11\x15aG\x0BW_\x80\xFD[aG\x17\x8D\x83\x8E\x01aCEV[\x90\x96P\x94Pa\x01\0\x8C\x015\x91P\x80\x82\x11\x15aG0W_\x80\xFD[PaG=\x8C\x82\x8D\x01aB/V[\x91P\x80\x93PP\x80\x91PP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[_` \x80\x83\x01` \x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P` \x87\x01_[\x82\x81\x10\x15aG\xA9W`?\x19\x88\x86\x03\x01\x84RaG\x97\x85\x83QaDVV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01aG{V[P\x92\x97\x96PPPPPPPV[_` \x82\x84\x03\x12\x15aG\xC6W_\x80\xFD[\x81Qa0\x97\x81aB\xEFV[_\x85QaG\xE2\x81\x84` \x8A\x01aA\xD0V[a\x10;`\xF1\x1B\x90\x83\x01\x90\x81R\x85QaH\x01\x81`\x02\x84\x01` \x8A\x01aA\xD0V[\x80\x82\x01\x91PP`\x17`\xF9\x1B\x80`\x02\x83\x01R\x85QaH%\x81`\x03\x85\x01` \x8A\x01aA\xD0V[`\x03\x92\x01\x91\x82\x01R\x83QaH@\x81`\x04\x84\x01` \x88\x01aA\xD0V[\x01`\x04\x01\x96\x95PPPPPPV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x11\x15Wa\x11\x15aHbV[\x805`\x03\x81\x90\x0B\x81\x14aC\x0EW_\x80\xFD[_`\x01`\x01`@\x1B\x03\x80\x84\x11\x15aH\xB3WaH\xB3aE(V[\x83`\x05\x1B` aH\xC4\x81\x83\x01aEeV[\x86\x81R\x91\x85\x01\x91\x81\x81\x01\x906\x84\x11\x15aH\xDBW_\x80\xFD[\x86[\x84\x81\x10\x15aI\xFDW\x805\x86\x81\x11\x15aH\xF3W_\x80\xFD[\x88\x01a\x01\x006\x82\x90\x03\x12\x15aI\x06W_\x80\xFD[aI\x0EaE<V[aI\x17\x82aC\x03V[\x81RaI$\x86\x83\x01aC\x03V[\x86\x82\x01R`@\x80\x83\x015\x89\x81\x11\x15aI:W_\x80\xFD[aIF6\x82\x86\x01aE\x95V[\x82\x84\x01RPP``\x80\x83\x015\x89\x81\x11\x15aI^W_\x80\xFD[aIj6\x82\x86\x01aE\x95V[\x82\x84\x01RPP`\x80aI}\x81\x84\x01aH\x89V[\x90\x82\x01R`\xA0\x82\x81\x015\x89\x81\x11\x15aI\x93W_\x80\xFD[aI\x9F6\x82\x86\x01aE\x95V[\x82\x84\x01RPP`\xC0\x80\x83\x015\x89\x81\x11\x15aI\xB7W_\x80\xFD[aI\xC36\x82\x86\x01aE\x95V[\x82\x84\x01RPP`\xE0\x80\x83\x015\x89\x81\x11\x15aI\xDBW_\x80\xFD[aI\xE76\x82\x86\x01aE\x95V[\x91\x83\x01\x91\x90\x91RP\x84RP\x91\x83\x01\x91\x83\x01aH\xDDV[P\x97\x96PPPPPPPV[\x81\x81\x03\x81\x81\x11\x15a\x11\x15Wa\x11\x15aHbV[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aJ1W_\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aJOW_\x80\xFD[\x806\x03\x82\x13\x15aBoW_\x80\xFD[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[_\x83\x83\x85R` \x80\x86\x01\x95P\x80\x85`\x05\x1B\x83\x01\x01\x84_[\x87\x81\x10\x15aK=W\x84\x83\x03`\x1F\x19\x01\x89R\x8156\x88\x90\x03`^\x19\x01\x81\x12aJ\xC1W_\x80\xFD[\x87\x01``aJ\xCF\x82\x80aJ\x1CV[\x82\x87RaJ\xDF\x83\x88\x01\x82\x84aJ]V[\x92PPPaJ\xEF\x86\x83\x01\x83aJ\x1CV[\x86\x83\x03\x88\x88\x01RaK\x01\x83\x82\x84aJ]V[\x92PPP`@aK\x13\x81\x84\x01\x84aJ\x1CV[\x93P\x86\x83\x03\x82\x88\x01RaK'\x83\x85\x83aJ]V[\x9C\x88\x01\x9C\x96PPP\x92\x85\x01\x92PP`\x01\x01aJ\x9CV[P\x90\x97\x96PPPPPPPV[`\xE0\x80\x82R\x81\x01\x87\x90R_a\x01\0\x80\x83\x01`\x05\x8A\x90\x1B\x84\x01\x82\x01\x8B\x84[\x8C\x81\x10\x15aL\xAAW\x86\x83\x03`\xFF\x19\x01\x84R\x8156\x8F\x90\x03`\xFE\x19\x01\x81\x12aK\x8CW_\x80\xFD[\x8E\x01aK\xA8\x84aK\x9B\x83aC\x03V[`\x01`\x01`\xA0\x1B\x03\x16\x90RV[` aK\xB5\x81\x83\x01aC\x03V[`\x01`\x01`\xA0\x1B\x03\x16\x81\x86\x01R`@aK\xD0\x83\x82\x01\x84aJ\x1CV[\x89\x83\x89\x01RaK\xE2\x8A\x89\x01\x82\x84aJ]V[\x92PPP``aK\xF4\x81\x85\x01\x85aJ\x1CV[\x88\x84\x03\x83\x8A\x01RaL\x06\x84\x82\x84aJ]V[\x93PPPP`\x80aL\x18\x81\x85\x01aH\x89V[aL&\x82\x89\x01\x82`\x03\x0B\x90RV[PP`\xA0aL6\x81\x85\x01\x85aJ\x1CV[\x88\x84\x03\x83\x8A\x01RaLH\x84\x82\x84aJ]V[\x93PPPP`\xC0aL[\x81\x85\x01\x85aJ\x1CV[\x88\x84\x03\x83\x8A\x01RaLm\x84\x82\x84aJ]V[\x93PPPPaL\x7F`\xE0\x84\x01\x84aJ\x1CV[\x93P\x86\x82\x03`\xE0\x88\x01RaL\x94\x82\x85\x83aJ]V[\x97\x83\x01\x97\x96PPP\x92\x90\x92\x01\x91P`\x01\x01aKgV[PPaL\xDA` \x86\x01\x8B\x805\x82R` \x81\x015` \x83\x01R`@\x81\x015`@\x83\x01R``\x81\x015``\x83\x01RPPV[\x84\x81\x03`\xA0\x86\x01RaL\xED\x81\x89\x8BaJ]V[\x92PPP\x82\x81\x03`\xC0\x84\x01RaM\x04\x81\x85\x87aJ\x85V[\x9A\x99PPPPPPPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80aM&W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aB\x86WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`~\x19\x836\x03\x01\x81\x12aMlW_\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aM\x8BW_\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aM\xA4W_\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aBoW_\x80\xFD[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aM\xD0W_\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aM\xE9W_\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aBoW_\x80\xFD[\x84\x81R\x83` \x82\x01R```@\x82\x01R_a4\xBB``\x83\x01\x84\x86aJ]V[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15aNKW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aN/V[P\x94\x95\x94PPPPPV[`@\x81R_aNh`@\x83\x01\x85aN\x1CV[\x82\x81\x03` \x84\x01Ra3\xAA\x81\x85aN\x1CV[_`\x01\x82\x01aN\x8BWaN\x8BaHbV[P`\x01\x01\x90V[\x805`\x02\x81\x10aC\x0EW_\x80\xFD[`\x02\x81\x10aN\xBCWcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90RV[_\x83\x83\x85R` \x80\x86\x01\x95P\x80\x85`\x05\x1B\x83\x01\x01\x84_[\x87\x81\x10\x15aK=W\x84\x83\x03`\x1F\x19\x01\x89R\x8156\x88\x90\x03`>\x19\x01\x81\x12aN\xFCW_\x80\xFD[\x87\x01`@aO\x12\x85aO\r\x84aN\x92V[aN\xA0V[aO\x1E\x86\x83\x01\x83aJ\x1CV[\x92P\x81\x87\x87\x01RaO2\x82\x87\x01\x84\x83aJ]V[\x9B\x87\x01\x9B\x95PPP\x91\x84\x01\x91P`\x01\x01aN\xD7V[_\x825`~\x19\x836\x03\x01\x81\x12aO[W_\x80\xFD[\x90\x91\x01\x92\x91PPV[_\x83\x83\x85R` \x80\x86\x01\x95P\x80\x85`\x05\x1B\x83\x01\x01\x84_[\x87\x81\x10\x15aK=W\x84\x83\x03`\x1F\x19\x01\x89RaO\x96\x82\x88aOGV[`\x80\x815\x85R\x85\x82\x015\x86\x86\x01R`@aO\xB2\x81\x84\x01\x84aJ\x1CV[\x83\x83\x89\x01RaO\xC4\x84\x89\x01\x82\x84aJ]V[\x93PPPP``aO\xD7\x81\x84\x01\x84aJ\x1CV[\x93P\x86\x83\x03\x82\x88\x01RaO\xEB\x83\x85\x83aJ]V[\x9C\x88\x01\x9C\x96PPP\x92\x85\x01\x92PP`\x01\x01aO{V[_\x82\x82Q\x80\x85R` \x80\x86\x01\x95P` \x82`\x05\x1B\x84\x01\x01` \x86\x01_[\x84\x81\x10\x15aK=W`\x1F\x19\x86\x84\x03\x01\x89RaP:\x83\x83QaA\xF2V[\x98\x84\x01\x98\x92P\x90\x83\x01\x90`\x01\x01aP\x1EV[``\x80\x82R\x81\x81\x01\x86\x90R_\x90`\x80\x80\x84\x01`\x05\x89\x81\x1B\x86\x01\x83\x01\x8B\x86[\x8C\x81\x10\x15aQ W\x88\x83\x03`\x7F\x19\x01\x85RaP\x85\x82\x8FaOGV[\x805\x84R` \x80\x82\x015\x81\x86\x01R`@\x80\x83\x015`\x1E\x19\x846\x03\x01\x81\x12aP\xAAW_\x80\xFD[\x83\x01\x82\x81\x01\x905`\x01`\x01`@\x1B\x03\x81\x11\x15aP\xC4W_\x80\xFD[\x80\x89\x1B6\x03\x82\x13\x15aP\xD4W_\x80\xFD[\x8A\x83\x89\x01RaP\xE6\x8B\x89\x01\x82\x84aN\xC0V[\x92PPPaP\xF6\x8A\x84\x01\x84aJ\x1CV[\x93P\x86\x82\x03\x8B\x88\x01RaQ\n\x82\x85\x83aJ]V[\x98\x83\x01\x98\x96PPP\x92\x90\x92\x01\x91P`\x01\x01aPjV[PP\x86\x81\x03` \x88\x01RaQ5\x81\x8A\x8CaOdV[\x94PPPPP\x82\x81\x03`@\x84\x01RaQM\x81\x85aP\x01V[\x98\x97PPPPPPPPV[``\x81R_aQk``\x83\x01\x86aA\xF2V[` \x83\x01\x94\x90\x94RP`@\x01R\x91\x90PV[_\x825`>\x19\x836\x03\x01\x81\x12aMlW_\x80\xFD[_` \x82\x84\x03\x12\x15aQ\xA1W_\x80\xFD[a0\x97\x82aN\x92V[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[\x83\x81R``\x81\x01aQ\xCD` \x83\x01\x85aN\xA0V[\x82`@\x83\x01R\x94\x93PPPPV[\x81Q_\x90\x82\x90` \x80\x86\x01\x84[\x83\x81\x10\x15aR\x04W\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01aQ\xE8V[P\x92\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15aR W_\x80\xFD[PQ\x91\x90PV[`\x1F\x82\x11\x15a.rW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15aRLWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a+\xE6W_\x81U`\x01\x01aRXV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aR\x84WaR\x84aE(V[aR\x98\x81aR\x92\x84TaM\x12V[\x84aR'V[` \x80`\x1F\x83\x11`\x01\x81\x14aR\xCBW_\x84\x15aR\xB4WP\x85\x83\x01Q[_\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85UaS\"V[_\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15aR\xF9W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01aR\xDAV[P\x85\x82\x10\x15aS\x16W\x87\x85\x01Q_\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PP`\x01\x84`\x01\x1B\x01\x85U[PPPPPPV[_\x82QaMl\x81\x84` \x87\x01aA\xD0V\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0",
    );
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
    /**Custom error with signature `CurrentKmsContextCannotBeDestroyed(uint256)` and selector `0x4595fce2`.
```solidity
error CurrentKmsContextCannotBeDestroyed(uint256 kmsContextId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CurrentKmsContextCannotBeDestroyed {
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
        impl ::core::convert::From<CurrentKmsContextCannotBeDestroyed>
        for UnderlyingRustTuple<'_> {
            fn from(value: CurrentKmsContextCannotBeDestroyed) -> Self {
                (value.kmsContextId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for CurrentKmsContextCannotBeDestroyed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { kmsContextId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CurrentKmsContextCannotBeDestroyed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CurrentKmsContextCannotBeDestroyed(uint256)";
            const SELECTOR: [u8; 4] = [69u8, 149u8, 252u8, 226u8];
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
    /**Custom error with signature `EpochNotUnderActiveContext(uint256,uint256)` and selector `0xa69d7d5b`.
```solidity
error EpochNotUnderActiveContext(uint256 epochId, uint256 contextId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EpochNotUnderActiveContext {
        #[allow(missing_docs)]
        pub epochId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<EpochNotUnderActiveContext>
        for UnderlyingRustTuple<'_> {
            fn from(value: EpochNotUnderActiveContext) -> Self {
                (value.epochId, value.contextId)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for EpochNotUnderActiveContext {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    epochId: tuple.0,
                    contextId: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EpochNotUnderActiveContext {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EpochNotUnderActiveContext(uint256,uint256)";
            const SELECTOR: [u8; 4] = [166u8, 157u8, 125u8, 91u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.contextId),
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
    /**Custom error with signature `InvalidEpoch(uint256)` and selector `0xa225656d`.
```solidity
error InvalidEpoch(uint256 epochId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidEpoch {
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
        impl ::core::convert::From<InvalidEpoch> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidEpoch) -> Self {
                (value.epochId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidEpoch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { epochId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidEpoch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidEpoch(uint256)";
            const SELECTOR: [u8; 4] = [162u8, 37u8, 101u8, 109u8];
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
    /**Custom error with signature `KmsContextNotCreated(uint256)` and selector `0x32c5b9f6`.
```solidity
error KmsContextNotCreated(uint256 kmsContextId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsContextNotCreated {
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
        impl ::core::convert::From<KmsContextNotCreated> for UnderlyingRustTuple<'_> {
            fn from(value: KmsContextNotCreated) -> Self {
                (value.kmsContextId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KmsContextNotCreated {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { kmsContextId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KmsContextNotCreated {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KmsContextNotCreated(uint256)";
            const SELECTOR: [u8; 4] = [50u8, 197u8, 185u8, 246u8];
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
    /**Custom error with signature `NonIncreasingEpochId(uint256,uint256)` and selector `0xe8121f51`.
```solidity
error NonIncreasingEpochId(uint256 epochId, uint256 currentEpochId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NonIncreasingEpochId {
        #[allow(missing_docs)]
        pub epochId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub currentEpochId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<NonIncreasingEpochId> for UnderlyingRustTuple<'_> {
            fn from(value: NonIncreasingEpochId) -> Self {
                (value.epochId, value.currentEpochId)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NonIncreasingEpochId {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    epochId: tuple.0,
                    currentEpochId: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NonIncreasingEpochId {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NonIncreasingEpochId(uint256,uint256)";
            const SELECTOR: [u8; 4] = [232u8, 18u8, 31u8, 81u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.currentEpochId),
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
    /**Custom error with signature `ThresholdExceedsProofFormatLimit(string,uint256,uint256)` and selector `0x22ba52db`.
```solidity
error ThresholdExceedsProofFormatLimit(string thresholdName, uint256 threshold, uint256 maxAllowed);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ThresholdExceedsProofFormatLimit {
        #[allow(missing_docs)]
        pub thresholdName: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub threshold: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<ThresholdExceedsProofFormatLimit>
        for UnderlyingRustTuple<'_> {
            fn from(value: ThresholdExceedsProofFormatLimit) -> Self {
                (value.thresholdName, value.threshold, value.maxAllowed)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ThresholdExceedsProofFormatLimit {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    thresholdName: tuple.0,
                    threshold: tuple.1,
                    maxAllowed: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ThresholdExceedsProofFormatLimit {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ThresholdExceedsProofFormatLimit(string,uint256,uint256)";
            const SELECTOR: [u8; 4] = [34u8, 186u8, 82u8, 219u8];
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
    /**Event with signature `KmsGenThresholdUpdated(uint256,uint256)` and selector `0xf21cb37be709148aabebd278543e62d1b1e6a4477fb1cc43e069d3eeb8c87f90`.
```solidity
event KmsGenThresholdUpdated(uint256 indexed kmsContextId, uint256 threshold);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct KmsGenThresholdUpdated {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub threshold: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for KmsGenThresholdUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "KmsGenThresholdUpdated(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                242u8, 28u8, 179u8, 123u8, 231u8, 9u8, 20u8, 138u8, 171u8, 235u8, 210u8,
                120u8, 84u8, 62u8, 98u8, 209u8, 177u8, 230u8, 164u8, 71u8, 127u8, 177u8,
                204u8, 67u8, 224u8, 105u8, 211u8, 238u8, 184u8, 200u8, 127u8, 144u8,
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
                    threshold: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.threshold),
                )
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
        impl alloy_sol_types::private::IntoLogData for KmsGenThresholdUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&KmsGenThresholdUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &KmsGenThresholdUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `MirrorKmsContextAndEpoch(uint256,uint256,(address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[])` and selector `0x2ac68f78f4ccde76b64906026d01ff3c42403eb7eef86fe788474a23267d64cf`.
```solidity
event MirrorKmsContextAndEpoch(uint256 indexed contextId, uint256 indexed epochId, KmsNodeParams[] kmsNodeParams, IProtocolConfig.KmsThresholds thresholds, string softwareVersion, PcrValues[] pcrValues);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct MirrorKmsContextAndEpoch {
        #[allow(missing_docs)]
        pub contextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub epochId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub kmsNodeParams: alloy::sol_types::private::Vec<
            <KmsNodeParams as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub thresholds: <IProtocolConfig::KmsThresholds as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for MirrorKmsContextAndEpoch {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<KmsNodeParams>,
                IProtocolConfig::KmsThresholds,
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
            const SIGNATURE: &'static str = "MirrorKmsContextAndEpoch(uint256,uint256,(address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                42u8, 198u8, 143u8, 120u8, 244u8, 204u8, 222u8, 118u8, 182u8, 73u8, 6u8,
                2u8, 109u8, 1u8, 255u8, 60u8, 66u8, 64u8, 62u8, 183u8, 238u8, 248u8,
                111u8, 231u8, 136u8, 71u8, 74u8, 35u8, 38u8, 125u8, 100u8, 207u8,
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
                    epochId: topics.2,
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
                    <IProtocolConfig::KmsThresholds as alloy_sol_types::SolType>::tokenize(
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.contextId);
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.epochId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for MirrorKmsContextAndEpoch {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MirrorKmsContextAndEpoch> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &MirrorKmsContextAndEpoch,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `MirrorKmsEpoch(uint256,uint256)` and selector `0x0a1c24c2ba5e6e1b1a8585795e5b781e372aee1db686247dac7574c10fd735a6`.
```solidity
event MirrorKmsEpoch(uint256 indexed contextId, uint256 indexed epochId);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct MirrorKmsEpoch {
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for MirrorKmsEpoch {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "MirrorKmsEpoch(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                10u8, 28u8, 36u8, 194u8, 186u8, 94u8, 110u8, 27u8, 26u8, 133u8, 133u8,
                121u8, 94u8, 91u8, 120u8, 30u8, 55u8, 42u8, 238u8, 29u8, 182u8, 134u8,
                36u8, 125u8, 172u8, 117u8, 116u8, 193u8, 15u8, 215u8, 53u8, 166u8,
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
                    epochId: topics.2,
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
                    self.contextId.clone(),
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.contextId);
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.epochId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for MirrorKmsEpoch {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MirrorKmsEpoch> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &MirrorKmsEpoch) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `MpcThresholdUpdated(uint256,uint256)` and selector `0x148f9c6cb77d12306b9f596534d14b7aae3e4f98a2dbe3cdb07ea4924c775f12`.
```solidity
event MpcThresholdUpdated(uint256 indexed kmsContextId, uint256 threshold);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct MpcThresholdUpdated {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub threshold: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for MpcThresholdUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "MpcThresholdUpdated(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                20u8, 143u8, 156u8, 108u8, 183u8, 125u8, 18u8, 48u8, 107u8, 159u8, 89u8,
                101u8, 52u8, 209u8, 75u8, 122u8, 174u8, 62u8, 79u8, 152u8, 162u8, 219u8,
                227u8, 205u8, 176u8, 126u8, 164u8, 146u8, 76u8, 119u8, 95u8, 18u8,
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
                    threshold: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.threshold),
                )
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
        impl alloy_sol_types::private::IntoLogData for MpcThresholdUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MpcThresholdUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &MpcThresholdUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `NewKmsContext(uint256,uint256,(address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[])` and selector `0x204d6b80121154cd87d99cf54c639a3dd0a53b3084277098de972ebdd34c6be9`.
```solidity
event NewKmsContext(uint256 indexed contextId, uint256 indexed previousContextId, KmsNodeParams[] kmsNodeParams, IProtocolConfig.KmsThresholds thresholds, string softwareVersion, PcrValues[] pcrValues);
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
        pub thresholds: <IProtocolConfig::KmsThresholds as alloy::sol_types::SolType>::RustType,
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
                IProtocolConfig::KmsThresholds,
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
                    <IProtocolConfig::KmsThresholds as alloy_sol_types::SolType>::tokenize(
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
    /**Event with signature `PendingContextAborted(uint256)` and selector `0x75e115b7f76bf21d0a2e42da9304d9c357b54c489e5af59ed3c70b7cd48335fc`.
```solidity
event PendingContextAborted(uint256 indexed kmsContextId);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PendingContextAborted {
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
        impl alloy_sol_types::SolEvent for PendingContextAborted {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "PendingContextAborted(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                117u8, 225u8, 21u8, 183u8, 247u8, 107u8, 242u8, 29u8, 10u8, 46u8, 66u8,
                218u8, 147u8, 4u8, 217u8, 195u8, 87u8, 181u8, 76u8, 72u8, 158u8, 90u8,
                245u8, 158u8, 211u8, 199u8, 11u8, 124u8, 212u8, 131u8, 53u8, 252u8,
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
        impl alloy_sol_types::private::IntoLogData for PendingContextAborted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PendingContextAborted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PendingContextAborted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `PendingEpochAborted(uint256,uint256)` and selector `0x6440aaea7b2480b82449c317aa5a9168df77eb69308ff8f7c3980a1ad848b7df`.
```solidity
event PendingEpochAborted(uint256 indexed kmsContextId, uint256 indexed epochId);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PendingEpochAborted {
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for PendingEpochAborted {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "PendingEpochAborted(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                100u8, 64u8, 170u8, 234u8, 123u8, 36u8, 128u8, 184u8, 36u8, 73u8, 195u8,
                23u8, 170u8, 90u8, 145u8, 104u8, 223u8, 119u8, 235u8, 105u8, 48u8, 143u8,
                248u8, 247u8, 195u8, 152u8, 10u8, 26u8, 216u8, 72u8, 183u8, 223u8,
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
        impl alloy_sol_types::private::IntoLogData for PendingEpochAborted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PendingEpochAborted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PendingEpochAborted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `PublicDecryptionThresholdUpdated(uint256,uint256)` and selector `0xd571bf833e41553bbe260e00b3af7a0e91aafd6cdc238a803aa9ac0e73efed65`.
```solidity
event PublicDecryptionThresholdUpdated(uint256 indexed kmsContextId, uint256 threshold);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PublicDecryptionThresholdUpdated {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub threshold: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for PublicDecryptionThresholdUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "PublicDecryptionThresholdUpdated(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                213u8, 113u8, 191u8, 131u8, 62u8, 65u8, 85u8, 59u8, 190u8, 38u8, 14u8,
                0u8, 179u8, 175u8, 122u8, 14u8, 145u8, 170u8, 253u8, 108u8, 220u8, 35u8,
                138u8, 128u8, 58u8, 169u8, 172u8, 14u8, 115u8, 239u8, 237u8, 101u8,
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
                    threshold: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.threshold),
                )
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
        impl alloy_sol_types::private::IntoLogData for PublicDecryptionThresholdUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PublicDecryptionThresholdUpdated>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &PublicDecryptionThresholdUpdated,
            ) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `UserDecryptionThresholdUpdated(uint256,uint256)` and selector `0x90f1918493831c1b6133489743103384c5600eae796eb34c51ea4f2baafa4f94`.
```solidity
event UserDecryptionThresholdUpdated(uint256 indexed kmsContextId, uint256 threshold);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UserDecryptionThresholdUpdated {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub threshold: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for UserDecryptionThresholdUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "UserDecryptionThresholdUpdated(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                144u8, 241u8, 145u8, 132u8, 147u8, 131u8, 28u8, 27u8, 97u8, 51u8, 72u8,
                151u8, 67u8, 16u8, 51u8, 132u8, 197u8, 96u8, 14u8, 174u8, 121u8, 110u8,
                179u8, 76u8, 81u8, 234u8, 79u8, 43u8, 170u8, 250u8, 79u8, 148u8,
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
                    threshold: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.threshold),
                )
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
        impl alloy_sol_types::private::IntoLogData for UserDecryptionThresholdUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UserDecryptionThresholdUpdated>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &UserDecryptionThresholdUpdated,
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
    /**Function with signature `abortPendingContext(uint256)` and selector `0x20a4eb39`.
```solidity
function abortPendingContext(uint256 kmsContextId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct abortPendingContextCall {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`abortPendingContext(uint256)`](abortPendingContextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct abortPendingContextReturn {}
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
            impl ::core::convert::From<abortPendingContextCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: abortPendingContextCall) -> Self {
                    (value.kmsContextId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for abortPendingContextCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { kmsContextId: tuple.0 }
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
            impl ::core::convert::From<abortPendingContextReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: abortPendingContextReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for abortPendingContextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl abortPendingContextReturn {
            fn _tokenize(
                &self,
            ) -> <abortPendingContextCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for abortPendingContextCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = abortPendingContextReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "abortPendingContext(uint256)";
            const SELECTOR: [u8; 4] = [32u8, 164u8, 235u8, 57u8];
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
                abortPendingContextReturn::_tokenize(ret)
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
    /**Function with signature `abortPendingEpoch(uint256)` and selector `0x3b56159e`.
```solidity
function abortPendingEpoch(uint256 epochId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct abortPendingEpochCall {
        #[allow(missing_docs)]
        pub epochId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`abortPendingEpoch(uint256)`](abortPendingEpochCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct abortPendingEpochReturn {}
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
            impl ::core::convert::From<abortPendingEpochCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: abortPendingEpochCall) -> Self {
                    (value.epochId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for abortPendingEpochCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { epochId: tuple.0 }
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
            impl ::core::convert::From<abortPendingEpochReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: abortPendingEpochReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for abortPendingEpochReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl abortPendingEpochReturn {
            fn _tokenize(
                &self,
            ) -> <abortPendingEpochCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for abortPendingEpochCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = abortPendingEpochReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "abortPendingEpoch(uint256)";
            const SELECTOR: [u8; 4] = [59u8, 86u8, 21u8, 158u8];
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
                abortPendingEpochReturn::_tokenize(ret)
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
function defineNewKmsContextAndEpoch(KmsNodeParams[] memory kmsNodeParams, IProtocolConfig.KmsThresholds memory thresholds, string memory softwareVersion, PcrValues[] memory pcrValues) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct defineNewKmsContextAndEpochCall {
        #[allow(missing_docs)]
        pub kmsNodeParams: alloy::sol_types::private::Vec<
            <KmsNodeParams as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub thresholds: <IProtocolConfig::KmsThresholds as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<KmsNodeParams>,
                IProtocolConfig::KmsThresholds,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<PcrValues>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <KmsNodeParams as alloy::sol_types::SolType>::RustType,
                >,
                <IProtocolConfig::KmsThresholds as alloy::sol_types::SolType>::RustType,
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
                IProtocolConfig::KmsThresholds,
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
                    <IProtocolConfig::KmsThresholds as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `initializeFromCanonical(uint256,uint256,(address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256))` and selector `0x16d4eb6f`.
```solidity
function initializeFromCanonical(uint256 canonicalContextId, uint256 canonicalEpochId, KmsNodeParams[] memory canonicalKmsNodeParams, IProtocolConfig.KmsThresholds memory canonicalThresholds) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeFromCanonicalCall {
        #[allow(missing_docs)]
        pub canonicalContextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub canonicalEpochId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub canonicalKmsNodeParams: alloy::sol_types::private::Vec<
            <KmsNodeParams as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub canonicalThresholds: <IProtocolConfig::KmsThresholds as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`initializeFromCanonical(uint256,uint256,(address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256))`](initializeFromCanonicalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeFromCanonicalReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<KmsNodeParams>,
                IProtocolConfig::KmsThresholds,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    <KmsNodeParams as alloy::sol_types::SolType>::RustType,
                >,
                <IProtocolConfig::KmsThresholds as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeFromCanonicalCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializeFromCanonicalCall) -> Self {
                    (
                        value.canonicalContextId,
                        value.canonicalEpochId,
                        value.canonicalKmsNodeParams,
                        value.canonicalThresholds,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeFromCanonicalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        canonicalContextId: tuple.0,
                        canonicalEpochId: tuple.1,
                        canonicalKmsNodeParams: tuple.2,
                        canonicalThresholds: tuple.3,
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
            impl ::core::convert::From<initializeFromCanonicalReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializeFromCanonicalReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeFromCanonicalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl initializeFromCanonicalReturn {
            fn _tokenize(
                &self,
            ) -> <initializeFromCanonicalCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeFromCanonicalCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<KmsNodeParams>,
                IProtocolConfig::KmsThresholds,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeFromCanonicalReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initializeFromCanonical(uint256,uint256,(address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256))";
            const SELECTOR: [u8; 4] = [22u8, 212u8, 235u8, 111u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.canonicalContextId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.canonicalEpochId),
                    <alloy::sol_types::sol_data::Array<
                        KmsNodeParams,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.canonicalKmsNodeParams,
                    ),
                    <IProtocolConfig::KmsThresholds as alloy_sol_types::SolType>::tokenize(
                        &self.canonicalThresholds,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                initializeFromCanonicalReturn::_tokenize(ret)
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
    /**Function with signature `initializeFromEmptyProxy((address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[])` and selector `0x221cdd4e`.
```solidity
function initializeFromEmptyProxy(KmsNodeParams[] memory initialKmsNodeParams, IProtocolConfig.KmsThresholds memory initialThresholds, string memory softwareVersion, PcrValues[] memory pcrValues) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeFromEmptyProxyCall {
        #[allow(missing_docs)]
        pub initialKmsNodeParams: alloy::sol_types::private::Vec<
            <KmsNodeParams as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub initialThresholds: <IProtocolConfig::KmsThresholds as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<KmsNodeParams>,
                IProtocolConfig::KmsThresholds,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<PcrValues>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <KmsNodeParams as alloy::sol_types::SolType>::RustType,
                >,
                <IProtocolConfig::KmsThresholds as alloy::sol_types::SolType>::RustType,
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
                IProtocolConfig::KmsThresholds,
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
                    <IProtocolConfig::KmsThresholds as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `mirrorKmsContextAndEpoch(uint256,uint256,(address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[])` and selector `0xbc4d07c2`.
```solidity
function mirrorKmsContextAndEpoch(uint256 contextId, uint256 epochId, KmsNodeParams[] memory kmsNodeParams, IProtocolConfig.KmsThresholds memory thresholds, string memory softwareVersion, PcrValues[] memory pcrValues) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mirrorKmsContextAndEpochCall {
        #[allow(missing_docs)]
        pub contextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub epochId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub kmsNodeParams: alloy::sol_types::private::Vec<
            <KmsNodeParams as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub thresholds: <IProtocolConfig::KmsThresholds as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub softwareVersion: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub pcrValues: alloy::sol_types::private::Vec<
            <PcrValues as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`mirrorKmsContextAndEpoch(uint256,uint256,(address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[])`](mirrorKmsContextAndEpochCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mirrorKmsContextAndEpochReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<KmsNodeParams>,
                IProtocolConfig::KmsThresholds,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<PcrValues>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    <KmsNodeParams as alloy::sol_types::SolType>::RustType,
                >,
                <IProtocolConfig::KmsThresholds as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<mirrorKmsContextAndEpochCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: mirrorKmsContextAndEpochCall) -> Self {
                    (
                        value.contextId,
                        value.epochId,
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
            for mirrorKmsContextAndEpochCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        contextId: tuple.0,
                        epochId: tuple.1,
                        kmsNodeParams: tuple.2,
                        thresholds: tuple.3,
                        softwareVersion: tuple.4,
                        pcrValues: tuple.5,
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
            impl ::core::convert::From<mirrorKmsContextAndEpochReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: mirrorKmsContextAndEpochReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for mirrorKmsContextAndEpochReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl mirrorKmsContextAndEpochReturn {
            fn _tokenize(
                &self,
            ) -> <mirrorKmsContextAndEpochCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for mirrorKmsContextAndEpochCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<KmsNodeParams>,
                IProtocolConfig::KmsThresholds,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<PcrValues>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = mirrorKmsContextAndEpochReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "mirrorKmsContextAndEpoch(uint256,uint256,(address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[])";
            const SELECTOR: [u8; 4] = [188u8, 77u8, 7u8, 194u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.epochId),
                    <alloy::sol_types::sol_data::Array<
                        KmsNodeParams,
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsNodeParams),
                    <IProtocolConfig::KmsThresholds as alloy_sol_types::SolType>::tokenize(
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
                mirrorKmsContextAndEpochReturn::_tokenize(ret)
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
    /**Function with signature `mirrorKmsEpoch(uint256,uint256)` and selector `0x8e97cb60`.
```solidity
function mirrorKmsEpoch(uint256 contextId, uint256 epochId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mirrorKmsEpochCall {
        #[allow(missing_docs)]
        pub contextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub epochId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`mirrorKmsEpoch(uint256,uint256)`](mirrorKmsEpochCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mirrorKmsEpochReturn {}
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
            impl ::core::convert::From<mirrorKmsEpochCall> for UnderlyingRustTuple<'_> {
                fn from(value: mirrorKmsEpochCall) -> Self {
                    (value.contextId, value.epochId)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mirrorKmsEpochCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        contextId: tuple.0,
                        epochId: tuple.1,
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
            impl ::core::convert::From<mirrorKmsEpochReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: mirrorKmsEpochReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for mirrorKmsEpochReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl mirrorKmsEpochReturn {
            fn _tokenize(
                &self,
            ) -> <mirrorKmsEpochCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for mirrorKmsEpochCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = mirrorKmsEpochReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "mirrorKmsEpoch(uint256,uint256)";
            const SELECTOR: [u8; 4] = [142u8, 151u8, 203u8, 96u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.epochId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                mirrorKmsEpochReturn::_tokenize(ret)
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
    /**Function with signature `reinitializeV2((address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[])` and selector `0x8aeac229`.
```solidity
function reinitializeV2(KmsNodeParams[] memory kmsNodeParams, IProtocolConfig.KmsThresholds memory thresholds, string memory softwareVersion, PcrValues[] memory pcrValues) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reinitializeV2Call {
        #[allow(missing_docs)]
        pub kmsNodeParams: alloy::sol_types::private::Vec<
            <KmsNodeParams as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub thresholds: <IProtocolConfig::KmsThresholds as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub softwareVersion: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub pcrValues: alloy::sol_types::private::Vec<
            <PcrValues as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`reinitializeV2((address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[])`](reinitializeV2Call) function.
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<KmsNodeParams>,
                IProtocolConfig::KmsThresholds,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<PcrValues>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <KmsNodeParams as alloy::sol_types::SolType>::RustType,
                >,
                <IProtocolConfig::KmsThresholds as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<reinitializeV2Call> for UnderlyingRustTuple<'_> {
                fn from(value: reinitializeV2Call) -> Self {
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
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for reinitializeV2Call {
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
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<KmsNodeParams>,
                IProtocolConfig::KmsThresholds,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<PcrValues>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = reinitializeV2Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "reinitializeV2((address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[])";
            const SELECTOR: [u8; 4] = [138u8, 234u8, 194u8, 41u8];
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
                    <IProtocolConfig::KmsThresholds as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `updateKmsGenThresholdForContext(uint256,uint256)` and selector `0xb0b461c4`.
```solidity
function updateKmsGenThresholdForContext(uint256 kmsContextId, uint256 threshold) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateKmsGenThresholdForContextCall {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub threshold: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`updateKmsGenThresholdForContext(uint256,uint256)`](updateKmsGenThresholdForContextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateKmsGenThresholdForContextReturn {}
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
            impl ::core::convert::From<updateKmsGenThresholdForContextCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateKmsGenThresholdForContextCall) -> Self {
                    (value.kmsContextId, value.threshold)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateKmsGenThresholdForContextCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        kmsContextId: tuple.0,
                        threshold: tuple.1,
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
            impl ::core::convert::From<updateKmsGenThresholdForContextReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateKmsGenThresholdForContextReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateKmsGenThresholdForContextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl updateKmsGenThresholdForContextReturn {
            fn _tokenize(
                &self,
            ) -> <updateKmsGenThresholdForContextCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateKmsGenThresholdForContextCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateKmsGenThresholdForContextReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateKmsGenThresholdForContext(uint256,uint256)";
            const SELECTOR: [u8; 4] = [176u8, 180u8, 97u8, 196u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.threshold),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                updateKmsGenThresholdForContextReturn::_tokenize(ret)
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
    /**Function with signature `updateMpcThresholdForContext(uint256,uint256)` and selector `0x77d38e24`.
```solidity
function updateMpcThresholdForContext(uint256 kmsContextId, uint256 threshold) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateMpcThresholdForContextCall {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub threshold: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`updateMpcThresholdForContext(uint256,uint256)`](updateMpcThresholdForContextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateMpcThresholdForContextReturn {}
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
            impl ::core::convert::From<updateMpcThresholdForContextCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateMpcThresholdForContextCall) -> Self {
                    (value.kmsContextId, value.threshold)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateMpcThresholdForContextCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        kmsContextId: tuple.0,
                        threshold: tuple.1,
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
            impl ::core::convert::From<updateMpcThresholdForContextReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateMpcThresholdForContextReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateMpcThresholdForContextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl updateMpcThresholdForContextReturn {
            fn _tokenize(
                &self,
            ) -> <updateMpcThresholdForContextCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateMpcThresholdForContextCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateMpcThresholdForContextReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateMpcThresholdForContext(uint256,uint256)";
            const SELECTOR: [u8; 4] = [119u8, 211u8, 142u8, 36u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.threshold),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                updateMpcThresholdForContextReturn::_tokenize(ret)
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
    /**Function with signature `updatePublicDecryptionThresholdForContext(uint256,uint256)` and selector `0x06834d1d`.
```solidity
function updatePublicDecryptionThresholdForContext(uint256 kmsContextId, uint256 threshold) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updatePublicDecryptionThresholdForContextCall {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub threshold: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`updatePublicDecryptionThresholdForContext(uint256,uint256)`](updatePublicDecryptionThresholdForContextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updatePublicDecryptionThresholdForContextReturn {}
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
            impl ::core::convert::From<updatePublicDecryptionThresholdForContextCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updatePublicDecryptionThresholdForContextCall) -> Self {
                    (value.kmsContextId, value.threshold)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updatePublicDecryptionThresholdForContextCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        kmsContextId: tuple.0,
                        threshold: tuple.1,
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
            impl ::core::convert::From<updatePublicDecryptionThresholdForContextReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updatePublicDecryptionThresholdForContextReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updatePublicDecryptionThresholdForContextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl updatePublicDecryptionThresholdForContextReturn {
            fn _tokenize(
                &self,
            ) -> <updatePublicDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updatePublicDecryptionThresholdForContextCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updatePublicDecryptionThresholdForContextReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updatePublicDecryptionThresholdForContext(uint256,uint256)";
            const SELECTOR: [u8; 4] = [6u8, 131u8, 77u8, 29u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.threshold),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                updatePublicDecryptionThresholdForContextReturn::_tokenize(ret)
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
    /**Function with signature `updateUserDecryptionThresholdForContext(uint256,uint256)` and selector `0xb181cda7`.
```solidity
function updateUserDecryptionThresholdForContext(uint256 kmsContextId, uint256 threshold) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateUserDecryptionThresholdForContextCall {
        #[allow(missing_docs)]
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub threshold: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`updateUserDecryptionThresholdForContext(uint256,uint256)`](updateUserDecryptionThresholdForContextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateUserDecryptionThresholdForContextReturn {}
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
            impl ::core::convert::From<updateUserDecryptionThresholdForContextCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateUserDecryptionThresholdForContextCall) -> Self {
                    (value.kmsContextId, value.threshold)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateUserDecryptionThresholdForContextCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        kmsContextId: tuple.0,
                        threshold: tuple.1,
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
            impl ::core::convert::From<updateUserDecryptionThresholdForContextReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateUserDecryptionThresholdForContextReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateUserDecryptionThresholdForContextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl updateUserDecryptionThresholdForContextReturn {
            fn _tokenize(
                &self,
            ) -> <updateUserDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateUserDecryptionThresholdForContextCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateUserDecryptionThresholdForContextReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateUserDecryptionThresholdForContext(uint256,uint256)";
            const SELECTOR: [u8; 4] = [177u8, 129u8, 205u8, 167u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.threshold),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                updateUserDecryptionThresholdForContextReturn::_tokenize(ret)
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
    ///Container for all the [`ProtocolConfig`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum ProtocolConfigCalls {
        #[allow(missing_docs)]
        UPGRADE_INTERFACE_VERSION(UPGRADE_INTERFACE_VERSIONCall),
        #[allow(missing_docs)]
        abortPendingContext(abortPendingContextCall),
        #[allow(missing_docs)]
        abortPendingEpoch(abortPendingEpochCall),
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
        getCurrentKmsContextAndEpoch(getCurrentKmsContextAndEpochCall),
        #[allow(missing_docs)]
        getCurrentKmsContextId(getCurrentKmsContextIdCall),
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
        initializeFromCanonical(initializeFromCanonicalCall),
        #[allow(missing_docs)]
        initializeFromEmptyProxy(initializeFromEmptyProxyCall),
        #[allow(missing_docs)]
        isKmsSigner(isKmsSignerCall),
        #[allow(missing_docs)]
        isKmsSignerForContext(isKmsSignerForContextCall),
        #[allow(missing_docs)]
        isKmsTxSenderForContext(isKmsTxSenderForContextCall),
        #[allow(missing_docs)]
        isValidEpochForContext(isValidEpochForContextCall),
        #[allow(missing_docs)]
        isValidKmsContext(isValidKmsContextCall),
        #[allow(missing_docs)]
        mirrorKmsContextAndEpoch(mirrorKmsContextAndEpochCall),
        #[allow(missing_docs)]
        mirrorKmsEpoch(mirrorKmsEpochCall),
        #[allow(missing_docs)]
        proxiableUUID(proxiableUUIDCall),
        #[allow(missing_docs)]
        reinitializeV2(reinitializeV2Call),
        #[allow(missing_docs)]
        updateKmsGenThresholdForContext(updateKmsGenThresholdForContextCall),
        #[allow(missing_docs)]
        updateMpcThresholdForContext(updateMpcThresholdForContextCall),
        #[allow(missing_docs)]
        updatePublicDecryptionThresholdForContext(
            updatePublicDecryptionThresholdForContextCall,
        ),
        #[allow(missing_docs)]
        updateUserDecryptionThresholdForContext(
            updateUserDecryptionThresholdForContextCall,
        ),
        #[allow(missing_docs)]
        upgradeToAndCall(upgradeToAndCallCall),
    }
    #[automatically_derived]
    impl ProtocolConfigCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [6u8, 131u8, 77u8, 29u8],
            [13u8, 142u8, 110u8, 44u8],
            [22u8, 212u8, 235u8, 111u8],
            [28u8, 227u8, 249u8, 188u8],
            [32u8, 61u8, 1u8, 20u8],
            [32u8, 164u8, 235u8, 57u8],
            [34u8, 28u8, 221u8, 78u8],
            [38u8, 207u8, 93u8, 239u8],
            [40u8, 30u8, 139u8, 254u8],
            [42u8, 56u8, 137u8, 152u8],
            [49u8, 255u8, 65u8, 200u8],
            [59u8, 86u8, 21u8, 158u8],
            [65u8, 173u8, 6u8, 156u8],
            [70u8, 197u8, 187u8, 189u8],
            [71u8, 232u8, 34u8, 149u8],
            [76u8, 185u8, 80u8, 225u8],
            [79u8, 30u8, 242u8, 134u8],
            [82u8, 209u8, 144u8, 45u8],
            [91u8, 255u8, 118u8, 217u8],
            [101u8, 179u8, 148u8, 175u8],
            [119u8, 211u8, 142u8, 36u8],
            [126u8, 170u8, 200u8, 242u8],
            [138u8, 234u8, 194u8, 41u8],
            [142u8, 151u8, 203u8, 96u8],
            [148u8, 71u8, 207u8, 212u8],
            [151u8, 108u8, 152u8, 181u8],
            [151u8, 111u8, 62u8, 185u8],
            [173u8, 60u8, 177u8, 204u8],
            [176u8, 180u8, 97u8, 196u8],
            [177u8, 129u8, 205u8, 167u8],
            [180u8, 114u8, 43u8, 196u8],
            [188u8, 77u8, 7u8, 194u8],
            [191u8, 155u8, 22u8, 200u8],
            [192u8, 174u8, 100u8, 247u8],
            [194u8, 180u8, 41u8, 134u8],
            [195u8, 170u8, 170u8, 90u8],
            [201u8, 153u8, 168u8, 180u8],
            [204u8, 234u8, 192u8, 25u8],
            [217u8, 190u8, 45u8, 228u8],
            [249u8, 198u8, 112u8, 195u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ProtocolConfigCalls {
        const NAME: &'static str = "ProtocolConfigCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 40usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::UPGRADE_INTERFACE_VERSION(_) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::abortPendingContext(_) => {
                    <abortPendingContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::abortPendingEpoch(_) => {
                    <abortPendingEpochCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::getCurrentKmsContextAndEpoch(_) => {
                    <getCurrentKmsContextAndEpochCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentKmsContextId(_) => {
                    <getCurrentKmsContextIdCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::initializeFromCanonical(_) => {
                    <initializeFromCanonicalCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::isValidEpochForContext(_) => {
                    <isValidEpochForContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isValidKmsContext(_) => {
                    <isValidKmsContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::mirrorKmsContextAndEpoch(_) => {
                    <mirrorKmsContextAndEpochCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::mirrorKmsEpoch(_) => {
                    <mirrorKmsEpochCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proxiableUUID(_) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::reinitializeV2(_) => {
                    <reinitializeV2Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateKmsGenThresholdForContext(_) => {
                    <updateKmsGenThresholdForContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateMpcThresholdForContext(_) => {
                    <updateMpcThresholdForContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updatePublicDecryptionThresholdForContext(_) => {
                    <updatePublicDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateUserDecryptionThresholdForContext(_) => {
                    <updateUserDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::SELECTOR
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
                    fn updatePublicDecryptionThresholdForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <updatePublicDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigCalls::updatePublicDecryptionThresholdForContext,
                            )
                    }
                    updatePublicDecryptionThresholdForContext
                },
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
                    fn initializeFromCanonical(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <initializeFromCanonicalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::initializeFromCanonical)
                    }
                    initializeFromCanonical
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
                    fn abortPendingContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <abortPendingContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::abortPendingContext)
                    }
                    abortPendingContext
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
                    fn abortPendingEpoch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <abortPendingEpochCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::abortPendingEpoch)
                    }
                    abortPendingEpoch
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
                    fn updateMpcThresholdForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <updateMpcThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::updateMpcThresholdForContext)
                    }
                    updateMpcThresholdForContext
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
                    fn reinitializeV2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <reinitializeV2Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::reinitializeV2)
                    }
                    reinitializeV2
                },
                {
                    fn mirrorKmsEpoch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <mirrorKmsEpochCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::mirrorKmsEpoch)
                    }
                    mirrorKmsEpoch
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
                    fn updateKmsGenThresholdForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <updateKmsGenThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::updateKmsGenThresholdForContext)
                    }
                    updateKmsGenThresholdForContext
                },
                {
                    fn updateUserDecryptionThresholdForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <updateUserDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigCalls::updateUserDecryptionThresholdForContext,
                            )
                    }
                    updateUserDecryptionThresholdForContext
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
                    fn mirrorKmsContextAndEpoch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <mirrorKmsContextAndEpochCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::mirrorKmsContextAndEpoch)
                    }
                    mirrorKmsContextAndEpoch
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
                    fn updatePublicDecryptionThresholdForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <updatePublicDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigCalls::updatePublicDecryptionThresholdForContext,
                            )
                    }
                    updatePublicDecryptionThresholdForContext
                },
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
                    fn initializeFromCanonical(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <initializeFromCanonicalCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::initializeFromCanonical)
                    }
                    initializeFromCanonical
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
                    fn abortPendingContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <abortPendingContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::abortPendingContext)
                    }
                    abortPendingContext
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
                    fn abortPendingEpoch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <abortPendingEpochCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::abortPendingEpoch)
                    }
                    abortPendingEpoch
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
                    fn updateMpcThresholdForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <updateMpcThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::updateMpcThresholdForContext)
                    }
                    updateMpcThresholdForContext
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
                    fn reinitializeV2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <reinitializeV2Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::reinitializeV2)
                    }
                    reinitializeV2
                },
                {
                    fn mirrorKmsEpoch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <mirrorKmsEpochCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::mirrorKmsEpoch)
                    }
                    mirrorKmsEpoch
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
                    fn updateKmsGenThresholdForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <updateKmsGenThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::updateKmsGenThresholdForContext)
                    }
                    updateKmsGenThresholdForContext
                },
                {
                    fn updateUserDecryptionThresholdForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <updateUserDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigCalls::updateUserDecryptionThresholdForContext,
                            )
                    }
                    updateUserDecryptionThresholdForContext
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
                    fn mirrorKmsContextAndEpoch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <mirrorKmsContextAndEpochCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::mirrorKmsContextAndEpoch)
                    }
                    mirrorKmsContextAndEpoch
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
                Self::abortPendingContext(inner) => {
                    <abortPendingContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::abortPendingEpoch(inner) => {
                    <abortPendingEpochCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::initializeFromCanonical(inner) => {
                    <initializeFromCanonicalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
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
                Self::mirrorKmsContextAndEpoch(inner) => {
                    <mirrorKmsContextAndEpochCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::mirrorKmsEpoch(inner) => {
                    <mirrorKmsEpochCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::updateKmsGenThresholdForContext(inner) => {
                    <updateKmsGenThresholdForContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateMpcThresholdForContext(inner) => {
                    <updateMpcThresholdForContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updatePublicDecryptionThresholdForContext(inner) => {
                    <updatePublicDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateUserDecryptionThresholdForContext(inner) => {
                    <updateUserDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::abortPendingContext(inner) => {
                    <abortPendingContextCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::abortPendingEpoch(inner) => {
                    <abortPendingEpochCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::initializeFromCanonical(inner) => {
                    <initializeFromCanonicalCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::mirrorKmsContextAndEpoch(inner) => {
                    <mirrorKmsContextAndEpochCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::mirrorKmsEpoch(inner) => {
                    <mirrorKmsEpochCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::updateKmsGenThresholdForContext(inner) => {
                    <updateKmsGenThresholdForContextCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateMpcThresholdForContext(inner) => {
                    <updateMpcThresholdForContextCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updatePublicDecryptionThresholdForContext(inner) => {
                    <updatePublicDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateUserDecryptionThresholdForContext(inner) => {
                    <updateUserDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum ProtocolConfigErrors {
        #[allow(missing_docs)]
        AddressEmptyCode(AddressEmptyCode),
        #[allow(missing_docs)]
        CurrentKmsContextCannotBeDestroyed(CurrentKmsContextCannotBeDestroyed),
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
        EmptyKmsNodes(EmptyKmsNodes),
        #[allow(missing_docs)]
        EpochActivationAlreadyConfirmed(EpochActivationAlreadyConfirmed),
        #[allow(missing_docs)]
        EpochActivationSignerDoesNotMatchTxSender(
            EpochActivationSignerDoesNotMatchTxSender,
        ),
        #[allow(missing_docs)]
        EpochActivationUnauthorized(EpochActivationUnauthorized),
        #[allow(missing_docs)]
        EpochNotUnderActiveContext(EpochNotUnderActiveContext),
        #[allow(missing_docs)]
        FailedCall(FailedCall),
        #[allow(missing_docs)]
        InvalidEpoch(InvalidEpoch),
        #[allow(missing_docs)]
        InvalidHighThreshold(InvalidHighThreshold),
        #[allow(missing_docs)]
        InvalidInitialization(InvalidInitialization),
        #[allow(missing_docs)]
        InvalidKmsContext(InvalidKmsContext),
        #[allow(missing_docs)]
        InvalidNullThreshold(InvalidNullThreshold),
        #[allow(missing_docs)]
        KmsContextCreationAlreadyConfirmed(KmsContextCreationAlreadyConfirmed),
        #[allow(missing_docs)]
        KmsContextCreationUnauthorized(KmsContextCreationUnauthorized),
        #[allow(missing_docs)]
        KmsContextNotCreated(KmsContextNotCreated),
        #[allow(missing_docs)]
        KmsContextNotPending(KmsContextNotPending),
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
        NonIncreasingEpochId(NonIncreasingEpochId),
        #[allow(missing_docs)]
        NonIncreasingKmsContextId(NonIncreasingKmsContextId),
        #[allow(missing_docs)]
        NotHostOwner(NotHostOwner),
        #[allow(missing_docs)]
        NotInitializing(NotInitializing),
        #[allow(missing_docs)]
        NotInitializingFromEmptyProxy(NotInitializingFromEmptyProxy),
        #[allow(missing_docs)]
        ThresholdExceedsProofFormatLimit(ThresholdExceedsProofFormatLimit),
        #[allow(missing_docs)]
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        #[allow(missing_docs)]
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
    }
    #[automatically_derived]
    impl ProtocolConfigErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [6u8, 140u8, 141u8, 64u8],
            [22u8, 167u8, 39u8, 120u8],
            [33u8, 191u8, 218u8, 16u8],
            [34u8, 186u8, 82u8, 219u8],
            [45u8, 236u8, 207u8, 77u8],
            [50u8, 197u8, 185u8, 246u8],
            [53u8, 134u8, 239u8, 161u8],
            [54u8, 191u8, 182u8, 14u8],
            [69u8, 149u8, 252u8, 226u8],
            [73u8, 65u8, 118u8, 54u8],
            [76u8, 156u8, 140u8, 227u8],
            [98u8, 88u8, 92u8, 200u8],
            [111u8, 79u8, 115u8, 31u8],
            [119u8, 221u8, 190u8, 129u8],
            [132u8, 102u8, 128u8, 74u8],
            [153u8, 150u8, 179u8, 21u8],
            [162u8, 37u8, 101u8, 109u8],
            [163u8, 244u8, 175u8, 235u8],
            [166u8, 157u8, 125u8, 91u8],
            [170u8, 29u8, 73u8, 164u8],
            [179u8, 152u8, 151u8, 159u8],
            [184u8, 29u8, 248u8, 232u8],
            [202u8, 168u8, 20u8, 163u8],
            [209u8, 140u8, 79u8, 240u8],
            [214u8, 189u8, 162u8, 117u8],
            [215u8, 139u8, 206u8, 12u8],
            [215u8, 230u8, 188u8, 248u8],
            [224u8, 124u8, 141u8, 186u8],
            [232u8, 18u8, 31u8, 81u8],
            [239u8, 213u8, 95u8, 103u8],
            [241u8, 115u8, 91u8, 70u8],
            [245u8, 26u8, 246u8, 187u8],
            [246u8, 69u8, 238u8, 223u8],
            [249u8, 46u8, 232u8, 169u8],
            [252u8, 230u8, 152u8, 247u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ProtocolConfigErrors {
        const NAME: &'static str = "ProtocolConfigErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 35usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AddressEmptyCode(_) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CurrentKmsContextCannotBeDestroyed(_) => {
                    <CurrentKmsContextCannotBeDestroyed as alloy_sol_types::SolError>::SELECTOR
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
                Self::EmptyKmsNodes(_) => {
                    <EmptyKmsNodes as alloy_sol_types::SolError>::SELECTOR
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
                Self::EpochNotUnderActiveContext(_) => {
                    <EpochNotUnderActiveContext as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FailedCall(_) => {
                    <FailedCall as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidEpoch(_) => {
                    <InvalidEpoch as alloy_sol_types::SolError>::SELECTOR
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
                Self::InvalidNullThreshold(_) => {
                    <InvalidNullThreshold as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsContextCreationAlreadyConfirmed(_) => {
                    <KmsContextCreationAlreadyConfirmed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsContextCreationUnauthorized(_) => {
                    <KmsContextCreationUnauthorized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsContextNotCreated(_) => {
                    <KmsContextNotCreated as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsContextNotPending(_) => {
                    <KmsContextNotPending as alloy_sol_types::SolError>::SELECTOR
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
                Self::NonIncreasingEpochId(_) => {
                    <NonIncreasingEpochId as alloy_sol_types::SolError>::SELECTOR
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
                Self::ThresholdExceedsProofFormatLimit(_) => {
                    <ThresholdExceedsProofFormatLimit as alloy_sol_types::SolError>::SELECTOR
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
                    fn NotHostOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <NotHostOwner as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ProtocolConfigErrors::NotHostOwner)
                    }
                    NotHostOwner
                },
                {
                    fn ThresholdExceedsProofFormatLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <ThresholdExceedsProofFormatLimit as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::ThresholdExceedsProofFormatLimit)
                    }
                    ThresholdExceedsProofFormatLimit
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
                    fn KmsContextNotCreated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsContextNotCreated as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::KmsContextNotCreated)
                    }
                    KmsContextNotCreated
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
                    fn CurrentKmsContextCannotBeDestroyed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <CurrentKmsContextCannotBeDestroyed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigErrors::CurrentKmsContextCannotBeDestroyed,
                            )
                    }
                    CurrentKmsContextCannotBeDestroyed
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
                    fn InvalidEpoch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <InvalidEpoch as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ProtocolConfigErrors::InvalidEpoch)
                    }
                    InvalidEpoch
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
                    fn EpochNotUnderActiveContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <EpochNotUnderActiveContext as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::EpochNotUnderActiveContext)
                    }
                    EpochNotUnderActiveContext
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
                    fn NonIncreasingEpochId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <NonIncreasingEpochId as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::NonIncreasingEpochId)
                    }
                    NonIncreasingEpochId
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
                    fn ThresholdExceedsProofFormatLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <ThresholdExceedsProofFormatLimit as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::ThresholdExceedsProofFormatLimit)
                    }
                    ThresholdExceedsProofFormatLimit
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
                    fn KmsContextNotCreated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsContextNotCreated as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::KmsContextNotCreated)
                    }
                    KmsContextNotCreated
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
                    fn CurrentKmsContextCannotBeDestroyed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <CurrentKmsContextCannotBeDestroyed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigErrors::CurrentKmsContextCannotBeDestroyed,
                            )
                    }
                    CurrentKmsContextCannotBeDestroyed
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
                    fn InvalidEpoch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <InvalidEpoch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::InvalidEpoch)
                    }
                    InvalidEpoch
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
                    fn EpochNotUnderActiveContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <EpochNotUnderActiveContext as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::EpochNotUnderActiveContext)
                    }
                    EpochNotUnderActiveContext
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
                    fn NonIncreasingEpochId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <NonIncreasingEpochId as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::NonIncreasingEpochId)
                    }
                    NonIncreasingEpochId
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
                Self::CurrentKmsContextCannotBeDestroyed(inner) => {
                    <CurrentKmsContextCannotBeDestroyed as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::EmptyKmsNodes(inner) => {
                    <EmptyKmsNodes as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::EpochNotUnderActiveContext(inner) => {
                    <EpochNotUnderActiveContext as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidEpoch(inner) => {
                    <InvalidEpoch as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::InvalidNullThreshold(inner) => {
                    <InvalidNullThreshold as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::KmsContextNotCreated(inner) => {
                    <KmsContextNotCreated as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KmsContextNotPending(inner) => {
                    <KmsContextNotPending as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::NonIncreasingEpochId(inner) => {
                    <NonIncreasingEpochId as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::ThresholdExceedsProofFormatLimit(inner) => {
                    <ThresholdExceedsProofFormatLimit as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::CurrentKmsContextCannotBeDestroyed(inner) => {
                    <CurrentKmsContextCannotBeDestroyed as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::EmptyKmsNodes(inner) => {
                    <EmptyKmsNodes as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::EpochNotUnderActiveContext(inner) => {
                    <EpochNotUnderActiveContext as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidEpoch(inner) => {
                    <InvalidEpoch as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidNullThreshold(inner) => {
                    <InvalidNullThreshold as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::KmsContextNotCreated(inner) => {
                    <KmsContextNotCreated as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::NonIncreasingEpochId(inner) => {
                    <NonIncreasingEpochId as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::ThresholdExceedsProofFormatLimit(inner) => {
                    <ThresholdExceedsProofFormatLimit as alloy_sol_types::SolError>::abi_encode_raw(
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
    ///Container for all the [`ProtocolConfig`](self) events.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum ProtocolConfigEvents {
        #[allow(missing_docs)]
        ActivateEpoch(ActivateEpoch),
        #[allow(missing_docs)]
        EpochActivationConfirmation(EpochActivationConfirmation),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        KmsContextCreationConfirmation(KmsContextCreationConfirmation),
        #[allow(missing_docs)]
        KmsContextDestroyed(KmsContextDestroyed),
        #[allow(missing_docs)]
        KmsGenThresholdUpdated(KmsGenThresholdUpdated),
        #[allow(missing_docs)]
        MirrorKmsContextAndEpoch(MirrorKmsContextAndEpoch),
        #[allow(missing_docs)]
        MirrorKmsEpoch(MirrorKmsEpoch),
        #[allow(missing_docs)]
        MpcThresholdUpdated(MpcThresholdUpdated),
        #[allow(missing_docs)]
        NewKmsContext(NewKmsContext),
        #[allow(missing_docs)]
        NewKmsEpoch(NewKmsEpoch),
        #[allow(missing_docs)]
        PendingContextAborted(PendingContextAborted),
        #[allow(missing_docs)]
        PendingEpochAborted(PendingEpochAborted),
        #[allow(missing_docs)]
        PublicDecryptionThresholdUpdated(PublicDecryptionThresholdUpdated),
        #[allow(missing_docs)]
        Upgraded(Upgraded),
        #[allow(missing_docs)]
        UserDecryptionThresholdUpdated(UserDecryptionThresholdUpdated),
    }
    #[automatically_derived]
    impl ProtocolConfigEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                10u8, 28u8, 36u8, 194u8, 186u8, 94u8, 110u8, 27u8, 26u8, 133u8, 133u8,
                121u8, 94u8, 91u8, 120u8, 30u8, 55u8, 42u8, 238u8, 29u8, 182u8, 134u8,
                36u8, 125u8, 172u8, 117u8, 116u8, 193u8, 15u8, 215u8, 53u8, 166u8,
            ],
            [
                20u8, 143u8, 156u8, 108u8, 183u8, 125u8, 18u8, 48u8, 107u8, 159u8, 89u8,
                101u8, 52u8, 209u8, 75u8, 122u8, 174u8, 62u8, 79u8, 152u8, 162u8, 219u8,
                227u8, 205u8, 176u8, 126u8, 164u8, 146u8, 76u8, 119u8, 95u8, 18u8,
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
                42u8, 198u8, 143u8, 120u8, 244u8, 204u8, 222u8, 118u8, 182u8, 73u8, 6u8,
                2u8, 109u8, 1u8, 255u8, 60u8, 66u8, 64u8, 62u8, 183u8, 238u8, 248u8,
                111u8, 231u8, 136u8, 71u8, 74u8, 35u8, 38u8, 125u8, 100u8, 207u8,
            ],
            [
                100u8, 64u8, 170u8, 234u8, 123u8, 36u8, 128u8, 184u8, 36u8, 73u8, 195u8,
                23u8, 170u8, 90u8, 145u8, 104u8, 223u8, 119u8, 235u8, 105u8, 48u8, 143u8,
                248u8, 247u8, 195u8, 152u8, 10u8, 26u8, 216u8, 72u8, 183u8, 223u8,
            ],
            [
                117u8, 225u8, 21u8, 183u8, 247u8, 107u8, 242u8, 29u8, 10u8, 46u8, 66u8,
                218u8, 147u8, 4u8, 217u8, 195u8, 87u8, 181u8, 76u8, 72u8, 158u8, 90u8,
                245u8, 158u8, 211u8, 199u8, 11u8, 124u8, 212u8, 131u8, 53u8, 252u8,
            ],
            [
                126u8, 218u8, 111u8, 133u8, 226u8, 59u8, 123u8, 145u8, 192u8, 25u8,
                176u8, 87u8, 13u8, 2u8, 182u8, 99u8, 96u8, 110u8, 249u8, 215u8, 69u8,
                148u8, 247u8, 224u8, 31u8, 207u8, 189u8, 176u8, 244u8, 233u8, 84u8, 213u8,
            ],
            [
                144u8, 241u8, 145u8, 132u8, 147u8, 131u8, 28u8, 27u8, 97u8, 51u8, 72u8,
                151u8, 67u8, 16u8, 51u8, 132u8, 197u8, 96u8, 14u8, 174u8, 121u8, 110u8,
                179u8, 76u8, 81u8, 234u8, 79u8, 43u8, 170u8, 250u8, 79u8, 148u8,
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
                199u8, 245u8, 5u8, 178u8, 243u8, 113u8, 174u8, 33u8, 117u8, 238u8, 73u8,
                19u8, 244u8, 73u8, 158u8, 31u8, 38u8, 51u8, 167u8, 181u8, 147u8, 99u8,
                33u8, 238u8, 209u8, 205u8, 174u8, 182u8, 17u8, 81u8, 129u8, 210u8,
            ],
            [
                213u8, 113u8, 191u8, 131u8, 62u8, 65u8, 85u8, 59u8, 190u8, 38u8, 14u8,
                0u8, 179u8, 175u8, 122u8, 14u8, 145u8, 170u8, 253u8, 108u8, 220u8, 35u8,
                138u8, 128u8, 58u8, 169u8, 172u8, 14u8, 115u8, 239u8, 237u8, 101u8,
            ],
            [
                218u8, 7u8, 93u8, 9u8, 25u8, 141u8, 32u8, 126u8, 58u8, 145u8, 141u8,
                75u8, 141u8, 252u8, 135u8, 223u8, 45u8, 96u8, 160u8, 11u8, 231u8, 3u8,
                253u8, 57u8, 234u8, 172u8, 144u8, 150u8, 45u8, 160u8, 183u8, 240u8,
            ],
            [
                242u8, 28u8, 179u8, 123u8, 231u8, 9u8, 20u8, 138u8, 171u8, 235u8, 210u8,
                120u8, 84u8, 62u8, 98u8, 209u8, 177u8, 230u8, 164u8, 71u8, 127u8, 177u8,
                204u8, 67u8, 224u8, 105u8, 211u8, 238u8, 184u8, 200u8, 127u8, 144u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for ProtocolConfigEvents {
        const NAME: &'static str = "ProtocolConfigEvents";
        const COUNT: usize = 16usize;
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
                    <KmsGenThresholdUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <KmsGenThresholdUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::KmsGenThresholdUpdated)
                }
                Some(
                    <MirrorKmsContextAndEpoch as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <MirrorKmsContextAndEpoch as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::MirrorKmsContextAndEpoch)
                }
                Some(<MirrorKmsEpoch as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <MirrorKmsEpoch as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::MirrorKmsEpoch)
                }
                Some(
                    <MpcThresholdUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <MpcThresholdUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::MpcThresholdUpdated)
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
                Some(
                    <PendingContextAborted as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PendingContextAborted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PendingContextAborted)
                }
                Some(
                    <PendingEpochAborted as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PendingEpochAborted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PendingEpochAborted)
                }
                Some(
                    <PublicDecryptionThresholdUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PublicDecryptionThresholdUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PublicDecryptionThresholdUpdated)
                }
                Some(<Upgraded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Upgraded as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Upgraded)
                }
                Some(
                    <UserDecryptionThresholdUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UserDecryptionThresholdUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::UserDecryptionThresholdUpdated)
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
                Self::KmsGenThresholdUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MirrorKmsContextAndEpoch(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MirrorKmsEpoch(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MpcThresholdUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NewKmsContext(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NewKmsEpoch(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PendingContextAborted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PendingEpochAborted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PublicDecryptionThresholdUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UserDecryptionThresholdUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ActivateEpoch(inner) => {
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
                Self::KmsGenThresholdUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MirrorKmsContextAndEpoch(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MirrorKmsEpoch(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MpcThresholdUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NewKmsContext(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NewKmsEpoch(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PendingContextAborted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PendingEpochAborted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PublicDecryptionThresholdUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UserDecryptionThresholdUpdated(inner) => {
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
        provider: P,
    ) -> ProtocolConfigInstance<P, N> {
        ProtocolConfigInstance::<P, N>::new(address, provider)
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
        Output = alloy_contract::Result<ProtocolConfigInstance<P, N>>,
    > {
        ProtocolConfigInstance::<P, N>::deploy(provider)
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
        ProtocolConfigInstance::<P, N>::deploy_builder(provider)
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
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ProtocolConfigInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`ProtocolConfig`](self) contract instance.

See the [wrapper's documentation](`ProtocolConfigInstance`) for more details.*/
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
        ) -> alloy_contract::Result<ProtocolConfigInstance<P, N>> {
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
    #[automatically_derived]
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
        ///Creates a new call builder for the [`abortPendingContext`] function.
        pub fn abortPendingContext(
            &self,
            kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, abortPendingContextCall, N> {
            self.call_builder(
                &abortPendingContextCall {
                    kmsContextId,
                },
            )
        }
        ///Creates a new call builder for the [`abortPendingEpoch`] function.
        pub fn abortPendingEpoch(
            &self,
            epochId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, abortPendingEpochCall, N> {
            self.call_builder(&abortPendingEpochCall { epochId })
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
            thresholds: <IProtocolConfig::KmsThresholds as alloy::sol_types::SolType>::RustType,
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
        ///Creates a new call builder for the [`initializeFromCanonical`] function.
        pub fn initializeFromCanonical(
            &self,
            canonicalContextId: alloy::sol_types::private::primitives::aliases::U256,
            canonicalEpochId: alloy::sol_types::private::primitives::aliases::U256,
            canonicalKmsNodeParams: alloy::sol_types::private::Vec<
                <KmsNodeParams as alloy::sol_types::SolType>::RustType,
            >,
            canonicalThresholds: <IProtocolConfig::KmsThresholds as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, initializeFromCanonicalCall, N> {
            self.call_builder(
                &initializeFromCanonicalCall {
                    canonicalContextId,
                    canonicalEpochId,
                    canonicalKmsNodeParams,
                    canonicalThresholds,
                },
            )
        }
        ///Creates a new call builder for the [`initializeFromEmptyProxy`] function.
        pub fn initializeFromEmptyProxy(
            &self,
            initialKmsNodeParams: alloy::sol_types::private::Vec<
                <KmsNodeParams as alloy::sol_types::SolType>::RustType,
            >,
            initialThresholds: <IProtocolConfig::KmsThresholds as alloy::sol_types::SolType>::RustType,
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
        ///Creates a new call builder for the [`mirrorKmsContextAndEpoch`] function.
        pub fn mirrorKmsContextAndEpoch(
            &self,
            contextId: alloy::sol_types::private::primitives::aliases::U256,
            epochId: alloy::sol_types::private::primitives::aliases::U256,
            kmsNodeParams: alloy::sol_types::private::Vec<
                <KmsNodeParams as alloy::sol_types::SolType>::RustType,
            >,
            thresholds: <IProtocolConfig::KmsThresholds as alloy::sol_types::SolType>::RustType,
            softwareVersion: alloy::sol_types::private::String,
            pcrValues: alloy::sol_types::private::Vec<
                <PcrValues as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, mirrorKmsContextAndEpochCall, N> {
            self.call_builder(
                &mirrorKmsContextAndEpochCall {
                    contextId,
                    epochId,
                    kmsNodeParams,
                    thresholds,
                    softwareVersion,
                    pcrValues,
                },
            )
        }
        ///Creates a new call builder for the [`mirrorKmsEpoch`] function.
        pub fn mirrorKmsEpoch(
            &self,
            contextId: alloy::sol_types::private::primitives::aliases::U256,
            epochId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, mirrorKmsEpochCall, N> {
            self.call_builder(
                &mirrorKmsEpochCall {
                    contextId,
                    epochId,
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
            kmsNodeParams: alloy::sol_types::private::Vec<
                <KmsNodeParams as alloy::sol_types::SolType>::RustType,
            >,
            thresholds: <IProtocolConfig::KmsThresholds as alloy::sol_types::SolType>::RustType,
            softwareVersion: alloy::sol_types::private::String,
            pcrValues: alloy::sol_types::private::Vec<
                <PcrValues as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, reinitializeV2Call, N> {
            self.call_builder(
                &reinitializeV2Call {
                    kmsNodeParams,
                    thresholds,
                    softwareVersion,
                    pcrValues,
                },
            )
        }
        ///Creates a new call builder for the [`updateKmsGenThresholdForContext`] function.
        pub fn updateKmsGenThresholdForContext(
            &self,
            kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
            threshold: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, updateKmsGenThresholdForContextCall, N> {
            self.call_builder(
                &updateKmsGenThresholdForContextCall {
                    kmsContextId,
                    threshold,
                },
            )
        }
        ///Creates a new call builder for the [`updateMpcThresholdForContext`] function.
        pub fn updateMpcThresholdForContext(
            &self,
            kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
            threshold: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, updateMpcThresholdForContextCall, N> {
            self.call_builder(
                &updateMpcThresholdForContextCall {
                    kmsContextId,
                    threshold,
                },
            )
        }
        ///Creates a new call builder for the [`updatePublicDecryptionThresholdForContext`] function.
        pub fn updatePublicDecryptionThresholdForContext(
            &self,
            kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
            threshold: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            updatePublicDecryptionThresholdForContextCall,
            N,
        > {
            self.call_builder(
                &updatePublicDecryptionThresholdForContextCall {
                    kmsContextId,
                    threshold,
                },
            )
        }
        ///Creates a new call builder for the [`updateUserDecryptionThresholdForContext`] function.
        pub fn updateUserDecryptionThresholdForContext(
            &self,
            kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
            threshold: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            updateUserDecryptionThresholdForContextCall,
            N,
        > {
            self.call_builder(
                &updateUserDecryptionThresholdForContextCall {
                    kmsContextId,
                    threshold,
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
        ///Creates a new event filter for the [`KmsGenThresholdUpdated`] event.
        pub fn KmsGenThresholdUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, KmsGenThresholdUpdated, N> {
            self.event_filter::<KmsGenThresholdUpdated>()
        }
        ///Creates a new event filter for the [`MirrorKmsContextAndEpoch`] event.
        pub fn MirrorKmsContextAndEpoch_filter(
            &self,
        ) -> alloy_contract::Event<&P, MirrorKmsContextAndEpoch, N> {
            self.event_filter::<MirrorKmsContextAndEpoch>()
        }
        ///Creates a new event filter for the [`MirrorKmsEpoch`] event.
        pub fn MirrorKmsEpoch_filter(
            &self,
        ) -> alloy_contract::Event<&P, MirrorKmsEpoch, N> {
            self.event_filter::<MirrorKmsEpoch>()
        }
        ///Creates a new event filter for the [`MpcThresholdUpdated`] event.
        pub fn MpcThresholdUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, MpcThresholdUpdated, N> {
            self.event_filter::<MpcThresholdUpdated>()
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
        ///Creates a new event filter for the [`PendingContextAborted`] event.
        pub fn PendingContextAborted_filter(
            &self,
        ) -> alloy_contract::Event<&P, PendingContextAborted, N> {
            self.event_filter::<PendingContextAborted>()
        }
        ///Creates a new event filter for the [`PendingEpochAborted`] event.
        pub fn PendingEpochAborted_filter(
            &self,
        ) -> alloy_contract::Event<&P, PendingEpochAborted, N> {
            self.event_filter::<PendingEpochAborted>()
        }
        ///Creates a new event filter for the [`PublicDecryptionThresholdUpdated`] event.
        pub fn PublicDecryptionThresholdUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, PublicDecryptionThresholdUpdated, N> {
            self.event_filter::<PublicDecryptionThresholdUpdated>()
        }
        ///Creates a new event filter for the [`Upgraded`] event.
        pub fn Upgraded_filter(&self) -> alloy_contract::Event<&P, Upgraded, N> {
            self.event_filter::<Upgraded>()
        }
        ///Creates a new event filter for the [`UserDecryptionThresholdUpdated`] event.
        pub fn UserDecryptionThresholdUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, UserDecryptionThresholdUpdated, N> {
            self.event_filter::<UserDecryptionThresholdUpdated>()
        }
    }
}
