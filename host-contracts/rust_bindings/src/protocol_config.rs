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
            pub const NAME: &'static str = stringify!(KeyType);
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                    "EpochKeyResult(uint256 prepKeygenId,uint256 keyId,KeyDigest[] keyDigests,bytes signature)",
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
    struct KmsThresholds {
        uint256 publicDecryption;
        uint256 userDecryption;
        uint256 kmsGen;
        uint256 mpc;
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
    error KmsContextNotCreated(uint256 kmsContextId);
    error KmsContextNotPending(uint256 kmsContextId);
    error KmsLifecycleOperationInFlight(uint256 kmsContextId, uint256 epochId);
    error KmsNodeNullSigner();
    error KmsNodeNullTxSender();
    error KmsSignerAlreadyRegistered(address signer);
    error KmsSignerSetExceedsProofFormatLimit(uint256 signerCount, uint256 maxAllowed);
    error KmsTxSenderAlreadyRegistered(address txSender);
    error LatestActiveKmsContextCannotBeDestroyed(uint256 kmsContextId);
    error LatestActiveKmsEpochCannotBeDestroyed(uint256 epochId);
    error NonIncreasingEpochId(uint256 epochId, uint256 currentEpochId);
    error NonIncreasingKmsContextId(uint256 contextId, uint256 latestActiveKmsContextId);
    error NotHostOwner(address sender);
    error NotInitializing();
    error NotInitializingFromEmptyProxy();
    error ThresholdExceedsProofFormatLimit(string thresholdName, uint256 threshold, uint256 maxAllowed);
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
    event KmsGenThresholdUpdated(uint256 indexed kmsContextId, uint256 threshold);
    event MirrorKmsContextAndEpoch(uint256 indexed contextId, uint256 indexed epochId, KmsNodeParams[] kmsNodeParams, IProtocolConfig.KmsThresholds thresholds, string softwareVersion, PcrValues[] pcrValues);
    event MirrorKmsEpoch(uint256 indexed contextId, uint256 indexed epochId);
    event MpcThresholdUpdated(uint256 indexed kmsContextId, uint256 threshold);
    event NewKmsContext(uint256 indexed contextId, uint256 indexed previousContextId, KmsNodeParams[] kmsNodeParams, IProtocolConfig.KmsThresholds thresholds, string softwareVersion, PcrValues[] pcrValues);
    event NewKmsEpoch(uint256 indexed kmsContextId, uint256 indexed epochId, uint256 previousContextId, uint256 previousEpochId, uint256 materialBlockNumber);
    event PublicDecryptionThresholdUpdated(uint256 indexed kmsContextId, uint256 threshold);
    event Upgraded(address indexed implementation);
    event UserDecryptionThresholdUpdated(uint256 indexed kmsContextId, uint256 threshold);

    constructor();

    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function confirmEpochActivation(uint256 epochId, IProtocolConfig.EpochKeyResult[] memory keys, IProtocolConfig.EpochCrsResult[] memory crsList) external;
    function confirmKmsContextCreation(uint256 kmsContextId) external;
    function defineNewEpochForCurrentKmsContext() external;
    function defineNewKmsContextAndEpoch(KmsNodeParams[] memory kmsNodeParams, IProtocolConfig.KmsThresholds memory thresholds, string memory softwareVersion, PcrValues[] memory pcrValues) external;
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
    function initializeFromCanonical(uint256 canonicalContextId, uint256 canonicalEpochId, KmsNodeParams[] memory canonicalKmsNodeParams, IProtocolConfig.KmsThresholds memory canonicalThresholds) external;
    function initializeFromEmptyProxy(KmsNodeParams[] memory initialKmsNodeParams, IProtocolConfig.KmsThresholds memory initialThresholds, string memory softwareVersion, PcrValues[] memory pcrValues) external;
    function isKmsSigner(address signer) external view returns (bool);
    function isKmsSignerForContext(uint256 kmsContextId, address signer) external view returns (bool);
    function isKmsTxSenderForContext(uint256 kmsContextId, address txSender) external view returns (bool);
    function isLiveKmsContext(uint256 kmsContextId) external view returns (bool);
    function isValidEpochForContext(uint256 kmsContextId, uint256 epochId) external view returns (bool);
    function isValidKmsContext(uint256 kmsContextId) external view returns (bool);
    function mirrorKmsContextAndEpoch(uint256 contextId, uint256 epochId, KmsNodeParams[] memory kmsNodeParams, IProtocolConfig.KmsThresholds memory thresholds, string memory softwareVersion, PcrValues[] memory pcrValues) external;
    function mirrorKmsEpoch(uint256 contextId, uint256 epochId) external;
    function proposeCoprocessorUpgrade(uint256 proposalId, string memory softwareVersion, ChainUpgradeWindow[] memory chainUpgradeWindows, uint64 gwStartBlock) external;
    function proxiableUUID() external view returns (bytes32);
    function reinitializeV2(KmsNodeParams[] memory kmsNodeParams, string memory softwareVersion, PcrValues[] memory pcrValues) external;
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: UPGRADE_INTERFACE_VERSIONReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            #[allow(dead_code)]
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
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
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: getCurrentKmsContextIdReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: getCurrentKmsContextIdCounterReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: getKmsGenThresholdReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: getKmsGenThresholdForContextReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: getKmsNodeForContextReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: getKmsNodesForContextReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: getKmsSignersReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: getKmsSignersForContextReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: getMpcThresholdReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: getMpcThresholdForContextReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: getPublicDecryptionThresholdReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: getPublicDecryptionThresholdForContextReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: getUserDecryptionThresholdReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: getUserDecryptionThresholdForContextReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: getVersionReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            #[allow(dead_code)]
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            #[allow(dead_code)]
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: isKmsSignerReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: isKmsSignerForContextReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: isKmsTxSenderForContextReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: isLiveKmsContextReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: isValidEpochForContextReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: isValidKmsContextReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            #[allow(dead_code)]
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: proxiableUUIDReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `reinitializeV2((address,address,string,string,int32,string,bytes,string)[],string,(bytes,bytes,bytes)[])` and selector `0x77af6987`.
```solidity
function reinitializeV2(KmsNodeParams[] memory kmsNodeParams, string memory softwareVersion, PcrValues[] memory pcrValues) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reinitializeV2Call {
        #[allow(missing_docs)]
        pub kmsNodeParams: alloy::sol_types::private::Vec<
            <KmsNodeParams as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub softwareVersion: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub pcrValues: alloy::sol_types::private::Vec<
            <PcrValues as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`reinitializeV2((address,address,string,string,int32,string,bytes,string)[],string,(bytes,bytes,bytes)[])`](reinitializeV2Call) function.
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<KmsNodeParams>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<PcrValues>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <KmsNodeParams as alloy::sol_types::SolType>::RustType,
                >,
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
                    (value.kmsNodeParams, value.softwareVersion, value.pcrValues)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for reinitializeV2Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        kmsNodeParams: tuple.0,
                        softwareVersion: tuple.1,
                        pcrValues: tuple.2,
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
            const SIGNATURE: &'static str = "reinitializeV2((address,address,string,string,int32,string,bytes,string)[],string,(bytes,bytes,bytes)[])";
            const SELECTOR: [u8; 4] = [119u8, 175u8, 105u8, 135u8];
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
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
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    ///Container for all the [`ProtocolConfig`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
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
        isLiveKmsContext(isLiveKmsContextCall),
        #[allow(missing_docs)]
        isValidEpochForContext(isValidEpochForContextCall),
        #[allow(missing_docs)]
        isValidKmsContext(isValidKmsContextCall),
        #[allow(missing_docs)]
        mirrorKmsContextAndEpoch(mirrorKmsContextAndEpochCall),
        #[allow(missing_docs)]
        mirrorKmsEpoch(mirrorKmsEpochCall),
        #[allow(missing_docs)]
        proposeCoprocessorUpgrade(proposeCoprocessorUpgradeCall),
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
            [119u8, 175u8, 105u8, 135u8],
            [119u8, 211u8, 142u8, 36u8],
            [126u8, 170u8, 200u8, 242u8],
            [142u8, 151u8, 203u8, 96u8],
            [148u8, 71u8, 207u8, 212u8],
            [151u8, 108u8, 152u8, 181u8],
            [151u8, 111u8, 62u8, 185u8],
            [157u8, 27u8, 27u8, 225u8],
            [173u8, 60u8, 177u8, 204u8],
            [176u8, 180u8, 97u8, 196u8],
            [177u8, 129u8, 205u8, 167u8],
            [180u8, 114u8, 43u8, 196u8],
            [188u8, 77u8, 7u8, 194u8],
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
            ::core::stringify!(updatePublicDecryptionThresholdForContext),
            ::core::stringify!(getVersion),
            ::core::stringify!(initializeFromCanonical),
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
            ::core::stringify!(reinitializeV2),
            ::core::stringify!(updateMpcThresholdForContext),
            ::core::stringify!(getKmsSigners),
            ::core::stringify!(mirrorKmsEpoch),
            ::core::stringify!(isKmsSignerForContext),
            ::core::stringify!(defineNewKmsContextAndEpoch),
            ::core::stringify!(getCurrentKmsContextId),
            ::core::stringify!(getCurrentKmsContextIdCounter),
            ::core::stringify!(UPGRADE_INTERFACE_VERSION),
            ::core::stringify!(updateKmsGenThresholdForContext),
            ::core::stringify!(updateUserDecryptionThresholdForContext),
            ::core::stringify!(getKmsGenThreshold),
            ::core::stringify!(mirrorKmsContextAndEpoch),
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
            <updatePublicDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getVersionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initializeFromCanonicalCall as alloy_sol_types::SolCall>::SIGNATURE,
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
            <reinitializeV2Call as alloy_sol_types::SolCall>::SIGNATURE,
            <updateMpcThresholdForContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getKmsSignersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <mirrorKmsEpochCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isKmsSignerForContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <defineNewKmsContextAndEpochCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getCurrentKmsContextIdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getCurrentKmsContextIdCounterCall as alloy_sol_types::SolCall>::SIGNATURE,
            <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SIGNATURE,
            <updateKmsGenThresholdForContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <updateUserDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getKmsGenThresholdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <mirrorKmsContextAndEpochCall as alloy_sol_types::SolCall>::SIGNATURE,
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
        const COUNT: usize = 43usize;
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
                Self::isLiveKmsContext(_) => {
                    <isLiveKmsContextCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::proposeCoprocessorUpgrade(_) => {
                    <proposeCoprocessorUpgradeCall as alloy_sol_types::SolCall>::SELECTOR
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
            Self::abi_decode_raw_with_config(
                selector,
                data,
                alloy_sol_types::abi::AbiDecoderConfig::default(),
            )
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_with_config(
            selector: [u8; 4],
            data: &[u8],
            config: alloy_sol_types::abi::AbiDecoderConfig,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<ProtocolConfigCalls>] = &[
                {
                    fn updatePublicDecryptionThresholdForContext(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <updatePublicDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
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
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getVersionCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::getVersion)
                    }
                    getVersion
                },
                {
                    fn initializeFromCanonical(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <initializeFromCanonicalCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::initializeFromCanonical)
                    }
                    initializeFromCanonical
                },
                {
                    fn defineNewEpochForCurrentKmsContext(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <defineNewEpochForCurrentKmsContextCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::defineNewEpochForCurrentKmsContext)
                    }
                    defineNewEpochForCurrentKmsContext
                },
                {
                    fn isKmsSigner(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <isKmsSignerCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::isKmsSigner)
                    }
                    isKmsSigner
                },
                {
                    fn initializeFromEmptyProxy(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::initializeFromEmptyProxy)
                    }
                    initializeFromEmptyProxy
                },
                {
                    fn getMpcThreshold(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getMpcThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::getMpcThreshold)
                    }
                    getMpcThreshold
                },
                {
                    fn getUserDecryptionThresholdForContext(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getUserDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
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
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getPublicDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::getPublicDecryptionThreshold)
                    }
                    getPublicDecryptionThreshold
                },
                {
                    fn getKmsNodeForContext(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getKmsNodeForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::getKmsNodeForContext)
                    }
                    getKmsNodeForContext
                },
                {
                    fn getContextCreationPreviousTxSenderThreshold(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getContextCreationPreviousTxSenderThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
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
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getKmsGenThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::getKmsGenThresholdForContext)
                    }
                    getKmsGenThresholdForContext
                },
                {
                    fn isKmsTxSenderForContext(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <isKmsTxSenderForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::isKmsTxSenderForContext)
                    }
                    isKmsTxSenderForContext
                },
                {
                    fn getMpcThresholdForContext(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getMpcThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::getMpcThresholdForContext)
                    }
                    getMpcThresholdForContext
                },
                {
                    fn confirmEpochActivation(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <confirmEpochActivationCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::confirmEpochActivation)
                    }
                    confirmEpochActivation
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn getKmsSignersForContext(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getKmsSignersForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::getKmsSignersForContext)
                    }
                    getKmsSignersForContext
                },
                {
                    fn getCurrentKmsContextAndEpoch(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getCurrentKmsContextAndEpochCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::getCurrentKmsContextAndEpoch)
                    }
                    getCurrentKmsContextAndEpoch
                },
                {
                    fn reinitializeV2(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <reinitializeV2Call as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::reinitializeV2)
                    }
                    reinitializeV2
                },
                {
                    fn updateMpcThresholdForContext(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <updateMpcThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::updateMpcThresholdForContext)
                    }
                    updateMpcThresholdForContext
                },
                {
                    fn getKmsSigners(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getKmsSignersCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::getKmsSigners)
                    }
                    getKmsSigners
                },
                {
                    fn mirrorKmsEpoch(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <mirrorKmsEpochCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::mirrorKmsEpoch)
                    }
                    mirrorKmsEpoch
                },
                {
                    fn isKmsSignerForContext(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <isKmsSignerForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::isKmsSignerForContext)
                    }
                    isKmsSignerForContext
                },
                {
                    fn defineNewKmsContextAndEpoch(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <defineNewKmsContextAndEpochCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::defineNewKmsContextAndEpoch)
                    }
                    defineNewKmsContextAndEpoch
                },
                {
                    fn getCurrentKmsContextId(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getCurrentKmsContextIdCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::getCurrentKmsContextId)
                    }
                    getCurrentKmsContextId
                },
                {
                    fn getCurrentKmsContextIdCounter(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getCurrentKmsContextIdCounterCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::getCurrentKmsContextIdCounter)
                    }
                    getCurrentKmsContextIdCounter
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn updateKmsGenThresholdForContext(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <updateKmsGenThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::updateKmsGenThresholdForContext)
                    }
                    updateKmsGenThresholdForContext
                },
                {
                    fn updateUserDecryptionThresholdForContext(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <updateUserDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
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
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getKmsGenThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::getKmsGenThreshold)
                    }
                    getKmsGenThreshold
                },
                {
                    fn mirrorKmsContextAndEpoch(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <mirrorKmsContextAndEpochCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::mirrorKmsContextAndEpoch)
                    }
                    mirrorKmsContextAndEpoch
                },
                {
                    fn isValidKmsContext(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <isValidKmsContextCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::isValidKmsContext)
                    }
                    isValidKmsContext
                },
                {
                    fn destroyKmsContext(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <destroyKmsContextCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::destroyKmsContext)
                    }
                    destroyKmsContext
                },
                {
                    fn destroyKmsEpoch(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <destroyKmsEpochCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::destroyKmsEpoch)
                    }
                    destroyKmsEpoch
                },
                {
                    fn getUserDecryptionThreshold(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getUserDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::getUserDecryptionThreshold)
                    }
                    getUserDecryptionThreshold
                },
                {
                    fn getPublicDecryptionThresholdForContext(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getPublicDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
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
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getKmsContextAnchorCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::getKmsContextAnchor)
                    }
                    getKmsContextAnchor
                },
                {
                    fn proposeCoprocessorUpgrade(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <proposeCoprocessorUpgradeCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::proposeCoprocessorUpgrade)
                    }
                    proposeCoprocessorUpgrade
                },
                {
                    fn isValidEpochForContext(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <isValidEpochForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::isValidEpochForContext)
                    }
                    isValidEpochForContext
                },
                {
                    fn confirmKmsContextCreation(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <confirmKmsContextCreationCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::confirmKmsContextCreation)
                    }
                    confirmKmsContextCreation
                },
                {
                    fn isLiveKmsContext(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <isLiveKmsContextCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigCalls::isLiveKmsContext)
                    }
                    isLiveKmsContext
                },
                {
                    fn getKmsNodesForContext(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getKmsNodesForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
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
            DECODE_SHIMS[idx](data, config)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            Self::abi_decode_raw_with_config(
                selector,
                data,
                alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
            )
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
        KmsContextNotCreated(KmsContextNotCreated),
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
            [34u8, 186u8, 82u8, 219u8],
            [45u8, 236u8, 207u8, 77u8],
            [50u8, 197u8, 185u8, 246u8],
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
            [232u8, 18u8, 31u8, 81u8],
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
            ::core::stringify!(ThresholdExceedsProofFormatLimit),
            ::core::stringify!(KmsNodeNullSigner),
            ::core::stringify!(KmsContextNotCreated),
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
            ::core::stringify!(NonIncreasingEpochId),
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
            <ThresholdExceedsProofFormatLimit as alloy_sol_types::SolError>::SIGNATURE,
            <KmsNodeNullSigner as alloy_sol_types::SolError>::SIGNATURE,
            <KmsContextNotCreated as alloy_sol_types::SolError>::SIGNATURE,
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
            <NonIncreasingEpochId as alloy_sol_types::SolError>::SIGNATURE,
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
                Self::KmsContextNotCreated(_) => {
                    <KmsContextNotCreated as alloy_sol_types::SolError>::SELECTOR
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
            Self::abi_decode_raw_with_config(
                selector,
                data,
                alloy_sol_types::abi::AbiDecoderConfig::default(),
            )
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_with_config(
            selector: [u8; 4],
            data: &[u8],
            config: alloy_sol_types::abi::AbiDecoderConfig,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<ProtocolConfigErrors>] = &[
                {
                    fn EmptyKmsNodes(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <EmptyKmsNodes as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::EmptyKmsNodes)
                    }
                    EmptyKmsNodes
                },
                {
                    fn KmsLifecycleOperationInFlight(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsLifecycleOperationInFlight as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::KmsLifecycleOperationInFlight)
                    }
                    KmsLifecycleOperationInFlight
                },
                {
                    fn InvalidProposalId(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <InvalidProposalId as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::InvalidProposalId)
                    }
                    InvalidProposalId
                },
                {
                    fn KmsSignerSetExceedsProofFormatLimit(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsSignerSetExceedsProofFormatLimit as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
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
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <ZeroGwStartBlock as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::ZeroGwStartBlock)
                    }
                    ZeroGwStartBlock
                },
                {
                    fn LatestActiveKmsContextCannotBeDestroyed(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <LatestActiveKmsContextCannotBeDestroyed as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
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
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <NotHostOwner as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::NotHostOwner)
                    }
                    NotHostOwner
                },
                {
                    fn ThresholdExceedsProofFormatLimit(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <ThresholdExceedsProofFormatLimit as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::ThresholdExceedsProofFormatLimit)
                    }
                    ThresholdExceedsProofFormatLimit
                },
                {
                    fn KmsNodeNullSigner(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsNodeNullSigner as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::KmsNodeNullSigner)
                    }
                    KmsNodeNullSigner
                },
                {
                    fn KmsContextNotCreated(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsContextNotCreated as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::KmsContextNotCreated)
                    }
                    KmsContextNotCreated
                },
                {
                    fn KmsContextNotPending(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsContextNotPending as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::KmsContextNotPending)
                    }
                    KmsContextNotPending
                },
                {
                    fn InvalidNullThreshold(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <InvalidNullThreshold as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::InvalidNullThreshold)
                    }
                    InvalidNullThreshold
                },
                {
                    fn EpochActivationAlreadyConfirmed(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <EpochActivationAlreadyConfirmed as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::EpochActivationAlreadyConfirmed)
                    }
                    EpochActivationAlreadyConfirmed
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn KmsContextCreationAlreadyConfirmed(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsContextCreationAlreadyConfirmed as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
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
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <DuplicateChainId as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::DuplicateChainId)
                    }
                    DuplicateChainId
                },
                {
                    fn NotInitializingFromEmptyProxy(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::NotInitializingFromEmptyProxy)
                    }
                    NotInitializingFromEmptyProxy
                },
                {
                    fn InvalidKmsContext(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <InvalidKmsContext as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::InvalidKmsContext)
                    }
                    InvalidKmsContext
                },
                {
                    fn InvalidKmsEpoch(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <InvalidKmsEpoch as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::InvalidKmsEpoch)
                    }
                    InvalidKmsEpoch
                },
                {
                    fn KmsNodeNullTxSender(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsNodeNullTxSender as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::KmsNodeNullTxSender)
                    }
                    KmsNodeNullTxSender
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn EpochActivationUnauthorized(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <EpochActivationUnauthorized as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::EpochActivationUnauthorized)
                    }
                    EpochActivationUnauthorized
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn EmptySoftwareVersion(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <EmptySoftwareVersion as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::EmptySoftwareVersion)
                    }
                    EmptySoftwareVersion
                },
                {
                    fn KmsContextCreationUnauthorized(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsContextCreationUnauthorized as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::KmsContextCreationUnauthorized)
                    }
                    KmsContextCreationUnauthorized
                },
                {
                    fn EmptyChainUpgradeWindows(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <EmptyChainUpgradeWindows as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::EmptyChainUpgradeWindows)
                    }
                    EmptyChainUpgradeWindows
                },
                {
                    fn ZeroChainId(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <ZeroChainId as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::ZeroChainId)
                    }
                    ZeroChainId
                },
                {
                    fn InvalidHighThreshold(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <InvalidHighThreshold as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::InvalidHighThreshold)
                    }
                    InvalidHighThreshold
                },
                {
                    fn EmptyEpochActivationAttestation(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <EmptyEpochActivationAttestation as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::EmptyEpochActivationAttestation)
                    }
                    EmptyEpochActivationAttestation
                },
                {
                    fn KmsTxSenderAlreadyRegistered(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsTxSenderAlreadyRegistered as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::KmsTxSenderAlreadyRegistered)
                    }
                    KmsTxSenderAlreadyRegistered
                },
                {
                    fn FailedCall(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn NonIncreasingEpochId(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <NonIncreasingEpochId as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::NonIncreasingEpochId)
                    }
                    NonIncreasingEpochId
                },
                {
                    fn NonIncreasingKmsContextId(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <NonIncreasingKmsContextId as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::NonIncreasingKmsContextId)
                    }
                    NonIncreasingKmsContextId
                },
                {
                    fn LatestActiveKmsEpochCannotBeDestroyed(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <LatestActiveKmsEpochCannotBeDestroyed as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
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
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <EpochActivationSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
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
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <InvalidBlockWindow as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::InvalidBlockWindow)
                    }
                    InvalidBlockWindow
                },
                {
                    fn KmsSignerAlreadyRegistered(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KmsSignerAlreadyRegistered as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::KmsSignerAlreadyRegistered)
                    }
                    KmsSignerAlreadyRegistered
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolConfigErrors::InvalidInitialization)
                    }
                    InvalidInitialization
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
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
            DECODE_SHIMS[idx](data, config)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            Self::abi_decode_raw_with_config(
                selector,
                data,
                alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
            )
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
    #[automatically_derived]
    impl ProtocolConfigErrors {
        /**Creates a [`AddressEmptyCode`] error.

```solidity
error AddressEmptyCode(address)
```*/
        #[inline]
        pub fn address_empty_code(target: alloy::sol_types::private::Address) -> Self {
            Self::AddressEmptyCode(AddressEmptyCode { target: target })
        }
        /**Creates a [`DuplicateChainId`] error.

```solidity
error DuplicateChainId(uint64)
```*/
        #[inline]
        pub fn duplicate_chain_id(chain_id: u64) -> Self {
            Self::DuplicateChainId(DuplicateChainId {
                chainId: chain_id,
            })
        }
        /**Creates a [`ECDSAInvalidSignature`] error.

```solidity
error ECDSAInvalidSignature()
```*/
        #[inline]
        pub fn ecdsa_invalid_signature() -> Self {
            Self::ECDSAInvalidSignature(ECDSAInvalidSignature)
        }
        /**Creates a [`ECDSAInvalidSignatureLength`] error.

```solidity
error ECDSAInvalidSignatureLength(uint256)
```*/
        #[inline]
        pub fn ecdsa_invalid_signature_length(
            length: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::ECDSAInvalidSignatureLength(ECDSAInvalidSignatureLength {
                length: length,
            })
        }
        /**Creates a [`ECDSAInvalidSignatureS`] error.

```solidity
error ECDSAInvalidSignatureS(bytes32)
```*/
        #[inline]
        pub fn ecdsa_invalid_signature_s(
            s: alloy::sol_types::private::FixedBytes<32>,
        ) -> Self {
            Self::ECDSAInvalidSignatureS(ECDSAInvalidSignatureS { s: s })
        }
        /**Creates a [`ERC1967InvalidImplementation`] error.

```solidity
error ERC1967InvalidImplementation(address)
```*/
        #[inline]
        pub fn erc_1967_invalid_implementation(
            implementation: alloy::sol_types::private::Address,
        ) -> Self {
            Self::ERC1967InvalidImplementation(ERC1967InvalidImplementation {
                implementation: implementation,
            })
        }
        /**Creates a [`ERC1967NonPayable`] error.

```solidity
error ERC1967NonPayable()
```*/
        #[inline]
        pub fn erc_1967_non_payable() -> Self {
            Self::ERC1967NonPayable(ERC1967NonPayable)
        }
        /**Creates a [`EmptyChainUpgradeWindows`] error.

```solidity
error EmptyChainUpgradeWindows()
```*/
        #[inline]
        pub fn empty_chain_upgrade_windows() -> Self {
            Self::EmptyChainUpgradeWindows(EmptyChainUpgradeWindows)
        }
        /**Creates a [`EmptyEpochActivationAttestation`] error.

```solidity
error EmptyEpochActivationAttestation(uint256)
```*/
        #[inline]
        pub fn empty_epoch_activation_attestation(
            epoch_id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::EmptyEpochActivationAttestation(EmptyEpochActivationAttestation {
                epochId: epoch_id,
            })
        }
        /**Creates a [`EmptyKmsNodes`] error.

```solidity
error EmptyKmsNodes()
```*/
        #[inline]
        pub fn empty_kms_nodes() -> Self {
            Self::EmptyKmsNodes(EmptyKmsNodes)
        }
        /**Creates a [`EmptySoftwareVersion`] error.

```solidity
error EmptySoftwareVersion()
```*/
        #[inline]
        pub fn empty_software_version() -> Self {
            Self::EmptySoftwareVersion(EmptySoftwareVersion)
        }
        /**Creates a [`EpochActivationAlreadyConfirmed`] error.

```solidity
error EpochActivationAlreadyConfirmed(address,uint256)
```*/
        #[inline]
        pub fn epoch_activation_already_confirmed(
            signer: alloy::sol_types::private::Address,
            epoch_id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::EpochActivationAlreadyConfirmed(EpochActivationAlreadyConfirmed {
                signer: signer,
                epochId: epoch_id,
            })
        }
        /**Creates a [`EpochActivationSignerDoesNotMatchTxSender`] error.

```solidity
error EpochActivationSignerDoesNotMatchTxSender(address,address)
```*/
        #[inline]
        pub fn epoch_activation_signer_does_not_match_tx_sender(
            signer: alloy::sol_types::private::Address,
            tx_sender: alloy::sol_types::private::Address,
        ) -> Self {
            Self::EpochActivationSignerDoesNotMatchTxSender(EpochActivationSignerDoesNotMatchTxSender {
                signer: signer,
                txSender: tx_sender,
            })
        }
        /**Creates a [`EpochActivationUnauthorized`] error.

```solidity
error EpochActivationUnauthorized(address,uint256)
```*/
        #[inline]
        pub fn epoch_activation_unauthorized(
            caller: alloy::sol_types::private::Address,
            epoch_id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::EpochActivationUnauthorized(EpochActivationUnauthorized {
                caller: caller,
                epochId: epoch_id,
            })
        }
        /**Creates a [`FailedCall`] error.

```solidity
error FailedCall()
```*/
        #[inline]
        pub fn failed_call() -> Self {
            Self::FailedCall(FailedCall)
        }
        /**Creates a [`InvalidBlockWindow`] error.

```solidity
error InvalidBlockWindow(uint64,uint64,uint64)
```*/
        #[inline]
        pub fn invalid_block_window(
            chain_id: u64,
            start_block: u64,
            end_block: u64,
        ) -> Self {
            Self::InvalidBlockWindow(InvalidBlockWindow {
                chainId: chain_id,
                startBlock: start_block,
                endBlock: end_block,
            })
        }
        /**Creates a [`InvalidHighThreshold`] error.

```solidity
error InvalidHighThreshold(string,uint256,uint256)
```*/
        #[inline]
        pub fn invalid_high_threshold(
            threshold_name: alloy::sol_types::private::String,
            threshold: alloy::sol_types::private::primitives::aliases::U256,
            node_count: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::InvalidHighThreshold(InvalidHighThreshold {
                thresholdName: threshold_name,
                threshold: threshold,
                nodeCount: node_count,
            })
        }
        /**Creates a [`InvalidInitialization`] error.

```solidity
error InvalidInitialization()
```*/
        #[inline]
        pub fn invalid_initialization() -> Self {
            Self::InvalidInitialization(InvalidInitialization)
        }
        /**Creates a [`InvalidKmsContext`] error.

```solidity
error InvalidKmsContext(uint256)
```*/
        #[inline]
        pub fn invalid_kms_context(
            kms_context_id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::InvalidKmsContext(InvalidKmsContext {
                kmsContextId: kms_context_id,
            })
        }
        /**Creates a [`InvalidKmsEpoch`] error.

```solidity
error InvalidKmsEpoch(uint256)
```*/
        #[inline]
        pub fn invalid_kms_epoch(
            epoch_id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::InvalidKmsEpoch(InvalidKmsEpoch {
                epochId: epoch_id,
            })
        }
        /**Creates a [`InvalidNullThreshold`] error.

```solidity
error InvalidNullThreshold(string)
```*/
        #[inline]
        pub fn invalid_null_threshold(
            threshold_name: alloy::sol_types::private::String,
        ) -> Self {
            Self::InvalidNullThreshold(InvalidNullThreshold {
                thresholdName: threshold_name,
            })
        }
        /**Creates a [`InvalidProposalId`] error.

```solidity
error InvalidProposalId()
```*/
        #[inline]
        pub fn invalid_proposal_id() -> Self {
            Self::InvalidProposalId(InvalidProposalId)
        }
        /**Creates a [`KmsContextCreationAlreadyConfirmed`] error.

```solidity
error KmsContextCreationAlreadyConfirmed(address,uint256)
```*/
        #[inline]
        pub fn kms_context_creation_already_confirmed(
            tx_sender: alloy::sol_types::private::Address,
            kms_context_id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::KmsContextCreationAlreadyConfirmed(KmsContextCreationAlreadyConfirmed {
                txSender: tx_sender,
                kmsContextId: kms_context_id,
            })
        }
        /**Creates a [`KmsContextCreationUnauthorized`] error.

```solidity
error KmsContextCreationUnauthorized(address,uint256)
```*/
        #[inline]
        pub fn kms_context_creation_unauthorized(
            caller: alloy::sol_types::private::Address,
            kms_context_id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::KmsContextCreationUnauthorized(KmsContextCreationUnauthorized {
                caller: caller,
                kmsContextId: kms_context_id,
            })
        }
        /**Creates a [`KmsContextNotCreated`] error.

```solidity
error KmsContextNotCreated(uint256)
```*/
        #[inline]
        pub fn kms_context_not_created(
            kms_context_id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::KmsContextNotCreated(KmsContextNotCreated {
                kmsContextId: kms_context_id,
            })
        }
        /**Creates a [`KmsContextNotPending`] error.

```solidity
error KmsContextNotPending(uint256)
```*/
        #[inline]
        pub fn kms_context_not_pending(
            kms_context_id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::KmsContextNotPending(KmsContextNotPending {
                kmsContextId: kms_context_id,
            })
        }
        /**Creates a [`KmsLifecycleOperationInFlight`] error.

```solidity
error KmsLifecycleOperationInFlight(uint256,uint256)
```*/
        #[inline]
        pub fn kms_lifecycle_operation_in_flight(
            kms_context_id: alloy::sol_types::private::primitives::aliases::U256,
            epoch_id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::KmsLifecycleOperationInFlight(KmsLifecycleOperationInFlight {
                kmsContextId: kms_context_id,
                epochId: epoch_id,
            })
        }
        /**Creates a [`KmsNodeNullSigner`] error.

```solidity
error KmsNodeNullSigner()
```*/
        #[inline]
        pub fn kms_node_null_signer() -> Self {
            Self::KmsNodeNullSigner(KmsNodeNullSigner)
        }
        /**Creates a [`KmsNodeNullTxSender`] error.

```solidity
error KmsNodeNullTxSender()
```*/
        #[inline]
        pub fn kms_node_null_tx_sender() -> Self {
            Self::KmsNodeNullTxSender(KmsNodeNullTxSender)
        }
        /**Creates a [`KmsSignerAlreadyRegistered`] error.

```solidity
error KmsSignerAlreadyRegistered(address)
```*/
        #[inline]
        pub fn kms_signer_already_registered(
            signer: alloy::sol_types::private::Address,
        ) -> Self {
            Self::KmsSignerAlreadyRegistered(KmsSignerAlreadyRegistered {
                signer: signer,
            })
        }
        /**Creates a [`KmsSignerSetExceedsProofFormatLimit`] error.

```solidity
error KmsSignerSetExceedsProofFormatLimit(uint256,uint256)
```*/
        #[inline]
        pub fn kms_signer_set_exceeds_proof_format_limit(
            signer_count: alloy::sol_types::private::primitives::aliases::U256,
            max_allowed: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::KmsSignerSetExceedsProofFormatLimit(KmsSignerSetExceedsProofFormatLimit {
                signerCount: signer_count,
                maxAllowed: max_allowed,
            })
        }
        /**Creates a [`KmsTxSenderAlreadyRegistered`] error.

```solidity
error KmsTxSenderAlreadyRegistered(address)
```*/
        #[inline]
        pub fn kms_tx_sender_already_registered(
            tx_sender: alloy::sol_types::private::Address,
        ) -> Self {
            Self::KmsTxSenderAlreadyRegistered(KmsTxSenderAlreadyRegistered {
                txSender: tx_sender,
            })
        }
        /**Creates a [`LatestActiveKmsContextCannotBeDestroyed`] error.

```solidity
error LatestActiveKmsContextCannotBeDestroyed(uint256)
```*/
        #[inline]
        pub fn latest_active_kms_context_cannot_be_destroyed(
            kms_context_id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::LatestActiveKmsContextCannotBeDestroyed(LatestActiveKmsContextCannotBeDestroyed {
                kmsContextId: kms_context_id,
            })
        }
        /**Creates a [`LatestActiveKmsEpochCannotBeDestroyed`] error.

```solidity
error LatestActiveKmsEpochCannotBeDestroyed(uint256)
```*/
        #[inline]
        pub fn latest_active_kms_epoch_cannot_be_destroyed(
            epoch_id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::LatestActiveKmsEpochCannotBeDestroyed(LatestActiveKmsEpochCannotBeDestroyed {
                epochId: epoch_id,
            })
        }
        /**Creates a [`NonIncreasingEpochId`] error.

```solidity
error NonIncreasingEpochId(uint256,uint256)
```*/
        #[inline]
        pub fn non_increasing_epoch_id(
            epoch_id: alloy::sol_types::private::primitives::aliases::U256,
            current_epoch_id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::NonIncreasingEpochId(NonIncreasingEpochId {
                epochId: epoch_id,
                currentEpochId: current_epoch_id,
            })
        }
        /**Creates a [`NonIncreasingKmsContextId`] error.

```solidity
error NonIncreasingKmsContextId(uint256,uint256)
```*/
        #[inline]
        pub fn non_increasing_kms_context_id(
            context_id: alloy::sol_types::private::primitives::aliases::U256,
            latest_active_kms_context_id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::NonIncreasingKmsContextId(NonIncreasingKmsContextId {
                contextId: context_id,
                latestActiveKmsContextId: latest_active_kms_context_id,
            })
        }
        /**Creates a [`NotHostOwner`] error.

```solidity
error NotHostOwner(address)
```*/
        #[inline]
        pub fn not_host_owner(sender: alloy::sol_types::private::Address) -> Self {
            Self::NotHostOwner(NotHostOwner { sender: sender })
        }
        /**Creates a [`NotInitializing`] error.

```solidity
error NotInitializing()
```*/
        #[inline]
        pub fn not_initializing() -> Self {
            Self::NotInitializing(NotInitializing)
        }
        /**Creates a [`NotInitializingFromEmptyProxy`] error.

```solidity
error NotInitializingFromEmptyProxy()
```*/
        #[inline]
        pub fn not_initializing_from_empty_proxy() -> Self {
            Self::NotInitializingFromEmptyProxy(NotInitializingFromEmptyProxy)
        }
        /**Creates a [`ThresholdExceedsProofFormatLimit`] error.

```solidity
error ThresholdExceedsProofFormatLimit(string,uint256,uint256)
```*/
        #[inline]
        pub fn threshold_exceeds_proof_format_limit(
            threshold_name: alloy::sol_types::private::String,
            threshold: alloy::sol_types::private::primitives::aliases::U256,
            max_allowed: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::ThresholdExceedsProofFormatLimit(ThresholdExceedsProofFormatLimit {
                thresholdName: threshold_name,
                threshold: threshold,
                maxAllowed: max_allowed,
            })
        }
        /**Creates a [`UUPSUnauthorizedCallContext`] error.

```solidity
error UUPSUnauthorizedCallContext()
```*/
        #[inline]
        pub fn uups_unauthorized_call_context() -> Self {
            Self::UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext)
        }
        /**Creates a [`UUPSUnsupportedProxiableUUID`] error.

```solidity
error UUPSUnsupportedProxiableUUID(bytes32)
```*/
        #[inline]
        pub fn uups_unsupported_proxiable_uuid(
            slot: alloy::sol_types::private::FixedBytes<32>,
        ) -> Self {
            Self::UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID {
                slot: slot,
            })
        }
        /**Creates a [`ZeroChainId`] error.

```solidity
error ZeroChainId()
```*/
        #[inline]
        pub fn zero_chain_id() -> Self {
            Self::ZeroChainId(ZeroChainId)
        }
        /**Creates a [`ZeroGwStartBlock`] error.

```solidity
error ZeroGwStartBlock()
```*/
        #[inline]
        pub fn zero_gw_start_block() -> Self {
            Self::ZeroGwStartBlock(ZeroGwStartBlock)
        }
    }
    ///Container for all the [`ProtocolConfig`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
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
        PublicDecryptionThresholdUpdated(PublicDecryptionThresholdUpdated),
        #[allow(missing_docs)]
        Upgraded(Upgraded),
        #[allow(missing_docs)]
        UserDecryptionThresholdUpdated(UserDecryptionThresholdUpdated),
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
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(KmsEpochDestroyed),
            ::core::stringify!(MirrorKmsEpoch),
            ::core::stringify!(MpcThresholdUpdated),
            ::core::stringify!(NewKmsEpoch),
            ::core::stringify!(ActivateEpoch),
            ::core::stringify!(NewKmsContext),
            ::core::stringify!(MirrorKmsContextAndEpoch),
            ::core::stringify!(EpochActivationConfirmation),
            ::core::stringify!(UserDecryptionThresholdUpdated),
            ::core::stringify!(KmsContextCreationConfirmation),
            ::core::stringify!(Upgraded),
            ::core::stringify!(CoprocessorUpgradeProposed),
            ::core::stringify!(Initialized),
            ::core::stringify!(PublicDecryptionThresholdUpdated),
            ::core::stringify!(KmsContextDestroyed),
            ::core::stringify!(KmsGenThresholdUpdated),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <KmsEpochDestroyed as alloy_sol_types::SolEvent>::SIGNATURE,
            <MirrorKmsEpoch as alloy_sol_types::SolEvent>::SIGNATURE,
            <MpcThresholdUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
            <NewKmsEpoch as alloy_sol_types::SolEvent>::SIGNATURE,
            <ActivateEpoch as alloy_sol_types::SolEvent>::SIGNATURE,
            <NewKmsContext as alloy_sol_types::SolEvent>::SIGNATURE,
            <MirrorKmsContextAndEpoch as alloy_sol_types::SolEvent>::SIGNATURE,
            <EpochActivationConfirmation as alloy_sol_types::SolEvent>::SIGNATURE,
            <UserDecryptionThresholdUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
            <KmsContextCreationConfirmation as alloy_sol_types::SolEvent>::SIGNATURE,
            <Upgraded as alloy_sol_types::SolEvent>::SIGNATURE,
            <CoprocessorUpgradeProposed as alloy_sol_types::SolEvent>::SIGNATURE,
            <Initialized as alloy_sol_types::SolEvent>::SIGNATURE,
            <PublicDecryptionThresholdUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
            <KmsContextDestroyed as alloy_sol_types::SolEvent>::SIGNATURE,
            <KmsGenThresholdUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
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
    #[automatically_derived]
    impl ProtocolConfigEvents {
        /**Creates a [`ActivateEpoch`] event.

```solidity
event ActivateEpoch(uint256,uint256,(uint256,uint256,(uint8,bytes)[],bytes)[],(uint256,uint256,bytes,bytes)[],string[])
```*/
        #[inline]
        pub fn activate_epoch(
            kms_context_id: alloy::sol_types::private::primitives::aliases::U256,
            epoch_id: alloy::sol_types::private::primitives::aliases::U256,
            keys: alloy::sol_types::private::Vec<
                <IProtocolConfig::EpochKeyResult as alloy::sol_types::SolType>::RustType,
            >,
            crs_list: alloy::sol_types::private::Vec<
                <IProtocolConfig::EpochCrsResult as alloy::sol_types::SolType>::RustType,
            >,
            kms_node_storage_urls: alloy::sol_types::private::Vec<
                alloy::sol_types::private::String,
            >,
        ) -> Self {
            Self::ActivateEpoch(ActivateEpoch {
                kmsContextId: kms_context_id,
                epochId: epoch_id,
                keys: keys,
                crsList: crs_list,
                kmsNodeStorageUrls: kms_node_storage_urls,
            })
        }
        /**Creates a [`CoprocessorUpgradeProposed`] event.

```solidity
event CoprocessorUpgradeProposed(uint256,string,(uint64,uint64,uint64)[],uint64)
```*/
        #[inline]
        pub fn coprocessor_upgrade_proposed(
            proposal_id: alloy::sol_types::private::primitives::aliases::U256,
            software_version: alloy::sol_types::private::String,
            chain_upgrade_windows: alloy::sol_types::private::Vec<
                <ChainUpgradeWindow as alloy::sol_types::SolType>::RustType,
            >,
            gw_start_block: u64,
        ) -> Self {
            Self::CoprocessorUpgradeProposed(CoprocessorUpgradeProposed {
                proposalId: proposal_id,
                softwareVersion: software_version,
                chainUpgradeWindows: chain_upgrade_windows,
                gwStartBlock: gw_start_block,
            })
        }
        /**Creates a [`EpochActivationConfirmation`] event.

```solidity
event EpochActivationConfirmation(uint256,address,bytes32)
```*/
        #[inline]
        pub fn epoch_activation_confirmation(
            epoch_id: alloy::sol_types::private::primitives::aliases::U256,
            signer: alloy::sol_types::private::Address,
            data_hash: alloy::sol_types::private::FixedBytes<32>,
        ) -> Self {
            Self::EpochActivationConfirmation(EpochActivationConfirmation {
                epochId: epoch_id,
                signer: signer,
                dataHash: data_hash,
            })
        }
        /**Creates a [`Initialized`] event.

```solidity
event Initialized(uint64)
```*/
        #[inline]
        pub fn initialized(version: u64) -> Self {
            Self::Initialized(Initialized { version: version })
        }
        /**Creates a [`KmsContextCreationConfirmation`] event.

```solidity
event KmsContextCreationConfirmation(uint256,address,bool,bool)
```*/
        #[inline]
        pub fn kms_context_creation_confirmation(
            kms_context_id: alloy::sol_types::private::primitives::aliases::U256,
            tx_sender: alloy::sol_types::private::Address,
            is_previous_tx_sender: bool,
            is_new_tx_sender: bool,
        ) -> Self {
            Self::KmsContextCreationConfirmation(KmsContextCreationConfirmation {
                kmsContextId: kms_context_id,
                txSender: tx_sender,
                isPreviousTxSender: is_previous_tx_sender,
                isNewTxSender: is_new_tx_sender,
            })
        }
        /**Creates a [`KmsContextDestroyed`] event.

```solidity
event KmsContextDestroyed(uint256)
```*/
        #[inline]
        pub fn kms_context_destroyed(
            kms_context_id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::KmsContextDestroyed(KmsContextDestroyed {
                kmsContextId: kms_context_id,
            })
        }
        /**Creates a [`KmsEpochDestroyed`] event.

```solidity
event KmsEpochDestroyed(uint256)
```*/
        #[inline]
        pub fn kms_epoch_destroyed(
            epoch_id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::KmsEpochDestroyed(KmsEpochDestroyed {
                epochId: epoch_id,
            })
        }
        /**Creates a [`KmsGenThresholdUpdated`] event.

```solidity
event KmsGenThresholdUpdated(uint256,uint256)
```*/
        #[inline]
        pub fn kms_gen_threshold_updated(
            kms_context_id: alloy::sol_types::private::primitives::aliases::U256,
            threshold: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::KmsGenThresholdUpdated(KmsGenThresholdUpdated {
                kmsContextId: kms_context_id,
                threshold: threshold,
            })
        }
        /**Creates a [`MirrorKmsContextAndEpoch`] event.

```solidity
event MirrorKmsContextAndEpoch(uint256,uint256,(address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[])
```*/
        #[inline]
        pub fn mirror_kms_context_and_epoch(
            context_id: alloy::sol_types::private::primitives::aliases::U256,
            epoch_id: alloy::sol_types::private::primitives::aliases::U256,
            kms_node_params: alloy::sol_types::private::Vec<
                <KmsNodeParams as alloy::sol_types::SolType>::RustType,
            >,
            thresholds: <IProtocolConfig::KmsThresholds as alloy::sol_types::SolType>::RustType,
            software_version: alloy::sol_types::private::String,
            pcr_values: alloy::sol_types::private::Vec<
                <PcrValues as alloy::sol_types::SolType>::RustType,
            >,
        ) -> Self {
            Self::MirrorKmsContextAndEpoch(MirrorKmsContextAndEpoch {
                contextId: context_id,
                epochId: epoch_id,
                kmsNodeParams: kms_node_params,
                thresholds: thresholds,
                softwareVersion: software_version,
                pcrValues: pcr_values,
            })
        }
        /**Creates a [`MirrorKmsEpoch`] event.

```solidity
event MirrorKmsEpoch(uint256,uint256)
```*/
        #[inline]
        pub fn mirror_kms_epoch(
            context_id: alloy::sol_types::private::primitives::aliases::U256,
            epoch_id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::MirrorKmsEpoch(MirrorKmsEpoch {
                contextId: context_id,
                epochId: epoch_id,
            })
        }
        /**Creates a [`MpcThresholdUpdated`] event.

```solidity
event MpcThresholdUpdated(uint256,uint256)
```*/
        #[inline]
        pub fn mpc_threshold_updated(
            kms_context_id: alloy::sol_types::private::primitives::aliases::U256,
            threshold: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::MpcThresholdUpdated(MpcThresholdUpdated {
                kmsContextId: kms_context_id,
                threshold: threshold,
            })
        }
        /**Creates a [`NewKmsContext`] event.

```solidity
event NewKmsContext(uint256,uint256,(address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[])
```*/
        #[inline]
        pub fn new_kms_context(
            context_id: alloy::sol_types::private::primitives::aliases::U256,
            previous_context_id: alloy::sol_types::private::primitives::aliases::U256,
            kms_node_params: alloy::sol_types::private::Vec<
                <KmsNodeParams as alloy::sol_types::SolType>::RustType,
            >,
            thresholds: <IProtocolConfig::KmsThresholds as alloy::sol_types::SolType>::RustType,
            software_version: alloy::sol_types::private::String,
            pcr_values: alloy::sol_types::private::Vec<
                <PcrValues as alloy::sol_types::SolType>::RustType,
            >,
        ) -> Self {
            Self::NewKmsContext(NewKmsContext {
                contextId: context_id,
                previousContextId: previous_context_id,
                kmsNodeParams: kms_node_params,
                thresholds: thresholds,
                softwareVersion: software_version,
                pcrValues: pcr_values,
            })
        }
        /**Creates a [`NewKmsEpoch`] event.

```solidity
event NewKmsEpoch(uint256,uint256,uint256,uint256,uint256)
```*/
        #[inline]
        pub fn new_kms_epoch(
            kms_context_id: alloy::sol_types::private::primitives::aliases::U256,
            epoch_id: alloy::sol_types::private::primitives::aliases::U256,
            previous_context_id: alloy::sol_types::private::primitives::aliases::U256,
            previous_epoch_id: alloy::sol_types::private::primitives::aliases::U256,
            material_block_number: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::NewKmsEpoch(NewKmsEpoch {
                kmsContextId: kms_context_id,
                epochId: epoch_id,
                previousContextId: previous_context_id,
                previousEpochId: previous_epoch_id,
                materialBlockNumber: material_block_number,
            })
        }
        /**Creates a [`PublicDecryptionThresholdUpdated`] event.

```solidity
event PublicDecryptionThresholdUpdated(uint256,uint256)
```*/
        #[inline]
        pub fn public_decryption_threshold_updated(
            kms_context_id: alloy::sol_types::private::primitives::aliases::U256,
            threshold: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::PublicDecryptionThresholdUpdated(PublicDecryptionThresholdUpdated {
                kmsContextId: kms_context_id,
                threshold: threshold,
            })
        }
        /**Creates a [`Upgraded`] event.

```solidity
event Upgraded(address)
```*/
        #[inline]
        pub fn upgraded(implementation: alloy::sol_types::private::Address) -> Self {
            Self::Upgraded(Upgraded {
                implementation: implementation,
            })
        }
        /**Creates a [`UserDecryptionThresholdUpdated`] event.

```solidity
event UserDecryptionThresholdUpdated(uint256,uint256)
```*/
        #[inline]
        pub fn user_decryption_threshold_updated(
            kms_context_id: alloy::sol_types::private::primitives::aliases::U256,
            threshold: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::UserDecryptionThresholdUpdated(UserDecryptionThresholdUpdated {
                kmsContextId: kms_context_id,
                threshold: threshold,
            })
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
        ///Creates a new call builder for the [`reinitializeV2`] function.
        pub fn reinitializeV2(
            &self,
            kmsNodeParams: alloy::sol_types::private::Vec<
                <KmsNodeParams as alloy::sol_types::SolType>::RustType,
            >,
            softwareVersion: alloy::sol_types::private::String,
            pcrValues: alloy::sol_types::private::Vec<
                <PcrValues as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, reinitializeV2Call, N> {
            self.call_builder(
                &reinitializeV2Call {
                    kmsNodeParams,
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
