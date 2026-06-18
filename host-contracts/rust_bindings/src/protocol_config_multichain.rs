///Module containing a contract's types and functions.
/**

```solidity
library IProtocolConfigCommon {
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
pub mod IProtocolConfigCommon {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
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
    /**Creates a new wrapper around an on-chain [`IProtocolConfigCommon`](self) contract instance.

See the [wrapper's documentation](`IProtocolConfigCommonInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IProtocolConfigCommonInstance<P, N> {
        IProtocolConfigCommonInstance::<P, N>::new(address, provider)
    }
    /**A [`IProtocolConfigCommon`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IProtocolConfigCommon`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IProtocolConfigCommonInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IProtocolConfigCommonInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IProtocolConfigCommonInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IProtocolConfigCommonInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`IProtocolConfigCommon`](self) contract instance.

See the [wrapper's documentation](`IProtocolConfigCommonInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> IProtocolConfigCommonInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IProtocolConfigCommonInstance<P, N> {
            IProtocolConfigCommonInstance {
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
    > IProtocolConfigCommonInstance<P, N> {
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
    > IProtocolConfigCommonInstance<P, N> {
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
library IProtocolConfigMultichain {
    struct MirroredContextSource { uint256 sourceChainId; uint256 sourceBlockNumber; address sourceProtocolConfig; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IProtocolConfigMultichain {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct MirroredContextSource { uint256 sourceChainId; uint256 sourceBlockNumber; address sourceProtocolConfig; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MirroredContextSource {
        #[allow(missing_docs)]
        pub sourceChainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub sourceBlockNumber: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub sourceProtocolConfig: alloy::sol_types::private::Address,
    }
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
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<MirroredContextSource> for UnderlyingRustTuple<'_> {
            fn from(value: MirroredContextSource) -> Self {
                (
                    value.sourceChainId,
                    value.sourceBlockNumber,
                    value.sourceProtocolConfig,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MirroredContextSource {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    sourceChainId: tuple.0,
                    sourceBlockNumber: tuple.1,
                    sourceProtocolConfig: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for MirroredContextSource {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for MirroredContextSource {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.sourceChainId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.sourceBlockNumber),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sourceProtocolConfig,
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
        impl alloy_sol_types::SolType for MirroredContextSource {
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
        impl alloy_sol_types::SolStruct for MirroredContextSource {
            const NAME: &'static str = "MirroredContextSource";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "MirroredContextSource(uint256 sourceChainId,uint256 sourceBlockNumber,address sourceProtocolConfig)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.sourceChainId)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.sourceBlockNumber,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.sourceProtocolConfig,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for MirroredContextSource {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sourceChainId,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sourceBlockNumber,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sourceProtocolConfig,
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
                    &rust.sourceChainId,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sourceBlockNumber,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sourceProtocolConfig,
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
    /**Creates a new wrapper around an on-chain [`IProtocolConfigMultichain`](self) contract instance.

See the [wrapper's documentation](`IProtocolConfigMultichainInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IProtocolConfigMultichainInstance<P, N> {
        IProtocolConfigMultichainInstance::<P, N>::new(address, provider)
    }
    /**A [`IProtocolConfigMultichain`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IProtocolConfigMultichain`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IProtocolConfigMultichainInstance<
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IProtocolConfigMultichainInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IProtocolConfigMultichainInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IProtocolConfigMultichainInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`IProtocolConfigMultichain`](self) contract instance.

See the [wrapper's documentation](`IProtocolConfigMultichainInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> IProtocolConfigMultichainInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IProtocolConfigMultichainInstance<P, N> {
            IProtocolConfigMultichainInstance {
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
    > IProtocolConfigMultichainInstance<P, N> {
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
    > IProtocolConfigMultichainInstance<P, N> {
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
library IProtocolConfigCommon {
    struct KmsThresholds {
        uint256 publicDecryption;
        uint256 userDecryption;
        uint256 kmsGen;
        uint256 mpc;
    }
}

library IProtocolConfigMultichain {
    struct MirroredContextSource {
        uint256 sourceChainId;
        uint256 sourceBlockNumber;
        address sourceProtocolConfig;
    }
}

interface ProtocolConfigMultichain {
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
    error ERC1967InvalidImplementation(address implementation);
    error ERC1967NonPayable();
    error EmptyKmsNodes();
    error FailedCall();
    error InvalidHighThreshold(string thresholdName, uint256 threshold, uint256 nodeCount);
    error InvalidInitialization();
    error InvalidKmsContext(uint256 kmsContextId);
    error InvalidNullThreshold(string thresholdName);
    error InvalidSourceProtocolConfig();
    error KmsNodeNullSigner();
    error KmsNodeNullTxSender();
    error KmsSignerAlreadyRegistered(address signer);
    error KmsSignerSetExceedsProofFormatLimit(uint256 signerCount, uint256 maxAllowed);
    error KmsTxSenderAlreadyRegistered(address txSender);
    error NonIncreasingKmsContextId(uint256 contextId, uint256 currentKmsContextId);
    error NotHostOwner(address sender);
    error NotInitializing();
    error NotInitializingFromEmptyProxy();
    error ThresholdExceedsProofFormatLimit(string thresholdName, uint256 threshold, uint256 maxAllowed);
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);

    event Initialized(uint64 version);
    event MirroredKmsContext(uint256 indexed contextId, KmsNodeParams[] kmsNodeParams, IProtocolConfigCommon.KmsThresholds thresholds, string softwareVersion, PcrValues[] pcrValues, uint256 indexed sourceChainId, uint256 sourceBlockNumber, address indexed sourceProtocolConfig);
    event MirroredKmsContextDestroyed(uint256 indexed contextId, uint256 indexed sourceChainId, uint256 sourceBlockNumber, address indexed sourceProtocolConfig);
    event MirroredKmsThresholds(uint256 indexed contextId, IProtocolConfigCommon.KmsThresholds thresholds);
    event Upgraded(address indexed implementation);

    constructor();

    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function getCurrentKmsContextId() external view returns (uint256);
    function getKmsGenThreshold() external view returns (uint256);
    function getKmsGenThresholdForContext(uint256 kmsContextId) external view returns (uint256);
    function getKmsNodeForContext(uint256 kmsContextId, address txSender) external view returns (KmsNode memory);
    function getKmsNodesForContext(uint256 kmsContextId) external view returns (KmsNode[] memory);
    function getKmsSigners() external view returns (address[] memory);
    function getKmsSignersForContext(uint256 kmsContextId) external view returns (address[] memory);
    function getMirroredContextSource(uint256 contextId) external view returns (IProtocolConfigMultichain.MirroredContextSource memory source);
    function getMpcThreshold() external view returns (uint256);
    function getMpcThresholdForContext(uint256 kmsContextId) external view returns (uint256);
    function getPublicDecryptionThreshold() external view returns (uint256);
    function getPublicDecryptionThresholdForContext(uint256 kmsContextId) external view returns (uint256);
    function getUserDecryptionThreshold() external view returns (uint256);
    function getUserDecryptionThresholdForContext(uint256 kmsContextId) external view returns (uint256);
    function getVersion() external pure returns (string memory);
    function initializeFromEmptyProxy(uint256 initialContextId, KmsNodeParams[] memory initialKmsNodeParams, IProtocolConfigCommon.KmsThresholds memory initialThresholds, string memory softwareVersion, PcrValues[] memory pcrValues, IProtocolConfigMultichain.MirroredContextSource memory source) external;
    function isKmsSigner(address signer) external view returns (bool);
    function isKmsSignerForContext(uint256 kmsContextId, address signer) external view returns (bool);
    function isKmsTxSenderForContext(uint256 kmsContextId, address txSender) external view returns (bool);
    function isValidKmsContext(uint256 kmsContextId) external view returns (bool);
    function mirrorKmsContext(uint256 contextId, KmsNodeParams[] memory kmsNodeParams, IProtocolConfigCommon.KmsThresholds memory thresholds, string memory softwareVersion, PcrValues[] memory pcrValues, IProtocolConfigMultichain.MirroredContextSource memory source) external;
    function mirrorKmsContextDestruction(uint256 contextId, IProtocolConfigMultichain.MirroredContextSource memory source) external;
    function mirrorKmsThresholds(uint256 contextId, IProtocolConfigCommon.KmsThresholds memory thresholds) external;
    function proxiableUUID() external view returns (bytes32);
    function reinitializeV2(IProtocolConfigMultichain.MirroredContextSource memory source) external;
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
    "name": "getMirroredContextSource",
    "inputs": [
      {
        "name": "contextId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "source",
        "type": "tuple",
        "internalType": "struct IProtocolConfigMultichain.MirroredContextSource",
        "components": [
          {
            "name": "sourceChainId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "sourceBlockNumber",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "sourceProtocolConfig",
            "type": "address",
            "internalType": "address"
          }
        ]
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
        "name": "initialContextId",
        "type": "uint256",
        "internalType": "uint256"
      },
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
        "internalType": "struct IProtocolConfigCommon.KmsThresholds",
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
      },
      {
        "name": "source",
        "type": "tuple",
        "internalType": "struct IProtocolConfigMultichain.MirroredContextSource",
        "components": [
          {
            "name": "sourceChainId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "sourceBlockNumber",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "sourceProtocolConfig",
            "type": "address",
            "internalType": "address"
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
    "name": "mirrorKmsContext",
    "inputs": [
      {
        "name": "contextId",
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
        "internalType": "struct IProtocolConfigCommon.KmsThresholds",
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
      },
      {
        "name": "source",
        "type": "tuple",
        "internalType": "struct IProtocolConfigMultichain.MirroredContextSource",
        "components": [
          {
            "name": "sourceChainId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "sourceBlockNumber",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "sourceProtocolConfig",
            "type": "address",
            "internalType": "address"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "mirrorKmsContextDestruction",
    "inputs": [
      {
        "name": "contextId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "source",
        "type": "tuple",
        "internalType": "struct IProtocolConfigMultichain.MirroredContextSource",
        "components": [
          {
            "name": "sourceChainId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "sourceBlockNumber",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "sourceProtocolConfig",
            "type": "address",
            "internalType": "address"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "mirrorKmsThresholds",
    "inputs": [
      {
        "name": "contextId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "thresholds",
        "type": "tuple",
        "internalType": "struct IProtocolConfigCommon.KmsThresholds",
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
        "name": "source",
        "type": "tuple",
        "internalType": "struct IProtocolConfigMultichain.MirroredContextSource",
        "components": [
          {
            "name": "sourceChainId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "sourceBlockNumber",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "sourceProtocolConfig",
            "type": "address",
            "internalType": "address"
          }
        ]
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
    "name": "MirroredKmsContext",
    "inputs": [
      {
        "name": "contextId",
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
        "internalType": "struct IProtocolConfigCommon.KmsThresholds",
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
      },
      {
        "name": "sourceChainId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "sourceBlockNumber",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "sourceProtocolConfig",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "MirroredKmsContextDestroyed",
    "inputs": [
      {
        "name": "contextId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "sourceChainId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "sourceBlockNumber",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "sourceProtocolConfig",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "MirroredKmsThresholds",
    "inputs": [
      {
        "name": "contextId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "thresholds",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IProtocolConfigCommon.KmsThresholds",
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
    "name": "FailedCall",
    "inputs": []
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
    "name": "InvalidSourceProtocolConfig",
    "inputs": []
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
    "name": "NonIncreasingKmsContextId",
    "inputs": [
      {
        "name": "contextId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "currentKmsContextId",
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
pub mod ProtocolConfigMultichain {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a06040523073ffffffffffffffffffffffffffffffffffffffff1660809073ffffffffffffffffffffffffffffffffffffffff1681525034801562000043575f80fd5b50620000546200005a60201b60201c565b620001c4565b5f6200006b6200015e60201b60201c565b9050805f0160089054906101000a900460ff1615620000b6576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b67ffffffffffffffff8016815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff16146200015b5767ffffffffffffffff815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d267ffffffffffffffff604051620001529190620001a9565b60405180910390a15b50565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b5f67ffffffffffffffff82169050919050565b620001a38162000185565b82525050565b5f602082019050620001be5f83018462000198565b92915050565b608051614a21620001eb5f395f81816121c60152818161221b01526124bd0152614a215ff3fe60806040526004361061019b575f3560e01c80634f1ef286116100eb578063976f3eb911610089578063bf9b16c811610063578063bf9b16c8146105ef578063c2b429861461062b578063c3aaaa5a14610655578063f9c670c3146106915761019b565b8063976f3eb914610571578063ad3cb1cc1461059b578063b4722bc4146105c55761019b565b80635bff76d9116100c55780635bff76d9146104a75780637eaac8f2146104e35780638afd0ca31461050d5780639447cfd4146105355761019b565b80634f1ef2861461043957806352d1902d146104555780635b7262961461047f5761019b565b80632c87fea21161015857806341ad069c1161013257806341ad069c1461035d57806346c5bbbd1461039957806347e82295146103d55780634cfb42be146104115761019b565b80632c87fea2146102bd5780632e879d7e146102f957806331ff41c8146103215761019b565b80630d8e6e2c1461019f5780631f81e10f146101c9578063203d0114146101f157806326cf5def1461022d578063281e8bfe146102575780632a38899814610293575b5f80fd5b3480156101aa575f80fd5b506101b36106cd565b6040516101c091906131ab565b60405180910390f35b3480156101d4575f80fd5b506101ef60048036038101906101ea9190613231565b610748565b005b3480156101fc575f80fd5b50610217600480360381019061021291906132c9565b6109a3565b604051610224919061330e565b60405180910390f35b348015610238575f80fd5b50610241610a15565b60405161024e9190613336565b60405180910390f35b348015610262575f80fd5b5061027d6004803603810190610278919061334f565b610a3e565b60405161028a9190613336565b60405180910390f35b34801561029e575f80fd5b506102a7610a6a565b6040516102b49190613336565b60405180910390f35b3480156102c8575f80fd5b506102e360048036038101906102de919061334f565b610a93565b6040516102f091906133d8565b60405180910390f35b348015610304575f80fd5b5061031f600480360381019061031a91906133f1565b610b38565b005b34801561032c575f80fd5b506103476004803603810190610342919061341c565b610d15565b6040516103549190613509565b60405180910390f35b348015610368575f80fd5b50610383600480360381019061037e919061334f565b610f57565b6040516103909190613336565b60405180910390f35b3480156103a4575f80fd5b506103bf60048036038101906103ba919061341c565b610f83565b6040516103cc919061330e565b60405180910390f35b3480156103e0575f80fd5b506103fb60048036038101906103f6919061334f565b610ff7565b6040516104089190613336565b60405180910390f35b34801561041c575f80fd5b5061043760048036038101906104329190613652565b611023565b005b610453600480360381019061044e9190613865565b611215565b005b348015610460575f80fd5b50610469611234565b60405161047691906138d7565b60405180910390f35b34801561048a575f80fd5b506104a560048036038101906104a09190613652565b611265565b005b3480156104b2575f80fd5b506104cd60048036038101906104c8919061334f565b6113cb565b6040516104da9190613998565b60405180910390f35b3480156104ee575f80fd5b506104f7611479565b6040516105049190613998565b60405180910390f35b348015610518575f80fd5b50610533600480360381019061052e91906139b8565b611524565b005b348015610540575f80fd5b5061055b6004803603810190610556919061341c565b6117fe565b604051610568919061330e565b60405180910390f35b34801561057c575f80fd5b50610585611872565b6040516105929190613336565b60405180910390f35b3480156105a6575f80fd5b506105af611883565b6040516105bc91906131ab565b60405180910390f35b3480156105d0575f80fd5b506105d96118bc565b6040516105e69190613336565b60405180910390f35b3480156105fa575f80fd5b506106156004803603810190610610919061334f565b6118e5565b604051610622919061330e565b60405180910390f35b348015610636575f80fd5b5061063f6118f6565b60405161064c9190613336565b60405180910390f35b348015610660575f80fd5b5061067b6004803603810190610676919061334f565b61191f565b6040516106889190613336565b60405180910390f35b34801561069c575f80fd5b506106b760048036038101906106b2919061334f565b61194b565b6040516106c49190613b18565b60405180910390f35b60606040518060400160405280601881526020017f50726f746f636f6c436f6e6669674d756c7469636861696e000000000000000081525061070e5f611b93565b6107186002611b93565b6107215f611b93565b6040516020016107349493929190613c06565b604051602081830303815290604052905090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156107a5573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107c99190613c78565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461083857336040517f21bfda1000000000000000000000000000000000000000000000000000000000815260040161082f9190613cb2565b60405180910390fd5b5f73ffffffffffffffffffffffffffffffffffffffff1681604001602081019061086291906132c9565b73ffffffffffffffffffffffffffffffffffffffff16036108af576040517fd8e1832b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f6108b8611c5d565b9050805f0154830361090157826040517f4595fce20000000000000000000000000000000000000000000000000000000081526004016108f89190613336565b60405180910390fd5b61090a83611c84565b600181600a015f8581526020019081526020015f205f6101000a81548160ff02191690831515021790555081604001602081019061094891906132c9565b73ffffffffffffffffffffffffffffffffffffffff16825f0135847f3f5d57d3508d73eef9374d0ee29403786cb45bfb661d47cad4ad6916e0760c3985602001356040516109969190613336565b60405180910390a4505050565b5f806109ad611c5d565b9050806003015f825f015481526020019081526020015f205f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16915050919050565b5f80610a1f611c5d565b9050806009015f825f015481526020019081526020015f205491505090565b5f610a4882611c84565b610a50611c5d565b6007015f8381526020019081526020015f20549050919050565b5f80610a74611c5d565b9050806006015f825f015481526020019081526020015f205491505090565b610a9b61309b565b610aa482611c84565b610aac611c5d565b600b015f8381526020019081526020015f206040518060600160405290815f820154815260200160018201548152602001600282015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815250509050919050565b60035f610b43611cd1565b9050805f0160089054906101000a900460ff1680610b8b57508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15610bc2576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055505f73ffffffffffffffffffffffffffffffffffffffff16836040016020810190610c3191906132c9565b73ffffffffffffffffffffffffffffffffffffffff1603610c7e576040517fd8e1832b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f610c87611c5d565b90505f815f01549050610c9981611c84565b8482600b015f8381526020019081526020015f208181610cb99190613ea0565b90505050505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051610d089190613ed0565b60405180910390a1505050565b610d1d6130cf565b610d2683611c84565b610d2e611c5d565b6004015f8481526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206040518060800160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600282018054610e3f90613f16565b80601f0160208091040260200160405190810160405280929190818152602001828054610e6b90613f16565b8015610eb65780601f10610e8d57610100808354040283529160200191610eb6565b820191905f5260205f20905b815481529060010190602001808311610e9957829003601f168201915b50505050508152602001600382018054610ecf90613f16565b80601f0160208091040260200160405190810160405280929190818152602001828054610efb90613f16565b8015610f465780601f10610f1d57610100808354040283529160200191610f46565b820191905f5260205f20905b815481529060010190602001808311610f2957829003601f168201915b505050505081525050905092915050565b5f610f6182611c84565b610f69611c5d565b6008015f8381526020019081526020015f20549050919050565b5f610f8d83611c84565b610f95611c5d565b6002015f8481526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f61100182611c84565b611009611c5d565b6009015f8381526020019081526020015f20549050919050565b600161102d611cf8565b67ffffffffffffffff161461106e576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60035f611079611cd1565b9050805f0160089054906101000a900460ff16806110c157508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b156110f8576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff021916908315150217905550600160f86007600881111561115557611154613f46565b5b901b6111619190613fa0565b8b10156111a5578a6040517f77ddbe8100000000000000000000000000000000000000000000000000000000815260040161119c9190613336565b60405180910390fd5b6111b68b8b8b8b8b8b8b8b8b611d1c565b5f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2826040516112009190613ed0565b60405180910390a15050505050505050505050565b61121d6121c4565b611226826122aa565b611230828261239d565b5050565b5f61123d6124bb565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156112c2573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112e69190613c78565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461135557336040517f21bfda1000000000000000000000000000000000000000000000000000000000815260040161134c9190613cb2565b60405180910390fd5b5f61135e611c5d565b90505f815f01549050808b116113ad578a816040517fefd55f670000000000000000000000000000000000000000000000000000000081526004016113a4929190613fd3565b60405180910390fd5b6113be8b8b8b8b8b8b8b8b8b611d1c565b5050505050505050505050565b60606113d682611c84565b6113de611c5d565b6005015f8381526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561146d57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311611424575b50505050509050919050565b60605f611484611c5d565b9050806005015f825f015481526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561151957602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116114d0575b505050505091505090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611581573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115a59190613c78565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461161457336040517f21bfda1000000000000000000000000000000000000000000000000000000000815260040161160b9190613cb2565b60405180910390fd5b5f61161d611c5d565b905061162883611c84565b5f816001015f8581526020019081526020015f208054905090506116856040518060400160405280601081526020017f7075626c696344656372797074696f6e00000000000000000000000000000000815250845f013583612542565b6116c96040518060400160405280600e81526020017f7573657244656372797074696f6e000000000000000000000000000000000000815250846020013583612542565b61170d6040518060400160405280600681526020017f6b6d7347656e0000000000000000000000000000000000000000000000000000815250846040013583612542565b6117516040518060400160405280600381526020017f6d70630000000000000000000000000000000000000000000000000000000000815250846060013583612542565b825f0135826006015f8681526020019081526020015f20819055508260200135826007015f8681526020019081526020015f20819055508260400135826008015f8681526020019081526020015f20819055508260600135826009015f8681526020019081526020015f2081905550837f8ec991e9b0a696c33afa5f2327c050777dfb01687d51237e2d683e9388b64776846040516117f09190614083565b60405180910390a250505050565b5f61180883611c84565b611810611c5d565b6003015f8481526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f61187b611c5d565b5f0154905090565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b5f806118c6611c5d565b9050806008015f825f015481526020019081526020015f205491505090565b5f6118ef82612623565b9050919050565b5f80611900611c5d565b9050806007015f825f015481526020019081526020015f205491505090565b5f61192982611c84565b611931611c5d565b6006015f8381526020019081526020015f20549050919050565b606061195682611c84565b61195e611c5d565b6001015f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020015f905b82821015611b88578382905f5260205f2090600402016040518060800160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600282018054611a6990613f16565b80601f0160208091040260200160405190810160405280929190818152602001828054611a9590613f16565b8015611ae05780601f10611ab757610100808354040283529160200191611ae0565b820191905f5260205f20905b815481529060010190602001808311611ac357829003601f168201915b50505050508152602001600382018054611af990613f16565b80601f0160208091040260200160405190810160405280929190818152602001828054611b2590613f16565b8015611b705780601f10611b4757610100808354040283529160200191611b70565b820191905f5260205f20905b815481529060010190602001808311611b5357829003601f168201915b5050505050815250508152602001906001019061198f565b505050509050919050565b60605f6001611ba1846126b8565b0190505f8167ffffffffffffffff811115611bbf57611bbe613741565b5b6040519080825280601f01601f191660200182016040528015611bf15781602001600182028036833780820191505090505b5090505f82602001820190505b600115611c52578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a8581611c4757611c4661409c565b5b0494505f8503611bfe575b819350505050919050565b5f7f80f3585af86806c5774303b06c1ee640aa83b6ef3e45df49bb26c8524500c200905090565b611c8d81612623565b611cce57806040517f77ddbe81000000000000000000000000000000000000000000000000000000008152600401611cc59190613336565b60405180910390fd5b50565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b5f611d01611cd1565b5f015f9054906101000a900467ffffffffffffffff16905090565b5f73ffffffffffffffffffffffffffffffffffffffff16816040016020810190611d4691906132c9565b73ffffffffffffffffffffffffffffffffffffffff1603611d93576040517fd8e1832b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f8888905003611dcf576040517f068c8d4000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60ff8016888890501115611e22578787905060ff80166040517f16a72778000000000000000000000000000000000000000000000000000000008152600401611e19929190613fd3565b60405180910390fd5b611e686040518060400160405280601081526020017f7075626c696344656372797074696f6e00000000000000000000000000000000815250875f01358a8a9050612542565b611eaf6040518060400160405280600e81526020017f7573657244656372797074696f6e00000000000000000000000000000000000081525087602001358a8a9050612542565b611ef66040518060400160405280600681526020017f6b6d7347656e000000000000000000000000000000000000000000000000000081525087604001358a8a9050612542565b611f3d6040518060400160405280600381526020017f6d7063000000000000000000000000000000000000000000000000000000000081525087606001358a8a9050612542565b5f611f46611c5d565b905089815f01819055505f5b898990508110156120ae57368a8a83818110611f7157611f706140c9565b5b9050602002810190611f839190614102565b90506120a08c6040518060800160405280845f016020810190611fa691906132c9565b73ffffffffffffffffffffffffffffffffffffffff168152602001846020016020810190611fd491906132c9565b73ffffffffffffffffffffffffffffffffffffffff168152602001848060400190611fff919061412a565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050508152602001848060600190612056919061412a565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050815250612809565b508080600101915050611f52565b50865f0135816006015f8c81526020019081526020015f20819055508660200135816007015f8c81526020019081526020015f20819055508660400135816008015f8c81526020019081526020015f20819055508660600135816009015f8c81526020019081526020015f20819055508181600b015f8c81526020019081526020015f20818161213e9190613ea0565b90505081604001602081019061215491906132c9565b73ffffffffffffffffffffffffffffffffffffffff16825f01358b7f33d3e2406cc61c76741a8c7fee27b520ea998661cace49c1523369e7fd340f168c8c8c8c8c8c8c8c602001356040516121b0989796959493929190614690565b60405180910390a450505050505050505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff16148061227157507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16612258612d77565b73ffffffffffffffffffffffffffffffffffffffff1614155b156122a8576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612307573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061232b9190613c78565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461239a57336040517f21bfda100000000000000000000000000000000000000000000000000000000081526004016123919190613cb2565b60405180910390fd5b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa92505050801561240557506040513d601f19601f820116820180604052508101906124029190614727565b60015b61244657816040517f4c9c8ce300000000000000000000000000000000000000000000000000000000815260040161243d9190613cb2565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b81146124ac57806040517faa1d49a40000000000000000000000000000000000000000000000000000000081526004016124a391906138d7565b60405180910390fd5b6124b68383612dca565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff1614612540576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f820361258657826040517f36bfb60e00000000000000000000000000000000000000000000000000000000815260040161257d91906131ab565b60405180910390fd5b60ff80168211156125d557828260ff80166040517f22ba52db0000000000000000000000000000000000000000000000000000000081526004016125cc93929190614752565b60405180910390fd5b8082111561261e578282826040517fcaa814a300000000000000000000000000000000000000000000000000000000815260040161261593929190614752565b60405180910390fd5b505050565b5f8061262d611c5d565b9050600160f86007600881111561264757612646613f46565b5b901b6126539190613fa0565b83101580156126655750805f01548311155b801561268757505f816001015f8581526020019081526020015f208054905014155b80156126b0575080600a015f8481526020019081526020015f205f9054906101000a900460ff16155b915050919050565b5f805f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f0100000000000000008310612714577a184f03e93ff9f4daa797ed6e38ed64bf6a1f010000000000000000838161270a5761270961409c565b5b0492506040810190505b6d04ee2d6d415b85acef81000000008310612751576d04ee2d6d415b85acef810000000083816127475761274661409c565b5b0492506020810190505b662386f26fc10000831061278057662386f26fc1000083816127765761277561409c565b5b0492506010810190505b6305f5e10083106127a9576305f5e100838161279f5761279e61409c565b5b0492506008810190505b61271083106127ce5761271083816127c4576127c361409c565b5b0492506004810190505b606483106127f157606483816127e7576127e661409c565b5b0492506002810190505b600a8310612800576001810190505b80915050919050565b5f612812611c5d565b90505f73ffffffffffffffffffffffffffffffffffffffff16825f015173ffffffffffffffffffffffffffffffffffffffff160361287c576040517f8466804a00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f73ffffffffffffffffffffffffffffffffffffffff16826020015173ffffffffffffffffffffffffffffffffffffffff16036128e5576040517f2deccf4d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b806002015f8481526020019081526020015f205f835f015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff161561298857815f01516040517fd18c4ff000000000000000000000000000000000000000000000000000000000815260040161297f9190613cb2565b60405180910390fd5b806003015f8481526020019081526020015f205f836020015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615612a2d5781602001516040517ff51af6bb000000000000000000000000000000000000000000000000000000008152600401612a249190613cb2565b60405180910390fd5b806001015f8481526020019081526020015f2082908060018154018082558091505060019003905f5260205f2090600402015f909190919091505f820151815f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506020820151816001015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506040820151816002019081612b0691906148f8565b506060820151816003019081612b1c91906148f8565b5050506001816002015f8581526020019081526020015f205f845f015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055506001816003015f8581526020019081526020015f205f846020015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555081816004015f8581526020019081526020015f205f845f015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f820151815f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506020820151816001015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506040820151816002019081612ce391906148f8565b506060820151816003019081612cf991906148f8565b50905050806005015f8481526020019081526020015f208260200151908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550505050565b5f612da37f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b612e3c565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b612dd382612e45565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f81511115612e2f57612e298282612f0e565b50612e38565b612e37612f8e565b5b5050565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b03612ea057806040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401612e979190613cb2565b60405180910390fd5b80612ecc7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b612e3c565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f808473ffffffffffffffffffffffffffffffffffffffff1684604051612f379190614a0b565b5f60405180830381855af49150503d805f8114612f6f576040519150601f19603f3d011682016040523d82523d5f602084013e612f74565b606091505b5091509150612f84858383612fca565b9250505092915050565b5f341115612fc8576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b606082612fdf57612fda82613057565b61304f565b5f825114801561300557505f8473ffffffffffffffffffffffffffffffffffffffff163b145b1561304757836040517f9996b31500000000000000000000000000000000000000000000000000000000815260040161303e9190613cb2565b60405180910390fd5b819050613050565b5b9392505050565b5f815111156130695780518082602001fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60405180606001604052805f81526020015f81526020015f73ffffffffffffffffffffffffffffffffffffffff1681525090565b60405180608001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff16815260200160608152602001606081525090565b5f81519050919050565b5f82825260208201905092915050565b5f5b8381101561315857808201518184015260208101905061313d565b5f8484015250505050565b5f601f19601f8301169050919050565b5f61317d82613121565b613187818561312b565b935061319781856020860161313b565b6131a081613163565b840191505092915050565b5f6020820190508181035f8301526131c38184613173565b905092915050565b5f604051905090565b5f80fd5b5f80fd5b5f819050919050565b6131ee816131dc565b81146131f8575f80fd5b50565b5f81359050613209816131e5565b92915050565b5f80fd5b5f606082840312156132285761322761320f565b5b81905092915050565b5f8060808385031215613247576132466131d4565b5b5f613254858286016131fb565b925050602061326585828601613213565b9150509250929050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6132988261326f565b9050919050565b6132a88161328e565b81146132b2575f80fd5b50565b5f813590506132c38161329f565b92915050565b5f602082840312156132de576132dd6131d4565b5b5f6132eb848285016132b5565b91505092915050565b5f8115159050919050565b613308816132f4565b82525050565b5f6020820190506133215f8301846132ff565b92915050565b613330816131dc565b82525050565b5f6020820190506133495f830184613327565b92915050565b5f60208284031215613364576133636131d4565b5b5f613371848285016131fb565b91505092915050565b613383816131dc565b82525050565b6133928161328e565b82525050565b606082015f8201516133ac5f85018261337a565b5060208201516133bf602085018261337a565b5060408201516133d26040850182613389565b50505050565b5f6060820190506133eb5f830184613398565b92915050565b5f60608284031215613406576134056131d4565b5b5f61341384828501613213565b91505092915050565b5f8060408385031215613432576134316131d4565b5b5f61343f858286016131fb565b9250506020613450858286016132b5565b9150509250929050565b5f82825260208201905092915050565b5f61347482613121565b61347e818561345a565b935061348e81856020860161313b565b61349781613163565b840191505092915050565b5f608083015f8301516134b75f860182613389565b5060208301516134ca6020860182613389565b50604083015184820360408601526134e2828261346a565b915050606083015184820360608601526134fc828261346a565b9150508091505092915050565b5f6020820190508181035f83015261352181846134a2565b905092915050565b5f80fd5b5f80fd5b5f80fd5b5f8083601f84011261354a57613549613529565b5b8235905067ffffffffffffffff8111156135675761356661352d565b5b60208301915083602082028301111561358357613582613531565b5b9250929050565b5f6080828403121561359f5761359e61320f565b5b81905092915050565b5f8083601f8401126135bd576135bc613529565b5b8235905067ffffffffffffffff8111156135da576135d961352d565b5b6020830191508360018202830111156135f6576135f5613531565b5b9250929050565b5f8083601f84011261361257613611613529565b5b8235905067ffffffffffffffff81111561362f5761362e61352d565b5b60208301915083602082028301111561364b5761364a613531565b5b9250929050565b5f805f805f805f805f6101608a8c0312156136705761366f6131d4565b5b5f61367d8c828d016131fb565b99505060208a013567ffffffffffffffff81111561369e5761369d6131d8565b5b6136aa8c828d01613535565b985098505060406136bd8c828d0161358a565b96505060c08a013567ffffffffffffffff8111156136de576136dd6131d8565b5b6136ea8c828d016135a8565b955095505060e08a013567ffffffffffffffff81111561370d5761370c6131d8565b5b6137198c828d016135fd565b935093505061010061372d8c828d01613213565b9150509295985092959850929598565b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b61377782613163565b810181811067ffffffffffffffff8211171561379657613795613741565b5b80604052505050565b5f6137a86131cb565b90506137b4828261376e565b919050565b5f67ffffffffffffffff8211156137d3576137d2613741565b5b6137dc82613163565b9050602081019050919050565b828183375f83830152505050565b5f613809613804846137b9565b61379f565b9050828152602081018484840111156138255761382461373d565b5b6138308482856137e9565b509392505050565b5f82601f83011261384c5761384b613529565b5b813561385c8482602086016137f7565b91505092915050565b5f806040838503121561387b5761387a6131d4565b5b5f613888858286016132b5565b925050602083013567ffffffffffffffff8111156138a9576138a86131d8565b5b6138b585828601613838565b9150509250929050565b5f819050919050565b6138d1816138bf565b82525050565b5f6020820190506138ea5f8301846138c8565b92915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f6139248383613389565b60208301905092915050565b5f602082019050919050565b5f613946826138f0565b61395081856138fa565b935061395b8361390a565b805f5b8381101561398b5781516139728882613919565b975061397d83613930565b92505060018101905061395e565b5085935050505092915050565b5f6020820190508181035f8301526139b0818461393c565b905092915050565b5f8060a083850312156139ce576139cd6131d4565b5b5f6139db858286016131fb565b92505060206139ec8582860161358a565b9150509250929050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f608083015f830151613a345f860182613389565b506020830151613a476020860182613389565b5060408301518482036040860152613a5f828261346a565b91505060608301518482036060860152613a79828261346a565b9150508091505092915050565b5f613a918383613a1f565b905092915050565b5f602082019050919050565b5f613aaf826139f6565b613ab98185613a00565b935083602082028501613acb85613a10565b805f5b85811015613b065784840389528151613ae78582613a86565b9450613af283613a99565b925060208a01995050600181019050613ace565b50829750879550505050505092915050565b5f6020820190508181035f830152613b308184613aa5565b905092915050565b5f81905092915050565b5f613b4c82613121565b613b568185613b38565b9350613b6681856020860161313b565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f613ba6600283613b38565b9150613bb182613b72565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f613bf0600183613b38565b9150613bfb82613bbc565b600182019050919050565b5f613c118287613b42565b9150613c1c82613b9a565b9150613c288286613b42565b9150613c3382613be4565b9150613c3f8285613b42565b9150613c4a82613be4565b9150613c568284613b42565b915081905095945050505050565b5f81519050613c728161329f565b92915050565b5f60208284031215613c8d57613c8c6131d4565b5b5f613c9a84828501613c64565b91505092915050565b613cac8161328e565b82525050565b5f602082019050613cc55f830184613ca3565b92915050565b5f8135613cd7816131e5565b80915050919050565b5f815f1b9050919050565b5f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff613d1684613ce0565b9350801983169250808416831791505092915050565b5f819050919050565b5f613d4f613d4a613d45846131dc565b613d2c565b6131dc565b9050919050565b5f819050919050565b613d6882613d35565b613d7b613d7482613d56565b8354613ceb565b8255505050565b5f8135613d8e8161329f565b80915050919050565b5f73ffffffffffffffffffffffffffffffffffffffff613db684613ce0565b9350801983169250808416831791505092915050565b5f613de6613de1613ddc8461326f565b613d2c565b61326f565b9050919050565b5f613df782613dcc565b9050919050565b5f613e0882613ded565b9050919050565b5f819050919050565b613e2182613dfe565b613e34613e2d82613e0f565b8354613d97565b8255505050565b5f81015f830180613e4b81613ccb565b9050613e578184613d5f565b505050600181016020830180613e6c81613ccb565b9050613e788184613d5f565b505050600281016040830180613e8d81613d82565b9050613e998184613e18565b5050505050565b613eaa8282613e3b565b5050565b5f67ffffffffffffffff82169050919050565b613eca81613eae565b82525050565b5f602082019050613ee35f830184613ec1565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f6002820490506001821680613f2d57607f821691505b602082108103613f4057613f3f613ee9565b5b50919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f613faa826131dc565b9150613fb5836131dc565b9250828201905080821115613fcd57613fcc613f73565b5b92915050565b5f604082019050613fe65f830185613327565b613ff36020830184613327565b9392505050565b5f61400860208401846131fb565b905092915050565b608082016140205f830183613ffa565b61402c5f85018261337a565b5061403a6020830183613ffa565b614047602085018261337a565b506140556040830183613ffa565b614062604085018261337a565b506140706060830183613ffa565b61407d606085018261337a565b50505050565b5f6080820190506140965f830184614010565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b5f823560016101000383360303811261411e5761411d6140f6565b5b80830191505092915050565b5f8083356001602003843603038112614146576141456140f6565b5b80840192508235915067ffffffffffffffff821115614168576141676140fa565b5b602083019250600182023603831315614184576141836140fe565b5b509250929050565b5f82825260208201905092915050565b5f819050919050565b5f6141b360208401846132b5565b905092915050565b5f80fd5b5f80fd5b5f80fd5b5f80833560016020038436030381126141e3576141e26141c3565b5b83810192508235915060208301925067ffffffffffffffff82111561420b5761420a6141bb565b5b600182023603831315614221576142206141bf565b5b509250929050565b5f614234838561345a565b93506142418385846137e9565b61424a83613163565b840190509392505050565b5f8160030b9050919050565b61426a81614255565b8114614274575f80fd5b50565b5f8135905061428581614261565b92915050565b5f6142996020840184614277565b905092915050565b6142aa81614255565b82525050565b5f80833560016020038436030381126142cc576142cb6141c3565b5b83810192508235915060208301925067ffffffffffffffff8211156142f4576142f36141bb565b5b60018202360383131561430a576143096141bf565b5b509250929050565b5f82825260208201905092915050565b5f61432d8385614312565b935061433a8385846137e9565b61434383613163565b840190509392505050565b5f61010083016143605f8401846141a5565b61436c5f860182613389565b5061437a60208401846141a5565b6143876020860182613389565b5061439560408401846141c7565b85830360408701526143a8838284614229565b925050506143b960608401846141c7565b85830360608701526143cc838284614229565b925050506143dd608084018461428b565b6143ea60808601826142a1565b506143f860a08401846141c7565b85830360a087015261440b838284614229565b9250505061441c60c08401846142b0565b85830360c087015261442f838284614322565b9250505061444060e08401846141c7565b85830360e0870152614453838284614229565b925050508091505092915050565b5f61446c838361434e565b905092915050565b5f82356001610100038336030381126144905761448f6141c3565b5b82810191505092915050565b5f602082019050919050565b5f6144b3838561418c565b9350836020840285016144c58461419c565b805f5b878110156145085784840389526144df8284614474565b6144e98582614461565b94506144f48361449c565b925060208a019950506001810190506144c8565b50829750879450505050509392505050565b5f614525838561312b565b93506145328385846137e9565b61453b83613163565b840190509392505050565b5f82825260208201905092915050565b5f819050919050565b5f606083016145705f8401846142b0565b8583035f870152614582838284614322565b9250505061459360208401846142b0565b85830360208701526145a6838284614322565b925050506145b760408401846142b0565b85830360408701526145ca838284614322565b925050508091505092915050565b5f6145e3838361455f565b905092915050565b5f82356001606003833603038112614606576146056141c3565b5b82810191505092915050565b5f602082019050919050565b5f6146298385614546565b93508360208402850161463b84614556565b805f5b8781101561467e57848403895261465582846145eb565b61465f85826145d8565b945061466a83614612565b925060208a0199505060018101905061463e565b50829750879450505050509392505050565b5f610100820190508181035f8301526146aa818a8c6144a8565b90506146b96020830189614010565b81810360a08301526146cc81878961451a565b905081810360c08301526146e181858761461e565b90506146f060e0830184613327565b9998505050505050505050565b614706816138bf565b8114614710575f80fd5b50565b5f81519050614721816146fd565b92915050565b5f6020828403121561473c5761473b6131d4565b5b5f61474984828501614713565b91505092915050565b5f6060820190508181035f83015261476a8186613173565b90506147796020830185613327565b6147866040830184613327565b949350505050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f600883026147ea7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff826147af565b6147f486836147af565b95508019841693508086168417925050509392505050565b61481583613d35565b61482961482182613d56565b8484546147bb565b825550505050565b5f90565b61483d614831565b61484881848461480c565b505050565b5b8181101561486b576148605f82614835565b60018101905061484e565b5050565b601f8211156148b0576148818161478e565b61488a846147a0565b81016020851015614899578190505b6148ad6148a5856147a0565b83018261484d565b50505b505050565b5f82821c905092915050565b5f6148d05f19846008026148b5565b1980831691505092915050565b5f6148e883836148c1565b9150826002028217905092915050565b61490182613121565b67ffffffffffffffff81111561491a57614919613741565b5b6149248254613f16565b61492f82828561486f565b5f60209050601f831160018114614960575f841561494e578287015190505b61495885826148dd565b8655506149bf565b601f19841661496e8661478e565b5f5b8281101561499557848901518255600182019150602085019450602081019050614970565b868310156149b257848901516149ae601f8916826148c1565b8355505b6001600288020188555050505b505050505050565b5f81519050919050565b5f81905092915050565b5f6149e5826149c7565b6149ef81856149d1565b93506149ff81856020860161313b565b80840191505092915050565b5f614a1682846149db565b91508190509291505056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP4\x80\x15b\0\0CW_\x80\xFD[Pb\0\0Tb\0\0Z` \x1B` \x1CV[b\0\x01\xC4V[_b\0\0kb\0\x01^` \x1B` \x1CV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15b\0\0\xB6W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14b\0\x01[Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Qb\0\x01R\x91\x90b\0\x01\xA9V[`@Q\x80\x91\x03\x90\xA1[PV[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[b\0\x01\xA3\x81b\0\x01\x85V[\x82RPPV[_` \x82\x01\x90Pb\0\x01\xBE_\x83\x01\x84b\0\x01\x98V[\x92\x91PPV[`\x80QaJ!b\0\x01\xEB_9_\x81\x81a!\xC6\x01R\x81\x81a\"\x1B\x01Ra$\xBD\x01RaJ!_\xF3\xFE`\x80`@R`\x046\x10a\x01\x9BW_5`\xE0\x1C\x80cO\x1E\xF2\x86\x11a\0\xEBW\x80c\x97o>\xB9\x11a\0\x89W\x80c\xBF\x9B\x16\xC8\x11a\0cW\x80c\xBF\x9B\x16\xC8\x14a\x05\xEFW\x80c\xC2\xB4)\x86\x14a\x06+W\x80c\xC3\xAA\xAAZ\x14a\x06UW\x80c\xF9\xC6p\xC3\x14a\x06\x91Wa\x01\x9BV[\x80c\x97o>\xB9\x14a\x05qW\x80c\xAD<\xB1\xCC\x14a\x05\x9BW\x80c\xB4r+\xC4\x14a\x05\xC5Wa\x01\x9BV[\x80c[\xFFv\xD9\x11a\0\xC5W\x80c[\xFFv\xD9\x14a\x04\xA7W\x80c~\xAA\xC8\xF2\x14a\x04\xE3W\x80c\x8A\xFD\x0C\xA3\x14a\x05\rW\x80c\x94G\xCF\xD4\x14a\x055Wa\x01\x9BV[\x80cO\x1E\xF2\x86\x14a\x049W\x80cR\xD1\x90-\x14a\x04UW\x80c[rb\x96\x14a\x04\x7FWa\x01\x9BV[\x80c,\x87\xFE\xA2\x11a\x01XW\x80cA\xAD\x06\x9C\x11a\x012W\x80cA\xAD\x06\x9C\x14a\x03]W\x80cF\xC5\xBB\xBD\x14a\x03\x99W\x80cG\xE8\"\x95\x14a\x03\xD5W\x80cL\xFBB\xBE\x14a\x04\x11Wa\x01\x9BV[\x80c,\x87\xFE\xA2\x14a\x02\xBDW\x80c.\x87\x9D~\x14a\x02\xF9W\x80c1\xFFA\xC8\x14a\x03!Wa\x01\x9BV[\x80c\r\x8En,\x14a\x01\x9FW\x80c\x1F\x81\xE1\x0F\x14a\x01\xC9W\x80c =\x01\x14\x14a\x01\xF1W\x80c&\xCF]\xEF\x14a\x02-W\x80c(\x1E\x8B\xFE\x14a\x02WW\x80c*8\x89\x98\x14a\x02\x93W[_\x80\xFD[4\x80\x15a\x01\xAAW_\x80\xFD[Pa\x01\xB3a\x06\xCDV[`@Qa\x01\xC0\x91\x90a1\xABV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xD4W_\x80\xFD[Pa\x01\xEF`\x04\x806\x03\x81\x01\x90a\x01\xEA\x91\x90a21V[a\x07HV[\0[4\x80\x15a\x01\xFCW_\x80\xFD[Pa\x02\x17`\x04\x806\x03\x81\x01\x90a\x02\x12\x91\x90a2\xC9V[a\t\xA3V[`@Qa\x02$\x91\x90a3\x0EV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x028W_\x80\xFD[Pa\x02Aa\n\x15V[`@Qa\x02N\x91\x90a36V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02bW_\x80\xFD[Pa\x02}`\x04\x806\x03\x81\x01\x90a\x02x\x91\x90a3OV[a\n>V[`@Qa\x02\x8A\x91\x90a36V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x9EW_\x80\xFD[Pa\x02\xA7a\njV[`@Qa\x02\xB4\x91\x90a36V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xC8W_\x80\xFD[Pa\x02\xE3`\x04\x806\x03\x81\x01\x90a\x02\xDE\x91\x90a3OV[a\n\x93V[`@Qa\x02\xF0\x91\x90a3\xD8V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x04W_\x80\xFD[Pa\x03\x1F`\x04\x806\x03\x81\x01\x90a\x03\x1A\x91\x90a3\xF1V[a\x0B8V[\0[4\x80\x15a\x03,W_\x80\xFD[Pa\x03G`\x04\x806\x03\x81\x01\x90a\x03B\x91\x90a4\x1CV[a\r\x15V[`@Qa\x03T\x91\x90a5\tV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03hW_\x80\xFD[Pa\x03\x83`\x04\x806\x03\x81\x01\x90a\x03~\x91\x90a3OV[a\x0FWV[`@Qa\x03\x90\x91\x90a36V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xA4W_\x80\xFD[Pa\x03\xBF`\x04\x806\x03\x81\x01\x90a\x03\xBA\x91\x90a4\x1CV[a\x0F\x83V[`@Qa\x03\xCC\x91\x90a3\x0EV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xE0W_\x80\xFD[Pa\x03\xFB`\x04\x806\x03\x81\x01\x90a\x03\xF6\x91\x90a3OV[a\x0F\xF7V[`@Qa\x04\x08\x91\x90a36V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\x1CW_\x80\xFD[Pa\x047`\x04\x806\x03\x81\x01\x90a\x042\x91\x90a6RV[a\x10#V[\0[a\x04S`\x04\x806\x03\x81\x01\x90a\x04N\x91\x90a8eV[a\x12\x15V[\0[4\x80\x15a\x04`W_\x80\xFD[Pa\x04ia\x124V[`@Qa\x04v\x91\x90a8\xD7V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\x8AW_\x80\xFD[Pa\x04\xA5`\x04\x806\x03\x81\x01\x90a\x04\xA0\x91\x90a6RV[a\x12eV[\0[4\x80\x15a\x04\xB2W_\x80\xFD[Pa\x04\xCD`\x04\x806\x03\x81\x01\x90a\x04\xC8\x91\x90a3OV[a\x13\xCBV[`@Qa\x04\xDA\x91\x90a9\x98V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\xEEW_\x80\xFD[Pa\x04\xF7a\x14yV[`@Qa\x05\x04\x91\x90a9\x98V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\x18W_\x80\xFD[Pa\x053`\x04\x806\x03\x81\x01\x90a\x05.\x91\x90a9\xB8V[a\x15$V[\0[4\x80\x15a\x05@W_\x80\xFD[Pa\x05[`\x04\x806\x03\x81\x01\x90a\x05V\x91\x90a4\x1CV[a\x17\xFEV[`@Qa\x05h\x91\x90a3\x0EV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05|W_\x80\xFD[Pa\x05\x85a\x18rV[`@Qa\x05\x92\x91\x90a36V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\xA6W_\x80\xFD[Pa\x05\xAFa\x18\x83V[`@Qa\x05\xBC\x91\x90a1\xABV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\xD0W_\x80\xFD[Pa\x05\xD9a\x18\xBCV[`@Qa\x05\xE6\x91\x90a36V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\xFAW_\x80\xFD[Pa\x06\x15`\x04\x806\x03\x81\x01\x90a\x06\x10\x91\x90a3OV[a\x18\xE5V[`@Qa\x06\"\x91\x90a3\x0EV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x066W_\x80\xFD[Pa\x06?a\x18\xF6V[`@Qa\x06L\x91\x90a36V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06`W_\x80\xFD[Pa\x06{`\x04\x806\x03\x81\x01\x90a\x06v\x91\x90a3OV[a\x19\x1FV[`@Qa\x06\x88\x91\x90a36V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06\x9CW_\x80\xFD[Pa\x06\xB7`\x04\x806\x03\x81\x01\x90a\x06\xB2\x91\x90a3OV[a\x19KV[`@Qa\x06\xC4\x91\x90a;\x18V[`@Q\x80\x91\x03\x90\xF3[```@Q\x80`@\x01`@R\x80`\x18\x81R` \x01\x7FProtocolConfigMultichain\0\0\0\0\0\0\0\0\x81RPa\x07\x0E_a\x1B\x93V[a\x07\x18`\x02a\x1B\x93V[a\x07!_a\x1B\x93V[`@Q` \x01a\x074\x94\x93\x92\x91\x90a<\x06V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xA5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xC9\x91\x90a<xV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x088W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08/\x91\x90a<\xB2V[`@Q\x80\x91\x03\x90\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`@\x01` \x81\x01\x90a\x08b\x91\x90a2\xC9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x08\xAFW`@Q\x7F\xD8\xE1\x83+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x08\xB8a\x1C]V[\x90P\x80_\x01T\x83\x03a\t\x01W\x82`@Q\x7FE\x95\xFC\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xF8\x91\x90a36V[`@Q\x80\x91\x03\x90\xFD[a\t\n\x83a\x1C\x84V[`\x01\x81`\n\x01_\x85\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x81`@\x01` \x81\x01\x90a\tH\x91\x90a2\xC9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82_\x015\x84\x7F?]W\xD3P\x8Ds\xEE\xF97M\x0E\xE2\x94\x03xl\xB4[\xFBf\x1DG\xCA\xD4\xADi\x16\xE0v\x0C9\x85` \x015`@Qa\t\x96\x91\x90a36V[`@Q\x80\x91\x03\x90\xA4PPPV[_\x80a\t\xADa\x1C]V[\x90P\x80`\x03\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ _\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[_\x80a\n\x1Fa\x1C]V[\x90P\x80`\t\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ T\x91PP\x90V[_a\nH\x82a\x1C\x84V[a\nPa\x1C]V[`\x07\x01_\x83\x81R` \x01\x90\x81R` \x01_ T\x90P\x91\x90PV[_\x80a\nta\x1C]V[\x90P\x80`\x06\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ T\x91PP\x90V[a\n\x9Ba0\x9BV[a\n\xA4\x82a\x1C\x84V[a\n\xACa\x1C]V[`\x0B\x01_\x83\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x91\x90PV[`\x03_a\x0BCa\x1C\xD1V[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x0B\x8BWP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x0B\xC2W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`@\x01` \x81\x01\x90a\x0C1\x91\x90a2\xC9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0C~W`@Q\x7F\xD8\xE1\x83+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0C\x87a\x1C]V[\x90P_\x81_\x01T\x90Pa\x0C\x99\x81a\x1C\x84V[\x84\x82`\x0B\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x81a\x0C\xB9\x91\x90a>\xA0V[\x90PPPP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\r\x08\x91\x90a>\xD0V[`@Q\x80\x91\x03\x90\xA1PPPV[a\r\x1Da0\xCFV[a\r&\x83a\x1C\x84V[a\r.a\x1C]V[`\x04\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`\x80\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01\x80Ta\x0E?\x90a?\x16V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0Ek\x90a?\x16V[\x80\x15a\x0E\xB6W\x80`\x1F\x10a\x0E\x8DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0E\xB6V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0E\x99W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\x0E\xCF\x90a?\x16V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0E\xFB\x90a?\x16V[\x80\x15a\x0FFW\x80`\x1F\x10a\x0F\x1DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0FFV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0F)W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x92\x91PPV[_a\x0Fa\x82a\x1C\x84V[a\x0Fia\x1C]V[`\x08\x01_\x83\x81R` \x01\x90\x81R` \x01_ T\x90P\x91\x90PV[_a\x0F\x8D\x83a\x1C\x84V[a\x0F\x95a\x1C]V[`\x02\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[_a\x10\x01\x82a\x1C\x84V[a\x10\ta\x1C]V[`\t\x01_\x83\x81R` \x01\x90\x81R` \x01_ T\x90P\x91\x90PV[`\x01a\x10-a\x1C\xF8V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x10nW`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03_a\x10ya\x1C\xD1V[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x10\xC1WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x10\xF8W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01`\xF8`\x07`\x08\x81\x11\x15a\x11UWa\x11Ta?FV[[\x90\x1Ba\x11a\x91\x90a?\xA0V[\x8B\x10\x15a\x11\xA5W\x8A`@Q\x7Fw\xDD\xBE\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\x9C\x91\x90a36V[`@Q\x80\x91\x03\x90\xFD[a\x11\xB6\x8B\x8B\x8B\x8B\x8B\x8B\x8B\x8B\x8Ba\x1D\x1CV[_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x12\0\x91\x90a>\xD0V[`@Q\x80\x91\x03\x90\xA1PPPPPPPPPPPV[a\x12\x1Da!\xC4V[a\x12&\x82a\"\xAAV[a\x120\x82\x82a#\x9DV[PPV[_a\x12=a$\xBBV[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xC2W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xE6\x91\x90a<xV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x13UW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13L\x91\x90a<\xB2V[`@Q\x80\x91\x03\x90\xFD[_a\x13^a\x1C]V[\x90P_\x81_\x01T\x90P\x80\x8B\x11a\x13\xADW\x8A\x81`@Q\x7F\xEF\xD5_g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13\xA4\x92\x91\x90a?\xD3V[`@Q\x80\x91\x03\x90\xFD[a\x13\xBE\x8B\x8B\x8B\x8B\x8B\x8B\x8B\x8B\x8Ba\x1D\x1CV[PPPPPPPPPPPV[``a\x13\xD6\x82a\x1C\x84V[a\x13\xDEa\x1C]V[`\x05\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x14mW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x14$W[PPPPP\x90P\x91\x90PV[``_a\x14\x84a\x1C]V[\x90P\x80`\x05\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x15\x19W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x14\xD0W[PPPPP\x91PP\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\x81W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xA5\x91\x90a<xV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x16\x14W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16\x0B\x91\x90a<\xB2V[`@Q\x80\x91\x03\x90\xFD[_a\x16\x1Da\x1C]V[\x90Pa\x16(\x83a\x1C\x84V[_\x81`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90Pa\x16\x85`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01\x7FpublicDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x84_\x015\x83a%BV[a\x16\xC9`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01\x7FuserDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x84` \x015\x83a%BV[a\x17\r`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7FkmsGen\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x84`@\x015\x83a%BV[a\x17Q`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01\x7Fmpc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x84``\x015\x83a%BV[\x82_\x015\x82`\x06\x01_\x86\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82` \x015\x82`\x07\x01_\x86\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82`@\x015\x82`\x08\x01_\x86\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82``\x015\x82`\t\x01_\x86\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x83\x7F\x8E\xC9\x91\xE9\xB0\xA6\x96\xC3:\xFA_#'\xC0Pw}\xFB\x01h}Q#~-h>\x93\x88\xB6Gv\x84`@Qa\x17\xF0\x91\x90a@\x83V[`@Q\x80\x91\x03\x90\xA2PPPPV[_a\x18\x08\x83a\x1C\x84V[a\x18\x10a\x1C]V[`\x03\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[_a\x18{a\x1C]V[_\x01T\x90P\x90V[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[_\x80a\x18\xC6a\x1C]V[\x90P\x80`\x08\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ T\x91PP\x90V[_a\x18\xEF\x82a&#V[\x90P\x91\x90PV[_\x80a\x19\0a\x1C]V[\x90P\x80`\x07\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ T\x91PP\x90V[_a\x19)\x82a\x1C\x84V[a\x191a\x1C]V[`\x06\x01_\x83\x81R` \x01\x90\x81R` \x01_ T\x90P\x91\x90PV[``a\x19V\x82a\x1C\x84V[a\x19^a\x1C]V[`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x1B\x88W\x83\x82\x90_R` _ \x90`\x04\x02\x01`@Q\x80`\x80\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01\x80Ta\x1Ai\x90a?\x16V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1A\x95\x90a?\x16V[\x80\x15a\x1A\xE0W\x80`\x1F\x10a\x1A\xB7Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1A\xE0V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1A\xC3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\x1A\xF9\x90a?\x16V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1B%\x90a?\x16V[\x80\x15a\x1BpW\x80`\x1F\x10a\x1BGWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1BpV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1BSW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x19\x8FV[PPPP\x90P\x91\x90PV[``_`\x01a\x1B\xA1\x84a&\xB8V[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B\xBFWa\x1B\xBEa7AV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1B\xF1W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x01\x82\x01\x90P[`\x01\x15a\x1CRW\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a\x1CGWa\x1CFa@\x9CV[[\x04\x94P_\x85\x03a\x1B\xFEW[\x81\x93PPPP\x91\x90PV[_\x7F\x80\xF3XZ\xF8h\x06\xC5wC\x03\xB0l\x1E\xE6@\xAA\x83\xB6\xEF>E\xDFI\xBB&\xC8RE\0\xC2\0\x90P\x90V[a\x1C\x8D\x81a&#V[a\x1C\xCEW\x80`@Q\x7Fw\xDD\xBE\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1C\xC5\x91\x90a36V[`@Q\x80\x91\x03\x90\xFD[PV[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[_a\x1D\x01a\x1C\xD1V[_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`@\x01` \x81\x01\x90a\x1DF\x91\x90a2\xC9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x1D\x93W`@Q\x7F\xD8\xE1\x83+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x88\x88\x90P\x03a\x1D\xCFW`@Q\x7F\x06\x8C\x8D@\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x80\x16\x88\x88\x90P\x11\x15a\x1E\"W\x87\x87\x90P`\xFF\x80\x16`@Q\x7F\x16\xA7'x\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1E\x19\x92\x91\x90a?\xD3V[`@Q\x80\x91\x03\x90\xFD[a\x1Eh`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01\x7FpublicDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x87_\x015\x8A\x8A\x90Pa%BV[a\x1E\xAF`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01\x7FuserDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x87` \x015\x8A\x8A\x90Pa%BV[a\x1E\xF6`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7FkmsGen\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x87`@\x015\x8A\x8A\x90Pa%BV[a\x1F=`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01\x7Fmpc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x87``\x015\x8A\x8A\x90Pa%BV[_a\x1FFa\x1C]V[\x90P\x89\x81_\x01\x81\x90UP_[\x89\x89\x90P\x81\x10\x15a \xAEW6\x8A\x8A\x83\x81\x81\x10a\x1FqWa\x1Fpa@\xC9V[[\x90P` \x02\x81\x01\x90a\x1F\x83\x91\x90aA\x02V[\x90Pa \xA0\x8C`@Q\x80`\x80\x01`@R\x80\x84_\x01` \x81\x01\x90a\x1F\xA6\x91\x90a2\xC9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84` \x01` \x81\x01\x90a\x1F\xD4\x91\x90a2\xC9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84\x80`@\x01\x90a\x1F\xFF\x91\x90aA*V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x84\x80``\x01\x90a V\x91\x90aA*V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81RPa(\tV[P\x80\x80`\x01\x01\x91PPa\x1FRV[P\x86_\x015\x81`\x06\x01_\x8C\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x86` \x015\x81`\x07\x01_\x8C\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x86`@\x015\x81`\x08\x01_\x8C\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x86``\x015\x81`\t\x01_\x8C\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x81\x81`\x0B\x01_\x8C\x81R` \x01\x90\x81R` \x01_ \x81\x81a!>\x91\x90a>\xA0V[\x90PP\x81`@\x01` \x81\x01\x90a!T\x91\x90a2\xC9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82_\x015\x8B\x7F3\xD3\xE2@l\xC6\x1Cvt\x1A\x8C\x7F\xEE'\xB5 \xEA\x99\x86a\xCA\xCEI\xC1R3i\xE7\xFD4\x0F\x16\x8C\x8C\x8C\x8C\x8C\x8C\x8C\x8C` \x015`@Qa!\xB0\x98\x97\x96\x95\x94\x93\x92\x91\x90aF\x90V[`@Q\x80\x91\x03\x90\xA4PPPPPPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a\"qWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\"Xa-wV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a\"\xA8W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#\x07W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#+\x91\x90a<xV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a#\x9AW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a#\x91\x91\x90a<\xB2V[`@Q\x80\x91\x03\x90\xFD[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a$\x05WP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\x02\x91\x90aG'V[`\x01[a$FW\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a$=\x91\x90a<\xB2V[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14a$\xACW\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a$\xA3\x91\x90a8\xD7V[`@Q\x80\x91\x03\x90\xFD[a$\xB6\x83\x83a-\xCAV[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a%@W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x82\x03a%\x86W\x82`@Q\x7F6\xBF\xB6\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a%}\x91\x90a1\xABV[`@Q\x80\x91\x03\x90\xFD[`\xFF\x80\x16\x82\x11\x15a%\xD5W\x82\x82`\xFF\x80\x16`@Q\x7F\"\xBAR\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a%\xCC\x93\x92\x91\x90aGRV[`@Q\x80\x91\x03\x90\xFD[\x80\x82\x11\x15a&\x1EW\x82\x82\x82`@Q\x7F\xCA\xA8\x14\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&\x15\x93\x92\x91\x90aGRV[`@Q\x80\x91\x03\x90\xFD[PPPV[_\x80a&-a\x1C]V[\x90P`\x01`\xF8`\x07`\x08\x81\x11\x15a&GWa&Fa?FV[[\x90\x1Ba&S\x91\x90a?\xA0V[\x83\x10\x15\x80\x15a&eWP\x80_\x01T\x83\x11\x15[\x80\x15a&\x87WP_\x81`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x14\x15[\x80\x15a&\xB0WP\x80`\n\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x91PP\x91\x90PV[_\x80_\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a'\x14Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81a'\nWa'\ta@\x9CV[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a'QWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81a'GWa'Fa@\x9CV[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10a'\x80Wf#\x86\xF2o\xC1\0\0\x83\x81a'vWa'ua@\x9CV[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10a'\xA9Wc\x05\xF5\xE1\0\x83\x81a'\x9FWa'\x9Ea@\x9CV[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10a'\xCEWa'\x10\x83\x81a'\xC4Wa'\xC3a@\x9CV[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10a'\xF1W`d\x83\x81a'\xE7Wa'\xE6a@\x9CV[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10a(\0W`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[_a(\x12a\x1C]V[\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a(|W`@Q\x7F\x84f\x80J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a(\xE5W`@Q\x7F-\xEC\xCFM\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x02\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x83_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a)\x88W\x81_\x01Q`@Q\x7F\xD1\x8CO\xF0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a)\x7F\x91\x90a<\xB2V[`@Q\x80\x91\x03\x90\xFD[\x80`\x03\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x83` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a*-W\x81` \x01Q`@Q\x7F\xF5\x1A\xF6\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a*$\x91\x90a<\xB2V[`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x04\x02\x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x02\x01\x90\x81a+\x06\x91\x90aH\xF8V[P``\x82\x01Q\x81`\x03\x01\x90\x81a+\x1C\x91\x90aH\xF8V[PPP`\x01\x81`\x02\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x84_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01\x81`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x84` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x81\x81`\x04\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x84_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x02\x01\x90\x81a,\xE3\x91\x90aH\xF8V[P``\x82\x01Q\x81`\x03\x01\x90\x81a,\xF9\x91\x90aH\xF8V[P\x90PP\x80`\x05\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x82` \x01Q\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPV[_a-\xA3\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba.<V[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a-\xD3\x82a.EV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15a./Wa.)\x82\x82a/\x0EV[Pa.8V[a.7a/\x8EV[[PPV[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a.\xA0W\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.\x97\x91\x90a<\xB2V[`@Q\x80\x91\x03\x90\xFD[\x80a.\xCC\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba.<V[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qa/7\x91\x90aJ\x0BV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a/oW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a/tV[``\x91P[P\x91P\x91Pa/\x84\x85\x83\x83a/\xCAV[\x92PPP\x92\x91PPV[_4\x11\x15a/\xC8W`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[``\x82a/\xDFWa/\xDA\x82a0WV[a0OV[_\x82Q\x14\x80\x15a0\x05WP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a0GW\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a0>\x91\x90a<\xB2V[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa0PV[[\x93\x92PPPV[_\x81Q\x11\x15a0iW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x80``\x01`@R\x80_\x81R` \x01_\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81R` \x01``\x81RP\x90V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15a1XW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa1=V[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a1}\x82a1!V[a1\x87\x81\x85a1+V[\x93Pa1\x97\x81\x85` \x86\x01a1;V[a1\xA0\x81a1cV[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra1\xC3\x81\x84a1sV[\x90P\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x81\x90P\x91\x90PV[a1\xEE\x81a1\xDCV[\x81\x14a1\xF8W_\x80\xFD[PV[_\x815\x90Pa2\t\x81a1\xE5V[\x92\x91PPV[_\x80\xFD[_``\x82\x84\x03\x12\x15a2(Wa2'a2\x0FV[[\x81\x90P\x92\x91PPV[_\x80`\x80\x83\x85\x03\x12\x15a2GWa2Fa1\xD4V[[_a2T\x85\x82\x86\x01a1\xFBV[\x92PP` a2e\x85\x82\x86\x01a2\x13V[\x91PP\x92P\x92\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a2\x98\x82a2oV[\x90P\x91\x90PV[a2\xA8\x81a2\x8EV[\x81\x14a2\xB2W_\x80\xFD[PV[_\x815\x90Pa2\xC3\x81a2\x9FV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a2\xDEWa2\xDDa1\xD4V[[_a2\xEB\x84\x82\x85\x01a2\xB5V[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a3\x08\x81a2\xF4V[\x82RPPV[_` \x82\x01\x90Pa3!_\x83\x01\x84a2\xFFV[\x92\x91PPV[a30\x81a1\xDCV[\x82RPPV[_` \x82\x01\x90Pa3I_\x83\x01\x84a3'V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a3dWa3ca1\xD4V[[_a3q\x84\x82\x85\x01a1\xFBV[\x91PP\x92\x91PPV[a3\x83\x81a1\xDCV[\x82RPPV[a3\x92\x81a2\x8EV[\x82RPPV[``\x82\x01_\x82\x01Qa3\xAC_\x85\x01\x82a3zV[P` \x82\x01Qa3\xBF` \x85\x01\x82a3zV[P`@\x82\x01Qa3\xD2`@\x85\x01\x82a3\x89V[PPPPV[_``\x82\x01\x90Pa3\xEB_\x83\x01\x84a3\x98V[\x92\x91PPV[_``\x82\x84\x03\x12\x15a4\x06Wa4\x05a1\xD4V[[_a4\x13\x84\x82\x85\x01a2\x13V[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a42Wa41a1\xD4V[[_a4?\x85\x82\x86\x01a1\xFBV[\x92PP` a4P\x85\x82\x86\x01a2\xB5V[\x91PP\x92P\x92\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a4t\x82a1!V[a4~\x81\x85a4ZV[\x93Pa4\x8E\x81\x85` \x86\x01a1;V[a4\x97\x81a1cV[\x84\x01\x91PP\x92\x91PPV[_`\x80\x83\x01_\x83\x01Qa4\xB7_\x86\x01\x82a3\x89V[P` \x83\x01Qa4\xCA` \x86\x01\x82a3\x89V[P`@\x83\x01Q\x84\x82\x03`@\x86\x01Ra4\xE2\x82\x82a4jV[\x91PP``\x83\x01Q\x84\x82\x03``\x86\x01Ra4\xFC\x82\x82a4jV[\x91PP\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra5!\x81\x84a4\xA2V[\x90P\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12a5JWa5Ia5)V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5gWa5fa5-V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a5\x83Wa5\x82a51V[[\x92P\x92\x90PV[_`\x80\x82\x84\x03\x12\x15a5\x9FWa5\x9Ea2\x0FV[[\x81\x90P\x92\x91PPV[_\x80\x83`\x1F\x84\x01\x12a5\xBDWa5\xBCa5)V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5\xDAWa5\xD9a5-V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a5\xF6Wa5\xF5a51V[[\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12a6\x12Wa6\x11a5)V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6/Wa6.a5-V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a6KWa6Ja51V[[\x92P\x92\x90PV[_\x80_\x80_\x80_\x80_a\x01`\x8A\x8C\x03\x12\x15a6pWa6oa1\xD4V[[_a6}\x8C\x82\x8D\x01a1\xFBV[\x99PP` \x8A\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\x9EWa6\x9Da1\xD8V[[a6\xAA\x8C\x82\x8D\x01a55V[\x98P\x98PP`@a6\xBD\x8C\x82\x8D\x01a5\x8AV[\x96PP`\xC0\x8A\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\xDEWa6\xDDa1\xD8V[[a6\xEA\x8C\x82\x8D\x01a5\xA8V[\x95P\x95PP`\xE0\x8A\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7\rWa7\x0Ca1\xD8V[[a7\x19\x8C\x82\x8D\x01a5\xFDV[\x93P\x93PPa\x01\0a7-\x8C\x82\x8D\x01a2\x13V[\x91PP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a7w\x82a1cV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a7\x96Wa7\x95a7AV[[\x80`@RPPPV[_a7\xA8a1\xCBV[\x90Pa7\xB4\x82\x82a7nV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a7\xD3Wa7\xD2a7AV[[a7\xDC\x82a1cV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a8\ta8\x04\x84a7\xB9V[a7\x9FV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a8%Wa8$a7=V[[a80\x84\x82\x85a7\xE9V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a8LWa8Ka5)V[[\x815a8\\\x84\x82` \x86\x01a7\xF7V[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a8{Wa8za1\xD4V[[_a8\x88\x85\x82\x86\x01a2\xB5V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8\xA9Wa8\xA8a1\xD8V[[a8\xB5\x85\x82\x86\x01a88V[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[a8\xD1\x81a8\xBFV[\x82RPPV[_` \x82\x01\x90Pa8\xEA_\x83\x01\x84a8\xC8V[\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_a9$\x83\x83a3\x89V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a9F\x82a8\xF0V[a9P\x81\x85a8\xFAV[\x93Pa9[\x83a9\nV[\x80_[\x83\x81\x10\x15a9\x8BW\x81Qa9r\x88\x82a9\x19V[\x97Pa9}\x83a90V[\x92PP`\x01\x81\x01\x90Pa9^V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra9\xB0\x81\x84a9<V[\x90P\x92\x91PPV[_\x80`\xA0\x83\x85\x03\x12\x15a9\xCEWa9\xCDa1\xD4V[[_a9\xDB\x85\x82\x86\x01a1\xFBV[\x92PP` a9\xEC\x85\x82\x86\x01a5\x8AV[\x91PP\x92P\x92\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_`\x80\x83\x01_\x83\x01Qa:4_\x86\x01\x82a3\x89V[P` \x83\x01Qa:G` \x86\x01\x82a3\x89V[P`@\x83\x01Q\x84\x82\x03`@\x86\x01Ra:_\x82\x82a4jV[\x91PP``\x83\x01Q\x84\x82\x03``\x86\x01Ra:y\x82\x82a4jV[\x91PP\x80\x91PP\x92\x91PPV[_a:\x91\x83\x83a:\x1FV[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a:\xAF\x82a9\xF6V[a:\xB9\x81\x85a:\0V[\x93P\x83` \x82\x02\x85\x01a:\xCB\x85a:\x10V[\x80_[\x85\x81\x10\x15a;\x06W\x84\x84\x03\x89R\x81Qa:\xE7\x85\x82a:\x86V[\x94Pa:\xF2\x83a:\x99V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa:\xCEV[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra;0\x81\x84a:\xA5V[\x90P\x92\x91PPV[_\x81\x90P\x92\x91PPV[_a;L\x82a1!V[a;V\x81\x85a;8V[\x93Pa;f\x81\x85` \x86\x01a1;V[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a;\xA6`\x02\x83a;8V[\x91Pa;\xB1\x82a;rV[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a;\xF0`\x01\x83a;8V[\x91Pa;\xFB\x82a;\xBCV[`\x01\x82\x01\x90P\x91\x90PV[_a<\x11\x82\x87a;BV[\x91Pa<\x1C\x82a;\x9AV[\x91Pa<(\x82\x86a;BV[\x91Pa<3\x82a;\xE4V[\x91Pa<?\x82\x85a;BV[\x91Pa<J\x82a;\xE4V[\x91Pa<V\x82\x84a;BV[\x91P\x81\x90P\x95\x94PPPPPV[_\x81Q\x90Pa<r\x81a2\x9FV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a<\x8DWa<\x8Ca1\xD4V[[_a<\x9A\x84\x82\x85\x01a<dV[\x91PP\x92\x91PPV[a<\xAC\x81a2\x8EV[\x82RPPV[_` \x82\x01\x90Pa<\xC5_\x83\x01\x84a<\xA3V[\x92\x91PPV[_\x815a<\xD7\x81a1\xE5V[\x80\x91PP\x91\x90PV[_\x81_\x1B\x90P\x91\x90PV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa=\x16\x84a<\xE0V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[_a=Oa=Ja=E\x84a1\xDCV[a=,V[a1\xDCV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a=h\x82a=5V[a={a=t\x82a=VV[\x83Ta<\xEBV[\x82UPPPV[_\x815a=\x8E\x81a2\x9FV[\x80\x91PP\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa=\xB6\x84a<\xE0V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_a=\xE6a=\xE1a=\xDC\x84a2oV[a=,V[a2oV[\x90P\x91\x90PV[_a=\xF7\x82a=\xCCV[\x90P\x91\x90PV[_a>\x08\x82a=\xEDV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a>!\x82a=\xFEV[a>4a>-\x82a>\x0FV[\x83Ta=\x97V[\x82UPPPV[_\x81\x01_\x83\x01\x80a>K\x81a<\xCBV[\x90Pa>W\x81\x84a=_V[PPP`\x01\x81\x01` \x83\x01\x80a>l\x81a<\xCBV[\x90Pa>x\x81\x84a=_V[PPP`\x02\x81\x01`@\x83\x01\x80a>\x8D\x81a=\x82V[\x90Pa>\x99\x81\x84a>\x18V[PPPPPV[a>\xAA\x82\x82a>;V[PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a>\xCA\x81a>\xAEV[\x82RPPV[_` \x82\x01\x90Pa>\xE3_\x83\x01\x84a>\xC1V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a?-W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a?@Wa??a>\xE9V[[P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a?\xAA\x82a1\xDCV[\x91Pa?\xB5\x83a1\xDCV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a?\xCDWa?\xCCa?sV[[\x92\x91PPV[_`@\x82\x01\x90Pa?\xE6_\x83\x01\x85a3'V[a?\xF3` \x83\x01\x84a3'V[\x93\x92PPPV[_a@\x08` \x84\x01\x84a1\xFBV[\x90P\x92\x91PPV[`\x80\x82\x01a@ _\x83\x01\x83a?\xFAV[a@,_\x85\x01\x82a3zV[Pa@:` \x83\x01\x83a?\xFAV[a@G` \x85\x01\x82a3zV[Pa@U`@\x83\x01\x83a?\xFAV[a@b`@\x85\x01\x82a3zV[Pa@p``\x83\x01\x83a?\xFAV[a@}``\x85\x01\x82a3zV[PPPPV[_`\x80\x82\x01\x90Pa@\x96_\x83\x01\x84a@\x10V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x825`\x01a\x01\0\x03\x836\x03\x03\x81\x12aA\x1EWaA\x1Da@\xF6V[[\x80\x83\x01\x91PP\x92\x91PPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aAFWaAEa@\xF6V[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aAhWaAga@\xFAV[[` \x83\x01\x92P`\x01\x82\x026\x03\x83\x13\x15aA\x84WaA\x83a@\xFEV[[P\x92P\x92\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[_aA\xB3` \x84\x01\x84a2\xB5V[\x90P\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aA\xE3WaA\xE2aA\xC3V[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aB\x0BWaB\naA\xBBV[[`\x01\x82\x026\x03\x83\x13\x15aB!WaB aA\xBFV[[P\x92P\x92\x90PV[_aB4\x83\x85a4ZV[\x93PaBA\x83\x85\x84a7\xE9V[aBJ\x83a1cV[\x84\x01\x90P\x93\x92PPPV[_\x81`\x03\x0B\x90P\x91\x90PV[aBj\x81aBUV[\x81\x14aBtW_\x80\xFD[PV[_\x815\x90PaB\x85\x81aBaV[\x92\x91PPV[_aB\x99` \x84\x01\x84aBwV[\x90P\x92\x91PPV[aB\xAA\x81aBUV[\x82RPPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aB\xCCWaB\xCBaA\xC3V[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aB\xF4WaB\xF3aA\xBBV[[`\x01\x82\x026\x03\x83\x13\x15aC\nWaC\taA\xBFV[[P\x92P\x92\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aC-\x83\x85aC\x12V[\x93PaC:\x83\x85\x84a7\xE9V[aCC\x83a1cV[\x84\x01\x90P\x93\x92PPPV[_a\x01\0\x83\x01aC`_\x84\x01\x84aA\xA5V[aCl_\x86\x01\x82a3\x89V[PaCz` \x84\x01\x84aA\xA5V[aC\x87` \x86\x01\x82a3\x89V[PaC\x95`@\x84\x01\x84aA\xC7V[\x85\x83\x03`@\x87\x01RaC\xA8\x83\x82\x84aB)V[\x92PPPaC\xB9``\x84\x01\x84aA\xC7V[\x85\x83\x03``\x87\x01RaC\xCC\x83\x82\x84aB)V[\x92PPPaC\xDD`\x80\x84\x01\x84aB\x8BV[aC\xEA`\x80\x86\x01\x82aB\xA1V[PaC\xF8`\xA0\x84\x01\x84aA\xC7V[\x85\x83\x03`\xA0\x87\x01RaD\x0B\x83\x82\x84aB)V[\x92PPPaD\x1C`\xC0\x84\x01\x84aB\xB0V[\x85\x83\x03`\xC0\x87\x01RaD/\x83\x82\x84aC\"V[\x92PPPaD@`\xE0\x84\x01\x84aA\xC7V[\x85\x83\x03`\xE0\x87\x01RaDS\x83\x82\x84aB)V[\x92PPP\x80\x91PP\x92\x91PPV[_aDl\x83\x83aCNV[\x90P\x92\x91PPV[_\x825`\x01a\x01\0\x03\x836\x03\x03\x81\x12aD\x90WaD\x8FaA\xC3V[[\x82\x81\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aD\xB3\x83\x85aA\x8CV[\x93P\x83` \x84\x02\x85\x01aD\xC5\x84aA\x9CV[\x80_[\x87\x81\x10\x15aE\x08W\x84\x84\x03\x89RaD\xDF\x82\x84aDtV[aD\xE9\x85\x82aDaV[\x94PaD\xF4\x83aD\x9CV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaD\xC8V[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_aE%\x83\x85a1+V[\x93PaE2\x83\x85\x84a7\xE9V[aE;\x83a1cV[\x84\x01\x90P\x93\x92PPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[_``\x83\x01aEp_\x84\x01\x84aB\xB0V[\x85\x83\x03_\x87\x01RaE\x82\x83\x82\x84aC\"V[\x92PPPaE\x93` \x84\x01\x84aB\xB0V[\x85\x83\x03` \x87\x01RaE\xA6\x83\x82\x84aC\"V[\x92PPPaE\xB7`@\x84\x01\x84aB\xB0V[\x85\x83\x03`@\x87\x01RaE\xCA\x83\x82\x84aC\"V[\x92PPP\x80\x91PP\x92\x91PPV[_aE\xE3\x83\x83aE_V[\x90P\x92\x91PPV[_\x825`\x01``\x03\x836\x03\x03\x81\x12aF\x06WaF\x05aA\xC3V[[\x82\x81\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aF)\x83\x85aEFV[\x93P\x83` \x84\x02\x85\x01aF;\x84aEVV[\x80_[\x87\x81\x10\x15aF~W\x84\x84\x03\x89RaFU\x82\x84aE\xEBV[aF_\x85\x82aE\xD8V[\x94PaFj\x83aF\x12V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaF>V[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_a\x01\0\x82\x01\x90P\x81\x81\x03_\x83\x01RaF\xAA\x81\x8A\x8CaD\xA8V[\x90PaF\xB9` \x83\x01\x89a@\x10V[\x81\x81\x03`\xA0\x83\x01RaF\xCC\x81\x87\x89aE\x1AV[\x90P\x81\x81\x03`\xC0\x83\x01RaF\xE1\x81\x85\x87aF\x1EV[\x90PaF\xF0`\xE0\x83\x01\x84a3'V[\x99\x98PPPPPPPPPV[aG\x06\x81a8\xBFV[\x81\x14aG\x10W_\x80\xFD[PV[_\x81Q\x90PaG!\x81aF\xFDV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aG<WaG;a1\xD4V[[_aGI\x84\x82\x85\x01aG\x13V[\x91PP\x92\x91PPV[_``\x82\x01\x90P\x81\x81\x03_\x83\x01RaGj\x81\x86a1sV[\x90PaGy` \x83\x01\x85a3'V[aG\x86`@\x83\x01\x84a3'V[\x94\x93PPPPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02aG\xEA\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82aG\xAFV[aG\xF4\x86\x83aG\xAFV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[aH\x15\x83a=5V[aH)aH!\x82a=VV[\x84\x84TaG\xBBV[\x82UPPPPV[_\x90V[aH=aH1V[aHH\x81\x84\x84aH\x0CV[PPPV[[\x81\x81\x10\x15aHkWaH`_\x82aH5V[`\x01\x81\x01\x90PaHNV[PPV[`\x1F\x82\x11\x15aH\xB0WaH\x81\x81aG\x8EV[aH\x8A\x84aG\xA0V[\x81\x01` \x85\x10\x15aH\x99W\x81\x90P[aH\xADaH\xA5\x85aG\xA0V[\x83\x01\x82aHMV[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_aH\xD0_\x19\x84`\x08\x02aH\xB5V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_aH\xE8\x83\x83aH\xC1V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[aI\x01\x82a1!V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aI\x1AWaI\x19a7AV[[aI$\x82Ta?\x16V[aI/\x82\x82\x85aHoV[_` \x90P`\x1F\x83\x11`\x01\x81\x14aI`W_\x84\x15aINW\x82\x87\x01Q\x90P[aIX\x85\x82aH\xDDV[\x86UPaI\xBFV[`\x1F\x19\x84\x16aIn\x86aG\x8EV[_[\x82\x81\x10\x15aI\x95W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PaIpV[\x86\x83\x10\x15aI\xB2W\x84\x89\x01QaI\xAE`\x1F\x89\x16\x82aH\xC1V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_aI\xE5\x82aI\xC7V[aI\xEF\x81\x85aI\xD1V[\x93PaI\xFF\x81\x85` \x86\x01a1;V[\x80\x84\x01\x91PP\x92\x91PPV[_aJ\x16\x82\x84aI\xDBV[\x91P\x81\x90P\x92\x91PPV",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361061019b575f3560e01c80634f1ef286116100eb578063976f3eb911610089578063bf9b16c811610063578063bf9b16c8146105ef578063c2b429861461062b578063c3aaaa5a14610655578063f9c670c3146106915761019b565b8063976f3eb914610571578063ad3cb1cc1461059b578063b4722bc4146105c55761019b565b80635bff76d9116100c55780635bff76d9146104a75780637eaac8f2146104e35780638afd0ca31461050d5780639447cfd4146105355761019b565b80634f1ef2861461043957806352d1902d146104555780635b7262961461047f5761019b565b80632c87fea21161015857806341ad069c1161013257806341ad069c1461035d57806346c5bbbd1461039957806347e82295146103d55780634cfb42be146104115761019b565b80632c87fea2146102bd5780632e879d7e146102f957806331ff41c8146103215761019b565b80630d8e6e2c1461019f5780631f81e10f146101c9578063203d0114146101f157806326cf5def1461022d578063281e8bfe146102575780632a38899814610293575b5f80fd5b3480156101aa575f80fd5b506101b36106cd565b6040516101c091906131ab565b60405180910390f35b3480156101d4575f80fd5b506101ef60048036038101906101ea9190613231565b610748565b005b3480156101fc575f80fd5b50610217600480360381019061021291906132c9565b6109a3565b604051610224919061330e565b60405180910390f35b348015610238575f80fd5b50610241610a15565b60405161024e9190613336565b60405180910390f35b348015610262575f80fd5b5061027d6004803603810190610278919061334f565b610a3e565b60405161028a9190613336565b60405180910390f35b34801561029e575f80fd5b506102a7610a6a565b6040516102b49190613336565b60405180910390f35b3480156102c8575f80fd5b506102e360048036038101906102de919061334f565b610a93565b6040516102f091906133d8565b60405180910390f35b348015610304575f80fd5b5061031f600480360381019061031a91906133f1565b610b38565b005b34801561032c575f80fd5b506103476004803603810190610342919061341c565b610d15565b6040516103549190613509565b60405180910390f35b348015610368575f80fd5b50610383600480360381019061037e919061334f565b610f57565b6040516103909190613336565b60405180910390f35b3480156103a4575f80fd5b506103bf60048036038101906103ba919061341c565b610f83565b6040516103cc919061330e565b60405180910390f35b3480156103e0575f80fd5b506103fb60048036038101906103f6919061334f565b610ff7565b6040516104089190613336565b60405180910390f35b34801561041c575f80fd5b5061043760048036038101906104329190613652565b611023565b005b610453600480360381019061044e9190613865565b611215565b005b348015610460575f80fd5b50610469611234565b60405161047691906138d7565b60405180910390f35b34801561048a575f80fd5b506104a560048036038101906104a09190613652565b611265565b005b3480156104b2575f80fd5b506104cd60048036038101906104c8919061334f565b6113cb565b6040516104da9190613998565b60405180910390f35b3480156104ee575f80fd5b506104f7611479565b6040516105049190613998565b60405180910390f35b348015610518575f80fd5b50610533600480360381019061052e91906139b8565b611524565b005b348015610540575f80fd5b5061055b6004803603810190610556919061341c565b6117fe565b604051610568919061330e565b60405180910390f35b34801561057c575f80fd5b50610585611872565b6040516105929190613336565b60405180910390f35b3480156105a6575f80fd5b506105af611883565b6040516105bc91906131ab565b60405180910390f35b3480156105d0575f80fd5b506105d96118bc565b6040516105e69190613336565b60405180910390f35b3480156105fa575f80fd5b506106156004803603810190610610919061334f565b6118e5565b604051610622919061330e565b60405180910390f35b348015610636575f80fd5b5061063f6118f6565b60405161064c9190613336565b60405180910390f35b348015610660575f80fd5b5061067b6004803603810190610676919061334f565b61191f565b6040516106889190613336565b60405180910390f35b34801561069c575f80fd5b506106b760048036038101906106b2919061334f565b61194b565b6040516106c49190613b18565b60405180910390f35b60606040518060400160405280601881526020017f50726f746f636f6c436f6e6669674d756c7469636861696e000000000000000081525061070e5f611b93565b6107186002611b93565b6107215f611b93565b6040516020016107349493929190613c06565b604051602081830303815290604052905090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156107a5573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107c99190613c78565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461083857336040517f21bfda1000000000000000000000000000000000000000000000000000000000815260040161082f9190613cb2565b60405180910390fd5b5f73ffffffffffffffffffffffffffffffffffffffff1681604001602081019061086291906132c9565b73ffffffffffffffffffffffffffffffffffffffff16036108af576040517fd8e1832b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f6108b8611c5d565b9050805f0154830361090157826040517f4595fce20000000000000000000000000000000000000000000000000000000081526004016108f89190613336565b60405180910390fd5b61090a83611c84565b600181600a015f8581526020019081526020015f205f6101000a81548160ff02191690831515021790555081604001602081019061094891906132c9565b73ffffffffffffffffffffffffffffffffffffffff16825f0135847f3f5d57d3508d73eef9374d0ee29403786cb45bfb661d47cad4ad6916e0760c3985602001356040516109969190613336565b60405180910390a4505050565b5f806109ad611c5d565b9050806003015f825f015481526020019081526020015f205f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16915050919050565b5f80610a1f611c5d565b9050806009015f825f015481526020019081526020015f205491505090565b5f610a4882611c84565b610a50611c5d565b6007015f8381526020019081526020015f20549050919050565b5f80610a74611c5d565b9050806006015f825f015481526020019081526020015f205491505090565b610a9b61309b565b610aa482611c84565b610aac611c5d565b600b015f8381526020019081526020015f206040518060600160405290815f820154815260200160018201548152602001600282015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815250509050919050565b60035f610b43611cd1565b9050805f0160089054906101000a900460ff1680610b8b57508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15610bc2576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055505f73ffffffffffffffffffffffffffffffffffffffff16836040016020810190610c3191906132c9565b73ffffffffffffffffffffffffffffffffffffffff1603610c7e576040517fd8e1832b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f610c87611c5d565b90505f815f01549050610c9981611c84565b8482600b015f8381526020019081526020015f208181610cb99190613ea0565b90505050505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051610d089190613ed0565b60405180910390a1505050565b610d1d6130cf565b610d2683611c84565b610d2e611c5d565b6004015f8481526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206040518060800160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600282018054610e3f90613f16565b80601f0160208091040260200160405190810160405280929190818152602001828054610e6b90613f16565b8015610eb65780601f10610e8d57610100808354040283529160200191610eb6565b820191905f5260205f20905b815481529060010190602001808311610e9957829003601f168201915b50505050508152602001600382018054610ecf90613f16565b80601f0160208091040260200160405190810160405280929190818152602001828054610efb90613f16565b8015610f465780601f10610f1d57610100808354040283529160200191610f46565b820191905f5260205f20905b815481529060010190602001808311610f2957829003601f168201915b505050505081525050905092915050565b5f610f6182611c84565b610f69611c5d565b6008015f8381526020019081526020015f20549050919050565b5f610f8d83611c84565b610f95611c5d565b6002015f8481526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f61100182611c84565b611009611c5d565b6009015f8381526020019081526020015f20549050919050565b600161102d611cf8565b67ffffffffffffffff161461106e576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60035f611079611cd1565b9050805f0160089054906101000a900460ff16806110c157508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b156110f8576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff021916908315150217905550600160f86007600881111561115557611154613f46565b5b901b6111619190613fa0565b8b10156111a5578a6040517f77ddbe8100000000000000000000000000000000000000000000000000000000815260040161119c9190613336565b60405180910390fd5b6111b68b8b8b8b8b8b8b8b8b611d1c565b5f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2826040516112009190613ed0565b60405180910390a15050505050505050505050565b61121d6121c4565b611226826122aa565b611230828261239d565b5050565b5f61123d6124bb565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156112c2573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112e69190613c78565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461135557336040517f21bfda1000000000000000000000000000000000000000000000000000000000815260040161134c9190613cb2565b60405180910390fd5b5f61135e611c5d565b90505f815f01549050808b116113ad578a816040517fefd55f670000000000000000000000000000000000000000000000000000000081526004016113a4929190613fd3565b60405180910390fd5b6113be8b8b8b8b8b8b8b8b8b611d1c565b5050505050505050505050565b60606113d682611c84565b6113de611c5d565b6005015f8381526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561146d57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311611424575b50505050509050919050565b60605f611484611c5d565b9050806005015f825f015481526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561151957602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116114d0575b505050505091505090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611581573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115a59190613c78565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461161457336040517f21bfda1000000000000000000000000000000000000000000000000000000000815260040161160b9190613cb2565b60405180910390fd5b5f61161d611c5d565b905061162883611c84565b5f816001015f8581526020019081526020015f208054905090506116856040518060400160405280601081526020017f7075626c696344656372797074696f6e00000000000000000000000000000000815250845f013583612542565b6116c96040518060400160405280600e81526020017f7573657244656372797074696f6e000000000000000000000000000000000000815250846020013583612542565b61170d6040518060400160405280600681526020017f6b6d7347656e0000000000000000000000000000000000000000000000000000815250846040013583612542565b6117516040518060400160405280600381526020017f6d70630000000000000000000000000000000000000000000000000000000000815250846060013583612542565b825f0135826006015f8681526020019081526020015f20819055508260200135826007015f8681526020019081526020015f20819055508260400135826008015f8681526020019081526020015f20819055508260600135826009015f8681526020019081526020015f2081905550837f8ec991e9b0a696c33afa5f2327c050777dfb01687d51237e2d683e9388b64776846040516117f09190614083565b60405180910390a250505050565b5f61180883611c84565b611810611c5d565b6003015f8481526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f61187b611c5d565b5f0154905090565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b5f806118c6611c5d565b9050806008015f825f015481526020019081526020015f205491505090565b5f6118ef82612623565b9050919050565b5f80611900611c5d565b9050806007015f825f015481526020019081526020015f205491505090565b5f61192982611c84565b611931611c5d565b6006015f8381526020019081526020015f20549050919050565b606061195682611c84565b61195e611c5d565b6001015f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020015f905b82821015611b88578382905f5260205f2090600402016040518060800160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600282018054611a6990613f16565b80601f0160208091040260200160405190810160405280929190818152602001828054611a9590613f16565b8015611ae05780601f10611ab757610100808354040283529160200191611ae0565b820191905f5260205f20905b815481529060010190602001808311611ac357829003601f168201915b50505050508152602001600382018054611af990613f16565b80601f0160208091040260200160405190810160405280929190818152602001828054611b2590613f16565b8015611b705780601f10611b4757610100808354040283529160200191611b70565b820191905f5260205f20905b815481529060010190602001808311611b5357829003601f168201915b5050505050815250508152602001906001019061198f565b505050509050919050565b60605f6001611ba1846126b8565b0190505f8167ffffffffffffffff811115611bbf57611bbe613741565b5b6040519080825280601f01601f191660200182016040528015611bf15781602001600182028036833780820191505090505b5090505f82602001820190505b600115611c52578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a8581611c4757611c4661409c565b5b0494505f8503611bfe575b819350505050919050565b5f7f80f3585af86806c5774303b06c1ee640aa83b6ef3e45df49bb26c8524500c200905090565b611c8d81612623565b611cce57806040517f77ddbe81000000000000000000000000000000000000000000000000000000008152600401611cc59190613336565b60405180910390fd5b50565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b5f611d01611cd1565b5f015f9054906101000a900467ffffffffffffffff16905090565b5f73ffffffffffffffffffffffffffffffffffffffff16816040016020810190611d4691906132c9565b73ffffffffffffffffffffffffffffffffffffffff1603611d93576040517fd8e1832b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f8888905003611dcf576040517f068c8d4000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60ff8016888890501115611e22578787905060ff80166040517f16a72778000000000000000000000000000000000000000000000000000000008152600401611e19929190613fd3565b60405180910390fd5b611e686040518060400160405280601081526020017f7075626c696344656372797074696f6e00000000000000000000000000000000815250875f01358a8a9050612542565b611eaf6040518060400160405280600e81526020017f7573657244656372797074696f6e00000000000000000000000000000000000081525087602001358a8a9050612542565b611ef66040518060400160405280600681526020017f6b6d7347656e000000000000000000000000000000000000000000000000000081525087604001358a8a9050612542565b611f3d6040518060400160405280600381526020017f6d7063000000000000000000000000000000000000000000000000000000000081525087606001358a8a9050612542565b5f611f46611c5d565b905089815f01819055505f5b898990508110156120ae57368a8a83818110611f7157611f706140c9565b5b9050602002810190611f839190614102565b90506120a08c6040518060800160405280845f016020810190611fa691906132c9565b73ffffffffffffffffffffffffffffffffffffffff168152602001846020016020810190611fd491906132c9565b73ffffffffffffffffffffffffffffffffffffffff168152602001848060400190611fff919061412a565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050508152602001848060600190612056919061412a565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050815250612809565b508080600101915050611f52565b50865f0135816006015f8c81526020019081526020015f20819055508660200135816007015f8c81526020019081526020015f20819055508660400135816008015f8c81526020019081526020015f20819055508660600135816009015f8c81526020019081526020015f20819055508181600b015f8c81526020019081526020015f20818161213e9190613ea0565b90505081604001602081019061215491906132c9565b73ffffffffffffffffffffffffffffffffffffffff16825f01358b7f33d3e2406cc61c76741a8c7fee27b520ea998661cace49c1523369e7fd340f168c8c8c8c8c8c8c8c602001356040516121b0989796959493929190614690565b60405180910390a450505050505050505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff16148061227157507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16612258612d77565b73ffffffffffffffffffffffffffffffffffffffff1614155b156122a8576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612307573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061232b9190613c78565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461239a57336040517f21bfda100000000000000000000000000000000000000000000000000000000081526004016123919190613cb2565b60405180910390fd5b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa92505050801561240557506040513d601f19601f820116820180604052508101906124029190614727565b60015b61244657816040517f4c9c8ce300000000000000000000000000000000000000000000000000000000815260040161243d9190613cb2565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b81146124ac57806040517faa1d49a40000000000000000000000000000000000000000000000000000000081526004016124a391906138d7565b60405180910390fd5b6124b68383612dca565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff1614612540576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f820361258657826040517f36bfb60e00000000000000000000000000000000000000000000000000000000815260040161257d91906131ab565b60405180910390fd5b60ff80168211156125d557828260ff80166040517f22ba52db0000000000000000000000000000000000000000000000000000000081526004016125cc93929190614752565b60405180910390fd5b8082111561261e578282826040517fcaa814a300000000000000000000000000000000000000000000000000000000815260040161261593929190614752565b60405180910390fd5b505050565b5f8061262d611c5d565b9050600160f86007600881111561264757612646613f46565b5b901b6126539190613fa0565b83101580156126655750805f01548311155b801561268757505f816001015f8581526020019081526020015f208054905014155b80156126b0575080600a015f8481526020019081526020015f205f9054906101000a900460ff16155b915050919050565b5f805f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f0100000000000000008310612714577a184f03e93ff9f4daa797ed6e38ed64bf6a1f010000000000000000838161270a5761270961409c565b5b0492506040810190505b6d04ee2d6d415b85acef81000000008310612751576d04ee2d6d415b85acef810000000083816127475761274661409c565b5b0492506020810190505b662386f26fc10000831061278057662386f26fc1000083816127765761277561409c565b5b0492506010810190505b6305f5e10083106127a9576305f5e100838161279f5761279e61409c565b5b0492506008810190505b61271083106127ce5761271083816127c4576127c361409c565b5b0492506004810190505b606483106127f157606483816127e7576127e661409c565b5b0492506002810190505b600a8310612800576001810190505b80915050919050565b5f612812611c5d565b90505f73ffffffffffffffffffffffffffffffffffffffff16825f015173ffffffffffffffffffffffffffffffffffffffff160361287c576040517f8466804a00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f73ffffffffffffffffffffffffffffffffffffffff16826020015173ffffffffffffffffffffffffffffffffffffffff16036128e5576040517f2deccf4d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b806002015f8481526020019081526020015f205f835f015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff161561298857815f01516040517fd18c4ff000000000000000000000000000000000000000000000000000000000815260040161297f9190613cb2565b60405180910390fd5b806003015f8481526020019081526020015f205f836020015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615612a2d5781602001516040517ff51af6bb000000000000000000000000000000000000000000000000000000008152600401612a249190613cb2565b60405180910390fd5b806001015f8481526020019081526020015f2082908060018154018082558091505060019003905f5260205f2090600402015f909190919091505f820151815f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506020820151816001015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506040820151816002019081612b0691906148f8565b506060820151816003019081612b1c91906148f8565b5050506001816002015f8581526020019081526020015f205f845f015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055506001816003015f8581526020019081526020015f205f846020015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555081816004015f8581526020019081526020015f205f845f015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f820151815f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506020820151816001015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506040820151816002019081612ce391906148f8565b506060820151816003019081612cf991906148f8565b50905050806005015f8481526020019081526020015f208260200151908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550505050565b5f612da37f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b612e3c565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b612dd382612e45565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f81511115612e2f57612e298282612f0e565b50612e38565b612e37612f8e565b5b5050565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b03612ea057806040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401612e979190613cb2565b60405180910390fd5b80612ecc7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b612e3c565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f808473ffffffffffffffffffffffffffffffffffffffff1684604051612f379190614a0b565b5f60405180830381855af49150503d805f8114612f6f576040519150601f19603f3d011682016040523d82523d5f602084013e612f74565b606091505b5091509150612f84858383612fca565b9250505092915050565b5f341115612fc8576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b606082612fdf57612fda82613057565b61304f565b5f825114801561300557505f8473ffffffffffffffffffffffffffffffffffffffff163b145b1561304757836040517f9996b31500000000000000000000000000000000000000000000000000000000815260040161303e9190613cb2565b60405180910390fd5b819050613050565b5b9392505050565b5f815111156130695780518082602001fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60405180606001604052805f81526020015f81526020015f73ffffffffffffffffffffffffffffffffffffffff1681525090565b60405180608001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff16815260200160608152602001606081525090565b5f81519050919050565b5f82825260208201905092915050565b5f5b8381101561315857808201518184015260208101905061313d565b5f8484015250505050565b5f601f19601f8301169050919050565b5f61317d82613121565b613187818561312b565b935061319781856020860161313b565b6131a081613163565b840191505092915050565b5f6020820190508181035f8301526131c38184613173565b905092915050565b5f604051905090565b5f80fd5b5f80fd5b5f819050919050565b6131ee816131dc565b81146131f8575f80fd5b50565b5f81359050613209816131e5565b92915050565b5f80fd5b5f606082840312156132285761322761320f565b5b81905092915050565b5f8060808385031215613247576132466131d4565b5b5f613254858286016131fb565b925050602061326585828601613213565b9150509250929050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6132988261326f565b9050919050565b6132a88161328e565b81146132b2575f80fd5b50565b5f813590506132c38161329f565b92915050565b5f602082840312156132de576132dd6131d4565b5b5f6132eb848285016132b5565b91505092915050565b5f8115159050919050565b613308816132f4565b82525050565b5f6020820190506133215f8301846132ff565b92915050565b613330816131dc565b82525050565b5f6020820190506133495f830184613327565b92915050565b5f60208284031215613364576133636131d4565b5b5f613371848285016131fb565b91505092915050565b613383816131dc565b82525050565b6133928161328e565b82525050565b606082015f8201516133ac5f85018261337a565b5060208201516133bf602085018261337a565b5060408201516133d26040850182613389565b50505050565b5f6060820190506133eb5f830184613398565b92915050565b5f60608284031215613406576134056131d4565b5b5f61341384828501613213565b91505092915050565b5f8060408385031215613432576134316131d4565b5b5f61343f858286016131fb565b9250506020613450858286016132b5565b9150509250929050565b5f82825260208201905092915050565b5f61347482613121565b61347e818561345a565b935061348e81856020860161313b565b61349781613163565b840191505092915050565b5f608083015f8301516134b75f860182613389565b5060208301516134ca6020860182613389565b50604083015184820360408601526134e2828261346a565b915050606083015184820360608601526134fc828261346a565b9150508091505092915050565b5f6020820190508181035f83015261352181846134a2565b905092915050565b5f80fd5b5f80fd5b5f80fd5b5f8083601f84011261354a57613549613529565b5b8235905067ffffffffffffffff8111156135675761356661352d565b5b60208301915083602082028301111561358357613582613531565b5b9250929050565b5f6080828403121561359f5761359e61320f565b5b81905092915050565b5f8083601f8401126135bd576135bc613529565b5b8235905067ffffffffffffffff8111156135da576135d961352d565b5b6020830191508360018202830111156135f6576135f5613531565b5b9250929050565b5f8083601f84011261361257613611613529565b5b8235905067ffffffffffffffff81111561362f5761362e61352d565b5b60208301915083602082028301111561364b5761364a613531565b5b9250929050565b5f805f805f805f805f6101608a8c0312156136705761366f6131d4565b5b5f61367d8c828d016131fb565b99505060208a013567ffffffffffffffff81111561369e5761369d6131d8565b5b6136aa8c828d01613535565b985098505060406136bd8c828d0161358a565b96505060c08a013567ffffffffffffffff8111156136de576136dd6131d8565b5b6136ea8c828d016135a8565b955095505060e08a013567ffffffffffffffff81111561370d5761370c6131d8565b5b6137198c828d016135fd565b935093505061010061372d8c828d01613213565b9150509295985092959850929598565b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b61377782613163565b810181811067ffffffffffffffff8211171561379657613795613741565b5b80604052505050565b5f6137a86131cb565b90506137b4828261376e565b919050565b5f67ffffffffffffffff8211156137d3576137d2613741565b5b6137dc82613163565b9050602081019050919050565b828183375f83830152505050565b5f613809613804846137b9565b61379f565b9050828152602081018484840111156138255761382461373d565b5b6138308482856137e9565b509392505050565b5f82601f83011261384c5761384b613529565b5b813561385c8482602086016137f7565b91505092915050565b5f806040838503121561387b5761387a6131d4565b5b5f613888858286016132b5565b925050602083013567ffffffffffffffff8111156138a9576138a86131d8565b5b6138b585828601613838565b9150509250929050565b5f819050919050565b6138d1816138bf565b82525050565b5f6020820190506138ea5f8301846138c8565b92915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f6139248383613389565b60208301905092915050565b5f602082019050919050565b5f613946826138f0565b61395081856138fa565b935061395b8361390a565b805f5b8381101561398b5781516139728882613919565b975061397d83613930565b92505060018101905061395e565b5085935050505092915050565b5f6020820190508181035f8301526139b0818461393c565b905092915050565b5f8060a083850312156139ce576139cd6131d4565b5b5f6139db858286016131fb565b92505060206139ec8582860161358a565b9150509250929050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f608083015f830151613a345f860182613389565b506020830151613a476020860182613389565b5060408301518482036040860152613a5f828261346a565b91505060608301518482036060860152613a79828261346a565b9150508091505092915050565b5f613a918383613a1f565b905092915050565b5f602082019050919050565b5f613aaf826139f6565b613ab98185613a00565b935083602082028501613acb85613a10565b805f5b85811015613b065784840389528151613ae78582613a86565b9450613af283613a99565b925060208a01995050600181019050613ace565b50829750879550505050505092915050565b5f6020820190508181035f830152613b308184613aa5565b905092915050565b5f81905092915050565b5f613b4c82613121565b613b568185613b38565b9350613b6681856020860161313b565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f613ba6600283613b38565b9150613bb182613b72565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f613bf0600183613b38565b9150613bfb82613bbc565b600182019050919050565b5f613c118287613b42565b9150613c1c82613b9a565b9150613c288286613b42565b9150613c3382613be4565b9150613c3f8285613b42565b9150613c4a82613be4565b9150613c568284613b42565b915081905095945050505050565b5f81519050613c728161329f565b92915050565b5f60208284031215613c8d57613c8c6131d4565b5b5f613c9a84828501613c64565b91505092915050565b613cac8161328e565b82525050565b5f602082019050613cc55f830184613ca3565b92915050565b5f8135613cd7816131e5565b80915050919050565b5f815f1b9050919050565b5f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff613d1684613ce0565b9350801983169250808416831791505092915050565b5f819050919050565b5f613d4f613d4a613d45846131dc565b613d2c565b6131dc565b9050919050565b5f819050919050565b613d6882613d35565b613d7b613d7482613d56565b8354613ceb565b8255505050565b5f8135613d8e8161329f565b80915050919050565b5f73ffffffffffffffffffffffffffffffffffffffff613db684613ce0565b9350801983169250808416831791505092915050565b5f613de6613de1613ddc8461326f565b613d2c565b61326f565b9050919050565b5f613df782613dcc565b9050919050565b5f613e0882613ded565b9050919050565b5f819050919050565b613e2182613dfe565b613e34613e2d82613e0f565b8354613d97565b8255505050565b5f81015f830180613e4b81613ccb565b9050613e578184613d5f565b505050600181016020830180613e6c81613ccb565b9050613e788184613d5f565b505050600281016040830180613e8d81613d82565b9050613e998184613e18565b5050505050565b613eaa8282613e3b565b5050565b5f67ffffffffffffffff82169050919050565b613eca81613eae565b82525050565b5f602082019050613ee35f830184613ec1565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f6002820490506001821680613f2d57607f821691505b602082108103613f4057613f3f613ee9565b5b50919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f613faa826131dc565b9150613fb5836131dc565b9250828201905080821115613fcd57613fcc613f73565b5b92915050565b5f604082019050613fe65f830185613327565b613ff36020830184613327565b9392505050565b5f61400860208401846131fb565b905092915050565b608082016140205f830183613ffa565b61402c5f85018261337a565b5061403a6020830183613ffa565b614047602085018261337a565b506140556040830183613ffa565b614062604085018261337a565b506140706060830183613ffa565b61407d606085018261337a565b50505050565b5f6080820190506140965f830184614010565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b5f823560016101000383360303811261411e5761411d6140f6565b5b80830191505092915050565b5f8083356001602003843603038112614146576141456140f6565b5b80840192508235915067ffffffffffffffff821115614168576141676140fa565b5b602083019250600182023603831315614184576141836140fe565b5b509250929050565b5f82825260208201905092915050565b5f819050919050565b5f6141b360208401846132b5565b905092915050565b5f80fd5b5f80fd5b5f80fd5b5f80833560016020038436030381126141e3576141e26141c3565b5b83810192508235915060208301925067ffffffffffffffff82111561420b5761420a6141bb565b5b600182023603831315614221576142206141bf565b5b509250929050565b5f614234838561345a565b93506142418385846137e9565b61424a83613163565b840190509392505050565b5f8160030b9050919050565b61426a81614255565b8114614274575f80fd5b50565b5f8135905061428581614261565b92915050565b5f6142996020840184614277565b905092915050565b6142aa81614255565b82525050565b5f80833560016020038436030381126142cc576142cb6141c3565b5b83810192508235915060208301925067ffffffffffffffff8211156142f4576142f36141bb565b5b60018202360383131561430a576143096141bf565b5b509250929050565b5f82825260208201905092915050565b5f61432d8385614312565b935061433a8385846137e9565b61434383613163565b840190509392505050565b5f61010083016143605f8401846141a5565b61436c5f860182613389565b5061437a60208401846141a5565b6143876020860182613389565b5061439560408401846141c7565b85830360408701526143a8838284614229565b925050506143b960608401846141c7565b85830360608701526143cc838284614229565b925050506143dd608084018461428b565b6143ea60808601826142a1565b506143f860a08401846141c7565b85830360a087015261440b838284614229565b9250505061441c60c08401846142b0565b85830360c087015261442f838284614322565b9250505061444060e08401846141c7565b85830360e0870152614453838284614229565b925050508091505092915050565b5f61446c838361434e565b905092915050565b5f82356001610100038336030381126144905761448f6141c3565b5b82810191505092915050565b5f602082019050919050565b5f6144b3838561418c565b9350836020840285016144c58461419c565b805f5b878110156145085784840389526144df8284614474565b6144e98582614461565b94506144f48361449c565b925060208a019950506001810190506144c8565b50829750879450505050509392505050565b5f614525838561312b565b93506145328385846137e9565b61453b83613163565b840190509392505050565b5f82825260208201905092915050565b5f819050919050565b5f606083016145705f8401846142b0565b8583035f870152614582838284614322565b9250505061459360208401846142b0565b85830360208701526145a6838284614322565b925050506145b760408401846142b0565b85830360408701526145ca838284614322565b925050508091505092915050565b5f6145e3838361455f565b905092915050565b5f82356001606003833603038112614606576146056141c3565b5b82810191505092915050565b5f602082019050919050565b5f6146298385614546565b93508360208402850161463b84614556565b805f5b8781101561467e57848403895261465582846145eb565b61465f85826145d8565b945061466a83614612565b925060208a0199505060018101905061463e565b50829750879450505050509392505050565b5f610100820190508181035f8301526146aa818a8c6144a8565b90506146b96020830189614010565b81810360a08301526146cc81878961451a565b905081810360c08301526146e181858761461e565b90506146f060e0830184613327565b9998505050505050505050565b614706816138bf565b8114614710575f80fd5b50565b5f81519050614721816146fd565b92915050565b5f6020828403121561473c5761473b6131d4565b5b5f61474984828501614713565b91505092915050565b5f6060820190508181035f83015261476a8186613173565b90506147796020830185613327565b6147866040830184613327565b949350505050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f600883026147ea7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff826147af565b6147f486836147af565b95508019841693508086168417925050509392505050565b61481583613d35565b61482961482182613d56565b8484546147bb565b825550505050565b5f90565b61483d614831565b61484881848461480c565b505050565b5b8181101561486b576148605f82614835565b60018101905061484e565b5050565b601f8211156148b0576148818161478e565b61488a846147a0565b81016020851015614899578190505b6148ad6148a5856147a0565b83018261484d565b50505b505050565b5f82821c905092915050565b5f6148d05f19846008026148b5565b1980831691505092915050565b5f6148e883836148c1565b9150826002028217905092915050565b61490182613121565b67ffffffffffffffff81111561491a57614919613741565b5b6149248254613f16565b61492f82828561486f565b5f60209050601f831160018114614960575f841561494e578287015190505b61495885826148dd565b8655506149bf565b601f19841661496e8661478e565b5f5b8281101561499557848901518255600182019150602085019450602081019050614970565b868310156149b257848901516149ae601f8916826148c1565b8355505b6001600288020188555050505b505050505050565b5f81519050919050565b5f81905092915050565b5f6149e5826149c7565b6149ef81856149d1565b93506149ff81856020860161313b565b80840191505092915050565b5f614a1682846149db565b91508190509291505056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01\x9BW_5`\xE0\x1C\x80cO\x1E\xF2\x86\x11a\0\xEBW\x80c\x97o>\xB9\x11a\0\x89W\x80c\xBF\x9B\x16\xC8\x11a\0cW\x80c\xBF\x9B\x16\xC8\x14a\x05\xEFW\x80c\xC2\xB4)\x86\x14a\x06+W\x80c\xC3\xAA\xAAZ\x14a\x06UW\x80c\xF9\xC6p\xC3\x14a\x06\x91Wa\x01\x9BV[\x80c\x97o>\xB9\x14a\x05qW\x80c\xAD<\xB1\xCC\x14a\x05\x9BW\x80c\xB4r+\xC4\x14a\x05\xC5Wa\x01\x9BV[\x80c[\xFFv\xD9\x11a\0\xC5W\x80c[\xFFv\xD9\x14a\x04\xA7W\x80c~\xAA\xC8\xF2\x14a\x04\xE3W\x80c\x8A\xFD\x0C\xA3\x14a\x05\rW\x80c\x94G\xCF\xD4\x14a\x055Wa\x01\x9BV[\x80cO\x1E\xF2\x86\x14a\x049W\x80cR\xD1\x90-\x14a\x04UW\x80c[rb\x96\x14a\x04\x7FWa\x01\x9BV[\x80c,\x87\xFE\xA2\x11a\x01XW\x80cA\xAD\x06\x9C\x11a\x012W\x80cA\xAD\x06\x9C\x14a\x03]W\x80cF\xC5\xBB\xBD\x14a\x03\x99W\x80cG\xE8\"\x95\x14a\x03\xD5W\x80cL\xFBB\xBE\x14a\x04\x11Wa\x01\x9BV[\x80c,\x87\xFE\xA2\x14a\x02\xBDW\x80c.\x87\x9D~\x14a\x02\xF9W\x80c1\xFFA\xC8\x14a\x03!Wa\x01\x9BV[\x80c\r\x8En,\x14a\x01\x9FW\x80c\x1F\x81\xE1\x0F\x14a\x01\xC9W\x80c =\x01\x14\x14a\x01\xF1W\x80c&\xCF]\xEF\x14a\x02-W\x80c(\x1E\x8B\xFE\x14a\x02WW\x80c*8\x89\x98\x14a\x02\x93W[_\x80\xFD[4\x80\x15a\x01\xAAW_\x80\xFD[Pa\x01\xB3a\x06\xCDV[`@Qa\x01\xC0\x91\x90a1\xABV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xD4W_\x80\xFD[Pa\x01\xEF`\x04\x806\x03\x81\x01\x90a\x01\xEA\x91\x90a21V[a\x07HV[\0[4\x80\x15a\x01\xFCW_\x80\xFD[Pa\x02\x17`\x04\x806\x03\x81\x01\x90a\x02\x12\x91\x90a2\xC9V[a\t\xA3V[`@Qa\x02$\x91\x90a3\x0EV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x028W_\x80\xFD[Pa\x02Aa\n\x15V[`@Qa\x02N\x91\x90a36V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02bW_\x80\xFD[Pa\x02}`\x04\x806\x03\x81\x01\x90a\x02x\x91\x90a3OV[a\n>V[`@Qa\x02\x8A\x91\x90a36V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x9EW_\x80\xFD[Pa\x02\xA7a\njV[`@Qa\x02\xB4\x91\x90a36V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xC8W_\x80\xFD[Pa\x02\xE3`\x04\x806\x03\x81\x01\x90a\x02\xDE\x91\x90a3OV[a\n\x93V[`@Qa\x02\xF0\x91\x90a3\xD8V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x04W_\x80\xFD[Pa\x03\x1F`\x04\x806\x03\x81\x01\x90a\x03\x1A\x91\x90a3\xF1V[a\x0B8V[\0[4\x80\x15a\x03,W_\x80\xFD[Pa\x03G`\x04\x806\x03\x81\x01\x90a\x03B\x91\x90a4\x1CV[a\r\x15V[`@Qa\x03T\x91\x90a5\tV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03hW_\x80\xFD[Pa\x03\x83`\x04\x806\x03\x81\x01\x90a\x03~\x91\x90a3OV[a\x0FWV[`@Qa\x03\x90\x91\x90a36V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xA4W_\x80\xFD[Pa\x03\xBF`\x04\x806\x03\x81\x01\x90a\x03\xBA\x91\x90a4\x1CV[a\x0F\x83V[`@Qa\x03\xCC\x91\x90a3\x0EV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xE0W_\x80\xFD[Pa\x03\xFB`\x04\x806\x03\x81\x01\x90a\x03\xF6\x91\x90a3OV[a\x0F\xF7V[`@Qa\x04\x08\x91\x90a36V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\x1CW_\x80\xFD[Pa\x047`\x04\x806\x03\x81\x01\x90a\x042\x91\x90a6RV[a\x10#V[\0[a\x04S`\x04\x806\x03\x81\x01\x90a\x04N\x91\x90a8eV[a\x12\x15V[\0[4\x80\x15a\x04`W_\x80\xFD[Pa\x04ia\x124V[`@Qa\x04v\x91\x90a8\xD7V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\x8AW_\x80\xFD[Pa\x04\xA5`\x04\x806\x03\x81\x01\x90a\x04\xA0\x91\x90a6RV[a\x12eV[\0[4\x80\x15a\x04\xB2W_\x80\xFD[Pa\x04\xCD`\x04\x806\x03\x81\x01\x90a\x04\xC8\x91\x90a3OV[a\x13\xCBV[`@Qa\x04\xDA\x91\x90a9\x98V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\xEEW_\x80\xFD[Pa\x04\xF7a\x14yV[`@Qa\x05\x04\x91\x90a9\x98V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\x18W_\x80\xFD[Pa\x053`\x04\x806\x03\x81\x01\x90a\x05.\x91\x90a9\xB8V[a\x15$V[\0[4\x80\x15a\x05@W_\x80\xFD[Pa\x05[`\x04\x806\x03\x81\x01\x90a\x05V\x91\x90a4\x1CV[a\x17\xFEV[`@Qa\x05h\x91\x90a3\x0EV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05|W_\x80\xFD[Pa\x05\x85a\x18rV[`@Qa\x05\x92\x91\x90a36V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\xA6W_\x80\xFD[Pa\x05\xAFa\x18\x83V[`@Qa\x05\xBC\x91\x90a1\xABV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\xD0W_\x80\xFD[Pa\x05\xD9a\x18\xBCV[`@Qa\x05\xE6\x91\x90a36V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\xFAW_\x80\xFD[Pa\x06\x15`\x04\x806\x03\x81\x01\x90a\x06\x10\x91\x90a3OV[a\x18\xE5V[`@Qa\x06\"\x91\x90a3\x0EV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x066W_\x80\xFD[Pa\x06?a\x18\xF6V[`@Qa\x06L\x91\x90a36V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06`W_\x80\xFD[Pa\x06{`\x04\x806\x03\x81\x01\x90a\x06v\x91\x90a3OV[a\x19\x1FV[`@Qa\x06\x88\x91\x90a36V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06\x9CW_\x80\xFD[Pa\x06\xB7`\x04\x806\x03\x81\x01\x90a\x06\xB2\x91\x90a3OV[a\x19KV[`@Qa\x06\xC4\x91\x90a;\x18V[`@Q\x80\x91\x03\x90\xF3[```@Q\x80`@\x01`@R\x80`\x18\x81R` \x01\x7FProtocolConfigMultichain\0\0\0\0\0\0\0\0\x81RPa\x07\x0E_a\x1B\x93V[a\x07\x18`\x02a\x1B\x93V[a\x07!_a\x1B\x93V[`@Q` \x01a\x074\x94\x93\x92\x91\x90a<\x06V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xA5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xC9\x91\x90a<xV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x088W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08/\x91\x90a<\xB2V[`@Q\x80\x91\x03\x90\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`@\x01` \x81\x01\x90a\x08b\x91\x90a2\xC9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x08\xAFW`@Q\x7F\xD8\xE1\x83+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x08\xB8a\x1C]V[\x90P\x80_\x01T\x83\x03a\t\x01W\x82`@Q\x7FE\x95\xFC\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xF8\x91\x90a36V[`@Q\x80\x91\x03\x90\xFD[a\t\n\x83a\x1C\x84V[`\x01\x81`\n\x01_\x85\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x81`@\x01` \x81\x01\x90a\tH\x91\x90a2\xC9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82_\x015\x84\x7F?]W\xD3P\x8Ds\xEE\xF97M\x0E\xE2\x94\x03xl\xB4[\xFBf\x1DG\xCA\xD4\xADi\x16\xE0v\x0C9\x85` \x015`@Qa\t\x96\x91\x90a36V[`@Q\x80\x91\x03\x90\xA4PPPV[_\x80a\t\xADa\x1C]V[\x90P\x80`\x03\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ _\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[_\x80a\n\x1Fa\x1C]V[\x90P\x80`\t\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ T\x91PP\x90V[_a\nH\x82a\x1C\x84V[a\nPa\x1C]V[`\x07\x01_\x83\x81R` \x01\x90\x81R` \x01_ T\x90P\x91\x90PV[_\x80a\nta\x1C]V[\x90P\x80`\x06\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ T\x91PP\x90V[a\n\x9Ba0\x9BV[a\n\xA4\x82a\x1C\x84V[a\n\xACa\x1C]V[`\x0B\x01_\x83\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x91\x90PV[`\x03_a\x0BCa\x1C\xD1V[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x0B\x8BWP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x0B\xC2W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`@\x01` \x81\x01\x90a\x0C1\x91\x90a2\xC9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0C~W`@Q\x7F\xD8\xE1\x83+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0C\x87a\x1C]V[\x90P_\x81_\x01T\x90Pa\x0C\x99\x81a\x1C\x84V[\x84\x82`\x0B\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x81a\x0C\xB9\x91\x90a>\xA0V[\x90PPPP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\r\x08\x91\x90a>\xD0V[`@Q\x80\x91\x03\x90\xA1PPPV[a\r\x1Da0\xCFV[a\r&\x83a\x1C\x84V[a\r.a\x1C]V[`\x04\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`\x80\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01\x80Ta\x0E?\x90a?\x16V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0Ek\x90a?\x16V[\x80\x15a\x0E\xB6W\x80`\x1F\x10a\x0E\x8DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0E\xB6V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0E\x99W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\x0E\xCF\x90a?\x16V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0E\xFB\x90a?\x16V[\x80\x15a\x0FFW\x80`\x1F\x10a\x0F\x1DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0FFV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0F)W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x92\x91PPV[_a\x0Fa\x82a\x1C\x84V[a\x0Fia\x1C]V[`\x08\x01_\x83\x81R` \x01\x90\x81R` \x01_ T\x90P\x91\x90PV[_a\x0F\x8D\x83a\x1C\x84V[a\x0F\x95a\x1C]V[`\x02\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[_a\x10\x01\x82a\x1C\x84V[a\x10\ta\x1C]V[`\t\x01_\x83\x81R` \x01\x90\x81R` \x01_ T\x90P\x91\x90PV[`\x01a\x10-a\x1C\xF8V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x10nW`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03_a\x10ya\x1C\xD1V[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x10\xC1WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x10\xF8W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01`\xF8`\x07`\x08\x81\x11\x15a\x11UWa\x11Ta?FV[[\x90\x1Ba\x11a\x91\x90a?\xA0V[\x8B\x10\x15a\x11\xA5W\x8A`@Q\x7Fw\xDD\xBE\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\x9C\x91\x90a36V[`@Q\x80\x91\x03\x90\xFD[a\x11\xB6\x8B\x8B\x8B\x8B\x8B\x8B\x8B\x8B\x8Ba\x1D\x1CV[_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x12\0\x91\x90a>\xD0V[`@Q\x80\x91\x03\x90\xA1PPPPPPPPPPPV[a\x12\x1Da!\xC4V[a\x12&\x82a\"\xAAV[a\x120\x82\x82a#\x9DV[PPV[_a\x12=a$\xBBV[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xC2W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xE6\x91\x90a<xV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x13UW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13L\x91\x90a<\xB2V[`@Q\x80\x91\x03\x90\xFD[_a\x13^a\x1C]V[\x90P_\x81_\x01T\x90P\x80\x8B\x11a\x13\xADW\x8A\x81`@Q\x7F\xEF\xD5_g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13\xA4\x92\x91\x90a?\xD3V[`@Q\x80\x91\x03\x90\xFD[a\x13\xBE\x8B\x8B\x8B\x8B\x8B\x8B\x8B\x8B\x8Ba\x1D\x1CV[PPPPPPPPPPPV[``a\x13\xD6\x82a\x1C\x84V[a\x13\xDEa\x1C]V[`\x05\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x14mW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x14$W[PPPPP\x90P\x91\x90PV[``_a\x14\x84a\x1C]V[\x90P\x80`\x05\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x15\x19W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x14\xD0W[PPPPP\x91PP\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\x81W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xA5\x91\x90a<xV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x16\x14W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16\x0B\x91\x90a<\xB2V[`@Q\x80\x91\x03\x90\xFD[_a\x16\x1Da\x1C]V[\x90Pa\x16(\x83a\x1C\x84V[_\x81`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90Pa\x16\x85`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01\x7FpublicDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x84_\x015\x83a%BV[a\x16\xC9`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01\x7FuserDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x84` \x015\x83a%BV[a\x17\r`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7FkmsGen\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x84`@\x015\x83a%BV[a\x17Q`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01\x7Fmpc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x84``\x015\x83a%BV[\x82_\x015\x82`\x06\x01_\x86\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82` \x015\x82`\x07\x01_\x86\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82`@\x015\x82`\x08\x01_\x86\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82``\x015\x82`\t\x01_\x86\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x83\x7F\x8E\xC9\x91\xE9\xB0\xA6\x96\xC3:\xFA_#'\xC0Pw}\xFB\x01h}Q#~-h>\x93\x88\xB6Gv\x84`@Qa\x17\xF0\x91\x90a@\x83V[`@Q\x80\x91\x03\x90\xA2PPPPV[_a\x18\x08\x83a\x1C\x84V[a\x18\x10a\x1C]V[`\x03\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[_a\x18{a\x1C]V[_\x01T\x90P\x90V[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[_\x80a\x18\xC6a\x1C]V[\x90P\x80`\x08\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ T\x91PP\x90V[_a\x18\xEF\x82a&#V[\x90P\x91\x90PV[_\x80a\x19\0a\x1C]V[\x90P\x80`\x07\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ T\x91PP\x90V[_a\x19)\x82a\x1C\x84V[a\x191a\x1C]V[`\x06\x01_\x83\x81R` \x01\x90\x81R` \x01_ T\x90P\x91\x90PV[``a\x19V\x82a\x1C\x84V[a\x19^a\x1C]V[`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x1B\x88W\x83\x82\x90_R` _ \x90`\x04\x02\x01`@Q\x80`\x80\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01\x80Ta\x1Ai\x90a?\x16V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1A\x95\x90a?\x16V[\x80\x15a\x1A\xE0W\x80`\x1F\x10a\x1A\xB7Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1A\xE0V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1A\xC3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\x1A\xF9\x90a?\x16V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1B%\x90a?\x16V[\x80\x15a\x1BpW\x80`\x1F\x10a\x1BGWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1BpV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1BSW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x19\x8FV[PPPP\x90P\x91\x90PV[``_`\x01a\x1B\xA1\x84a&\xB8V[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B\xBFWa\x1B\xBEa7AV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1B\xF1W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x01\x82\x01\x90P[`\x01\x15a\x1CRW\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a\x1CGWa\x1CFa@\x9CV[[\x04\x94P_\x85\x03a\x1B\xFEW[\x81\x93PPPP\x91\x90PV[_\x7F\x80\xF3XZ\xF8h\x06\xC5wC\x03\xB0l\x1E\xE6@\xAA\x83\xB6\xEF>E\xDFI\xBB&\xC8RE\0\xC2\0\x90P\x90V[a\x1C\x8D\x81a&#V[a\x1C\xCEW\x80`@Q\x7Fw\xDD\xBE\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1C\xC5\x91\x90a36V[`@Q\x80\x91\x03\x90\xFD[PV[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[_a\x1D\x01a\x1C\xD1V[_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`@\x01` \x81\x01\x90a\x1DF\x91\x90a2\xC9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x1D\x93W`@Q\x7F\xD8\xE1\x83+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x88\x88\x90P\x03a\x1D\xCFW`@Q\x7F\x06\x8C\x8D@\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x80\x16\x88\x88\x90P\x11\x15a\x1E\"W\x87\x87\x90P`\xFF\x80\x16`@Q\x7F\x16\xA7'x\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1E\x19\x92\x91\x90a?\xD3V[`@Q\x80\x91\x03\x90\xFD[a\x1Eh`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01\x7FpublicDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x87_\x015\x8A\x8A\x90Pa%BV[a\x1E\xAF`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01\x7FuserDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x87` \x015\x8A\x8A\x90Pa%BV[a\x1E\xF6`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7FkmsGen\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x87`@\x015\x8A\x8A\x90Pa%BV[a\x1F=`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01\x7Fmpc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x87``\x015\x8A\x8A\x90Pa%BV[_a\x1FFa\x1C]V[\x90P\x89\x81_\x01\x81\x90UP_[\x89\x89\x90P\x81\x10\x15a \xAEW6\x8A\x8A\x83\x81\x81\x10a\x1FqWa\x1Fpa@\xC9V[[\x90P` \x02\x81\x01\x90a\x1F\x83\x91\x90aA\x02V[\x90Pa \xA0\x8C`@Q\x80`\x80\x01`@R\x80\x84_\x01` \x81\x01\x90a\x1F\xA6\x91\x90a2\xC9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84` \x01` \x81\x01\x90a\x1F\xD4\x91\x90a2\xC9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84\x80`@\x01\x90a\x1F\xFF\x91\x90aA*V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x84\x80``\x01\x90a V\x91\x90aA*V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81RPa(\tV[P\x80\x80`\x01\x01\x91PPa\x1FRV[P\x86_\x015\x81`\x06\x01_\x8C\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x86` \x015\x81`\x07\x01_\x8C\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x86`@\x015\x81`\x08\x01_\x8C\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x86``\x015\x81`\t\x01_\x8C\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x81\x81`\x0B\x01_\x8C\x81R` \x01\x90\x81R` \x01_ \x81\x81a!>\x91\x90a>\xA0V[\x90PP\x81`@\x01` \x81\x01\x90a!T\x91\x90a2\xC9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82_\x015\x8B\x7F3\xD3\xE2@l\xC6\x1Cvt\x1A\x8C\x7F\xEE'\xB5 \xEA\x99\x86a\xCA\xCEI\xC1R3i\xE7\xFD4\x0F\x16\x8C\x8C\x8C\x8C\x8C\x8C\x8C\x8C` \x015`@Qa!\xB0\x98\x97\x96\x95\x94\x93\x92\x91\x90aF\x90V[`@Q\x80\x91\x03\x90\xA4PPPPPPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a\"qWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\"Xa-wV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a\"\xA8W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#\x07W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#+\x91\x90a<xV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a#\x9AW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a#\x91\x91\x90a<\xB2V[`@Q\x80\x91\x03\x90\xFD[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a$\x05WP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\x02\x91\x90aG'V[`\x01[a$FW\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a$=\x91\x90a<\xB2V[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14a$\xACW\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a$\xA3\x91\x90a8\xD7V[`@Q\x80\x91\x03\x90\xFD[a$\xB6\x83\x83a-\xCAV[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a%@W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x82\x03a%\x86W\x82`@Q\x7F6\xBF\xB6\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a%}\x91\x90a1\xABV[`@Q\x80\x91\x03\x90\xFD[`\xFF\x80\x16\x82\x11\x15a%\xD5W\x82\x82`\xFF\x80\x16`@Q\x7F\"\xBAR\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a%\xCC\x93\x92\x91\x90aGRV[`@Q\x80\x91\x03\x90\xFD[\x80\x82\x11\x15a&\x1EW\x82\x82\x82`@Q\x7F\xCA\xA8\x14\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&\x15\x93\x92\x91\x90aGRV[`@Q\x80\x91\x03\x90\xFD[PPPV[_\x80a&-a\x1C]V[\x90P`\x01`\xF8`\x07`\x08\x81\x11\x15a&GWa&Fa?FV[[\x90\x1Ba&S\x91\x90a?\xA0V[\x83\x10\x15\x80\x15a&eWP\x80_\x01T\x83\x11\x15[\x80\x15a&\x87WP_\x81`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x14\x15[\x80\x15a&\xB0WP\x80`\n\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x91PP\x91\x90PV[_\x80_\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a'\x14Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81a'\nWa'\ta@\x9CV[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a'QWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81a'GWa'Fa@\x9CV[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10a'\x80Wf#\x86\xF2o\xC1\0\0\x83\x81a'vWa'ua@\x9CV[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10a'\xA9Wc\x05\xF5\xE1\0\x83\x81a'\x9FWa'\x9Ea@\x9CV[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10a'\xCEWa'\x10\x83\x81a'\xC4Wa'\xC3a@\x9CV[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10a'\xF1W`d\x83\x81a'\xE7Wa'\xE6a@\x9CV[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10a(\0W`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[_a(\x12a\x1C]V[\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a(|W`@Q\x7F\x84f\x80J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a(\xE5W`@Q\x7F-\xEC\xCFM\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x02\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x83_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a)\x88W\x81_\x01Q`@Q\x7F\xD1\x8CO\xF0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a)\x7F\x91\x90a<\xB2V[`@Q\x80\x91\x03\x90\xFD[\x80`\x03\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x83` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a*-W\x81` \x01Q`@Q\x7F\xF5\x1A\xF6\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a*$\x91\x90a<\xB2V[`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x04\x02\x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x02\x01\x90\x81a+\x06\x91\x90aH\xF8V[P``\x82\x01Q\x81`\x03\x01\x90\x81a+\x1C\x91\x90aH\xF8V[PPP`\x01\x81`\x02\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x84_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01\x81`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x84` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x81\x81`\x04\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x84_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x02\x01\x90\x81a,\xE3\x91\x90aH\xF8V[P``\x82\x01Q\x81`\x03\x01\x90\x81a,\xF9\x91\x90aH\xF8V[P\x90PP\x80`\x05\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x82` \x01Q\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPV[_a-\xA3\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba.<V[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a-\xD3\x82a.EV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15a./Wa.)\x82\x82a/\x0EV[Pa.8V[a.7a/\x8EV[[PPV[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a.\xA0W\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.\x97\x91\x90a<\xB2V[`@Q\x80\x91\x03\x90\xFD[\x80a.\xCC\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba.<V[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qa/7\x91\x90aJ\x0BV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a/oW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a/tV[``\x91P[P\x91P\x91Pa/\x84\x85\x83\x83a/\xCAV[\x92PPP\x92\x91PPV[_4\x11\x15a/\xC8W`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[``\x82a/\xDFWa/\xDA\x82a0WV[a0OV[_\x82Q\x14\x80\x15a0\x05WP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a0GW\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a0>\x91\x90a<\xB2V[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa0PV[[\x93\x92PPPV[_\x81Q\x11\x15a0iW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x80``\x01`@R\x80_\x81R` \x01_\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81R` \x01``\x81RP\x90V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15a1XW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa1=V[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a1}\x82a1!V[a1\x87\x81\x85a1+V[\x93Pa1\x97\x81\x85` \x86\x01a1;V[a1\xA0\x81a1cV[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra1\xC3\x81\x84a1sV[\x90P\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x81\x90P\x91\x90PV[a1\xEE\x81a1\xDCV[\x81\x14a1\xF8W_\x80\xFD[PV[_\x815\x90Pa2\t\x81a1\xE5V[\x92\x91PPV[_\x80\xFD[_``\x82\x84\x03\x12\x15a2(Wa2'a2\x0FV[[\x81\x90P\x92\x91PPV[_\x80`\x80\x83\x85\x03\x12\x15a2GWa2Fa1\xD4V[[_a2T\x85\x82\x86\x01a1\xFBV[\x92PP` a2e\x85\x82\x86\x01a2\x13V[\x91PP\x92P\x92\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a2\x98\x82a2oV[\x90P\x91\x90PV[a2\xA8\x81a2\x8EV[\x81\x14a2\xB2W_\x80\xFD[PV[_\x815\x90Pa2\xC3\x81a2\x9FV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a2\xDEWa2\xDDa1\xD4V[[_a2\xEB\x84\x82\x85\x01a2\xB5V[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a3\x08\x81a2\xF4V[\x82RPPV[_` \x82\x01\x90Pa3!_\x83\x01\x84a2\xFFV[\x92\x91PPV[a30\x81a1\xDCV[\x82RPPV[_` \x82\x01\x90Pa3I_\x83\x01\x84a3'V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a3dWa3ca1\xD4V[[_a3q\x84\x82\x85\x01a1\xFBV[\x91PP\x92\x91PPV[a3\x83\x81a1\xDCV[\x82RPPV[a3\x92\x81a2\x8EV[\x82RPPV[``\x82\x01_\x82\x01Qa3\xAC_\x85\x01\x82a3zV[P` \x82\x01Qa3\xBF` \x85\x01\x82a3zV[P`@\x82\x01Qa3\xD2`@\x85\x01\x82a3\x89V[PPPPV[_``\x82\x01\x90Pa3\xEB_\x83\x01\x84a3\x98V[\x92\x91PPV[_``\x82\x84\x03\x12\x15a4\x06Wa4\x05a1\xD4V[[_a4\x13\x84\x82\x85\x01a2\x13V[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a42Wa41a1\xD4V[[_a4?\x85\x82\x86\x01a1\xFBV[\x92PP` a4P\x85\x82\x86\x01a2\xB5V[\x91PP\x92P\x92\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a4t\x82a1!V[a4~\x81\x85a4ZV[\x93Pa4\x8E\x81\x85` \x86\x01a1;V[a4\x97\x81a1cV[\x84\x01\x91PP\x92\x91PPV[_`\x80\x83\x01_\x83\x01Qa4\xB7_\x86\x01\x82a3\x89V[P` \x83\x01Qa4\xCA` \x86\x01\x82a3\x89V[P`@\x83\x01Q\x84\x82\x03`@\x86\x01Ra4\xE2\x82\x82a4jV[\x91PP``\x83\x01Q\x84\x82\x03``\x86\x01Ra4\xFC\x82\x82a4jV[\x91PP\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra5!\x81\x84a4\xA2V[\x90P\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12a5JWa5Ia5)V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5gWa5fa5-V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a5\x83Wa5\x82a51V[[\x92P\x92\x90PV[_`\x80\x82\x84\x03\x12\x15a5\x9FWa5\x9Ea2\x0FV[[\x81\x90P\x92\x91PPV[_\x80\x83`\x1F\x84\x01\x12a5\xBDWa5\xBCa5)V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5\xDAWa5\xD9a5-V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a5\xF6Wa5\xF5a51V[[\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12a6\x12Wa6\x11a5)V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6/Wa6.a5-V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a6KWa6Ja51V[[\x92P\x92\x90PV[_\x80_\x80_\x80_\x80_a\x01`\x8A\x8C\x03\x12\x15a6pWa6oa1\xD4V[[_a6}\x8C\x82\x8D\x01a1\xFBV[\x99PP` \x8A\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\x9EWa6\x9Da1\xD8V[[a6\xAA\x8C\x82\x8D\x01a55V[\x98P\x98PP`@a6\xBD\x8C\x82\x8D\x01a5\x8AV[\x96PP`\xC0\x8A\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\xDEWa6\xDDa1\xD8V[[a6\xEA\x8C\x82\x8D\x01a5\xA8V[\x95P\x95PP`\xE0\x8A\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7\rWa7\x0Ca1\xD8V[[a7\x19\x8C\x82\x8D\x01a5\xFDV[\x93P\x93PPa\x01\0a7-\x8C\x82\x8D\x01a2\x13V[\x91PP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a7w\x82a1cV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a7\x96Wa7\x95a7AV[[\x80`@RPPPV[_a7\xA8a1\xCBV[\x90Pa7\xB4\x82\x82a7nV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a7\xD3Wa7\xD2a7AV[[a7\xDC\x82a1cV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a8\ta8\x04\x84a7\xB9V[a7\x9FV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a8%Wa8$a7=V[[a80\x84\x82\x85a7\xE9V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a8LWa8Ka5)V[[\x815a8\\\x84\x82` \x86\x01a7\xF7V[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a8{Wa8za1\xD4V[[_a8\x88\x85\x82\x86\x01a2\xB5V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8\xA9Wa8\xA8a1\xD8V[[a8\xB5\x85\x82\x86\x01a88V[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[a8\xD1\x81a8\xBFV[\x82RPPV[_` \x82\x01\x90Pa8\xEA_\x83\x01\x84a8\xC8V[\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_a9$\x83\x83a3\x89V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a9F\x82a8\xF0V[a9P\x81\x85a8\xFAV[\x93Pa9[\x83a9\nV[\x80_[\x83\x81\x10\x15a9\x8BW\x81Qa9r\x88\x82a9\x19V[\x97Pa9}\x83a90V[\x92PP`\x01\x81\x01\x90Pa9^V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra9\xB0\x81\x84a9<V[\x90P\x92\x91PPV[_\x80`\xA0\x83\x85\x03\x12\x15a9\xCEWa9\xCDa1\xD4V[[_a9\xDB\x85\x82\x86\x01a1\xFBV[\x92PP` a9\xEC\x85\x82\x86\x01a5\x8AV[\x91PP\x92P\x92\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_`\x80\x83\x01_\x83\x01Qa:4_\x86\x01\x82a3\x89V[P` \x83\x01Qa:G` \x86\x01\x82a3\x89V[P`@\x83\x01Q\x84\x82\x03`@\x86\x01Ra:_\x82\x82a4jV[\x91PP``\x83\x01Q\x84\x82\x03``\x86\x01Ra:y\x82\x82a4jV[\x91PP\x80\x91PP\x92\x91PPV[_a:\x91\x83\x83a:\x1FV[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a:\xAF\x82a9\xF6V[a:\xB9\x81\x85a:\0V[\x93P\x83` \x82\x02\x85\x01a:\xCB\x85a:\x10V[\x80_[\x85\x81\x10\x15a;\x06W\x84\x84\x03\x89R\x81Qa:\xE7\x85\x82a:\x86V[\x94Pa:\xF2\x83a:\x99V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa:\xCEV[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra;0\x81\x84a:\xA5V[\x90P\x92\x91PPV[_\x81\x90P\x92\x91PPV[_a;L\x82a1!V[a;V\x81\x85a;8V[\x93Pa;f\x81\x85` \x86\x01a1;V[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a;\xA6`\x02\x83a;8V[\x91Pa;\xB1\x82a;rV[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a;\xF0`\x01\x83a;8V[\x91Pa;\xFB\x82a;\xBCV[`\x01\x82\x01\x90P\x91\x90PV[_a<\x11\x82\x87a;BV[\x91Pa<\x1C\x82a;\x9AV[\x91Pa<(\x82\x86a;BV[\x91Pa<3\x82a;\xE4V[\x91Pa<?\x82\x85a;BV[\x91Pa<J\x82a;\xE4V[\x91Pa<V\x82\x84a;BV[\x91P\x81\x90P\x95\x94PPPPPV[_\x81Q\x90Pa<r\x81a2\x9FV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a<\x8DWa<\x8Ca1\xD4V[[_a<\x9A\x84\x82\x85\x01a<dV[\x91PP\x92\x91PPV[a<\xAC\x81a2\x8EV[\x82RPPV[_` \x82\x01\x90Pa<\xC5_\x83\x01\x84a<\xA3V[\x92\x91PPV[_\x815a<\xD7\x81a1\xE5V[\x80\x91PP\x91\x90PV[_\x81_\x1B\x90P\x91\x90PV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa=\x16\x84a<\xE0V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[_a=Oa=Ja=E\x84a1\xDCV[a=,V[a1\xDCV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a=h\x82a=5V[a={a=t\x82a=VV[\x83Ta<\xEBV[\x82UPPPV[_\x815a=\x8E\x81a2\x9FV[\x80\x91PP\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa=\xB6\x84a<\xE0V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_a=\xE6a=\xE1a=\xDC\x84a2oV[a=,V[a2oV[\x90P\x91\x90PV[_a=\xF7\x82a=\xCCV[\x90P\x91\x90PV[_a>\x08\x82a=\xEDV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a>!\x82a=\xFEV[a>4a>-\x82a>\x0FV[\x83Ta=\x97V[\x82UPPPV[_\x81\x01_\x83\x01\x80a>K\x81a<\xCBV[\x90Pa>W\x81\x84a=_V[PPP`\x01\x81\x01` \x83\x01\x80a>l\x81a<\xCBV[\x90Pa>x\x81\x84a=_V[PPP`\x02\x81\x01`@\x83\x01\x80a>\x8D\x81a=\x82V[\x90Pa>\x99\x81\x84a>\x18V[PPPPPV[a>\xAA\x82\x82a>;V[PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a>\xCA\x81a>\xAEV[\x82RPPV[_` \x82\x01\x90Pa>\xE3_\x83\x01\x84a>\xC1V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a?-W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a?@Wa??a>\xE9V[[P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a?\xAA\x82a1\xDCV[\x91Pa?\xB5\x83a1\xDCV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a?\xCDWa?\xCCa?sV[[\x92\x91PPV[_`@\x82\x01\x90Pa?\xE6_\x83\x01\x85a3'V[a?\xF3` \x83\x01\x84a3'V[\x93\x92PPPV[_a@\x08` \x84\x01\x84a1\xFBV[\x90P\x92\x91PPV[`\x80\x82\x01a@ _\x83\x01\x83a?\xFAV[a@,_\x85\x01\x82a3zV[Pa@:` \x83\x01\x83a?\xFAV[a@G` \x85\x01\x82a3zV[Pa@U`@\x83\x01\x83a?\xFAV[a@b`@\x85\x01\x82a3zV[Pa@p``\x83\x01\x83a?\xFAV[a@}``\x85\x01\x82a3zV[PPPPV[_`\x80\x82\x01\x90Pa@\x96_\x83\x01\x84a@\x10V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x825`\x01a\x01\0\x03\x836\x03\x03\x81\x12aA\x1EWaA\x1Da@\xF6V[[\x80\x83\x01\x91PP\x92\x91PPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aAFWaAEa@\xF6V[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aAhWaAga@\xFAV[[` \x83\x01\x92P`\x01\x82\x026\x03\x83\x13\x15aA\x84WaA\x83a@\xFEV[[P\x92P\x92\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[_aA\xB3` \x84\x01\x84a2\xB5V[\x90P\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aA\xE3WaA\xE2aA\xC3V[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aB\x0BWaB\naA\xBBV[[`\x01\x82\x026\x03\x83\x13\x15aB!WaB aA\xBFV[[P\x92P\x92\x90PV[_aB4\x83\x85a4ZV[\x93PaBA\x83\x85\x84a7\xE9V[aBJ\x83a1cV[\x84\x01\x90P\x93\x92PPPV[_\x81`\x03\x0B\x90P\x91\x90PV[aBj\x81aBUV[\x81\x14aBtW_\x80\xFD[PV[_\x815\x90PaB\x85\x81aBaV[\x92\x91PPV[_aB\x99` \x84\x01\x84aBwV[\x90P\x92\x91PPV[aB\xAA\x81aBUV[\x82RPPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aB\xCCWaB\xCBaA\xC3V[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aB\xF4WaB\xF3aA\xBBV[[`\x01\x82\x026\x03\x83\x13\x15aC\nWaC\taA\xBFV[[P\x92P\x92\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aC-\x83\x85aC\x12V[\x93PaC:\x83\x85\x84a7\xE9V[aCC\x83a1cV[\x84\x01\x90P\x93\x92PPPV[_a\x01\0\x83\x01aC`_\x84\x01\x84aA\xA5V[aCl_\x86\x01\x82a3\x89V[PaCz` \x84\x01\x84aA\xA5V[aC\x87` \x86\x01\x82a3\x89V[PaC\x95`@\x84\x01\x84aA\xC7V[\x85\x83\x03`@\x87\x01RaC\xA8\x83\x82\x84aB)V[\x92PPPaC\xB9``\x84\x01\x84aA\xC7V[\x85\x83\x03``\x87\x01RaC\xCC\x83\x82\x84aB)V[\x92PPPaC\xDD`\x80\x84\x01\x84aB\x8BV[aC\xEA`\x80\x86\x01\x82aB\xA1V[PaC\xF8`\xA0\x84\x01\x84aA\xC7V[\x85\x83\x03`\xA0\x87\x01RaD\x0B\x83\x82\x84aB)V[\x92PPPaD\x1C`\xC0\x84\x01\x84aB\xB0V[\x85\x83\x03`\xC0\x87\x01RaD/\x83\x82\x84aC\"V[\x92PPPaD@`\xE0\x84\x01\x84aA\xC7V[\x85\x83\x03`\xE0\x87\x01RaDS\x83\x82\x84aB)V[\x92PPP\x80\x91PP\x92\x91PPV[_aDl\x83\x83aCNV[\x90P\x92\x91PPV[_\x825`\x01a\x01\0\x03\x836\x03\x03\x81\x12aD\x90WaD\x8FaA\xC3V[[\x82\x81\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aD\xB3\x83\x85aA\x8CV[\x93P\x83` \x84\x02\x85\x01aD\xC5\x84aA\x9CV[\x80_[\x87\x81\x10\x15aE\x08W\x84\x84\x03\x89RaD\xDF\x82\x84aDtV[aD\xE9\x85\x82aDaV[\x94PaD\xF4\x83aD\x9CV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaD\xC8V[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_aE%\x83\x85a1+V[\x93PaE2\x83\x85\x84a7\xE9V[aE;\x83a1cV[\x84\x01\x90P\x93\x92PPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[_``\x83\x01aEp_\x84\x01\x84aB\xB0V[\x85\x83\x03_\x87\x01RaE\x82\x83\x82\x84aC\"V[\x92PPPaE\x93` \x84\x01\x84aB\xB0V[\x85\x83\x03` \x87\x01RaE\xA6\x83\x82\x84aC\"V[\x92PPPaE\xB7`@\x84\x01\x84aB\xB0V[\x85\x83\x03`@\x87\x01RaE\xCA\x83\x82\x84aC\"V[\x92PPP\x80\x91PP\x92\x91PPV[_aE\xE3\x83\x83aE_V[\x90P\x92\x91PPV[_\x825`\x01``\x03\x836\x03\x03\x81\x12aF\x06WaF\x05aA\xC3V[[\x82\x81\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aF)\x83\x85aEFV[\x93P\x83` \x84\x02\x85\x01aF;\x84aEVV[\x80_[\x87\x81\x10\x15aF~W\x84\x84\x03\x89RaFU\x82\x84aE\xEBV[aF_\x85\x82aE\xD8V[\x94PaFj\x83aF\x12V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaF>V[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_a\x01\0\x82\x01\x90P\x81\x81\x03_\x83\x01RaF\xAA\x81\x8A\x8CaD\xA8V[\x90PaF\xB9` \x83\x01\x89a@\x10V[\x81\x81\x03`\xA0\x83\x01RaF\xCC\x81\x87\x89aE\x1AV[\x90P\x81\x81\x03`\xC0\x83\x01RaF\xE1\x81\x85\x87aF\x1EV[\x90PaF\xF0`\xE0\x83\x01\x84a3'V[\x99\x98PPPPPPPPPV[aG\x06\x81a8\xBFV[\x81\x14aG\x10W_\x80\xFD[PV[_\x81Q\x90PaG!\x81aF\xFDV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aG<WaG;a1\xD4V[[_aGI\x84\x82\x85\x01aG\x13V[\x91PP\x92\x91PPV[_``\x82\x01\x90P\x81\x81\x03_\x83\x01RaGj\x81\x86a1sV[\x90PaGy` \x83\x01\x85a3'V[aG\x86`@\x83\x01\x84a3'V[\x94\x93PPPPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02aG\xEA\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82aG\xAFV[aG\xF4\x86\x83aG\xAFV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[aH\x15\x83a=5V[aH)aH!\x82a=VV[\x84\x84TaG\xBBV[\x82UPPPPV[_\x90V[aH=aH1V[aHH\x81\x84\x84aH\x0CV[PPPV[[\x81\x81\x10\x15aHkWaH`_\x82aH5V[`\x01\x81\x01\x90PaHNV[PPV[`\x1F\x82\x11\x15aH\xB0WaH\x81\x81aG\x8EV[aH\x8A\x84aG\xA0V[\x81\x01` \x85\x10\x15aH\x99W\x81\x90P[aH\xADaH\xA5\x85aG\xA0V[\x83\x01\x82aHMV[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_aH\xD0_\x19\x84`\x08\x02aH\xB5V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_aH\xE8\x83\x83aH\xC1V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[aI\x01\x82a1!V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aI\x1AWaI\x19a7AV[[aI$\x82Ta?\x16V[aI/\x82\x82\x85aHoV[_` \x90P`\x1F\x83\x11`\x01\x81\x14aI`W_\x84\x15aINW\x82\x87\x01Q\x90P[aIX\x85\x82aH\xDDV[\x86UPaI\xBFV[`\x1F\x19\x84\x16aIn\x86aG\x8EV[_[\x82\x81\x10\x15aI\x95W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PaIpV[\x86\x83\x10\x15aI\xB2W\x84\x89\x01QaI\xAE`\x1F\x89\x16\x82aH\xC1V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_aI\xE5\x82aI\xC7V[aI\xEF\x81\x85aI\xD1V[\x93PaI\xFF\x81\x85` \x86\x01a1;V[\x80\x84\x01\x91PP\x92\x91PPV[_aJ\x16\x82\x84aI\xDBV[\x91P\x81\x90P\x92\x91PPV",
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
    /**Custom error with signature `InvalidSourceProtocolConfig()` and selector `0xd8e1832b`.
```solidity
error InvalidSourceProtocolConfig();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSourceProtocolConfig;
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
        impl ::core::convert::From<InvalidSourceProtocolConfig>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSourceProtocolConfig) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidSourceProtocolConfig {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSourceProtocolConfig {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSourceProtocolConfig()";
            const SELECTOR: [u8; 4] = [216u8, 225u8, 131u8, 43u8];
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
    /**Custom error with signature `NonIncreasingKmsContextId(uint256,uint256)` and selector `0xefd55f67`.
```solidity
error NonIncreasingKmsContextId(uint256 contextId, uint256 currentKmsContextId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NonIncreasingKmsContextId {
        #[allow(missing_docs)]
        pub contextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub currentKmsContextId: alloy::sol_types::private::primitives::aliases::U256,
    }
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
                (value.contextId, value.currentKmsContextId)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for NonIncreasingKmsContextId {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    contextId: tuple.0,
                    currentKmsContextId: tuple.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.currentKmsContextId),
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
    /**Event with signature `MirroredKmsContext(uint256,(address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[],uint256,uint256,address)` and selector `0x33d3e2406cc61c76741a8c7fee27b520ea998661cace49c1523369e7fd340f16`.
```solidity
event MirroredKmsContext(uint256 indexed contextId, KmsNodeParams[] kmsNodeParams, IProtocolConfigCommon.KmsThresholds thresholds, string softwareVersion, PcrValues[] pcrValues, uint256 indexed sourceChainId, uint256 sourceBlockNumber, address indexed sourceProtocolConfig);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct MirroredKmsContext {
        #[allow(missing_docs)]
        pub contextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub kmsNodeParams: alloy::sol_types::private::Vec<
            <KmsNodeParams as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub thresholds: <IProtocolConfigCommon::KmsThresholds as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub softwareVersion: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub pcrValues: alloy::sol_types::private::Vec<
            <PcrValues as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub sourceChainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub sourceBlockNumber: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub sourceProtocolConfig: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for MirroredKmsContext {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<KmsNodeParams>,
                IProtocolConfigCommon::KmsThresholds,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<PcrValues>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "MirroredKmsContext(uint256,(address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[],uint256,uint256,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                51u8, 211u8, 226u8, 64u8, 108u8, 198u8, 28u8, 118u8, 116u8, 26u8, 140u8,
                127u8, 238u8, 39u8, 181u8, 32u8, 234u8, 153u8, 134u8, 97u8, 202u8, 206u8,
                73u8, 193u8, 82u8, 51u8, 105u8, 231u8, 253u8, 52u8, 15u8, 22u8,
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
                    kmsNodeParams: data.0,
                    thresholds: data.1,
                    softwareVersion: data.2,
                    pcrValues: data.3,
                    sourceChainId: topics.2,
                    sourceBlockNumber: data.4,
                    sourceProtocolConfig: topics.3,
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
                    <IProtocolConfigCommon::KmsThresholds as alloy_sol_types::SolType>::tokenize(
                        &self.thresholds,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.softwareVersion,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        PcrValues,
                    > as alloy_sol_types::SolType>::tokenize(&self.pcrValues),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.sourceBlockNumber),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.contextId.clone(),
                    self.sourceChainId.clone(),
                    self.sourceProtocolConfig.clone(),
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.sourceChainId);
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sourceProtocolConfig,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for MirroredKmsContext {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MirroredKmsContext> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &MirroredKmsContext) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `MirroredKmsContextDestroyed(uint256,uint256,uint256,address)` and selector `0x3f5d57d3508d73eef9374d0ee29403786cb45bfb661d47cad4ad6916e0760c39`.
```solidity
event MirroredKmsContextDestroyed(uint256 indexed contextId, uint256 indexed sourceChainId, uint256 sourceBlockNumber, address indexed sourceProtocolConfig);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct MirroredKmsContextDestroyed {
        #[allow(missing_docs)]
        pub contextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub sourceChainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub sourceBlockNumber: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub sourceProtocolConfig: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for MirroredKmsContextDestroyed {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "MirroredKmsContextDestroyed(uint256,uint256,uint256,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                63u8, 93u8, 87u8, 211u8, 80u8, 141u8, 115u8, 238u8, 249u8, 55u8, 77u8,
                14u8, 226u8, 148u8, 3u8, 120u8, 108u8, 180u8, 91u8, 251u8, 102u8, 29u8,
                71u8, 202u8, 212u8, 173u8, 105u8, 22u8, 224u8, 118u8, 12u8, 57u8,
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
                    sourceChainId: topics.2,
                    sourceBlockNumber: data.0,
                    sourceProtocolConfig: topics.3,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.sourceBlockNumber),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.contextId.clone(),
                    self.sourceChainId.clone(),
                    self.sourceProtocolConfig.clone(),
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.sourceChainId);
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sourceProtocolConfig,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for MirroredKmsContextDestroyed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MirroredKmsContextDestroyed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &MirroredKmsContextDestroyed,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `MirroredKmsThresholds(uint256,(uint256,uint256,uint256,uint256))` and selector `0x8ec991e9b0a696c33afa5f2327c050777dfb01687d51237e2d683e9388b64776`.
```solidity
event MirroredKmsThresholds(uint256 indexed contextId, IProtocolConfigCommon.KmsThresholds thresholds);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct MirroredKmsThresholds {
        #[allow(missing_docs)]
        pub contextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub thresholds: <IProtocolConfigCommon::KmsThresholds as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for MirroredKmsThresholds {
            type DataTuple<'a> = (IProtocolConfigCommon::KmsThresholds,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "MirroredKmsThresholds(uint256,(uint256,uint256,uint256,uint256))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                142u8, 201u8, 145u8, 233u8, 176u8, 166u8, 150u8, 195u8, 58u8, 250u8,
                95u8, 35u8, 39u8, 192u8, 80u8, 119u8, 125u8, 251u8, 1u8, 104u8, 125u8,
                81u8, 35u8, 126u8, 45u8, 104u8, 62u8, 147u8, 136u8, 182u8, 71u8, 118u8,
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
                    thresholds: data.0,
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
                    <IProtocolConfigCommon::KmsThresholds as alloy_sol_types::SolType>::tokenize(
                        &self.thresholds,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.contextId.clone())
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
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for MirroredKmsThresholds {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MirroredKmsThresholds> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &MirroredKmsThresholds) -> alloy_sol_types::private::LogData {
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
    /**Function with signature `getMirroredContextSource(uint256)` and selector `0x2c87fea2`.
```solidity
function getMirroredContextSource(uint256 contextId) external view returns (IProtocolConfigMultichain.MirroredContextSource memory source);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMirroredContextSourceCall {
        #[allow(missing_docs)]
        pub contextId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getMirroredContextSource(uint256)`](getMirroredContextSourceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMirroredContextSourceReturn {
        #[allow(missing_docs)]
        pub source: <IProtocolConfigMultichain::MirroredContextSource as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getMirroredContextSourceCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMirroredContextSourceCall) -> Self {
                    (value.contextId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMirroredContextSourceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { contextId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                IProtocolConfigMultichain::MirroredContextSource,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IProtocolConfigMultichain::MirroredContextSource as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getMirroredContextSourceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMirroredContextSourceReturn) -> Self {
                    (value.source,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMirroredContextSourceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { source: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getMirroredContextSourceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <IProtocolConfigMultichain::MirroredContextSource as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (IProtocolConfigMultichain::MirroredContextSource,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getMirroredContextSource(uint256)";
            const SELECTOR: [u8; 4] = [44u8, 135u8, 254u8, 162u8];
            #[inline]
            fn new<'a>(
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
                (
                    <IProtocolConfigMultichain::MirroredContextSource as alloy_sol_types::SolType>::tokenize(
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
                        let r: getMirroredContextSourceReturn = r.into();
                        r.source
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
                        let r: getMirroredContextSourceReturn = r.into();
                        r.source
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
    /**Function with signature `initializeFromEmptyProxy(uint256,(address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[],(uint256,uint256,address))` and selector `0x4cfb42be`.
```solidity
function initializeFromEmptyProxy(uint256 initialContextId, KmsNodeParams[] memory initialKmsNodeParams, IProtocolConfigCommon.KmsThresholds memory initialThresholds, string memory softwareVersion, PcrValues[] memory pcrValues, IProtocolConfigMultichain.MirroredContextSource memory source) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeFromEmptyProxyCall {
        #[allow(missing_docs)]
        pub initialContextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub initialKmsNodeParams: alloy::sol_types::private::Vec<
            <KmsNodeParams as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub initialThresholds: <IProtocolConfigCommon::KmsThresholds as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub softwareVersion: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub pcrValues: alloy::sol_types::private::Vec<
            <PcrValues as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub source: <IProtocolConfigMultichain::MirroredContextSource as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`initializeFromEmptyProxy(uint256,(address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[],(uint256,uint256,address))`](initializeFromEmptyProxyCall) function.
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<KmsNodeParams>,
                IProtocolConfigCommon::KmsThresholds,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<PcrValues>,
                IProtocolConfigMultichain::MirroredContextSource,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    <KmsNodeParams as alloy::sol_types::SolType>::RustType,
                >,
                <IProtocolConfigCommon::KmsThresholds as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::String,
                alloy::sol_types::private::Vec<
                    <PcrValues as alloy::sol_types::SolType>::RustType,
                >,
                <IProtocolConfigMultichain::MirroredContextSource as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
                        value.initialContextId,
                        value.initialKmsNodeParams,
                        value.initialThresholds,
                        value.softwareVersion,
                        value.pcrValues,
                        value.source,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeFromEmptyProxyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        initialContextId: tuple.0,
                        initialKmsNodeParams: tuple.1,
                        initialThresholds: tuple.2,
                        softwareVersion: tuple.3,
                        pcrValues: tuple.4,
                        source: tuple.5,
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<KmsNodeParams>,
                IProtocolConfigCommon::KmsThresholds,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<PcrValues>,
                IProtocolConfigMultichain::MirroredContextSource,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeFromEmptyProxyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initializeFromEmptyProxy(uint256,(address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[],(uint256,uint256,address))";
            const SELECTOR: [u8; 4] = [76u8, 251u8, 66u8, 190u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.initialContextId),
                    <alloy::sol_types::sol_data::Array<
                        KmsNodeParams,
                    > as alloy_sol_types::SolType>::tokenize(&self.initialKmsNodeParams),
                    <IProtocolConfigCommon::KmsThresholds as alloy_sol_types::SolType>::tokenize(
                        &self.initialThresholds,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.softwareVersion,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        PcrValues,
                    > as alloy_sol_types::SolType>::tokenize(&self.pcrValues),
                    <IProtocolConfigMultichain::MirroredContextSource as alloy_sol_types::SolType>::tokenize(
                        &self.source,
                    ),
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
    /**Function with signature `mirrorKmsContext(uint256,(address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[],(uint256,uint256,address))` and selector `0x5b726296`.
```solidity
function mirrorKmsContext(uint256 contextId, KmsNodeParams[] memory kmsNodeParams, IProtocolConfigCommon.KmsThresholds memory thresholds, string memory softwareVersion, PcrValues[] memory pcrValues, IProtocolConfigMultichain.MirroredContextSource memory source) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mirrorKmsContextCall {
        #[allow(missing_docs)]
        pub contextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub kmsNodeParams: alloy::sol_types::private::Vec<
            <KmsNodeParams as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub thresholds: <IProtocolConfigCommon::KmsThresholds as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub softwareVersion: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub pcrValues: alloy::sol_types::private::Vec<
            <PcrValues as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub source: <IProtocolConfigMultichain::MirroredContextSource as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`mirrorKmsContext(uint256,(address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[],(uint256,uint256,address))`](mirrorKmsContextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mirrorKmsContextReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Array<KmsNodeParams>,
                IProtocolConfigCommon::KmsThresholds,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<PcrValues>,
                IProtocolConfigMultichain::MirroredContextSource,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    <KmsNodeParams as alloy::sol_types::SolType>::RustType,
                >,
                <IProtocolConfigCommon::KmsThresholds as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::String,
                alloy::sol_types::private::Vec<
                    <PcrValues as alloy::sol_types::SolType>::RustType,
                >,
                <IProtocolConfigMultichain::MirroredContextSource as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<mirrorKmsContextCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: mirrorKmsContextCall) -> Self {
                    (
                        value.contextId,
                        value.kmsNodeParams,
                        value.thresholds,
                        value.softwareVersion,
                        value.pcrValues,
                        value.source,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for mirrorKmsContextCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        contextId: tuple.0,
                        kmsNodeParams: tuple.1,
                        thresholds: tuple.2,
                        softwareVersion: tuple.3,
                        pcrValues: tuple.4,
                        source: tuple.5,
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
            impl ::core::convert::From<mirrorKmsContextReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: mirrorKmsContextReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for mirrorKmsContextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl mirrorKmsContextReturn {
            fn _tokenize(
                &self,
            ) -> <mirrorKmsContextCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for mirrorKmsContextCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<KmsNodeParams>,
                IProtocolConfigCommon::KmsThresholds,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<PcrValues>,
                IProtocolConfigMultichain::MirroredContextSource,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = mirrorKmsContextReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "mirrorKmsContext(uint256,(address,address,string,string,int32,string,bytes,string)[],(uint256,uint256,uint256,uint256),string,(bytes,bytes,bytes)[],(uint256,uint256,address))";
            const SELECTOR: [u8; 4] = [91u8, 114u8, 98u8, 150u8];
            #[inline]
            fn new<'a>(
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
                    <alloy::sol_types::sol_data::Array<
                        KmsNodeParams,
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsNodeParams),
                    <IProtocolConfigCommon::KmsThresholds as alloy_sol_types::SolType>::tokenize(
                        &self.thresholds,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.softwareVersion,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        PcrValues,
                    > as alloy_sol_types::SolType>::tokenize(&self.pcrValues),
                    <IProtocolConfigMultichain::MirroredContextSource as alloy_sol_types::SolType>::tokenize(
                        &self.source,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                mirrorKmsContextReturn::_tokenize(ret)
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
    /**Function with signature `mirrorKmsContextDestruction(uint256,(uint256,uint256,address))` and selector `0x1f81e10f`.
```solidity
function mirrorKmsContextDestruction(uint256 contextId, IProtocolConfigMultichain.MirroredContextSource memory source) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mirrorKmsContextDestructionCall {
        #[allow(missing_docs)]
        pub contextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub source: <IProtocolConfigMultichain::MirroredContextSource as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`mirrorKmsContextDestruction(uint256,(uint256,uint256,address))`](mirrorKmsContextDestructionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mirrorKmsContextDestructionReturn {}
    #[allow(
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
                IProtocolConfigMultichain::MirroredContextSource,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                <IProtocolConfigMultichain::MirroredContextSource as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<mirrorKmsContextDestructionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: mirrorKmsContextDestructionCall) -> Self {
                    (value.contextId, value.source)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for mirrorKmsContextDestructionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        contextId: tuple.0,
                        source: tuple.1,
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
            impl ::core::convert::From<mirrorKmsContextDestructionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: mirrorKmsContextDestructionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for mirrorKmsContextDestructionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl mirrorKmsContextDestructionReturn {
            fn _tokenize(
                &self,
            ) -> <mirrorKmsContextDestructionCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for mirrorKmsContextDestructionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                IProtocolConfigMultichain::MirroredContextSource,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = mirrorKmsContextDestructionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "mirrorKmsContextDestruction(uint256,(uint256,uint256,address))";
            const SELECTOR: [u8; 4] = [31u8, 129u8, 225u8, 15u8];
            #[inline]
            fn new<'a>(
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
                    <IProtocolConfigMultichain::MirroredContextSource as alloy_sol_types::SolType>::tokenize(
                        &self.source,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                mirrorKmsContextDestructionReturn::_tokenize(ret)
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
    /**Function with signature `mirrorKmsThresholds(uint256,(uint256,uint256,uint256,uint256))` and selector `0x8afd0ca3`.
```solidity
function mirrorKmsThresholds(uint256 contextId, IProtocolConfigCommon.KmsThresholds memory thresholds) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mirrorKmsThresholdsCall {
        #[allow(missing_docs)]
        pub contextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub thresholds: <IProtocolConfigCommon::KmsThresholds as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`mirrorKmsThresholds(uint256,(uint256,uint256,uint256,uint256))`](mirrorKmsThresholdsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mirrorKmsThresholdsReturn {}
    #[allow(
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
                IProtocolConfigCommon::KmsThresholds,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                <IProtocolConfigCommon::KmsThresholds as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<mirrorKmsThresholdsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: mirrorKmsThresholdsCall) -> Self {
                    (value.contextId, value.thresholds)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for mirrorKmsThresholdsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        contextId: tuple.0,
                        thresholds: tuple.1,
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
            impl ::core::convert::From<mirrorKmsThresholdsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: mirrorKmsThresholdsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for mirrorKmsThresholdsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl mirrorKmsThresholdsReturn {
            fn _tokenize(
                &self,
            ) -> <mirrorKmsThresholdsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for mirrorKmsThresholdsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                IProtocolConfigCommon::KmsThresholds,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = mirrorKmsThresholdsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "mirrorKmsThresholds(uint256,(uint256,uint256,uint256,uint256))";
            const SELECTOR: [u8; 4] = [138u8, 253u8, 12u8, 163u8];
            #[inline]
            fn new<'a>(
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
                    <IProtocolConfigCommon::KmsThresholds as alloy_sol_types::SolType>::tokenize(
                        &self.thresholds,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                mirrorKmsThresholdsReturn::_tokenize(ret)
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
    /**Function with signature `reinitializeV2((uint256,uint256,address))` and selector `0x2e879d7e`.
```solidity
function reinitializeV2(IProtocolConfigMultichain.MirroredContextSource memory source) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reinitializeV2Call {
        #[allow(missing_docs)]
        pub source: <IProtocolConfigMultichain::MirroredContextSource as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`reinitializeV2((uint256,uint256,address))`](reinitializeV2Call) function.
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
                IProtocolConfigMultichain::MirroredContextSource,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IProtocolConfigMultichain::MirroredContextSource as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
                    (value.source,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for reinitializeV2Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { source: tuple.0 }
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
            type Parameters<'a> = (IProtocolConfigMultichain::MirroredContextSource,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = reinitializeV2Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "reinitializeV2((uint256,uint256,address))";
            const SELECTOR: [u8; 4] = [46u8, 135u8, 157u8, 126u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IProtocolConfigMultichain::MirroredContextSource as alloy_sol_types::SolType>::tokenize(
                        &self.source,
                    ),
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
    ///Container for all the [`ProtocolConfigMultichain`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum ProtocolConfigMultichainCalls {
        #[allow(missing_docs)]
        UPGRADE_INTERFACE_VERSION(UPGRADE_INTERFACE_VERSIONCall),
        #[allow(missing_docs)]
        getCurrentKmsContextId(getCurrentKmsContextIdCall),
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
        getMirroredContextSource(getMirroredContextSourceCall),
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
        isValidKmsContext(isValidKmsContextCall),
        #[allow(missing_docs)]
        mirrorKmsContext(mirrorKmsContextCall),
        #[allow(missing_docs)]
        mirrorKmsContextDestruction(mirrorKmsContextDestructionCall),
        #[allow(missing_docs)]
        mirrorKmsThresholds(mirrorKmsThresholdsCall),
        #[allow(missing_docs)]
        proxiableUUID(proxiableUUIDCall),
        #[allow(missing_docs)]
        reinitializeV2(reinitializeV2Call),
        #[allow(missing_docs)]
        upgradeToAndCall(upgradeToAndCallCall),
    }
    #[automatically_derived]
    impl ProtocolConfigMultichainCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [13u8, 142u8, 110u8, 44u8],
            [31u8, 129u8, 225u8, 15u8],
            [32u8, 61u8, 1u8, 20u8],
            [38u8, 207u8, 93u8, 239u8],
            [40u8, 30u8, 139u8, 254u8],
            [42u8, 56u8, 137u8, 152u8],
            [44u8, 135u8, 254u8, 162u8],
            [46u8, 135u8, 157u8, 126u8],
            [49u8, 255u8, 65u8, 200u8],
            [65u8, 173u8, 6u8, 156u8],
            [70u8, 197u8, 187u8, 189u8],
            [71u8, 232u8, 34u8, 149u8],
            [76u8, 251u8, 66u8, 190u8],
            [79u8, 30u8, 242u8, 134u8],
            [82u8, 209u8, 144u8, 45u8],
            [91u8, 114u8, 98u8, 150u8],
            [91u8, 255u8, 118u8, 217u8],
            [126u8, 170u8, 200u8, 242u8],
            [138u8, 253u8, 12u8, 163u8],
            [148u8, 71u8, 207u8, 212u8],
            [151u8, 111u8, 62u8, 185u8],
            [173u8, 60u8, 177u8, 204u8],
            [180u8, 114u8, 43u8, 196u8],
            [191u8, 155u8, 22u8, 200u8],
            [194u8, 180u8, 41u8, 134u8],
            [195u8, 170u8, 170u8, 90u8],
            [249u8, 198u8, 112u8, 195u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ProtocolConfigMultichainCalls {
        const NAME: &'static str = "ProtocolConfigMultichainCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 27usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::UPGRADE_INTERFACE_VERSION(_) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentKmsContextId(_) => {
                    <getCurrentKmsContextIdCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::getMirroredContextSource(_) => {
                    <getMirroredContextSourceCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::isValidKmsContext(_) => {
                    <isValidKmsContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::mirrorKmsContext(_) => {
                    <mirrorKmsContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::mirrorKmsContextDestruction(_) => {
                    <mirrorKmsContextDestructionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::mirrorKmsThresholds(_) => {
                    <mirrorKmsThresholdsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proxiableUUID(_) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::reinitializeV2(_) => {
                    <reinitializeV2Call as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls>] = &[
                {
                    fn getVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getVersionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::getVersion)
                    }
                    getVersion
                },
                {
                    fn mirrorKmsContextDestruction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <mirrorKmsContextDestructionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainCalls::mirrorKmsContextDestruction,
                            )
                    }
                    mirrorKmsContextDestruction
                },
                {
                    fn isKmsSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <isKmsSignerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::isKmsSigner)
                    }
                    isKmsSigner
                },
                {
                    fn getMpcThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getMpcThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::getMpcThreshold)
                    }
                    getMpcThreshold
                },
                {
                    fn getUserDecryptionThresholdForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getUserDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainCalls::getUserDecryptionThresholdForContext,
                            )
                    }
                    getUserDecryptionThresholdForContext
                },
                {
                    fn getPublicDecryptionThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getPublicDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainCalls::getPublicDecryptionThreshold,
                            )
                    }
                    getPublicDecryptionThreshold
                },
                {
                    fn getMirroredContextSource(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getMirroredContextSourceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::getMirroredContextSource)
                    }
                    getMirroredContextSource
                },
                {
                    fn reinitializeV2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <reinitializeV2Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::reinitializeV2)
                    }
                    reinitializeV2
                },
                {
                    fn getKmsNodeForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getKmsNodeForContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::getKmsNodeForContext)
                    }
                    getKmsNodeForContext
                },
                {
                    fn getKmsGenThresholdForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getKmsGenThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainCalls::getKmsGenThresholdForContext,
                            )
                    }
                    getKmsGenThresholdForContext
                },
                {
                    fn isKmsTxSenderForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <isKmsTxSenderForContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::isKmsTxSenderForContext)
                    }
                    isKmsTxSenderForContext
                },
                {
                    fn getMpcThresholdForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getMpcThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainCalls::getMpcThresholdForContext,
                            )
                    }
                    getMpcThresholdForContext
                },
                {
                    fn initializeFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::initializeFromEmptyProxy)
                    }
                    initializeFromEmptyProxy
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn mirrorKmsContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <mirrorKmsContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::mirrorKmsContext)
                    }
                    mirrorKmsContext
                },
                {
                    fn getKmsSignersForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getKmsSignersForContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::getKmsSignersForContext)
                    }
                    getKmsSignersForContext
                },
                {
                    fn getKmsSigners(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getKmsSignersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::getKmsSigners)
                    }
                    getKmsSigners
                },
                {
                    fn mirrorKmsThresholds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <mirrorKmsThresholdsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::mirrorKmsThresholds)
                    }
                    mirrorKmsThresholds
                },
                {
                    fn isKmsSignerForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <isKmsSignerForContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::isKmsSignerForContext)
                    }
                    isKmsSignerForContext
                },
                {
                    fn getCurrentKmsContextId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getCurrentKmsContextIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::getCurrentKmsContextId)
                    }
                    getCurrentKmsContextId
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainCalls::UPGRADE_INTERFACE_VERSION,
                            )
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn getKmsGenThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getKmsGenThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::getKmsGenThreshold)
                    }
                    getKmsGenThreshold
                },
                {
                    fn isValidKmsContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <isValidKmsContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::isValidKmsContext)
                    }
                    isValidKmsContext
                },
                {
                    fn getUserDecryptionThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getUserDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainCalls::getUserDecryptionThreshold,
                            )
                    }
                    getUserDecryptionThreshold
                },
                {
                    fn getPublicDecryptionThresholdForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getPublicDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainCalls::getPublicDecryptionThresholdForContext,
                            )
                    }
                    getPublicDecryptionThresholdForContext
                },
                {
                    fn getKmsNodesForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getKmsNodesForContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::getKmsNodesForContext)
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
            ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls>] = &[
                {
                    fn getVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getVersionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::getVersion)
                    }
                    getVersion
                },
                {
                    fn mirrorKmsContextDestruction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <mirrorKmsContextDestructionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainCalls::mirrorKmsContextDestruction,
                            )
                    }
                    mirrorKmsContextDestruction
                },
                {
                    fn isKmsSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <isKmsSignerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::isKmsSigner)
                    }
                    isKmsSigner
                },
                {
                    fn getMpcThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getMpcThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::getMpcThreshold)
                    }
                    getMpcThreshold
                },
                {
                    fn getUserDecryptionThresholdForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getUserDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainCalls::getUserDecryptionThresholdForContext,
                            )
                    }
                    getUserDecryptionThresholdForContext
                },
                {
                    fn getPublicDecryptionThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getPublicDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainCalls::getPublicDecryptionThreshold,
                            )
                    }
                    getPublicDecryptionThreshold
                },
                {
                    fn getMirroredContextSource(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getMirroredContextSourceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::getMirroredContextSource)
                    }
                    getMirroredContextSource
                },
                {
                    fn reinitializeV2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <reinitializeV2Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::reinitializeV2)
                    }
                    reinitializeV2
                },
                {
                    fn getKmsNodeForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getKmsNodeForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::getKmsNodeForContext)
                    }
                    getKmsNodeForContext
                },
                {
                    fn getKmsGenThresholdForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getKmsGenThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainCalls::getKmsGenThresholdForContext,
                            )
                    }
                    getKmsGenThresholdForContext
                },
                {
                    fn isKmsTxSenderForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <isKmsTxSenderForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::isKmsTxSenderForContext)
                    }
                    isKmsTxSenderForContext
                },
                {
                    fn getMpcThresholdForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getMpcThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainCalls::getMpcThresholdForContext,
                            )
                    }
                    getMpcThresholdForContext
                },
                {
                    fn initializeFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::initializeFromEmptyProxy)
                    }
                    initializeFromEmptyProxy
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn mirrorKmsContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <mirrorKmsContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::mirrorKmsContext)
                    }
                    mirrorKmsContext
                },
                {
                    fn getKmsSignersForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getKmsSignersForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::getKmsSignersForContext)
                    }
                    getKmsSignersForContext
                },
                {
                    fn getKmsSigners(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getKmsSignersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::getKmsSigners)
                    }
                    getKmsSigners
                },
                {
                    fn mirrorKmsThresholds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <mirrorKmsThresholdsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::mirrorKmsThresholds)
                    }
                    mirrorKmsThresholds
                },
                {
                    fn isKmsSignerForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <isKmsSignerForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::isKmsSignerForContext)
                    }
                    isKmsSignerForContext
                },
                {
                    fn getCurrentKmsContextId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getCurrentKmsContextIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::getCurrentKmsContextId)
                    }
                    getCurrentKmsContextId
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainCalls::UPGRADE_INTERFACE_VERSION,
                            )
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn getKmsGenThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getKmsGenThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::getKmsGenThreshold)
                    }
                    getKmsGenThreshold
                },
                {
                    fn isValidKmsContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <isValidKmsContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::isValidKmsContext)
                    }
                    isValidKmsContext
                },
                {
                    fn getUserDecryptionThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getUserDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainCalls::getUserDecryptionThreshold,
                            )
                    }
                    getUserDecryptionThreshold
                },
                {
                    fn getPublicDecryptionThresholdForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getPublicDecryptionThresholdForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainCalls::getPublicDecryptionThresholdForContext,
                            )
                    }
                    getPublicDecryptionThresholdForContext
                },
                {
                    fn getKmsNodesForContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainCalls> {
                        <getKmsNodesForContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainCalls::getKmsNodesForContext)
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
                Self::getCurrentKmsContextId(inner) => {
                    <getCurrentKmsContextIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::getMirroredContextSource(inner) => {
                    <getMirroredContextSourceCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::isValidKmsContext(inner) => {
                    <isValidKmsContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::mirrorKmsContext(inner) => {
                    <mirrorKmsContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::mirrorKmsContextDestruction(inner) => {
                    <mirrorKmsContextDestructionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::mirrorKmsThresholds(inner) => {
                    <mirrorKmsThresholdsCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::getCurrentKmsContextId(inner) => {
                    <getCurrentKmsContextIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getMirroredContextSource(inner) => {
                    <getMirroredContextSourceCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::isValidKmsContext(inner) => {
                    <isValidKmsContextCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::mirrorKmsContext(inner) => {
                    <mirrorKmsContextCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::mirrorKmsContextDestruction(inner) => {
                    <mirrorKmsContextDestructionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::mirrorKmsThresholds(inner) => {
                    <mirrorKmsThresholdsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::upgradeToAndCall(inner) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`ProtocolConfigMultichain`](self) custom errors.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum ProtocolConfigMultichainErrors {
        #[allow(missing_docs)]
        AddressEmptyCode(AddressEmptyCode),
        #[allow(missing_docs)]
        CurrentKmsContextCannotBeDestroyed(CurrentKmsContextCannotBeDestroyed),
        #[allow(missing_docs)]
        ERC1967InvalidImplementation(ERC1967InvalidImplementation),
        #[allow(missing_docs)]
        ERC1967NonPayable(ERC1967NonPayable),
        #[allow(missing_docs)]
        EmptyKmsNodes(EmptyKmsNodes),
        #[allow(missing_docs)]
        FailedCall(FailedCall),
        #[allow(missing_docs)]
        InvalidHighThreshold(InvalidHighThreshold),
        #[allow(missing_docs)]
        InvalidInitialization(InvalidInitialization),
        #[allow(missing_docs)]
        InvalidKmsContext(InvalidKmsContext),
        #[allow(missing_docs)]
        InvalidNullThreshold(InvalidNullThreshold),
        #[allow(missing_docs)]
        InvalidSourceProtocolConfig(InvalidSourceProtocolConfig),
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
    impl ProtocolConfigMultichainErrors {
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
            [54u8, 191u8, 182u8, 14u8],
            [69u8, 149u8, 252u8, 226u8],
            [76u8, 156u8, 140u8, 227u8],
            [111u8, 79u8, 115u8, 31u8],
            [119u8, 221u8, 190u8, 129u8],
            [132u8, 102u8, 128u8, 74u8],
            [153u8, 150u8, 179u8, 21u8],
            [170u8, 29u8, 73u8, 164u8],
            [179u8, 152u8, 151u8, 159u8],
            [202u8, 168u8, 20u8, 163u8],
            [209u8, 140u8, 79u8, 240u8],
            [214u8, 189u8, 162u8, 117u8],
            [215u8, 230u8, 188u8, 248u8],
            [216u8, 225u8, 131u8, 43u8],
            [224u8, 124u8, 141u8, 186u8],
            [239u8, 213u8, 95u8, 103u8],
            [245u8, 26u8, 246u8, 187u8],
            [249u8, 46u8, 232u8, 169u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ProtocolConfigMultichainErrors {
        const NAME: &'static str = "ProtocolConfigMultichainErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 23usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AddressEmptyCode(_) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CurrentKmsContextCannotBeDestroyed(_) => {
                    <CurrentKmsContextCannotBeDestroyed as alloy_sol_types::SolError>::SELECTOR
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
                Self::FailedCall(_) => {
                    <FailedCall as alloy_sol_types::SolError>::SELECTOR
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
                Self::InvalidSourceProtocolConfig(_) => {
                    <InvalidSourceProtocolConfig as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors>] = &[
                {
                    fn EmptyKmsNodes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <EmptyKmsNodes as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainErrors::EmptyKmsNodes)
                    }
                    EmptyKmsNodes
                },
                {
                    fn KmsSignerSetExceedsProofFormatLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <KmsSignerSetExceedsProofFormatLimit as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainErrors::KmsSignerSetExceedsProofFormatLimit,
                            )
                    }
                    KmsSignerSetExceedsProofFormatLimit
                },
                {
                    fn NotHostOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <NotHostOwner as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ProtocolConfigMultichainErrors::NotHostOwner)
                    }
                    NotHostOwner
                },
                {
                    fn ThresholdExceedsProofFormatLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <ThresholdExceedsProofFormatLimit as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainErrors::ThresholdExceedsProofFormatLimit,
                            )
                    }
                    ThresholdExceedsProofFormatLimit
                },
                {
                    fn KmsNodeNullSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <KmsNodeNullSigner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainErrors::KmsNodeNullSigner)
                    }
                    KmsNodeNullSigner
                },
                {
                    fn InvalidNullThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <InvalidNullThreshold as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainErrors::InvalidNullThreshold)
                    }
                    InvalidNullThreshold
                },
                {
                    fn CurrentKmsContextCannotBeDestroyed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <CurrentKmsContextCannotBeDestroyed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainErrors::CurrentKmsContextCannotBeDestroyed,
                            )
                    }
                    CurrentKmsContextCannotBeDestroyed
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainErrors::ERC1967InvalidImplementation,
                            )
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn NotInitializingFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainErrors::NotInitializingFromEmptyProxy,
                            )
                    }
                    NotInitializingFromEmptyProxy
                },
                {
                    fn InvalidKmsContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <InvalidKmsContext as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainErrors::InvalidKmsContext)
                    }
                    InvalidKmsContext
                },
                {
                    fn KmsNodeNullTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <KmsNodeNullTxSender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainErrors::KmsNodeNullTxSender)
                    }
                    KmsNodeNullTxSender
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainErrors::UUPSUnsupportedProxiableUUID,
                            )
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn InvalidHighThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <InvalidHighThreshold as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainErrors::InvalidHighThreshold)
                    }
                    InvalidHighThreshold
                },
                {
                    fn KmsTxSenderAlreadyRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <KmsTxSenderAlreadyRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainErrors::KmsTxSenderAlreadyRegistered,
                            )
                    }
                    KmsTxSenderAlreadyRegistered
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ProtocolConfigMultichainErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn InvalidSourceProtocolConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <InvalidSourceProtocolConfig as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainErrors::InvalidSourceProtocolConfig,
                            )
                    }
                    InvalidSourceProtocolConfig
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainErrors::UUPSUnauthorizedCallContext,
                            )
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn NonIncreasingKmsContextId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <NonIncreasingKmsContextId as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainErrors::NonIncreasingKmsContextId,
                            )
                    }
                    NonIncreasingKmsContextId
                },
                {
                    fn KmsSignerAlreadyRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <KmsSignerAlreadyRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainErrors::KmsSignerAlreadyRegistered,
                            )
                    }
                    KmsSignerAlreadyRegistered
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigMultichainErrors::InvalidInitialization)
                    }
                    InvalidInitialization
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
            ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors>] = &[
                {
                    fn EmptyKmsNodes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <EmptyKmsNodes as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainErrors::EmptyKmsNodes)
                    }
                    EmptyKmsNodes
                },
                {
                    fn KmsSignerSetExceedsProofFormatLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <KmsSignerSetExceedsProofFormatLimit as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainErrors::KmsSignerSetExceedsProofFormatLimit,
                            )
                    }
                    KmsSignerSetExceedsProofFormatLimit
                },
                {
                    fn NotHostOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <NotHostOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainErrors::NotHostOwner)
                    }
                    NotHostOwner
                },
                {
                    fn ThresholdExceedsProofFormatLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <ThresholdExceedsProofFormatLimit as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainErrors::ThresholdExceedsProofFormatLimit,
                            )
                    }
                    ThresholdExceedsProofFormatLimit
                },
                {
                    fn KmsNodeNullSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <KmsNodeNullSigner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainErrors::KmsNodeNullSigner)
                    }
                    KmsNodeNullSigner
                },
                {
                    fn InvalidNullThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <InvalidNullThreshold as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainErrors::InvalidNullThreshold)
                    }
                    InvalidNullThreshold
                },
                {
                    fn CurrentKmsContextCannotBeDestroyed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <CurrentKmsContextCannotBeDestroyed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainErrors::CurrentKmsContextCannotBeDestroyed,
                            )
                    }
                    CurrentKmsContextCannotBeDestroyed
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainErrors::ERC1967InvalidImplementation,
                            )
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn NotInitializingFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainErrors::NotInitializingFromEmptyProxy,
                            )
                    }
                    NotInitializingFromEmptyProxy
                },
                {
                    fn InvalidKmsContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <InvalidKmsContext as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainErrors::InvalidKmsContext)
                    }
                    InvalidKmsContext
                },
                {
                    fn KmsNodeNullTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <KmsNodeNullTxSender as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainErrors::KmsNodeNullTxSender)
                    }
                    KmsNodeNullTxSender
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainErrors::UUPSUnsupportedProxiableUUID,
                            )
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn InvalidHighThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <InvalidHighThreshold as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainErrors::InvalidHighThreshold)
                    }
                    InvalidHighThreshold
                },
                {
                    fn KmsTxSenderAlreadyRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <KmsTxSenderAlreadyRegistered as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainErrors::KmsTxSenderAlreadyRegistered,
                            )
                    }
                    KmsTxSenderAlreadyRegistered
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn InvalidSourceProtocolConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <InvalidSourceProtocolConfig as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainErrors::InvalidSourceProtocolConfig,
                            )
                    }
                    InvalidSourceProtocolConfig
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainErrors::UUPSUnauthorizedCallContext,
                            )
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn NonIncreasingKmsContextId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <NonIncreasingKmsContextId as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainErrors::NonIncreasingKmsContextId,
                            )
                    }
                    NonIncreasingKmsContextId
                },
                {
                    fn KmsSignerAlreadyRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <KmsSignerAlreadyRegistered as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ProtocolConfigMultichainErrors::KmsSignerAlreadyRegistered,
                            )
                    }
                    KmsSignerAlreadyRegistered
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigMultichainErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigMultichainErrors::InvalidInitialization)
                    }
                    InvalidInitialization
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
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::InvalidSourceProtocolConfig(inner) => {
                    <InvalidSourceProtocolConfig as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
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
                Self::InvalidSourceProtocolConfig(inner) => {
                    <InvalidSourceProtocolConfig as alloy_sol_types::SolError>::abi_encode_raw(
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
    ///Container for all the [`ProtocolConfigMultichain`](self) events.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum ProtocolConfigMultichainEvents {
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        MirroredKmsContext(MirroredKmsContext),
        #[allow(missing_docs)]
        MirroredKmsContextDestroyed(MirroredKmsContextDestroyed),
        #[allow(missing_docs)]
        MirroredKmsThresholds(MirroredKmsThresholds),
        #[allow(missing_docs)]
        Upgraded(Upgraded),
    }
    #[automatically_derived]
    impl ProtocolConfigMultichainEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                51u8, 211u8, 226u8, 64u8, 108u8, 198u8, 28u8, 118u8, 116u8, 26u8, 140u8,
                127u8, 238u8, 39u8, 181u8, 32u8, 234u8, 153u8, 134u8, 97u8, 202u8, 206u8,
                73u8, 193u8, 82u8, 51u8, 105u8, 231u8, 253u8, 52u8, 15u8, 22u8,
            ],
            [
                63u8, 93u8, 87u8, 211u8, 80u8, 141u8, 115u8, 238u8, 249u8, 55u8, 77u8,
                14u8, 226u8, 148u8, 3u8, 120u8, 108u8, 180u8, 91u8, 251u8, 102u8, 29u8,
                71u8, 202u8, 212u8, 173u8, 105u8, 22u8, 224u8, 118u8, 12u8, 57u8,
            ],
            [
                142u8, 201u8, 145u8, 233u8, 176u8, 166u8, 150u8, 195u8, 58u8, 250u8,
                95u8, 35u8, 39u8, 192u8, 80u8, 119u8, 125u8, 251u8, 1u8, 104u8, 125u8,
                81u8, 35u8, 126u8, 45u8, 104u8, 62u8, 147u8, 136u8, 182u8, 71u8, 118u8,
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
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for ProtocolConfigMultichainEvents {
        const NAME: &'static str = "ProtocolConfigMultichainEvents";
        const COUNT: usize = 5usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::Initialized)
                }
                Some(
                    <MirroredKmsContext as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <MirroredKmsContext as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::MirroredKmsContext)
                }
                Some(
                    <MirroredKmsContextDestroyed as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <MirroredKmsContextDestroyed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::MirroredKmsContextDestroyed)
                }
                Some(
                    <MirroredKmsThresholds as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <MirroredKmsThresholds as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::MirroredKmsThresholds)
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
    impl alloy_sol_types::private::IntoLogData for ProtocolConfigMultichainEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MirroredKmsContext(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MirroredKmsContextDestroyed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MirroredKmsThresholds(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MirroredKmsContext(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MirroredKmsContextDestroyed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MirroredKmsThresholds(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ProtocolConfigMultichain`](self) contract instance.

See the [wrapper's documentation](`ProtocolConfigMultichainInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ProtocolConfigMultichainInstance<P, N> {
        ProtocolConfigMultichainInstance::<P, N>::new(address, provider)
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
        Output = alloy_contract::Result<ProtocolConfigMultichainInstance<P, N>>,
    > {
        ProtocolConfigMultichainInstance::<P, N>::deploy(provider)
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
        ProtocolConfigMultichainInstance::<P, N>::deploy_builder(provider)
    }
    /**A [`ProtocolConfigMultichain`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ProtocolConfigMultichain`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ProtocolConfigMultichainInstance<
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for ProtocolConfigMultichainInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ProtocolConfigMultichainInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ProtocolConfigMultichainInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`ProtocolConfigMultichain`](self) contract instance.

See the [wrapper's documentation](`ProtocolConfigMultichainInstance`) for more details.*/
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
        ) -> alloy_contract::Result<ProtocolConfigMultichainInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> ProtocolConfigMultichainInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ProtocolConfigMultichainInstance<P, N> {
            ProtocolConfigMultichainInstance {
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
    > ProtocolConfigMultichainInstance<P, N> {
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
        ///Creates a new call builder for the [`getCurrentKmsContextId`] function.
        pub fn getCurrentKmsContextId(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getCurrentKmsContextIdCall, N> {
            self.call_builder(&getCurrentKmsContextIdCall)
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
        ///Creates a new call builder for the [`getMirroredContextSource`] function.
        pub fn getMirroredContextSource(
            &self,
            contextId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getMirroredContextSourceCall, N> {
            self.call_builder(
                &getMirroredContextSourceCall {
                    contextId,
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
            initialContextId: alloy::sol_types::private::primitives::aliases::U256,
            initialKmsNodeParams: alloy::sol_types::private::Vec<
                <KmsNodeParams as alloy::sol_types::SolType>::RustType,
            >,
            initialThresholds: <IProtocolConfigCommon::KmsThresholds as alloy::sol_types::SolType>::RustType,
            softwareVersion: alloy::sol_types::private::String,
            pcrValues: alloy::sol_types::private::Vec<
                <PcrValues as alloy::sol_types::SolType>::RustType,
            >,
            source: <IProtocolConfigMultichain::MirroredContextSource as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, initializeFromEmptyProxyCall, N> {
            self.call_builder(
                &initializeFromEmptyProxyCall {
                    initialContextId,
                    initialKmsNodeParams,
                    initialThresholds,
                    softwareVersion,
                    pcrValues,
                    source,
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
        ///Creates a new call builder for the [`mirrorKmsContext`] function.
        pub fn mirrorKmsContext(
            &self,
            contextId: alloy::sol_types::private::primitives::aliases::U256,
            kmsNodeParams: alloy::sol_types::private::Vec<
                <KmsNodeParams as alloy::sol_types::SolType>::RustType,
            >,
            thresholds: <IProtocolConfigCommon::KmsThresholds as alloy::sol_types::SolType>::RustType,
            softwareVersion: alloy::sol_types::private::String,
            pcrValues: alloy::sol_types::private::Vec<
                <PcrValues as alloy::sol_types::SolType>::RustType,
            >,
            source: <IProtocolConfigMultichain::MirroredContextSource as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, mirrorKmsContextCall, N> {
            self.call_builder(
                &mirrorKmsContextCall {
                    contextId,
                    kmsNodeParams,
                    thresholds,
                    softwareVersion,
                    pcrValues,
                    source,
                },
            )
        }
        ///Creates a new call builder for the [`mirrorKmsContextDestruction`] function.
        pub fn mirrorKmsContextDestruction(
            &self,
            contextId: alloy::sol_types::private::primitives::aliases::U256,
            source: <IProtocolConfigMultichain::MirroredContextSource as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, mirrorKmsContextDestructionCall, N> {
            self.call_builder(
                &mirrorKmsContextDestructionCall {
                    contextId,
                    source,
                },
            )
        }
        ///Creates a new call builder for the [`mirrorKmsThresholds`] function.
        pub fn mirrorKmsThresholds(
            &self,
            contextId: alloy::sol_types::private::primitives::aliases::U256,
            thresholds: <IProtocolConfigCommon::KmsThresholds as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, mirrorKmsThresholdsCall, N> {
            self.call_builder(
                &mirrorKmsThresholdsCall {
                    contextId,
                    thresholds,
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
            source: <IProtocolConfigMultichain::MirroredContextSource as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, reinitializeV2Call, N> {
            self.call_builder(&reinitializeV2Call { source })
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
    > ProtocolConfigMultichainInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<&P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`MirroredKmsContext`] event.
        pub fn MirroredKmsContext_filter(
            &self,
        ) -> alloy_contract::Event<&P, MirroredKmsContext, N> {
            self.event_filter::<MirroredKmsContext>()
        }
        ///Creates a new event filter for the [`MirroredKmsContextDestroyed`] event.
        pub fn MirroredKmsContextDestroyed_filter(
            &self,
        ) -> alloy_contract::Event<&P, MirroredKmsContextDestroyed, N> {
            self.event_filter::<MirroredKmsContextDestroyed>()
        }
        ///Creates a new event filter for the [`MirroredKmsThresholds`] event.
        pub fn MirroredKmsThresholds_filter(
            &self,
        ) -> alloy_contract::Event<&P, MirroredKmsThresholds, N> {
            self.event_filter::<MirroredKmsThresholds>()
        }
        ///Creates a new event filter for the [`Upgraded`] event.
        pub fn Upgraded_filter(&self) -> alloy_contract::Event<&P, Upgraded, N> {
            self.event_filter::<Upgraded>()
        }
    }
}
