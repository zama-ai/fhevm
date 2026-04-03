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
///Module containing a contract's types and functions.
/**

```solidity
library IKMSGenerationMigration {
    struct MigrationState { uint256 prepKeygenCounter; uint256 keyCounter; uint256 crsCounter; uint256 activeKeyId; uint256 activeCrsId; uint256 activePrepKeygenId; uint256 activeKeyIdForPairing; IKMSGeneration.KeyDigest[] activeKeyDigests; bytes activeCrsDigest; address[] keyConsensusTxSenders; bytes32 keyConsensusDigest; address[] crsConsensusTxSenders; bytes32 crsConsensusDigest; address[] prepKeygenConsensusTxSenders; bytes32 prepKeygenConsensusDigest; bool isPrepKeygenDone; bool isKeygenDone; bool isCrsgenDone; uint256 crsMaxBitLength; IKMSGeneration.ParamsType prepKeygenParamsType; IKMSGeneration.ParamsType crsParamsType; bytes prepKeygenExtraData; bytes keygenExtraData; bytes crsgenExtraData; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IKMSGenerationMigration {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    /**```solidity
struct MigrationState { uint256 prepKeygenCounter; uint256 keyCounter; uint256 crsCounter; uint256 activeKeyId; uint256 activeCrsId; uint256 activePrepKeygenId; uint256 activeKeyIdForPairing; IKMSGeneration.KeyDigest[] activeKeyDigests; bytes activeCrsDigest; address[] keyConsensusTxSenders; bytes32 keyConsensusDigest; address[] crsConsensusTxSenders; bytes32 crsConsensusDigest; address[] prepKeygenConsensusTxSenders; bytes32 prepKeygenConsensusDigest; bool isPrepKeygenDone; bool isKeygenDone; bool isCrsgenDone; uint256 crsMaxBitLength; IKMSGeneration.ParamsType prepKeygenParamsType; IKMSGeneration.ParamsType crsParamsType; bytes prepKeygenExtraData; bytes keygenExtraData; bytes crsgenExtraData; }
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
        pub activeKeyIdForPairing: alloy::sol_types::private::primitives::aliases::U256,
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
        pub isPrepKeygenDone: bool,
        #[allow(missing_docs)]
        pub isKeygenDone: bool,
        #[allow(missing_docs)]
        pub isCrsgenDone: bool,
        #[allow(missing_docs)]
        pub crsMaxBitLength: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub prepKeygenParamsType: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub crsParamsType: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub prepKeygenExtraData: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub keygenExtraData: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub crsgenExtraData: alloy::sol_types::private::Bytes,
    }
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Array<IKMSGeneration::KeyDigest>,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Bool,
            alloy::sol_types::sol_data::Bool,
            alloy::sol_types::sol_data::Bool,
            alloy::sol_types::sol_data::Uint<256>,
            IKMSGeneration::ParamsType,
            IKMSGeneration::ParamsType,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
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
            bool,
            bool,
            bool,
            alloy::sol_types::private::primitives::aliases::U256,
            <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
            <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<MigrationState> for UnderlyingRustTuple<'_> {
            fn from(value: MigrationState) -> Self {
                (
                    value.prepKeygenCounter,
                    value.keyCounter,
                    value.crsCounter,
                    value.activeKeyId,
                    value.activeCrsId,
                    value.activePrepKeygenId,
                    value.activeKeyIdForPairing,
                    value.activeKeyDigests,
                    value.activeCrsDigest,
                    value.keyConsensusTxSenders,
                    value.keyConsensusDigest,
                    value.crsConsensusTxSenders,
                    value.crsConsensusDigest,
                    value.prepKeygenConsensusTxSenders,
                    value.prepKeygenConsensusDigest,
                    value.isPrepKeygenDone,
                    value.isKeygenDone,
                    value.isCrsgenDone,
                    value.crsMaxBitLength,
                    value.prepKeygenParamsType,
                    value.crsParamsType,
                    value.prepKeygenExtraData,
                    value.keygenExtraData,
                    value.crsgenExtraData,
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
                    activeKeyIdForPairing: tuple.6,
                    activeKeyDigests: tuple.7,
                    activeCrsDigest: tuple.8,
                    keyConsensusTxSenders: tuple.9,
                    keyConsensusDigest: tuple.10,
                    crsConsensusTxSenders: tuple.11,
                    crsConsensusDigest: tuple.12,
                    prepKeygenConsensusTxSenders: tuple.13,
                    prepKeygenConsensusDigest: tuple.14,
                    isPrepKeygenDone: tuple.15,
                    isKeygenDone: tuple.16,
                    isCrsgenDone: tuple.17,
                    crsMaxBitLength: tuple.18,
                    prepKeygenParamsType: tuple.19,
                    crsParamsType: tuple.20,
                    prepKeygenExtraData: tuple.21,
                    keygenExtraData: tuple.22,
                    crsgenExtraData: tuple.23,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.activeKeyIdForPairing,
                    ),
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
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.isPrepKeygenDone,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.isKeygenDone,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.isCrsgenDone,
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.prepKeygenExtraData,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.keygenExtraData,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.crsgenExtraData,
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
                    "MigrationState(uint256 prepKeygenCounter,uint256 keyCounter,uint256 crsCounter,uint256 activeKeyId,uint256 activeCrsId,uint256 activePrepKeygenId,uint256 activeKeyIdForPairing,IKMSGeneration.KeyDigest[] activeKeyDigests,bytes activeCrsDigest,address[] keyConsensusTxSenders,bytes32 keyConsensusDigest,address[] crsConsensusTxSenders,bytes32 crsConsensusDigest,address[] prepKeygenConsensusTxSenders,bytes32 prepKeygenConsensusDigest,bool isPrepKeygenDone,bool isKeygenDone,bool isCrsgenDone,uint256 crsMaxBitLength,uint8 prepKeygenParamsType,uint8 crsParamsType,bytes prepKeygenExtraData,bytes keygenExtraData,bytes crsgenExtraData)",
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.activeKeyIdForPairing,
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
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.isPrepKeygenDone,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.isKeygenDone,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.isCrsgenDone,
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.prepKeygenExtraData,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.keygenExtraData,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.crsgenExtraData,
                        )
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
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.activeKeyIdForPairing,
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
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.isPrepKeygenDone,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.isKeygenDone,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.isCrsgenDone,
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
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.prepKeygenExtraData,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.keygenExtraData,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.crsgenExtraData,
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
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.activeKeyIdForPairing,
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
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.isPrepKeygenDone,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.isKeygenDone,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.isCrsgenDone,
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
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.prepKeygenExtraData,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.keygenExtraData,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.crsgenExtraData,
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
    /**Creates a new wrapper around an on-chain [`IKMSGenerationMigration`](self) contract instance.

See the [wrapper's documentation](`IKMSGenerationMigrationInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IKMSGenerationMigrationInstance<P, N> {
        IKMSGenerationMigrationInstance::<P, N>::new(address, provider)
    }
    /**A [`IKMSGenerationMigration`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IKMSGenerationMigration`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IKMSGenerationMigrationInstance<
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IKMSGenerationMigrationInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IKMSGenerationMigrationInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IKMSGenerationMigrationInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`IKMSGenerationMigration`](self) contract instance.

See the [wrapper's documentation](`IKMSGenerationMigrationInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> IKMSGenerationMigrationInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IKMSGenerationMigrationInstance<P, N> {
            IKMSGenerationMigrationInstance {
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
    > IKMSGenerationMigrationInstance<P, N> {
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
    > IKMSGenerationMigrationInstance<P, N> {
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

library IKMSGenerationMigration {
    struct MigrationState {
        uint256 prepKeygenCounter;
        uint256 keyCounter;
        uint256 crsCounter;
        uint256 activeKeyId;
        uint256 activeCrsId;
        uint256 activePrepKeygenId;
        uint256 activeKeyIdForPairing;
        IKMSGeneration.KeyDigest[] activeKeyDigests;
        bytes activeCrsDigest;
        address[] keyConsensusTxSenders;
        bytes32 keyConsensusDigest;
        address[] crsConsensusTxSenders;
        bytes32 crsConsensusDigest;
        address[] prepKeygenConsensusTxSenders;
        bytes32 prepKeygenConsensusDigest;
        bool isPrepKeygenDone;
        bool isKeygenDone;
        bool isCrsgenDone;
        uint256 crsMaxBitLength;
        IKMSGeneration.ParamsType prepKeygenParamsType;
        IKMSGeneration.ParamsType crsParamsType;
        bytes prepKeygenExtraData;
        bytes keygenExtraData;
        bytes crsgenExtraData;
    }
}

interface KMSGeneration {
    error AbortCrsgenAlreadyDone(uint256 crsId);
    error AbortCrsgenInvalidId(uint256 crsId);
    error AbortKeygenAlreadyDone(uint256 keyId);
    error AbortKeygenInvalidId(uint256 keyId);
    error AddressEmptyCode(address target);
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
    error InvalidInitialization();
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
    event PrepKeygenRequest(uint256 prepKeygenId, uint256 epochId, IKMSGeneration.ParamsType paramsType, bytes extraData);
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
    function initializeFromMigration(IKMSGenerationMigration.MigrationState memory state) external;
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
        "internalType": "struct IKMSGenerationMigration.MigrationState",
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
            "name": "activeKeyIdForPairing",
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
            "name": "isPrepKeygenDone",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "isKeygenDone",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "isCrsgenDone",
            "type": "bool",
            "internalType": "bool"
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
            "name": "prepKeygenExtraData",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "keygenExtraData",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "crsgenExtraData",
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
    ///0x60a06040523073ffffffffffffffffffffffffffffffffffffffff1660809073ffffffffffffffffffffffffffffffffffffffff1681525034801562000043575f80fd5b50620000546200005a60201b60201c565b620001c4565b5f6200006b6200015e60201b60201c565b9050805f0160089054906101000a900460ff1615620000b6576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b67ffffffffffffffff8016815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff16146200015b5767ffffffffffffffff815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d267ffffffffffffffff604051620001529190620001a9565b60405180910390a15b50565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b5f67ffffffffffffffff82169050919050565b620001a38162000185565b82525050565b5f602082019050620001be5f83018462000198565b92915050565b608051617a26620001eb5f395f8181614091015281816140e601526143880152617a265ff3fe608060405260043610610134575f3560e01c806362978787116100aa578063ad3cb1cc1161006e578063ad3cb1cc146103f9578063baff211e14610423578063c2c1faee1461044d578063c55b872414610475578063caa367db146104b2578063d52f10eb146104da57610134565b806362978787146103125780636c68e71f1461033a578063735b12631461036257806384b0196e1461038c578063936608ae146103bc57610134565b80633c02f834116100fc5780633c02f8341461021857806345af261b146102405780634610ffe81461027c5780634f1ef286146102a457806352d1902d146102c0578063589adb0e146102ea57610134565b80630d8e6e2c1461013857806316c713d9146101625780631703c61a1461019e57806319f4f632146101c657806339f7381014610202575b5f80fd5b348015610143575f80fd5b5061014c610504565b60405161015991906157e7565b60405180910390f35b34801561016d575f80fd5b506101886004803603810190610183919061584b565b61057f565b604051610195919061595d565b60405180910390f35b3480156101a9575f80fd5b506101c460048036038101906101bf919061584b565b610650565b005b3480156101d1575f80fd5b506101ec60048036038101906101e7919061584b565b610897565b6040516101f991906159f0565b60405180910390f35b34801561020d575f80fd5b50610216610a22565b005b348015610223575f80fd5b5061023e60048036038101906102399190615a2c565b610c3c565b005b34801561024b575f80fd5b506102666004803603810190610261919061584b565b610f3b565b60405161027391906159f0565b60405180910390f35b348015610287575f80fd5b506102a2600480360381019061029d9190615b20565b61107d565b005b6102be60048036038101906102b99190615d03565b61176f565b005b3480156102cb575f80fd5b506102d461178e565b6040516102e19190615d75565b60405180910390f35b3480156102f5575f80fd5b50610310600480360381019061030b9190615d8e565b6117bf565b005b34801561031d575f80fd5b5061033860048036038101906103339190615deb565b611ccf565b005b348015610345575f80fd5b50610360600480360381019061035b9190615e9f565b61229a565b005b34801561036d575f80fd5b50610376612c06565b6040516103839190615f00565b60405180910390f35b348015610397575f80fd5b506103a0612cb1565b6040516103b39796959493929190616028565b60405180910390f35b3480156103c7575f80fd5b506103e260048036038101906103dd919061584b565b612dba565b6040516103f092919061633a565b60405180910390f35b348015610404575f80fd5b5061040d613187565b60405161041a91906157e7565b60405180910390f35b34801561042e575f80fd5b506104376131c0565b604051610444919061636f565b60405180910390f35b348015610458575f80fd5b50610473600480360381019061046e919061584b565b6131d7565b005b348015610480575f80fd5b5061049b6004803603810190610496919061584b565b613469565b6040516104a99291906163d0565b60405180910390f35b3480156104bd575f80fd5b506104d860048036038101906104d39190616405565b6137a1565b005b3480156104e5575f80fd5b506104ee613b34565b6040516104fb919061636f565b60405180910390f35b60606040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506105455f613b4b565b61054f6001613b4b565b6105585f613b4b565b60405160200161056b94939291906164fe565b604051602081830303815290604052905090565b60605f61058a613c15565b90505f816003015f8581526020019081526020015f20549050816002015f8581526020019081526020015f205f8281526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561064257602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116105f9575b505050505092505050919050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156106ad573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106d19190616570565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461074057336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401610737919061659b565b60405180910390fd5b5f610749613c15565b90508060090154821180610762575060f86005901b8211155b156107a457816040517fcbe9265600000000000000000000000000000000000000000000000000000000815260040161079b919061636f565b60405180910390fd5b806001015f8381526020019081526020015f205f9054906101000a900460ff161561080657816040517fdf0db5fb0000000000000000000000000000000000000000000000000000000081526004016107fd919061636f565b60405180910390fd5b6001816001015f8481526020019081526020015f205f6101000a81548160ff021916908315150217905550600181600f015f8481526020019081526020015f205f6101000a81548160ff0219169083151502179055507f384f90fefbcfaa68f22e00094aeaa52b2bc693936d2ce1afed1212520b59b58e8260405161088b919061636f565b60405180910390a15050565b5f806108a1613c15565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff1661090457826040517f84de13310000000000000000000000000000000000000000000000000000000081526004016108fb919061636f565b60405180910390fd5b80600f015f8481526020019081526020015f205f9054906101000a900460ff161561096657826040517f84de133100000000000000000000000000000000000000000000000000000000815260040161095d919061636f565b60405180910390fd5b6109708184613c3c565b6109b157826040517f84de13310000000000000000000000000000000000000000000000000000000081526004016109a8919061636f565b60405180910390fd5b5f816006015f8581526020019081526020015f205490505f81036109f85781600d015f8581526020019081526020015f205f9054906101000a900460ff1692505050610a1d565b81600d015f8281526020019081526020015f205f9054906101000a900460ff16925050505b919050565b6001610a2c613c86565b67ffffffffffffffff1614610a6d576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60025f610a78613caa565b9050805f0160089054906101000a900460ff1680610ac057508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15610af7576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff021916908315150217905550610bb06040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506040518060400160405280600181526020017f3100000000000000000000000000000000000000000000000000000000000000815250613cd1565b5f610bb9613c15565b905060f86003901b816004018190555060f86004901b816005018190555060f86005901b8160090181905550505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051610c3091906165d6565b60405180910390a15050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610c99573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cbd9190616570565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610d2c57336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401610d23919061659b565b60405180910390fd5b5f610d35613c15565b90505f8160090154905060f86005901b8114158015610d715750816001015f8281526020019081526020015f205f9054906101000a900460ff16155b15610db357806040517f061ac61d000000000000000000000000000000000000000000000000000000008152600401610daa919061636f565b60405180910390fd5b816009015f815480929190610dc79061661c565b91905055505f826009015490508483600a015f8381526020019081526020015f20819055508383600d015f8381526020019081526020015f205f6101000a81548160ff02191690836001811115610e2157610e2061597d565b5b02179055505f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa158015610e84573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ea89190616677565b90505f8060028383604051602001610ec293929190616702565b60405160208183030381529060405290508086600e015f8681526020019081526020015f209081610ef39190616938565b507f8cf0151393f84fd694c5e315cb74cc05b247de0a454fd9e9129c661efdf9401d84898984604051610f299493929190616a07565b60405180910390a15050505050505050565b5f80610f45613c15565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff16610fa857826040517fda32d00f000000000000000000000000000000000000000000000000000000008152600401610f9f919061636f565b60405180910390fd5b80600f015f8481526020019081526020015f205f9054906101000a900460ff161561100a57826040517fda32d00f000000000000000000000000000000000000000000000000000000008152600401611001919061636f565b60405180910390fd5b6110148184613c3c565b61105557826040517fda32d00f00000000000000000000000000000000000000000000000000000000815260040161104c919061636f565b60405180910390fd5b80600d015f8481526020019081526020015f205f9054906101000a900460ff16915050919050565b5f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa1580156110db573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110ff9190616677565b90507344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166346c5bbbd82336040518363ffffffff1660e01b8152600401611150929190616a51565b602060405180830381865afa15801561116b573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061118f9190616aa2565b6111d057336040517faee863230000000000000000000000000000000000000000000000000000000081526004016111c7919061659b565b60405180910390fd5b5f6111d9613c15565b905080600501548711806111f2575060f86004901b8711155b1561123457866040517fadfab90400000000000000000000000000000000000000000000000000000000815260040161122b919061636f565b60405180910390fd5b5f868690500361127b57866040517fe6f9083b000000000000000000000000000000000000000000000000000000008152600401611272919061636f565b60405180910390fd5b5f61128882898989613ce7565b90505f826006015f8a81526020019081526020015f205490505f81141580156112ce5750826001015f8281526020019081526020015f205f9054906101000a900460ff16155b15611305576040517f6fbcdd2b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f611311838888613db5565b9050835f015f8b81526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16156113b15789816040517f98fb957d0000000000000000000000000000000000000000000000000000000081526004016113a8929190616a51565b60405180910390fd5b6001845f015f8c81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f846002015f8c81526020019081526020015f205f8581526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507f2afe64fb3afde8e2678aea84cf36223f330e2fb1286d37aed573ab9cd1db47c78b8b8b8b8b336040516114d396959493929190616cd9565b60405180910390a1846001015f8c81526020019081526020015f205f9054906101000a900460ff1615801561151157506115108180549050613e1b565b5b15611762576001856001015f8d81526020019081526020015f205f6101000a81548160ff0219169083151502179055505f5b8a8a90508110156115c757856007015f8d81526020019081526020015f208b8b8381811061157457611573616d2e565b5b90506020028101906115869190616d67565b908060018154018082558091505060019003905f5260205f2090600202015f9091909190915081816115b89190616f9a565b50508080600101915050611543565b5083856003015f8d81526020019081526020015f20819055508a85600801819055505f61168c86600e015f8e81526020019081526020015f20805461160b9061676b565b80601f01602080910402602001604051908101604052809291908181526020018280546116379061676b565b80156116825780601f1061165957610100808354040283529160200191611682565b820191905f5260205f20905b81548152906001019060200180831161166557829003601f168201915b5050505050613eac565b90505f611720828480548060200260200160405190810160405280929190818152602001828054801561171157602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116116c8575b50505050508580549050613f4c565b90507feb85c26dbcad46b80a68a0f24cce7c2c90f0a1faded84184138839fc9e80a25b8d828e8e6040516117579493929190616fa8565b60405180910390a150505b5050505050505050505050565b61177761408f565b61178082614175565b61178a8282614268565b5050565b5f611797614386565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b5f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa15801561181d573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118419190616677565b90507344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166346c5bbbd82336040518363ffffffff1660e01b8152600401611892929190616a51565b602060405180830381865afa1580156118ad573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118d19190616aa2565b61191257336040517faee86323000000000000000000000000000000000000000000000000000000008152600401611909919061659b565b60405180910390fd5b5f61191b613c15565b90508060040154851180611934575060f86003901b8511155b1561197657846040517f0ab7f68700000000000000000000000000000000000000000000000000000000815260040161196d919061636f565b60405180910390fd5b5f81600e015f8781526020019081526020015f2080546119959061676b565b80601f01602080910402602001604051908101604052809291908181526020018280546119c19061676b565b8015611a0c5780601f106119e357610100808354040283529160200191611a0c565b820191905f5260205f20905b8154815290600101906020018083116119ef57829003601f168201915b505050505090505f611a1e878361440d565b90505f611a2c828888613db5565b9050835f015f8981526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615611acc5787816040517f33ca1fe3000000000000000000000000000000000000000000000000000000008152600401611ac3929190616a51565b60405180910390fd5b6001845f015f8a81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f846002015f8a81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507f4c715c5734ce5c18c9c12e8496e53d2a65f1ec381d476957f0f596b364a59b0c89898933604051611bea9493929190616fed565b60405180910390a1846001015f8a81526020019081526020015f205f9054906101000a900460ff16158015611c285750611c278180549050613e1b565b5b15611cc4576001856001015f8b81526020019081526020015f205f6101000a81548160ff02191690831515021790555082856003015f8b81526020019081526020015f20819055505f856006015f8b81526020019081526020015f205490507f3a116120cca5d4f073cc1fc31ff26133ab7b0499f2712fa010023b87d5a1f9ee8a8287604051611cba9392919061702b565b60405180910390a1505b505050505050505050565b5f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa158015611d2d573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d519190616677565b90507344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166346c5bbbd82336040518363ffffffff1660e01b8152600401611da2929190616a51565b602060405180830381865afa158015611dbd573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611de19190616aa2565b611e2257336040517faee86323000000000000000000000000000000000000000000000000000000008152600401611e19919061659b565b60405180910390fd5b5f611e2b613c15565b90508060090154871180611e44575060f86005901b8711155b15611e8657866040517f8d8c940a000000000000000000000000000000000000000000000000000000008152600401611e7d919061636f565b60405180910390fd5b5f611e938289898961446f565b90505f611ea1828787613db5565b9050825f015f8a81526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615611f415788816040517ffcf5a6e9000000000000000000000000000000000000000000000000000000008152600401611f38929190616a51565b60405180910390fd5b6001835f015f8b81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f836002015f8b81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507f7bf1b42c10e9497c879620c5b7afced10bda17d8c90b22f0e3bc6b2fd6ced0bd8a8a8a8a8a3360405161206396959493929190617067565b60405180910390a1836001015f8b81526020019081526020015f205f9054906101000a900460ff161580156120a157506120a08180549050613e1b565b5b1561228e576001846001015f8c81526020019081526020015f205f6101000a81548160ff021916908315150217905550888885600b015f8d81526020019081526020015f2091826120f3929190616e79565b5082846003015f8c81526020019081526020015f20819055508984600c01819055505f6121b885600e015f8d81526020019081526020015f2080546121379061676b565b80601f01602080910402602001604051908101604052809291908181526020018280546121639061676b565b80156121ae5780601f10612185576101008083540402835291602001916121ae565b820191905f5260205f20905b81548152906001019060200180831161219157829003601f168201915b5050505050613eac565b90505f61224c828480548060200260200160405190810160405280929190818152602001828054801561223d57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116121f4575b50505050508580549050613f4c565b90507f2258b73faed33fb2e2ea454403bef974920caf682ab3a723484fcf67553b16a28c828d8d60405161228394939291906170bc565b60405180910390a150505b50505050505050505050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156122f7573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061231b9190616570565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461238a57336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401612381919061659b565b60405180910390fd5b60025f612395613caa565b9050805f0160089054906101000a900460ff16806123dd57508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15612414576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055506124cd6040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506040518060400160405280600181526020017f3100000000000000000000000000000000000000000000000000000000000000815250613cd1565b5f6124d6613c15565b905060f86003901b845f013510156124ec575f80fd5b60f86004901b84602001351015612501575f80fd5b60f86005901b84604001351015612516575f80fd5b835f01358160040181905550836020013581600501819055508360400135816009018190555083606001358160080181905550836080013581600c01819055505f8460a001351415801561256e57505f8460c0013514155b156125b4578360c00135816006015f8660a0013581526020019081526020015f20819055508360a00135816006015f8660c0013581526020019081526020015f20819055505b5f846060013514612667575f5b848060e001906125d19190617101565b905081101561266557816007015f866060013581526020019081526020015f20858060e001906126019190617101565b8381811061261257612611616d2e565b5b90506020028101906126249190616d67565b908060018154018082558091505060019003905f5260205f2090600202015f9091909190915081816126569190616f9a565b505080806001019150506125c1565b505b5f84608001351415801561268c57505f848061010001906126889190616e0d565b9050115b156126e957838061010001906126a29190616e0d565b82600b015f876080013581526020019081526020015f2091826126c6929190616e79565b5083610240013581600a015f866080013581526020019081526020015f20819055505b612775818560600135868061012001906127039190617163565b88610140013589806102c0019061271a9190616e0d565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050508a61020001602081019061277091906171d9565b61453d565b6128018185608001358680610160019061278f9190617163565b88610180013589806102e001906127a69190616e0d565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050508a6102200160208101906127fc91906171d9565b61453d565b61288d818560a0013586806101a0019061281b9190617163565b886101c0013589806102a001906128329190616e0d565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050508a6101e001602081019061288891906171d9565b61453d565b5f8460a00135146128da57836101e00160208101906128ac91906171d9565b816001015f8660a0013581526020019081526020015f205f6101000a81548160ff0219169083151502179055505b5f84606001351461292757836102000160208101906128f991906171d9565b816001015f866060013581526020019081526020015f205f6101000a81548160ff0219169083151502179055505b5f846080013514612974578361022001602081019061294691906171d9565b816001015f866080013581526020019081526020015f205f6101000a81548160ff0219169083151502179055505b5f8460a00135146129d157836102600160208101906129939190616405565b81600d015f8660a0013581526020019081526020015f205f6101000a81548160ff021916908360018111156129cb576129ca61597d565b5b02179055505b5f846060013514612a2e57836102600160208101906129f09190616405565b81600d015f866060013581526020019081526020015f205f6101000a81548160ff02191690836001811115612a2857612a2761597d565b5b02179055505b5f846080013514612a8b5783610280016020810190612a4d9190616405565b81600d015f866080013581526020019081526020015f205f6101000a81548160ff02191690836001811115612a8557612a8461597d565b5b02179055505b5f8460a0013514158015612ab057505f84806102a00190612aac9190616e0d565b9050115b15612aec5783806102a00190612ac69190616e0d565b82600e015f8760a0013581526020019081526020015f209182612aea929190616e79565b505b5f846060013514158015612b1157505f84806102c00190612b0d9190616e0d565b9050115b15612b4d5783806102c00190612b279190616e0d565b82600e015f876060013581526020019081526020015f209182612b4b929190616e79565b505b5f846080013514158015612b7257505f84806102e00190612b6e9190616e0d565b9050115b15612bae5783806102e00190612b889190616e0d565b82600e015f876080013581526020019081526020015f209182612bac929190616e79565b505b505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051612bf991906165d6565b60405180910390a1505050565b5f80612c10613c15565b90505f8160050154905060f86004901b8114158015612c4c5750816001015f8281526020019081526020015f205f9054906101000a900460ff16155b15612c5c57600192505050612cae565b5f8260090154905060f86005901b8114158015612c965750826001015f8281526020019081526020015f205f9054906101000a900460ff16155b15612ca75760019350505050612cae565b5f93505050505b90565b5f6060805f805f60605f612cc3614780565b90505f801b815f0154148015612cde57505f801b8160010154145b612d1d576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612d149061724e565b60405180910390fd5b612d256147a7565b612d2d614845565b46305f801b5f67ffffffffffffffff811115612d4c57612d4b615bdf565b5b604051908082528060200260200182016040528015612d7a5781602001602082028036833780820191505090505b507f0f0000000000000000000000000000000000000000000000000000000000000095949392919097509750975097509750975097505090919293949596565b6060805f612dc6613c15565b9050806001015f8581526020019081526020015f205f9054906101000a900460ff16612e2957836040517f84de1331000000000000000000000000000000000000000000000000000000008152600401612e20919061636f565b60405180910390fd5b80600f015f8581526020019081526020015f205f9054906101000a900460ff1615612e8b57836040517f84de1331000000000000000000000000000000000000000000000000000000008152600401612e82919061636f565b60405180910390fd5b5f816003015f8681526020019081526020015f205490505f826002015f8781526020019081526020015f205f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020018280548015612f4257602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311612ef9575b505050505090505f815190505f8103612f9257866040517f84de1331000000000000000000000000000000000000000000000000000000008152600401612f89919061636f565b60405180910390fd5b5f61303585600e015f8a81526020019081526020015f208054612fb49061676b565b80601f0160208091040260200160405190810160405280929190818152602001828054612fe09061676b565b801561302b5780601f106130025761010080835404028352916020019161302b565b820191905f5260205f20905b81548152906001019060200180831161300e57829003601f168201915b5050505050613eac565b90505f613043828585613f4c565b905080866007015f8b81526020019081526020015f2080805480602002602001604051908101604052809291908181526020015f905b82821015613172578382905f5260205f2090600202016040518060400160405290815f82015f9054906101000a900460ff1660018111156130bd576130bc61597d565b5b60018111156130cf576130ce61597d565b5b81526020016001820180546130e39061676b565b80601f016020809104026020016040519081016040528092919081815260200182805461310f9061676b565b801561315a5780601f106131315761010080835404028352916020019161315a565b820191905f5260205f20905b81548152906001019060200180831161313d57829003601f168201915b50505050508152505081526020019060010190613079565b50505050905097509750505050505050915091565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b5f806131ca613c15565b905080600c015491505090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613234573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906132589190616570565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146132c757336040517f21bfda100000000000000000000000000000000000000000000000000000000081526004016132be919061659b565b60405180910390fd5b5f6132d0613c15565b905080600501548211806132e9575060f86004901b8211155b1561332b57816040517ffcf2db7a000000000000000000000000000000000000000000000000000000008152600401613322919061636f565b60405180910390fd5b806001015f8381526020019081526020015f205f9054906101000a900460ff161561338d57816040517f92789b67000000000000000000000000000000000000000000000000000000008152600401613384919061636f565b60405180910390fd5b6001816001015f8481526020019081526020015f205f6101000a81548160ff021916908315150217905550600181600f015f8481526020019081526020015f205f6101000a81548160ff0219169083151502179055505f816006015f8481526020019081526020015f205490505f811461342d576001826001015f8381526020019081526020015f205f6101000a81548160ff0219169083151502179055505b7f2b087b884b35a81d769d1a1e092880f1da56de964e4b339eabcb1f45f5fe32648360405161345c919061636f565b60405180910390a1505050565b6060805f613475613c15565b9050806001015f8581526020019081526020015f205f9054906101000a900460ff166134d857836040517fda32d00f0000000000000000000000000000000000000000000000000000000081526004016134cf919061636f565b60405180910390fd5b80600f015f8581526020019081526020015f205f9054906101000a900460ff161561353a57836040517fda32d00f000000000000000000000000000000000000000000000000000000008152600401613531919061636f565b60405180910390fd5b5f816003015f8681526020019081526020015f205490505f826002015f8781526020019081526020015f205f8381526020019081526020015f208054806020026020016040519081016040528092919081815260200182805480156135f157602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116135a8575b505050505090505f815190505f810361364157866040517fda32d00f000000000000000000000000000000000000000000000000000000008152600401613638919061636f565b60405180910390fd5b5f6136e485600e015f8a81526020019081526020015f2080546136639061676b565b80601f016020809104026020016040519081016040528092919081815260200182805461368f9061676b565b80156136da5780601f106136b1576101008083540402835291602001916136da565b820191905f5260205f20905b8154815290600101906020018083116136bd57829003601f168201915b5050505050613eac565b90505f6136f2828585613f4c565b90508086600b015f8b81526020019081526020015f208080546137149061676b565b80601f01602080910402602001604051908101604052809291908181526020018280546137409061676b565b801561378b5780601f106137625761010080835404028352916020019161378b565b820191905f5260205f20905b81548152906001019060200180831161376e57829003601f168201915b5050505050905097509750505050505050915091565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156137fe573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906138229190616570565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461389157336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401613888919061659b565b60405180910390fd5b5f61389a613c15565b90505f8160050154905060f86004901b81141580156138d65750816001015f8281526020019081526020015f205f9054906101000a900460ff16155b1561391857806040517f3b853da800000000000000000000000000000000000000000000000000000000815260040161390f919061636f565b60405180910390fd5b816004015f81548092919061392c9061661c565b91905055505f82600401549050826005015f81548092919061394d9061661c565b91905055505f8360050154905080846006015f8481526020019081526020015f208190555081846006015f8381526020019081526020015f20819055505f8585600d015f8581526020019081526020015f205f6101000a81548160ff021916908360018111156139c0576139bf61597d565b5b02179055508585600d015f8481526020019081526020015f205f6101000a81548160ff021916908360018111156139fa576139f961597d565b5b02179055505f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa158015613a5d573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613a819190616677565b90505f60028284604051602001613a9a93929190616702565b60405160208183030381529060405290508087600e015f8781526020019081526020015f209081613acb9190616938565b508087600e015f8681526020019081526020015f209081613aec9190616938565b507fc0ada6fd1b65690a546d49df7531a21c52ed502d14efd98ee0c8f4846abb931085848a84604051613b229493929190616a07565b60405180910390a15050505050505050565b5f80613b3e613c15565b9050806008015491505090565b60605f6001613b59846148e3565b0190505f8167ffffffffffffffff811115613b7757613b76615bdf565b5b6040519080825280601f01601f191660200182016040528015613ba95781602001600182028036833780820191505090505b5090505f82602001820190505b600115613c0a578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a8581613bff57613bfe61726c565b5b0494505f8503613bb6575b819350505050919050565b5f7f26fdaf8a2cb20d20b55e36218986905e534ee7a970dd2fa827946e4b7496db00905090565b5f80836003015f8481526020019081526020015f205490505f846002015f8581526020019081526020015f205f8381526020019081526020015f2080549050141591505092915050565b5f613c8f613caa565b5f015f9054906101000a900467ffffffffffffffff16905090565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b613cd9614a34565b613ce38282614a74565b5050565b5f80856006015f8681526020019081526020015f205490505f86600e015f8781526020019081526020015f208054613d1e9061676b565b80601f0160208091040260200160405190810160405280929190818152602001828054613d4a9061676b565b8015613d955780601f10613d6c57610100808354040283529160200191613d95565b820191905f5260205f20905b815481529060010190602001808311613d7857829003601f168201915b50505050509050613da98287878785614ac5565b92505050949350505050565b5f80613e048585858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050614ca6565b9050613e108133614cd0565b809150509392505050565b5f807344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663b4722bc46040518163ffffffff1660e01b8152600401602060405180830381865afa158015613e7a573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613e9e9190616677565b905080831015915050919050565b5f602182511015613f3f577344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa158015613f14573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613f389190616677565b9050613f47565b602182015190505b919050565b60605f8267ffffffffffffffff811115613f6957613f68615bdf565b5b604051908082528060200260200182016040528015613f9c57816020015b6060815260200190600190039081613f875790505b5090505f5b83811015614083577344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166331ff41c887878481518110613fed57613fec616d2e565b5b60200260200101516040518363ffffffff1660e01b8152600401614012929190616a51565b5f60405180830381865afa15801561402c573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f8201168201806040525081019061405491906173ec565b6060015182828151811061406b5761406a616d2e565b5b60200260200101819052508080600101915050613fa1565b50809150509392505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff16148061413c57507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16614123614f35565b73ffffffffffffffffffffffffffffffffffffffff1614155b15614173576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156141d2573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906141f69190616570565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461426557336040517f21bfda1000000000000000000000000000000000000000000000000000000000815260040161425c919061659b565b60405180910390fd5b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa9250505080156142d057506040513d601f19601f820116820180604052508101906142cd919061745d565b60015b61431157816040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401614308919061659b565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b811461437757806040517faa1d49a400000000000000000000000000000000000000000000000000000000815260040161436e9190615d75565b60405180910390fd5b6143818383614f88565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff161461440b576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f6144676040518060600160405280603c81526020016178ed603c91398051906020012084848051906020012060405160200161444c93929190617488565b60405160208183030381529060405280519060200120614ffa565b905092915050565b5f8085600a015f8681526020019081526020015f205490505f86600e015f8781526020019081526020015f2080546144a69061676b565b80601f01602080910402602001604051908101604052809291908181526020018280546144d29061676b565b801561451d5780601f106144f45761010080835404028352916020019161451d565b820191905f5260205f20905b81548152906001019060200180831161450057829003601f168201915b505050505090506145318683878785615013565b92505050949350505050565b5f86148061454d57505f85859050145b6147775780156145705782876003015f8881526020019081526020015f20819055505b5f61457a83613eac565b90505f5b86869050811015614774575f87878381811061459d5761459c616d2e565b5b90506020020160208101906145b291906174bd565b9050896002015f8a81526020019081526020015f205f8781526020019081526020015f2081908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166331ff41c885846040518363ffffffff1660e01b8152600401614684929190616a51565b5f60405180830381865afa15801561469e573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f820116820180604052508101906146c691906173ec565b6020015190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16146147655760018b5f015f8c81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505b5050808060010191505061457e565b50505b50505050505050565b5f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100905090565b60605f6147b2614780565b90508060020180546147c39061676b565b80601f01602080910402602001604051908101604052809291908181526020018280546147ef9061676b565b801561483a5780601f106148115761010080835404028352916020019161483a565b820191905f5260205f20905b81548152906001019060200180831161481d57829003601f168201915b505050505091505090565b60605f614850614780565b90508060030180546148619061676b565b80601f016020809104026020016040519081016040528092919081815260200182805461488d9061676b565b80156148d85780601f106148af576101008083540402835291602001916148d8565b820191905f5260205f20905b8154815290600101906020018083116148bb57829003601f168201915b505050505091505090565b5f805f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f010000000000000000831061493f577a184f03e93ff9f4daa797ed6e38ed64bf6a1f01000000000000000083816149355761493461726c565b5b0492506040810190505b6d04ee2d6d415b85acef8100000000831061497c576d04ee2d6d415b85acef810000000083816149725761497161726c565b5b0492506020810190505b662386f26fc1000083106149ab57662386f26fc1000083816149a1576149a061726c565b5b0492506010810190505b6305f5e10083106149d4576305f5e10083816149ca576149c961726c565b5b0492506008810190505b61271083106149f95761271083816149ef576149ee61726c565b5b0492506004810190505b60648310614a1c5760648381614a1257614a1161726c565b5b0492506002810190505b600a8310614a2b576001810190505b80915050919050565b614a3c6150a4565b614a72576040517fd7e6bcf800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b614a7c614a34565b5f614a85614780565b905082816002019081614a989190617540565b5081816003019081614aaa9190617540565b505f801b815f01819055505f801b8160010181905550505050565b5f808484905067ffffffffffffffff811115614ae457614ae3615bdf565b5b604051908082528060200260200182016040528015614b125781602001602082028036833780820191505090505b5090505f5b85859050811015614c1657604051806060016040528060258152602001617a016025913980519060200120868683818110614b5557614b54616d2e565b5b9050602002810190614b679190616d67565b5f016020810190614b78919061760f565b878784818110614b8b57614b8a616d2e565b5b9050602002810190614b9d9190616d67565b8060200190614bac9190616e0d565b604051614bba929190617668565b6040518091039020604051602001614bd49392919061768f565b60405160208183030381529060405280519060200120828281518110614bfd57614bfc616d2e565b5b6020026020010181815250508080600101915050614b17565b50614c9a6040518060c001604052806082815260200161797f6082913980519060200120888884604051602001614c4d9190617775565b604051602081830303815290604052805190602001208780519060200120604051602001614c7f95949392919061778b565b60405160208183030381529060405280519060200120614ffa565b91505095945050505050565b5f805f80614cb486866150c2565b925092509250614cc48282615117565b82935050505092915050565b5f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa158015614d2e573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614d529190616677565b90507344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff16639447cfd482856040518363ffffffff1660e01b8152600401614da3929190616a51565b602060405180830381865afa158015614dbe573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614de29190616aa2565b614e255782826040517f0d86f521000000000000000000000000000000000000000000000000000000008152600401614e1c9291906177dc565b60405180910390fd5b5f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166331ff41c883856040518363ffffffff1660e01b8152600401614e75929190616a51565b5f60405180830381865afa158015614e8f573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f82011682018060405250810190614eb791906173ec565b90508373ffffffffffffffffffffffffffffffffffffffff16816020015173ffffffffffffffffffffffffffffffffffffffff1614614f2f5783836040517f0d86f521000000000000000000000000000000000000000000000000000000008152600401614f269291906177dc565b60405180910390fd5b50505050565b5f614f617f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b615279565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b614f9182615282565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f81511115614fed57614fe7828261534b565b50614ff6565b614ff56153cb565b5b5050565b5f61500c615006615407565b83615415565b9050919050565b5f61509960405180608001604052806056815260200161792960569139805190602001208787878760405160200161504c929190617668565b60405160208183030381529060405280519060200120868051906020012060405160200161507e95949392919061778b565b60405160208183030381529060405280519060200120614ffa565b905095945050505050565b5f6150ad613caa565b5f0160089054906101000a900460ff16905090565b5f805f6041845103615102575f805f602087015192506040870151915060608701515f1a90506150f488828585615455565b955095509550505050615110565b5f600285515f1b9250925092505b9250925092565b5f600381111561512a5761512961597d565b5b82600381111561513d5761513c61597d565b5b031561527557600160038111156151575761515661597d565b5b82600381111561516a5761516961597d565b5b036151a1576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600260038111156151b5576151b461597d565b5b8260038111156151c8576151c761597d565b5b0361520c57805f1c6040517ffce698f7000000000000000000000000000000000000000000000000000000008152600401615203919061636f565b60405180910390fd5b60038081111561521f5761521e61597d565b5b8260038111156152325761523161597d565b5b0361527457806040517fd78bce0c00000000000000000000000000000000000000000000000000000000815260040161526b9190615d75565b60405180910390fd5b5b5050565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b036152dd57806040517f4c9c8ce30000000000000000000000000000000000000000000000000000000081526004016152d4919061659b565b60405180910390fd5b806153097f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b615279565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f808473ffffffffffffffffffffffffffffffffffffffff16846040516153749190617833565b5f60405180830381855af49150503d805f81146153ac576040519150601f19603f3d011682016040523d82523d5f602084013e6153b1565b606091505b50915091506153c185838361553c565b9250505092915050565b5f341115615405576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f6154106155c9565b905090565b5f6040517f190100000000000000000000000000000000000000000000000000000000000081528360028201528260228201526042812091505092915050565b5f805f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c1115615491575f600385925092509250615532565b5f6001888888886040515f81526020016040526040516154b49493929190617858565b6020604051602081039080840390855afa1580156154d4573d5f803e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603615525575f60015f801b93509350935050615532565b805f805f1b935093509350505b9450945094915050565b6060826155515761554c8261562c565b6155c1565b5f825114801561557757505f8473ffffffffffffffffffffffffffffffffffffffff163b145b156155b957836040517f9996b3150000000000000000000000000000000000000000000000000000000081526004016155b0919061659b565b60405180910390fd5b8190506155c2565b5b9392505050565b5f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f6155f3615670565b6155fb6156e6565b463060405160200161561195949392919061789b565b60405160208183030381529060405280519060200120905090565b5f8151111561563e5780518082602001fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f8061567a614780565b90505f6156856147a7565b90505f815111156156a1578080519060200120925050506156e3565b5f825f015490505f801b81146156bc578093505050506156e3565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f806156f0614780565b90505f6156fb614845565b90505f815111156157175780805190602001209250505061575a565b5f826001015490505f801b81146157335780935050505061575a565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f81519050919050565b5f82825260208201905092915050565b5f5b83811015615794578082015181840152602081019050615779565b5f8484015250505050565b5f601f19601f8301169050919050565b5f6157b98261575d565b6157c38185615767565b93506157d3818560208601615777565b6157dc8161579f565b840191505092915050565b5f6020820190508181035f8301526157ff81846157af565b905092915050565b5f604051905090565b5f80fd5b5f80fd5b5f819050919050565b61582a81615818565b8114615834575f80fd5b50565b5f8135905061584581615821565b92915050565b5f602082840312156158605761585f615810565b5b5f61586d84828501615837565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6158c88261589f565b9050919050565b6158d8816158be565b82525050565b5f6158e983836158cf565b60208301905092915050565b5f602082019050919050565b5f61590b82615876565b6159158185615880565b935061592083615890565b805f5b8381101561595057815161593788826158de565b9750615942836158f5565b925050600181019050615923565b5085935050505092915050565b5f6020820190508181035f8301526159758184615901565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b600281106159bb576159ba61597d565b5b50565b5f8190506159cb826159aa565b919050565b5f6159da826159be565b9050919050565b6159ea816159d0565b82525050565b5f602082019050615a035f8301846159e1565b92915050565b60028110615a15575f80fd5b50565b5f81359050615a2681615a09565b92915050565b5f8060408385031215615a4257615a41615810565b5b5f615a4f85828601615837565b9250506020615a6085828601615a18565b9150509250929050565b5f80fd5b5f80fd5b5f80fd5b5f8083601f840112615a8b57615a8a615a6a565b5b8235905067ffffffffffffffff811115615aa857615aa7615a6e565b5b602083019150836020820283011115615ac457615ac3615a72565b5b9250929050565b5f8083601f840112615ae057615adf615a6a565b5b8235905067ffffffffffffffff811115615afd57615afc615a6e565b5b602083019150836001820283011115615b1957615b18615a72565b5b9250929050565b5f805f805f60608688031215615b3957615b38615810565b5b5f615b4688828901615837565b955050602086013567ffffffffffffffff811115615b6757615b66615814565b5b615b7388828901615a76565b9450945050604086013567ffffffffffffffff811115615b9657615b95615814565b5b615ba288828901615acb565b92509250509295509295909350565b615bba816158be565b8114615bc4575f80fd5b50565b5f81359050615bd581615bb1565b92915050565b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b615c158261579f565b810181811067ffffffffffffffff82111715615c3457615c33615bdf565b5b80604052505050565b5f615c46615807565b9050615c528282615c0c565b919050565b5f67ffffffffffffffff821115615c7157615c70615bdf565b5b615c7a8261579f565b9050602081019050919050565b828183375f83830152505050565b5f615ca7615ca284615c57565b615c3d565b905082815260208101848484011115615cc357615cc2615bdb565b5b615cce848285615c87565b509392505050565b5f82601f830112615cea57615ce9615a6a565b5b8135615cfa848260208601615c95565b91505092915050565b5f8060408385031215615d1957615d18615810565b5b5f615d2685828601615bc7565b925050602083013567ffffffffffffffff811115615d4757615d46615814565b5b615d5385828601615cd6565b9150509250929050565b5f819050919050565b615d6f81615d5d565b82525050565b5f602082019050615d885f830184615d66565b92915050565b5f805f60408486031215615da557615da4615810565b5b5f615db286828701615837565b935050602084013567ffffffffffffffff811115615dd357615dd2615814565b5b615ddf86828701615acb565b92509250509250925092565b5f805f805f60608688031215615e0457615e03615810565b5b5f615e1188828901615837565b955050602086013567ffffffffffffffff811115615e3257615e31615814565b5b615e3e88828901615acb565b9450945050604086013567ffffffffffffffff811115615e6157615e60615814565b5b615e6d88828901615acb565b92509250509295509295909350565b5f80fd5b5f6103008284031215615e9657615e95615e7c565b5b81905092915050565b5f60208284031215615eb457615eb3615810565b5b5f82013567ffffffffffffffff811115615ed157615ed0615814565b5b615edd84828501615e80565b91505092915050565b5f8115159050919050565b615efa81615ee6565b82525050565b5f602082019050615f135f830184615ef1565b92915050565b5f7fff0000000000000000000000000000000000000000000000000000000000000082169050919050565b615f4d81615f19565b82525050565b615f5c81615818565b82525050565b615f6b816158be565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b615fa381615818565b82525050565b5f615fb48383615f9a565b60208301905092915050565b5f602082019050919050565b5f615fd682615f71565b615fe08185615f7b565b9350615feb83615f8b565b805f5b8381101561601b5781516160028882615fa9565b975061600d83615fc0565b925050600181019050615fee565b5085935050505092915050565b5f60e08201905061603b5f83018a615f44565b818103602083015261604d81896157af565b9050818103604083015261606181886157af565b90506160706060830187615f53565b61607d6080830186615f62565b61608a60a0830185615d66565b81810360c083015261609c8184615fcc565b905098975050505050505050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f82825260208201905092915050565b5f6160ed8261575d565b6160f781856160d3565b9350616107818560208601615777565b6161108161579f565b840191505092915050565b5f61612683836160e3565b905092915050565b5f602082019050919050565b5f616144826160aa565b61614e81856160b4565b935083602082028501616160856160c4565b805f5b8581101561619b578484038952815161617c858261611b565b94506161878361612e565b925060208a01995050600181019050616163565b50829750879550505050505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b600281106161e7576161e661597d565b5b50565b5f8190506161f7826161d6565b919050565b5f616206826161ea565b9050919050565b616216816161fc565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f6162408261621c565b61624a8185616226565b935061625a818560208601615777565b6162638161579f565b840191505092915050565b5f604083015f8301516162835f86018261620d565b506020830151848203602086015261629b8282616236565b9150508091505092915050565b5f6162b3838361626e565b905092915050565b5f602082019050919050565b5f6162d1826161ad565b6162db81856161b7565b9350836020820285016162ed856161c7565b805f5b85811015616328578484038952815161630985826162a8565b9450616314836162bb565b925060208a019950506001810190506162f0565b50829750879550505050505092915050565b5f6040820190508181035f830152616352818561613a565b9050818103602083015261636681846162c7565b90509392505050565b5f6020820190506163825f830184615f53565b92915050565b5f82825260208201905092915050565b5f6163a28261621c565b6163ac8185616388565b93506163bc818560208601615777565b6163c58161579f565b840191505092915050565b5f6040820190508181035f8301526163e8818561613a565b905081810360208301526163fc8184616398565b90509392505050565b5f6020828403121561641a57616419615810565b5b5f61642784828501615a18565b91505092915050565b5f81905092915050565b5f6164448261575d565b61644e8185616430565b935061645e818560208601615777565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f61649e600283616430565b91506164a98261646a565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f6164e8600183616430565b91506164f3826164b4565b600182019050919050565b5f616509828761643a565b915061651482616492565b9150616520828661643a565b915061652b826164dc565b9150616537828561643a565b9150616542826164dc565b915061654e828461643a565b915081905095945050505050565b5f8151905061656a81615bb1565b92915050565b5f6020828403121561658557616584615810565b5b5f6165928482850161655c565b91505092915050565b5f6020820190506165ae5f830184615f62565b92915050565b5f67ffffffffffffffff82169050919050565b6165d0816165b4565b82525050565b5f6020820190506165e95f8301846165c7565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61662682615818565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203616658576166576165ef565b5b600182019050919050565b5f8151905061667181615821565b92915050565b5f6020828403121561668c5761668b615810565b5b5f61669984828501616663565b91505092915050565b5f60ff82169050919050565b5f8160f81b9050919050565b5f6166c4826166ae565b9050919050565b6166dc6166d7826166a2565b6166ba565b82525050565b5f819050919050565b6166fc6166f782615818565b6166e2565b82525050565b5f61670d82866166cb565b60018201915061671d82856166eb565b60208201915061672d82846166eb565b602082019150819050949350505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f600282049050600182168061678257607f821691505b6020821081036167955761679461673e565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f600883026167f77fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff826167bc565b61680186836167bc565b95508019841693508086168417925050509392505050565b5f819050919050565b5f61683c61683761683284615818565b616819565b615818565b9050919050565b5f819050919050565b61685583616822565b61686961686182616843565b8484546167c8565b825550505050565b5f90565b61687d616871565b61688881848461684c565b505050565b5b818110156168ab576168a05f82616875565b60018101905061688e565b5050565b601f8211156168f0576168c18161679b565b6168ca846167ad565b810160208510156168d9578190505b6168ed6168e5856167ad565b83018261688d565b50505b505050565b5f82821c905092915050565b5f6169105f19846008026168f5565b1980831691505092915050565b5f6169288383616901565b9150826002028217905092915050565b6169418261621c565b67ffffffffffffffff81111561695a57616959615bdf565b5b616964825461676b565b61696f8282856168af565b5f60209050601f8311600181146169a0575f841561698e578287015190505b616998858261691d565b8655506169ff565b601f1984166169ae8661679b565b5f5b828110156169d5578489015182556001820191506020850194506020810190506169b0565b868310156169f257848901516169ee601f891682616901565b8355505b6001600288020188555050505b505050505050565b5f608082019050616a1a5f830187615f53565b616a276020830186615f53565b616a3460408301856159e1565b8181036060830152616a468184616398565b905095945050505050565b5f604082019050616a645f830185615f53565b616a716020830184615f62565b9392505050565b616a8181615ee6565b8114616a8b575f80fd5b50565b5f81519050616a9c81616a78565b92915050565b5f60208284031215616ab757616ab6615810565b5b5f616ac484828501616a8e565b91505092915050565b5f819050919050565b60028110616ae2575f80fd5b50565b5f81359050616af381616ad6565b92915050565b5f616b076020840184616ae5565b905092915050565b5f80fd5b5f80fd5b5f80fd5b5f8083356001602003843603038112616b3757616b36616b17565b5b83810192508235915060208301925067ffffffffffffffff821115616b5f57616b5e616b0f565b5b600182023603831315616b7557616b74616b13565b5b509250929050565b5f616b888385616226565b9350616b95838584615c87565b616b9e8361579f565b840190509392505050565b5f60408301616bba5f840184616af9565b616bc65f86018261620d565b50616bd46020840184616b1b565b8583036020870152616be7838284616b7d565b925050508091505092915050565b5f616c008383616ba9565b905092915050565b5f82356001604003833603038112616c2357616c22616b17565b5b82810191505092915050565b5f602082019050919050565b5f616c4683856161b7565b935083602084028501616c5884616acd565b805f5b87811015616c9b578484038952616c728284616c08565b616c7c8582616bf5565b9450616c8783616c2f565b925060208a01995050600181019050616c5b565b50829750879450505050509392505050565b5f616cb88385616388565b9350616cc5838584615c87565b616cce8361579f565b840190509392505050565b5f608082019050616cec5f830189615f53565b8181036020830152616cff818789616c3b565b90508181036040830152616d14818587616cad565b9050616d236060830184615f62565b979650505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b5f82356001604003833603038112616d8257616d81616d5b565b5b80830191505092915050565b5f8135616d9a81616ad6565b80915050919050565b5f815f1b9050919050565b5f60ff616dba84616da3565b9350801983169250808416831791505092915050565b5f616dda826161ea565b9050919050565b5f819050919050565b616df382616dd0565b616e06616dff82616de1565b8354616dae565b8255505050565b5f8083356001602003843603038112616e2957616e28616d5b565b5b80840192508235915067ffffffffffffffff821115616e4b57616e4a616d5f565b5b602083019250600182023603831315616e6757616e66616d63565b5b509250929050565b5f82905092915050565b616e838383616e6f565b67ffffffffffffffff811115616e9c57616e9b615bdf565b5b616ea6825461676b565b616eb18282856168af565b5f601f831160018114616ede575f8415616ecc578287013590505b616ed6858261691d565b865550616f3d565b601f198416616eec8661679b565b5f5b82811015616f1357848901358255600182019150602085019450602081019050616eee565b86831015616f305784890135616f2c601f891682616901565b8355505b6001600288020188555050505b50505050505050565b616f51838383616e79565b505050565b5f81015f830180616f6681616d8e565b9050616f728184616dea565b5050506001810160208301616f878185616e0d565b616f92818386616f46565b505050505050565b616fa48282616f56565b5050565b5f606082019050616fbb5f830187615f53565b8181036020830152616fcd818661613a565b90508181036040830152616fe2818486616c3b565b905095945050505050565b5f6060820190506170005f830187615f53565b8181036020830152617013818587616cad565b90506170226040830184615f62565b95945050505050565b5f60608201905061703e5f830186615f53565b61704b6020830185615f53565b818103604083015261705d8184616398565b9050949350505050565b5f60808201905061707a5f830189615f53565b818103602083015261708d818789616cad565b905081810360408301526170a2818587616cad565b90506170b16060830184615f62565b979650505050505050565b5f6060820190506170cf5f830187615f53565b81810360208301526170e1818661613a565b905081810360408301526170f6818486616cad565b905095945050505050565b5f808335600160200384360303811261711d5761711c616d5b565b5b80840192508235915067ffffffffffffffff82111561713f5761713e616d5f565b5b60208301925060208202360383131561715b5761715a616d63565b5b509250929050565b5f808335600160200384360303811261717f5761717e616d5b565b5b80840192508235915067ffffffffffffffff8211156171a1576171a0616d5f565b5b6020830192506020820236038313156171bd576171bc616d63565b5b509250929050565b5f813590506171d381616a78565b92915050565b5f602082840312156171ee576171ed615810565b5b5f6171fb848285016171c5565b91505092915050565b7f4549503731323a20556e696e697469616c697a656400000000000000000000005f82015250565b5f617238601583615767565b915061724382617204565b602082019050919050565b5f6020820190508181035f8301526172658161722c565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f80fd5b5f80fd5b5f67ffffffffffffffff8211156172bb576172ba615bdf565b5b6172c48261579f565b9050602081019050919050565b5f6172e36172de846172a1565b615c3d565b9050828152602081018484840111156172ff576172fe615bdb565b5b61730a848285615777565b509392505050565b5f82601f83011261732657617325615a6a565b5b81516173368482602086016172d1565b91505092915050565b5f6080828403121561735457617353617299565b5b61735e6080615c3d565b90505f61736d8482850161655c565b5f8301525060206173808482850161655c565b602083015250604082015167ffffffffffffffff8111156173a4576173a361729d565b5b6173b084828501617312565b604083015250606082015167ffffffffffffffff8111156173d4576173d361729d565b5b6173e084828501617312565b60608301525092915050565b5f6020828403121561740157617400615810565b5b5f82015167ffffffffffffffff81111561741e5761741d615814565b5b61742a8482850161733f565b91505092915050565b61743c81615d5d565b8114617446575f80fd5b50565b5f8151905061745781617433565b92915050565b5f6020828403121561747257617471615810565b5b5f61747f84828501617449565b91505092915050565b5f60608201905061749b5f830186615d66565b6174a86020830185615f53565b6174b56040830184615d66565b949350505050565b5f602082840312156174d2576174d1615810565b5b5f6174df84828501615bc7565b91505092915050565b5f819050815f5260205f209050919050565b601f82111561753b5761750c816174e8565b617515846167ad565b81016020851015617524578190505b617538617530856167ad565b83018261688d565b50505b505050565b6175498261575d565b67ffffffffffffffff81111561756257617561615bdf565b5b61756c825461676b565b6175778282856174fa565b5f60209050601f8311600181146175a8575f8415617596578287015190505b6175a0858261691d565b865550617607565b601f1984166175b6866174e8565b5f5b828110156175dd578489015182556001820191506020850194506020810190506175b8565b868310156175fa57848901516175f6601f891682616901565b8355505b6001600288020188555050505b505050505050565b5f6020828403121561762457617623615810565b5b5f61763184828501616ae5565b91505092915050565b5f81905092915050565b5f61764f838561763a565b935061765c838584615c87565b82840190509392505050565b5f617674828486617644565b91508190509392505050565b617689816161fc565b82525050565b5f6060820190506176a25f830186615d66565b6176af6020830185617680565b6176bc6040830184615d66565b949350505050565b5f81519050919050565b5f81905092915050565b5f819050602082019050919050565b6176f081615d5d565b82525050565b5f61770183836176e7565b60208301905092915050565b5f602082019050919050565b5f617723826176c4565b61772d81856176ce565b9350617738836176d8565b805f5b8381101561776857815161774f88826176f6565b975061775a8361770d565b92505060018101905061773b565b5085935050505092915050565b5f6177808284617719565b915081905092915050565b5f60a08201905061779e5f830188615d66565b6177ab6020830187615f53565b6177b86040830186615f53565b6177c56060830185615d66565b6177d26080830184615d66565b9695505050505050565b5f6040820190506177ef5f830185615f62565b6177fc6020830184615f62565b9392505050565b5f61780d8261621c565b617817818561763a565b9350617827818560208601615777565b80840191505092915050565b5f61783e8284617803565b915081905092915050565b617852816166a2565b82525050565b5f60808201905061786b5f830187615d66565b6178786020830186617849565b6178856040830185615d66565b6178926060830184615d66565b95945050505050565b5f60a0820190506178ae5f830188615d66565b6178bb6020830187615d66565b6178c86040830186615d66565b6178d56060830185615f53565b6178e26080830184615f62565b969550505050505056fe507265704b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e49642c6279746573206578747261446174612943727367656e566572696669636174696f6e2875696e743235362063727349642c75696e74323536206d61784269744c656e6774682c6279746573206372734469676573742c627974657320657874726144617461294b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e49642c75696e74323536206b657949642c4b65794469676573745b5d206b6579446967657374732c627974657320657874726144617461294b65794469676573742875696e7438206b6579547970652c627974657320646967657374294b65794469676573742875696e7438206b6579547970652c62797465732064696765737429
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP4\x80\x15b\0\0CW_\x80\xFD[Pb\0\0Tb\0\0Z` \x1B` \x1CV[b\0\x01\xC4V[_b\0\0kb\0\x01^` \x1B` \x1CV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15b\0\0\xB6W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14b\0\x01[Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Qb\0\x01R\x91\x90b\0\x01\xA9V[`@Q\x80\x91\x03\x90\xA1[PV[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[b\0\x01\xA3\x81b\0\x01\x85V[\x82RPPV[_` \x82\x01\x90Pb\0\x01\xBE_\x83\x01\x84b\0\x01\x98V[\x92\x91PPV[`\x80Qaz&b\0\x01\xEB_9_\x81\x81a@\x91\x01R\x81\x81a@\xE6\x01RaC\x88\x01Raz&_\xF3\xFE`\x80`@R`\x046\x10a\x014W_5`\xE0\x1C\x80cb\x97\x87\x87\x11a\0\xAAW\x80c\xAD<\xB1\xCC\x11a\0nW\x80c\xAD<\xB1\xCC\x14a\x03\xF9W\x80c\xBA\xFF!\x1E\x14a\x04#W\x80c\xC2\xC1\xFA\xEE\x14a\x04MW\x80c\xC5[\x87$\x14a\x04uW\x80c\xCA\xA3g\xDB\x14a\x04\xB2W\x80c\xD5/\x10\xEB\x14a\x04\xDAWa\x014V[\x80cb\x97\x87\x87\x14a\x03\x12W\x80clh\xE7\x1F\x14a\x03:W\x80cs[\x12c\x14a\x03bW\x80c\x84\xB0\x19n\x14a\x03\x8CW\x80c\x93f\x08\xAE\x14a\x03\xBCWa\x014V[\x80c<\x02\xF84\x11a\0\xFCW\x80c<\x02\xF84\x14a\x02\x18W\x80cE\xAF&\x1B\x14a\x02@W\x80cF\x10\xFF\xE8\x14a\x02|W\x80cO\x1E\xF2\x86\x14a\x02\xA4W\x80cR\xD1\x90-\x14a\x02\xC0W\x80cX\x9A\xDB\x0E\x14a\x02\xEAWa\x014V[\x80c\r\x8En,\x14a\x018W\x80c\x16\xC7\x13\xD9\x14a\x01bW\x80c\x17\x03\xC6\x1A\x14a\x01\x9EW\x80c\x19\xF4\xF62\x14a\x01\xC6W\x80c9\xF78\x10\x14a\x02\x02W[_\x80\xFD[4\x80\x15a\x01CW_\x80\xFD[Pa\x01La\x05\x04V[`@Qa\x01Y\x91\x90aW\xE7V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01mW_\x80\xFD[Pa\x01\x88`\x04\x806\x03\x81\x01\x90a\x01\x83\x91\x90aXKV[a\x05\x7FV[`@Qa\x01\x95\x91\x90aY]V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xA9W_\x80\xFD[Pa\x01\xC4`\x04\x806\x03\x81\x01\x90a\x01\xBF\x91\x90aXKV[a\x06PV[\0[4\x80\x15a\x01\xD1W_\x80\xFD[Pa\x01\xEC`\x04\x806\x03\x81\x01\x90a\x01\xE7\x91\x90aXKV[a\x08\x97V[`@Qa\x01\xF9\x91\x90aY\xF0V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\rW_\x80\xFD[Pa\x02\x16a\n\"V[\0[4\x80\x15a\x02#W_\x80\xFD[Pa\x02>`\x04\x806\x03\x81\x01\x90a\x029\x91\x90aZ,V[a\x0C<V[\0[4\x80\x15a\x02KW_\x80\xFD[Pa\x02f`\x04\x806\x03\x81\x01\x90a\x02a\x91\x90aXKV[a\x0F;V[`@Qa\x02s\x91\x90aY\xF0V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x87W_\x80\xFD[Pa\x02\xA2`\x04\x806\x03\x81\x01\x90a\x02\x9D\x91\x90a[ V[a\x10}V[\0[a\x02\xBE`\x04\x806\x03\x81\x01\x90a\x02\xB9\x91\x90a]\x03V[a\x17oV[\0[4\x80\x15a\x02\xCBW_\x80\xFD[Pa\x02\xD4a\x17\x8EV[`@Qa\x02\xE1\x91\x90a]uV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xF5W_\x80\xFD[Pa\x03\x10`\x04\x806\x03\x81\x01\x90a\x03\x0B\x91\x90a]\x8EV[a\x17\xBFV[\0[4\x80\x15a\x03\x1DW_\x80\xFD[Pa\x038`\x04\x806\x03\x81\x01\x90a\x033\x91\x90a]\xEBV[a\x1C\xCFV[\0[4\x80\x15a\x03EW_\x80\xFD[Pa\x03``\x04\x806\x03\x81\x01\x90a\x03[\x91\x90a^\x9FV[a\"\x9AV[\0[4\x80\x15a\x03mW_\x80\xFD[Pa\x03va,\x06V[`@Qa\x03\x83\x91\x90a_\0V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x97W_\x80\xFD[Pa\x03\xA0a,\xB1V[`@Qa\x03\xB3\x97\x96\x95\x94\x93\x92\x91\x90a`(V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xC7W_\x80\xFD[Pa\x03\xE2`\x04\x806\x03\x81\x01\x90a\x03\xDD\x91\x90aXKV[a-\xBAV[`@Qa\x03\xF0\x92\x91\x90ac:V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\x04W_\x80\xFD[Pa\x04\ra1\x87V[`@Qa\x04\x1A\x91\x90aW\xE7V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04.W_\x80\xFD[Pa\x047a1\xC0V[`@Qa\x04D\x91\x90acoV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04XW_\x80\xFD[Pa\x04s`\x04\x806\x03\x81\x01\x90a\x04n\x91\x90aXKV[a1\xD7V[\0[4\x80\x15a\x04\x80W_\x80\xFD[Pa\x04\x9B`\x04\x806\x03\x81\x01\x90a\x04\x96\x91\x90aXKV[a4iV[`@Qa\x04\xA9\x92\x91\x90ac\xD0V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\xBDW_\x80\xFD[Pa\x04\xD8`\x04\x806\x03\x81\x01\x90a\x04\xD3\x91\x90ad\x05V[a7\xA1V[\0[4\x80\x15a\x04\xE5W_\x80\xFD[Pa\x04\xEEa;4V[`@Qa\x04\xFB\x91\x90acoV[`@Q\x80\x91\x03\x90\xF3[```@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x05E_a;KV[a\x05O`\x01a;KV[a\x05X_a;KV[`@Q` \x01a\x05k\x94\x93\x92\x91\x90ad\xFEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[``_a\x05\x8Aa<\x15V[\x90P_\x81`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\x02\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x82\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x06BW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x05\xF9W[PPPPP\x92PPP\x91\x90PV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xADW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xD1\x91\x90aepV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x07@W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x077\x91\x90ae\x9BV[`@Q\x80\x91\x03\x90\xFD[_a\x07Ia<\x15V[\x90P\x80`\t\x01T\x82\x11\x80a\x07bWP`\xF8`\x05\x90\x1B\x82\x11\x15[\x15a\x07\xA4W\x81`@Q\x7F\xCB\xE9&V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\x9B\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x08\x06W\x81`@Q\x7F\xDF\r\xB5\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\xFD\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[`\x01\x81`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01\x81`\x0F\x01_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F8O\x90\xFE\xFB\xCF\xAAh\xF2.\0\tJ\xEA\xA5++\xC6\x93\x93m,\xE1\xAF\xED\x12\x12R\x0BY\xB5\x8E\x82`@Qa\x08\x8B\x91\x90acoV[`@Q\x80\x91\x03\x90\xA1PPV[_\x80a\x08\xA1a<\x15V[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\t\x04W\x82`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xFB\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[\x80`\x0F\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\tfW\x82`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t]\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[a\tp\x81\x84a<<V[a\t\xB1W\x82`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xA8\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P_\x81\x03a\t\xF8W\x81`\r\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x92PPPa\n\x1DV[\x81`\r\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x92PPP[\x91\x90PV[`\x01a\n,a<\x86V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\nmW`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02_a\nxa<\xAAV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\n\xC0WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\n\xF7W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x0B\xB0`@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa<\xD1V[_a\x0B\xB9a<\x15V[\x90P`\xF8`\x03\x90\x1B\x81`\x04\x01\x81\x90UP`\xF8`\x04\x90\x1B\x81`\x05\x01\x81\x90UP`\xF8`\x05\x90\x1B\x81`\t\x01\x81\x90UPP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x0C0\x91\x90ae\xD6V[`@Q\x80\x91\x03\x90\xA1PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x99W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xBD\x91\x90aepV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\r,W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r#\x91\x90ae\x9BV[`@Q\x80\x91\x03\x90\xFD[_a\r5a<\x15V[\x90P_\x81`\t\x01T\x90P`\xF8`\x05\x90\x1B\x81\x14\x15\x80\x15a\rqWP\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15a\r\xB3W\x80`@Q\x7F\x06\x1A\xC6\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\xAA\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[\x81`\t\x01_\x81T\x80\x92\x91\x90a\r\xC7\x90af\x1CV[\x91\x90PUP_\x82`\t\x01T\x90P\x84\x83`\n\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x83\x83`\r\x01_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a\x0E!Wa\x0E aY}V[[\x02\x17\x90UP_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\x84W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xA8\x91\x90afwV[\x90P_\x80`\x02\x83\x83`@Q` \x01a\x0E\xC2\x93\x92\x91\x90ag\x02V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80\x86`\x0E\x01_\x86\x81R` \x01\x90\x81R` \x01_ \x90\x81a\x0E\xF3\x91\x90ai8V[P\x7F\x8C\xF0\x15\x13\x93\xF8O\xD6\x94\xC5\xE3\x15\xCBt\xCC\x05\xB2G\xDE\nEO\xD9\xE9\x12\x9Cf\x1E\xFD\xF9@\x1D\x84\x89\x89\x84`@Qa\x0F)\x94\x93\x92\x91\x90aj\x07V[`@Q\x80\x91\x03\x90\xA1PPPPPPPPV[_\x80a\x0FEa<\x15V[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x0F\xA8W\x82`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\x9F\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[\x80`\x0F\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x10\nW\x82`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\x01\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[a\x10\x14\x81\x84a<<V[a\x10UW\x82`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10L\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[\x80`\r\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xDBW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xFF\x91\x90afwV[\x90PsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cF\xC5\xBB\xBD\x823`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11P\x92\x91\x90ajQV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11kW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x8F\x91\x90aj\xA2V[a\x11\xD0W3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\xC7\x91\x90ae\x9BV[`@Q\x80\x91\x03\x90\xFD[_a\x11\xD9a<\x15V[\x90P\x80`\x05\x01T\x87\x11\x80a\x11\xF2WP`\xF8`\x04\x90\x1B\x87\x11\x15[\x15a\x124W\x86`@Q\x7F\xAD\xFA\xB9\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12+\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[_\x86\x86\x90P\x03a\x12{W\x86`@Q\x7F\xE6\xF9\x08;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12r\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[_a\x12\x88\x82\x89\x89\x89a<\xE7V[\x90P_\x82`\x06\x01_\x8A\x81R` \x01\x90\x81R` \x01_ T\x90P_\x81\x14\x15\x80\x15a\x12\xCEWP\x82`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15a\x13\x05W`@Q\x7Fo\xBC\xDD+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x13\x11\x83\x88\x88a=\xB5V[\x90P\x83_\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x13\xB1W\x89\x81`@Q\x7F\x98\xFB\x95}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13\xA8\x92\x91\x90ajQV[`@Q\x80\x91\x03\x90\xFD[`\x01\x84_\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x84`\x02\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x85\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F*\xFEd\xFB:\xFD\xE8\xE2g\x8A\xEA\x84\xCF6\"?3\x0E/\xB1(m7\xAE\xD5s\xAB\x9C\xD1\xDBG\xC7\x8B\x8B\x8B\x8B\x8B3`@Qa\x14\xD3\x96\x95\x94\x93\x92\x91\x90al\xD9V[`@Q\x80\x91\x03\x90\xA1\x84`\x01\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x15\x11WPa\x15\x10\x81\x80T\x90Pa>\x1BV[[\x15a\x17bW`\x01\x85`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_[\x8A\x8A\x90P\x81\x10\x15a\x15\xC7W\x85`\x07\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x8B\x8B\x83\x81\x81\x10a\x15tWa\x15sam.V[[\x90P` \x02\x81\x01\x90a\x15\x86\x91\x90amgV[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P\x81\x81a\x15\xB8\x91\x90ao\x9AV[PP\x80\x80`\x01\x01\x91PPa\x15CV[P\x83\x85`\x03\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x8A\x85`\x08\x01\x81\x90UP_a\x16\x8C\x86`\x0E\x01_\x8E\x81R` \x01\x90\x81R` \x01_ \x80Ta\x16\x0B\x90agkV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x167\x90agkV[\x80\x15a\x16\x82W\x80`\x1F\x10a\x16YWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x16\x82V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x16eW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa>\xACV[\x90P_a\x17 \x82\x84\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x17\x11W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x16\xC8W[PPPPP\x85\x80T\x90Pa?LV[\x90P\x7F\xEB\x85\xC2m\xBC\xADF\xB8\nh\xA0\xF2L\xCE|,\x90\xF0\xA1\xFA\xDE\xD8A\x84\x13\x889\xFC\x9E\x80\xA2[\x8D\x82\x8E\x8E`@Qa\x17W\x94\x93\x92\x91\x90ao\xA8V[`@Q\x80\x91\x03\x90\xA1PP[PPPPPPPPPPPV[a\x17wa@\x8FV[a\x17\x80\x82aAuV[a\x17\x8A\x82\x82aBhV[PPV[_a\x17\x97aC\x86V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\x1DW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18A\x91\x90afwV[\x90PsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cF\xC5\xBB\xBD\x823`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\x92\x92\x91\x90ajQV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xADW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xD1\x91\x90aj\xA2V[a\x19\x12W3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19\t\x91\x90ae\x9BV[`@Q\x80\x91\x03\x90\xFD[_a\x19\x1Ba<\x15V[\x90P\x80`\x04\x01T\x85\x11\x80a\x194WP`\xF8`\x03\x90\x1B\x85\x11\x15[\x15a\x19vW\x84`@Q\x7F\n\xB7\xF6\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19m\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[_\x81`\x0E\x01_\x87\x81R` \x01\x90\x81R` \x01_ \x80Ta\x19\x95\x90agkV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x19\xC1\x90agkV[\x80\x15a\x1A\x0CW\x80`\x1F\x10a\x19\xE3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1A\x0CV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x19\xEFW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P_a\x1A\x1E\x87\x83aD\rV[\x90P_a\x1A,\x82\x88\x88a=\xB5V[\x90P\x83_\x01_\x89\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x1A\xCCW\x87\x81`@Q\x7F3\xCA\x1F\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1A\xC3\x92\x91\x90ajQV[`@Q\x80\x91\x03\x90\xFD[`\x01\x84_\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x84`\x02\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7FLq\\W4\xCE\\\x18\xC9\xC1.\x84\x96\xE5=*e\xF1\xEC8\x1DGiW\xF0\xF5\x96\xB3d\xA5\x9B\x0C\x89\x89\x893`@Qa\x1B\xEA\x94\x93\x92\x91\x90ao\xEDV[`@Q\x80\x91\x03\x90\xA1\x84`\x01\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x1C(WPa\x1C'\x81\x80T\x90Pa>\x1BV[[\x15a\x1C\xC4W`\x01\x85`\x01\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x82\x85`\x03\x01_\x8B\x81R` \x01\x90\x81R` \x01_ \x81\x90UP_\x85`\x06\x01_\x8B\x81R` \x01\x90\x81R` \x01_ T\x90P\x7F:\x11a \xCC\xA5\xD4\xF0s\xCC\x1F\xC3\x1F\xF2a3\xAB{\x04\x99\xF2q/\xA0\x10\x02;\x87\xD5\xA1\xF9\xEE\x8A\x82\x87`@Qa\x1C\xBA\x93\x92\x91\x90ap+V[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPPPV[_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D-W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1DQ\x91\x90afwV[\x90PsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cF\xC5\xBB\xBD\x823`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1D\xA2\x92\x91\x90ajQV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xBDW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xE1\x91\x90aj\xA2V[a\x1E\"W3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1E\x19\x91\x90ae\x9BV[`@Q\x80\x91\x03\x90\xFD[_a\x1E+a<\x15V[\x90P\x80`\t\x01T\x87\x11\x80a\x1EDWP`\xF8`\x05\x90\x1B\x87\x11\x15[\x15a\x1E\x86W\x86`@Q\x7F\x8D\x8C\x94\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1E}\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[_a\x1E\x93\x82\x89\x89\x89aDoV[\x90P_a\x1E\xA1\x82\x87\x87a=\xB5V[\x90P\x82_\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x1FAW\x88\x81`@Q\x7F\xFC\xF5\xA6\xE9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1F8\x92\x91\x90ajQV[`@Q\x80\x91\x03\x90\xFD[`\x01\x83_\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x83`\x02\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F{\xF1\xB4,\x10\xE9I|\x87\x96 \xC5\xB7\xAF\xCE\xD1\x0B\xDA\x17\xD8\xC9\x0B\"\xF0\xE3\xBCk/\xD6\xCE\xD0\xBD\x8A\x8A\x8A\x8A\x8A3`@Qa c\x96\x95\x94\x93\x92\x91\x90apgV[`@Q\x80\x91\x03\x90\xA1\x83`\x01\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a \xA1WPa \xA0\x81\x80T\x90Pa>\x1BV[[\x15a\"\x8EW`\x01\x84`\x01\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x88\x88\x85`\x0B\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x91\x82a \xF3\x92\x91\x90anyV[P\x82\x84`\x03\x01_\x8C\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x89\x84`\x0C\x01\x81\x90UP_a!\xB8\x85`\x0E\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x80Ta!7\x90agkV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta!c\x90agkV[\x80\x15a!\xAEW\x80`\x1F\x10a!\x85Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a!\xAEV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a!\x91W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa>\xACV[\x90P_a\"L\x82\x84\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\"=W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a!\xF4W[PPPPP\x85\x80T\x90Pa?LV[\x90P\x7F\"X\xB7?\xAE\xD3?\xB2\xE2\xEAED\x03\xBE\xF9t\x92\x0C\xAFh*\xB3\xA7#HO\xCFgU;\x16\xA2\x8C\x82\x8D\x8D`@Qa\"\x83\x94\x93\x92\x91\x90ap\xBCV[`@Q\x80\x91\x03\x90\xA1PP[PPPPPPPPPPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\xF7W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\x1B\x91\x90aepV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a#\x8AW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a#\x81\x91\x90ae\x9BV[`@Q\x80\x91\x03\x90\xFD[`\x02_a#\x95a<\xAAV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a#\xDDWP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a$\x14W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa$\xCD`@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa<\xD1V[_a$\xD6a<\x15V[\x90P`\xF8`\x03\x90\x1B\x84_\x015\x10\x15a$\xECW_\x80\xFD[`\xF8`\x04\x90\x1B\x84` \x015\x10\x15a%\x01W_\x80\xFD[`\xF8`\x05\x90\x1B\x84`@\x015\x10\x15a%\x16W_\x80\xFD[\x83_\x015\x81`\x04\x01\x81\x90UP\x83` \x015\x81`\x05\x01\x81\x90UP\x83`@\x015\x81`\t\x01\x81\x90UP\x83``\x015\x81`\x08\x01\x81\x90UP\x83`\x80\x015\x81`\x0C\x01\x81\x90UP_\x84`\xA0\x015\x14\x15\x80\x15a%nWP_\x84`\xC0\x015\x14\x15[\x15a%\xB4W\x83`\xC0\x015\x81`\x06\x01_\x86`\xA0\x015\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x83`\xA0\x015\x81`\x06\x01_\x86`\xC0\x015\x81R` \x01\x90\x81R` \x01_ \x81\x90UP[_\x84``\x015\x14a&gW_[\x84\x80`\xE0\x01\x90a%\xD1\x91\x90aq\x01V[\x90P\x81\x10\x15a&eW\x81`\x07\x01_\x86``\x015\x81R` \x01\x90\x81R` \x01_ \x85\x80`\xE0\x01\x90a&\x01\x91\x90aq\x01V[\x83\x81\x81\x10a&\x12Wa&\x11am.V[[\x90P` \x02\x81\x01\x90a&$\x91\x90amgV[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P\x81\x81a&V\x91\x90ao\x9AV[PP\x80\x80`\x01\x01\x91PPa%\xC1V[P[_\x84`\x80\x015\x14\x15\x80\x15a&\x8CWP_\x84\x80a\x01\0\x01\x90a&\x88\x91\x90an\rV[\x90P\x11[\x15a&\xE9W\x83\x80a\x01\0\x01\x90a&\xA2\x91\x90an\rV[\x82`\x0B\x01_\x87`\x80\x015\x81R` \x01\x90\x81R` \x01_ \x91\x82a&\xC6\x92\x91\x90anyV[P\x83a\x02@\x015\x81`\n\x01_\x86`\x80\x015\x81R` \x01\x90\x81R` \x01_ \x81\x90UP[a'u\x81\x85``\x015\x86\x80a\x01 \x01\x90a'\x03\x91\x90aqcV[\x88a\x01@\x015\x89\x80a\x02\xC0\x01\x90a'\x1A\x91\x90an\rV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x8Aa\x02\0\x01` \x81\x01\x90a'p\x91\x90aq\xD9V[aE=V[a(\x01\x81\x85`\x80\x015\x86\x80a\x01`\x01\x90a'\x8F\x91\x90aqcV[\x88a\x01\x80\x015\x89\x80a\x02\xE0\x01\x90a'\xA6\x91\x90an\rV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x8Aa\x02 \x01` \x81\x01\x90a'\xFC\x91\x90aq\xD9V[aE=V[a(\x8D\x81\x85`\xA0\x015\x86\x80a\x01\xA0\x01\x90a(\x1B\x91\x90aqcV[\x88a\x01\xC0\x015\x89\x80a\x02\xA0\x01\x90a(2\x91\x90an\rV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x8Aa\x01\xE0\x01` \x81\x01\x90a(\x88\x91\x90aq\xD9V[aE=V[_\x84`\xA0\x015\x14a(\xDAW\x83a\x01\xE0\x01` \x81\x01\x90a(\xAC\x91\x90aq\xD9V[\x81`\x01\x01_\x86`\xA0\x015\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP[_\x84``\x015\x14a)'W\x83a\x02\0\x01` \x81\x01\x90a(\xF9\x91\x90aq\xD9V[\x81`\x01\x01_\x86``\x015\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP[_\x84`\x80\x015\x14a)tW\x83a\x02 \x01` \x81\x01\x90a)F\x91\x90aq\xD9V[\x81`\x01\x01_\x86`\x80\x015\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP[_\x84`\xA0\x015\x14a)\xD1W\x83a\x02`\x01` \x81\x01\x90a)\x93\x91\x90ad\x05V[\x81`\r\x01_\x86`\xA0\x015\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a)\xCBWa)\xCAaY}V[[\x02\x17\x90UP[_\x84``\x015\x14a*.W\x83a\x02`\x01` \x81\x01\x90a)\xF0\x91\x90ad\x05V[\x81`\r\x01_\x86``\x015\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a*(Wa*'aY}V[[\x02\x17\x90UP[_\x84`\x80\x015\x14a*\x8BW\x83a\x02\x80\x01` \x81\x01\x90a*M\x91\x90ad\x05V[\x81`\r\x01_\x86`\x80\x015\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a*\x85Wa*\x84aY}V[[\x02\x17\x90UP[_\x84`\xA0\x015\x14\x15\x80\x15a*\xB0WP_\x84\x80a\x02\xA0\x01\x90a*\xAC\x91\x90an\rV[\x90P\x11[\x15a*\xECW\x83\x80a\x02\xA0\x01\x90a*\xC6\x91\x90an\rV[\x82`\x0E\x01_\x87`\xA0\x015\x81R` \x01\x90\x81R` \x01_ \x91\x82a*\xEA\x92\x91\x90anyV[P[_\x84``\x015\x14\x15\x80\x15a+\x11WP_\x84\x80a\x02\xC0\x01\x90a+\r\x91\x90an\rV[\x90P\x11[\x15a+MW\x83\x80a\x02\xC0\x01\x90a+'\x91\x90an\rV[\x82`\x0E\x01_\x87``\x015\x81R` \x01\x90\x81R` \x01_ \x91\x82a+K\x92\x91\x90anyV[P[_\x84`\x80\x015\x14\x15\x80\x15a+rWP_\x84\x80a\x02\xE0\x01\x90a+n\x91\x90an\rV[\x90P\x11[\x15a+\xAEW\x83\x80a\x02\xE0\x01\x90a+\x88\x91\x90an\rV[\x82`\x0E\x01_\x87`\x80\x015\x81R` \x01\x90\x81R` \x01_ \x91\x82a+\xAC\x92\x91\x90anyV[P[P_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa+\xF9\x91\x90ae\xD6V[`@Q\x80\x91\x03\x90\xA1PPPV[_\x80a,\x10a<\x15V[\x90P_\x81`\x05\x01T\x90P`\xF8`\x04\x90\x1B\x81\x14\x15\x80\x15a,LWP\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15a,\\W`\x01\x92PPPa,\xAEV[_\x82`\t\x01T\x90P`\xF8`\x05\x90\x1B\x81\x14\x15\x80\x15a,\x96WP\x82`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15a,\xA7W`\x01\x93PPPPa,\xAEV[_\x93PPPP[\x90V[_``\x80_\x80_``_a,\xC3aG\x80V[\x90P_\x80\x1B\x81_\x01T\x14\x80\x15a,\xDEWP_\x80\x1B\x81`\x01\x01T\x14[a-\x1DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a-\x14\x90arNV[`@Q\x80\x91\x03\x90\xFD[a-%aG\xA7V[a--aHEV[F0_\x80\x1B_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-LWa-Ka[\xDFV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a-zW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94\x93\x92\x91\x90\x97P\x97P\x97P\x97P\x97P\x97P\x97PP\x90\x91\x92\x93\x94\x95\x96V[``\x80_a-\xC6a<\x15V[\x90P\x80`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a.)W\x83`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a. \x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[\x80`\x0F\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a.\x8BW\x83`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.\x82\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[_\x81`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x82`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a/BW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a.\xF9W[PPPPP\x90P_\x81Q\x90P_\x81\x03a/\x92W\x86`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a/\x89\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[_a05\x85`\x0E\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80Ta/\xB4\x90agkV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta/\xE0\x90agkV[\x80\x15a0+W\x80`\x1F\x10a0\x02Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a0+V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a0\x0EW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa>\xACV[\x90P_a0C\x82\x85\x85a?LV[\x90P\x80\x86`\x07\x01_\x8B\x81R` \x01\x90\x81R` \x01_ \x80\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a1rW\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x01\x81\x11\x15a0\xBDWa0\xBCaY}V[[`\x01\x81\x11\x15a0\xCFWa0\xCEaY}V[[\x81R` \x01`\x01\x82\x01\x80Ta0\xE3\x90agkV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta1\x0F\x90agkV[\x80\x15a1ZW\x80`\x1F\x10a11Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a1ZV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a1=W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a0yV[PPPP\x90P\x97P\x97PPPPPPP\x91P\x91V[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[_\x80a1\xCAa<\x15V[\x90P\x80`\x0C\x01T\x91PP\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a24W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2X\x91\x90aepV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a2\xC7W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a2\xBE\x91\x90ae\x9BV[`@Q\x80\x91\x03\x90\xFD[_a2\xD0a<\x15V[\x90P\x80`\x05\x01T\x82\x11\x80a2\xE9WP`\xF8`\x04\x90\x1B\x82\x11\x15[\x15a3+W\x81`@Q\x7F\xFC\xF2\xDBz\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a3\"\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a3\x8DW\x81`@Q\x7F\x92x\x9Bg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a3\x84\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[`\x01\x81`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01\x81`\x0F\x01_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x81`\x06\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P_\x81\x14a4-W`\x01\x82`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP[\x7F+\x08{\x88K5\xA8\x1Dv\x9D\x1A\x1E\t(\x80\xF1\xDAV\xDE\x96NK3\x9E\xAB\xCB\x1FE\xF5\xFE2d\x83`@Qa4\\\x91\x90acoV[`@Q\x80\x91\x03\x90\xA1PPPV[``\x80_a4ua<\x15V[\x90P\x80`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a4\xD8W\x83`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a4\xCF\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[\x80`\x0F\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a5:W\x83`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a51\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[_\x81`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x82`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a5\xF1W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a5\xA8W[PPPPP\x90P_\x81Q\x90P_\x81\x03a6AW\x86`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a68\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[_a6\xE4\x85`\x0E\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80Ta6c\x90agkV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta6\x8F\x90agkV[\x80\x15a6\xDAW\x80`\x1F\x10a6\xB1Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a6\xDAV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a6\xBDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa>\xACV[\x90P_a6\xF2\x82\x85\x85a?LV[\x90P\x80\x86`\x0B\x01_\x8B\x81R` \x01\x90\x81R` \x01_ \x80\x80Ta7\x14\x90agkV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta7@\x90agkV[\x80\x15a7\x8BW\x80`\x1F\x10a7bWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a7\x8BV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a7nW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x97P\x97PPPPPPP\x91P\x91V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\xFEW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\"\x91\x90aepV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a8\x91W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a8\x88\x91\x90ae\x9BV[`@Q\x80\x91\x03\x90\xFD[_a8\x9Aa<\x15V[\x90P_\x81`\x05\x01T\x90P`\xF8`\x04\x90\x1B\x81\x14\x15\x80\x15a8\xD6WP\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15a9\x18W\x80`@Q\x7F;\x85=\xA8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a9\x0F\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[\x81`\x04\x01_\x81T\x80\x92\x91\x90a9,\x90af\x1CV[\x91\x90PUP_\x82`\x04\x01T\x90P\x82`\x05\x01_\x81T\x80\x92\x91\x90a9M\x90af\x1CV[\x91\x90PUP_\x83`\x05\x01T\x90P\x80\x84`\x06\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x81\x84`\x06\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UP_\x85\x85`\r\x01_\x85\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a9\xC0Wa9\xBFaY}V[[\x02\x17\x90UP\x85\x85`\r\x01_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a9\xFAWa9\xF9aY}V[[\x02\x17\x90UP_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:]W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:\x81\x91\x90afwV[\x90P_`\x02\x82\x84`@Q` \x01a:\x9A\x93\x92\x91\x90ag\x02V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80\x87`\x0E\x01_\x87\x81R` \x01\x90\x81R` \x01_ \x90\x81a:\xCB\x91\x90ai8V[P\x80\x87`\x0E\x01_\x86\x81R` \x01\x90\x81R` \x01_ \x90\x81a:\xEC\x91\x90ai8V[P\x7F\xC0\xAD\xA6\xFD\x1Bei\nTmI\xDFu1\xA2\x1CR\xEDP-\x14\xEF\xD9\x8E\xE0\xC8\xF4\x84j\xBB\x93\x10\x85\x84\x8A\x84`@Qa;\"\x94\x93\x92\x91\x90aj\x07V[`@Q\x80\x91\x03\x90\xA1PPPPPPPPV[_\x80a;>a<\x15V[\x90P\x80`\x08\x01T\x91PP\x90V[``_`\x01a;Y\x84aH\xE3V[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a;wWa;va[\xDFV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a;\xA9W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x01\x82\x01\x90P[`\x01\x15a<\nW\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a;\xFFWa;\xFEarlV[[\x04\x94P_\x85\x03a;\xB6W[\x81\x93PPPP\x91\x90PV[_\x7F&\xFD\xAF\x8A,\xB2\r \xB5^6!\x89\x86\x90^SN\xE7\xA9p\xDD/\xA8'\x94nKt\x96\xDB\0\x90P\x90V[_\x80\x83`\x03\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P_\x84`\x02\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x14\x15\x91PP\x92\x91PPV[_a<\x8Fa<\xAAV[_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[a<\xD9aJ4V[a<\xE3\x82\x82aJtV[PPV[_\x80\x85`\x06\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x86`\x0E\x01_\x87\x81R` \x01\x90\x81R` \x01_ \x80Ta=\x1E\x90agkV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta=J\x90agkV[\x80\x15a=\x95W\x80`\x1F\x10a=lWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a=\x95V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a=xW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90Pa=\xA9\x82\x87\x87\x87\x85aJ\xC5V[\x92PPP\x94\x93PPPPV[_\x80a>\x04\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPaL\xA6V[\x90Pa>\x10\x813aL\xD0V[\x80\x91PP\x93\x92PPPV[_\x80sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xB4r+\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>zW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>\x9E\x91\x90afwV[\x90P\x80\x83\x10\x15\x91PP\x91\x90PV[_`!\x82Q\x10\x15a??WsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a?\x14W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a?8\x91\x90afwV[\x90Pa?GV[`!\x82\x01Q\x90P[\x91\x90PV[``_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a?iWa?ha[\xDFV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a?\x9CW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a?\x87W\x90P[P\x90P_[\x83\x81\x10\x15a@\x83WsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c1\xFFA\xC8\x87\x87\x84\x81Q\x81\x10a?\xEDWa?\xECam.V[[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a@\x12\x92\x91\x90ajQV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a@,W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@T\x91\x90as\xECV[``\x01Q\x82\x82\x81Q\x81\x10a@kWa@jam.V[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa?\xA1V[P\x80\x91PP\x93\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80aA<WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aA#aO5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15aAsW`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aA\xD2W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aA\xF6\x91\x90aepV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14aBeW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aB\\\x91\x90ae\x9BV[`@Q\x80\x91\x03\x90\xFD[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15aB\xD0WP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aB\xCD\x91\x90at]V[`\x01[aC\x11W\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aC\x08\x91\x90ae\x9BV[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14aCwW\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aCn\x91\x90a]uV[`@Q\x80\x91\x03\x90\xFD[aC\x81\x83\x83aO\x88V[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14aD\x0BW`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_aDg`@Q\x80``\x01`@R\x80`<\x81R` \x01ax\xED`<\x919\x80Q\x90` \x01 \x84\x84\x80Q\x90` \x01 `@Q` \x01aDL\x93\x92\x91\x90at\x88V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 aO\xFAV[\x90P\x92\x91PPV[_\x80\x85`\n\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x86`\x0E\x01_\x87\x81R` \x01\x90\x81R` \x01_ \x80TaD\xA6\x90agkV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80TaD\xD2\x90agkV[\x80\x15aE\x1DW\x80`\x1F\x10aD\xF4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91aE\x1DV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11aE\0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90PaE1\x86\x83\x87\x87\x85aP\x13V[\x92PPP\x94\x93PPPPV[_\x86\x14\x80aEMWP_\x85\x85\x90P\x14[aGwW\x80\x15aEpW\x82\x87`\x03\x01_\x88\x81R` \x01\x90\x81R` \x01_ \x81\x90UP[_aEz\x83a>\xACV[\x90P_[\x86\x86\x90P\x81\x10\x15aGtW_\x87\x87\x83\x81\x81\x10aE\x9DWaE\x9Cam.V[[\x90P` \x02\x01` \x81\x01\x90aE\xB2\x91\x90at\xBDV[\x90P\x89`\x02\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x87\x81R` \x01\x90\x81R` \x01_ \x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c1\xFFA\xC8\x85\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aF\x84\x92\x91\x90ajQV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aF\x9EW=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aF\xC6\x91\x90as\xECV[` \x01Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14aGeW`\x01\x8B_\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP[PP\x80\x80`\x01\x01\x91PPaE~V[PP[PPPPPPPV[_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x90P\x90V[``_aG\xB2aG\x80V[\x90P\x80`\x02\x01\x80TaG\xC3\x90agkV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80TaG\xEF\x90agkV[\x80\x15aH:W\x80`\x1F\x10aH\x11Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91aH:V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11aH\x1DW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[``_aHPaG\x80V[\x90P\x80`\x03\x01\x80TaHa\x90agkV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80TaH\x8D\x90agkV[\x80\x15aH\xD8W\x80`\x1F\x10aH\xAFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91aH\xD8V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11aH\xBBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[_\x80_\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10aI?Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81aI5WaI4arlV[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10aI|Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81aIrWaIqarlV[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10aI\xABWf#\x86\xF2o\xC1\0\0\x83\x81aI\xA1WaI\xA0arlV[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10aI\xD4Wc\x05\xF5\xE1\0\x83\x81aI\xCAWaI\xC9arlV[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10aI\xF9Wa'\x10\x83\x81aI\xEFWaI\xEEarlV[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10aJ\x1CW`d\x83\x81aJ\x12WaJ\x11arlV[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10aJ+W`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[aJ<aP\xA4V[aJrW`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[aJ|aJ4V[_aJ\x85aG\x80V[\x90P\x82\x81`\x02\x01\x90\x81aJ\x98\x91\x90au@V[P\x81\x81`\x03\x01\x90\x81aJ\xAA\x91\x90au@V[P_\x80\x1B\x81_\x01\x81\x90UP_\x80\x1B\x81`\x01\x01\x81\x90UPPPPV[_\x80\x84\x84\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aJ\xE4WaJ\xE3a[\xDFV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aK\x12W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_[\x85\x85\x90P\x81\x10\x15aL\x16W`@Q\x80``\x01`@R\x80`%\x81R` \x01az\x01`%\x919\x80Q\x90` \x01 \x86\x86\x83\x81\x81\x10aKUWaKTam.V[[\x90P` \x02\x81\x01\x90aKg\x91\x90amgV[_\x01` \x81\x01\x90aKx\x91\x90av\x0FV[\x87\x87\x84\x81\x81\x10aK\x8BWaK\x8Aam.V[[\x90P` \x02\x81\x01\x90aK\x9D\x91\x90amgV[\x80` \x01\x90aK\xAC\x91\x90an\rV[`@QaK\xBA\x92\x91\x90avhV[`@Q\x80\x91\x03\x90 `@Q` \x01aK\xD4\x93\x92\x91\x90av\x8FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x82\x81Q\x81\x10aK\xFDWaK\xFCam.V[[` \x02` \x01\x01\x81\x81RPP\x80\x80`\x01\x01\x91PPaK\x17V[PaL\x9A`@Q\x80`\xC0\x01`@R\x80`\x82\x81R` \x01ay\x7F`\x82\x919\x80Q\x90` \x01 \x88\x88\x84`@Q` \x01aLM\x91\x90awuV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x87\x80Q\x90` \x01 `@Q` \x01aL\x7F\x95\x94\x93\x92\x91\x90aw\x8BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 aO\xFAV[\x91PP\x95\x94PPPPPV[_\x80_\x80aL\xB4\x86\x86aP\xC2V[\x92P\x92P\x92PaL\xC4\x82\x82aQ\x17V[\x82\x93PPPP\x92\x91PPV[_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aM.W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aMR\x91\x90afwV[\x90PsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x94G\xCF\xD4\x82\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aM\xA3\x92\x91\x90ajQV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aM\xBEW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aM\xE2\x91\x90aj\xA2V[aN%W\x82\x82`@Q\x7F\r\x86\xF5!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aN\x1C\x92\x91\x90aw\xDCV[`@Q\x80\x91\x03\x90\xFD[_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c1\xFFA\xC8\x83\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aNu\x92\x91\x90ajQV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aN\x8FW=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aN\xB7\x91\x90as\xECV[\x90P\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14aO/W\x83\x83`@Q\x7F\r\x86\xF5!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aO&\x92\x91\x90aw\xDCV[`@Q\x80\x91\x03\x90\xFD[PPPPV[_aOa\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1BaRyV[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[aO\x91\x82aR\x82V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15aO\xEDWaO\xE7\x82\x82aSKV[PaO\xF6V[aO\xF5aS\xCBV[[PPV[_aP\x0CaP\x06aT\x07V[\x83aT\x15V[\x90P\x91\x90PV[_aP\x99`@Q\x80`\x80\x01`@R\x80`V\x81R` \x01ay)`V\x919\x80Q\x90` \x01 \x87\x87\x87\x87`@Q` \x01aPL\x92\x91\x90avhV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x86\x80Q\x90` \x01 `@Q` \x01aP~\x95\x94\x93\x92\x91\x90aw\x8BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 aO\xFAV[\x90P\x95\x94PPPPPV[_aP\xADa<\xAAV[_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[_\x80_`A\x84Q\x03aQ\x02W_\x80_` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90PaP\xF4\x88\x82\x85\x85aTUV[\x95P\x95P\x95PPPPaQ\x10V[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15aQ*WaQ)aY}V[[\x82`\x03\x81\x11\x15aQ=WaQ<aY}V[[\x03\x15aRuW`\x01`\x03\x81\x11\x15aQWWaQVaY}V[[\x82`\x03\x81\x11\x15aQjWaQiaY}V[[\x03aQ\xA1W`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15aQ\xB5WaQ\xB4aY}V[[\x82`\x03\x81\x11\x15aQ\xC8WaQ\xC7aY}V[[\x03aR\x0CW\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aR\x03\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15aR\x1FWaR\x1EaY}V[[\x82`\x03\x81\x11\x15aR2WaR1aY}V[[\x03aRtW\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aRk\x91\x90a]uV[`@Q\x80\x91\x03\x90\xFD[[PPV[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03aR\xDDW\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aR\xD4\x91\x90ae\x9BV[`@Q\x80\x91\x03\x90\xFD[\x80aS\t\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1BaRyV[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@QaSt\x91\x90ax3V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14aS\xACW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>aS\xB1V[``\x91P[P\x91P\x91PaS\xC1\x85\x83\x83aU<V[\x92PPP\x92\x91PPV[_4\x11\x15aT\x05W`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_aT\x10aU\xC9V[\x90P\x90V[_`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x02\x82\x01R\x82`\"\x82\x01R`B\x81 \x91PP\x92\x91PPV[_\x80_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15aT\x91W_`\x03\x85\x92P\x92P\x92PaU2V[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@QaT\xB4\x94\x93\x92\x91\x90axXV[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aT\xD4W=_\x80>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03aU%W_`\x01_\x80\x1B\x93P\x93P\x93PPaU2V[\x80_\x80_\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[``\x82aUQWaUL\x82aV,V[aU\xC1V[_\x82Q\x14\x80\x15aUwWP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15aU\xB9W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aU\xB0\x91\x90ae\x9BV[`@Q\x80\x91\x03\x90\xFD[\x81\x90PaU\xC2V[[\x93\x92PPPV[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0FaU\xF3aVpV[aU\xFBaV\xE6V[F0`@Q` \x01aV\x11\x95\x94\x93\x92\x91\x90ax\x9BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x81Q\x11\x15aV>W\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80aVzaG\x80V[\x90P_aV\x85aG\xA7V[\x90P_\x81Q\x11\x15aV\xA1W\x80\x80Q\x90` \x01 \x92PPPaV\xE3V[_\x82_\x01T\x90P_\x80\x1B\x81\x14aV\xBCW\x80\x93PPPPaV\xE3V[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x80aV\xF0aG\x80V[\x90P_aV\xFBaHEV[\x90P_\x81Q\x11\x15aW\x17W\x80\x80Q\x90` \x01 \x92PPPaWZV[_\x82`\x01\x01T\x90P_\x80\x1B\x81\x14aW3W\x80\x93PPPPaWZV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15aW\x94W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90PaWyV[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_aW\xB9\x82aW]V[aW\xC3\x81\x85aWgV[\x93PaW\xD3\x81\x85` \x86\x01aWwV[aW\xDC\x81aW\x9FV[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaW\xFF\x81\x84aW\xAFV[\x90P\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x81\x90P\x91\x90PV[aX*\x81aX\x18V[\x81\x14aX4W_\x80\xFD[PV[_\x815\x90PaXE\x81aX!V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aX`WaX_aX\x10V[[_aXm\x84\x82\x85\x01aX7V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_aX\xC8\x82aX\x9FV[\x90P\x91\x90PV[aX\xD8\x81aX\xBEV[\x82RPPV[_aX\xE9\x83\x83aX\xCFV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aY\x0B\x82aXvV[aY\x15\x81\x85aX\x80V[\x93PaY \x83aX\x90V[\x80_[\x83\x81\x10\x15aYPW\x81QaY7\x88\x82aX\xDEV[\x97PaYB\x83aX\xF5V[\x92PP`\x01\x81\x01\x90PaY#V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaYu\x81\x84aY\x01V[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x02\x81\x10aY\xBBWaY\xBAaY}V[[PV[_\x81\x90PaY\xCB\x82aY\xAAV[\x91\x90PV[_aY\xDA\x82aY\xBEV[\x90P\x91\x90PV[aY\xEA\x81aY\xD0V[\x82RPPV[_` \x82\x01\x90PaZ\x03_\x83\x01\x84aY\xE1V[\x92\x91PPV[`\x02\x81\x10aZ\x15W_\x80\xFD[PV[_\x815\x90PaZ&\x81aZ\tV[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15aZBWaZAaX\x10V[[_aZO\x85\x82\x86\x01aX7V[\x92PP` aZ`\x85\x82\x86\x01aZ\x18V[\x91PP\x92P\x92\x90PV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12aZ\x8BWaZ\x8AaZjV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZ\xA8WaZ\xA7aZnV[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aZ\xC4WaZ\xC3aZrV[[\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12aZ\xE0WaZ\xDFaZjV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZ\xFDWaZ\xFCaZnV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a[\x19Wa[\x18aZrV[[\x92P\x92\x90PV[_\x80_\x80_``\x86\x88\x03\x12\x15a[9Wa[8aX\x10V[[_a[F\x88\x82\x89\x01aX7V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a[gWa[faX\x14V[[a[s\x88\x82\x89\x01aZvV[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a[\x96Wa[\x95aX\x14V[[a[\xA2\x88\x82\x89\x01aZ\xCBV[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[a[\xBA\x81aX\xBEV[\x81\x14a[\xC4W_\x80\xFD[PV[_\x815\x90Pa[\xD5\x81a[\xB1V[\x92\x91PPV[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\\\x15\x82aW\x9FV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\\4Wa\\3a[\xDFV[[\x80`@RPPPV[_a\\FaX\x07V[\x90Pa\\R\x82\x82a\\\x0CV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\\qWa\\pa[\xDFV[[a\\z\x82aW\x9FV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a\\\xA7a\\\xA2\x84a\\WV[a\\=V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\\\xC3Wa\\\xC2a[\xDBV[[a\\\xCE\x84\x82\x85a\\\x87V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\\\xEAWa\\\xE9aZjV[[\x815a\\\xFA\x84\x82` \x86\x01a\\\x95V[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a]\x19Wa]\x18aX\x10V[[_a]&\x85\x82\x86\x01a[\xC7V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a]GWa]FaX\x14V[[a]S\x85\x82\x86\x01a\\\xD6V[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[a]o\x81a]]V[\x82RPPV[_` \x82\x01\x90Pa]\x88_\x83\x01\x84a]fV[\x92\x91PPV[_\x80_`@\x84\x86\x03\x12\x15a]\xA5Wa]\xA4aX\x10V[[_a]\xB2\x86\x82\x87\x01aX7V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a]\xD3Wa]\xD2aX\x14V[[a]\xDF\x86\x82\x87\x01aZ\xCBV[\x92P\x92PP\x92P\x92P\x92V[_\x80_\x80_``\x86\x88\x03\x12\x15a^\x04Wa^\x03aX\x10V[[_a^\x11\x88\x82\x89\x01aX7V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a^2Wa^1aX\x14V[[a^>\x88\x82\x89\x01aZ\xCBV[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a^aWa^`aX\x14V[[a^m\x88\x82\x89\x01aZ\xCBV[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[_\x80\xFD[_a\x03\0\x82\x84\x03\x12\x15a^\x96Wa^\x95a^|V[[\x81\x90P\x92\x91PPV[_` \x82\x84\x03\x12\x15a^\xB4Wa^\xB3aX\x10V[[_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a^\xD1Wa^\xD0aX\x14V[[a^\xDD\x84\x82\x85\x01a^\x80V[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a^\xFA\x81a^\xE6V[\x82RPPV[_` \x82\x01\x90Pa_\x13_\x83\x01\x84a^\xF1V[\x92\x91PPV[_\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a_M\x81a_\x19V[\x82RPPV[a_\\\x81aX\x18V[\x82RPPV[a_k\x81aX\xBEV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a_\xA3\x81aX\x18V[\x82RPPV[_a_\xB4\x83\x83a_\x9AV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a_\xD6\x82a_qV[a_\xE0\x81\x85a_{V[\x93Pa_\xEB\x83a_\x8BV[\x80_[\x83\x81\x10\x15a`\x1BW\x81Qa`\x02\x88\x82a_\xA9V[\x97Pa`\r\x83a_\xC0V[\x92PP`\x01\x81\x01\x90Pa_\xEEV[P\x85\x93PPPP\x92\x91PPV[_`\xE0\x82\x01\x90Pa`;_\x83\x01\x8Aa_DV[\x81\x81\x03` \x83\x01Ra`M\x81\x89aW\xAFV[\x90P\x81\x81\x03`@\x83\x01Ra`a\x81\x88aW\xAFV[\x90Pa`p``\x83\x01\x87a_SV[a`}`\x80\x83\x01\x86a_bV[a`\x8A`\xA0\x83\x01\x85a]fV[\x81\x81\x03`\xC0\x83\x01Ra`\x9C\x81\x84a_\xCCV[\x90P\x98\x97PPPPPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a`\xED\x82aW]V[a`\xF7\x81\x85a`\xD3V[\x93Paa\x07\x81\x85` \x86\x01aWwV[aa\x10\x81aW\x9FV[\x84\x01\x91PP\x92\x91PPV[_aa&\x83\x83a`\xE3V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aaD\x82a`\xAAV[aaN\x81\x85a`\xB4V[\x93P\x83` \x82\x02\x85\x01aa`\x85a`\xC4V[\x80_[\x85\x81\x10\x15aa\x9BW\x84\x84\x03\x89R\x81Qaa|\x85\x82aa\x1BV[\x94Paa\x87\x83aa.V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaacV[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[`\x02\x81\x10aa\xE7Waa\xE6aY}V[[PV[_\x81\x90Paa\xF7\x82aa\xD6V[\x91\x90PV[_ab\x06\x82aa\xEAV[\x90P\x91\x90PV[ab\x16\x81aa\xFCV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_ab@\x82ab\x1CV[abJ\x81\x85ab&V[\x93PabZ\x81\x85` \x86\x01aWwV[abc\x81aW\x9FV[\x84\x01\x91PP\x92\x91PPV[_`@\x83\x01_\x83\x01Qab\x83_\x86\x01\x82ab\rV[P` \x83\x01Q\x84\x82\x03` \x86\x01Rab\x9B\x82\x82ab6V[\x91PP\x80\x91PP\x92\x91PPV[_ab\xB3\x83\x83abnV[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_ab\xD1\x82aa\xADV[ab\xDB\x81\x85aa\xB7V[\x93P\x83` \x82\x02\x85\x01ab\xED\x85aa\xC7V[\x80_[\x85\x81\x10\x15ac(W\x84\x84\x03\x89R\x81Qac\t\x85\x82ab\xA8V[\x94Pac\x14\x83ab\xBBV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pab\xF0V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01RacR\x81\x85aa:V[\x90P\x81\x81\x03` \x83\x01Racf\x81\x84ab\xC7V[\x90P\x93\x92PPPV[_` \x82\x01\x90Pac\x82_\x83\x01\x84a_SV[\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_ac\xA2\x82ab\x1CV[ac\xAC\x81\x85ac\x88V[\x93Pac\xBC\x81\x85` \x86\x01aWwV[ac\xC5\x81aW\x9FV[\x84\x01\x91PP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Rac\xE8\x81\x85aa:V[\x90P\x81\x81\x03` \x83\x01Rac\xFC\x81\x84ac\x98V[\x90P\x93\x92PPPV[_` \x82\x84\x03\x12\x15ad\x1AWad\x19aX\x10V[[_ad'\x84\x82\x85\x01aZ\x18V[\x91PP\x92\x91PPV[_\x81\x90P\x92\x91PPV[_adD\x82aW]V[adN\x81\x85ad0V[\x93Pad^\x81\x85` \x86\x01aWwV[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_ad\x9E`\x02\x83ad0V[\x91Pad\xA9\x82adjV[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_ad\xE8`\x01\x83ad0V[\x91Pad\xF3\x82ad\xB4V[`\x01\x82\x01\x90P\x91\x90PV[_ae\t\x82\x87ad:V[\x91Pae\x14\x82ad\x92V[\x91Pae \x82\x86ad:V[\x91Pae+\x82ad\xDCV[\x91Pae7\x82\x85ad:V[\x91PaeB\x82ad\xDCV[\x91PaeN\x82\x84ad:V[\x91P\x81\x90P\x95\x94PPPPPV[_\x81Q\x90Paej\x81a[\xB1V[\x92\x91PPV[_` \x82\x84\x03\x12\x15ae\x85Wae\x84aX\x10V[[_ae\x92\x84\x82\x85\x01ae\\V[\x91PP\x92\x91PPV[_` \x82\x01\x90Pae\xAE_\x83\x01\x84a_bV[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[ae\xD0\x81ae\xB4V[\x82RPPV[_` \x82\x01\x90Pae\xE9_\x83\x01\x84ae\xC7V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_af&\x82aX\x18V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03afXWafWae\xEFV[[`\x01\x82\x01\x90P\x91\x90PV[_\x81Q\x90Pafq\x81aX!V[\x92\x91PPV[_` \x82\x84\x03\x12\x15af\x8CWaf\x8BaX\x10V[[_af\x99\x84\x82\x85\x01afcV[\x91PP\x92\x91PPV[_`\xFF\x82\x16\x90P\x91\x90PV[_\x81`\xF8\x1B\x90P\x91\x90PV[_af\xC4\x82af\xAEV[\x90P\x91\x90PV[af\xDCaf\xD7\x82af\xA2V[af\xBAV[\x82RPPV[_\x81\x90P\x91\x90PV[af\xFCaf\xF7\x82aX\x18V[af\xE2V[\x82RPPV[_ag\r\x82\x86af\xCBV[`\x01\x82\x01\x91Pag\x1D\x82\x85af\xEBV[` \x82\x01\x91Pag-\x82\x84af\xEBV[` \x82\x01\x91P\x81\x90P\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80ag\x82W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03ag\x95Wag\x94ag>V[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02ag\xF7\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82ag\xBCV[ah\x01\x86\x83ag\xBCV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_ah<ah7ah2\x84aX\x18V[ah\x19V[aX\x18V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[ahU\x83ah\"V[ahiaha\x82ahCV[\x84\x84Tag\xC8V[\x82UPPPPV[_\x90V[ah}ahqV[ah\x88\x81\x84\x84ahLV[PPPV[[\x81\x81\x10\x15ah\xABWah\xA0_\x82ahuV[`\x01\x81\x01\x90Pah\x8EV[PPV[`\x1F\x82\x11\x15ah\xF0Wah\xC1\x81ag\x9BV[ah\xCA\x84ag\xADV[\x81\x01` \x85\x10\x15ah\xD9W\x81\x90P[ah\xEDah\xE5\x85ag\xADV[\x83\x01\x82ah\x8DV[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_ai\x10_\x19\x84`\x08\x02ah\xF5V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_ai(\x83\x83ai\x01V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[aiA\x82ab\x1CV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aiZWaiYa[\xDFV[[aid\x82TagkV[aio\x82\x82\x85ah\xAFV[_` \x90P`\x1F\x83\x11`\x01\x81\x14ai\xA0W_\x84\x15ai\x8EW\x82\x87\x01Q\x90P[ai\x98\x85\x82ai\x1DV[\x86UPai\xFFV[`\x1F\x19\x84\x16ai\xAE\x86ag\x9BV[_[\x82\x81\x10\x15ai\xD5W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pai\xB0V[\x86\x83\x10\x15ai\xF2W\x84\x89\x01Qai\xEE`\x1F\x89\x16\x82ai\x01V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_`\x80\x82\x01\x90Paj\x1A_\x83\x01\x87a_SV[aj'` \x83\x01\x86a_SV[aj4`@\x83\x01\x85aY\xE1V[\x81\x81\x03``\x83\x01RajF\x81\x84ac\x98V[\x90P\x95\x94PPPPPV[_`@\x82\x01\x90Pajd_\x83\x01\x85a_SV[ajq` \x83\x01\x84a_bV[\x93\x92PPPV[aj\x81\x81a^\xE6V[\x81\x14aj\x8BW_\x80\xFD[PV[_\x81Q\x90Paj\x9C\x81ajxV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aj\xB7Waj\xB6aX\x10V[[_aj\xC4\x84\x82\x85\x01aj\x8EV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[`\x02\x81\x10aj\xE2W_\x80\xFD[PV[_\x815\x90Paj\xF3\x81aj\xD6V[\x92\x91PPV[_ak\x07` \x84\x01\x84aj\xE5V[\x90P\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12ak7Wak6ak\x17V[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15ak_Wak^ak\x0FV[[`\x01\x82\x026\x03\x83\x13\x15akuWaktak\x13V[[P\x92P\x92\x90PV[_ak\x88\x83\x85ab&V[\x93Pak\x95\x83\x85\x84a\\\x87V[ak\x9E\x83aW\x9FV[\x84\x01\x90P\x93\x92PPPV[_`@\x83\x01ak\xBA_\x84\x01\x84aj\xF9V[ak\xC6_\x86\x01\x82ab\rV[Pak\xD4` \x84\x01\x84ak\x1BV[\x85\x83\x03` \x87\x01Rak\xE7\x83\x82\x84ak}V[\x92PPP\x80\x91PP\x92\x91PPV[_al\0\x83\x83ak\xA9V[\x90P\x92\x91PPV[_\x825`\x01`@\x03\x836\x03\x03\x81\x12al#Wal\"ak\x17V[[\x82\x81\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_alF\x83\x85aa\xB7V[\x93P\x83` \x84\x02\x85\x01alX\x84aj\xCDV[\x80_[\x87\x81\x10\x15al\x9BW\x84\x84\x03\x89Ralr\x82\x84al\x08V[al|\x85\x82ak\xF5V[\x94Pal\x87\x83al/V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pal[V[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_al\xB8\x83\x85ac\x88V[\x93Pal\xC5\x83\x85\x84a\\\x87V[al\xCE\x83aW\x9FV[\x84\x01\x90P\x93\x92PPPV[_`\x80\x82\x01\x90Pal\xEC_\x83\x01\x89a_SV[\x81\x81\x03` \x83\x01Ral\xFF\x81\x87\x89al;V[\x90P\x81\x81\x03`@\x83\x01Ram\x14\x81\x85\x87al\xADV[\x90Pam#``\x83\x01\x84a_bV[\x97\x96PPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x825`\x01`@\x03\x836\x03\x03\x81\x12am\x82Wam\x81am[V[[\x80\x83\x01\x91PP\x92\x91PPV[_\x815am\x9A\x81aj\xD6V[\x80\x91PP\x91\x90PV[_\x81_\x1B\x90P\x91\x90PV[_`\xFFam\xBA\x84am\xA3V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_am\xDA\x82aa\xEAV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[am\xF3\x82am\xD0V[an\x06am\xFF\x82am\xE1V[\x83Tam\xAEV[\x82UPPPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12an)Wan(am[V[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15anKWanJam_V[[` \x83\x01\x92P`\x01\x82\x026\x03\x83\x13\x15angWanfamcV[[P\x92P\x92\x90PV[_\x82\x90P\x92\x91PPV[an\x83\x83\x83anoV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15an\x9CWan\x9Ba[\xDFV[[an\xA6\x82TagkV[an\xB1\x82\x82\x85ah\xAFV[_`\x1F\x83\x11`\x01\x81\x14an\xDEW_\x84\x15an\xCCW\x82\x87\x015\x90P[an\xD6\x85\x82ai\x1DV[\x86UPao=V[`\x1F\x19\x84\x16an\xEC\x86ag\x9BV[_[\x82\x81\x10\x15ao\x13W\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pan\xEEV[\x86\x83\x10\x15ao0W\x84\x89\x015ao,`\x1F\x89\x16\x82ai\x01V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[aoQ\x83\x83\x83anyV[PPPV[_\x81\x01_\x83\x01\x80aof\x81am\x8EV[\x90Paor\x81\x84am\xEAV[PPP`\x01\x81\x01` \x83\x01ao\x87\x81\x85an\rV[ao\x92\x81\x83\x86aoFV[PPPPPPV[ao\xA4\x82\x82aoVV[PPV[_``\x82\x01\x90Pao\xBB_\x83\x01\x87a_SV[\x81\x81\x03` \x83\x01Rao\xCD\x81\x86aa:V[\x90P\x81\x81\x03`@\x83\x01Rao\xE2\x81\x84\x86al;V[\x90P\x95\x94PPPPPV[_``\x82\x01\x90Pap\0_\x83\x01\x87a_SV[\x81\x81\x03` \x83\x01Rap\x13\x81\x85\x87al\xADV[\x90Pap\"`@\x83\x01\x84a_bV[\x95\x94PPPPPV[_``\x82\x01\x90Pap>_\x83\x01\x86a_SV[apK` \x83\x01\x85a_SV[\x81\x81\x03`@\x83\x01Rap]\x81\x84ac\x98V[\x90P\x94\x93PPPPV[_`\x80\x82\x01\x90Papz_\x83\x01\x89a_SV[\x81\x81\x03` \x83\x01Rap\x8D\x81\x87\x89al\xADV[\x90P\x81\x81\x03`@\x83\x01Rap\xA2\x81\x85\x87al\xADV[\x90Pap\xB1``\x83\x01\x84a_bV[\x97\x96PPPPPPPV[_``\x82\x01\x90Pap\xCF_\x83\x01\x87a_SV[\x81\x81\x03` \x83\x01Rap\xE1\x81\x86aa:V[\x90P\x81\x81\x03`@\x83\x01Rap\xF6\x81\x84\x86al\xADV[\x90P\x95\x94PPPPPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aq\x1DWaq\x1Cam[V[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aq?Waq>am_V[[` \x83\x01\x92P` \x82\x026\x03\x83\x13\x15aq[WaqZamcV[[P\x92P\x92\x90PV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aq\x7FWaq~am[V[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aq\xA1Waq\xA0am_V[[` \x83\x01\x92P` \x82\x026\x03\x83\x13\x15aq\xBDWaq\xBCamcV[[P\x92P\x92\x90PV[_\x815\x90Paq\xD3\x81ajxV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aq\xEEWaq\xEDaX\x10V[[_aq\xFB\x84\x82\x85\x01aq\xC5V[\x91PP\x92\x91PPV[\x7FEIP712: Uninitialized\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_ar8`\x15\x83aWgV[\x91ParC\x82ar\x04V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Rare\x81ar,V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15ar\xBBWar\xBAa[\xDFV[[ar\xC4\x82aW\x9FV[\x90P` \x81\x01\x90P\x91\x90PV[_ar\xE3ar\xDE\x84ar\xA1V[a\\=V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15ar\xFFWar\xFEa[\xDBV[[as\n\x84\x82\x85aWwV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12as&Was%aZjV[[\x81Qas6\x84\x82` \x86\x01ar\xD1V[\x91PP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15asTWasSar\x99V[[as^`\x80a\\=V[\x90P_asm\x84\x82\x85\x01ae\\V[_\x83\x01RP` as\x80\x84\x82\x85\x01ae\\V[` \x83\x01RP`@\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15as\xA4Was\xA3ar\x9DV[[as\xB0\x84\x82\x85\x01as\x12V[`@\x83\x01RP``\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15as\xD4Was\xD3ar\x9DV[[as\xE0\x84\x82\x85\x01as\x12V[``\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15at\x01Wat\0aX\x10V[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15at\x1EWat\x1DaX\x14V[[at*\x84\x82\x85\x01as?V[\x91PP\x92\x91PPV[at<\x81a]]V[\x81\x14atFW_\x80\xFD[PV[_\x81Q\x90PatW\x81at3V[\x92\x91PPV[_` \x82\x84\x03\x12\x15atrWatqaX\x10V[[_at\x7F\x84\x82\x85\x01atIV[\x91PP\x92\x91PPV[_``\x82\x01\x90Pat\x9B_\x83\x01\x86a]fV[at\xA8` \x83\x01\x85a_SV[at\xB5`@\x83\x01\x84a]fV[\x94\x93PPPPV[_` \x82\x84\x03\x12\x15at\xD2Wat\xD1aX\x10V[[_at\xDF\x84\x82\x85\x01a[\xC7V[\x91PP\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[`\x1F\x82\x11\x15au;Wau\x0C\x81at\xE8V[au\x15\x84ag\xADV[\x81\x01` \x85\x10\x15au$W\x81\x90P[au8au0\x85ag\xADV[\x83\x01\x82ah\x8DV[PP[PPPV[auI\x82aW]V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aubWauaa[\xDFV[[aul\x82TagkV[auw\x82\x82\x85at\xFAV[_` \x90P`\x1F\x83\x11`\x01\x81\x14au\xA8W_\x84\x15au\x96W\x82\x87\x01Q\x90P[au\xA0\x85\x82ai\x1DV[\x86UPav\x07V[`\x1F\x19\x84\x16au\xB6\x86at\xE8V[_[\x82\x81\x10\x15au\xDDW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pau\xB8V[\x86\x83\x10\x15au\xFAW\x84\x89\x01Qau\xF6`\x1F\x89\x16\x82ai\x01V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_` \x82\x84\x03\x12\x15av$Wav#aX\x10V[[_av1\x84\x82\x85\x01aj\xE5V[\x91PP\x92\x91PPV[_\x81\x90P\x92\x91PPV[_avO\x83\x85av:V[\x93Pav\\\x83\x85\x84a\\\x87V[\x82\x84\x01\x90P\x93\x92PPPV[_avt\x82\x84\x86avDV[\x91P\x81\x90P\x93\x92PPPV[av\x89\x81aa\xFCV[\x82RPPV[_``\x82\x01\x90Pav\xA2_\x83\x01\x86a]fV[av\xAF` \x83\x01\x85av\x80V[av\xBC`@\x83\x01\x84a]fV[\x94\x93PPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[av\xF0\x81a]]V[\x82RPPV[_aw\x01\x83\x83av\xE7V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aw#\x82av\xC4V[aw-\x81\x85av\xCEV[\x93Paw8\x83av\xD8V[\x80_[\x83\x81\x10\x15awhW\x81QawO\x88\x82av\xF6V[\x97PawZ\x83aw\rV[\x92PP`\x01\x81\x01\x90Paw;V[P\x85\x93PPPP\x92\x91PPV[_aw\x80\x82\x84aw\x19V[\x91P\x81\x90P\x92\x91PPV[_`\xA0\x82\x01\x90Paw\x9E_\x83\x01\x88a]fV[aw\xAB` \x83\x01\x87a_SV[aw\xB8`@\x83\x01\x86a_SV[aw\xC5``\x83\x01\x85a]fV[aw\xD2`\x80\x83\x01\x84a]fV[\x96\x95PPPPPPV[_`@\x82\x01\x90Paw\xEF_\x83\x01\x85a_bV[aw\xFC` \x83\x01\x84a_bV[\x93\x92PPPV[_ax\r\x82ab\x1CV[ax\x17\x81\x85av:V[\x93Pax'\x81\x85` \x86\x01aWwV[\x80\x84\x01\x91PP\x92\x91PPV[_ax>\x82\x84ax\x03V[\x91P\x81\x90P\x92\x91PPV[axR\x81af\xA2V[\x82RPPV[_`\x80\x82\x01\x90Paxk_\x83\x01\x87a]fV[axx` \x83\x01\x86axIV[ax\x85`@\x83\x01\x85a]fV[ax\x92``\x83\x01\x84a]fV[\x95\x94PPPPPV[_`\xA0\x82\x01\x90Pax\xAE_\x83\x01\x88a]fV[ax\xBB` \x83\x01\x87a]fV[ax\xC8`@\x83\x01\x86a]fV[ax\xD5``\x83\x01\x85a_SV[ax\xE2`\x80\x83\x01\x84a_bV[\x96\x95PPPPPPV\xFEPrepKeygenVerification(uint256 prepKeygenId,bytes extraData)CrsgenVerification(uint256 crsId,uint256 maxBitLength,bytes crsDigest,bytes extraData)KeygenVerification(uint256 prepKeygenId,uint256 keyId,KeyDigest[] keyDigests,bytes extraData)KeyDigest(uint8 keyType,bytes digest)KeyDigest(uint8 keyType,bytes digest)",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405260043610610134575f3560e01c806362978787116100aa578063ad3cb1cc1161006e578063ad3cb1cc146103f9578063baff211e14610423578063c2c1faee1461044d578063c55b872414610475578063caa367db146104b2578063d52f10eb146104da57610134565b806362978787146103125780636c68e71f1461033a578063735b12631461036257806384b0196e1461038c578063936608ae146103bc57610134565b80633c02f834116100fc5780633c02f8341461021857806345af261b146102405780634610ffe81461027c5780634f1ef286146102a457806352d1902d146102c0578063589adb0e146102ea57610134565b80630d8e6e2c1461013857806316c713d9146101625780631703c61a1461019e57806319f4f632146101c657806339f7381014610202575b5f80fd5b348015610143575f80fd5b5061014c610504565b60405161015991906157e7565b60405180910390f35b34801561016d575f80fd5b506101886004803603810190610183919061584b565b61057f565b604051610195919061595d565b60405180910390f35b3480156101a9575f80fd5b506101c460048036038101906101bf919061584b565b610650565b005b3480156101d1575f80fd5b506101ec60048036038101906101e7919061584b565b610897565b6040516101f991906159f0565b60405180910390f35b34801561020d575f80fd5b50610216610a22565b005b348015610223575f80fd5b5061023e60048036038101906102399190615a2c565b610c3c565b005b34801561024b575f80fd5b506102666004803603810190610261919061584b565b610f3b565b60405161027391906159f0565b60405180910390f35b348015610287575f80fd5b506102a2600480360381019061029d9190615b20565b61107d565b005b6102be60048036038101906102b99190615d03565b61176f565b005b3480156102cb575f80fd5b506102d461178e565b6040516102e19190615d75565b60405180910390f35b3480156102f5575f80fd5b50610310600480360381019061030b9190615d8e565b6117bf565b005b34801561031d575f80fd5b5061033860048036038101906103339190615deb565b611ccf565b005b348015610345575f80fd5b50610360600480360381019061035b9190615e9f565b61229a565b005b34801561036d575f80fd5b50610376612c06565b6040516103839190615f00565b60405180910390f35b348015610397575f80fd5b506103a0612cb1565b6040516103b39796959493929190616028565b60405180910390f35b3480156103c7575f80fd5b506103e260048036038101906103dd919061584b565b612dba565b6040516103f092919061633a565b60405180910390f35b348015610404575f80fd5b5061040d613187565b60405161041a91906157e7565b60405180910390f35b34801561042e575f80fd5b506104376131c0565b604051610444919061636f565b60405180910390f35b348015610458575f80fd5b50610473600480360381019061046e919061584b565b6131d7565b005b348015610480575f80fd5b5061049b6004803603810190610496919061584b565b613469565b6040516104a99291906163d0565b60405180910390f35b3480156104bd575f80fd5b506104d860048036038101906104d39190616405565b6137a1565b005b3480156104e5575f80fd5b506104ee613b34565b6040516104fb919061636f565b60405180910390f35b60606040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506105455f613b4b565b61054f6001613b4b565b6105585f613b4b565b60405160200161056b94939291906164fe565b604051602081830303815290604052905090565b60605f61058a613c15565b90505f816003015f8581526020019081526020015f20549050816002015f8581526020019081526020015f205f8281526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561064257602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116105f9575b505050505092505050919050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156106ad573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106d19190616570565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461074057336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401610737919061659b565b60405180910390fd5b5f610749613c15565b90508060090154821180610762575060f86005901b8211155b156107a457816040517fcbe9265600000000000000000000000000000000000000000000000000000000815260040161079b919061636f565b60405180910390fd5b806001015f8381526020019081526020015f205f9054906101000a900460ff161561080657816040517fdf0db5fb0000000000000000000000000000000000000000000000000000000081526004016107fd919061636f565b60405180910390fd5b6001816001015f8481526020019081526020015f205f6101000a81548160ff021916908315150217905550600181600f015f8481526020019081526020015f205f6101000a81548160ff0219169083151502179055507f384f90fefbcfaa68f22e00094aeaa52b2bc693936d2ce1afed1212520b59b58e8260405161088b919061636f565b60405180910390a15050565b5f806108a1613c15565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff1661090457826040517f84de13310000000000000000000000000000000000000000000000000000000081526004016108fb919061636f565b60405180910390fd5b80600f015f8481526020019081526020015f205f9054906101000a900460ff161561096657826040517f84de133100000000000000000000000000000000000000000000000000000000815260040161095d919061636f565b60405180910390fd5b6109708184613c3c565b6109b157826040517f84de13310000000000000000000000000000000000000000000000000000000081526004016109a8919061636f565b60405180910390fd5b5f816006015f8581526020019081526020015f205490505f81036109f85781600d015f8581526020019081526020015f205f9054906101000a900460ff1692505050610a1d565b81600d015f8281526020019081526020015f205f9054906101000a900460ff16925050505b919050565b6001610a2c613c86565b67ffffffffffffffff1614610a6d576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60025f610a78613caa565b9050805f0160089054906101000a900460ff1680610ac057508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15610af7576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff021916908315150217905550610bb06040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506040518060400160405280600181526020017f3100000000000000000000000000000000000000000000000000000000000000815250613cd1565b5f610bb9613c15565b905060f86003901b816004018190555060f86004901b816005018190555060f86005901b8160090181905550505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051610c3091906165d6565b60405180910390a15050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610c99573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cbd9190616570565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610d2c57336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401610d23919061659b565b60405180910390fd5b5f610d35613c15565b90505f8160090154905060f86005901b8114158015610d715750816001015f8281526020019081526020015f205f9054906101000a900460ff16155b15610db357806040517f061ac61d000000000000000000000000000000000000000000000000000000008152600401610daa919061636f565b60405180910390fd5b816009015f815480929190610dc79061661c565b91905055505f826009015490508483600a015f8381526020019081526020015f20819055508383600d015f8381526020019081526020015f205f6101000a81548160ff02191690836001811115610e2157610e2061597d565b5b02179055505f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa158015610e84573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ea89190616677565b90505f8060028383604051602001610ec293929190616702565b60405160208183030381529060405290508086600e015f8681526020019081526020015f209081610ef39190616938565b507f8cf0151393f84fd694c5e315cb74cc05b247de0a454fd9e9129c661efdf9401d84898984604051610f299493929190616a07565b60405180910390a15050505050505050565b5f80610f45613c15565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff16610fa857826040517fda32d00f000000000000000000000000000000000000000000000000000000008152600401610f9f919061636f565b60405180910390fd5b80600f015f8481526020019081526020015f205f9054906101000a900460ff161561100a57826040517fda32d00f000000000000000000000000000000000000000000000000000000008152600401611001919061636f565b60405180910390fd5b6110148184613c3c565b61105557826040517fda32d00f00000000000000000000000000000000000000000000000000000000815260040161104c919061636f565b60405180910390fd5b80600d015f8481526020019081526020015f205f9054906101000a900460ff16915050919050565b5f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa1580156110db573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110ff9190616677565b90507344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166346c5bbbd82336040518363ffffffff1660e01b8152600401611150929190616a51565b602060405180830381865afa15801561116b573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061118f9190616aa2565b6111d057336040517faee863230000000000000000000000000000000000000000000000000000000081526004016111c7919061659b565b60405180910390fd5b5f6111d9613c15565b905080600501548711806111f2575060f86004901b8711155b1561123457866040517fadfab90400000000000000000000000000000000000000000000000000000000815260040161122b919061636f565b60405180910390fd5b5f868690500361127b57866040517fe6f9083b000000000000000000000000000000000000000000000000000000008152600401611272919061636f565b60405180910390fd5b5f61128882898989613ce7565b90505f826006015f8a81526020019081526020015f205490505f81141580156112ce5750826001015f8281526020019081526020015f205f9054906101000a900460ff16155b15611305576040517f6fbcdd2b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f611311838888613db5565b9050835f015f8b81526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16156113b15789816040517f98fb957d0000000000000000000000000000000000000000000000000000000081526004016113a8929190616a51565b60405180910390fd5b6001845f015f8c81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f846002015f8c81526020019081526020015f205f8581526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507f2afe64fb3afde8e2678aea84cf36223f330e2fb1286d37aed573ab9cd1db47c78b8b8b8b8b336040516114d396959493929190616cd9565b60405180910390a1846001015f8c81526020019081526020015f205f9054906101000a900460ff1615801561151157506115108180549050613e1b565b5b15611762576001856001015f8d81526020019081526020015f205f6101000a81548160ff0219169083151502179055505f5b8a8a90508110156115c757856007015f8d81526020019081526020015f208b8b8381811061157457611573616d2e565b5b90506020028101906115869190616d67565b908060018154018082558091505060019003905f5260205f2090600202015f9091909190915081816115b89190616f9a565b50508080600101915050611543565b5083856003015f8d81526020019081526020015f20819055508a85600801819055505f61168c86600e015f8e81526020019081526020015f20805461160b9061676b565b80601f01602080910402602001604051908101604052809291908181526020018280546116379061676b565b80156116825780601f1061165957610100808354040283529160200191611682565b820191905f5260205f20905b81548152906001019060200180831161166557829003601f168201915b5050505050613eac565b90505f611720828480548060200260200160405190810160405280929190818152602001828054801561171157602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116116c8575b50505050508580549050613f4c565b90507feb85c26dbcad46b80a68a0f24cce7c2c90f0a1faded84184138839fc9e80a25b8d828e8e6040516117579493929190616fa8565b60405180910390a150505b5050505050505050505050565b61177761408f565b61178082614175565b61178a8282614268565b5050565b5f611797614386565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b5f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa15801561181d573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118419190616677565b90507344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166346c5bbbd82336040518363ffffffff1660e01b8152600401611892929190616a51565b602060405180830381865afa1580156118ad573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118d19190616aa2565b61191257336040517faee86323000000000000000000000000000000000000000000000000000000008152600401611909919061659b565b60405180910390fd5b5f61191b613c15565b90508060040154851180611934575060f86003901b8511155b1561197657846040517f0ab7f68700000000000000000000000000000000000000000000000000000000815260040161196d919061636f565b60405180910390fd5b5f81600e015f8781526020019081526020015f2080546119959061676b565b80601f01602080910402602001604051908101604052809291908181526020018280546119c19061676b565b8015611a0c5780601f106119e357610100808354040283529160200191611a0c565b820191905f5260205f20905b8154815290600101906020018083116119ef57829003601f168201915b505050505090505f611a1e878361440d565b90505f611a2c828888613db5565b9050835f015f8981526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615611acc5787816040517f33ca1fe3000000000000000000000000000000000000000000000000000000008152600401611ac3929190616a51565b60405180910390fd5b6001845f015f8a81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f846002015f8a81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507f4c715c5734ce5c18c9c12e8496e53d2a65f1ec381d476957f0f596b364a59b0c89898933604051611bea9493929190616fed565b60405180910390a1846001015f8a81526020019081526020015f205f9054906101000a900460ff16158015611c285750611c278180549050613e1b565b5b15611cc4576001856001015f8b81526020019081526020015f205f6101000a81548160ff02191690831515021790555082856003015f8b81526020019081526020015f20819055505f856006015f8b81526020019081526020015f205490507f3a116120cca5d4f073cc1fc31ff26133ab7b0499f2712fa010023b87d5a1f9ee8a8287604051611cba9392919061702b565b60405180910390a1505b505050505050505050565b5f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa158015611d2d573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d519190616677565b90507344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166346c5bbbd82336040518363ffffffff1660e01b8152600401611da2929190616a51565b602060405180830381865afa158015611dbd573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611de19190616aa2565b611e2257336040517faee86323000000000000000000000000000000000000000000000000000000008152600401611e19919061659b565b60405180910390fd5b5f611e2b613c15565b90508060090154871180611e44575060f86005901b8711155b15611e8657866040517f8d8c940a000000000000000000000000000000000000000000000000000000008152600401611e7d919061636f565b60405180910390fd5b5f611e938289898961446f565b90505f611ea1828787613db5565b9050825f015f8a81526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615611f415788816040517ffcf5a6e9000000000000000000000000000000000000000000000000000000008152600401611f38929190616a51565b60405180910390fd5b6001835f015f8b81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f836002015f8b81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507f7bf1b42c10e9497c879620c5b7afced10bda17d8c90b22f0e3bc6b2fd6ced0bd8a8a8a8a8a3360405161206396959493929190617067565b60405180910390a1836001015f8b81526020019081526020015f205f9054906101000a900460ff161580156120a157506120a08180549050613e1b565b5b1561228e576001846001015f8c81526020019081526020015f205f6101000a81548160ff021916908315150217905550888885600b015f8d81526020019081526020015f2091826120f3929190616e79565b5082846003015f8c81526020019081526020015f20819055508984600c01819055505f6121b885600e015f8d81526020019081526020015f2080546121379061676b565b80601f01602080910402602001604051908101604052809291908181526020018280546121639061676b565b80156121ae5780601f10612185576101008083540402835291602001916121ae565b820191905f5260205f20905b81548152906001019060200180831161219157829003601f168201915b5050505050613eac565b90505f61224c828480548060200260200160405190810160405280929190818152602001828054801561223d57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116121f4575b50505050508580549050613f4c565b90507f2258b73faed33fb2e2ea454403bef974920caf682ab3a723484fcf67553b16a28c828d8d60405161228394939291906170bc565b60405180910390a150505b50505050505050505050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156122f7573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061231b9190616570565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461238a57336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401612381919061659b565b60405180910390fd5b60025f612395613caa565b9050805f0160089054906101000a900460ff16806123dd57508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15612414576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055506124cd6040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506040518060400160405280600181526020017f3100000000000000000000000000000000000000000000000000000000000000815250613cd1565b5f6124d6613c15565b905060f86003901b845f013510156124ec575f80fd5b60f86004901b84602001351015612501575f80fd5b60f86005901b84604001351015612516575f80fd5b835f01358160040181905550836020013581600501819055508360400135816009018190555083606001358160080181905550836080013581600c01819055505f8460a001351415801561256e57505f8460c0013514155b156125b4578360c00135816006015f8660a0013581526020019081526020015f20819055508360a00135816006015f8660c0013581526020019081526020015f20819055505b5f846060013514612667575f5b848060e001906125d19190617101565b905081101561266557816007015f866060013581526020019081526020015f20858060e001906126019190617101565b8381811061261257612611616d2e565b5b90506020028101906126249190616d67565b908060018154018082558091505060019003905f5260205f2090600202015f9091909190915081816126569190616f9a565b505080806001019150506125c1565b505b5f84608001351415801561268c57505f848061010001906126889190616e0d565b9050115b156126e957838061010001906126a29190616e0d565b82600b015f876080013581526020019081526020015f2091826126c6929190616e79565b5083610240013581600a015f866080013581526020019081526020015f20819055505b612775818560600135868061012001906127039190617163565b88610140013589806102c0019061271a9190616e0d565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050508a61020001602081019061277091906171d9565b61453d565b6128018185608001358680610160019061278f9190617163565b88610180013589806102e001906127a69190616e0d565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050508a6102200160208101906127fc91906171d9565b61453d565b61288d818560a0013586806101a0019061281b9190617163565b886101c0013589806102a001906128329190616e0d565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050508a6101e001602081019061288891906171d9565b61453d565b5f8460a00135146128da57836101e00160208101906128ac91906171d9565b816001015f8660a0013581526020019081526020015f205f6101000a81548160ff0219169083151502179055505b5f84606001351461292757836102000160208101906128f991906171d9565b816001015f866060013581526020019081526020015f205f6101000a81548160ff0219169083151502179055505b5f846080013514612974578361022001602081019061294691906171d9565b816001015f866080013581526020019081526020015f205f6101000a81548160ff0219169083151502179055505b5f8460a00135146129d157836102600160208101906129939190616405565b81600d015f8660a0013581526020019081526020015f205f6101000a81548160ff021916908360018111156129cb576129ca61597d565b5b02179055505b5f846060013514612a2e57836102600160208101906129f09190616405565b81600d015f866060013581526020019081526020015f205f6101000a81548160ff02191690836001811115612a2857612a2761597d565b5b02179055505b5f846080013514612a8b5783610280016020810190612a4d9190616405565b81600d015f866080013581526020019081526020015f205f6101000a81548160ff02191690836001811115612a8557612a8461597d565b5b02179055505b5f8460a0013514158015612ab057505f84806102a00190612aac9190616e0d565b9050115b15612aec5783806102a00190612ac69190616e0d565b82600e015f8760a0013581526020019081526020015f209182612aea929190616e79565b505b5f846060013514158015612b1157505f84806102c00190612b0d9190616e0d565b9050115b15612b4d5783806102c00190612b279190616e0d565b82600e015f876060013581526020019081526020015f209182612b4b929190616e79565b505b5f846080013514158015612b7257505f84806102e00190612b6e9190616e0d565b9050115b15612bae5783806102e00190612b889190616e0d565b82600e015f876080013581526020019081526020015f209182612bac929190616e79565b505b505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051612bf991906165d6565b60405180910390a1505050565b5f80612c10613c15565b90505f8160050154905060f86004901b8114158015612c4c5750816001015f8281526020019081526020015f205f9054906101000a900460ff16155b15612c5c57600192505050612cae565b5f8260090154905060f86005901b8114158015612c965750826001015f8281526020019081526020015f205f9054906101000a900460ff16155b15612ca75760019350505050612cae565b5f93505050505b90565b5f6060805f805f60605f612cc3614780565b90505f801b815f0154148015612cde57505f801b8160010154145b612d1d576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612d149061724e565b60405180910390fd5b612d256147a7565b612d2d614845565b46305f801b5f67ffffffffffffffff811115612d4c57612d4b615bdf565b5b604051908082528060200260200182016040528015612d7a5781602001602082028036833780820191505090505b507f0f0000000000000000000000000000000000000000000000000000000000000095949392919097509750975097509750975097505090919293949596565b6060805f612dc6613c15565b9050806001015f8581526020019081526020015f205f9054906101000a900460ff16612e2957836040517f84de1331000000000000000000000000000000000000000000000000000000008152600401612e20919061636f565b60405180910390fd5b80600f015f8581526020019081526020015f205f9054906101000a900460ff1615612e8b57836040517f84de1331000000000000000000000000000000000000000000000000000000008152600401612e82919061636f565b60405180910390fd5b5f816003015f8681526020019081526020015f205490505f826002015f8781526020019081526020015f205f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020018280548015612f4257602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311612ef9575b505050505090505f815190505f8103612f9257866040517f84de1331000000000000000000000000000000000000000000000000000000008152600401612f89919061636f565b60405180910390fd5b5f61303585600e015f8a81526020019081526020015f208054612fb49061676b565b80601f0160208091040260200160405190810160405280929190818152602001828054612fe09061676b565b801561302b5780601f106130025761010080835404028352916020019161302b565b820191905f5260205f20905b81548152906001019060200180831161300e57829003601f168201915b5050505050613eac565b90505f613043828585613f4c565b905080866007015f8b81526020019081526020015f2080805480602002602001604051908101604052809291908181526020015f905b82821015613172578382905f5260205f2090600202016040518060400160405290815f82015f9054906101000a900460ff1660018111156130bd576130bc61597d565b5b60018111156130cf576130ce61597d565b5b81526020016001820180546130e39061676b565b80601f016020809104026020016040519081016040528092919081815260200182805461310f9061676b565b801561315a5780601f106131315761010080835404028352916020019161315a565b820191905f5260205f20905b81548152906001019060200180831161313d57829003601f168201915b50505050508152505081526020019060010190613079565b50505050905097509750505050505050915091565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b5f806131ca613c15565b905080600c015491505090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613234573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906132589190616570565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146132c757336040517f21bfda100000000000000000000000000000000000000000000000000000000081526004016132be919061659b565b60405180910390fd5b5f6132d0613c15565b905080600501548211806132e9575060f86004901b8211155b1561332b57816040517ffcf2db7a000000000000000000000000000000000000000000000000000000008152600401613322919061636f565b60405180910390fd5b806001015f8381526020019081526020015f205f9054906101000a900460ff161561338d57816040517f92789b67000000000000000000000000000000000000000000000000000000008152600401613384919061636f565b60405180910390fd5b6001816001015f8481526020019081526020015f205f6101000a81548160ff021916908315150217905550600181600f015f8481526020019081526020015f205f6101000a81548160ff0219169083151502179055505f816006015f8481526020019081526020015f205490505f811461342d576001826001015f8381526020019081526020015f205f6101000a81548160ff0219169083151502179055505b7f2b087b884b35a81d769d1a1e092880f1da56de964e4b339eabcb1f45f5fe32648360405161345c919061636f565b60405180910390a1505050565b6060805f613475613c15565b9050806001015f8581526020019081526020015f205f9054906101000a900460ff166134d857836040517fda32d00f0000000000000000000000000000000000000000000000000000000081526004016134cf919061636f565b60405180910390fd5b80600f015f8581526020019081526020015f205f9054906101000a900460ff161561353a57836040517fda32d00f000000000000000000000000000000000000000000000000000000008152600401613531919061636f565b60405180910390fd5b5f816003015f8681526020019081526020015f205490505f826002015f8781526020019081526020015f205f8381526020019081526020015f208054806020026020016040519081016040528092919081815260200182805480156135f157602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116135a8575b505050505090505f815190505f810361364157866040517fda32d00f000000000000000000000000000000000000000000000000000000008152600401613638919061636f565b60405180910390fd5b5f6136e485600e015f8a81526020019081526020015f2080546136639061676b565b80601f016020809104026020016040519081016040528092919081815260200182805461368f9061676b565b80156136da5780601f106136b1576101008083540402835291602001916136da565b820191905f5260205f20905b8154815290600101906020018083116136bd57829003601f168201915b5050505050613eac565b90505f6136f2828585613f4c565b90508086600b015f8b81526020019081526020015f208080546137149061676b565b80601f01602080910402602001604051908101604052809291908181526020018280546137409061676b565b801561378b5780601f106137625761010080835404028352916020019161378b565b820191905f5260205f20905b81548152906001019060200180831161376e57829003601f168201915b5050505050905097509750505050505050915091565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156137fe573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906138229190616570565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461389157336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401613888919061659b565b60405180910390fd5b5f61389a613c15565b90505f8160050154905060f86004901b81141580156138d65750816001015f8281526020019081526020015f205f9054906101000a900460ff16155b1561391857806040517f3b853da800000000000000000000000000000000000000000000000000000000815260040161390f919061636f565b60405180910390fd5b816004015f81548092919061392c9061661c565b91905055505f82600401549050826005015f81548092919061394d9061661c565b91905055505f8360050154905080846006015f8481526020019081526020015f208190555081846006015f8381526020019081526020015f20819055505f8585600d015f8581526020019081526020015f205f6101000a81548160ff021916908360018111156139c0576139bf61597d565b5b02179055508585600d015f8481526020019081526020015f205f6101000a81548160ff021916908360018111156139fa576139f961597d565b5b02179055505f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa158015613a5d573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613a819190616677565b90505f60028284604051602001613a9a93929190616702565b60405160208183030381529060405290508087600e015f8781526020019081526020015f209081613acb9190616938565b508087600e015f8681526020019081526020015f209081613aec9190616938565b507fc0ada6fd1b65690a546d49df7531a21c52ed502d14efd98ee0c8f4846abb931085848a84604051613b229493929190616a07565b60405180910390a15050505050505050565b5f80613b3e613c15565b9050806008015491505090565b60605f6001613b59846148e3565b0190505f8167ffffffffffffffff811115613b7757613b76615bdf565b5b6040519080825280601f01601f191660200182016040528015613ba95781602001600182028036833780820191505090505b5090505f82602001820190505b600115613c0a578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a8581613bff57613bfe61726c565b5b0494505f8503613bb6575b819350505050919050565b5f7f26fdaf8a2cb20d20b55e36218986905e534ee7a970dd2fa827946e4b7496db00905090565b5f80836003015f8481526020019081526020015f205490505f846002015f8581526020019081526020015f205f8381526020019081526020015f2080549050141591505092915050565b5f613c8f613caa565b5f015f9054906101000a900467ffffffffffffffff16905090565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b613cd9614a34565b613ce38282614a74565b5050565b5f80856006015f8681526020019081526020015f205490505f86600e015f8781526020019081526020015f208054613d1e9061676b565b80601f0160208091040260200160405190810160405280929190818152602001828054613d4a9061676b565b8015613d955780601f10613d6c57610100808354040283529160200191613d95565b820191905f5260205f20905b815481529060010190602001808311613d7857829003601f168201915b50505050509050613da98287878785614ac5565b92505050949350505050565b5f80613e048585858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050614ca6565b9050613e108133614cd0565b809150509392505050565b5f807344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663b4722bc46040518163ffffffff1660e01b8152600401602060405180830381865afa158015613e7a573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613e9e9190616677565b905080831015915050919050565b5f602182511015613f3f577344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa158015613f14573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613f389190616677565b9050613f47565b602182015190505b919050565b60605f8267ffffffffffffffff811115613f6957613f68615bdf565b5b604051908082528060200260200182016040528015613f9c57816020015b6060815260200190600190039081613f875790505b5090505f5b83811015614083577344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166331ff41c887878481518110613fed57613fec616d2e565b5b60200260200101516040518363ffffffff1660e01b8152600401614012929190616a51565b5f60405180830381865afa15801561402c573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f8201168201806040525081019061405491906173ec565b6060015182828151811061406b5761406a616d2e565b5b60200260200101819052508080600101915050613fa1565b50809150509392505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff16148061413c57507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16614123614f35565b73ffffffffffffffffffffffffffffffffffffffff1614155b15614173576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156141d2573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906141f69190616570565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461426557336040517f21bfda1000000000000000000000000000000000000000000000000000000000815260040161425c919061659b565b60405180910390fd5b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa9250505080156142d057506040513d601f19601f820116820180604052508101906142cd919061745d565b60015b61431157816040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401614308919061659b565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b811461437757806040517faa1d49a400000000000000000000000000000000000000000000000000000000815260040161436e9190615d75565b60405180910390fd5b6143818383614f88565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff161461440b576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f6144676040518060600160405280603c81526020016178ed603c91398051906020012084848051906020012060405160200161444c93929190617488565b60405160208183030381529060405280519060200120614ffa565b905092915050565b5f8085600a015f8681526020019081526020015f205490505f86600e015f8781526020019081526020015f2080546144a69061676b565b80601f01602080910402602001604051908101604052809291908181526020018280546144d29061676b565b801561451d5780601f106144f45761010080835404028352916020019161451d565b820191905f5260205f20905b81548152906001019060200180831161450057829003601f168201915b505050505090506145318683878785615013565b92505050949350505050565b5f86148061454d57505f85859050145b6147775780156145705782876003015f8881526020019081526020015f20819055505b5f61457a83613eac565b90505f5b86869050811015614774575f87878381811061459d5761459c616d2e565b5b90506020020160208101906145b291906174bd565b9050896002015f8a81526020019081526020015f205f8781526020019081526020015f2081908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166331ff41c885846040518363ffffffff1660e01b8152600401614684929190616a51565b5f60405180830381865afa15801561469e573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f820116820180604052508101906146c691906173ec565b6020015190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16146147655760018b5f015f8c81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505b5050808060010191505061457e565b50505b50505050505050565b5f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100905090565b60605f6147b2614780565b90508060020180546147c39061676b565b80601f01602080910402602001604051908101604052809291908181526020018280546147ef9061676b565b801561483a5780601f106148115761010080835404028352916020019161483a565b820191905f5260205f20905b81548152906001019060200180831161481d57829003601f168201915b505050505091505090565b60605f614850614780565b90508060030180546148619061676b565b80601f016020809104026020016040519081016040528092919081815260200182805461488d9061676b565b80156148d85780601f106148af576101008083540402835291602001916148d8565b820191905f5260205f20905b8154815290600101906020018083116148bb57829003601f168201915b505050505091505090565b5f805f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f010000000000000000831061493f577a184f03e93ff9f4daa797ed6e38ed64bf6a1f01000000000000000083816149355761493461726c565b5b0492506040810190505b6d04ee2d6d415b85acef8100000000831061497c576d04ee2d6d415b85acef810000000083816149725761497161726c565b5b0492506020810190505b662386f26fc1000083106149ab57662386f26fc1000083816149a1576149a061726c565b5b0492506010810190505b6305f5e10083106149d4576305f5e10083816149ca576149c961726c565b5b0492506008810190505b61271083106149f95761271083816149ef576149ee61726c565b5b0492506004810190505b60648310614a1c5760648381614a1257614a1161726c565b5b0492506002810190505b600a8310614a2b576001810190505b80915050919050565b614a3c6150a4565b614a72576040517fd7e6bcf800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b614a7c614a34565b5f614a85614780565b905082816002019081614a989190617540565b5081816003019081614aaa9190617540565b505f801b815f01819055505f801b8160010181905550505050565b5f808484905067ffffffffffffffff811115614ae457614ae3615bdf565b5b604051908082528060200260200182016040528015614b125781602001602082028036833780820191505090505b5090505f5b85859050811015614c1657604051806060016040528060258152602001617a016025913980519060200120868683818110614b5557614b54616d2e565b5b9050602002810190614b679190616d67565b5f016020810190614b78919061760f565b878784818110614b8b57614b8a616d2e565b5b9050602002810190614b9d9190616d67565b8060200190614bac9190616e0d565b604051614bba929190617668565b6040518091039020604051602001614bd49392919061768f565b60405160208183030381529060405280519060200120828281518110614bfd57614bfc616d2e565b5b6020026020010181815250508080600101915050614b17565b50614c9a6040518060c001604052806082815260200161797f6082913980519060200120888884604051602001614c4d9190617775565b604051602081830303815290604052805190602001208780519060200120604051602001614c7f95949392919061778b565b60405160208183030381529060405280519060200120614ffa565b91505095945050505050565b5f805f80614cb486866150c2565b925092509250614cc48282615117565b82935050505092915050565b5f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa158015614d2e573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614d529190616677565b90507344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff16639447cfd482856040518363ffffffff1660e01b8152600401614da3929190616a51565b602060405180830381865afa158015614dbe573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614de29190616aa2565b614e255782826040517f0d86f521000000000000000000000000000000000000000000000000000000008152600401614e1c9291906177dc565b60405180910390fd5b5f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166331ff41c883856040518363ffffffff1660e01b8152600401614e75929190616a51565b5f60405180830381865afa158015614e8f573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f82011682018060405250810190614eb791906173ec565b90508373ffffffffffffffffffffffffffffffffffffffff16816020015173ffffffffffffffffffffffffffffffffffffffff1614614f2f5783836040517f0d86f521000000000000000000000000000000000000000000000000000000008152600401614f269291906177dc565b60405180910390fd5b50505050565b5f614f617f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b615279565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b614f9182615282565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f81511115614fed57614fe7828261534b565b50614ff6565b614ff56153cb565b5b5050565b5f61500c615006615407565b83615415565b9050919050565b5f61509960405180608001604052806056815260200161792960569139805190602001208787878760405160200161504c929190617668565b60405160208183030381529060405280519060200120868051906020012060405160200161507e95949392919061778b565b60405160208183030381529060405280519060200120614ffa565b905095945050505050565b5f6150ad613caa565b5f0160089054906101000a900460ff16905090565b5f805f6041845103615102575f805f602087015192506040870151915060608701515f1a90506150f488828585615455565b955095509550505050615110565b5f600285515f1b9250925092505b9250925092565b5f600381111561512a5761512961597d565b5b82600381111561513d5761513c61597d565b5b031561527557600160038111156151575761515661597d565b5b82600381111561516a5761516961597d565b5b036151a1576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600260038111156151b5576151b461597d565b5b8260038111156151c8576151c761597d565b5b0361520c57805f1c6040517ffce698f7000000000000000000000000000000000000000000000000000000008152600401615203919061636f565b60405180910390fd5b60038081111561521f5761521e61597d565b5b8260038111156152325761523161597d565b5b0361527457806040517fd78bce0c00000000000000000000000000000000000000000000000000000000815260040161526b9190615d75565b60405180910390fd5b5b5050565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b036152dd57806040517f4c9c8ce30000000000000000000000000000000000000000000000000000000081526004016152d4919061659b565b60405180910390fd5b806153097f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b615279565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f808473ffffffffffffffffffffffffffffffffffffffff16846040516153749190617833565b5f60405180830381855af49150503d805f81146153ac576040519150601f19603f3d011682016040523d82523d5f602084013e6153b1565b606091505b50915091506153c185838361553c565b9250505092915050565b5f341115615405576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f6154106155c9565b905090565b5f6040517f190100000000000000000000000000000000000000000000000000000000000081528360028201528260228201526042812091505092915050565b5f805f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c1115615491575f600385925092509250615532565b5f6001888888886040515f81526020016040526040516154b49493929190617858565b6020604051602081039080840390855afa1580156154d4573d5f803e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603615525575f60015f801b93509350935050615532565b805f805f1b935093509350505b9450945094915050565b6060826155515761554c8261562c565b6155c1565b5f825114801561557757505f8473ffffffffffffffffffffffffffffffffffffffff163b145b156155b957836040517f9996b3150000000000000000000000000000000000000000000000000000000081526004016155b0919061659b565b60405180910390fd5b8190506155c2565b5b9392505050565b5f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f6155f3615670565b6155fb6156e6565b463060405160200161561195949392919061789b565b60405160208183030381529060405280519060200120905090565b5f8151111561563e5780518082602001fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f8061567a614780565b90505f6156856147a7565b90505f815111156156a1578080519060200120925050506156e3565b5f825f015490505f801b81146156bc578093505050506156e3565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f806156f0614780565b90505f6156fb614845565b90505f815111156157175780805190602001209250505061575a565b5f826001015490505f801b81146157335780935050505061575a565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f81519050919050565b5f82825260208201905092915050565b5f5b83811015615794578082015181840152602081019050615779565b5f8484015250505050565b5f601f19601f8301169050919050565b5f6157b98261575d565b6157c38185615767565b93506157d3818560208601615777565b6157dc8161579f565b840191505092915050565b5f6020820190508181035f8301526157ff81846157af565b905092915050565b5f604051905090565b5f80fd5b5f80fd5b5f819050919050565b61582a81615818565b8114615834575f80fd5b50565b5f8135905061584581615821565b92915050565b5f602082840312156158605761585f615810565b5b5f61586d84828501615837565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6158c88261589f565b9050919050565b6158d8816158be565b82525050565b5f6158e983836158cf565b60208301905092915050565b5f602082019050919050565b5f61590b82615876565b6159158185615880565b935061592083615890565b805f5b8381101561595057815161593788826158de565b9750615942836158f5565b925050600181019050615923565b5085935050505092915050565b5f6020820190508181035f8301526159758184615901565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b600281106159bb576159ba61597d565b5b50565b5f8190506159cb826159aa565b919050565b5f6159da826159be565b9050919050565b6159ea816159d0565b82525050565b5f602082019050615a035f8301846159e1565b92915050565b60028110615a15575f80fd5b50565b5f81359050615a2681615a09565b92915050565b5f8060408385031215615a4257615a41615810565b5b5f615a4f85828601615837565b9250506020615a6085828601615a18565b9150509250929050565b5f80fd5b5f80fd5b5f80fd5b5f8083601f840112615a8b57615a8a615a6a565b5b8235905067ffffffffffffffff811115615aa857615aa7615a6e565b5b602083019150836020820283011115615ac457615ac3615a72565b5b9250929050565b5f8083601f840112615ae057615adf615a6a565b5b8235905067ffffffffffffffff811115615afd57615afc615a6e565b5b602083019150836001820283011115615b1957615b18615a72565b5b9250929050565b5f805f805f60608688031215615b3957615b38615810565b5b5f615b4688828901615837565b955050602086013567ffffffffffffffff811115615b6757615b66615814565b5b615b7388828901615a76565b9450945050604086013567ffffffffffffffff811115615b9657615b95615814565b5b615ba288828901615acb565b92509250509295509295909350565b615bba816158be565b8114615bc4575f80fd5b50565b5f81359050615bd581615bb1565b92915050565b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b615c158261579f565b810181811067ffffffffffffffff82111715615c3457615c33615bdf565b5b80604052505050565b5f615c46615807565b9050615c528282615c0c565b919050565b5f67ffffffffffffffff821115615c7157615c70615bdf565b5b615c7a8261579f565b9050602081019050919050565b828183375f83830152505050565b5f615ca7615ca284615c57565b615c3d565b905082815260208101848484011115615cc357615cc2615bdb565b5b615cce848285615c87565b509392505050565b5f82601f830112615cea57615ce9615a6a565b5b8135615cfa848260208601615c95565b91505092915050565b5f8060408385031215615d1957615d18615810565b5b5f615d2685828601615bc7565b925050602083013567ffffffffffffffff811115615d4757615d46615814565b5b615d5385828601615cd6565b9150509250929050565b5f819050919050565b615d6f81615d5d565b82525050565b5f602082019050615d885f830184615d66565b92915050565b5f805f60408486031215615da557615da4615810565b5b5f615db286828701615837565b935050602084013567ffffffffffffffff811115615dd357615dd2615814565b5b615ddf86828701615acb565b92509250509250925092565b5f805f805f60608688031215615e0457615e03615810565b5b5f615e1188828901615837565b955050602086013567ffffffffffffffff811115615e3257615e31615814565b5b615e3e88828901615acb565b9450945050604086013567ffffffffffffffff811115615e6157615e60615814565b5b615e6d88828901615acb565b92509250509295509295909350565b5f80fd5b5f6103008284031215615e9657615e95615e7c565b5b81905092915050565b5f60208284031215615eb457615eb3615810565b5b5f82013567ffffffffffffffff811115615ed157615ed0615814565b5b615edd84828501615e80565b91505092915050565b5f8115159050919050565b615efa81615ee6565b82525050565b5f602082019050615f135f830184615ef1565b92915050565b5f7fff0000000000000000000000000000000000000000000000000000000000000082169050919050565b615f4d81615f19565b82525050565b615f5c81615818565b82525050565b615f6b816158be565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b615fa381615818565b82525050565b5f615fb48383615f9a565b60208301905092915050565b5f602082019050919050565b5f615fd682615f71565b615fe08185615f7b565b9350615feb83615f8b565b805f5b8381101561601b5781516160028882615fa9565b975061600d83615fc0565b925050600181019050615fee565b5085935050505092915050565b5f60e08201905061603b5f83018a615f44565b818103602083015261604d81896157af565b9050818103604083015261606181886157af565b90506160706060830187615f53565b61607d6080830186615f62565b61608a60a0830185615d66565b81810360c083015261609c8184615fcc565b905098975050505050505050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f82825260208201905092915050565b5f6160ed8261575d565b6160f781856160d3565b9350616107818560208601615777565b6161108161579f565b840191505092915050565b5f61612683836160e3565b905092915050565b5f602082019050919050565b5f616144826160aa565b61614e81856160b4565b935083602082028501616160856160c4565b805f5b8581101561619b578484038952815161617c858261611b565b94506161878361612e565b925060208a01995050600181019050616163565b50829750879550505050505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b600281106161e7576161e661597d565b5b50565b5f8190506161f7826161d6565b919050565b5f616206826161ea565b9050919050565b616216816161fc565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f6162408261621c565b61624a8185616226565b935061625a818560208601615777565b6162638161579f565b840191505092915050565b5f604083015f8301516162835f86018261620d565b506020830151848203602086015261629b8282616236565b9150508091505092915050565b5f6162b3838361626e565b905092915050565b5f602082019050919050565b5f6162d1826161ad565b6162db81856161b7565b9350836020820285016162ed856161c7565b805f5b85811015616328578484038952815161630985826162a8565b9450616314836162bb565b925060208a019950506001810190506162f0565b50829750879550505050505092915050565b5f6040820190508181035f830152616352818561613a565b9050818103602083015261636681846162c7565b90509392505050565b5f6020820190506163825f830184615f53565b92915050565b5f82825260208201905092915050565b5f6163a28261621c565b6163ac8185616388565b93506163bc818560208601615777565b6163c58161579f565b840191505092915050565b5f6040820190508181035f8301526163e8818561613a565b905081810360208301526163fc8184616398565b90509392505050565b5f6020828403121561641a57616419615810565b5b5f61642784828501615a18565b91505092915050565b5f81905092915050565b5f6164448261575d565b61644e8185616430565b935061645e818560208601615777565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f61649e600283616430565b91506164a98261646a565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f6164e8600183616430565b91506164f3826164b4565b600182019050919050565b5f616509828761643a565b915061651482616492565b9150616520828661643a565b915061652b826164dc565b9150616537828561643a565b9150616542826164dc565b915061654e828461643a565b915081905095945050505050565b5f8151905061656a81615bb1565b92915050565b5f6020828403121561658557616584615810565b5b5f6165928482850161655c565b91505092915050565b5f6020820190506165ae5f830184615f62565b92915050565b5f67ffffffffffffffff82169050919050565b6165d0816165b4565b82525050565b5f6020820190506165e95f8301846165c7565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61662682615818565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203616658576166576165ef565b5b600182019050919050565b5f8151905061667181615821565b92915050565b5f6020828403121561668c5761668b615810565b5b5f61669984828501616663565b91505092915050565b5f60ff82169050919050565b5f8160f81b9050919050565b5f6166c4826166ae565b9050919050565b6166dc6166d7826166a2565b6166ba565b82525050565b5f819050919050565b6166fc6166f782615818565b6166e2565b82525050565b5f61670d82866166cb565b60018201915061671d82856166eb565b60208201915061672d82846166eb565b602082019150819050949350505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f600282049050600182168061678257607f821691505b6020821081036167955761679461673e565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f600883026167f77fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff826167bc565b61680186836167bc565b95508019841693508086168417925050509392505050565b5f819050919050565b5f61683c61683761683284615818565b616819565b615818565b9050919050565b5f819050919050565b61685583616822565b61686961686182616843565b8484546167c8565b825550505050565b5f90565b61687d616871565b61688881848461684c565b505050565b5b818110156168ab576168a05f82616875565b60018101905061688e565b5050565b601f8211156168f0576168c18161679b565b6168ca846167ad565b810160208510156168d9578190505b6168ed6168e5856167ad565b83018261688d565b50505b505050565b5f82821c905092915050565b5f6169105f19846008026168f5565b1980831691505092915050565b5f6169288383616901565b9150826002028217905092915050565b6169418261621c565b67ffffffffffffffff81111561695a57616959615bdf565b5b616964825461676b565b61696f8282856168af565b5f60209050601f8311600181146169a0575f841561698e578287015190505b616998858261691d565b8655506169ff565b601f1984166169ae8661679b565b5f5b828110156169d5578489015182556001820191506020850194506020810190506169b0565b868310156169f257848901516169ee601f891682616901565b8355505b6001600288020188555050505b505050505050565b5f608082019050616a1a5f830187615f53565b616a276020830186615f53565b616a3460408301856159e1565b8181036060830152616a468184616398565b905095945050505050565b5f604082019050616a645f830185615f53565b616a716020830184615f62565b9392505050565b616a8181615ee6565b8114616a8b575f80fd5b50565b5f81519050616a9c81616a78565b92915050565b5f60208284031215616ab757616ab6615810565b5b5f616ac484828501616a8e565b91505092915050565b5f819050919050565b60028110616ae2575f80fd5b50565b5f81359050616af381616ad6565b92915050565b5f616b076020840184616ae5565b905092915050565b5f80fd5b5f80fd5b5f80fd5b5f8083356001602003843603038112616b3757616b36616b17565b5b83810192508235915060208301925067ffffffffffffffff821115616b5f57616b5e616b0f565b5b600182023603831315616b7557616b74616b13565b5b509250929050565b5f616b888385616226565b9350616b95838584615c87565b616b9e8361579f565b840190509392505050565b5f60408301616bba5f840184616af9565b616bc65f86018261620d565b50616bd46020840184616b1b565b8583036020870152616be7838284616b7d565b925050508091505092915050565b5f616c008383616ba9565b905092915050565b5f82356001604003833603038112616c2357616c22616b17565b5b82810191505092915050565b5f602082019050919050565b5f616c4683856161b7565b935083602084028501616c5884616acd565b805f5b87811015616c9b578484038952616c728284616c08565b616c7c8582616bf5565b9450616c8783616c2f565b925060208a01995050600181019050616c5b565b50829750879450505050509392505050565b5f616cb88385616388565b9350616cc5838584615c87565b616cce8361579f565b840190509392505050565b5f608082019050616cec5f830189615f53565b8181036020830152616cff818789616c3b565b90508181036040830152616d14818587616cad565b9050616d236060830184615f62565b979650505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b5f82356001604003833603038112616d8257616d81616d5b565b5b80830191505092915050565b5f8135616d9a81616ad6565b80915050919050565b5f815f1b9050919050565b5f60ff616dba84616da3565b9350801983169250808416831791505092915050565b5f616dda826161ea565b9050919050565b5f819050919050565b616df382616dd0565b616e06616dff82616de1565b8354616dae565b8255505050565b5f8083356001602003843603038112616e2957616e28616d5b565b5b80840192508235915067ffffffffffffffff821115616e4b57616e4a616d5f565b5b602083019250600182023603831315616e6757616e66616d63565b5b509250929050565b5f82905092915050565b616e838383616e6f565b67ffffffffffffffff811115616e9c57616e9b615bdf565b5b616ea6825461676b565b616eb18282856168af565b5f601f831160018114616ede575f8415616ecc578287013590505b616ed6858261691d565b865550616f3d565b601f198416616eec8661679b565b5f5b82811015616f1357848901358255600182019150602085019450602081019050616eee565b86831015616f305784890135616f2c601f891682616901565b8355505b6001600288020188555050505b50505050505050565b616f51838383616e79565b505050565b5f81015f830180616f6681616d8e565b9050616f728184616dea565b5050506001810160208301616f878185616e0d565b616f92818386616f46565b505050505050565b616fa48282616f56565b5050565b5f606082019050616fbb5f830187615f53565b8181036020830152616fcd818661613a565b90508181036040830152616fe2818486616c3b565b905095945050505050565b5f6060820190506170005f830187615f53565b8181036020830152617013818587616cad565b90506170226040830184615f62565b95945050505050565b5f60608201905061703e5f830186615f53565b61704b6020830185615f53565b818103604083015261705d8184616398565b9050949350505050565b5f60808201905061707a5f830189615f53565b818103602083015261708d818789616cad565b905081810360408301526170a2818587616cad565b90506170b16060830184615f62565b979650505050505050565b5f6060820190506170cf5f830187615f53565b81810360208301526170e1818661613a565b905081810360408301526170f6818486616cad565b905095945050505050565b5f808335600160200384360303811261711d5761711c616d5b565b5b80840192508235915067ffffffffffffffff82111561713f5761713e616d5f565b5b60208301925060208202360383131561715b5761715a616d63565b5b509250929050565b5f808335600160200384360303811261717f5761717e616d5b565b5b80840192508235915067ffffffffffffffff8211156171a1576171a0616d5f565b5b6020830192506020820236038313156171bd576171bc616d63565b5b509250929050565b5f813590506171d381616a78565b92915050565b5f602082840312156171ee576171ed615810565b5b5f6171fb848285016171c5565b91505092915050565b7f4549503731323a20556e696e697469616c697a656400000000000000000000005f82015250565b5f617238601583615767565b915061724382617204565b602082019050919050565b5f6020820190508181035f8301526172658161722c565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f80fd5b5f80fd5b5f67ffffffffffffffff8211156172bb576172ba615bdf565b5b6172c48261579f565b9050602081019050919050565b5f6172e36172de846172a1565b615c3d565b9050828152602081018484840111156172ff576172fe615bdb565b5b61730a848285615777565b509392505050565b5f82601f83011261732657617325615a6a565b5b81516173368482602086016172d1565b91505092915050565b5f6080828403121561735457617353617299565b5b61735e6080615c3d565b90505f61736d8482850161655c565b5f8301525060206173808482850161655c565b602083015250604082015167ffffffffffffffff8111156173a4576173a361729d565b5b6173b084828501617312565b604083015250606082015167ffffffffffffffff8111156173d4576173d361729d565b5b6173e084828501617312565b60608301525092915050565b5f6020828403121561740157617400615810565b5b5f82015167ffffffffffffffff81111561741e5761741d615814565b5b61742a8482850161733f565b91505092915050565b61743c81615d5d565b8114617446575f80fd5b50565b5f8151905061745781617433565b92915050565b5f6020828403121561747257617471615810565b5b5f61747f84828501617449565b91505092915050565b5f60608201905061749b5f830186615d66565b6174a86020830185615f53565b6174b56040830184615d66565b949350505050565b5f602082840312156174d2576174d1615810565b5b5f6174df84828501615bc7565b91505092915050565b5f819050815f5260205f209050919050565b601f82111561753b5761750c816174e8565b617515846167ad565b81016020851015617524578190505b617538617530856167ad565b83018261688d565b50505b505050565b6175498261575d565b67ffffffffffffffff81111561756257617561615bdf565b5b61756c825461676b565b6175778282856174fa565b5f60209050601f8311600181146175a8575f8415617596578287015190505b6175a0858261691d565b865550617607565b601f1984166175b6866174e8565b5f5b828110156175dd578489015182556001820191506020850194506020810190506175b8565b868310156175fa57848901516175f6601f891682616901565b8355505b6001600288020188555050505b505050505050565b5f6020828403121561762457617623615810565b5b5f61763184828501616ae5565b91505092915050565b5f81905092915050565b5f61764f838561763a565b935061765c838584615c87565b82840190509392505050565b5f617674828486617644565b91508190509392505050565b617689816161fc565b82525050565b5f6060820190506176a25f830186615d66565b6176af6020830185617680565b6176bc6040830184615d66565b949350505050565b5f81519050919050565b5f81905092915050565b5f819050602082019050919050565b6176f081615d5d565b82525050565b5f61770183836176e7565b60208301905092915050565b5f602082019050919050565b5f617723826176c4565b61772d81856176ce565b9350617738836176d8565b805f5b8381101561776857815161774f88826176f6565b975061775a8361770d565b92505060018101905061773b565b5085935050505092915050565b5f6177808284617719565b915081905092915050565b5f60a08201905061779e5f830188615d66565b6177ab6020830187615f53565b6177b86040830186615f53565b6177c56060830185615d66565b6177d26080830184615d66565b9695505050505050565b5f6040820190506177ef5f830185615f62565b6177fc6020830184615f62565b9392505050565b5f61780d8261621c565b617817818561763a565b9350617827818560208601615777565b80840191505092915050565b5f61783e8284617803565b915081905092915050565b617852816166a2565b82525050565b5f60808201905061786b5f830187615d66565b6178786020830186617849565b6178856040830185615d66565b6178926060830184615d66565b95945050505050565b5f60a0820190506178ae5f830188615d66565b6178bb6020830187615d66565b6178c86040830186615d66565b6178d56060830185615f53565b6178e26080830184615f62565b969550505050505056fe507265704b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e49642c6279746573206578747261446174612943727367656e566572696669636174696f6e2875696e743235362063727349642c75696e74323536206d61784269744c656e6774682c6279746573206372734469676573742c627974657320657874726144617461294b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e49642c75696e74323536206b657949642c4b65794469676573745b5d206b6579446967657374732c627974657320657874726144617461294b65794469676573742875696e7438206b6579547970652c627974657320646967657374294b65794469676573742875696e7438206b6579547970652c62797465732064696765737429
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x014W_5`\xE0\x1C\x80cb\x97\x87\x87\x11a\0\xAAW\x80c\xAD<\xB1\xCC\x11a\0nW\x80c\xAD<\xB1\xCC\x14a\x03\xF9W\x80c\xBA\xFF!\x1E\x14a\x04#W\x80c\xC2\xC1\xFA\xEE\x14a\x04MW\x80c\xC5[\x87$\x14a\x04uW\x80c\xCA\xA3g\xDB\x14a\x04\xB2W\x80c\xD5/\x10\xEB\x14a\x04\xDAWa\x014V[\x80cb\x97\x87\x87\x14a\x03\x12W\x80clh\xE7\x1F\x14a\x03:W\x80cs[\x12c\x14a\x03bW\x80c\x84\xB0\x19n\x14a\x03\x8CW\x80c\x93f\x08\xAE\x14a\x03\xBCWa\x014V[\x80c<\x02\xF84\x11a\0\xFCW\x80c<\x02\xF84\x14a\x02\x18W\x80cE\xAF&\x1B\x14a\x02@W\x80cF\x10\xFF\xE8\x14a\x02|W\x80cO\x1E\xF2\x86\x14a\x02\xA4W\x80cR\xD1\x90-\x14a\x02\xC0W\x80cX\x9A\xDB\x0E\x14a\x02\xEAWa\x014V[\x80c\r\x8En,\x14a\x018W\x80c\x16\xC7\x13\xD9\x14a\x01bW\x80c\x17\x03\xC6\x1A\x14a\x01\x9EW\x80c\x19\xF4\xF62\x14a\x01\xC6W\x80c9\xF78\x10\x14a\x02\x02W[_\x80\xFD[4\x80\x15a\x01CW_\x80\xFD[Pa\x01La\x05\x04V[`@Qa\x01Y\x91\x90aW\xE7V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01mW_\x80\xFD[Pa\x01\x88`\x04\x806\x03\x81\x01\x90a\x01\x83\x91\x90aXKV[a\x05\x7FV[`@Qa\x01\x95\x91\x90aY]V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xA9W_\x80\xFD[Pa\x01\xC4`\x04\x806\x03\x81\x01\x90a\x01\xBF\x91\x90aXKV[a\x06PV[\0[4\x80\x15a\x01\xD1W_\x80\xFD[Pa\x01\xEC`\x04\x806\x03\x81\x01\x90a\x01\xE7\x91\x90aXKV[a\x08\x97V[`@Qa\x01\xF9\x91\x90aY\xF0V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\rW_\x80\xFD[Pa\x02\x16a\n\"V[\0[4\x80\x15a\x02#W_\x80\xFD[Pa\x02>`\x04\x806\x03\x81\x01\x90a\x029\x91\x90aZ,V[a\x0C<V[\0[4\x80\x15a\x02KW_\x80\xFD[Pa\x02f`\x04\x806\x03\x81\x01\x90a\x02a\x91\x90aXKV[a\x0F;V[`@Qa\x02s\x91\x90aY\xF0V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x87W_\x80\xFD[Pa\x02\xA2`\x04\x806\x03\x81\x01\x90a\x02\x9D\x91\x90a[ V[a\x10}V[\0[a\x02\xBE`\x04\x806\x03\x81\x01\x90a\x02\xB9\x91\x90a]\x03V[a\x17oV[\0[4\x80\x15a\x02\xCBW_\x80\xFD[Pa\x02\xD4a\x17\x8EV[`@Qa\x02\xE1\x91\x90a]uV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xF5W_\x80\xFD[Pa\x03\x10`\x04\x806\x03\x81\x01\x90a\x03\x0B\x91\x90a]\x8EV[a\x17\xBFV[\0[4\x80\x15a\x03\x1DW_\x80\xFD[Pa\x038`\x04\x806\x03\x81\x01\x90a\x033\x91\x90a]\xEBV[a\x1C\xCFV[\0[4\x80\x15a\x03EW_\x80\xFD[Pa\x03``\x04\x806\x03\x81\x01\x90a\x03[\x91\x90a^\x9FV[a\"\x9AV[\0[4\x80\x15a\x03mW_\x80\xFD[Pa\x03va,\x06V[`@Qa\x03\x83\x91\x90a_\0V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x97W_\x80\xFD[Pa\x03\xA0a,\xB1V[`@Qa\x03\xB3\x97\x96\x95\x94\x93\x92\x91\x90a`(V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xC7W_\x80\xFD[Pa\x03\xE2`\x04\x806\x03\x81\x01\x90a\x03\xDD\x91\x90aXKV[a-\xBAV[`@Qa\x03\xF0\x92\x91\x90ac:V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\x04W_\x80\xFD[Pa\x04\ra1\x87V[`@Qa\x04\x1A\x91\x90aW\xE7V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04.W_\x80\xFD[Pa\x047a1\xC0V[`@Qa\x04D\x91\x90acoV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04XW_\x80\xFD[Pa\x04s`\x04\x806\x03\x81\x01\x90a\x04n\x91\x90aXKV[a1\xD7V[\0[4\x80\x15a\x04\x80W_\x80\xFD[Pa\x04\x9B`\x04\x806\x03\x81\x01\x90a\x04\x96\x91\x90aXKV[a4iV[`@Qa\x04\xA9\x92\x91\x90ac\xD0V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\xBDW_\x80\xFD[Pa\x04\xD8`\x04\x806\x03\x81\x01\x90a\x04\xD3\x91\x90ad\x05V[a7\xA1V[\0[4\x80\x15a\x04\xE5W_\x80\xFD[Pa\x04\xEEa;4V[`@Qa\x04\xFB\x91\x90acoV[`@Q\x80\x91\x03\x90\xF3[```@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x05E_a;KV[a\x05O`\x01a;KV[a\x05X_a;KV[`@Q` \x01a\x05k\x94\x93\x92\x91\x90ad\xFEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[``_a\x05\x8Aa<\x15V[\x90P_\x81`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\x02\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x82\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x06BW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x05\xF9W[PPPPP\x92PPP\x91\x90PV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xADW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xD1\x91\x90aepV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x07@W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x077\x91\x90ae\x9BV[`@Q\x80\x91\x03\x90\xFD[_a\x07Ia<\x15V[\x90P\x80`\t\x01T\x82\x11\x80a\x07bWP`\xF8`\x05\x90\x1B\x82\x11\x15[\x15a\x07\xA4W\x81`@Q\x7F\xCB\xE9&V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\x9B\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x08\x06W\x81`@Q\x7F\xDF\r\xB5\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\xFD\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[`\x01\x81`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01\x81`\x0F\x01_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F8O\x90\xFE\xFB\xCF\xAAh\xF2.\0\tJ\xEA\xA5++\xC6\x93\x93m,\xE1\xAF\xED\x12\x12R\x0BY\xB5\x8E\x82`@Qa\x08\x8B\x91\x90acoV[`@Q\x80\x91\x03\x90\xA1PPV[_\x80a\x08\xA1a<\x15V[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\t\x04W\x82`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xFB\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[\x80`\x0F\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\tfW\x82`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t]\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[a\tp\x81\x84a<<V[a\t\xB1W\x82`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xA8\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P_\x81\x03a\t\xF8W\x81`\r\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x92PPPa\n\x1DV[\x81`\r\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x92PPP[\x91\x90PV[`\x01a\n,a<\x86V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\nmW`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02_a\nxa<\xAAV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\n\xC0WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\n\xF7W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x0B\xB0`@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa<\xD1V[_a\x0B\xB9a<\x15V[\x90P`\xF8`\x03\x90\x1B\x81`\x04\x01\x81\x90UP`\xF8`\x04\x90\x1B\x81`\x05\x01\x81\x90UP`\xF8`\x05\x90\x1B\x81`\t\x01\x81\x90UPP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x0C0\x91\x90ae\xD6V[`@Q\x80\x91\x03\x90\xA1PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x99W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xBD\x91\x90aepV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\r,W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r#\x91\x90ae\x9BV[`@Q\x80\x91\x03\x90\xFD[_a\r5a<\x15V[\x90P_\x81`\t\x01T\x90P`\xF8`\x05\x90\x1B\x81\x14\x15\x80\x15a\rqWP\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15a\r\xB3W\x80`@Q\x7F\x06\x1A\xC6\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\xAA\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[\x81`\t\x01_\x81T\x80\x92\x91\x90a\r\xC7\x90af\x1CV[\x91\x90PUP_\x82`\t\x01T\x90P\x84\x83`\n\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x83\x83`\r\x01_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a\x0E!Wa\x0E aY}V[[\x02\x17\x90UP_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\x84W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xA8\x91\x90afwV[\x90P_\x80`\x02\x83\x83`@Q` \x01a\x0E\xC2\x93\x92\x91\x90ag\x02V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80\x86`\x0E\x01_\x86\x81R` \x01\x90\x81R` \x01_ \x90\x81a\x0E\xF3\x91\x90ai8V[P\x7F\x8C\xF0\x15\x13\x93\xF8O\xD6\x94\xC5\xE3\x15\xCBt\xCC\x05\xB2G\xDE\nEO\xD9\xE9\x12\x9Cf\x1E\xFD\xF9@\x1D\x84\x89\x89\x84`@Qa\x0F)\x94\x93\x92\x91\x90aj\x07V[`@Q\x80\x91\x03\x90\xA1PPPPPPPPV[_\x80a\x0FEa<\x15V[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x0F\xA8W\x82`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\x9F\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[\x80`\x0F\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x10\nW\x82`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\x01\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[a\x10\x14\x81\x84a<<V[a\x10UW\x82`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10L\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[\x80`\r\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xDBW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xFF\x91\x90afwV[\x90PsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cF\xC5\xBB\xBD\x823`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11P\x92\x91\x90ajQV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11kW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x8F\x91\x90aj\xA2V[a\x11\xD0W3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\xC7\x91\x90ae\x9BV[`@Q\x80\x91\x03\x90\xFD[_a\x11\xD9a<\x15V[\x90P\x80`\x05\x01T\x87\x11\x80a\x11\xF2WP`\xF8`\x04\x90\x1B\x87\x11\x15[\x15a\x124W\x86`@Q\x7F\xAD\xFA\xB9\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12+\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[_\x86\x86\x90P\x03a\x12{W\x86`@Q\x7F\xE6\xF9\x08;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12r\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[_a\x12\x88\x82\x89\x89\x89a<\xE7V[\x90P_\x82`\x06\x01_\x8A\x81R` \x01\x90\x81R` \x01_ T\x90P_\x81\x14\x15\x80\x15a\x12\xCEWP\x82`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15a\x13\x05W`@Q\x7Fo\xBC\xDD+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x13\x11\x83\x88\x88a=\xB5V[\x90P\x83_\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x13\xB1W\x89\x81`@Q\x7F\x98\xFB\x95}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13\xA8\x92\x91\x90ajQV[`@Q\x80\x91\x03\x90\xFD[`\x01\x84_\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x84`\x02\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x85\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F*\xFEd\xFB:\xFD\xE8\xE2g\x8A\xEA\x84\xCF6\"?3\x0E/\xB1(m7\xAE\xD5s\xAB\x9C\xD1\xDBG\xC7\x8B\x8B\x8B\x8B\x8B3`@Qa\x14\xD3\x96\x95\x94\x93\x92\x91\x90al\xD9V[`@Q\x80\x91\x03\x90\xA1\x84`\x01\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x15\x11WPa\x15\x10\x81\x80T\x90Pa>\x1BV[[\x15a\x17bW`\x01\x85`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_[\x8A\x8A\x90P\x81\x10\x15a\x15\xC7W\x85`\x07\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x8B\x8B\x83\x81\x81\x10a\x15tWa\x15sam.V[[\x90P` \x02\x81\x01\x90a\x15\x86\x91\x90amgV[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P\x81\x81a\x15\xB8\x91\x90ao\x9AV[PP\x80\x80`\x01\x01\x91PPa\x15CV[P\x83\x85`\x03\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x8A\x85`\x08\x01\x81\x90UP_a\x16\x8C\x86`\x0E\x01_\x8E\x81R` \x01\x90\x81R` \x01_ \x80Ta\x16\x0B\x90agkV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x167\x90agkV[\x80\x15a\x16\x82W\x80`\x1F\x10a\x16YWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x16\x82V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x16eW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa>\xACV[\x90P_a\x17 \x82\x84\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x17\x11W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x16\xC8W[PPPPP\x85\x80T\x90Pa?LV[\x90P\x7F\xEB\x85\xC2m\xBC\xADF\xB8\nh\xA0\xF2L\xCE|,\x90\xF0\xA1\xFA\xDE\xD8A\x84\x13\x889\xFC\x9E\x80\xA2[\x8D\x82\x8E\x8E`@Qa\x17W\x94\x93\x92\x91\x90ao\xA8V[`@Q\x80\x91\x03\x90\xA1PP[PPPPPPPPPPPV[a\x17wa@\x8FV[a\x17\x80\x82aAuV[a\x17\x8A\x82\x82aBhV[PPV[_a\x17\x97aC\x86V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\x1DW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18A\x91\x90afwV[\x90PsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cF\xC5\xBB\xBD\x823`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\x92\x92\x91\x90ajQV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xADW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xD1\x91\x90aj\xA2V[a\x19\x12W3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19\t\x91\x90ae\x9BV[`@Q\x80\x91\x03\x90\xFD[_a\x19\x1Ba<\x15V[\x90P\x80`\x04\x01T\x85\x11\x80a\x194WP`\xF8`\x03\x90\x1B\x85\x11\x15[\x15a\x19vW\x84`@Q\x7F\n\xB7\xF6\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19m\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[_\x81`\x0E\x01_\x87\x81R` \x01\x90\x81R` \x01_ \x80Ta\x19\x95\x90agkV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x19\xC1\x90agkV[\x80\x15a\x1A\x0CW\x80`\x1F\x10a\x19\xE3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1A\x0CV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x19\xEFW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P_a\x1A\x1E\x87\x83aD\rV[\x90P_a\x1A,\x82\x88\x88a=\xB5V[\x90P\x83_\x01_\x89\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x1A\xCCW\x87\x81`@Q\x7F3\xCA\x1F\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1A\xC3\x92\x91\x90ajQV[`@Q\x80\x91\x03\x90\xFD[`\x01\x84_\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x84`\x02\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7FLq\\W4\xCE\\\x18\xC9\xC1.\x84\x96\xE5=*e\xF1\xEC8\x1DGiW\xF0\xF5\x96\xB3d\xA5\x9B\x0C\x89\x89\x893`@Qa\x1B\xEA\x94\x93\x92\x91\x90ao\xEDV[`@Q\x80\x91\x03\x90\xA1\x84`\x01\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x1C(WPa\x1C'\x81\x80T\x90Pa>\x1BV[[\x15a\x1C\xC4W`\x01\x85`\x01\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x82\x85`\x03\x01_\x8B\x81R` \x01\x90\x81R` \x01_ \x81\x90UP_\x85`\x06\x01_\x8B\x81R` \x01\x90\x81R` \x01_ T\x90P\x7F:\x11a \xCC\xA5\xD4\xF0s\xCC\x1F\xC3\x1F\xF2a3\xAB{\x04\x99\xF2q/\xA0\x10\x02;\x87\xD5\xA1\xF9\xEE\x8A\x82\x87`@Qa\x1C\xBA\x93\x92\x91\x90ap+V[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPPPV[_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D-W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1DQ\x91\x90afwV[\x90PsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cF\xC5\xBB\xBD\x823`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1D\xA2\x92\x91\x90ajQV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xBDW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xE1\x91\x90aj\xA2V[a\x1E\"W3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1E\x19\x91\x90ae\x9BV[`@Q\x80\x91\x03\x90\xFD[_a\x1E+a<\x15V[\x90P\x80`\t\x01T\x87\x11\x80a\x1EDWP`\xF8`\x05\x90\x1B\x87\x11\x15[\x15a\x1E\x86W\x86`@Q\x7F\x8D\x8C\x94\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1E}\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[_a\x1E\x93\x82\x89\x89\x89aDoV[\x90P_a\x1E\xA1\x82\x87\x87a=\xB5V[\x90P\x82_\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x1FAW\x88\x81`@Q\x7F\xFC\xF5\xA6\xE9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1F8\x92\x91\x90ajQV[`@Q\x80\x91\x03\x90\xFD[`\x01\x83_\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x83`\x02\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F{\xF1\xB4,\x10\xE9I|\x87\x96 \xC5\xB7\xAF\xCE\xD1\x0B\xDA\x17\xD8\xC9\x0B\"\xF0\xE3\xBCk/\xD6\xCE\xD0\xBD\x8A\x8A\x8A\x8A\x8A3`@Qa c\x96\x95\x94\x93\x92\x91\x90apgV[`@Q\x80\x91\x03\x90\xA1\x83`\x01\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a \xA1WPa \xA0\x81\x80T\x90Pa>\x1BV[[\x15a\"\x8EW`\x01\x84`\x01\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x88\x88\x85`\x0B\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x91\x82a \xF3\x92\x91\x90anyV[P\x82\x84`\x03\x01_\x8C\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x89\x84`\x0C\x01\x81\x90UP_a!\xB8\x85`\x0E\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x80Ta!7\x90agkV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta!c\x90agkV[\x80\x15a!\xAEW\x80`\x1F\x10a!\x85Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a!\xAEV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a!\x91W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa>\xACV[\x90P_a\"L\x82\x84\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\"=W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a!\xF4W[PPPPP\x85\x80T\x90Pa?LV[\x90P\x7F\"X\xB7?\xAE\xD3?\xB2\xE2\xEAED\x03\xBE\xF9t\x92\x0C\xAFh*\xB3\xA7#HO\xCFgU;\x16\xA2\x8C\x82\x8D\x8D`@Qa\"\x83\x94\x93\x92\x91\x90ap\xBCV[`@Q\x80\x91\x03\x90\xA1PP[PPPPPPPPPPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\xF7W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\x1B\x91\x90aepV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a#\x8AW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a#\x81\x91\x90ae\x9BV[`@Q\x80\x91\x03\x90\xFD[`\x02_a#\x95a<\xAAV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a#\xDDWP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a$\x14W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa$\xCD`@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa<\xD1V[_a$\xD6a<\x15V[\x90P`\xF8`\x03\x90\x1B\x84_\x015\x10\x15a$\xECW_\x80\xFD[`\xF8`\x04\x90\x1B\x84` \x015\x10\x15a%\x01W_\x80\xFD[`\xF8`\x05\x90\x1B\x84`@\x015\x10\x15a%\x16W_\x80\xFD[\x83_\x015\x81`\x04\x01\x81\x90UP\x83` \x015\x81`\x05\x01\x81\x90UP\x83`@\x015\x81`\t\x01\x81\x90UP\x83``\x015\x81`\x08\x01\x81\x90UP\x83`\x80\x015\x81`\x0C\x01\x81\x90UP_\x84`\xA0\x015\x14\x15\x80\x15a%nWP_\x84`\xC0\x015\x14\x15[\x15a%\xB4W\x83`\xC0\x015\x81`\x06\x01_\x86`\xA0\x015\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x83`\xA0\x015\x81`\x06\x01_\x86`\xC0\x015\x81R` \x01\x90\x81R` \x01_ \x81\x90UP[_\x84``\x015\x14a&gW_[\x84\x80`\xE0\x01\x90a%\xD1\x91\x90aq\x01V[\x90P\x81\x10\x15a&eW\x81`\x07\x01_\x86``\x015\x81R` \x01\x90\x81R` \x01_ \x85\x80`\xE0\x01\x90a&\x01\x91\x90aq\x01V[\x83\x81\x81\x10a&\x12Wa&\x11am.V[[\x90P` \x02\x81\x01\x90a&$\x91\x90amgV[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P\x81\x81a&V\x91\x90ao\x9AV[PP\x80\x80`\x01\x01\x91PPa%\xC1V[P[_\x84`\x80\x015\x14\x15\x80\x15a&\x8CWP_\x84\x80a\x01\0\x01\x90a&\x88\x91\x90an\rV[\x90P\x11[\x15a&\xE9W\x83\x80a\x01\0\x01\x90a&\xA2\x91\x90an\rV[\x82`\x0B\x01_\x87`\x80\x015\x81R` \x01\x90\x81R` \x01_ \x91\x82a&\xC6\x92\x91\x90anyV[P\x83a\x02@\x015\x81`\n\x01_\x86`\x80\x015\x81R` \x01\x90\x81R` \x01_ \x81\x90UP[a'u\x81\x85``\x015\x86\x80a\x01 \x01\x90a'\x03\x91\x90aqcV[\x88a\x01@\x015\x89\x80a\x02\xC0\x01\x90a'\x1A\x91\x90an\rV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x8Aa\x02\0\x01` \x81\x01\x90a'p\x91\x90aq\xD9V[aE=V[a(\x01\x81\x85`\x80\x015\x86\x80a\x01`\x01\x90a'\x8F\x91\x90aqcV[\x88a\x01\x80\x015\x89\x80a\x02\xE0\x01\x90a'\xA6\x91\x90an\rV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x8Aa\x02 \x01` \x81\x01\x90a'\xFC\x91\x90aq\xD9V[aE=V[a(\x8D\x81\x85`\xA0\x015\x86\x80a\x01\xA0\x01\x90a(\x1B\x91\x90aqcV[\x88a\x01\xC0\x015\x89\x80a\x02\xA0\x01\x90a(2\x91\x90an\rV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x8Aa\x01\xE0\x01` \x81\x01\x90a(\x88\x91\x90aq\xD9V[aE=V[_\x84`\xA0\x015\x14a(\xDAW\x83a\x01\xE0\x01` \x81\x01\x90a(\xAC\x91\x90aq\xD9V[\x81`\x01\x01_\x86`\xA0\x015\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP[_\x84``\x015\x14a)'W\x83a\x02\0\x01` \x81\x01\x90a(\xF9\x91\x90aq\xD9V[\x81`\x01\x01_\x86``\x015\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP[_\x84`\x80\x015\x14a)tW\x83a\x02 \x01` \x81\x01\x90a)F\x91\x90aq\xD9V[\x81`\x01\x01_\x86`\x80\x015\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP[_\x84`\xA0\x015\x14a)\xD1W\x83a\x02`\x01` \x81\x01\x90a)\x93\x91\x90ad\x05V[\x81`\r\x01_\x86`\xA0\x015\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a)\xCBWa)\xCAaY}V[[\x02\x17\x90UP[_\x84``\x015\x14a*.W\x83a\x02`\x01` \x81\x01\x90a)\xF0\x91\x90ad\x05V[\x81`\r\x01_\x86``\x015\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a*(Wa*'aY}V[[\x02\x17\x90UP[_\x84`\x80\x015\x14a*\x8BW\x83a\x02\x80\x01` \x81\x01\x90a*M\x91\x90ad\x05V[\x81`\r\x01_\x86`\x80\x015\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a*\x85Wa*\x84aY}V[[\x02\x17\x90UP[_\x84`\xA0\x015\x14\x15\x80\x15a*\xB0WP_\x84\x80a\x02\xA0\x01\x90a*\xAC\x91\x90an\rV[\x90P\x11[\x15a*\xECW\x83\x80a\x02\xA0\x01\x90a*\xC6\x91\x90an\rV[\x82`\x0E\x01_\x87`\xA0\x015\x81R` \x01\x90\x81R` \x01_ \x91\x82a*\xEA\x92\x91\x90anyV[P[_\x84``\x015\x14\x15\x80\x15a+\x11WP_\x84\x80a\x02\xC0\x01\x90a+\r\x91\x90an\rV[\x90P\x11[\x15a+MW\x83\x80a\x02\xC0\x01\x90a+'\x91\x90an\rV[\x82`\x0E\x01_\x87``\x015\x81R` \x01\x90\x81R` \x01_ \x91\x82a+K\x92\x91\x90anyV[P[_\x84`\x80\x015\x14\x15\x80\x15a+rWP_\x84\x80a\x02\xE0\x01\x90a+n\x91\x90an\rV[\x90P\x11[\x15a+\xAEW\x83\x80a\x02\xE0\x01\x90a+\x88\x91\x90an\rV[\x82`\x0E\x01_\x87`\x80\x015\x81R` \x01\x90\x81R` \x01_ \x91\x82a+\xAC\x92\x91\x90anyV[P[P_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa+\xF9\x91\x90ae\xD6V[`@Q\x80\x91\x03\x90\xA1PPPV[_\x80a,\x10a<\x15V[\x90P_\x81`\x05\x01T\x90P`\xF8`\x04\x90\x1B\x81\x14\x15\x80\x15a,LWP\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15a,\\W`\x01\x92PPPa,\xAEV[_\x82`\t\x01T\x90P`\xF8`\x05\x90\x1B\x81\x14\x15\x80\x15a,\x96WP\x82`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15a,\xA7W`\x01\x93PPPPa,\xAEV[_\x93PPPP[\x90V[_``\x80_\x80_``_a,\xC3aG\x80V[\x90P_\x80\x1B\x81_\x01T\x14\x80\x15a,\xDEWP_\x80\x1B\x81`\x01\x01T\x14[a-\x1DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a-\x14\x90arNV[`@Q\x80\x91\x03\x90\xFD[a-%aG\xA7V[a--aHEV[F0_\x80\x1B_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-LWa-Ka[\xDFV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a-zW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94\x93\x92\x91\x90\x97P\x97P\x97P\x97P\x97P\x97P\x97PP\x90\x91\x92\x93\x94\x95\x96V[``\x80_a-\xC6a<\x15V[\x90P\x80`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a.)W\x83`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a. \x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[\x80`\x0F\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a.\x8BW\x83`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.\x82\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[_\x81`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x82`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a/BW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a.\xF9W[PPPPP\x90P_\x81Q\x90P_\x81\x03a/\x92W\x86`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a/\x89\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[_a05\x85`\x0E\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80Ta/\xB4\x90agkV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta/\xE0\x90agkV[\x80\x15a0+W\x80`\x1F\x10a0\x02Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a0+V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a0\x0EW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa>\xACV[\x90P_a0C\x82\x85\x85a?LV[\x90P\x80\x86`\x07\x01_\x8B\x81R` \x01\x90\x81R` \x01_ \x80\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a1rW\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x01\x81\x11\x15a0\xBDWa0\xBCaY}V[[`\x01\x81\x11\x15a0\xCFWa0\xCEaY}V[[\x81R` \x01`\x01\x82\x01\x80Ta0\xE3\x90agkV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta1\x0F\x90agkV[\x80\x15a1ZW\x80`\x1F\x10a11Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a1ZV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a1=W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a0yV[PPPP\x90P\x97P\x97PPPPPPP\x91P\x91V[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[_\x80a1\xCAa<\x15V[\x90P\x80`\x0C\x01T\x91PP\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a24W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2X\x91\x90aepV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a2\xC7W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a2\xBE\x91\x90ae\x9BV[`@Q\x80\x91\x03\x90\xFD[_a2\xD0a<\x15V[\x90P\x80`\x05\x01T\x82\x11\x80a2\xE9WP`\xF8`\x04\x90\x1B\x82\x11\x15[\x15a3+W\x81`@Q\x7F\xFC\xF2\xDBz\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a3\"\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a3\x8DW\x81`@Q\x7F\x92x\x9Bg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a3\x84\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[`\x01\x81`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01\x81`\x0F\x01_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x81`\x06\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P_\x81\x14a4-W`\x01\x82`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP[\x7F+\x08{\x88K5\xA8\x1Dv\x9D\x1A\x1E\t(\x80\xF1\xDAV\xDE\x96NK3\x9E\xAB\xCB\x1FE\xF5\xFE2d\x83`@Qa4\\\x91\x90acoV[`@Q\x80\x91\x03\x90\xA1PPPV[``\x80_a4ua<\x15V[\x90P\x80`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a4\xD8W\x83`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a4\xCF\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[\x80`\x0F\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a5:W\x83`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a51\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[_\x81`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x82`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a5\xF1W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a5\xA8W[PPPPP\x90P_\x81Q\x90P_\x81\x03a6AW\x86`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a68\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[_a6\xE4\x85`\x0E\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80Ta6c\x90agkV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta6\x8F\x90agkV[\x80\x15a6\xDAW\x80`\x1F\x10a6\xB1Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a6\xDAV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a6\xBDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa>\xACV[\x90P_a6\xF2\x82\x85\x85a?LV[\x90P\x80\x86`\x0B\x01_\x8B\x81R` \x01\x90\x81R` \x01_ \x80\x80Ta7\x14\x90agkV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta7@\x90agkV[\x80\x15a7\x8BW\x80`\x1F\x10a7bWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a7\x8BV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a7nW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x97P\x97PPPPPPP\x91P\x91V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\xFEW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\"\x91\x90aepV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a8\x91W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a8\x88\x91\x90ae\x9BV[`@Q\x80\x91\x03\x90\xFD[_a8\x9Aa<\x15V[\x90P_\x81`\x05\x01T\x90P`\xF8`\x04\x90\x1B\x81\x14\x15\x80\x15a8\xD6WP\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15a9\x18W\x80`@Q\x7F;\x85=\xA8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a9\x0F\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[\x81`\x04\x01_\x81T\x80\x92\x91\x90a9,\x90af\x1CV[\x91\x90PUP_\x82`\x04\x01T\x90P\x82`\x05\x01_\x81T\x80\x92\x91\x90a9M\x90af\x1CV[\x91\x90PUP_\x83`\x05\x01T\x90P\x80\x84`\x06\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x81\x84`\x06\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UP_\x85\x85`\r\x01_\x85\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a9\xC0Wa9\xBFaY}V[[\x02\x17\x90UP\x85\x85`\r\x01_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a9\xFAWa9\xF9aY}V[[\x02\x17\x90UP_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:]W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:\x81\x91\x90afwV[\x90P_`\x02\x82\x84`@Q` \x01a:\x9A\x93\x92\x91\x90ag\x02V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80\x87`\x0E\x01_\x87\x81R` \x01\x90\x81R` \x01_ \x90\x81a:\xCB\x91\x90ai8V[P\x80\x87`\x0E\x01_\x86\x81R` \x01\x90\x81R` \x01_ \x90\x81a:\xEC\x91\x90ai8V[P\x7F\xC0\xAD\xA6\xFD\x1Bei\nTmI\xDFu1\xA2\x1CR\xEDP-\x14\xEF\xD9\x8E\xE0\xC8\xF4\x84j\xBB\x93\x10\x85\x84\x8A\x84`@Qa;\"\x94\x93\x92\x91\x90aj\x07V[`@Q\x80\x91\x03\x90\xA1PPPPPPPPV[_\x80a;>a<\x15V[\x90P\x80`\x08\x01T\x91PP\x90V[``_`\x01a;Y\x84aH\xE3V[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a;wWa;va[\xDFV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a;\xA9W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x01\x82\x01\x90P[`\x01\x15a<\nW\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a;\xFFWa;\xFEarlV[[\x04\x94P_\x85\x03a;\xB6W[\x81\x93PPPP\x91\x90PV[_\x7F&\xFD\xAF\x8A,\xB2\r \xB5^6!\x89\x86\x90^SN\xE7\xA9p\xDD/\xA8'\x94nKt\x96\xDB\0\x90P\x90V[_\x80\x83`\x03\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P_\x84`\x02\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x14\x15\x91PP\x92\x91PPV[_a<\x8Fa<\xAAV[_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[a<\xD9aJ4V[a<\xE3\x82\x82aJtV[PPV[_\x80\x85`\x06\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x86`\x0E\x01_\x87\x81R` \x01\x90\x81R` \x01_ \x80Ta=\x1E\x90agkV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta=J\x90agkV[\x80\x15a=\x95W\x80`\x1F\x10a=lWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a=\x95V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a=xW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90Pa=\xA9\x82\x87\x87\x87\x85aJ\xC5V[\x92PPP\x94\x93PPPPV[_\x80a>\x04\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPaL\xA6V[\x90Pa>\x10\x813aL\xD0V[\x80\x91PP\x93\x92PPPV[_\x80sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xB4r+\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>zW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>\x9E\x91\x90afwV[\x90P\x80\x83\x10\x15\x91PP\x91\x90PV[_`!\x82Q\x10\x15a??WsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a?\x14W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a?8\x91\x90afwV[\x90Pa?GV[`!\x82\x01Q\x90P[\x91\x90PV[``_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a?iWa?ha[\xDFV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a?\x9CW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a?\x87W\x90P[P\x90P_[\x83\x81\x10\x15a@\x83WsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c1\xFFA\xC8\x87\x87\x84\x81Q\x81\x10a?\xEDWa?\xECam.V[[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a@\x12\x92\x91\x90ajQV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a@,W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@T\x91\x90as\xECV[``\x01Q\x82\x82\x81Q\x81\x10a@kWa@jam.V[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa?\xA1V[P\x80\x91PP\x93\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80aA<WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aA#aO5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15aAsW`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aA\xD2W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aA\xF6\x91\x90aepV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14aBeW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aB\\\x91\x90ae\x9BV[`@Q\x80\x91\x03\x90\xFD[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15aB\xD0WP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aB\xCD\x91\x90at]V[`\x01[aC\x11W\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aC\x08\x91\x90ae\x9BV[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14aCwW\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aCn\x91\x90a]uV[`@Q\x80\x91\x03\x90\xFD[aC\x81\x83\x83aO\x88V[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14aD\x0BW`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_aDg`@Q\x80``\x01`@R\x80`<\x81R` \x01ax\xED`<\x919\x80Q\x90` \x01 \x84\x84\x80Q\x90` \x01 `@Q` \x01aDL\x93\x92\x91\x90at\x88V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 aO\xFAV[\x90P\x92\x91PPV[_\x80\x85`\n\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x86`\x0E\x01_\x87\x81R` \x01\x90\x81R` \x01_ \x80TaD\xA6\x90agkV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80TaD\xD2\x90agkV[\x80\x15aE\x1DW\x80`\x1F\x10aD\xF4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91aE\x1DV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11aE\0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90PaE1\x86\x83\x87\x87\x85aP\x13V[\x92PPP\x94\x93PPPPV[_\x86\x14\x80aEMWP_\x85\x85\x90P\x14[aGwW\x80\x15aEpW\x82\x87`\x03\x01_\x88\x81R` \x01\x90\x81R` \x01_ \x81\x90UP[_aEz\x83a>\xACV[\x90P_[\x86\x86\x90P\x81\x10\x15aGtW_\x87\x87\x83\x81\x81\x10aE\x9DWaE\x9Cam.V[[\x90P` \x02\x01` \x81\x01\x90aE\xB2\x91\x90at\xBDV[\x90P\x89`\x02\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x87\x81R` \x01\x90\x81R` \x01_ \x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c1\xFFA\xC8\x85\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aF\x84\x92\x91\x90ajQV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aF\x9EW=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aF\xC6\x91\x90as\xECV[` \x01Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14aGeW`\x01\x8B_\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP[PP\x80\x80`\x01\x01\x91PPaE~V[PP[PPPPPPPV[_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x90P\x90V[``_aG\xB2aG\x80V[\x90P\x80`\x02\x01\x80TaG\xC3\x90agkV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80TaG\xEF\x90agkV[\x80\x15aH:W\x80`\x1F\x10aH\x11Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91aH:V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11aH\x1DW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[``_aHPaG\x80V[\x90P\x80`\x03\x01\x80TaHa\x90agkV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80TaH\x8D\x90agkV[\x80\x15aH\xD8W\x80`\x1F\x10aH\xAFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91aH\xD8V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11aH\xBBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[_\x80_\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10aI?Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81aI5WaI4arlV[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10aI|Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81aIrWaIqarlV[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10aI\xABWf#\x86\xF2o\xC1\0\0\x83\x81aI\xA1WaI\xA0arlV[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10aI\xD4Wc\x05\xF5\xE1\0\x83\x81aI\xCAWaI\xC9arlV[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10aI\xF9Wa'\x10\x83\x81aI\xEFWaI\xEEarlV[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10aJ\x1CW`d\x83\x81aJ\x12WaJ\x11arlV[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10aJ+W`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[aJ<aP\xA4V[aJrW`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[aJ|aJ4V[_aJ\x85aG\x80V[\x90P\x82\x81`\x02\x01\x90\x81aJ\x98\x91\x90au@V[P\x81\x81`\x03\x01\x90\x81aJ\xAA\x91\x90au@V[P_\x80\x1B\x81_\x01\x81\x90UP_\x80\x1B\x81`\x01\x01\x81\x90UPPPPV[_\x80\x84\x84\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aJ\xE4WaJ\xE3a[\xDFV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aK\x12W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_[\x85\x85\x90P\x81\x10\x15aL\x16W`@Q\x80``\x01`@R\x80`%\x81R` \x01az\x01`%\x919\x80Q\x90` \x01 \x86\x86\x83\x81\x81\x10aKUWaKTam.V[[\x90P` \x02\x81\x01\x90aKg\x91\x90amgV[_\x01` \x81\x01\x90aKx\x91\x90av\x0FV[\x87\x87\x84\x81\x81\x10aK\x8BWaK\x8Aam.V[[\x90P` \x02\x81\x01\x90aK\x9D\x91\x90amgV[\x80` \x01\x90aK\xAC\x91\x90an\rV[`@QaK\xBA\x92\x91\x90avhV[`@Q\x80\x91\x03\x90 `@Q` \x01aK\xD4\x93\x92\x91\x90av\x8FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x82\x81Q\x81\x10aK\xFDWaK\xFCam.V[[` \x02` \x01\x01\x81\x81RPP\x80\x80`\x01\x01\x91PPaK\x17V[PaL\x9A`@Q\x80`\xC0\x01`@R\x80`\x82\x81R` \x01ay\x7F`\x82\x919\x80Q\x90` \x01 \x88\x88\x84`@Q` \x01aLM\x91\x90awuV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x87\x80Q\x90` \x01 `@Q` \x01aL\x7F\x95\x94\x93\x92\x91\x90aw\x8BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 aO\xFAV[\x91PP\x95\x94PPPPPV[_\x80_\x80aL\xB4\x86\x86aP\xC2V[\x92P\x92P\x92PaL\xC4\x82\x82aQ\x17V[\x82\x93PPPP\x92\x91PPV[_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aM.W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aMR\x91\x90afwV[\x90PsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x94G\xCF\xD4\x82\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aM\xA3\x92\x91\x90ajQV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aM\xBEW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aM\xE2\x91\x90aj\xA2V[aN%W\x82\x82`@Q\x7F\r\x86\xF5!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aN\x1C\x92\x91\x90aw\xDCV[`@Q\x80\x91\x03\x90\xFD[_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c1\xFFA\xC8\x83\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aNu\x92\x91\x90ajQV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aN\x8FW=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aN\xB7\x91\x90as\xECV[\x90P\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14aO/W\x83\x83`@Q\x7F\r\x86\xF5!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aO&\x92\x91\x90aw\xDCV[`@Q\x80\x91\x03\x90\xFD[PPPPV[_aOa\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1BaRyV[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[aO\x91\x82aR\x82V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15aO\xEDWaO\xE7\x82\x82aSKV[PaO\xF6V[aO\xF5aS\xCBV[[PPV[_aP\x0CaP\x06aT\x07V[\x83aT\x15V[\x90P\x91\x90PV[_aP\x99`@Q\x80`\x80\x01`@R\x80`V\x81R` \x01ay)`V\x919\x80Q\x90` \x01 \x87\x87\x87\x87`@Q` \x01aPL\x92\x91\x90avhV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x86\x80Q\x90` \x01 `@Q` \x01aP~\x95\x94\x93\x92\x91\x90aw\x8BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 aO\xFAV[\x90P\x95\x94PPPPPV[_aP\xADa<\xAAV[_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[_\x80_`A\x84Q\x03aQ\x02W_\x80_` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90PaP\xF4\x88\x82\x85\x85aTUV[\x95P\x95P\x95PPPPaQ\x10V[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15aQ*WaQ)aY}V[[\x82`\x03\x81\x11\x15aQ=WaQ<aY}V[[\x03\x15aRuW`\x01`\x03\x81\x11\x15aQWWaQVaY}V[[\x82`\x03\x81\x11\x15aQjWaQiaY}V[[\x03aQ\xA1W`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15aQ\xB5WaQ\xB4aY}V[[\x82`\x03\x81\x11\x15aQ\xC8WaQ\xC7aY}V[[\x03aR\x0CW\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aR\x03\x91\x90acoV[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15aR\x1FWaR\x1EaY}V[[\x82`\x03\x81\x11\x15aR2WaR1aY}V[[\x03aRtW\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aRk\x91\x90a]uV[`@Q\x80\x91\x03\x90\xFD[[PPV[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03aR\xDDW\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aR\xD4\x91\x90ae\x9BV[`@Q\x80\x91\x03\x90\xFD[\x80aS\t\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1BaRyV[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@QaSt\x91\x90ax3V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14aS\xACW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>aS\xB1V[``\x91P[P\x91P\x91PaS\xC1\x85\x83\x83aU<V[\x92PPP\x92\x91PPV[_4\x11\x15aT\x05W`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_aT\x10aU\xC9V[\x90P\x90V[_`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x02\x82\x01R\x82`\"\x82\x01R`B\x81 \x91PP\x92\x91PPV[_\x80_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15aT\x91W_`\x03\x85\x92P\x92P\x92PaU2V[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@QaT\xB4\x94\x93\x92\x91\x90axXV[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aT\xD4W=_\x80>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03aU%W_`\x01_\x80\x1B\x93P\x93P\x93PPaU2V[\x80_\x80_\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[``\x82aUQWaUL\x82aV,V[aU\xC1V[_\x82Q\x14\x80\x15aUwWP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15aU\xB9W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aU\xB0\x91\x90ae\x9BV[`@Q\x80\x91\x03\x90\xFD[\x81\x90PaU\xC2V[[\x93\x92PPPV[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0FaU\xF3aVpV[aU\xFBaV\xE6V[F0`@Q` \x01aV\x11\x95\x94\x93\x92\x91\x90ax\x9BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x81Q\x11\x15aV>W\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80aVzaG\x80V[\x90P_aV\x85aG\xA7V[\x90P_\x81Q\x11\x15aV\xA1W\x80\x80Q\x90` \x01 \x92PPPaV\xE3V[_\x82_\x01T\x90P_\x80\x1B\x81\x14aV\xBCW\x80\x93PPPPaV\xE3V[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x80aV\xF0aG\x80V[\x90P_aV\xFBaHEV[\x90P_\x81Q\x11\x15aW\x17W\x80\x80Q\x90` \x01 \x92PPPaWZV[_\x82`\x01\x01T\x90P_\x80\x1B\x81\x14aW3W\x80\x93PPPPaWZV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15aW\x94W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90PaWyV[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_aW\xB9\x82aW]V[aW\xC3\x81\x85aWgV[\x93PaW\xD3\x81\x85` \x86\x01aWwV[aW\xDC\x81aW\x9FV[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaW\xFF\x81\x84aW\xAFV[\x90P\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x81\x90P\x91\x90PV[aX*\x81aX\x18V[\x81\x14aX4W_\x80\xFD[PV[_\x815\x90PaXE\x81aX!V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aX`WaX_aX\x10V[[_aXm\x84\x82\x85\x01aX7V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_aX\xC8\x82aX\x9FV[\x90P\x91\x90PV[aX\xD8\x81aX\xBEV[\x82RPPV[_aX\xE9\x83\x83aX\xCFV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aY\x0B\x82aXvV[aY\x15\x81\x85aX\x80V[\x93PaY \x83aX\x90V[\x80_[\x83\x81\x10\x15aYPW\x81QaY7\x88\x82aX\xDEV[\x97PaYB\x83aX\xF5V[\x92PP`\x01\x81\x01\x90PaY#V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaYu\x81\x84aY\x01V[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x02\x81\x10aY\xBBWaY\xBAaY}V[[PV[_\x81\x90PaY\xCB\x82aY\xAAV[\x91\x90PV[_aY\xDA\x82aY\xBEV[\x90P\x91\x90PV[aY\xEA\x81aY\xD0V[\x82RPPV[_` \x82\x01\x90PaZ\x03_\x83\x01\x84aY\xE1V[\x92\x91PPV[`\x02\x81\x10aZ\x15W_\x80\xFD[PV[_\x815\x90PaZ&\x81aZ\tV[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15aZBWaZAaX\x10V[[_aZO\x85\x82\x86\x01aX7V[\x92PP` aZ`\x85\x82\x86\x01aZ\x18V[\x91PP\x92P\x92\x90PV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12aZ\x8BWaZ\x8AaZjV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZ\xA8WaZ\xA7aZnV[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aZ\xC4WaZ\xC3aZrV[[\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12aZ\xE0WaZ\xDFaZjV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZ\xFDWaZ\xFCaZnV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a[\x19Wa[\x18aZrV[[\x92P\x92\x90PV[_\x80_\x80_``\x86\x88\x03\x12\x15a[9Wa[8aX\x10V[[_a[F\x88\x82\x89\x01aX7V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a[gWa[faX\x14V[[a[s\x88\x82\x89\x01aZvV[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a[\x96Wa[\x95aX\x14V[[a[\xA2\x88\x82\x89\x01aZ\xCBV[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[a[\xBA\x81aX\xBEV[\x81\x14a[\xC4W_\x80\xFD[PV[_\x815\x90Pa[\xD5\x81a[\xB1V[\x92\x91PPV[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\\\x15\x82aW\x9FV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\\4Wa\\3a[\xDFV[[\x80`@RPPPV[_a\\FaX\x07V[\x90Pa\\R\x82\x82a\\\x0CV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\\qWa\\pa[\xDFV[[a\\z\x82aW\x9FV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a\\\xA7a\\\xA2\x84a\\WV[a\\=V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\\\xC3Wa\\\xC2a[\xDBV[[a\\\xCE\x84\x82\x85a\\\x87V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\\\xEAWa\\\xE9aZjV[[\x815a\\\xFA\x84\x82` \x86\x01a\\\x95V[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a]\x19Wa]\x18aX\x10V[[_a]&\x85\x82\x86\x01a[\xC7V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a]GWa]FaX\x14V[[a]S\x85\x82\x86\x01a\\\xD6V[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[a]o\x81a]]V[\x82RPPV[_` \x82\x01\x90Pa]\x88_\x83\x01\x84a]fV[\x92\x91PPV[_\x80_`@\x84\x86\x03\x12\x15a]\xA5Wa]\xA4aX\x10V[[_a]\xB2\x86\x82\x87\x01aX7V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a]\xD3Wa]\xD2aX\x14V[[a]\xDF\x86\x82\x87\x01aZ\xCBV[\x92P\x92PP\x92P\x92P\x92V[_\x80_\x80_``\x86\x88\x03\x12\x15a^\x04Wa^\x03aX\x10V[[_a^\x11\x88\x82\x89\x01aX7V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a^2Wa^1aX\x14V[[a^>\x88\x82\x89\x01aZ\xCBV[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a^aWa^`aX\x14V[[a^m\x88\x82\x89\x01aZ\xCBV[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[_\x80\xFD[_a\x03\0\x82\x84\x03\x12\x15a^\x96Wa^\x95a^|V[[\x81\x90P\x92\x91PPV[_` \x82\x84\x03\x12\x15a^\xB4Wa^\xB3aX\x10V[[_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a^\xD1Wa^\xD0aX\x14V[[a^\xDD\x84\x82\x85\x01a^\x80V[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a^\xFA\x81a^\xE6V[\x82RPPV[_` \x82\x01\x90Pa_\x13_\x83\x01\x84a^\xF1V[\x92\x91PPV[_\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a_M\x81a_\x19V[\x82RPPV[a_\\\x81aX\x18V[\x82RPPV[a_k\x81aX\xBEV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a_\xA3\x81aX\x18V[\x82RPPV[_a_\xB4\x83\x83a_\x9AV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a_\xD6\x82a_qV[a_\xE0\x81\x85a_{V[\x93Pa_\xEB\x83a_\x8BV[\x80_[\x83\x81\x10\x15a`\x1BW\x81Qa`\x02\x88\x82a_\xA9V[\x97Pa`\r\x83a_\xC0V[\x92PP`\x01\x81\x01\x90Pa_\xEEV[P\x85\x93PPPP\x92\x91PPV[_`\xE0\x82\x01\x90Pa`;_\x83\x01\x8Aa_DV[\x81\x81\x03` \x83\x01Ra`M\x81\x89aW\xAFV[\x90P\x81\x81\x03`@\x83\x01Ra`a\x81\x88aW\xAFV[\x90Pa`p``\x83\x01\x87a_SV[a`}`\x80\x83\x01\x86a_bV[a`\x8A`\xA0\x83\x01\x85a]fV[\x81\x81\x03`\xC0\x83\x01Ra`\x9C\x81\x84a_\xCCV[\x90P\x98\x97PPPPPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a`\xED\x82aW]V[a`\xF7\x81\x85a`\xD3V[\x93Paa\x07\x81\x85` \x86\x01aWwV[aa\x10\x81aW\x9FV[\x84\x01\x91PP\x92\x91PPV[_aa&\x83\x83a`\xE3V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aaD\x82a`\xAAV[aaN\x81\x85a`\xB4V[\x93P\x83` \x82\x02\x85\x01aa`\x85a`\xC4V[\x80_[\x85\x81\x10\x15aa\x9BW\x84\x84\x03\x89R\x81Qaa|\x85\x82aa\x1BV[\x94Paa\x87\x83aa.V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaacV[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[`\x02\x81\x10aa\xE7Waa\xE6aY}V[[PV[_\x81\x90Paa\xF7\x82aa\xD6V[\x91\x90PV[_ab\x06\x82aa\xEAV[\x90P\x91\x90PV[ab\x16\x81aa\xFCV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_ab@\x82ab\x1CV[abJ\x81\x85ab&V[\x93PabZ\x81\x85` \x86\x01aWwV[abc\x81aW\x9FV[\x84\x01\x91PP\x92\x91PPV[_`@\x83\x01_\x83\x01Qab\x83_\x86\x01\x82ab\rV[P` \x83\x01Q\x84\x82\x03` \x86\x01Rab\x9B\x82\x82ab6V[\x91PP\x80\x91PP\x92\x91PPV[_ab\xB3\x83\x83abnV[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_ab\xD1\x82aa\xADV[ab\xDB\x81\x85aa\xB7V[\x93P\x83` \x82\x02\x85\x01ab\xED\x85aa\xC7V[\x80_[\x85\x81\x10\x15ac(W\x84\x84\x03\x89R\x81Qac\t\x85\x82ab\xA8V[\x94Pac\x14\x83ab\xBBV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pab\xF0V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01RacR\x81\x85aa:V[\x90P\x81\x81\x03` \x83\x01Racf\x81\x84ab\xC7V[\x90P\x93\x92PPPV[_` \x82\x01\x90Pac\x82_\x83\x01\x84a_SV[\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_ac\xA2\x82ab\x1CV[ac\xAC\x81\x85ac\x88V[\x93Pac\xBC\x81\x85` \x86\x01aWwV[ac\xC5\x81aW\x9FV[\x84\x01\x91PP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Rac\xE8\x81\x85aa:V[\x90P\x81\x81\x03` \x83\x01Rac\xFC\x81\x84ac\x98V[\x90P\x93\x92PPPV[_` \x82\x84\x03\x12\x15ad\x1AWad\x19aX\x10V[[_ad'\x84\x82\x85\x01aZ\x18V[\x91PP\x92\x91PPV[_\x81\x90P\x92\x91PPV[_adD\x82aW]V[adN\x81\x85ad0V[\x93Pad^\x81\x85` \x86\x01aWwV[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_ad\x9E`\x02\x83ad0V[\x91Pad\xA9\x82adjV[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_ad\xE8`\x01\x83ad0V[\x91Pad\xF3\x82ad\xB4V[`\x01\x82\x01\x90P\x91\x90PV[_ae\t\x82\x87ad:V[\x91Pae\x14\x82ad\x92V[\x91Pae \x82\x86ad:V[\x91Pae+\x82ad\xDCV[\x91Pae7\x82\x85ad:V[\x91PaeB\x82ad\xDCV[\x91PaeN\x82\x84ad:V[\x91P\x81\x90P\x95\x94PPPPPV[_\x81Q\x90Paej\x81a[\xB1V[\x92\x91PPV[_` \x82\x84\x03\x12\x15ae\x85Wae\x84aX\x10V[[_ae\x92\x84\x82\x85\x01ae\\V[\x91PP\x92\x91PPV[_` \x82\x01\x90Pae\xAE_\x83\x01\x84a_bV[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[ae\xD0\x81ae\xB4V[\x82RPPV[_` \x82\x01\x90Pae\xE9_\x83\x01\x84ae\xC7V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_af&\x82aX\x18V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03afXWafWae\xEFV[[`\x01\x82\x01\x90P\x91\x90PV[_\x81Q\x90Pafq\x81aX!V[\x92\x91PPV[_` \x82\x84\x03\x12\x15af\x8CWaf\x8BaX\x10V[[_af\x99\x84\x82\x85\x01afcV[\x91PP\x92\x91PPV[_`\xFF\x82\x16\x90P\x91\x90PV[_\x81`\xF8\x1B\x90P\x91\x90PV[_af\xC4\x82af\xAEV[\x90P\x91\x90PV[af\xDCaf\xD7\x82af\xA2V[af\xBAV[\x82RPPV[_\x81\x90P\x91\x90PV[af\xFCaf\xF7\x82aX\x18V[af\xE2V[\x82RPPV[_ag\r\x82\x86af\xCBV[`\x01\x82\x01\x91Pag\x1D\x82\x85af\xEBV[` \x82\x01\x91Pag-\x82\x84af\xEBV[` \x82\x01\x91P\x81\x90P\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80ag\x82W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03ag\x95Wag\x94ag>V[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02ag\xF7\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82ag\xBCV[ah\x01\x86\x83ag\xBCV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_ah<ah7ah2\x84aX\x18V[ah\x19V[aX\x18V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[ahU\x83ah\"V[ahiaha\x82ahCV[\x84\x84Tag\xC8V[\x82UPPPPV[_\x90V[ah}ahqV[ah\x88\x81\x84\x84ahLV[PPPV[[\x81\x81\x10\x15ah\xABWah\xA0_\x82ahuV[`\x01\x81\x01\x90Pah\x8EV[PPV[`\x1F\x82\x11\x15ah\xF0Wah\xC1\x81ag\x9BV[ah\xCA\x84ag\xADV[\x81\x01` \x85\x10\x15ah\xD9W\x81\x90P[ah\xEDah\xE5\x85ag\xADV[\x83\x01\x82ah\x8DV[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_ai\x10_\x19\x84`\x08\x02ah\xF5V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_ai(\x83\x83ai\x01V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[aiA\x82ab\x1CV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aiZWaiYa[\xDFV[[aid\x82TagkV[aio\x82\x82\x85ah\xAFV[_` \x90P`\x1F\x83\x11`\x01\x81\x14ai\xA0W_\x84\x15ai\x8EW\x82\x87\x01Q\x90P[ai\x98\x85\x82ai\x1DV[\x86UPai\xFFV[`\x1F\x19\x84\x16ai\xAE\x86ag\x9BV[_[\x82\x81\x10\x15ai\xD5W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pai\xB0V[\x86\x83\x10\x15ai\xF2W\x84\x89\x01Qai\xEE`\x1F\x89\x16\x82ai\x01V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_`\x80\x82\x01\x90Paj\x1A_\x83\x01\x87a_SV[aj'` \x83\x01\x86a_SV[aj4`@\x83\x01\x85aY\xE1V[\x81\x81\x03``\x83\x01RajF\x81\x84ac\x98V[\x90P\x95\x94PPPPPV[_`@\x82\x01\x90Pajd_\x83\x01\x85a_SV[ajq` \x83\x01\x84a_bV[\x93\x92PPPV[aj\x81\x81a^\xE6V[\x81\x14aj\x8BW_\x80\xFD[PV[_\x81Q\x90Paj\x9C\x81ajxV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aj\xB7Waj\xB6aX\x10V[[_aj\xC4\x84\x82\x85\x01aj\x8EV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[`\x02\x81\x10aj\xE2W_\x80\xFD[PV[_\x815\x90Paj\xF3\x81aj\xD6V[\x92\x91PPV[_ak\x07` \x84\x01\x84aj\xE5V[\x90P\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12ak7Wak6ak\x17V[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15ak_Wak^ak\x0FV[[`\x01\x82\x026\x03\x83\x13\x15akuWaktak\x13V[[P\x92P\x92\x90PV[_ak\x88\x83\x85ab&V[\x93Pak\x95\x83\x85\x84a\\\x87V[ak\x9E\x83aW\x9FV[\x84\x01\x90P\x93\x92PPPV[_`@\x83\x01ak\xBA_\x84\x01\x84aj\xF9V[ak\xC6_\x86\x01\x82ab\rV[Pak\xD4` \x84\x01\x84ak\x1BV[\x85\x83\x03` \x87\x01Rak\xE7\x83\x82\x84ak}V[\x92PPP\x80\x91PP\x92\x91PPV[_al\0\x83\x83ak\xA9V[\x90P\x92\x91PPV[_\x825`\x01`@\x03\x836\x03\x03\x81\x12al#Wal\"ak\x17V[[\x82\x81\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_alF\x83\x85aa\xB7V[\x93P\x83` \x84\x02\x85\x01alX\x84aj\xCDV[\x80_[\x87\x81\x10\x15al\x9BW\x84\x84\x03\x89Ralr\x82\x84al\x08V[al|\x85\x82ak\xF5V[\x94Pal\x87\x83al/V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pal[V[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_al\xB8\x83\x85ac\x88V[\x93Pal\xC5\x83\x85\x84a\\\x87V[al\xCE\x83aW\x9FV[\x84\x01\x90P\x93\x92PPPV[_`\x80\x82\x01\x90Pal\xEC_\x83\x01\x89a_SV[\x81\x81\x03` \x83\x01Ral\xFF\x81\x87\x89al;V[\x90P\x81\x81\x03`@\x83\x01Ram\x14\x81\x85\x87al\xADV[\x90Pam#``\x83\x01\x84a_bV[\x97\x96PPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x825`\x01`@\x03\x836\x03\x03\x81\x12am\x82Wam\x81am[V[[\x80\x83\x01\x91PP\x92\x91PPV[_\x815am\x9A\x81aj\xD6V[\x80\x91PP\x91\x90PV[_\x81_\x1B\x90P\x91\x90PV[_`\xFFam\xBA\x84am\xA3V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_am\xDA\x82aa\xEAV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[am\xF3\x82am\xD0V[an\x06am\xFF\x82am\xE1V[\x83Tam\xAEV[\x82UPPPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12an)Wan(am[V[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15anKWanJam_V[[` \x83\x01\x92P`\x01\x82\x026\x03\x83\x13\x15angWanfamcV[[P\x92P\x92\x90PV[_\x82\x90P\x92\x91PPV[an\x83\x83\x83anoV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15an\x9CWan\x9Ba[\xDFV[[an\xA6\x82TagkV[an\xB1\x82\x82\x85ah\xAFV[_`\x1F\x83\x11`\x01\x81\x14an\xDEW_\x84\x15an\xCCW\x82\x87\x015\x90P[an\xD6\x85\x82ai\x1DV[\x86UPao=V[`\x1F\x19\x84\x16an\xEC\x86ag\x9BV[_[\x82\x81\x10\x15ao\x13W\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pan\xEEV[\x86\x83\x10\x15ao0W\x84\x89\x015ao,`\x1F\x89\x16\x82ai\x01V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[aoQ\x83\x83\x83anyV[PPPV[_\x81\x01_\x83\x01\x80aof\x81am\x8EV[\x90Paor\x81\x84am\xEAV[PPP`\x01\x81\x01` \x83\x01ao\x87\x81\x85an\rV[ao\x92\x81\x83\x86aoFV[PPPPPPV[ao\xA4\x82\x82aoVV[PPV[_``\x82\x01\x90Pao\xBB_\x83\x01\x87a_SV[\x81\x81\x03` \x83\x01Rao\xCD\x81\x86aa:V[\x90P\x81\x81\x03`@\x83\x01Rao\xE2\x81\x84\x86al;V[\x90P\x95\x94PPPPPV[_``\x82\x01\x90Pap\0_\x83\x01\x87a_SV[\x81\x81\x03` \x83\x01Rap\x13\x81\x85\x87al\xADV[\x90Pap\"`@\x83\x01\x84a_bV[\x95\x94PPPPPV[_``\x82\x01\x90Pap>_\x83\x01\x86a_SV[apK` \x83\x01\x85a_SV[\x81\x81\x03`@\x83\x01Rap]\x81\x84ac\x98V[\x90P\x94\x93PPPPV[_`\x80\x82\x01\x90Papz_\x83\x01\x89a_SV[\x81\x81\x03` \x83\x01Rap\x8D\x81\x87\x89al\xADV[\x90P\x81\x81\x03`@\x83\x01Rap\xA2\x81\x85\x87al\xADV[\x90Pap\xB1``\x83\x01\x84a_bV[\x97\x96PPPPPPPV[_``\x82\x01\x90Pap\xCF_\x83\x01\x87a_SV[\x81\x81\x03` \x83\x01Rap\xE1\x81\x86aa:V[\x90P\x81\x81\x03`@\x83\x01Rap\xF6\x81\x84\x86al\xADV[\x90P\x95\x94PPPPPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aq\x1DWaq\x1Cam[V[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aq?Waq>am_V[[` \x83\x01\x92P` \x82\x026\x03\x83\x13\x15aq[WaqZamcV[[P\x92P\x92\x90PV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aq\x7FWaq~am[V[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aq\xA1Waq\xA0am_V[[` \x83\x01\x92P` \x82\x026\x03\x83\x13\x15aq\xBDWaq\xBCamcV[[P\x92P\x92\x90PV[_\x815\x90Paq\xD3\x81ajxV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aq\xEEWaq\xEDaX\x10V[[_aq\xFB\x84\x82\x85\x01aq\xC5V[\x91PP\x92\x91PPV[\x7FEIP712: Uninitialized\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_ar8`\x15\x83aWgV[\x91ParC\x82ar\x04V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Rare\x81ar,V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15ar\xBBWar\xBAa[\xDFV[[ar\xC4\x82aW\x9FV[\x90P` \x81\x01\x90P\x91\x90PV[_ar\xE3ar\xDE\x84ar\xA1V[a\\=V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15ar\xFFWar\xFEa[\xDBV[[as\n\x84\x82\x85aWwV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12as&Was%aZjV[[\x81Qas6\x84\x82` \x86\x01ar\xD1V[\x91PP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15asTWasSar\x99V[[as^`\x80a\\=V[\x90P_asm\x84\x82\x85\x01ae\\V[_\x83\x01RP` as\x80\x84\x82\x85\x01ae\\V[` \x83\x01RP`@\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15as\xA4Was\xA3ar\x9DV[[as\xB0\x84\x82\x85\x01as\x12V[`@\x83\x01RP``\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15as\xD4Was\xD3ar\x9DV[[as\xE0\x84\x82\x85\x01as\x12V[``\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15at\x01Wat\0aX\x10V[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15at\x1EWat\x1DaX\x14V[[at*\x84\x82\x85\x01as?V[\x91PP\x92\x91PPV[at<\x81a]]V[\x81\x14atFW_\x80\xFD[PV[_\x81Q\x90PatW\x81at3V[\x92\x91PPV[_` \x82\x84\x03\x12\x15atrWatqaX\x10V[[_at\x7F\x84\x82\x85\x01atIV[\x91PP\x92\x91PPV[_``\x82\x01\x90Pat\x9B_\x83\x01\x86a]fV[at\xA8` \x83\x01\x85a_SV[at\xB5`@\x83\x01\x84a]fV[\x94\x93PPPPV[_` \x82\x84\x03\x12\x15at\xD2Wat\xD1aX\x10V[[_at\xDF\x84\x82\x85\x01a[\xC7V[\x91PP\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[`\x1F\x82\x11\x15au;Wau\x0C\x81at\xE8V[au\x15\x84ag\xADV[\x81\x01` \x85\x10\x15au$W\x81\x90P[au8au0\x85ag\xADV[\x83\x01\x82ah\x8DV[PP[PPPV[auI\x82aW]V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aubWauaa[\xDFV[[aul\x82TagkV[auw\x82\x82\x85at\xFAV[_` \x90P`\x1F\x83\x11`\x01\x81\x14au\xA8W_\x84\x15au\x96W\x82\x87\x01Q\x90P[au\xA0\x85\x82ai\x1DV[\x86UPav\x07V[`\x1F\x19\x84\x16au\xB6\x86at\xE8V[_[\x82\x81\x10\x15au\xDDW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pau\xB8V[\x86\x83\x10\x15au\xFAW\x84\x89\x01Qau\xF6`\x1F\x89\x16\x82ai\x01V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_` \x82\x84\x03\x12\x15av$Wav#aX\x10V[[_av1\x84\x82\x85\x01aj\xE5V[\x91PP\x92\x91PPV[_\x81\x90P\x92\x91PPV[_avO\x83\x85av:V[\x93Pav\\\x83\x85\x84a\\\x87V[\x82\x84\x01\x90P\x93\x92PPPV[_avt\x82\x84\x86avDV[\x91P\x81\x90P\x93\x92PPPV[av\x89\x81aa\xFCV[\x82RPPV[_``\x82\x01\x90Pav\xA2_\x83\x01\x86a]fV[av\xAF` \x83\x01\x85av\x80V[av\xBC`@\x83\x01\x84a]fV[\x94\x93PPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[av\xF0\x81a]]V[\x82RPPV[_aw\x01\x83\x83av\xE7V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aw#\x82av\xC4V[aw-\x81\x85av\xCEV[\x93Paw8\x83av\xD8V[\x80_[\x83\x81\x10\x15awhW\x81QawO\x88\x82av\xF6V[\x97PawZ\x83aw\rV[\x92PP`\x01\x81\x01\x90Paw;V[P\x85\x93PPPP\x92\x91PPV[_aw\x80\x82\x84aw\x19V[\x91P\x81\x90P\x92\x91PPV[_`\xA0\x82\x01\x90Paw\x9E_\x83\x01\x88a]fV[aw\xAB` \x83\x01\x87a_SV[aw\xB8`@\x83\x01\x86a_SV[aw\xC5``\x83\x01\x85a]fV[aw\xD2`\x80\x83\x01\x84a]fV[\x96\x95PPPPPPV[_`@\x82\x01\x90Paw\xEF_\x83\x01\x85a_bV[aw\xFC` \x83\x01\x84a_bV[\x93\x92PPPV[_ax\r\x82ab\x1CV[ax\x17\x81\x85av:V[\x93Pax'\x81\x85` \x86\x01aWwV[\x80\x84\x01\x91PP\x92\x91PPV[_ax>\x82\x84ax\x03V[\x91P\x81\x90P\x92\x91PPV[axR\x81af\xA2V[\x82RPPV[_`\x80\x82\x01\x90Paxk_\x83\x01\x87a]fV[axx` \x83\x01\x86axIV[ax\x85`@\x83\x01\x85a]fV[ax\x92``\x83\x01\x84a]fV[\x95\x94PPPPPV[_`\xA0\x82\x01\x90Pax\xAE_\x83\x01\x88a]fV[ax\xBB` \x83\x01\x87a]fV[ax\xC8`@\x83\x01\x86a]fV[ax\xD5``\x83\x01\x85a_SV[ax\xE2`\x80\x83\x01\x84a_bV[\x96\x95PPPPPPV\xFEPrepKeygenVerification(uint256 prepKeygenId,bytes extraData)CrsgenVerification(uint256 crsId,uint256 maxBitLength,bytes crsDigest,bytes extraData)KeygenVerification(uint256 prepKeygenId,uint256 keyId,KeyDigest[] keyDigests,bytes extraData)KeyDigest(uint8 keyType,bytes digest)KeyDigest(uint8 keyType,bytes digest)",
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
    /**Event with signature `PrepKeygenRequest(uint256,uint256,uint8,bytes)` and selector `0xc0ada6fd1b65690a546d49df7531a21c52ed502d14efd98ee0c8f4846abb9310`.
```solidity
event PrepKeygenRequest(uint256 prepKeygenId, uint256 epochId, IKMSGeneration.ParamsType paramsType, bytes extraData);
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
                alloy::sol_types::sol_data::Uint<256>,
                IKMSGeneration::ParamsType,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "PrepKeygenRequest(uint256,uint256,uint8,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                192u8, 173u8, 166u8, 253u8, 27u8, 101u8, 105u8, 10u8, 84u8, 109u8, 73u8,
                223u8, 117u8, 49u8, 162u8, 28u8, 82u8, 237u8, 80u8, 45u8, 20u8, 239u8,
                217u8, 142u8, 224u8, 200u8, 244u8, 132u8, 106u8, 187u8, 147u8, 16u8,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.prepKeygenId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.epochId),
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
    /**Function with signature `initializeFromMigration((uint256,uint256,uint256,uint256,uint256,uint256,uint256,(uint8,bytes)[],bytes,address[],bytes32,address[],bytes32,address[],bytes32,bool,bool,bool,uint256,uint8,uint8,bytes,bytes,bytes))` and selector `0x6c68e71f`.
```solidity
function initializeFromMigration(IKMSGenerationMigration.MigrationState memory state) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeFromMigrationCall {
        #[allow(missing_docs)]
        pub state: <IKMSGenerationMigration::MigrationState as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`initializeFromMigration((uint256,uint256,uint256,uint256,uint256,uint256,uint256,(uint8,bytes)[],bytes,address[],bytes32,address[],bytes32,address[],bytes32,bool,bool,bool,uint256,uint8,uint8,bytes,bytes,bytes))`](initializeFromMigrationCall) function.
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
            type UnderlyingSolTuple<'a> = (IKMSGenerationMigration::MigrationState,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IKMSGenerationMigration::MigrationState as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
            type Parameters<'a> = (IKMSGenerationMigration::MigrationState,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeFromMigrationReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initializeFromMigration((uint256,uint256,uint256,uint256,uint256,uint256,uint256,(uint8,bytes)[],bytes,address[],bytes32,address[],bytes32,address[],bytes32,bool,bool,bool,uint256,uint8,uint8,bytes,bytes,bytes))";
            const SELECTOR: [u8; 4] = [108u8, 104u8, 231u8, 31u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IKMSGenerationMigration::MigrationState as alloy_sol_types::SolType>::tokenize(
                        &self.state,
                    ),
                )
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
            [108u8, 104u8, 231u8, 31u8],
            [115u8, 91u8, 18u8, 99u8],
            [132u8, 176u8, 25u8, 110u8],
            [147u8, 102u8, 8u8, 174u8],
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
            [33u8, 191u8, 218u8, 16u8],
            [51u8, 202u8, 31u8, 227u8],
            [59u8, 133u8, 61u8, 168u8],
            [76u8, 156u8, 140u8, 227u8],
            [111u8, 79u8, 115u8, 31u8],
            [111u8, 188u8, 221u8, 43u8],
            [132u8, 222u8, 19u8, 49u8],
            [141u8, 140u8, 148u8, 10u8],
            [146u8, 120u8, 155u8, 103u8],
            [152u8, 251u8, 149u8, 125u8],
            [153u8, 150u8, 179u8, 21u8],
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
        const COUNT: usize = 31usize;
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
                192u8, 173u8, 166u8, 253u8, 27u8, 101u8, 105u8, 10u8, 84u8, 109u8, 73u8,
                223u8, 117u8, 49u8, 162u8, 28u8, 82u8, 237u8, 80u8, 45u8, 20u8, 239u8,
                217u8, 142u8, 224u8, 200u8, 244u8, 132u8, 106u8, 187u8, 147u8, 16u8,
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
            state: <IKMSGenerationMigration::MigrationState as alloy::sol_types::SolType>::RustType,
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
