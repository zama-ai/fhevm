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
    error AddressEmptyCode(address target);
    error CrsNotGenerated(uint256 crsId);
    error ECDSAInvalidSignature();
    error ECDSAInvalidSignatureLength(uint256 length);
    error ECDSAInvalidSignatureS(bytes32 s);
    error ERC1967InvalidImplementation(address implementation);
    error ERC1967NonPayable();
    error FailedCall();
    error HostChainNotRegistered(uint256 chainId);
    error InvalidInitialization();
    error KeyNotGenerated(uint256 keyId);
    error KmsAlreadySignedForCrsgen(uint256 crsId, address kmsSigner);
    error KmsAlreadySignedForKeygen(uint256 keyId, address kmsSigner);
    error KmsAlreadySignedForPrepKeygen(uint256 prepKeygenId, address kmsSigner);
    error NotCoprocessorSigner(address signerAddress);
    error NotCoprocessorTxSender(address txSenderAddress);
    error NotCustodianSigner(address signerAddress);
    error NotCustodianTxSender(address txSenderAddress);
    error NotGatewayOwner(address sender);
    error NotInitializing();
    error NotInitializingFromEmptyProxy();
    error NotKmsSigner(address signerAddress);
    error NotKmsTxSender(address txSenderAddress);
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);

    event ActivateCrs(uint256 crsId, string[] kmsNodeStorageUrls, bytes crsDigest);
    event ActivateKey(uint256 keyId, string[] kmsNodeStorageUrls, IKMSGeneration.KeyDigest[] keyDigests);
    event CrsgenRequest(uint256 crsId, uint256 maxBitLength, IKMSGeneration.ParamsType paramsType);
    event EIP712DomainChanged();
    event Initialized(uint64 version);
    event KeygenRequest(uint256 prepKeygenId, uint256 keyId);
    event PRSSInit();
    event PrepKeygenRequest(uint256 prepKeygenId, uint256 epochId, IKMSGeneration.ParamsType paramsType);
    event RetryKeygenReshare(uint256 prepKeygenId, uint256 keyId, uint256 epochId, IKMSGeneration.ParamsType paramsType);
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
    function keygen(IKMSGeneration.ParamsType paramsType) external;
    function keygenResponse(uint256 keyId, IKMSGeneration.KeyDigest[] memory keyDigests, bytes memory signature) external;
    function prepKeygenResponse(uint256 prepKeygenId, bytes memory signature) external;
    function proxiableUUID() external view returns (bytes32);
    function prssInit() external;
    function reinitializeV2() external;
    function retryKeygenReshare(uint256 keyId) external;
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
    "name": "reinitializeV2",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "retryKeygenReshare",
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
    "name": "RetryKeygenReshare",
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
    ///0x60a06040523073ffffffffffffffffffffffffffffffffffffffff1660809073ffffffffffffffffffffffffffffffffffffffff1681525034801562000043575f80fd5b50620000546200005a60201b60201c565b620001c4565b5f6200006b6200015e60201b60201c565b9050805f0160089054906101000a900460ff1615620000b6576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b67ffffffffffffffff8016815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff16146200015b5767ffffffffffffffff815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d267ffffffffffffffff604051620001529190620001a9565b60405180910390a15b50565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b5f67ffffffffffffffff82169050919050565b620001a38162000185565b82525050565b5f602082019050620001be5f83018462000198565b92915050565b608051615a75620001eb5f395f8181612bdc01528181612c310152612ed30152615a755ff3fe608060405260043610610129575f3560e01c806362978787116100aa578063baff211e1161006e578063baff211e146103b4578063c4115874146103de578063c55b8724146103f4578063caa367db14610431578063d2c08a9114610459578063d52f10eb1461048157610129565b806362978787146102df5780637514a2ac1461030757806384b0196e1461031d578063936608ae1461034d578063ad3cb1cc1461038a57610129565b806345af261b116100f157806345af261b1461020d5780634610ffe8146102495780634f1ef2861461027157806352d1902d1461028d578063589adb0e146102b757610129565b80630d8e6e2c1461012d57806316c713d91461015757806319f4f6321461019357806339f73810146101cf5780633c02f834146101e5575b5f80fd5b348015610138575f80fd5b506101416104ab565b60405161014e9190613c97565b60405180910390f35b348015610162575f80fd5b5061017d60048036038101906101789190613cfb565b610526565b60405161018a9190613e0d565b60405180910390f35b34801561019e575f80fd5b506101b960048036038101906101b49190613cfb565b6105f7565b6040516101c69190613ea0565b60405180910390f35b3480156101da575f80fd5b506101e36106a4565b005b3480156101f0575f80fd5b5061020b60048036038101906102069190613edc565b610913565b005b348015610218575f80fd5b50610233600480360381019061022e9190613cfb565b610ac2565b6040516102409190613ea0565b60405180910390f35b348015610254575f80fd5b5061026f600480360381019061026a9190613fd0565b610b57565b005b61028b600480360381019061028691906141b3565b61109c565b005b348015610298575f80fd5b506102a16110bb565b6040516102ae9190614225565b60405180910390f35b3480156102c2575f80fd5b506102dd60048036038101906102d8919061423e565b6110ec565b005b3480156102ea575f80fd5b506103056004803603810190610300919061429b565b61143e565b005b348015610312575f80fd5b5061031b61191f565b005b348015610328575f80fd5b50610331611a3d565b604051610344979695949392919061443b565b60405180910390f35b348015610358575f80fd5b50610373600480360381019061036e9190613cfb565b611b46565b60405161038192919061474d565b60405180910390f35b348015610395575f80fd5b5061039e611eed565b6040516103ab9190613c97565b60405180910390f35b3480156103bf575f80fd5b506103c8611f26565b6040516103d59190614782565b60405180910390f35b3480156103e9575f80fd5b506103f2611f3d565b005b3480156103ff575f80fd5b5061041a60048036038101906104159190613cfb565b61208d565b6040516104289291906147e3565b60405180910390f35b34801561043c575f80fd5b5061045760048036038101906104529190614818565b61239f565b005b348015610464575f80fd5b5061047f600480360381019061047a9190613cfb565b6125a9565b005b34801561048c575f80fd5b506104956127a4565b6040516104a29190614782565b60405180910390f35b60606040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506104ec5f6127bb565b6104f660016127bb565b6104ff5f6127bb565b6040516020016105129493929190614911565b604051602081830303815290604052905090565b60605f610531612885565b90505f816003015f8581526020019081526020015f20549050816002015f8581526020019081526020015f205f8281526020019081526020015f208054806020026020016040519081016040528092919081815260200182805480156105e957602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116105a0575b505050505092505050919050565b5f80610601612885565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff1661066457826040517f84de133100000000000000000000000000000000000000000000000000000000815260040161065b9190614782565b60405180910390fd5b5f816006015f8581526020019081526020015f2054905081600d015f8281526020019081526020015f205f9054906101000a900460ff1692505050919050565b60016106ae6128ac565b67ffffffffffffffff16146106ef576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60035f6106fa6128d0565b9050805f0160089054906101000a900460ff168061074257508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15610779576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055506108326040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506040518060400160405280600181526020017f31000000000000000000000000000000000000000000000000000000000000008152506128f7565b5f61083b612885565b905060f86003600681111561085357610852613e2d565b5b901b816004018190555060f86004600681111561087357610872613e2d565b5b901b816005018190555060f86005600681111561089357610892613e2d565b5b901b816009018190555060f86006808111156108b2576108b1613e2d565b5b901b81600e0181905550505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2826040516109079190614991565b60405180910390a15050565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610970573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061099491906149be565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610a0357336040517f0e56cf3d0000000000000000000000000000000000000000000000000000000081526004016109fa91906149e9565b60405180910390fd5b5f610a0c612885565b9050806009015f815480929190610a2290614a2f565b91905055505f816009015490508382600a015f8381526020019081526020015f20819055508282600d015f8381526020019081526020015f205f6101000a81548160ff02191690836001811115610a7c57610a7b613e2d565b5b02179055507f3f038f6f88cb3031b7718588403a2ec220576a868be07dde4c02b846ca352ef5818585604051610ab493929190614a76565b60405180910390a150505050565b5f80610acc612885565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff16610b2f57826040517fda32d00f000000000000000000000000000000000000000000000000000000008152600401610b269190614782565b60405180910390fd5b80600d015f8481526020019081526020015f205f9054906101000a900460ff16915050919050565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663e5275eaf336040518263ffffffff1660e01b8152600401610ba491906149e9565b602060405180830381865afa158015610bbf573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610be39190614ae0565b610c2457336040517faee86323000000000000000000000000000000000000000000000000000000008152600401610c1b91906149e9565b60405180910390fd5b5f610c2d612885565b90505f816006015f8881526020019081526020015f205490505f610c538289898961290d565b90505f610c61828787612ae4565b9050835f015f8a81526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615610d015788816040517f98fb957d000000000000000000000000000000000000000000000000000000008152600401610cf8929190614b0b565b60405180910390fd5b6001845f015f8b81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f846002015f8b81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505f81805490509050856001015f8c81526020019081526020015f205f9054906101000a900460ff16158015610e245750610e2381612b49565b5b1561108f576001866001015f8d81526020019081526020015f205f6101000a81548160ff0219169083151502179055505f5b8a8a9050811015610eda57866007015f8d81526020019081526020015f208b8b83818110610e8757610e86614b32565b5b9050602002810190610e999190614b6b565b908060018154018082558091505060019003905f5260205f2090600202015f909190919091508181610ecb9190614fa7565b50508080600101915050610e56565b5083866003015f8d81526020019081526020015f20819055508a86600801819055505f8167ffffffffffffffff811115610f1757610f1661408f565b5b604051908082528060200260200182016040528015610f4a57816020015b6060815260200190600190039081610f355790505b5090505f5b8281101561104f5773a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663e3b2a874858381548110610f9a57610f99614b32565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b8152600401610fde91906149e9565b5f60405180830381865afa158015610ff8573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f820116820180604052508101906110209190615108565b6060015182828151811061103757611036614b32565b5b60200260200101819052508080600101915050610f4f565b507feb85c26dbcad46b80a68a0f24cce7c2c90f0a1faded84184138839fc9e80a25b8c828d8d6040516110859493929190615320565b60405180910390a1505b5050505050505050505050565b6110a4612bda565b6110ad82612cc0565b6110b78282612db3565b5050565b5f6110c4612ed1565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663e5275eaf336040518263ffffffff1660e01b815260040161113991906149e9565b602060405180830381865afa158015611154573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111789190614ae0565b6111b957336040517faee863230000000000000000000000000000000000000000000000000000000081526004016111b091906149e9565b60405180910390fd5b5f6111c2612885565b90505f6111ce85612f58565b90505f6111dc828686612ae4565b9050825f015f8781526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff161561127c5785816040517f33ca1fe3000000000000000000000000000000000000000000000000000000008152600401611273929190614b0b565b60405180910390fd5b6001835f015f8881526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f836002015f8881526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550836001015f8881526020019081526020015f205f9054906101000a900460ff1615801561139b575061139a8180549050612b49565b5b15611435576001846001015f8981526020019081526020015f205f6101000a81548160ff02191690831515021790555082846003015f8981526020019081526020015f20819055505f846006015f8981526020019081526020015f205490507f78b179176d1f19d7c28e80823deba2624da2ca2ec64b1701f3632a87c9aedc92888260405161142b929190615365565b60405180910390a1505b50505050505050565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663e5275eaf336040518263ffffffff1660e01b815260040161148b91906149e9565b602060405180830381865afa1580156114a6573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114ca9190614ae0565b61150b57336040517faee8632300000000000000000000000000000000000000000000000000000000815260040161150291906149e9565b60405180910390fd5b5f611514612885565b90505f81600a015f8881526020019081526020015f205490505f61153a88838989612fb0565b90505f611548828787612ae4565b9050835f015f8a81526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16156115e85788816040517ffcf5a6e90000000000000000000000000000000000000000000000000000000081526004016115df929190614b0b565b60405180910390fd5b6001845f015f8b81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f846002015f8b81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505f81805490509050856001015f8c81526020019081526020015f205f9054906101000a900460ff1615801561170b575061170a81612b49565b5b15611912576001866001015f8d81526020019081526020015f205f6101000a81548160ff021916908315150217905550898987600b015f8e81526020019081526020015f20918261175d929190614e86565b5083866003015f8d81526020019081526020015f20819055508a86600c01819055505f8167ffffffffffffffff81111561179a5761179961408f565b5b6040519080825280602002602001820160405280156117cd57816020015b60608152602001906001900390816117b85790505b5090505f5b828110156118d25773a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663e3b2a87485838154811061181d5761181c614b32565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b815260040161186191906149e9565b5f60405180830381865afa15801561187b573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f820116820180604052508101906118a39190615108565b606001518282815181106118ba576118b9614b32565b5b602002602001018190525080806001019150506117d2565b507f2258b73faed33fb2e2ea454403bef974920caf682ab3a723484fcf67553b16a28c828d8d60405161190894939291906153b8565b60405180910390a1505b5050505050505050505050565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561197c573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119a091906149be565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614611a0f57336040517f0e56cf3d000000000000000000000000000000000000000000000000000000008152600401611a0691906149e9565b60405180910390fd5b7f11db42c1878f2e2819241f5250984563f06cf22818e7adb86a66921d15d59d3f60405160405180910390a1565b5f6060805f805f60605f611a4f613037565b90505f801b815f0154148015611a6a57505f801b8160010154145b611aa9576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611aa090615447565b60405180910390fd5b611ab161305e565b611ab96130fc565b46305f801b5f67ffffffffffffffff811115611ad857611ad761408f565b5b604051908082528060200260200182016040528015611b065781602001602082028036833780820191505090505b507f0f0000000000000000000000000000000000000000000000000000000000000095949392919097509750975097509750975097505090919293949596565b6060805f611b52612885565b9050806001015f8581526020019081526020015f205f9054906101000a900460ff16611bb557836040517f84de1331000000000000000000000000000000000000000000000000000000008152600401611bac9190614782565b60405180910390fd5b5f816003015f8681526020019081526020015f205490505f826002015f8781526020019081526020015f205f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020018280548015611c6c57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311611c23575b505050505090505f815190505f8167ffffffffffffffff811115611c9357611c9261408f565b5b604051908082528060200260200182016040528015611cc657816020015b6060815260200190600190039081611cb15790505b5090505f5b82811015611dab5773a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663e3b2a874858381518110611d1657611d15614b32565b5b60200260200101516040518263ffffffff1660e01b8152600401611d3a91906149e9565b5f60405180830381865afa158015611d54573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f82011682018060405250810190611d7c9190615108565b60600151828281518110611d9357611d92614b32565b5b60200260200101819052508080600101915050611ccb565b5080856007015f8a81526020019081526020015f2080805480602002602001604051908101604052809291908181526020015f905b82821015611ed9578382905f5260205f2090600202016040518060400160405290815f82015f9054906101000a900460ff166001811115611e2457611e23613e2d565b5b6001811115611e3657611e35613e2d565b5b8152602001600182018054611e4a90614cb9565b80601f0160208091040260200160405190810160405280929190818152602001828054611e7690614cb9565b8015611ec15780601f10611e9857610100808354040283529160200191611ec1565b820191905f5260205f20905b815481529060010190602001808311611ea457829003601f168201915b50505050508152505081526020019060010190611de0565b505050509050965096505050505050915091565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b5f80611f30612885565b905080600c015491505090565b60035f611f486128d0565b9050805f0160089054906101000a900460ff1680611f9057508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15611fc7576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055505f612015612885565b905060f860068081111561202c5761202b613e2d565b5b901b81600e0181905550505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2826040516120819190614991565b60405180910390a15050565b6060805f612099612885565b9050806001015f8581526020019081526020015f205f9054906101000a900460ff166120fc57836040517fda32d00f0000000000000000000000000000000000000000000000000000000081526004016120f39190614782565b60405180910390fd5b5f816003015f8681526020019081526020015f205490505f826002015f8781526020019081526020015f205f8381526020019081526020015f208054806020026020016040519081016040528092919081815260200182805480156121b357602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001906001019080831161216a575b505050505090505f815190505f8167ffffffffffffffff8111156121da576121d961408f565b5b60405190808252806020026020018201604052801561220d57816020015b60608152602001906001900390816121f85790505b5090505f5b828110156122f25773a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663e3b2a87485838151811061225d5761225c614b32565b5b60200260200101516040518263ffffffff1660e01b815260040161228191906149e9565b5f60405180830381865afa15801561229b573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f820116820180604052508101906122c39190615108565b606001518282815181106122da576122d9614b32565b5b60200260200101819052508080600101915050612212565b508085600b015f8a81526020019081526020015f2080805461231390614cb9565b80601f016020809104026020016040519081016040528092919081815260200182805461233f90614cb9565b801561238a5780601f106123615761010080835404028352916020019161238a565b820191905f5260205f20905b81548152906001019060200180831161236d57829003601f168201915b50505050509050965096505050505050915091565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156123fc573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061242091906149be565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461248f57336040517f0e56cf3d00000000000000000000000000000000000000000000000000000000815260040161248691906149e9565b60405180910390fd5b5f612498612885565b9050806004015f8154809291906124ae90614a2f565b91905055505f81600401549050816005015f8154809291906124cf90614a2f565b91905055505f8260050154905080836006015f8481526020019081526020015f208190555081836006015f8381526020019081526020015f208190555082600e015f81548092919061252090614a2f565b91905055505f83600e015490508484600d015f8581526020019081526020015f205f6101000a81548160ff0219169083600181111561256257612561613e2d565b5b02179055507f02024007d96574dbc9d11328bfee9893e7c7bb4ef4aa806df33bfdf454eb5e6083828760405161259a93929190614a76565b60405180910390a15050505050565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612606573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061262a91906149be565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461269957336040517f0e56cf3d00000000000000000000000000000000000000000000000000000000815260040161269091906149e9565b60405180910390fd5b5f6126a2612885565b9050806001015f8381526020019081526020015f205f9054906101000a900460ff1661270557816040517f84de13310000000000000000000000000000000000000000000000000000000081526004016126fc9190614782565b60405180910390fd5b5f816006015f8481526020019081526020015f205490505f82600d015f8381526020019081526020015f205f9054906101000a900460ff16905082600e015f81548092919061275390614a2f565b91905055505f83600e015490507f7cf0be262c7aec97bcb24f48cb25550722e8dd371d79c42f6698f3bb27005731838683856040516127959493929190615465565b60405180910390a15050505050565b5f806127ae612885565b9050806008015491505090565b60605f60016127c98461319a565b0190505f8167ffffffffffffffff8111156127e7576127e661408f565b5b6040519080825280601f01601f1916602001820160405280156128195781602001600182028036833780820191505090505b5090505f82602001820190505b60011561287a578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a858161286f5761286e6154a8565b5b0494505f8503612826575b819350505050919050565b5f7f0b8fdb1f0a6356dd20a6cbc6f9668fac23b85f96575d10e333e603faa794ac00905090565b5f6128b56128d0565b5f015f9054906101000a900467ffffffffffffffff16905090565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b6128ff6132eb565b612909828261332b565b5050565b5f808383905067ffffffffffffffff81111561292c5761292b61408f565b5b60405190808252806020026020018201604052801561295a5781602001602082028036833780820191505090505b5090505f5b84849050811015612a5e57604051806060016040528060258152602001615a50602591398051906020012085858381811061299d5761299c614b32565b5b90506020028101906129af9190614b6b565b5f0160208101906129c091906154d5565b8686848181106129d3576129d2614b32565b5b90506020028101906129e59190614b6b565b80602001906129f49190614c20565b604051612a0292919061552e565b6040518091039020604051602001612a1c93929190615555565b60405160208183030381529060405280519060200120828281518110612a4557612a44614b32565b5b602002602001018181525050808060010191505061295f565b50612ad96040518060a00160405280607281526020016159de6072913980519060200120878784604051602001612a95919061563b565b60405160208183030381529060405280519060200120604051602001612abe9493929190615651565b6040516020818303038152906040528051906020012061337c565b915050949350505050565b5f80612b338585858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050613395565b9050612b3e816133bf565b809150509392505050565b5f8073a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663b4722bc46040518163ffffffff1660e01b8152600401602060405180830381865afa158015612ba8573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612bcc91906156a8565b905080831015915050919050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff161480612c8757507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16612c6e61348f565b73ffffffffffffffffffffffffffffffffffffffff1614155b15612cbe576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612d1d573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612d4191906149be565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614612db057336040517f0e56cf3d000000000000000000000000000000000000000000000000000000008152600401612da791906149e9565b60405180910390fd5b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa925050508015612e1b57506040513d601f19601f82011682018060405250810190612e1891906156fd565b60015b612e5c57816040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401612e5391906149e9565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b8114612ec257806040517faa1d49a4000000000000000000000000000000000000000000000000000000008152600401612eb99190614225565b60405180910390fd5b612ecc83836134e2565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff1614612f56576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f612fa96040518060600160405280602c81526020016159b2602c91398051906020012083604051602001612f8e929190615728565b6040516020818303038152906040528051906020012061337c565b9050919050565b5f61302d60405180608001604052806046815260200161596c604691398051906020012086868686604051602001612fe992919061552e565b604051602081830303815290604052805190602001206040516020016130129493929190615651565b6040516020818303038152906040528051906020012061337c565b9050949350505050565b5f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100905090565b60605f613069613037565b905080600201805461307a90614cb9565b80601f01602080910402602001604051908101604052809291908181526020018280546130a690614cb9565b80156130f15780601f106130c8576101008083540402835291602001916130f1565b820191905f5260205f20905b8154815290600101906020018083116130d457829003601f168201915b505050505091505090565b60605f613107613037565b905080600301805461311890614cb9565b80601f016020809104026020016040519081016040528092919081815260200182805461314490614cb9565b801561318f5780601f106131665761010080835404028352916020019161318f565b820191905f5260205f20905b81548152906001019060200180831161317257829003601f168201915b505050505091505090565b5f805f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f01000000000000000083106131f6577a184f03e93ff9f4daa797ed6e38ed64bf6a1f01000000000000000083816131ec576131eb6154a8565b5b0492506040810190505b6d04ee2d6d415b85acef81000000008310613233576d04ee2d6d415b85acef81000000008381613229576132286154a8565b5b0492506020810190505b662386f26fc10000831061326257662386f26fc100008381613258576132576154a8565b5b0492506010810190505b6305f5e100831061328b576305f5e1008381613281576132806154a8565b5b0492506008810190505b61271083106132b05761271083816132a6576132a56154a8565b5b0492506004810190505b606483106132d357606483816132c9576132c86154a8565b5b0492506002810190505b600a83106132e2576001810190505b80915050919050565b6132f3613554565b613329576040517fd7e6bcf800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b6133336132eb565b5f61333c613037565b90508281600201908161334f91906157a7565b508181600301908161336191906157a7565b505f801b815f01819055505f801b8160010181905550505050565b5f61338e613388613572565b83613580565b9050919050565b5f805f806133a386866135c0565b9250925092506133b38282613615565b82935050505092915050565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663203d0114826040518263ffffffff1660e01b815260040161340c91906149e9565b602060405180830381865afa158015613427573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061344b9190614ae0565b61348c57806040517f2a7c6ef600000000000000000000000000000000000000000000000000000000815260040161348391906149e9565b60405180910390fd5b50565b5f6134bb7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b613777565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b6134eb82613780565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f81511115613547576135418282613849565b50613550565b61354f6138c9565b5b5050565b5f61355d6128d0565b5f0160089054906101000a900460ff16905090565b5f61357b613905565b905090565b5f6040517f190100000000000000000000000000000000000000000000000000000000000081528360028201528260228201526042812091505092915050565b5f805f6041845103613600575f805f602087015192506040870151915060608701515f1a90506135f288828585613968565b95509550955050505061360e565b5f600285515f1b9250925092505b9250925092565b5f600381111561362857613627613e2d565b5b82600381111561363b5761363a613e2d565b5b0315613773576001600381111561365557613654613e2d565b5b82600381111561366857613667613e2d565b5b0361369f576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600260038111156136b3576136b2613e2d565b5b8260038111156136c6576136c5613e2d565b5b0361370a57805f1c6040517ffce698f70000000000000000000000000000000000000000000000000000000081526004016137019190614782565b60405180910390fd5b60038081111561371d5761371c613e2d565b5b8260038111156137305761372f613e2d565b5b0361377257806040517fd78bce0c0000000000000000000000000000000000000000000000000000000081526004016137699190614225565b60405180910390fd5b5b5050565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b036137db57806040517f4c9c8ce30000000000000000000000000000000000000000000000000000000081526004016137d291906149e9565b60405180910390fd5b806138077f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b613777565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f808473ffffffffffffffffffffffffffffffffffffffff168460405161387291906158a6565b5f60405180830381855af49150503d805f81146138aa576040519150601f19603f3d011682016040523d82523d5f602084013e6138af565b606091505b50915091506138bf858383613a4f565b9250505092915050565b5f341115613903576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f61392f613adc565b613937613b52565b463060405160200161394d9594939291906158bc565b60405160208183030381529060405280519060200120905090565b5f805f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c11156139a4575f600385925092509250613a45565b5f6001888888886040515f81526020016040526040516139c79493929190615928565b6020604051602081039080840390855afa1580156139e7573d5f803e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603613a38575f60015f801b93509350935050613a45565b805f805f1b935093509350505b9450945094915050565b606082613a6457613a5f82613bc9565b613ad4565b5f8251148015613a8a57505f8473ffffffffffffffffffffffffffffffffffffffff163b145b15613acc57836040517f9996b315000000000000000000000000000000000000000000000000000000008152600401613ac391906149e9565b60405180910390fd5b819050613ad5565b5b9392505050565b5f80613ae6613037565b90505f613af161305e565b90505f81511115613b0d57808051906020012092505050613b4f565b5f825f015490505f801b8114613b2857809350505050613b4f565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f80613b5c613037565b90505f613b676130fc565b90505f81511115613b8357808051906020012092505050613bc6565b5f826001015490505f801b8114613b9f57809350505050613bc6565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f81511115613bdb5780518082602001fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f81519050919050565b5f82825260208201905092915050565b5f5b83811015613c44578082015181840152602081019050613c29565b5f8484015250505050565b5f601f19601f8301169050919050565b5f613c6982613c0d565b613c738185613c17565b9350613c83818560208601613c27565b613c8c81613c4f565b840191505092915050565b5f6020820190508181035f830152613caf8184613c5f565b905092915050565b5f604051905090565b5f80fd5b5f80fd5b5f819050919050565b613cda81613cc8565b8114613ce4575f80fd5b50565b5f81359050613cf581613cd1565b92915050565b5f60208284031215613d1057613d0f613cc0565b5b5f613d1d84828501613ce7565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f613d7882613d4f565b9050919050565b613d8881613d6e565b82525050565b5f613d998383613d7f565b60208301905092915050565b5f602082019050919050565b5f613dbb82613d26565b613dc58185613d30565b9350613dd083613d40565b805f5b83811015613e00578151613de78882613d8e565b9750613df283613da5565b925050600181019050613dd3565b5085935050505092915050565b5f6020820190508181035f830152613e258184613db1565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b60028110613e6b57613e6a613e2d565b5b50565b5f819050613e7b82613e5a565b919050565b5f613e8a82613e6e565b9050919050565b613e9a81613e80565b82525050565b5f602082019050613eb35f830184613e91565b92915050565b60028110613ec5575f80fd5b50565b5f81359050613ed681613eb9565b92915050565b5f8060408385031215613ef257613ef1613cc0565b5b5f613eff85828601613ce7565b9250506020613f1085828601613ec8565b9150509250929050565b5f80fd5b5f80fd5b5f80fd5b5f8083601f840112613f3b57613f3a613f1a565b5b8235905067ffffffffffffffff811115613f5857613f57613f1e565b5b602083019150836020820283011115613f7457613f73613f22565b5b9250929050565b5f8083601f840112613f9057613f8f613f1a565b5b8235905067ffffffffffffffff811115613fad57613fac613f1e565b5b602083019150836001820283011115613fc957613fc8613f22565b5b9250929050565b5f805f805f60608688031215613fe957613fe8613cc0565b5b5f613ff688828901613ce7565b955050602086013567ffffffffffffffff81111561401757614016613cc4565b5b61402388828901613f26565b9450945050604086013567ffffffffffffffff81111561404657614045613cc4565b5b61405288828901613f7b565b92509250509295509295909350565b61406a81613d6e565b8114614074575f80fd5b50565b5f8135905061408581614061565b92915050565b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6140c582613c4f565b810181811067ffffffffffffffff821117156140e4576140e361408f565b5b80604052505050565b5f6140f6613cb7565b905061410282826140bc565b919050565b5f67ffffffffffffffff8211156141215761412061408f565b5b61412a82613c4f565b9050602081019050919050565b828183375f83830152505050565b5f61415761415284614107565b6140ed565b9050828152602081018484840111156141735761417261408b565b5b61417e848285614137565b509392505050565b5f82601f83011261419a57614199613f1a565b5b81356141aa848260208601614145565b91505092915050565b5f80604083850312156141c9576141c8613cc0565b5b5f6141d685828601614077565b925050602083013567ffffffffffffffff8111156141f7576141f6613cc4565b5b61420385828601614186565b9150509250929050565b5f819050919050565b61421f8161420d565b82525050565b5f6020820190506142385f830184614216565b92915050565b5f805f6040848603121561425557614254613cc0565b5b5f61426286828701613ce7565b935050602084013567ffffffffffffffff81111561428357614282613cc4565b5b61428f86828701613f7b565b92509250509250925092565b5f805f805f606086880312156142b4576142b3613cc0565b5b5f6142c188828901613ce7565b955050602086013567ffffffffffffffff8111156142e2576142e1613cc4565b5b6142ee88828901613f7b565b9450945050604086013567ffffffffffffffff81111561431157614310613cc4565b5b61431d88828901613f7b565b92509250509295509295909350565b5f7fff0000000000000000000000000000000000000000000000000000000000000082169050919050565b6143608161432c565b82525050565b61436f81613cc8565b82525050565b61437e81613d6e565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b6143b681613cc8565b82525050565b5f6143c783836143ad565b60208301905092915050565b5f602082019050919050565b5f6143e982614384565b6143f3818561438e565b93506143fe8361439e565b805f5b8381101561442e57815161441588826143bc565b9750614420836143d3565b925050600181019050614401565b5085935050505092915050565b5f60e08201905061444e5f83018a614357565b81810360208301526144608189613c5f565b905081810360408301526144748188613c5f565b90506144836060830187614366565b6144906080830186614375565b61449d60a0830185614216565b81810360c08301526144af81846143df565b905098975050505050505050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f82825260208201905092915050565b5f61450082613c0d565b61450a81856144e6565b935061451a818560208601613c27565b61452381613c4f565b840191505092915050565b5f61453983836144f6565b905092915050565b5f602082019050919050565b5f614557826144bd565b61456181856144c7565b935083602082028501614573856144d7565b805f5b858110156145ae578484038952815161458f858261452e565b945061459a83614541565b925060208a01995050600181019050614576565b50829750879550505050505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b600281106145fa576145f9613e2d565b5b50565b5f81905061460a826145e9565b919050565b5f614619826145fd565b9050919050565b6146298161460f565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f6146538261462f565b61465d8185614639565b935061466d818560208601613c27565b61467681613c4f565b840191505092915050565b5f604083015f8301516146965f860182614620565b50602083015184820360208601526146ae8282614649565b9150508091505092915050565b5f6146c68383614681565b905092915050565b5f602082019050919050565b5f6146e4826145c0565b6146ee81856145ca565b935083602082028501614700856145da565b805f5b8581101561473b578484038952815161471c85826146bb565b9450614727836146ce565b925060208a01995050600181019050614703565b50829750879550505050505092915050565b5f6040820190508181035f830152614765818561454d565b9050818103602083015261477981846146da565b90509392505050565b5f6020820190506147955f830184614366565b92915050565b5f82825260208201905092915050565b5f6147b58261462f565b6147bf818561479b565b93506147cf818560208601613c27565b6147d881613c4f565b840191505092915050565b5f6040820190508181035f8301526147fb818561454d565b9050818103602083015261480f81846147ab565b90509392505050565b5f6020828403121561482d5761482c613cc0565b5b5f61483a84828501613ec8565b91505092915050565b5f81905092915050565b5f61485782613c0d565b6148618185614843565b9350614871818560208601613c27565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f6148b1600283614843565b91506148bc8261487d565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f6148fb600183614843565b9150614906826148c7565b600182019050919050565b5f61491c828761484d565b9150614927826148a5565b9150614933828661484d565b915061493e826148ef565b915061494a828561484d565b9150614955826148ef565b9150614961828461484d565b915081905095945050505050565b5f67ffffffffffffffff82169050919050565b61498b8161496f565b82525050565b5f6020820190506149a45f830184614982565b92915050565b5f815190506149b881614061565b92915050565b5f602082840312156149d3576149d2613cc0565b5b5f6149e0848285016149aa565b91505092915050565b5f6020820190506149fc5f830184614375565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f614a3982613cc8565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203614a6b57614a6a614a02565b5b600182019050919050565b5f606082019050614a895f830186614366565b614a966020830185614366565b614aa36040830184613e91565b949350505050565b5f8115159050919050565b614abf81614aab565b8114614ac9575f80fd5b50565b5f81519050614ada81614ab6565b92915050565b5f60208284031215614af557614af4613cc0565b5b5f614b0284828501614acc565b91505092915050565b5f604082019050614b1e5f830185614366565b614b2b6020830184614375565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b5f82356001604003833603038112614b8657614b85614b5f565b5b80830191505092915050565b60028110614b9e575f80fd5b50565b5f8135614bad81614b92565b80915050919050565b5f815f1b9050919050565b5f60ff614bcd84614bb6565b9350801983169250808416831791505092915050565b5f614bed826145fd565b9050919050565b5f819050919050565b614c0682614be3565b614c19614c1282614bf4565b8354614bc1565b8255505050565b5f8083356001602003843603038112614c3c57614c3b614b5f565b5b80840192508235915067ffffffffffffffff821115614c5e57614c5d614b63565b5b602083019250600182023603831315614c7a57614c79614b67565b5b509250929050565b5f82905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f6002820490506001821680614cd057607f821691505b602082108103614ce357614ce2614c8c565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f60088302614d457fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82614d0a565b614d4f8683614d0a565b95508019841693508086168417925050509392505050565b5f819050919050565b5f614d8a614d85614d8084613cc8565b614d67565b613cc8565b9050919050565b5f819050919050565b614da383614d70565b614db7614daf82614d91565b848454614d16565b825550505050565b5f90565b614dcb614dbf565b614dd6818484614d9a565b505050565b5b81811015614df957614dee5f82614dc3565b600181019050614ddc565b5050565b601f821115614e3e57614e0f81614ce9565b614e1884614cfb565b81016020851015614e27578190505b614e3b614e3385614cfb565b830182614ddb565b50505b505050565b5f82821c905092915050565b5f614e5e5f1984600802614e43565b1980831691505092915050565b5f614e768383614e4f565b9150826002028217905092915050565b614e908383614c82565b67ffffffffffffffff811115614ea957614ea861408f565b5b614eb38254614cb9565b614ebe828285614dfd565b5f601f831160018114614eeb575f8415614ed9578287013590505b614ee38582614e6b565b865550614f4a565b601f198416614ef986614ce9565b5f5b82811015614f2057848901358255600182019150602085019450602081019050614efb565b86831015614f3d5784890135614f39601f891682614e4f565b8355505b6001600288020188555050505b50505050505050565b614f5e838383614e86565b505050565b5f81015f830180614f7381614ba1565b9050614f7f8184614bfd565b5050506001810160208301614f948185614c20565b614f9f818386614f53565b505050505050565b614fb18282614f63565b5050565b5f80fd5b5f80fd5b5f67ffffffffffffffff821115614fd757614fd661408f565b5b614fe082613c4f565b9050602081019050919050565b5f614fff614ffa84614fbd565b6140ed565b90508281526020810184848401111561501b5761501a61408b565b5b615026848285613c27565b509392505050565b5f82601f83011261504257615041613f1a565b5b8151615052848260208601614fed565b91505092915050565b5f608082840312156150705761506f614fb5565b5b61507a60806140ed565b90505f615089848285016149aa565b5f83015250602061509c848285016149aa565b602083015250604082015167ffffffffffffffff8111156150c0576150bf614fb9565b5b6150cc8482850161502e565b604083015250606082015167ffffffffffffffff8111156150f0576150ef614fb9565b5b6150fc8482850161502e565b60608301525092915050565b5f6020828403121561511d5761511c613cc0565b5b5f82015167ffffffffffffffff81111561513a57615139613cc4565b5b6151468482850161505b565b91505092915050565b5f819050919050565b5f8135905061516681614b92565b92915050565b5f61517a6020840184615158565b905092915050565b5f80fd5b5f80fd5b5f80fd5b5f80833560016020038436030381126151aa576151a961518a565b5b83810192508235915060208301925067ffffffffffffffff8211156151d2576151d1615182565b5b6001820236038313156151e8576151e7615186565b5b509250929050565b5f6151fb8385614639565b9350615208838584614137565b61521183613c4f565b840190509392505050565b5f6040830161522d5f84018461516c565b6152395f860182614620565b50615247602084018461518e565b858303602087015261525a8382846151f0565b925050508091505092915050565b5f615273838361521c565b905092915050565b5f823560016040038336030381126152965761529561518a565b5b82810191505092915050565b5f602082019050919050565b5f6152b983856145ca565b9350836020840285016152cb8461514f565b805f5b8781101561530e5784840389526152e5828461527b565b6152ef8582615268565b94506152fa836152a2565b925060208a019950506001810190506152ce565b50829750879450505050509392505050565b5f6060820190506153335f830187614366565b8181036020830152615345818661454d565b9050818103604083015261535a8184866152ae565b905095945050505050565b5f6040820190506153785f830185614366565b6153856020830184614366565b9392505050565b5f615397838561479b565b93506153a4838584614137565b6153ad83613c4f565b840190509392505050565b5f6060820190506153cb5f830187614366565b81810360208301526153dd818661454d565b905081810360408301526153f281848661538c565b905095945050505050565b7f4549503731323a20556e696e697469616c697a656400000000000000000000005f82015250565b5f615431601583613c17565b915061543c826153fd565b602082019050919050565b5f6020820190508181035f83015261545e81615425565b9050919050565b5f6080820190506154785f830187614366565b6154856020830186614366565b6154926040830185614366565b61549f6060830184613e91565b95945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f602082840312156154ea576154e9613cc0565b5b5f6154f784828501615158565b91505092915050565b5f81905092915050565b5f6155158385615500565b9350615522838584614137565b82840190509392505050565b5f61553a82848661550a565b91508190509392505050565b61554f8161460f565b82525050565b5f6060820190506155685f830186614216565b6155756020830185615546565b6155826040830184614216565b949350505050565b5f81519050919050565b5f81905092915050565b5f819050602082019050919050565b6155b68161420d565b82525050565b5f6155c783836155ad565b60208301905092915050565b5f602082019050919050565b5f6155e98261558a565b6155f38185615594565b93506155fe8361559e565b805f5b8381101561562e57815161561588826155bc565b9750615620836155d3565b925050600181019050615601565b5085935050505092915050565b5f61564682846155df565b915081905092915050565b5f6080820190506156645f830187614216565b6156716020830186614366565b61567e6040830185614366565b61568b6060830184614216565b95945050505050565b5f815190506156a281613cd1565b92915050565b5f602082840312156156bd576156bc613cc0565b5b5f6156ca84828501615694565b91505092915050565b6156dc8161420d565b81146156e6575f80fd5b50565b5f815190506156f7816156d3565b92915050565b5f6020828403121561571257615711613cc0565b5b5f61571f848285016156e9565b91505092915050565b5f60408201905061573b5f830185614216565b6157486020830184614366565b9392505050565b5f819050815f5260205f209050919050565b601f8211156157a2576157738161574f565b61577c84614cfb565b8101602085101561578b578190505b61579f61579785614cfb565b830182614ddb565b50505b505050565b6157b082613c0d565b67ffffffffffffffff8111156157c9576157c861408f565b5b6157d38254614cb9565b6157de828285615761565b5f60209050601f83116001811461580f575f84156157fd578287015190505b6158078582614e6b565b86555061586e565b601f19841661581d8661574f565b5f5b828110156158445784890151825560018201915060208501945060208101905061581f565b86831015615861578489015161585d601f891682614e4f565b8355505b6001600288020188555050505b505050505050565b5f6158808261462f565b61588a8185615500565b935061589a818560208601613c27565b80840191505092915050565b5f6158b18284615876565b915081905092915050565b5f60a0820190506158cf5f830188614216565b6158dc6020830187614216565b6158e96040830186614216565b6158f66060830185614366565b6159036080830184614375565b9695505050505050565b5f60ff82169050919050565b6159228161590d565b82525050565b5f60808201905061593b5f830187614216565b6159486020830186615919565b6159556040830185614216565b6159626060830184614216565b9594505050505056fe43727367656e566572696669636174696f6e2875696e743235362063727349642c75696e74323536206d61784269744c656e6774682c62797465732063727344696765737429507265704b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e4964294b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e49642c75696e74323536206b657949642c4b65794469676573745b5d206b657944696765737473294b65794469676573742875696e7438206b6579547970652c627974657320646967657374294b65794469676573742875696e7438206b6579547970652c62797465732064696765737429
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP4\x80\x15b\0\0CW_\x80\xFD[Pb\0\0Tb\0\0Z` \x1B` \x1CV[b\0\x01\xC4V[_b\0\0kb\0\x01^` \x1B` \x1CV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15b\0\0\xB6W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14b\0\x01[Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Qb\0\x01R\x91\x90b\0\x01\xA9V[`@Q\x80\x91\x03\x90\xA1[PV[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[b\0\x01\xA3\x81b\0\x01\x85V[\x82RPPV[_` \x82\x01\x90Pb\0\x01\xBE_\x83\x01\x84b\0\x01\x98V[\x92\x91PPV[`\x80QaZub\0\x01\xEB_9_\x81\x81a+\xDC\x01R\x81\x81a,1\x01Ra.\xD3\x01RaZu_\xF3\xFE`\x80`@R`\x046\x10a\x01)W_5`\xE0\x1C\x80cb\x97\x87\x87\x11a\0\xAAW\x80c\xBA\xFF!\x1E\x11a\0nW\x80c\xBA\xFF!\x1E\x14a\x03\xB4W\x80c\xC4\x11Xt\x14a\x03\xDEW\x80c\xC5[\x87$\x14a\x03\xF4W\x80c\xCA\xA3g\xDB\x14a\x041W\x80c\xD2\xC0\x8A\x91\x14a\x04YW\x80c\xD5/\x10\xEB\x14a\x04\x81Wa\x01)V[\x80cb\x97\x87\x87\x14a\x02\xDFW\x80cu\x14\xA2\xAC\x14a\x03\x07W\x80c\x84\xB0\x19n\x14a\x03\x1DW\x80c\x93f\x08\xAE\x14a\x03MW\x80c\xAD<\xB1\xCC\x14a\x03\x8AWa\x01)V[\x80cE\xAF&\x1B\x11a\0\xF1W\x80cE\xAF&\x1B\x14a\x02\rW\x80cF\x10\xFF\xE8\x14a\x02IW\x80cO\x1E\xF2\x86\x14a\x02qW\x80cR\xD1\x90-\x14a\x02\x8DW\x80cX\x9A\xDB\x0E\x14a\x02\xB7Wa\x01)V[\x80c\r\x8En,\x14a\x01-W\x80c\x16\xC7\x13\xD9\x14a\x01WW\x80c\x19\xF4\xF62\x14a\x01\x93W\x80c9\xF78\x10\x14a\x01\xCFW\x80c<\x02\xF84\x14a\x01\xE5W[_\x80\xFD[4\x80\x15a\x018W_\x80\xFD[Pa\x01Aa\x04\xABV[`@Qa\x01N\x91\x90a<\x97V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01bW_\x80\xFD[Pa\x01}`\x04\x806\x03\x81\x01\x90a\x01x\x91\x90a<\xFBV[a\x05&V[`@Qa\x01\x8A\x91\x90a>\rV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x9EW_\x80\xFD[Pa\x01\xB9`\x04\x806\x03\x81\x01\x90a\x01\xB4\x91\x90a<\xFBV[a\x05\xF7V[`@Qa\x01\xC6\x91\x90a>\xA0V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xDAW_\x80\xFD[Pa\x01\xE3a\x06\xA4V[\0[4\x80\x15a\x01\xF0W_\x80\xFD[Pa\x02\x0B`\x04\x806\x03\x81\x01\x90a\x02\x06\x91\x90a>\xDCV[a\t\x13V[\0[4\x80\x15a\x02\x18W_\x80\xFD[Pa\x023`\x04\x806\x03\x81\x01\x90a\x02.\x91\x90a<\xFBV[a\n\xC2V[`@Qa\x02@\x91\x90a>\xA0V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02TW_\x80\xFD[Pa\x02o`\x04\x806\x03\x81\x01\x90a\x02j\x91\x90a?\xD0V[a\x0BWV[\0[a\x02\x8B`\x04\x806\x03\x81\x01\x90a\x02\x86\x91\x90aA\xB3V[a\x10\x9CV[\0[4\x80\x15a\x02\x98W_\x80\xFD[Pa\x02\xA1a\x10\xBBV[`@Qa\x02\xAE\x91\x90aB%V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xC2W_\x80\xFD[Pa\x02\xDD`\x04\x806\x03\x81\x01\x90a\x02\xD8\x91\x90aB>V[a\x10\xECV[\0[4\x80\x15a\x02\xEAW_\x80\xFD[Pa\x03\x05`\x04\x806\x03\x81\x01\x90a\x03\0\x91\x90aB\x9BV[a\x14>V[\0[4\x80\x15a\x03\x12W_\x80\xFD[Pa\x03\x1Ba\x19\x1FV[\0[4\x80\x15a\x03(W_\x80\xFD[Pa\x031a\x1A=V[`@Qa\x03D\x97\x96\x95\x94\x93\x92\x91\x90aD;V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03XW_\x80\xFD[Pa\x03s`\x04\x806\x03\x81\x01\x90a\x03n\x91\x90a<\xFBV[a\x1BFV[`@Qa\x03\x81\x92\x91\x90aGMV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x95W_\x80\xFD[Pa\x03\x9Ea\x1E\xEDV[`@Qa\x03\xAB\x91\x90a<\x97V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xBFW_\x80\xFD[Pa\x03\xC8a\x1F&V[`@Qa\x03\xD5\x91\x90aG\x82V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xE9W_\x80\xFD[Pa\x03\xF2a\x1F=V[\0[4\x80\x15a\x03\xFFW_\x80\xFD[Pa\x04\x1A`\x04\x806\x03\x81\x01\x90a\x04\x15\x91\x90a<\xFBV[a \x8DV[`@Qa\x04(\x92\x91\x90aG\xE3V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04<W_\x80\xFD[Pa\x04W`\x04\x806\x03\x81\x01\x90a\x04R\x91\x90aH\x18V[a#\x9FV[\0[4\x80\x15a\x04dW_\x80\xFD[Pa\x04\x7F`\x04\x806\x03\x81\x01\x90a\x04z\x91\x90a<\xFBV[a%\xA9V[\0[4\x80\x15a\x04\x8CW_\x80\xFD[Pa\x04\x95a'\xA4V[`@Qa\x04\xA2\x91\x90aG\x82V[`@Q\x80\x91\x03\x90\xF3[```@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x04\xEC_a'\xBBV[a\x04\xF6`\x01a'\xBBV[a\x04\xFF_a'\xBBV[`@Q` \x01a\x05\x12\x94\x93\x92\x91\x90aI\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[``_a\x051a(\x85V[\x90P_\x81`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\x02\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x82\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x05\xE9W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x05\xA0W[PPPPP\x92PPP\x91\x90PV[_\x80a\x06\x01a(\x85V[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x06dW\x82`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06[\x91\x90aG\x82V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\r\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x92PPP\x91\x90PV[`\x01a\x06\xAEa(\xACV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x06\xEFW`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03_a\x06\xFAa(\xD0V[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x07BWP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x07yW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x082`@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa(\xF7V[_a\x08;a(\x85V[\x90P`\xF8`\x03`\x06\x81\x11\x15a\x08SWa\x08Ra>-V[[\x90\x1B\x81`\x04\x01\x81\x90UP`\xF8`\x04`\x06\x81\x11\x15a\x08sWa\x08ra>-V[[\x90\x1B\x81`\x05\x01\x81\x90UP`\xF8`\x05`\x06\x81\x11\x15a\x08\x93Wa\x08\x92a>-V[[\x90\x1B\x81`\t\x01\x81\x90UP`\xF8`\x06\x80\x81\x11\x15a\x08\xB2Wa\x08\xB1a>-V[[\x90\x1B\x81`\x0E\x01\x81\x90UPP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\t\x07\x91\x90aI\x91V[`@Q\x80\x91\x03\x90\xA1PPV[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tpW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x94\x91\x90aI\xBEV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\n\x03W3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xFA\x91\x90aI\xE9V[`@Q\x80\x91\x03\x90\xFD[_a\n\x0Ca(\x85V[\x90P\x80`\t\x01_\x81T\x80\x92\x91\x90a\n\"\x90aJ/V[\x91\x90PUP_\x81`\t\x01T\x90P\x83\x82`\n\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82\x82`\r\x01_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a\n|Wa\n{a>-V[[\x02\x17\x90UP\x7F?\x03\x8Fo\x88\xCB01\xB7q\x85\x88@:.\xC2 Wj\x86\x8B\xE0}\xDEL\x02\xB8F\xCA5.\xF5\x81\x85\x85`@Qa\n\xB4\x93\x92\x91\x90aJvV[`@Q\x80\x91\x03\x90\xA1PPPPV[_\x80a\n\xCCa(\x85V[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x0B/W\x82`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B&\x91\x90aG\x82V[`@Q\x80\x91\x03\x90\xFD[\x80`\r\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE5'^\xAF3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\xA4\x91\x90aI\xE9V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xBFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xE3\x91\x90aJ\xE0V[a\x0C$W3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\x1B\x91\x90aI\xE9V[`@Q\x80\x91\x03\x90\xFD[_a\x0C-a(\x85V[\x90P_\x81`\x06\x01_\x88\x81R` \x01\x90\x81R` \x01_ T\x90P_a\x0CS\x82\x89\x89\x89a)\rV[\x90P_a\x0Ca\x82\x87\x87a*\xE4V[\x90P\x83_\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\r\x01W\x88\x81`@Q\x7F\x98\xFB\x95}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\xF8\x92\x91\x90aK\x0BV[`@Q\x80\x91\x03\x90\xFD[`\x01\x84_\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x84`\x02\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_\x81\x80T\x90P\x90P\x85`\x01\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x0E$WPa\x0E#\x81a+IV[[\x15a\x10\x8FW`\x01\x86`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_[\x8A\x8A\x90P\x81\x10\x15a\x0E\xDAW\x86`\x07\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x8B\x8B\x83\x81\x81\x10a\x0E\x87Wa\x0E\x86aK2V[[\x90P` \x02\x81\x01\x90a\x0E\x99\x91\x90aKkV[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P\x81\x81a\x0E\xCB\x91\x90aO\xA7V[PP\x80\x80`\x01\x01\x91PPa\x0EVV[P\x83\x86`\x03\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x8A\x86`\x08\x01\x81\x90UP_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x17Wa\x0F\x16a@\x8FV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0FJW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0F5W\x90P[P\x90P_[\x82\x81\x10\x15a\x10OWs\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3\xB2\xA8t\x85\x83\x81T\x81\x10a\x0F\x9AWa\x0F\x99aK2V[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\xDE\x91\x90aI\xE9V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xF8W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10 \x91\x90aQ\x08V[``\x01Q\x82\x82\x81Q\x81\x10a\x107Wa\x106aK2V[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa\x0FOV[P\x7F\xEB\x85\xC2m\xBC\xADF\xB8\nh\xA0\xF2L\xCE|,\x90\xF0\xA1\xFA\xDE\xD8A\x84\x13\x889\xFC\x9E\x80\xA2[\x8C\x82\x8D\x8D`@Qa\x10\x85\x94\x93\x92\x91\x90aS V[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPPPPPV[a\x10\xA4a+\xDAV[a\x10\xAD\x82a,\xC0V[a\x10\xB7\x82\x82a-\xB3V[PPV[_a\x10\xC4a.\xD1V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE5'^\xAF3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x119\x91\x90aI\xE9V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11TW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11x\x91\x90aJ\xE0V[a\x11\xB9W3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\xB0\x91\x90aI\xE9V[`@Q\x80\x91\x03\x90\xFD[_a\x11\xC2a(\x85V[\x90P_a\x11\xCE\x85a/XV[\x90P_a\x11\xDC\x82\x86\x86a*\xE4V[\x90P\x82_\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x12|W\x85\x81`@Q\x7F3\xCA\x1F\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12s\x92\x91\x90aK\x0BV[`@Q\x80\x91\x03\x90\xFD[`\x01\x83_\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x83`\x02\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x83`\x01\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x13\x9BWPa\x13\x9A\x81\x80T\x90Pa+IV[[\x15a\x145W`\x01\x84`\x01\x01_\x89\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x82\x84`\x03\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x81\x90UP_\x84`\x06\x01_\x89\x81R` \x01\x90\x81R` \x01_ T\x90P\x7Fx\xB1y\x17m\x1F\x19\xD7\xC2\x8E\x80\x82=\xEB\xA2bM\xA2\xCA.\xC6K\x17\x01\xF3c*\x87\xC9\xAE\xDC\x92\x88\x82`@Qa\x14+\x92\x91\x90aSeV[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPV[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE5'^\xAF3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\x8B\x91\x90aI\xE9V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xA6W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xCA\x91\x90aJ\xE0V[a\x15\x0BW3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15\x02\x91\x90aI\xE9V[`@Q\x80\x91\x03\x90\xFD[_a\x15\x14a(\x85V[\x90P_\x81`\n\x01_\x88\x81R` \x01\x90\x81R` \x01_ T\x90P_a\x15:\x88\x83\x89\x89a/\xB0V[\x90P_a\x15H\x82\x87\x87a*\xE4V[\x90P\x83_\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x15\xE8W\x88\x81`@Q\x7F\xFC\xF5\xA6\xE9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15\xDF\x92\x91\x90aK\x0BV[`@Q\x80\x91\x03\x90\xFD[`\x01\x84_\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x84`\x02\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_\x81\x80T\x90P\x90P\x85`\x01\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x17\x0BWPa\x17\n\x81a+IV[[\x15a\x19\x12W`\x01\x86`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x89\x89\x87`\x0B\x01_\x8E\x81R` \x01\x90\x81R` \x01_ \x91\x82a\x17]\x92\x91\x90aN\x86V[P\x83\x86`\x03\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x8A\x86`\x0C\x01\x81\x90UP_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\x9AWa\x17\x99a@\x8FV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\xCDW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x17\xB8W\x90P[P\x90P_[\x82\x81\x10\x15a\x18\xD2Ws\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3\xB2\xA8t\x85\x83\x81T\x81\x10a\x18\x1DWa\x18\x1CaK2V[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18a\x91\x90aI\xE9V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18{W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xA3\x91\x90aQ\x08V[``\x01Q\x82\x82\x81Q\x81\x10a\x18\xBAWa\x18\xB9aK2V[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa\x17\xD2V[P\x7F\"X\xB7?\xAE\xD3?\xB2\xE2\xEAED\x03\xBE\xF9t\x92\x0C\xAFh*\xB3\xA7#HO\xCFgU;\x16\xA2\x8C\x82\x8D\x8D`@Qa\x19\x08\x94\x93\x92\x91\x90aS\xB8V[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPPPPPV[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19|W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xA0\x91\x90aI\xBEV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x1A\x0FW3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1A\x06\x91\x90aI\xE9V[`@Q\x80\x91\x03\x90\xFD[\x7F\x11\xDBB\xC1\x87\x8F.(\x19$\x1FRP\x98Ec\xF0l\xF2(\x18\xE7\xAD\xB8jf\x92\x1D\x15\xD5\x9D?`@Q`@Q\x80\x91\x03\x90\xA1V[_``\x80_\x80_``_a\x1AOa07V[\x90P_\x80\x1B\x81_\x01T\x14\x80\x15a\x1AjWP_\x80\x1B\x81`\x01\x01T\x14[a\x1A\xA9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1A\xA0\x90aTGV[`@Q\x80\x91\x03\x90\xFD[a\x1A\xB1a0^V[a\x1A\xB9a0\xFCV[F0_\x80\x1B_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\xD8Wa\x1A\xD7a@\x8FV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1B\x06W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94\x93\x92\x91\x90\x97P\x97P\x97P\x97P\x97P\x97P\x97PP\x90\x91\x92\x93\x94\x95\x96V[``\x80_a\x1BRa(\x85V[\x90P\x80`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x1B\xB5W\x83`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1B\xAC\x91\x90aG\x82V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x82`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1ClW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x1C#W[PPPPP\x90P_\x81Q\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1C\x93Wa\x1C\x92a@\x8FV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1C\xC6W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1C\xB1W\x90P[P\x90P_[\x82\x81\x10\x15a\x1D\xABWs\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3\xB2\xA8t\x85\x83\x81Q\x81\x10a\x1D\x16Wa\x1D\x15aK2V[[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1D:\x91\x90aI\xE9V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1DTW=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D|\x91\x90aQ\x08V[``\x01Q\x82\x82\x81Q\x81\x10a\x1D\x93Wa\x1D\x92aK2V[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa\x1C\xCBV[P\x80\x85`\x07\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x1E\xD9W\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x1E$Wa\x1E#a>-V[[`\x01\x81\x11\x15a\x1E6Wa\x1E5a>-V[[\x81R` \x01`\x01\x82\x01\x80Ta\x1EJ\x90aL\xB9V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1Ev\x90aL\xB9V[\x80\x15a\x1E\xC1W\x80`\x1F\x10a\x1E\x98Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1E\xC1V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1E\xA4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x1D\xE0V[PPPP\x90P\x96P\x96PPPPPP\x91P\x91V[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[_\x80a\x1F0a(\x85V[\x90P\x80`\x0C\x01T\x91PP\x90V[`\x03_a\x1FHa(\xD0V[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x1F\x90WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x1F\xC7W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_a \x15a(\x85V[\x90P`\xF8`\x06\x80\x81\x11\x15a ,Wa +a>-V[[\x90\x1B\x81`\x0E\x01\x81\x90UPP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa \x81\x91\x90aI\x91V[`@Q\x80\x91\x03\x90\xA1PPV[``\x80_a \x99a(\x85V[\x90P\x80`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a \xFCW\x83`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a \xF3\x91\x90aG\x82V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x82`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a!\xB3W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a!jW[PPPPP\x90P_\x81Q\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!\xDAWa!\xD9a@\x8FV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\rW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a!\xF8W\x90P[P\x90P_[\x82\x81\x10\x15a\"\xF2Ws\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3\xB2\xA8t\x85\x83\x81Q\x81\x10a\"]Wa\"\\aK2V[[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\"\x81\x91\x90aI\xE9V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\x9BW=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\xC3\x91\x90aQ\x08V[``\x01Q\x82\x82\x81Q\x81\x10a\"\xDAWa\"\xD9aK2V[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa\"\x12V[P\x80\x85`\x0B\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80\x80Ta#\x13\x90aL\xB9V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta#?\x90aL\xB9V[\x80\x15a#\x8AW\x80`\x1F\x10a#aWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a#\x8AV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a#mW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x96P\x96PPPPPP\x91P\x91V[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#\xFCW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$ \x91\x90aI\xBEV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a$\x8FW3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a$\x86\x91\x90aI\xE9V[`@Q\x80\x91\x03\x90\xFD[_a$\x98a(\x85V[\x90P\x80`\x04\x01_\x81T\x80\x92\x91\x90a$\xAE\x90aJ/V[\x91\x90PUP_\x81`\x04\x01T\x90P\x81`\x05\x01_\x81T\x80\x92\x91\x90a$\xCF\x90aJ/V[\x91\x90PUP_\x82`\x05\x01T\x90P\x80\x83`\x06\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x81\x83`\x06\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82`\x0E\x01_\x81T\x80\x92\x91\x90a% \x90aJ/V[\x91\x90PUP_\x83`\x0E\x01T\x90P\x84\x84`\r\x01_\x85\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a%bWa%aa>-V[[\x02\x17\x90UP\x7F\x02\x02@\x07\xD9et\xDB\xC9\xD1\x13(\xBF\xEE\x98\x93\xE7\xC7\xBBN\xF4\xAA\x80m\xF3;\xFD\xF4T\xEB^`\x83\x82\x87`@Qa%\x9A\x93\x92\x91\x90aJvV[`@Q\x80\x91\x03\x90\xA1PPPPPV[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\x06W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&*\x91\x90aI\xBEV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a&\x99W3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&\x90\x91\x90aI\xE9V[`@Q\x80\x91\x03\x90\xFD[_a&\xA2a(\x85V[\x90P\x80`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a'\x05W\x81`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&\xFC\x91\x90aG\x82V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P_\x82`\r\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x82`\x0E\x01_\x81T\x80\x92\x91\x90a'S\x90aJ/V[\x91\x90PUP_\x83`\x0E\x01T\x90P\x7F|\xF0\xBE&,z\xEC\x97\xBC\xB2OH\xCB%U\x07\"\xE8\xDD7\x1Dy\xC4/f\x98\xF3\xBB'\0W1\x83\x86\x83\x85`@Qa'\x95\x94\x93\x92\x91\x90aTeV[`@Q\x80\x91\x03\x90\xA1PPPPPV[_\x80a'\xAEa(\x85V[\x90P\x80`\x08\x01T\x91PP\x90V[``_`\x01a'\xC9\x84a1\x9AV[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'\xE7Wa'\xE6a@\x8FV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a(\x19W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x01\x82\x01\x90P[`\x01\x15a(zW\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a(oWa(naT\xA8V[[\x04\x94P_\x85\x03a(&W[\x81\x93PPPP\x91\x90PV[_\x7F\x0B\x8F\xDB\x1F\ncV\xDD \xA6\xCB\xC6\xF9f\x8F\xAC#\xB8_\x96W]\x10\xE33\xE6\x03\xFA\xA7\x94\xAC\0\x90P\x90V[_a(\xB5a(\xD0V[_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[a(\xFFa2\xEBV[a)\t\x82\x82a3+V[PPV[_\x80\x83\x83\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a),Wa)+a@\x8FV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a)ZW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_[\x84\x84\x90P\x81\x10\x15a*^W`@Q\x80``\x01`@R\x80`%\x81R` \x01aZP`%\x919\x80Q\x90` \x01 \x85\x85\x83\x81\x81\x10a)\x9DWa)\x9CaK2V[[\x90P` \x02\x81\x01\x90a)\xAF\x91\x90aKkV[_\x01` \x81\x01\x90a)\xC0\x91\x90aT\xD5V[\x86\x86\x84\x81\x81\x10a)\xD3Wa)\xD2aK2V[[\x90P` \x02\x81\x01\x90a)\xE5\x91\x90aKkV[\x80` \x01\x90a)\xF4\x91\x90aL V[`@Qa*\x02\x92\x91\x90aU.V[`@Q\x80\x91\x03\x90 `@Q` \x01a*\x1C\x93\x92\x91\x90aUUV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x82\x81Q\x81\x10a*EWa*DaK2V[[` \x02` \x01\x01\x81\x81RPP\x80\x80`\x01\x01\x91PPa)_V[Pa*\xD9`@Q\x80`\xA0\x01`@R\x80`r\x81R` \x01aY\xDE`r\x919\x80Q\x90` \x01 \x87\x87\x84`@Q` \x01a*\x95\x91\x90aV;V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a*\xBE\x94\x93\x92\x91\x90aVQV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a3|V[\x91PP\x94\x93PPPPV[_\x80a+3\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa3\x95V[\x90Pa+>\x81a3\xBFV[\x80\x91PP\x93\x92PPPV[_\x80s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xB4r+\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\xA8W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\xCC\x91\x90aV\xA8V[\x90P\x80\x83\x10\x15\x91PP\x91\x90PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a,\x87WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a,na4\x8FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a,\xBEW`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\x1DW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-A\x91\x90aI\xBEV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a-\xB0W3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a-\xA7\x91\x90aI\xE9V[`@Q\x80\x91\x03\x90\xFD[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a.\x1BWP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\x18\x91\x90aV\xFDV[`\x01[a.\\W\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.S\x91\x90aI\xE9V[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14a.\xC2W\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.\xB9\x91\x90aB%V[`@Q\x80\x91\x03\x90\xFD[a.\xCC\x83\x83a4\xE2V[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a/VW`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_a/\xA9`@Q\x80``\x01`@R\x80`,\x81R` \x01aY\xB2`,\x919\x80Q\x90` \x01 \x83`@Q` \x01a/\x8E\x92\x91\x90aW(V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a3|V[\x90P\x91\x90PV[_a0-`@Q\x80`\x80\x01`@R\x80`F\x81R` \x01aYl`F\x919\x80Q\x90` \x01 \x86\x86\x86\x86`@Q` \x01a/\xE9\x92\x91\x90aU.V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a0\x12\x94\x93\x92\x91\x90aVQV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a3|V[\x90P\x94\x93PPPPV[_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x90P\x90V[``_a0ia07V[\x90P\x80`\x02\x01\x80Ta0z\x90aL\xB9V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta0\xA6\x90aL\xB9V[\x80\x15a0\xF1W\x80`\x1F\x10a0\xC8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a0\xF1V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a0\xD4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[``_a1\x07a07V[\x90P\x80`\x03\x01\x80Ta1\x18\x90aL\xB9V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta1D\x90aL\xB9V[\x80\x15a1\x8FW\x80`\x1F\x10a1fWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a1\x8FV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a1rW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[_\x80_\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a1\xF6Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81a1\xECWa1\xEBaT\xA8V[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a23Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81a2)Wa2(aT\xA8V[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10a2bWf#\x86\xF2o\xC1\0\0\x83\x81a2XWa2WaT\xA8V[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10a2\x8BWc\x05\xF5\xE1\0\x83\x81a2\x81Wa2\x80aT\xA8V[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10a2\xB0Wa'\x10\x83\x81a2\xA6Wa2\xA5aT\xA8V[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10a2\xD3W`d\x83\x81a2\xC9Wa2\xC8aT\xA8V[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10a2\xE2W`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[a2\xF3a5TV[a3)W`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a33a2\xEBV[_a3<a07V[\x90P\x82\x81`\x02\x01\x90\x81a3O\x91\x90aW\xA7V[P\x81\x81`\x03\x01\x90\x81a3a\x91\x90aW\xA7V[P_\x80\x1B\x81_\x01\x81\x90UP_\x80\x1B\x81`\x01\x01\x81\x90UPPPPV[_a3\x8Ea3\x88a5rV[\x83a5\x80V[\x90P\x91\x90PV[_\x80_\x80a3\xA3\x86\x86a5\xC0V[\x92P\x92P\x92Pa3\xB3\x82\x82a6\x15V[\x82\x93PPPP\x92\x91PPV[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c =\x01\x14\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a4\x0C\x91\x90aI\xE9V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4'W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4K\x91\x90aJ\xE0V[a4\x8CW\x80`@Q\x7F*|n\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a4\x83\x91\x90aI\xE9V[`@Q\x80\x91\x03\x90\xFD[PV[_a4\xBB\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba7wV[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a4\xEB\x82a7\x80V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15a5GWa5A\x82\x82a8IV[Pa5PV[a5Oa8\xC9V[[PPV[_a5]a(\xD0V[_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[_a5{a9\x05V[\x90P\x90V[_`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x02\x82\x01R\x82`\"\x82\x01R`B\x81 \x91PP\x92\x91PPV[_\x80_`A\x84Q\x03a6\0W_\x80_` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90Pa5\xF2\x88\x82\x85\x85a9hV[\x95P\x95P\x95PPPPa6\x0EV[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15a6(Wa6'a>-V[[\x82`\x03\x81\x11\x15a6;Wa6:a>-V[[\x03\x15a7sW`\x01`\x03\x81\x11\x15a6UWa6Ta>-V[[\x82`\x03\x81\x11\x15a6hWa6ga>-V[[\x03a6\x9FW`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15a6\xB3Wa6\xB2a>-V[[\x82`\x03\x81\x11\x15a6\xC6Wa6\xC5a>-V[[\x03a7\nW\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a7\x01\x91\x90aG\x82V[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15a7\x1DWa7\x1Ca>-V[[\x82`\x03\x81\x11\x15a70Wa7/a>-V[[\x03a7rW\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a7i\x91\x90aB%V[`@Q\x80\x91\x03\x90\xFD[[PPV[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a7\xDBW\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a7\xD2\x91\x90aI\xE9V[`@Q\x80\x91\x03\x90\xFD[\x80a8\x07\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba7wV[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qa8r\x91\x90aX\xA6V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a8\xAAW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a8\xAFV[``\x91P[P\x91P\x91Pa8\xBF\x85\x83\x83a:OV[\x92PPP\x92\x91PPV[_4\x11\x15a9\x03W`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa9/a:\xDCV[a97a;RV[F0`@Q` \x01a9M\x95\x94\x93\x92\x91\x90aX\xBCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x80_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15a9\xA4W_`\x03\x85\x92P\x92P\x92Pa:EV[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@Qa9\xC7\x94\x93\x92\x91\x90aY(V[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a9\xE7W=_\x80>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a:8W_`\x01_\x80\x1B\x93P\x93P\x93PPa:EV[\x80_\x80_\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[``\x82a:dWa:_\x82a;\xC9V[a:\xD4V[_\x82Q\x14\x80\x15a:\x8AWP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a:\xCCW\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a:\xC3\x91\x90aI\xE9V[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa:\xD5V[[\x93\x92PPPV[_\x80a:\xE6a07V[\x90P_a:\xF1a0^V[\x90P_\x81Q\x11\x15a;\rW\x80\x80Q\x90` \x01 \x92PPPa;OV[_\x82_\x01T\x90P_\x80\x1B\x81\x14a;(W\x80\x93PPPPa;OV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x80a;\\a07V[\x90P_a;ga0\xFCV[\x90P_\x81Q\x11\x15a;\x83W\x80\x80Q\x90` \x01 \x92PPPa;\xC6V[_\x82`\x01\x01T\x90P_\x80\x1B\x81\x14a;\x9FW\x80\x93PPPPa;\xC6V[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x81Q\x11\x15a;\xDBW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15a<DW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa<)V[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a<i\x82a<\rV[a<s\x81\x85a<\x17V[\x93Pa<\x83\x81\x85` \x86\x01a<'V[a<\x8C\x81a<OV[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra<\xAF\x81\x84a<_V[\x90P\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x81\x90P\x91\x90PV[a<\xDA\x81a<\xC8V[\x81\x14a<\xE4W_\x80\xFD[PV[_\x815\x90Pa<\xF5\x81a<\xD1V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a=\x10Wa=\x0Fa<\xC0V[[_a=\x1D\x84\x82\x85\x01a<\xE7V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a=x\x82a=OV[\x90P\x91\x90PV[a=\x88\x81a=nV[\x82RPPV[_a=\x99\x83\x83a=\x7FV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a=\xBB\x82a=&V[a=\xC5\x81\x85a=0V[\x93Pa=\xD0\x83a=@V[\x80_[\x83\x81\x10\x15a>\0W\x81Qa=\xE7\x88\x82a=\x8EV[\x97Pa=\xF2\x83a=\xA5V[\x92PP`\x01\x81\x01\x90Pa=\xD3V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra>%\x81\x84a=\xB1V[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x02\x81\x10a>kWa>ja>-V[[PV[_\x81\x90Pa>{\x82a>ZV[\x91\x90PV[_a>\x8A\x82a>nV[\x90P\x91\x90PV[a>\x9A\x81a>\x80V[\x82RPPV[_` \x82\x01\x90Pa>\xB3_\x83\x01\x84a>\x91V[\x92\x91PPV[`\x02\x81\x10a>\xC5W_\x80\xFD[PV[_\x815\x90Pa>\xD6\x81a>\xB9V[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a>\xF2Wa>\xF1a<\xC0V[[_a>\xFF\x85\x82\x86\x01a<\xE7V[\x92PP` a?\x10\x85\x82\x86\x01a>\xC8V[\x91PP\x92P\x92\x90PV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12a?;Wa?:a?\x1AV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a?XWa?Wa?\x1EV[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a?tWa?sa?\"V[[\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12a?\x90Wa?\x8Fa?\x1AV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a?\xADWa?\xACa?\x1EV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a?\xC9Wa?\xC8a?\"V[[\x92P\x92\x90PV[_\x80_\x80_``\x86\x88\x03\x12\x15a?\xE9Wa?\xE8a<\xC0V[[_a?\xF6\x88\x82\x89\x01a<\xE7V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a@\x17Wa@\x16a<\xC4V[[a@#\x88\x82\x89\x01a?&V[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a@FWa@Ea<\xC4V[[a@R\x88\x82\x89\x01a?{V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[a@j\x81a=nV[\x81\x14a@tW_\x80\xFD[PV[_\x815\x90Pa@\x85\x81a@aV[\x92\x91PPV[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a@\xC5\x82a<OV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a@\xE4Wa@\xE3a@\x8FV[[\x80`@RPPPV[_a@\xF6a<\xB7V[\x90PaA\x02\x82\x82a@\xBCV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aA!WaA a@\x8FV[[aA*\x82a<OV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_aAWaAR\x84aA\x07V[a@\xEDV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15aAsWaAra@\x8BV[[aA~\x84\x82\x85aA7V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aA\x9AWaA\x99a?\x1AV[[\x815aA\xAA\x84\x82` \x86\x01aAEV[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15aA\xC9WaA\xC8a<\xC0V[[_aA\xD6\x85\x82\x86\x01a@wV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aA\xF7WaA\xF6a<\xC4V[[aB\x03\x85\x82\x86\x01aA\x86V[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[aB\x1F\x81aB\rV[\x82RPPV[_` \x82\x01\x90PaB8_\x83\x01\x84aB\x16V[\x92\x91PPV[_\x80_`@\x84\x86\x03\x12\x15aBUWaBTa<\xC0V[[_aBb\x86\x82\x87\x01a<\xE7V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aB\x83WaB\x82a<\xC4V[[aB\x8F\x86\x82\x87\x01a?{V[\x92P\x92PP\x92P\x92P\x92V[_\x80_\x80_``\x86\x88\x03\x12\x15aB\xB4WaB\xB3a<\xC0V[[_aB\xC1\x88\x82\x89\x01a<\xE7V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aB\xE2WaB\xE1a<\xC4V[[aB\xEE\x88\x82\x89\x01a?{V[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aC\x11WaC\x10a<\xC4V[[aC\x1D\x88\x82\x89\x01a?{V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[_\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[aC`\x81aC,V[\x82RPPV[aCo\x81a<\xC8V[\x82RPPV[aC~\x81a=nV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[aC\xB6\x81a<\xC8V[\x82RPPV[_aC\xC7\x83\x83aC\xADV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aC\xE9\x82aC\x84V[aC\xF3\x81\x85aC\x8EV[\x93PaC\xFE\x83aC\x9EV[\x80_[\x83\x81\x10\x15aD.W\x81QaD\x15\x88\x82aC\xBCV[\x97PaD \x83aC\xD3V[\x92PP`\x01\x81\x01\x90PaD\x01V[P\x85\x93PPPP\x92\x91PPV[_`\xE0\x82\x01\x90PaDN_\x83\x01\x8AaCWV[\x81\x81\x03` \x83\x01RaD`\x81\x89a<_V[\x90P\x81\x81\x03`@\x83\x01RaDt\x81\x88a<_V[\x90PaD\x83``\x83\x01\x87aCfV[aD\x90`\x80\x83\x01\x86aCuV[aD\x9D`\xA0\x83\x01\x85aB\x16V[\x81\x81\x03`\xC0\x83\x01RaD\xAF\x81\x84aC\xDFV[\x90P\x98\x97PPPPPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aE\0\x82a<\rV[aE\n\x81\x85aD\xE6V[\x93PaE\x1A\x81\x85` \x86\x01a<'V[aE#\x81a<OV[\x84\x01\x91PP\x92\x91PPV[_aE9\x83\x83aD\xF6V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aEW\x82aD\xBDV[aEa\x81\x85aD\xC7V[\x93P\x83` \x82\x02\x85\x01aEs\x85aD\xD7V[\x80_[\x85\x81\x10\x15aE\xAEW\x84\x84\x03\x89R\x81QaE\x8F\x85\x82aE.V[\x94PaE\x9A\x83aEAV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaEvV[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[`\x02\x81\x10aE\xFAWaE\xF9a>-V[[PV[_\x81\x90PaF\n\x82aE\xE9V[\x91\x90PV[_aF\x19\x82aE\xFDV[\x90P\x91\x90PV[aF)\x81aF\x0FV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aFS\x82aF/V[aF]\x81\x85aF9V[\x93PaFm\x81\x85` \x86\x01a<'V[aFv\x81a<OV[\x84\x01\x91PP\x92\x91PPV[_`@\x83\x01_\x83\x01QaF\x96_\x86\x01\x82aF V[P` \x83\x01Q\x84\x82\x03` \x86\x01RaF\xAE\x82\x82aFIV[\x91PP\x80\x91PP\x92\x91PPV[_aF\xC6\x83\x83aF\x81V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aF\xE4\x82aE\xC0V[aF\xEE\x81\x85aE\xCAV[\x93P\x83` \x82\x02\x85\x01aG\0\x85aE\xDAV[\x80_[\x85\x81\x10\x15aG;W\x84\x84\x03\x89R\x81QaG\x1C\x85\x82aF\xBBV[\x94PaG'\x83aF\xCEV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaG\x03V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01RaGe\x81\x85aEMV[\x90P\x81\x81\x03` \x83\x01RaGy\x81\x84aF\xDAV[\x90P\x93\x92PPPV[_` \x82\x01\x90PaG\x95_\x83\x01\x84aCfV[\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aG\xB5\x82aF/V[aG\xBF\x81\x85aG\x9BV[\x93PaG\xCF\x81\x85` \x86\x01a<'V[aG\xD8\x81a<OV[\x84\x01\x91PP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01RaG\xFB\x81\x85aEMV[\x90P\x81\x81\x03` \x83\x01RaH\x0F\x81\x84aG\xABV[\x90P\x93\x92PPPV[_` \x82\x84\x03\x12\x15aH-WaH,a<\xC0V[[_aH:\x84\x82\x85\x01a>\xC8V[\x91PP\x92\x91PPV[_\x81\x90P\x92\x91PPV[_aHW\x82a<\rV[aHa\x81\x85aHCV[\x93PaHq\x81\x85` \x86\x01a<'V[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aH\xB1`\x02\x83aHCV[\x91PaH\xBC\x82aH}V[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aH\xFB`\x01\x83aHCV[\x91PaI\x06\x82aH\xC7V[`\x01\x82\x01\x90P\x91\x90PV[_aI\x1C\x82\x87aHMV[\x91PaI'\x82aH\xA5V[\x91PaI3\x82\x86aHMV[\x91PaI>\x82aH\xEFV[\x91PaIJ\x82\x85aHMV[\x91PaIU\x82aH\xEFV[\x91PaIa\x82\x84aHMV[\x91P\x81\x90P\x95\x94PPPPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[aI\x8B\x81aIoV[\x82RPPV[_` \x82\x01\x90PaI\xA4_\x83\x01\x84aI\x82V[\x92\x91PPV[_\x81Q\x90PaI\xB8\x81a@aV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aI\xD3WaI\xD2a<\xC0V[[_aI\xE0\x84\x82\x85\x01aI\xAAV[\x91PP\x92\x91PPV[_` \x82\x01\x90PaI\xFC_\x83\x01\x84aCuV[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_aJ9\x82a<\xC8V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03aJkWaJjaJ\x02V[[`\x01\x82\x01\x90P\x91\x90PV[_``\x82\x01\x90PaJ\x89_\x83\x01\x86aCfV[aJ\x96` \x83\x01\x85aCfV[aJ\xA3`@\x83\x01\x84a>\x91V[\x94\x93PPPPV[_\x81\x15\x15\x90P\x91\x90PV[aJ\xBF\x81aJ\xABV[\x81\x14aJ\xC9W_\x80\xFD[PV[_\x81Q\x90PaJ\xDA\x81aJ\xB6V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aJ\xF5WaJ\xF4a<\xC0V[[_aK\x02\x84\x82\x85\x01aJ\xCCV[\x91PP\x92\x91PPV[_`@\x82\x01\x90PaK\x1E_\x83\x01\x85aCfV[aK+` \x83\x01\x84aCuV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x825`\x01`@\x03\x836\x03\x03\x81\x12aK\x86WaK\x85aK_V[[\x80\x83\x01\x91PP\x92\x91PPV[`\x02\x81\x10aK\x9EW_\x80\xFD[PV[_\x815aK\xAD\x81aK\x92V[\x80\x91PP\x91\x90PV[_\x81_\x1B\x90P\x91\x90PV[_`\xFFaK\xCD\x84aK\xB6V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_aK\xED\x82aE\xFDV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[aL\x06\x82aK\xE3V[aL\x19aL\x12\x82aK\xF4V[\x83TaK\xC1V[\x82UPPPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aL<WaL;aK_V[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aL^WaL]aKcV[[` \x83\x01\x92P`\x01\x82\x026\x03\x83\x13\x15aLzWaLyaKgV[[P\x92P\x92\x90PV[_\x82\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80aL\xD0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aL\xE3WaL\xE2aL\x8CV[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02aME\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82aM\nV[aMO\x86\x83aM\nV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_aM\x8AaM\x85aM\x80\x84a<\xC8V[aMgV[a<\xC8V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[aM\xA3\x83aMpV[aM\xB7aM\xAF\x82aM\x91V[\x84\x84TaM\x16V[\x82UPPPPV[_\x90V[aM\xCBaM\xBFV[aM\xD6\x81\x84\x84aM\x9AV[PPPV[[\x81\x81\x10\x15aM\xF9WaM\xEE_\x82aM\xC3V[`\x01\x81\x01\x90PaM\xDCV[PPV[`\x1F\x82\x11\x15aN>WaN\x0F\x81aL\xE9V[aN\x18\x84aL\xFBV[\x81\x01` \x85\x10\x15aN'W\x81\x90P[aN;aN3\x85aL\xFBV[\x83\x01\x82aM\xDBV[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_aN^_\x19\x84`\x08\x02aNCV[\x19\x80\x83\x16\x91PP\x92\x91PPV[_aNv\x83\x83aNOV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[aN\x90\x83\x83aL\x82V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aN\xA9WaN\xA8a@\x8FV[[aN\xB3\x82TaL\xB9V[aN\xBE\x82\x82\x85aM\xFDV[_`\x1F\x83\x11`\x01\x81\x14aN\xEBW_\x84\x15aN\xD9W\x82\x87\x015\x90P[aN\xE3\x85\x82aNkV[\x86UPaOJV[`\x1F\x19\x84\x16aN\xF9\x86aL\xE9V[_[\x82\x81\x10\x15aO W\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PaN\xFBV[\x86\x83\x10\x15aO=W\x84\x89\x015aO9`\x1F\x89\x16\x82aNOV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[aO^\x83\x83\x83aN\x86V[PPPV[_\x81\x01_\x83\x01\x80aOs\x81aK\xA1V[\x90PaO\x7F\x81\x84aK\xFDV[PPP`\x01\x81\x01` \x83\x01aO\x94\x81\x85aL V[aO\x9F\x81\x83\x86aOSV[PPPPPPV[aO\xB1\x82\x82aOcV[PPV[_\x80\xFD[_\x80\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aO\xD7WaO\xD6a@\x8FV[[aO\xE0\x82a<OV[\x90P` \x81\x01\x90P\x91\x90PV[_aO\xFFaO\xFA\x84aO\xBDV[a@\xEDV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15aP\x1BWaP\x1Aa@\x8BV[[aP&\x84\x82\x85a<'V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aPBWaPAa?\x1AV[[\x81QaPR\x84\x82` \x86\x01aO\xEDV[\x91PP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15aPpWaPoaO\xB5V[[aPz`\x80a@\xEDV[\x90P_aP\x89\x84\x82\x85\x01aI\xAAV[_\x83\x01RP` aP\x9C\x84\x82\x85\x01aI\xAAV[` \x83\x01RP`@\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aP\xC0WaP\xBFaO\xB9V[[aP\xCC\x84\x82\x85\x01aP.V[`@\x83\x01RP``\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aP\xF0WaP\xEFaO\xB9V[[aP\xFC\x84\x82\x85\x01aP.V[``\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15aQ\x1DWaQ\x1Ca<\xC0V[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aQ:WaQ9a<\xC4V[[aQF\x84\x82\x85\x01aP[V[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[_\x815\x90PaQf\x81aK\x92V[\x92\x91PPV[_aQz` \x84\x01\x84aQXV[\x90P\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aQ\xAAWaQ\xA9aQ\x8AV[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aQ\xD2WaQ\xD1aQ\x82V[[`\x01\x82\x026\x03\x83\x13\x15aQ\xE8WaQ\xE7aQ\x86V[[P\x92P\x92\x90PV[_aQ\xFB\x83\x85aF9V[\x93PaR\x08\x83\x85\x84aA7V[aR\x11\x83a<OV[\x84\x01\x90P\x93\x92PPPV[_`@\x83\x01aR-_\x84\x01\x84aQlV[aR9_\x86\x01\x82aF V[PaRG` \x84\x01\x84aQ\x8EV[\x85\x83\x03` \x87\x01RaRZ\x83\x82\x84aQ\xF0V[\x92PPP\x80\x91PP\x92\x91PPV[_aRs\x83\x83aR\x1CV[\x90P\x92\x91PPV[_\x825`\x01`@\x03\x836\x03\x03\x81\x12aR\x96WaR\x95aQ\x8AV[[\x82\x81\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aR\xB9\x83\x85aE\xCAV[\x93P\x83` \x84\x02\x85\x01aR\xCB\x84aQOV[\x80_[\x87\x81\x10\x15aS\x0EW\x84\x84\x03\x89RaR\xE5\x82\x84aR{V[aR\xEF\x85\x82aRhV[\x94PaR\xFA\x83aR\xA2V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaR\xCEV[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_``\x82\x01\x90PaS3_\x83\x01\x87aCfV[\x81\x81\x03` \x83\x01RaSE\x81\x86aEMV[\x90P\x81\x81\x03`@\x83\x01RaSZ\x81\x84\x86aR\xAEV[\x90P\x95\x94PPPPPV[_`@\x82\x01\x90PaSx_\x83\x01\x85aCfV[aS\x85` \x83\x01\x84aCfV[\x93\x92PPPV[_aS\x97\x83\x85aG\x9BV[\x93PaS\xA4\x83\x85\x84aA7V[aS\xAD\x83a<OV[\x84\x01\x90P\x93\x92PPPV[_``\x82\x01\x90PaS\xCB_\x83\x01\x87aCfV[\x81\x81\x03` \x83\x01RaS\xDD\x81\x86aEMV[\x90P\x81\x81\x03`@\x83\x01RaS\xF2\x81\x84\x86aS\x8CV[\x90P\x95\x94PPPPPV[\x7FEIP712: Uninitialized\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aT1`\x15\x83a<\x17V[\x91PaT<\x82aS\xFDV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaT^\x81aT%V[\x90P\x91\x90PV[_`\x80\x82\x01\x90PaTx_\x83\x01\x87aCfV[aT\x85` \x83\x01\x86aCfV[aT\x92`@\x83\x01\x85aCfV[aT\x9F``\x83\x01\x84a>\x91V[\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aT\xEAWaT\xE9a<\xC0V[[_aT\xF7\x84\x82\x85\x01aQXV[\x91PP\x92\x91PPV[_\x81\x90P\x92\x91PPV[_aU\x15\x83\x85aU\0V[\x93PaU\"\x83\x85\x84aA7V[\x82\x84\x01\x90P\x93\x92PPPV[_aU:\x82\x84\x86aU\nV[\x91P\x81\x90P\x93\x92PPPV[aUO\x81aF\x0FV[\x82RPPV[_``\x82\x01\x90PaUh_\x83\x01\x86aB\x16V[aUu` \x83\x01\x85aUFV[aU\x82`@\x83\x01\x84aB\x16V[\x94\x93PPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[aU\xB6\x81aB\rV[\x82RPPV[_aU\xC7\x83\x83aU\xADV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aU\xE9\x82aU\x8AV[aU\xF3\x81\x85aU\x94V[\x93PaU\xFE\x83aU\x9EV[\x80_[\x83\x81\x10\x15aV.W\x81QaV\x15\x88\x82aU\xBCV[\x97PaV \x83aU\xD3V[\x92PP`\x01\x81\x01\x90PaV\x01V[P\x85\x93PPPP\x92\x91PPV[_aVF\x82\x84aU\xDFV[\x91P\x81\x90P\x92\x91PPV[_`\x80\x82\x01\x90PaVd_\x83\x01\x87aB\x16V[aVq` \x83\x01\x86aCfV[aV~`@\x83\x01\x85aCfV[aV\x8B``\x83\x01\x84aB\x16V[\x95\x94PPPPPV[_\x81Q\x90PaV\xA2\x81a<\xD1V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aV\xBDWaV\xBCa<\xC0V[[_aV\xCA\x84\x82\x85\x01aV\x94V[\x91PP\x92\x91PPV[aV\xDC\x81aB\rV[\x81\x14aV\xE6W_\x80\xFD[PV[_\x81Q\x90PaV\xF7\x81aV\xD3V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aW\x12WaW\x11a<\xC0V[[_aW\x1F\x84\x82\x85\x01aV\xE9V[\x91PP\x92\x91PPV[_`@\x82\x01\x90PaW;_\x83\x01\x85aB\x16V[aWH` \x83\x01\x84aCfV[\x93\x92PPPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[`\x1F\x82\x11\x15aW\xA2WaWs\x81aWOV[aW|\x84aL\xFBV[\x81\x01` \x85\x10\x15aW\x8BW\x81\x90P[aW\x9FaW\x97\x85aL\xFBV[\x83\x01\x82aM\xDBV[PP[PPPV[aW\xB0\x82a<\rV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aW\xC9WaW\xC8a@\x8FV[[aW\xD3\x82TaL\xB9V[aW\xDE\x82\x82\x85aWaV[_` \x90P`\x1F\x83\x11`\x01\x81\x14aX\x0FW_\x84\x15aW\xFDW\x82\x87\x01Q\x90P[aX\x07\x85\x82aNkV[\x86UPaXnV[`\x1F\x19\x84\x16aX\x1D\x86aWOV[_[\x82\x81\x10\x15aXDW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PaX\x1FV[\x86\x83\x10\x15aXaW\x84\x89\x01QaX]`\x1F\x89\x16\x82aNOV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_aX\x80\x82aF/V[aX\x8A\x81\x85aU\0V[\x93PaX\x9A\x81\x85` \x86\x01a<'V[\x80\x84\x01\x91PP\x92\x91PPV[_aX\xB1\x82\x84aXvV[\x91P\x81\x90P\x92\x91PPV[_`\xA0\x82\x01\x90PaX\xCF_\x83\x01\x88aB\x16V[aX\xDC` \x83\x01\x87aB\x16V[aX\xE9`@\x83\x01\x86aB\x16V[aX\xF6``\x83\x01\x85aCfV[aY\x03`\x80\x83\x01\x84aCuV[\x96\x95PPPPPPV[_`\xFF\x82\x16\x90P\x91\x90PV[aY\"\x81aY\rV[\x82RPPV[_`\x80\x82\x01\x90PaY;_\x83\x01\x87aB\x16V[aYH` \x83\x01\x86aY\x19V[aYU`@\x83\x01\x85aB\x16V[aYb``\x83\x01\x84aB\x16V[\x95\x94PPPPPV\xFECrsgenVerification(uint256 crsId,uint256 maxBitLength,bytes crsDigest)PrepKeygenVerification(uint256 prepKeygenId)KeygenVerification(uint256 prepKeygenId,uint256 keyId,KeyDigest[] keyDigests)KeyDigest(uint8 keyType,bytes digest)KeyDigest(uint8 keyType,bytes digest)",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405260043610610129575f3560e01c806362978787116100aa578063baff211e1161006e578063baff211e146103b4578063c4115874146103de578063c55b8724146103f4578063caa367db14610431578063d2c08a9114610459578063d52f10eb1461048157610129565b806362978787146102df5780637514a2ac1461030757806384b0196e1461031d578063936608ae1461034d578063ad3cb1cc1461038a57610129565b806345af261b116100f157806345af261b1461020d5780634610ffe8146102495780634f1ef2861461027157806352d1902d1461028d578063589adb0e146102b757610129565b80630d8e6e2c1461012d57806316c713d91461015757806319f4f6321461019357806339f73810146101cf5780633c02f834146101e5575b5f80fd5b348015610138575f80fd5b506101416104ab565b60405161014e9190613c97565b60405180910390f35b348015610162575f80fd5b5061017d60048036038101906101789190613cfb565b610526565b60405161018a9190613e0d565b60405180910390f35b34801561019e575f80fd5b506101b960048036038101906101b49190613cfb565b6105f7565b6040516101c69190613ea0565b60405180910390f35b3480156101da575f80fd5b506101e36106a4565b005b3480156101f0575f80fd5b5061020b60048036038101906102069190613edc565b610913565b005b348015610218575f80fd5b50610233600480360381019061022e9190613cfb565b610ac2565b6040516102409190613ea0565b60405180910390f35b348015610254575f80fd5b5061026f600480360381019061026a9190613fd0565b610b57565b005b61028b600480360381019061028691906141b3565b61109c565b005b348015610298575f80fd5b506102a16110bb565b6040516102ae9190614225565b60405180910390f35b3480156102c2575f80fd5b506102dd60048036038101906102d8919061423e565b6110ec565b005b3480156102ea575f80fd5b506103056004803603810190610300919061429b565b61143e565b005b348015610312575f80fd5b5061031b61191f565b005b348015610328575f80fd5b50610331611a3d565b604051610344979695949392919061443b565b60405180910390f35b348015610358575f80fd5b50610373600480360381019061036e9190613cfb565b611b46565b60405161038192919061474d565b60405180910390f35b348015610395575f80fd5b5061039e611eed565b6040516103ab9190613c97565b60405180910390f35b3480156103bf575f80fd5b506103c8611f26565b6040516103d59190614782565b60405180910390f35b3480156103e9575f80fd5b506103f2611f3d565b005b3480156103ff575f80fd5b5061041a60048036038101906104159190613cfb565b61208d565b6040516104289291906147e3565b60405180910390f35b34801561043c575f80fd5b5061045760048036038101906104529190614818565b61239f565b005b348015610464575f80fd5b5061047f600480360381019061047a9190613cfb565b6125a9565b005b34801561048c575f80fd5b506104956127a4565b6040516104a29190614782565b60405180910390f35b60606040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506104ec5f6127bb565b6104f660016127bb565b6104ff5f6127bb565b6040516020016105129493929190614911565b604051602081830303815290604052905090565b60605f610531612885565b90505f816003015f8581526020019081526020015f20549050816002015f8581526020019081526020015f205f8281526020019081526020015f208054806020026020016040519081016040528092919081815260200182805480156105e957602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116105a0575b505050505092505050919050565b5f80610601612885565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff1661066457826040517f84de133100000000000000000000000000000000000000000000000000000000815260040161065b9190614782565b60405180910390fd5b5f816006015f8581526020019081526020015f2054905081600d015f8281526020019081526020015f205f9054906101000a900460ff1692505050919050565b60016106ae6128ac565b67ffffffffffffffff16146106ef576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60035f6106fa6128d0565b9050805f0160089054906101000a900460ff168061074257508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15610779576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055506108326040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506040518060400160405280600181526020017f31000000000000000000000000000000000000000000000000000000000000008152506128f7565b5f61083b612885565b905060f86003600681111561085357610852613e2d565b5b901b816004018190555060f86004600681111561087357610872613e2d565b5b901b816005018190555060f86005600681111561089357610892613e2d565b5b901b816009018190555060f86006808111156108b2576108b1613e2d565b5b901b81600e0181905550505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2826040516109079190614991565b60405180910390a15050565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610970573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061099491906149be565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610a0357336040517f0e56cf3d0000000000000000000000000000000000000000000000000000000081526004016109fa91906149e9565b60405180910390fd5b5f610a0c612885565b9050806009015f815480929190610a2290614a2f565b91905055505f816009015490508382600a015f8381526020019081526020015f20819055508282600d015f8381526020019081526020015f205f6101000a81548160ff02191690836001811115610a7c57610a7b613e2d565b5b02179055507f3f038f6f88cb3031b7718588403a2ec220576a868be07dde4c02b846ca352ef5818585604051610ab493929190614a76565b60405180910390a150505050565b5f80610acc612885565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff16610b2f57826040517fda32d00f000000000000000000000000000000000000000000000000000000008152600401610b269190614782565b60405180910390fd5b80600d015f8481526020019081526020015f205f9054906101000a900460ff16915050919050565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663e5275eaf336040518263ffffffff1660e01b8152600401610ba491906149e9565b602060405180830381865afa158015610bbf573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610be39190614ae0565b610c2457336040517faee86323000000000000000000000000000000000000000000000000000000008152600401610c1b91906149e9565b60405180910390fd5b5f610c2d612885565b90505f816006015f8881526020019081526020015f205490505f610c538289898961290d565b90505f610c61828787612ae4565b9050835f015f8a81526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615610d015788816040517f98fb957d000000000000000000000000000000000000000000000000000000008152600401610cf8929190614b0b565b60405180910390fd5b6001845f015f8b81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f846002015f8b81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505f81805490509050856001015f8c81526020019081526020015f205f9054906101000a900460ff16158015610e245750610e2381612b49565b5b1561108f576001866001015f8d81526020019081526020015f205f6101000a81548160ff0219169083151502179055505f5b8a8a9050811015610eda57866007015f8d81526020019081526020015f208b8b83818110610e8757610e86614b32565b5b9050602002810190610e999190614b6b565b908060018154018082558091505060019003905f5260205f2090600202015f909190919091508181610ecb9190614fa7565b50508080600101915050610e56565b5083866003015f8d81526020019081526020015f20819055508a86600801819055505f8167ffffffffffffffff811115610f1757610f1661408f565b5b604051908082528060200260200182016040528015610f4a57816020015b6060815260200190600190039081610f355790505b5090505f5b8281101561104f5773a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663e3b2a874858381548110610f9a57610f99614b32565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b8152600401610fde91906149e9565b5f60405180830381865afa158015610ff8573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f820116820180604052508101906110209190615108565b6060015182828151811061103757611036614b32565b5b60200260200101819052508080600101915050610f4f565b507feb85c26dbcad46b80a68a0f24cce7c2c90f0a1faded84184138839fc9e80a25b8c828d8d6040516110859493929190615320565b60405180910390a1505b5050505050505050505050565b6110a4612bda565b6110ad82612cc0565b6110b78282612db3565b5050565b5f6110c4612ed1565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663e5275eaf336040518263ffffffff1660e01b815260040161113991906149e9565b602060405180830381865afa158015611154573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111789190614ae0565b6111b957336040517faee863230000000000000000000000000000000000000000000000000000000081526004016111b091906149e9565b60405180910390fd5b5f6111c2612885565b90505f6111ce85612f58565b90505f6111dc828686612ae4565b9050825f015f8781526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff161561127c5785816040517f33ca1fe3000000000000000000000000000000000000000000000000000000008152600401611273929190614b0b565b60405180910390fd5b6001835f015f8881526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f836002015f8881526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550836001015f8881526020019081526020015f205f9054906101000a900460ff1615801561139b575061139a8180549050612b49565b5b15611435576001846001015f8981526020019081526020015f205f6101000a81548160ff02191690831515021790555082846003015f8981526020019081526020015f20819055505f846006015f8981526020019081526020015f205490507f78b179176d1f19d7c28e80823deba2624da2ca2ec64b1701f3632a87c9aedc92888260405161142b929190615365565b60405180910390a1505b50505050505050565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663e5275eaf336040518263ffffffff1660e01b815260040161148b91906149e9565b602060405180830381865afa1580156114a6573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114ca9190614ae0565b61150b57336040517faee8632300000000000000000000000000000000000000000000000000000000815260040161150291906149e9565b60405180910390fd5b5f611514612885565b90505f81600a015f8881526020019081526020015f205490505f61153a88838989612fb0565b90505f611548828787612ae4565b9050835f015f8a81526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16156115e85788816040517ffcf5a6e90000000000000000000000000000000000000000000000000000000081526004016115df929190614b0b565b60405180910390fd5b6001845f015f8b81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f846002015f8b81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505f81805490509050856001015f8c81526020019081526020015f205f9054906101000a900460ff1615801561170b575061170a81612b49565b5b15611912576001866001015f8d81526020019081526020015f205f6101000a81548160ff021916908315150217905550898987600b015f8e81526020019081526020015f20918261175d929190614e86565b5083866003015f8d81526020019081526020015f20819055508a86600c01819055505f8167ffffffffffffffff81111561179a5761179961408f565b5b6040519080825280602002602001820160405280156117cd57816020015b60608152602001906001900390816117b85790505b5090505f5b828110156118d25773a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663e3b2a87485838154811061181d5761181c614b32565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b815260040161186191906149e9565b5f60405180830381865afa15801561187b573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f820116820180604052508101906118a39190615108565b606001518282815181106118ba576118b9614b32565b5b602002602001018190525080806001019150506117d2565b507f2258b73faed33fb2e2ea454403bef974920caf682ab3a723484fcf67553b16a28c828d8d60405161190894939291906153b8565b60405180910390a1505b5050505050505050505050565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561197c573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119a091906149be565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614611a0f57336040517f0e56cf3d000000000000000000000000000000000000000000000000000000008152600401611a0691906149e9565b60405180910390fd5b7f11db42c1878f2e2819241f5250984563f06cf22818e7adb86a66921d15d59d3f60405160405180910390a1565b5f6060805f805f60605f611a4f613037565b90505f801b815f0154148015611a6a57505f801b8160010154145b611aa9576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611aa090615447565b60405180910390fd5b611ab161305e565b611ab96130fc565b46305f801b5f67ffffffffffffffff811115611ad857611ad761408f565b5b604051908082528060200260200182016040528015611b065781602001602082028036833780820191505090505b507f0f0000000000000000000000000000000000000000000000000000000000000095949392919097509750975097509750975097505090919293949596565b6060805f611b52612885565b9050806001015f8581526020019081526020015f205f9054906101000a900460ff16611bb557836040517f84de1331000000000000000000000000000000000000000000000000000000008152600401611bac9190614782565b60405180910390fd5b5f816003015f8681526020019081526020015f205490505f826002015f8781526020019081526020015f205f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020018280548015611c6c57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311611c23575b505050505090505f815190505f8167ffffffffffffffff811115611c9357611c9261408f565b5b604051908082528060200260200182016040528015611cc657816020015b6060815260200190600190039081611cb15790505b5090505f5b82811015611dab5773a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663e3b2a874858381518110611d1657611d15614b32565b5b60200260200101516040518263ffffffff1660e01b8152600401611d3a91906149e9565b5f60405180830381865afa158015611d54573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f82011682018060405250810190611d7c9190615108565b60600151828281518110611d9357611d92614b32565b5b60200260200101819052508080600101915050611ccb565b5080856007015f8a81526020019081526020015f2080805480602002602001604051908101604052809291908181526020015f905b82821015611ed9578382905f5260205f2090600202016040518060400160405290815f82015f9054906101000a900460ff166001811115611e2457611e23613e2d565b5b6001811115611e3657611e35613e2d565b5b8152602001600182018054611e4a90614cb9565b80601f0160208091040260200160405190810160405280929190818152602001828054611e7690614cb9565b8015611ec15780601f10611e9857610100808354040283529160200191611ec1565b820191905f5260205f20905b815481529060010190602001808311611ea457829003601f168201915b50505050508152505081526020019060010190611de0565b505050509050965096505050505050915091565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b5f80611f30612885565b905080600c015491505090565b60035f611f486128d0565b9050805f0160089054906101000a900460ff1680611f9057508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15611fc7576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055505f612015612885565b905060f860068081111561202c5761202b613e2d565b5b901b81600e0181905550505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2826040516120819190614991565b60405180910390a15050565b6060805f612099612885565b9050806001015f8581526020019081526020015f205f9054906101000a900460ff166120fc57836040517fda32d00f0000000000000000000000000000000000000000000000000000000081526004016120f39190614782565b60405180910390fd5b5f816003015f8681526020019081526020015f205490505f826002015f8781526020019081526020015f205f8381526020019081526020015f208054806020026020016040519081016040528092919081815260200182805480156121b357602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001906001019080831161216a575b505050505090505f815190505f8167ffffffffffffffff8111156121da576121d961408f565b5b60405190808252806020026020018201604052801561220d57816020015b60608152602001906001900390816121f85790505b5090505f5b828110156122f25773a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663e3b2a87485838151811061225d5761225c614b32565b5b60200260200101516040518263ffffffff1660e01b815260040161228191906149e9565b5f60405180830381865afa15801561229b573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f820116820180604052508101906122c39190615108565b606001518282815181106122da576122d9614b32565b5b60200260200101819052508080600101915050612212565b508085600b015f8a81526020019081526020015f2080805461231390614cb9565b80601f016020809104026020016040519081016040528092919081815260200182805461233f90614cb9565b801561238a5780601f106123615761010080835404028352916020019161238a565b820191905f5260205f20905b81548152906001019060200180831161236d57829003601f168201915b50505050509050965096505050505050915091565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156123fc573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061242091906149be565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461248f57336040517f0e56cf3d00000000000000000000000000000000000000000000000000000000815260040161248691906149e9565b60405180910390fd5b5f612498612885565b9050806004015f8154809291906124ae90614a2f565b91905055505f81600401549050816005015f8154809291906124cf90614a2f565b91905055505f8260050154905080836006015f8481526020019081526020015f208190555081836006015f8381526020019081526020015f208190555082600e015f81548092919061252090614a2f565b91905055505f83600e015490508484600d015f8581526020019081526020015f205f6101000a81548160ff0219169083600181111561256257612561613e2d565b5b02179055507f02024007d96574dbc9d11328bfee9893e7c7bb4ef4aa806df33bfdf454eb5e6083828760405161259a93929190614a76565b60405180910390a15050505050565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612606573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061262a91906149be565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461269957336040517f0e56cf3d00000000000000000000000000000000000000000000000000000000815260040161269091906149e9565b60405180910390fd5b5f6126a2612885565b9050806001015f8381526020019081526020015f205f9054906101000a900460ff1661270557816040517f84de13310000000000000000000000000000000000000000000000000000000081526004016126fc9190614782565b60405180910390fd5b5f816006015f8481526020019081526020015f205490505f82600d015f8381526020019081526020015f205f9054906101000a900460ff16905082600e015f81548092919061275390614a2f565b91905055505f83600e015490507f7cf0be262c7aec97bcb24f48cb25550722e8dd371d79c42f6698f3bb27005731838683856040516127959493929190615465565b60405180910390a15050505050565b5f806127ae612885565b9050806008015491505090565b60605f60016127c98461319a565b0190505f8167ffffffffffffffff8111156127e7576127e661408f565b5b6040519080825280601f01601f1916602001820160405280156128195781602001600182028036833780820191505090505b5090505f82602001820190505b60011561287a578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a858161286f5761286e6154a8565b5b0494505f8503612826575b819350505050919050565b5f7f0b8fdb1f0a6356dd20a6cbc6f9668fac23b85f96575d10e333e603faa794ac00905090565b5f6128b56128d0565b5f015f9054906101000a900467ffffffffffffffff16905090565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b6128ff6132eb565b612909828261332b565b5050565b5f808383905067ffffffffffffffff81111561292c5761292b61408f565b5b60405190808252806020026020018201604052801561295a5781602001602082028036833780820191505090505b5090505f5b84849050811015612a5e57604051806060016040528060258152602001615a50602591398051906020012085858381811061299d5761299c614b32565b5b90506020028101906129af9190614b6b565b5f0160208101906129c091906154d5565b8686848181106129d3576129d2614b32565b5b90506020028101906129e59190614b6b565b80602001906129f49190614c20565b604051612a0292919061552e565b6040518091039020604051602001612a1c93929190615555565b60405160208183030381529060405280519060200120828281518110612a4557612a44614b32565b5b602002602001018181525050808060010191505061295f565b50612ad96040518060a00160405280607281526020016159de6072913980519060200120878784604051602001612a95919061563b565b60405160208183030381529060405280519060200120604051602001612abe9493929190615651565b6040516020818303038152906040528051906020012061337c565b915050949350505050565b5f80612b338585858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050613395565b9050612b3e816133bf565b809150509392505050565b5f8073a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663b4722bc46040518163ffffffff1660e01b8152600401602060405180830381865afa158015612ba8573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612bcc91906156a8565b905080831015915050919050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff161480612c8757507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16612c6e61348f565b73ffffffffffffffffffffffffffffffffffffffff1614155b15612cbe576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612d1d573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612d4191906149be565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614612db057336040517f0e56cf3d000000000000000000000000000000000000000000000000000000008152600401612da791906149e9565b60405180910390fd5b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa925050508015612e1b57506040513d601f19601f82011682018060405250810190612e1891906156fd565b60015b612e5c57816040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401612e5391906149e9565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b8114612ec257806040517faa1d49a4000000000000000000000000000000000000000000000000000000008152600401612eb99190614225565b60405180910390fd5b612ecc83836134e2565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff1614612f56576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f612fa96040518060600160405280602c81526020016159b2602c91398051906020012083604051602001612f8e929190615728565b6040516020818303038152906040528051906020012061337c565b9050919050565b5f61302d60405180608001604052806046815260200161596c604691398051906020012086868686604051602001612fe992919061552e565b604051602081830303815290604052805190602001206040516020016130129493929190615651565b6040516020818303038152906040528051906020012061337c565b9050949350505050565b5f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100905090565b60605f613069613037565b905080600201805461307a90614cb9565b80601f01602080910402602001604051908101604052809291908181526020018280546130a690614cb9565b80156130f15780601f106130c8576101008083540402835291602001916130f1565b820191905f5260205f20905b8154815290600101906020018083116130d457829003601f168201915b505050505091505090565b60605f613107613037565b905080600301805461311890614cb9565b80601f016020809104026020016040519081016040528092919081815260200182805461314490614cb9565b801561318f5780601f106131665761010080835404028352916020019161318f565b820191905f5260205f20905b81548152906001019060200180831161317257829003601f168201915b505050505091505090565b5f805f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f01000000000000000083106131f6577a184f03e93ff9f4daa797ed6e38ed64bf6a1f01000000000000000083816131ec576131eb6154a8565b5b0492506040810190505b6d04ee2d6d415b85acef81000000008310613233576d04ee2d6d415b85acef81000000008381613229576132286154a8565b5b0492506020810190505b662386f26fc10000831061326257662386f26fc100008381613258576132576154a8565b5b0492506010810190505b6305f5e100831061328b576305f5e1008381613281576132806154a8565b5b0492506008810190505b61271083106132b05761271083816132a6576132a56154a8565b5b0492506004810190505b606483106132d357606483816132c9576132c86154a8565b5b0492506002810190505b600a83106132e2576001810190505b80915050919050565b6132f3613554565b613329576040517fd7e6bcf800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b6133336132eb565b5f61333c613037565b90508281600201908161334f91906157a7565b508181600301908161336191906157a7565b505f801b815f01819055505f801b8160010181905550505050565b5f61338e613388613572565b83613580565b9050919050565b5f805f806133a386866135c0565b9250925092506133b38282613615565b82935050505092915050565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663203d0114826040518263ffffffff1660e01b815260040161340c91906149e9565b602060405180830381865afa158015613427573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061344b9190614ae0565b61348c57806040517f2a7c6ef600000000000000000000000000000000000000000000000000000000815260040161348391906149e9565b60405180910390fd5b50565b5f6134bb7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b613777565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b6134eb82613780565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f81511115613547576135418282613849565b50613550565b61354f6138c9565b5b5050565b5f61355d6128d0565b5f0160089054906101000a900460ff16905090565b5f61357b613905565b905090565b5f6040517f190100000000000000000000000000000000000000000000000000000000000081528360028201528260228201526042812091505092915050565b5f805f6041845103613600575f805f602087015192506040870151915060608701515f1a90506135f288828585613968565b95509550955050505061360e565b5f600285515f1b9250925092505b9250925092565b5f600381111561362857613627613e2d565b5b82600381111561363b5761363a613e2d565b5b0315613773576001600381111561365557613654613e2d565b5b82600381111561366857613667613e2d565b5b0361369f576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600260038111156136b3576136b2613e2d565b5b8260038111156136c6576136c5613e2d565b5b0361370a57805f1c6040517ffce698f70000000000000000000000000000000000000000000000000000000081526004016137019190614782565b60405180910390fd5b60038081111561371d5761371c613e2d565b5b8260038111156137305761372f613e2d565b5b0361377257806040517fd78bce0c0000000000000000000000000000000000000000000000000000000081526004016137699190614225565b60405180910390fd5b5b5050565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b036137db57806040517f4c9c8ce30000000000000000000000000000000000000000000000000000000081526004016137d291906149e9565b60405180910390fd5b806138077f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b613777565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f808473ffffffffffffffffffffffffffffffffffffffff168460405161387291906158a6565b5f60405180830381855af49150503d805f81146138aa576040519150601f19603f3d011682016040523d82523d5f602084013e6138af565b606091505b50915091506138bf858383613a4f565b9250505092915050565b5f341115613903576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f61392f613adc565b613937613b52565b463060405160200161394d9594939291906158bc565b60405160208183030381529060405280519060200120905090565b5f805f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c11156139a4575f600385925092509250613a45565b5f6001888888886040515f81526020016040526040516139c79493929190615928565b6020604051602081039080840390855afa1580156139e7573d5f803e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603613a38575f60015f801b93509350935050613a45565b805f805f1b935093509350505b9450945094915050565b606082613a6457613a5f82613bc9565b613ad4565b5f8251148015613a8a57505f8473ffffffffffffffffffffffffffffffffffffffff163b145b15613acc57836040517f9996b315000000000000000000000000000000000000000000000000000000008152600401613ac391906149e9565b60405180910390fd5b819050613ad5565b5b9392505050565b5f80613ae6613037565b90505f613af161305e565b90505f81511115613b0d57808051906020012092505050613b4f565b5f825f015490505f801b8114613b2857809350505050613b4f565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f80613b5c613037565b90505f613b676130fc565b90505f81511115613b8357808051906020012092505050613bc6565b5f826001015490505f801b8114613b9f57809350505050613bc6565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f81511115613bdb5780518082602001fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f81519050919050565b5f82825260208201905092915050565b5f5b83811015613c44578082015181840152602081019050613c29565b5f8484015250505050565b5f601f19601f8301169050919050565b5f613c6982613c0d565b613c738185613c17565b9350613c83818560208601613c27565b613c8c81613c4f565b840191505092915050565b5f6020820190508181035f830152613caf8184613c5f565b905092915050565b5f604051905090565b5f80fd5b5f80fd5b5f819050919050565b613cda81613cc8565b8114613ce4575f80fd5b50565b5f81359050613cf581613cd1565b92915050565b5f60208284031215613d1057613d0f613cc0565b5b5f613d1d84828501613ce7565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f613d7882613d4f565b9050919050565b613d8881613d6e565b82525050565b5f613d998383613d7f565b60208301905092915050565b5f602082019050919050565b5f613dbb82613d26565b613dc58185613d30565b9350613dd083613d40565b805f5b83811015613e00578151613de78882613d8e565b9750613df283613da5565b925050600181019050613dd3565b5085935050505092915050565b5f6020820190508181035f830152613e258184613db1565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b60028110613e6b57613e6a613e2d565b5b50565b5f819050613e7b82613e5a565b919050565b5f613e8a82613e6e565b9050919050565b613e9a81613e80565b82525050565b5f602082019050613eb35f830184613e91565b92915050565b60028110613ec5575f80fd5b50565b5f81359050613ed681613eb9565b92915050565b5f8060408385031215613ef257613ef1613cc0565b5b5f613eff85828601613ce7565b9250506020613f1085828601613ec8565b9150509250929050565b5f80fd5b5f80fd5b5f80fd5b5f8083601f840112613f3b57613f3a613f1a565b5b8235905067ffffffffffffffff811115613f5857613f57613f1e565b5b602083019150836020820283011115613f7457613f73613f22565b5b9250929050565b5f8083601f840112613f9057613f8f613f1a565b5b8235905067ffffffffffffffff811115613fad57613fac613f1e565b5b602083019150836001820283011115613fc957613fc8613f22565b5b9250929050565b5f805f805f60608688031215613fe957613fe8613cc0565b5b5f613ff688828901613ce7565b955050602086013567ffffffffffffffff81111561401757614016613cc4565b5b61402388828901613f26565b9450945050604086013567ffffffffffffffff81111561404657614045613cc4565b5b61405288828901613f7b565b92509250509295509295909350565b61406a81613d6e565b8114614074575f80fd5b50565b5f8135905061408581614061565b92915050565b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6140c582613c4f565b810181811067ffffffffffffffff821117156140e4576140e361408f565b5b80604052505050565b5f6140f6613cb7565b905061410282826140bc565b919050565b5f67ffffffffffffffff8211156141215761412061408f565b5b61412a82613c4f565b9050602081019050919050565b828183375f83830152505050565b5f61415761415284614107565b6140ed565b9050828152602081018484840111156141735761417261408b565b5b61417e848285614137565b509392505050565b5f82601f83011261419a57614199613f1a565b5b81356141aa848260208601614145565b91505092915050565b5f80604083850312156141c9576141c8613cc0565b5b5f6141d685828601614077565b925050602083013567ffffffffffffffff8111156141f7576141f6613cc4565b5b61420385828601614186565b9150509250929050565b5f819050919050565b61421f8161420d565b82525050565b5f6020820190506142385f830184614216565b92915050565b5f805f6040848603121561425557614254613cc0565b5b5f61426286828701613ce7565b935050602084013567ffffffffffffffff81111561428357614282613cc4565b5b61428f86828701613f7b565b92509250509250925092565b5f805f805f606086880312156142b4576142b3613cc0565b5b5f6142c188828901613ce7565b955050602086013567ffffffffffffffff8111156142e2576142e1613cc4565b5b6142ee88828901613f7b565b9450945050604086013567ffffffffffffffff81111561431157614310613cc4565b5b61431d88828901613f7b565b92509250509295509295909350565b5f7fff0000000000000000000000000000000000000000000000000000000000000082169050919050565b6143608161432c565b82525050565b61436f81613cc8565b82525050565b61437e81613d6e565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b6143b681613cc8565b82525050565b5f6143c783836143ad565b60208301905092915050565b5f602082019050919050565b5f6143e982614384565b6143f3818561438e565b93506143fe8361439e565b805f5b8381101561442e57815161441588826143bc565b9750614420836143d3565b925050600181019050614401565b5085935050505092915050565b5f60e08201905061444e5f83018a614357565b81810360208301526144608189613c5f565b905081810360408301526144748188613c5f565b90506144836060830187614366565b6144906080830186614375565b61449d60a0830185614216565b81810360c08301526144af81846143df565b905098975050505050505050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f82825260208201905092915050565b5f61450082613c0d565b61450a81856144e6565b935061451a818560208601613c27565b61452381613c4f565b840191505092915050565b5f61453983836144f6565b905092915050565b5f602082019050919050565b5f614557826144bd565b61456181856144c7565b935083602082028501614573856144d7565b805f5b858110156145ae578484038952815161458f858261452e565b945061459a83614541565b925060208a01995050600181019050614576565b50829750879550505050505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b600281106145fa576145f9613e2d565b5b50565b5f81905061460a826145e9565b919050565b5f614619826145fd565b9050919050565b6146298161460f565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f6146538261462f565b61465d8185614639565b935061466d818560208601613c27565b61467681613c4f565b840191505092915050565b5f604083015f8301516146965f860182614620565b50602083015184820360208601526146ae8282614649565b9150508091505092915050565b5f6146c68383614681565b905092915050565b5f602082019050919050565b5f6146e4826145c0565b6146ee81856145ca565b935083602082028501614700856145da565b805f5b8581101561473b578484038952815161471c85826146bb565b9450614727836146ce565b925060208a01995050600181019050614703565b50829750879550505050505092915050565b5f6040820190508181035f830152614765818561454d565b9050818103602083015261477981846146da565b90509392505050565b5f6020820190506147955f830184614366565b92915050565b5f82825260208201905092915050565b5f6147b58261462f565b6147bf818561479b565b93506147cf818560208601613c27565b6147d881613c4f565b840191505092915050565b5f6040820190508181035f8301526147fb818561454d565b9050818103602083015261480f81846147ab565b90509392505050565b5f6020828403121561482d5761482c613cc0565b5b5f61483a84828501613ec8565b91505092915050565b5f81905092915050565b5f61485782613c0d565b6148618185614843565b9350614871818560208601613c27565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f6148b1600283614843565b91506148bc8261487d565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f6148fb600183614843565b9150614906826148c7565b600182019050919050565b5f61491c828761484d565b9150614927826148a5565b9150614933828661484d565b915061493e826148ef565b915061494a828561484d565b9150614955826148ef565b9150614961828461484d565b915081905095945050505050565b5f67ffffffffffffffff82169050919050565b61498b8161496f565b82525050565b5f6020820190506149a45f830184614982565b92915050565b5f815190506149b881614061565b92915050565b5f602082840312156149d3576149d2613cc0565b5b5f6149e0848285016149aa565b91505092915050565b5f6020820190506149fc5f830184614375565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f614a3982613cc8565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203614a6b57614a6a614a02565b5b600182019050919050565b5f606082019050614a895f830186614366565b614a966020830185614366565b614aa36040830184613e91565b949350505050565b5f8115159050919050565b614abf81614aab565b8114614ac9575f80fd5b50565b5f81519050614ada81614ab6565b92915050565b5f60208284031215614af557614af4613cc0565b5b5f614b0284828501614acc565b91505092915050565b5f604082019050614b1e5f830185614366565b614b2b6020830184614375565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b5f82356001604003833603038112614b8657614b85614b5f565b5b80830191505092915050565b60028110614b9e575f80fd5b50565b5f8135614bad81614b92565b80915050919050565b5f815f1b9050919050565b5f60ff614bcd84614bb6565b9350801983169250808416831791505092915050565b5f614bed826145fd565b9050919050565b5f819050919050565b614c0682614be3565b614c19614c1282614bf4565b8354614bc1565b8255505050565b5f8083356001602003843603038112614c3c57614c3b614b5f565b5b80840192508235915067ffffffffffffffff821115614c5e57614c5d614b63565b5b602083019250600182023603831315614c7a57614c79614b67565b5b509250929050565b5f82905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f6002820490506001821680614cd057607f821691505b602082108103614ce357614ce2614c8c565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f60088302614d457fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82614d0a565b614d4f8683614d0a565b95508019841693508086168417925050509392505050565b5f819050919050565b5f614d8a614d85614d8084613cc8565b614d67565b613cc8565b9050919050565b5f819050919050565b614da383614d70565b614db7614daf82614d91565b848454614d16565b825550505050565b5f90565b614dcb614dbf565b614dd6818484614d9a565b505050565b5b81811015614df957614dee5f82614dc3565b600181019050614ddc565b5050565b601f821115614e3e57614e0f81614ce9565b614e1884614cfb565b81016020851015614e27578190505b614e3b614e3385614cfb565b830182614ddb565b50505b505050565b5f82821c905092915050565b5f614e5e5f1984600802614e43565b1980831691505092915050565b5f614e768383614e4f565b9150826002028217905092915050565b614e908383614c82565b67ffffffffffffffff811115614ea957614ea861408f565b5b614eb38254614cb9565b614ebe828285614dfd565b5f601f831160018114614eeb575f8415614ed9578287013590505b614ee38582614e6b565b865550614f4a565b601f198416614ef986614ce9565b5f5b82811015614f2057848901358255600182019150602085019450602081019050614efb565b86831015614f3d5784890135614f39601f891682614e4f565b8355505b6001600288020188555050505b50505050505050565b614f5e838383614e86565b505050565b5f81015f830180614f7381614ba1565b9050614f7f8184614bfd565b5050506001810160208301614f948185614c20565b614f9f818386614f53565b505050505050565b614fb18282614f63565b5050565b5f80fd5b5f80fd5b5f67ffffffffffffffff821115614fd757614fd661408f565b5b614fe082613c4f565b9050602081019050919050565b5f614fff614ffa84614fbd565b6140ed565b90508281526020810184848401111561501b5761501a61408b565b5b615026848285613c27565b509392505050565b5f82601f83011261504257615041613f1a565b5b8151615052848260208601614fed565b91505092915050565b5f608082840312156150705761506f614fb5565b5b61507a60806140ed565b90505f615089848285016149aa565b5f83015250602061509c848285016149aa565b602083015250604082015167ffffffffffffffff8111156150c0576150bf614fb9565b5b6150cc8482850161502e565b604083015250606082015167ffffffffffffffff8111156150f0576150ef614fb9565b5b6150fc8482850161502e565b60608301525092915050565b5f6020828403121561511d5761511c613cc0565b5b5f82015167ffffffffffffffff81111561513a57615139613cc4565b5b6151468482850161505b565b91505092915050565b5f819050919050565b5f8135905061516681614b92565b92915050565b5f61517a6020840184615158565b905092915050565b5f80fd5b5f80fd5b5f80fd5b5f80833560016020038436030381126151aa576151a961518a565b5b83810192508235915060208301925067ffffffffffffffff8211156151d2576151d1615182565b5b6001820236038313156151e8576151e7615186565b5b509250929050565b5f6151fb8385614639565b9350615208838584614137565b61521183613c4f565b840190509392505050565b5f6040830161522d5f84018461516c565b6152395f860182614620565b50615247602084018461518e565b858303602087015261525a8382846151f0565b925050508091505092915050565b5f615273838361521c565b905092915050565b5f823560016040038336030381126152965761529561518a565b5b82810191505092915050565b5f602082019050919050565b5f6152b983856145ca565b9350836020840285016152cb8461514f565b805f5b8781101561530e5784840389526152e5828461527b565b6152ef8582615268565b94506152fa836152a2565b925060208a019950506001810190506152ce565b50829750879450505050509392505050565b5f6060820190506153335f830187614366565b8181036020830152615345818661454d565b9050818103604083015261535a8184866152ae565b905095945050505050565b5f6040820190506153785f830185614366565b6153856020830184614366565b9392505050565b5f615397838561479b565b93506153a4838584614137565b6153ad83613c4f565b840190509392505050565b5f6060820190506153cb5f830187614366565b81810360208301526153dd818661454d565b905081810360408301526153f281848661538c565b905095945050505050565b7f4549503731323a20556e696e697469616c697a656400000000000000000000005f82015250565b5f615431601583613c17565b915061543c826153fd565b602082019050919050565b5f6020820190508181035f83015261545e81615425565b9050919050565b5f6080820190506154785f830187614366565b6154856020830186614366565b6154926040830185614366565b61549f6060830184613e91565b95945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f602082840312156154ea576154e9613cc0565b5b5f6154f784828501615158565b91505092915050565b5f81905092915050565b5f6155158385615500565b9350615522838584614137565b82840190509392505050565b5f61553a82848661550a565b91508190509392505050565b61554f8161460f565b82525050565b5f6060820190506155685f830186614216565b6155756020830185615546565b6155826040830184614216565b949350505050565b5f81519050919050565b5f81905092915050565b5f819050602082019050919050565b6155b68161420d565b82525050565b5f6155c783836155ad565b60208301905092915050565b5f602082019050919050565b5f6155e98261558a565b6155f38185615594565b93506155fe8361559e565b805f5b8381101561562e57815161561588826155bc565b9750615620836155d3565b925050600181019050615601565b5085935050505092915050565b5f61564682846155df565b915081905092915050565b5f6080820190506156645f830187614216565b6156716020830186614366565b61567e6040830185614366565b61568b6060830184614216565b95945050505050565b5f815190506156a281613cd1565b92915050565b5f602082840312156156bd576156bc613cc0565b5b5f6156ca84828501615694565b91505092915050565b6156dc8161420d565b81146156e6575f80fd5b50565b5f815190506156f7816156d3565b92915050565b5f6020828403121561571257615711613cc0565b5b5f61571f848285016156e9565b91505092915050565b5f60408201905061573b5f830185614216565b6157486020830184614366565b9392505050565b5f819050815f5260205f209050919050565b601f8211156157a2576157738161574f565b61577c84614cfb565b8101602085101561578b578190505b61579f61579785614cfb565b830182614ddb565b50505b505050565b6157b082613c0d565b67ffffffffffffffff8111156157c9576157c861408f565b5b6157d38254614cb9565b6157de828285615761565b5f60209050601f83116001811461580f575f84156157fd578287015190505b6158078582614e6b565b86555061586e565b601f19841661581d8661574f565b5f5b828110156158445784890151825560018201915060208501945060208101905061581f565b86831015615861578489015161585d601f891682614e4f565b8355505b6001600288020188555050505b505050505050565b5f6158808261462f565b61588a8185615500565b935061589a818560208601613c27565b80840191505092915050565b5f6158b18284615876565b915081905092915050565b5f60a0820190506158cf5f830188614216565b6158dc6020830187614216565b6158e96040830186614216565b6158f66060830185614366565b6159036080830184614375565b9695505050505050565b5f60ff82169050919050565b6159228161590d565b82525050565b5f60808201905061593b5f830187614216565b6159486020830186615919565b6159556040830185614216565b6159626060830184614216565b9594505050505056fe43727367656e566572696669636174696f6e2875696e743235362063727349642c75696e74323536206d61784269744c656e6774682c62797465732063727344696765737429507265704b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e4964294b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e49642c75696e74323536206b657949642c4b65794469676573745b5d206b657944696765737473294b65794469676573742875696e7438206b6579547970652c627974657320646967657374294b65794469676573742875696e7438206b6579547970652c62797465732064696765737429
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01)W_5`\xE0\x1C\x80cb\x97\x87\x87\x11a\0\xAAW\x80c\xBA\xFF!\x1E\x11a\0nW\x80c\xBA\xFF!\x1E\x14a\x03\xB4W\x80c\xC4\x11Xt\x14a\x03\xDEW\x80c\xC5[\x87$\x14a\x03\xF4W\x80c\xCA\xA3g\xDB\x14a\x041W\x80c\xD2\xC0\x8A\x91\x14a\x04YW\x80c\xD5/\x10\xEB\x14a\x04\x81Wa\x01)V[\x80cb\x97\x87\x87\x14a\x02\xDFW\x80cu\x14\xA2\xAC\x14a\x03\x07W\x80c\x84\xB0\x19n\x14a\x03\x1DW\x80c\x93f\x08\xAE\x14a\x03MW\x80c\xAD<\xB1\xCC\x14a\x03\x8AWa\x01)V[\x80cE\xAF&\x1B\x11a\0\xF1W\x80cE\xAF&\x1B\x14a\x02\rW\x80cF\x10\xFF\xE8\x14a\x02IW\x80cO\x1E\xF2\x86\x14a\x02qW\x80cR\xD1\x90-\x14a\x02\x8DW\x80cX\x9A\xDB\x0E\x14a\x02\xB7Wa\x01)V[\x80c\r\x8En,\x14a\x01-W\x80c\x16\xC7\x13\xD9\x14a\x01WW\x80c\x19\xF4\xF62\x14a\x01\x93W\x80c9\xF78\x10\x14a\x01\xCFW\x80c<\x02\xF84\x14a\x01\xE5W[_\x80\xFD[4\x80\x15a\x018W_\x80\xFD[Pa\x01Aa\x04\xABV[`@Qa\x01N\x91\x90a<\x97V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01bW_\x80\xFD[Pa\x01}`\x04\x806\x03\x81\x01\x90a\x01x\x91\x90a<\xFBV[a\x05&V[`@Qa\x01\x8A\x91\x90a>\rV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x9EW_\x80\xFD[Pa\x01\xB9`\x04\x806\x03\x81\x01\x90a\x01\xB4\x91\x90a<\xFBV[a\x05\xF7V[`@Qa\x01\xC6\x91\x90a>\xA0V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xDAW_\x80\xFD[Pa\x01\xE3a\x06\xA4V[\0[4\x80\x15a\x01\xF0W_\x80\xFD[Pa\x02\x0B`\x04\x806\x03\x81\x01\x90a\x02\x06\x91\x90a>\xDCV[a\t\x13V[\0[4\x80\x15a\x02\x18W_\x80\xFD[Pa\x023`\x04\x806\x03\x81\x01\x90a\x02.\x91\x90a<\xFBV[a\n\xC2V[`@Qa\x02@\x91\x90a>\xA0V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02TW_\x80\xFD[Pa\x02o`\x04\x806\x03\x81\x01\x90a\x02j\x91\x90a?\xD0V[a\x0BWV[\0[a\x02\x8B`\x04\x806\x03\x81\x01\x90a\x02\x86\x91\x90aA\xB3V[a\x10\x9CV[\0[4\x80\x15a\x02\x98W_\x80\xFD[Pa\x02\xA1a\x10\xBBV[`@Qa\x02\xAE\x91\x90aB%V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xC2W_\x80\xFD[Pa\x02\xDD`\x04\x806\x03\x81\x01\x90a\x02\xD8\x91\x90aB>V[a\x10\xECV[\0[4\x80\x15a\x02\xEAW_\x80\xFD[Pa\x03\x05`\x04\x806\x03\x81\x01\x90a\x03\0\x91\x90aB\x9BV[a\x14>V[\0[4\x80\x15a\x03\x12W_\x80\xFD[Pa\x03\x1Ba\x19\x1FV[\0[4\x80\x15a\x03(W_\x80\xFD[Pa\x031a\x1A=V[`@Qa\x03D\x97\x96\x95\x94\x93\x92\x91\x90aD;V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03XW_\x80\xFD[Pa\x03s`\x04\x806\x03\x81\x01\x90a\x03n\x91\x90a<\xFBV[a\x1BFV[`@Qa\x03\x81\x92\x91\x90aGMV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x95W_\x80\xFD[Pa\x03\x9Ea\x1E\xEDV[`@Qa\x03\xAB\x91\x90a<\x97V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xBFW_\x80\xFD[Pa\x03\xC8a\x1F&V[`@Qa\x03\xD5\x91\x90aG\x82V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xE9W_\x80\xFD[Pa\x03\xF2a\x1F=V[\0[4\x80\x15a\x03\xFFW_\x80\xFD[Pa\x04\x1A`\x04\x806\x03\x81\x01\x90a\x04\x15\x91\x90a<\xFBV[a \x8DV[`@Qa\x04(\x92\x91\x90aG\xE3V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04<W_\x80\xFD[Pa\x04W`\x04\x806\x03\x81\x01\x90a\x04R\x91\x90aH\x18V[a#\x9FV[\0[4\x80\x15a\x04dW_\x80\xFD[Pa\x04\x7F`\x04\x806\x03\x81\x01\x90a\x04z\x91\x90a<\xFBV[a%\xA9V[\0[4\x80\x15a\x04\x8CW_\x80\xFD[Pa\x04\x95a'\xA4V[`@Qa\x04\xA2\x91\x90aG\x82V[`@Q\x80\x91\x03\x90\xF3[```@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x04\xEC_a'\xBBV[a\x04\xF6`\x01a'\xBBV[a\x04\xFF_a'\xBBV[`@Q` \x01a\x05\x12\x94\x93\x92\x91\x90aI\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[``_a\x051a(\x85V[\x90P_\x81`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\x02\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x82\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x05\xE9W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x05\xA0W[PPPPP\x92PPP\x91\x90PV[_\x80a\x06\x01a(\x85V[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x06dW\x82`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06[\x91\x90aG\x82V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\r\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x92PPP\x91\x90PV[`\x01a\x06\xAEa(\xACV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x06\xEFW`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03_a\x06\xFAa(\xD0V[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x07BWP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x07yW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x082`@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa(\xF7V[_a\x08;a(\x85V[\x90P`\xF8`\x03`\x06\x81\x11\x15a\x08SWa\x08Ra>-V[[\x90\x1B\x81`\x04\x01\x81\x90UP`\xF8`\x04`\x06\x81\x11\x15a\x08sWa\x08ra>-V[[\x90\x1B\x81`\x05\x01\x81\x90UP`\xF8`\x05`\x06\x81\x11\x15a\x08\x93Wa\x08\x92a>-V[[\x90\x1B\x81`\t\x01\x81\x90UP`\xF8`\x06\x80\x81\x11\x15a\x08\xB2Wa\x08\xB1a>-V[[\x90\x1B\x81`\x0E\x01\x81\x90UPP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\t\x07\x91\x90aI\x91V[`@Q\x80\x91\x03\x90\xA1PPV[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tpW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x94\x91\x90aI\xBEV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\n\x03W3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xFA\x91\x90aI\xE9V[`@Q\x80\x91\x03\x90\xFD[_a\n\x0Ca(\x85V[\x90P\x80`\t\x01_\x81T\x80\x92\x91\x90a\n\"\x90aJ/V[\x91\x90PUP_\x81`\t\x01T\x90P\x83\x82`\n\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82\x82`\r\x01_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a\n|Wa\n{a>-V[[\x02\x17\x90UP\x7F?\x03\x8Fo\x88\xCB01\xB7q\x85\x88@:.\xC2 Wj\x86\x8B\xE0}\xDEL\x02\xB8F\xCA5.\xF5\x81\x85\x85`@Qa\n\xB4\x93\x92\x91\x90aJvV[`@Q\x80\x91\x03\x90\xA1PPPPV[_\x80a\n\xCCa(\x85V[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x0B/W\x82`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B&\x91\x90aG\x82V[`@Q\x80\x91\x03\x90\xFD[\x80`\r\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE5'^\xAF3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\xA4\x91\x90aI\xE9V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xBFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xE3\x91\x90aJ\xE0V[a\x0C$W3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\x1B\x91\x90aI\xE9V[`@Q\x80\x91\x03\x90\xFD[_a\x0C-a(\x85V[\x90P_\x81`\x06\x01_\x88\x81R` \x01\x90\x81R` \x01_ T\x90P_a\x0CS\x82\x89\x89\x89a)\rV[\x90P_a\x0Ca\x82\x87\x87a*\xE4V[\x90P\x83_\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\r\x01W\x88\x81`@Q\x7F\x98\xFB\x95}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\xF8\x92\x91\x90aK\x0BV[`@Q\x80\x91\x03\x90\xFD[`\x01\x84_\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x84`\x02\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_\x81\x80T\x90P\x90P\x85`\x01\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x0E$WPa\x0E#\x81a+IV[[\x15a\x10\x8FW`\x01\x86`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_[\x8A\x8A\x90P\x81\x10\x15a\x0E\xDAW\x86`\x07\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x8B\x8B\x83\x81\x81\x10a\x0E\x87Wa\x0E\x86aK2V[[\x90P` \x02\x81\x01\x90a\x0E\x99\x91\x90aKkV[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P\x81\x81a\x0E\xCB\x91\x90aO\xA7V[PP\x80\x80`\x01\x01\x91PPa\x0EVV[P\x83\x86`\x03\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x8A\x86`\x08\x01\x81\x90UP_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x17Wa\x0F\x16a@\x8FV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0FJW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0F5W\x90P[P\x90P_[\x82\x81\x10\x15a\x10OWs\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3\xB2\xA8t\x85\x83\x81T\x81\x10a\x0F\x9AWa\x0F\x99aK2V[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\xDE\x91\x90aI\xE9V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xF8W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10 \x91\x90aQ\x08V[``\x01Q\x82\x82\x81Q\x81\x10a\x107Wa\x106aK2V[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa\x0FOV[P\x7F\xEB\x85\xC2m\xBC\xADF\xB8\nh\xA0\xF2L\xCE|,\x90\xF0\xA1\xFA\xDE\xD8A\x84\x13\x889\xFC\x9E\x80\xA2[\x8C\x82\x8D\x8D`@Qa\x10\x85\x94\x93\x92\x91\x90aS V[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPPPPPV[a\x10\xA4a+\xDAV[a\x10\xAD\x82a,\xC0V[a\x10\xB7\x82\x82a-\xB3V[PPV[_a\x10\xC4a.\xD1V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE5'^\xAF3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x119\x91\x90aI\xE9V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11TW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11x\x91\x90aJ\xE0V[a\x11\xB9W3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\xB0\x91\x90aI\xE9V[`@Q\x80\x91\x03\x90\xFD[_a\x11\xC2a(\x85V[\x90P_a\x11\xCE\x85a/XV[\x90P_a\x11\xDC\x82\x86\x86a*\xE4V[\x90P\x82_\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x12|W\x85\x81`@Q\x7F3\xCA\x1F\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12s\x92\x91\x90aK\x0BV[`@Q\x80\x91\x03\x90\xFD[`\x01\x83_\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x83`\x02\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x83`\x01\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x13\x9BWPa\x13\x9A\x81\x80T\x90Pa+IV[[\x15a\x145W`\x01\x84`\x01\x01_\x89\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x82\x84`\x03\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x81\x90UP_\x84`\x06\x01_\x89\x81R` \x01\x90\x81R` \x01_ T\x90P\x7Fx\xB1y\x17m\x1F\x19\xD7\xC2\x8E\x80\x82=\xEB\xA2bM\xA2\xCA.\xC6K\x17\x01\xF3c*\x87\xC9\xAE\xDC\x92\x88\x82`@Qa\x14+\x92\x91\x90aSeV[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPV[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE5'^\xAF3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\x8B\x91\x90aI\xE9V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xA6W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xCA\x91\x90aJ\xE0V[a\x15\x0BW3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15\x02\x91\x90aI\xE9V[`@Q\x80\x91\x03\x90\xFD[_a\x15\x14a(\x85V[\x90P_\x81`\n\x01_\x88\x81R` \x01\x90\x81R` \x01_ T\x90P_a\x15:\x88\x83\x89\x89a/\xB0V[\x90P_a\x15H\x82\x87\x87a*\xE4V[\x90P\x83_\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x15\xE8W\x88\x81`@Q\x7F\xFC\xF5\xA6\xE9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15\xDF\x92\x91\x90aK\x0BV[`@Q\x80\x91\x03\x90\xFD[`\x01\x84_\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x84`\x02\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_\x81\x80T\x90P\x90P\x85`\x01\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x17\x0BWPa\x17\n\x81a+IV[[\x15a\x19\x12W`\x01\x86`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x89\x89\x87`\x0B\x01_\x8E\x81R` \x01\x90\x81R` \x01_ \x91\x82a\x17]\x92\x91\x90aN\x86V[P\x83\x86`\x03\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x8A\x86`\x0C\x01\x81\x90UP_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\x9AWa\x17\x99a@\x8FV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\xCDW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x17\xB8W\x90P[P\x90P_[\x82\x81\x10\x15a\x18\xD2Ws\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3\xB2\xA8t\x85\x83\x81T\x81\x10a\x18\x1DWa\x18\x1CaK2V[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18a\x91\x90aI\xE9V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18{W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xA3\x91\x90aQ\x08V[``\x01Q\x82\x82\x81Q\x81\x10a\x18\xBAWa\x18\xB9aK2V[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa\x17\xD2V[P\x7F\"X\xB7?\xAE\xD3?\xB2\xE2\xEAED\x03\xBE\xF9t\x92\x0C\xAFh*\xB3\xA7#HO\xCFgU;\x16\xA2\x8C\x82\x8D\x8D`@Qa\x19\x08\x94\x93\x92\x91\x90aS\xB8V[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPPPPPV[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19|W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xA0\x91\x90aI\xBEV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x1A\x0FW3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1A\x06\x91\x90aI\xE9V[`@Q\x80\x91\x03\x90\xFD[\x7F\x11\xDBB\xC1\x87\x8F.(\x19$\x1FRP\x98Ec\xF0l\xF2(\x18\xE7\xAD\xB8jf\x92\x1D\x15\xD5\x9D?`@Q`@Q\x80\x91\x03\x90\xA1V[_``\x80_\x80_``_a\x1AOa07V[\x90P_\x80\x1B\x81_\x01T\x14\x80\x15a\x1AjWP_\x80\x1B\x81`\x01\x01T\x14[a\x1A\xA9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1A\xA0\x90aTGV[`@Q\x80\x91\x03\x90\xFD[a\x1A\xB1a0^V[a\x1A\xB9a0\xFCV[F0_\x80\x1B_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\xD8Wa\x1A\xD7a@\x8FV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1B\x06W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94\x93\x92\x91\x90\x97P\x97P\x97P\x97P\x97P\x97P\x97PP\x90\x91\x92\x93\x94\x95\x96V[``\x80_a\x1BRa(\x85V[\x90P\x80`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x1B\xB5W\x83`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1B\xAC\x91\x90aG\x82V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x82`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1ClW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x1C#W[PPPPP\x90P_\x81Q\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1C\x93Wa\x1C\x92a@\x8FV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1C\xC6W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1C\xB1W\x90P[P\x90P_[\x82\x81\x10\x15a\x1D\xABWs\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3\xB2\xA8t\x85\x83\x81Q\x81\x10a\x1D\x16Wa\x1D\x15aK2V[[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1D:\x91\x90aI\xE9V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1DTW=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D|\x91\x90aQ\x08V[``\x01Q\x82\x82\x81Q\x81\x10a\x1D\x93Wa\x1D\x92aK2V[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa\x1C\xCBV[P\x80\x85`\x07\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x1E\xD9W\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x1E$Wa\x1E#a>-V[[`\x01\x81\x11\x15a\x1E6Wa\x1E5a>-V[[\x81R` \x01`\x01\x82\x01\x80Ta\x1EJ\x90aL\xB9V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1Ev\x90aL\xB9V[\x80\x15a\x1E\xC1W\x80`\x1F\x10a\x1E\x98Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1E\xC1V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1E\xA4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x1D\xE0V[PPPP\x90P\x96P\x96PPPPPP\x91P\x91V[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[_\x80a\x1F0a(\x85V[\x90P\x80`\x0C\x01T\x91PP\x90V[`\x03_a\x1FHa(\xD0V[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x1F\x90WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x1F\xC7W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_a \x15a(\x85V[\x90P`\xF8`\x06\x80\x81\x11\x15a ,Wa +a>-V[[\x90\x1B\x81`\x0E\x01\x81\x90UPP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa \x81\x91\x90aI\x91V[`@Q\x80\x91\x03\x90\xA1PPV[``\x80_a \x99a(\x85V[\x90P\x80`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a \xFCW\x83`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a \xF3\x91\x90aG\x82V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x82`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a!\xB3W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a!jW[PPPPP\x90P_\x81Q\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!\xDAWa!\xD9a@\x8FV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\rW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a!\xF8W\x90P[P\x90P_[\x82\x81\x10\x15a\"\xF2Ws\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3\xB2\xA8t\x85\x83\x81Q\x81\x10a\"]Wa\"\\aK2V[[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\"\x81\x91\x90aI\xE9V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\x9BW=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\xC3\x91\x90aQ\x08V[``\x01Q\x82\x82\x81Q\x81\x10a\"\xDAWa\"\xD9aK2V[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa\"\x12V[P\x80\x85`\x0B\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80\x80Ta#\x13\x90aL\xB9V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta#?\x90aL\xB9V[\x80\x15a#\x8AW\x80`\x1F\x10a#aWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a#\x8AV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a#mW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x96P\x96PPPPPP\x91P\x91V[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#\xFCW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$ \x91\x90aI\xBEV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a$\x8FW3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a$\x86\x91\x90aI\xE9V[`@Q\x80\x91\x03\x90\xFD[_a$\x98a(\x85V[\x90P\x80`\x04\x01_\x81T\x80\x92\x91\x90a$\xAE\x90aJ/V[\x91\x90PUP_\x81`\x04\x01T\x90P\x81`\x05\x01_\x81T\x80\x92\x91\x90a$\xCF\x90aJ/V[\x91\x90PUP_\x82`\x05\x01T\x90P\x80\x83`\x06\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x81\x83`\x06\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82`\x0E\x01_\x81T\x80\x92\x91\x90a% \x90aJ/V[\x91\x90PUP_\x83`\x0E\x01T\x90P\x84\x84`\r\x01_\x85\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a%bWa%aa>-V[[\x02\x17\x90UP\x7F\x02\x02@\x07\xD9et\xDB\xC9\xD1\x13(\xBF\xEE\x98\x93\xE7\xC7\xBBN\xF4\xAA\x80m\xF3;\xFD\xF4T\xEB^`\x83\x82\x87`@Qa%\x9A\x93\x92\x91\x90aJvV[`@Q\x80\x91\x03\x90\xA1PPPPPV[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\x06W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&*\x91\x90aI\xBEV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a&\x99W3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&\x90\x91\x90aI\xE9V[`@Q\x80\x91\x03\x90\xFD[_a&\xA2a(\x85V[\x90P\x80`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a'\x05W\x81`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&\xFC\x91\x90aG\x82V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P_\x82`\r\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x82`\x0E\x01_\x81T\x80\x92\x91\x90a'S\x90aJ/V[\x91\x90PUP_\x83`\x0E\x01T\x90P\x7F|\xF0\xBE&,z\xEC\x97\xBC\xB2OH\xCB%U\x07\"\xE8\xDD7\x1Dy\xC4/f\x98\xF3\xBB'\0W1\x83\x86\x83\x85`@Qa'\x95\x94\x93\x92\x91\x90aTeV[`@Q\x80\x91\x03\x90\xA1PPPPPV[_\x80a'\xAEa(\x85V[\x90P\x80`\x08\x01T\x91PP\x90V[``_`\x01a'\xC9\x84a1\x9AV[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'\xE7Wa'\xE6a@\x8FV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a(\x19W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x01\x82\x01\x90P[`\x01\x15a(zW\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a(oWa(naT\xA8V[[\x04\x94P_\x85\x03a(&W[\x81\x93PPPP\x91\x90PV[_\x7F\x0B\x8F\xDB\x1F\ncV\xDD \xA6\xCB\xC6\xF9f\x8F\xAC#\xB8_\x96W]\x10\xE33\xE6\x03\xFA\xA7\x94\xAC\0\x90P\x90V[_a(\xB5a(\xD0V[_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[a(\xFFa2\xEBV[a)\t\x82\x82a3+V[PPV[_\x80\x83\x83\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a),Wa)+a@\x8FV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a)ZW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_[\x84\x84\x90P\x81\x10\x15a*^W`@Q\x80``\x01`@R\x80`%\x81R` \x01aZP`%\x919\x80Q\x90` \x01 \x85\x85\x83\x81\x81\x10a)\x9DWa)\x9CaK2V[[\x90P` \x02\x81\x01\x90a)\xAF\x91\x90aKkV[_\x01` \x81\x01\x90a)\xC0\x91\x90aT\xD5V[\x86\x86\x84\x81\x81\x10a)\xD3Wa)\xD2aK2V[[\x90P` \x02\x81\x01\x90a)\xE5\x91\x90aKkV[\x80` \x01\x90a)\xF4\x91\x90aL V[`@Qa*\x02\x92\x91\x90aU.V[`@Q\x80\x91\x03\x90 `@Q` \x01a*\x1C\x93\x92\x91\x90aUUV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x82\x81Q\x81\x10a*EWa*DaK2V[[` \x02` \x01\x01\x81\x81RPP\x80\x80`\x01\x01\x91PPa)_V[Pa*\xD9`@Q\x80`\xA0\x01`@R\x80`r\x81R` \x01aY\xDE`r\x919\x80Q\x90` \x01 \x87\x87\x84`@Q` \x01a*\x95\x91\x90aV;V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a*\xBE\x94\x93\x92\x91\x90aVQV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a3|V[\x91PP\x94\x93PPPPV[_\x80a+3\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa3\x95V[\x90Pa+>\x81a3\xBFV[\x80\x91PP\x93\x92PPPV[_\x80s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xB4r+\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\xA8W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\xCC\x91\x90aV\xA8V[\x90P\x80\x83\x10\x15\x91PP\x91\x90PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a,\x87WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a,na4\x8FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a,\xBEW`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\x1DW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-A\x91\x90aI\xBEV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a-\xB0W3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a-\xA7\x91\x90aI\xE9V[`@Q\x80\x91\x03\x90\xFD[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a.\x1BWP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\x18\x91\x90aV\xFDV[`\x01[a.\\W\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.S\x91\x90aI\xE9V[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14a.\xC2W\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.\xB9\x91\x90aB%V[`@Q\x80\x91\x03\x90\xFD[a.\xCC\x83\x83a4\xE2V[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a/VW`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_a/\xA9`@Q\x80``\x01`@R\x80`,\x81R` \x01aY\xB2`,\x919\x80Q\x90` \x01 \x83`@Q` \x01a/\x8E\x92\x91\x90aW(V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a3|V[\x90P\x91\x90PV[_a0-`@Q\x80`\x80\x01`@R\x80`F\x81R` \x01aYl`F\x919\x80Q\x90` \x01 \x86\x86\x86\x86`@Q` \x01a/\xE9\x92\x91\x90aU.V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a0\x12\x94\x93\x92\x91\x90aVQV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a3|V[\x90P\x94\x93PPPPV[_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x90P\x90V[``_a0ia07V[\x90P\x80`\x02\x01\x80Ta0z\x90aL\xB9V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta0\xA6\x90aL\xB9V[\x80\x15a0\xF1W\x80`\x1F\x10a0\xC8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a0\xF1V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a0\xD4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[``_a1\x07a07V[\x90P\x80`\x03\x01\x80Ta1\x18\x90aL\xB9V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta1D\x90aL\xB9V[\x80\x15a1\x8FW\x80`\x1F\x10a1fWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a1\x8FV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a1rW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[_\x80_\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a1\xF6Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81a1\xECWa1\xEBaT\xA8V[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a23Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81a2)Wa2(aT\xA8V[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10a2bWf#\x86\xF2o\xC1\0\0\x83\x81a2XWa2WaT\xA8V[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10a2\x8BWc\x05\xF5\xE1\0\x83\x81a2\x81Wa2\x80aT\xA8V[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10a2\xB0Wa'\x10\x83\x81a2\xA6Wa2\xA5aT\xA8V[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10a2\xD3W`d\x83\x81a2\xC9Wa2\xC8aT\xA8V[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10a2\xE2W`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[a2\xF3a5TV[a3)W`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a33a2\xEBV[_a3<a07V[\x90P\x82\x81`\x02\x01\x90\x81a3O\x91\x90aW\xA7V[P\x81\x81`\x03\x01\x90\x81a3a\x91\x90aW\xA7V[P_\x80\x1B\x81_\x01\x81\x90UP_\x80\x1B\x81`\x01\x01\x81\x90UPPPPV[_a3\x8Ea3\x88a5rV[\x83a5\x80V[\x90P\x91\x90PV[_\x80_\x80a3\xA3\x86\x86a5\xC0V[\x92P\x92P\x92Pa3\xB3\x82\x82a6\x15V[\x82\x93PPPP\x92\x91PPV[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c =\x01\x14\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a4\x0C\x91\x90aI\xE9V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4'W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4K\x91\x90aJ\xE0V[a4\x8CW\x80`@Q\x7F*|n\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a4\x83\x91\x90aI\xE9V[`@Q\x80\x91\x03\x90\xFD[PV[_a4\xBB\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba7wV[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a4\xEB\x82a7\x80V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15a5GWa5A\x82\x82a8IV[Pa5PV[a5Oa8\xC9V[[PPV[_a5]a(\xD0V[_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[_a5{a9\x05V[\x90P\x90V[_`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x02\x82\x01R\x82`\"\x82\x01R`B\x81 \x91PP\x92\x91PPV[_\x80_`A\x84Q\x03a6\0W_\x80_` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90Pa5\xF2\x88\x82\x85\x85a9hV[\x95P\x95P\x95PPPPa6\x0EV[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15a6(Wa6'a>-V[[\x82`\x03\x81\x11\x15a6;Wa6:a>-V[[\x03\x15a7sW`\x01`\x03\x81\x11\x15a6UWa6Ta>-V[[\x82`\x03\x81\x11\x15a6hWa6ga>-V[[\x03a6\x9FW`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15a6\xB3Wa6\xB2a>-V[[\x82`\x03\x81\x11\x15a6\xC6Wa6\xC5a>-V[[\x03a7\nW\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a7\x01\x91\x90aG\x82V[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15a7\x1DWa7\x1Ca>-V[[\x82`\x03\x81\x11\x15a70Wa7/a>-V[[\x03a7rW\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a7i\x91\x90aB%V[`@Q\x80\x91\x03\x90\xFD[[PPV[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a7\xDBW\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a7\xD2\x91\x90aI\xE9V[`@Q\x80\x91\x03\x90\xFD[\x80a8\x07\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba7wV[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qa8r\x91\x90aX\xA6V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a8\xAAW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a8\xAFV[``\x91P[P\x91P\x91Pa8\xBF\x85\x83\x83a:OV[\x92PPP\x92\x91PPV[_4\x11\x15a9\x03W`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa9/a:\xDCV[a97a;RV[F0`@Q` \x01a9M\x95\x94\x93\x92\x91\x90aX\xBCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x80_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15a9\xA4W_`\x03\x85\x92P\x92P\x92Pa:EV[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@Qa9\xC7\x94\x93\x92\x91\x90aY(V[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a9\xE7W=_\x80>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a:8W_`\x01_\x80\x1B\x93P\x93P\x93PPa:EV[\x80_\x80_\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[``\x82a:dWa:_\x82a;\xC9V[a:\xD4V[_\x82Q\x14\x80\x15a:\x8AWP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a:\xCCW\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a:\xC3\x91\x90aI\xE9V[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa:\xD5V[[\x93\x92PPPV[_\x80a:\xE6a07V[\x90P_a:\xF1a0^V[\x90P_\x81Q\x11\x15a;\rW\x80\x80Q\x90` \x01 \x92PPPa;OV[_\x82_\x01T\x90P_\x80\x1B\x81\x14a;(W\x80\x93PPPPa;OV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x80a;\\a07V[\x90P_a;ga0\xFCV[\x90P_\x81Q\x11\x15a;\x83W\x80\x80Q\x90` \x01 \x92PPPa;\xC6V[_\x82`\x01\x01T\x90P_\x80\x1B\x81\x14a;\x9FW\x80\x93PPPPa;\xC6V[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x81Q\x11\x15a;\xDBW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15a<DW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa<)V[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a<i\x82a<\rV[a<s\x81\x85a<\x17V[\x93Pa<\x83\x81\x85` \x86\x01a<'V[a<\x8C\x81a<OV[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra<\xAF\x81\x84a<_V[\x90P\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x81\x90P\x91\x90PV[a<\xDA\x81a<\xC8V[\x81\x14a<\xE4W_\x80\xFD[PV[_\x815\x90Pa<\xF5\x81a<\xD1V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a=\x10Wa=\x0Fa<\xC0V[[_a=\x1D\x84\x82\x85\x01a<\xE7V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a=x\x82a=OV[\x90P\x91\x90PV[a=\x88\x81a=nV[\x82RPPV[_a=\x99\x83\x83a=\x7FV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a=\xBB\x82a=&V[a=\xC5\x81\x85a=0V[\x93Pa=\xD0\x83a=@V[\x80_[\x83\x81\x10\x15a>\0W\x81Qa=\xE7\x88\x82a=\x8EV[\x97Pa=\xF2\x83a=\xA5V[\x92PP`\x01\x81\x01\x90Pa=\xD3V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra>%\x81\x84a=\xB1V[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x02\x81\x10a>kWa>ja>-V[[PV[_\x81\x90Pa>{\x82a>ZV[\x91\x90PV[_a>\x8A\x82a>nV[\x90P\x91\x90PV[a>\x9A\x81a>\x80V[\x82RPPV[_` \x82\x01\x90Pa>\xB3_\x83\x01\x84a>\x91V[\x92\x91PPV[`\x02\x81\x10a>\xC5W_\x80\xFD[PV[_\x815\x90Pa>\xD6\x81a>\xB9V[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a>\xF2Wa>\xF1a<\xC0V[[_a>\xFF\x85\x82\x86\x01a<\xE7V[\x92PP` a?\x10\x85\x82\x86\x01a>\xC8V[\x91PP\x92P\x92\x90PV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12a?;Wa?:a?\x1AV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a?XWa?Wa?\x1EV[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a?tWa?sa?\"V[[\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12a?\x90Wa?\x8Fa?\x1AV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a?\xADWa?\xACa?\x1EV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a?\xC9Wa?\xC8a?\"V[[\x92P\x92\x90PV[_\x80_\x80_``\x86\x88\x03\x12\x15a?\xE9Wa?\xE8a<\xC0V[[_a?\xF6\x88\x82\x89\x01a<\xE7V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a@\x17Wa@\x16a<\xC4V[[a@#\x88\x82\x89\x01a?&V[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a@FWa@Ea<\xC4V[[a@R\x88\x82\x89\x01a?{V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[a@j\x81a=nV[\x81\x14a@tW_\x80\xFD[PV[_\x815\x90Pa@\x85\x81a@aV[\x92\x91PPV[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a@\xC5\x82a<OV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a@\xE4Wa@\xE3a@\x8FV[[\x80`@RPPPV[_a@\xF6a<\xB7V[\x90PaA\x02\x82\x82a@\xBCV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aA!WaA a@\x8FV[[aA*\x82a<OV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_aAWaAR\x84aA\x07V[a@\xEDV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15aAsWaAra@\x8BV[[aA~\x84\x82\x85aA7V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aA\x9AWaA\x99a?\x1AV[[\x815aA\xAA\x84\x82` \x86\x01aAEV[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15aA\xC9WaA\xC8a<\xC0V[[_aA\xD6\x85\x82\x86\x01a@wV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aA\xF7WaA\xF6a<\xC4V[[aB\x03\x85\x82\x86\x01aA\x86V[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[aB\x1F\x81aB\rV[\x82RPPV[_` \x82\x01\x90PaB8_\x83\x01\x84aB\x16V[\x92\x91PPV[_\x80_`@\x84\x86\x03\x12\x15aBUWaBTa<\xC0V[[_aBb\x86\x82\x87\x01a<\xE7V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aB\x83WaB\x82a<\xC4V[[aB\x8F\x86\x82\x87\x01a?{V[\x92P\x92PP\x92P\x92P\x92V[_\x80_\x80_``\x86\x88\x03\x12\x15aB\xB4WaB\xB3a<\xC0V[[_aB\xC1\x88\x82\x89\x01a<\xE7V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aB\xE2WaB\xE1a<\xC4V[[aB\xEE\x88\x82\x89\x01a?{V[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aC\x11WaC\x10a<\xC4V[[aC\x1D\x88\x82\x89\x01a?{V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[_\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[aC`\x81aC,V[\x82RPPV[aCo\x81a<\xC8V[\x82RPPV[aC~\x81a=nV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[aC\xB6\x81a<\xC8V[\x82RPPV[_aC\xC7\x83\x83aC\xADV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aC\xE9\x82aC\x84V[aC\xF3\x81\x85aC\x8EV[\x93PaC\xFE\x83aC\x9EV[\x80_[\x83\x81\x10\x15aD.W\x81QaD\x15\x88\x82aC\xBCV[\x97PaD \x83aC\xD3V[\x92PP`\x01\x81\x01\x90PaD\x01V[P\x85\x93PPPP\x92\x91PPV[_`\xE0\x82\x01\x90PaDN_\x83\x01\x8AaCWV[\x81\x81\x03` \x83\x01RaD`\x81\x89a<_V[\x90P\x81\x81\x03`@\x83\x01RaDt\x81\x88a<_V[\x90PaD\x83``\x83\x01\x87aCfV[aD\x90`\x80\x83\x01\x86aCuV[aD\x9D`\xA0\x83\x01\x85aB\x16V[\x81\x81\x03`\xC0\x83\x01RaD\xAF\x81\x84aC\xDFV[\x90P\x98\x97PPPPPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aE\0\x82a<\rV[aE\n\x81\x85aD\xE6V[\x93PaE\x1A\x81\x85` \x86\x01a<'V[aE#\x81a<OV[\x84\x01\x91PP\x92\x91PPV[_aE9\x83\x83aD\xF6V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aEW\x82aD\xBDV[aEa\x81\x85aD\xC7V[\x93P\x83` \x82\x02\x85\x01aEs\x85aD\xD7V[\x80_[\x85\x81\x10\x15aE\xAEW\x84\x84\x03\x89R\x81QaE\x8F\x85\x82aE.V[\x94PaE\x9A\x83aEAV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaEvV[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[`\x02\x81\x10aE\xFAWaE\xF9a>-V[[PV[_\x81\x90PaF\n\x82aE\xE9V[\x91\x90PV[_aF\x19\x82aE\xFDV[\x90P\x91\x90PV[aF)\x81aF\x0FV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aFS\x82aF/V[aF]\x81\x85aF9V[\x93PaFm\x81\x85` \x86\x01a<'V[aFv\x81a<OV[\x84\x01\x91PP\x92\x91PPV[_`@\x83\x01_\x83\x01QaF\x96_\x86\x01\x82aF V[P` \x83\x01Q\x84\x82\x03` \x86\x01RaF\xAE\x82\x82aFIV[\x91PP\x80\x91PP\x92\x91PPV[_aF\xC6\x83\x83aF\x81V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aF\xE4\x82aE\xC0V[aF\xEE\x81\x85aE\xCAV[\x93P\x83` \x82\x02\x85\x01aG\0\x85aE\xDAV[\x80_[\x85\x81\x10\x15aG;W\x84\x84\x03\x89R\x81QaG\x1C\x85\x82aF\xBBV[\x94PaG'\x83aF\xCEV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaG\x03V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01RaGe\x81\x85aEMV[\x90P\x81\x81\x03` \x83\x01RaGy\x81\x84aF\xDAV[\x90P\x93\x92PPPV[_` \x82\x01\x90PaG\x95_\x83\x01\x84aCfV[\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aG\xB5\x82aF/V[aG\xBF\x81\x85aG\x9BV[\x93PaG\xCF\x81\x85` \x86\x01a<'V[aG\xD8\x81a<OV[\x84\x01\x91PP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01RaG\xFB\x81\x85aEMV[\x90P\x81\x81\x03` \x83\x01RaH\x0F\x81\x84aG\xABV[\x90P\x93\x92PPPV[_` \x82\x84\x03\x12\x15aH-WaH,a<\xC0V[[_aH:\x84\x82\x85\x01a>\xC8V[\x91PP\x92\x91PPV[_\x81\x90P\x92\x91PPV[_aHW\x82a<\rV[aHa\x81\x85aHCV[\x93PaHq\x81\x85` \x86\x01a<'V[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aH\xB1`\x02\x83aHCV[\x91PaH\xBC\x82aH}V[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aH\xFB`\x01\x83aHCV[\x91PaI\x06\x82aH\xC7V[`\x01\x82\x01\x90P\x91\x90PV[_aI\x1C\x82\x87aHMV[\x91PaI'\x82aH\xA5V[\x91PaI3\x82\x86aHMV[\x91PaI>\x82aH\xEFV[\x91PaIJ\x82\x85aHMV[\x91PaIU\x82aH\xEFV[\x91PaIa\x82\x84aHMV[\x91P\x81\x90P\x95\x94PPPPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[aI\x8B\x81aIoV[\x82RPPV[_` \x82\x01\x90PaI\xA4_\x83\x01\x84aI\x82V[\x92\x91PPV[_\x81Q\x90PaI\xB8\x81a@aV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aI\xD3WaI\xD2a<\xC0V[[_aI\xE0\x84\x82\x85\x01aI\xAAV[\x91PP\x92\x91PPV[_` \x82\x01\x90PaI\xFC_\x83\x01\x84aCuV[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_aJ9\x82a<\xC8V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03aJkWaJjaJ\x02V[[`\x01\x82\x01\x90P\x91\x90PV[_``\x82\x01\x90PaJ\x89_\x83\x01\x86aCfV[aJ\x96` \x83\x01\x85aCfV[aJ\xA3`@\x83\x01\x84a>\x91V[\x94\x93PPPPV[_\x81\x15\x15\x90P\x91\x90PV[aJ\xBF\x81aJ\xABV[\x81\x14aJ\xC9W_\x80\xFD[PV[_\x81Q\x90PaJ\xDA\x81aJ\xB6V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aJ\xF5WaJ\xF4a<\xC0V[[_aK\x02\x84\x82\x85\x01aJ\xCCV[\x91PP\x92\x91PPV[_`@\x82\x01\x90PaK\x1E_\x83\x01\x85aCfV[aK+` \x83\x01\x84aCuV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x825`\x01`@\x03\x836\x03\x03\x81\x12aK\x86WaK\x85aK_V[[\x80\x83\x01\x91PP\x92\x91PPV[`\x02\x81\x10aK\x9EW_\x80\xFD[PV[_\x815aK\xAD\x81aK\x92V[\x80\x91PP\x91\x90PV[_\x81_\x1B\x90P\x91\x90PV[_`\xFFaK\xCD\x84aK\xB6V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_aK\xED\x82aE\xFDV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[aL\x06\x82aK\xE3V[aL\x19aL\x12\x82aK\xF4V[\x83TaK\xC1V[\x82UPPPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aL<WaL;aK_V[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aL^WaL]aKcV[[` \x83\x01\x92P`\x01\x82\x026\x03\x83\x13\x15aLzWaLyaKgV[[P\x92P\x92\x90PV[_\x82\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80aL\xD0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aL\xE3WaL\xE2aL\x8CV[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02aME\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82aM\nV[aMO\x86\x83aM\nV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_aM\x8AaM\x85aM\x80\x84a<\xC8V[aMgV[a<\xC8V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[aM\xA3\x83aMpV[aM\xB7aM\xAF\x82aM\x91V[\x84\x84TaM\x16V[\x82UPPPPV[_\x90V[aM\xCBaM\xBFV[aM\xD6\x81\x84\x84aM\x9AV[PPPV[[\x81\x81\x10\x15aM\xF9WaM\xEE_\x82aM\xC3V[`\x01\x81\x01\x90PaM\xDCV[PPV[`\x1F\x82\x11\x15aN>WaN\x0F\x81aL\xE9V[aN\x18\x84aL\xFBV[\x81\x01` \x85\x10\x15aN'W\x81\x90P[aN;aN3\x85aL\xFBV[\x83\x01\x82aM\xDBV[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_aN^_\x19\x84`\x08\x02aNCV[\x19\x80\x83\x16\x91PP\x92\x91PPV[_aNv\x83\x83aNOV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[aN\x90\x83\x83aL\x82V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aN\xA9WaN\xA8a@\x8FV[[aN\xB3\x82TaL\xB9V[aN\xBE\x82\x82\x85aM\xFDV[_`\x1F\x83\x11`\x01\x81\x14aN\xEBW_\x84\x15aN\xD9W\x82\x87\x015\x90P[aN\xE3\x85\x82aNkV[\x86UPaOJV[`\x1F\x19\x84\x16aN\xF9\x86aL\xE9V[_[\x82\x81\x10\x15aO W\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PaN\xFBV[\x86\x83\x10\x15aO=W\x84\x89\x015aO9`\x1F\x89\x16\x82aNOV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[aO^\x83\x83\x83aN\x86V[PPPV[_\x81\x01_\x83\x01\x80aOs\x81aK\xA1V[\x90PaO\x7F\x81\x84aK\xFDV[PPP`\x01\x81\x01` \x83\x01aO\x94\x81\x85aL V[aO\x9F\x81\x83\x86aOSV[PPPPPPV[aO\xB1\x82\x82aOcV[PPV[_\x80\xFD[_\x80\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aO\xD7WaO\xD6a@\x8FV[[aO\xE0\x82a<OV[\x90P` \x81\x01\x90P\x91\x90PV[_aO\xFFaO\xFA\x84aO\xBDV[a@\xEDV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15aP\x1BWaP\x1Aa@\x8BV[[aP&\x84\x82\x85a<'V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aPBWaPAa?\x1AV[[\x81QaPR\x84\x82` \x86\x01aO\xEDV[\x91PP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15aPpWaPoaO\xB5V[[aPz`\x80a@\xEDV[\x90P_aP\x89\x84\x82\x85\x01aI\xAAV[_\x83\x01RP` aP\x9C\x84\x82\x85\x01aI\xAAV[` \x83\x01RP`@\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aP\xC0WaP\xBFaO\xB9V[[aP\xCC\x84\x82\x85\x01aP.V[`@\x83\x01RP``\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aP\xF0WaP\xEFaO\xB9V[[aP\xFC\x84\x82\x85\x01aP.V[``\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15aQ\x1DWaQ\x1Ca<\xC0V[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aQ:WaQ9a<\xC4V[[aQF\x84\x82\x85\x01aP[V[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[_\x815\x90PaQf\x81aK\x92V[\x92\x91PPV[_aQz` \x84\x01\x84aQXV[\x90P\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aQ\xAAWaQ\xA9aQ\x8AV[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aQ\xD2WaQ\xD1aQ\x82V[[`\x01\x82\x026\x03\x83\x13\x15aQ\xE8WaQ\xE7aQ\x86V[[P\x92P\x92\x90PV[_aQ\xFB\x83\x85aF9V[\x93PaR\x08\x83\x85\x84aA7V[aR\x11\x83a<OV[\x84\x01\x90P\x93\x92PPPV[_`@\x83\x01aR-_\x84\x01\x84aQlV[aR9_\x86\x01\x82aF V[PaRG` \x84\x01\x84aQ\x8EV[\x85\x83\x03` \x87\x01RaRZ\x83\x82\x84aQ\xF0V[\x92PPP\x80\x91PP\x92\x91PPV[_aRs\x83\x83aR\x1CV[\x90P\x92\x91PPV[_\x825`\x01`@\x03\x836\x03\x03\x81\x12aR\x96WaR\x95aQ\x8AV[[\x82\x81\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aR\xB9\x83\x85aE\xCAV[\x93P\x83` \x84\x02\x85\x01aR\xCB\x84aQOV[\x80_[\x87\x81\x10\x15aS\x0EW\x84\x84\x03\x89RaR\xE5\x82\x84aR{V[aR\xEF\x85\x82aRhV[\x94PaR\xFA\x83aR\xA2V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaR\xCEV[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_``\x82\x01\x90PaS3_\x83\x01\x87aCfV[\x81\x81\x03` \x83\x01RaSE\x81\x86aEMV[\x90P\x81\x81\x03`@\x83\x01RaSZ\x81\x84\x86aR\xAEV[\x90P\x95\x94PPPPPV[_`@\x82\x01\x90PaSx_\x83\x01\x85aCfV[aS\x85` \x83\x01\x84aCfV[\x93\x92PPPV[_aS\x97\x83\x85aG\x9BV[\x93PaS\xA4\x83\x85\x84aA7V[aS\xAD\x83a<OV[\x84\x01\x90P\x93\x92PPPV[_``\x82\x01\x90PaS\xCB_\x83\x01\x87aCfV[\x81\x81\x03` \x83\x01RaS\xDD\x81\x86aEMV[\x90P\x81\x81\x03`@\x83\x01RaS\xF2\x81\x84\x86aS\x8CV[\x90P\x95\x94PPPPPV[\x7FEIP712: Uninitialized\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aT1`\x15\x83a<\x17V[\x91PaT<\x82aS\xFDV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaT^\x81aT%V[\x90P\x91\x90PV[_`\x80\x82\x01\x90PaTx_\x83\x01\x87aCfV[aT\x85` \x83\x01\x86aCfV[aT\x92`@\x83\x01\x85aCfV[aT\x9F``\x83\x01\x84a>\x91V[\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aT\xEAWaT\xE9a<\xC0V[[_aT\xF7\x84\x82\x85\x01aQXV[\x91PP\x92\x91PPV[_\x81\x90P\x92\x91PPV[_aU\x15\x83\x85aU\0V[\x93PaU\"\x83\x85\x84aA7V[\x82\x84\x01\x90P\x93\x92PPPV[_aU:\x82\x84\x86aU\nV[\x91P\x81\x90P\x93\x92PPPV[aUO\x81aF\x0FV[\x82RPPV[_``\x82\x01\x90PaUh_\x83\x01\x86aB\x16V[aUu` \x83\x01\x85aUFV[aU\x82`@\x83\x01\x84aB\x16V[\x94\x93PPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[aU\xB6\x81aB\rV[\x82RPPV[_aU\xC7\x83\x83aU\xADV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aU\xE9\x82aU\x8AV[aU\xF3\x81\x85aU\x94V[\x93PaU\xFE\x83aU\x9EV[\x80_[\x83\x81\x10\x15aV.W\x81QaV\x15\x88\x82aU\xBCV[\x97PaV \x83aU\xD3V[\x92PP`\x01\x81\x01\x90PaV\x01V[P\x85\x93PPPP\x92\x91PPV[_aVF\x82\x84aU\xDFV[\x91P\x81\x90P\x92\x91PPV[_`\x80\x82\x01\x90PaVd_\x83\x01\x87aB\x16V[aVq` \x83\x01\x86aCfV[aV~`@\x83\x01\x85aCfV[aV\x8B``\x83\x01\x84aB\x16V[\x95\x94PPPPPV[_\x81Q\x90PaV\xA2\x81a<\xD1V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aV\xBDWaV\xBCa<\xC0V[[_aV\xCA\x84\x82\x85\x01aV\x94V[\x91PP\x92\x91PPV[aV\xDC\x81aB\rV[\x81\x14aV\xE6W_\x80\xFD[PV[_\x81Q\x90PaV\xF7\x81aV\xD3V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aW\x12WaW\x11a<\xC0V[[_aW\x1F\x84\x82\x85\x01aV\xE9V[\x91PP\x92\x91PPV[_`@\x82\x01\x90PaW;_\x83\x01\x85aB\x16V[aWH` \x83\x01\x84aCfV[\x93\x92PPPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[`\x1F\x82\x11\x15aW\xA2WaWs\x81aWOV[aW|\x84aL\xFBV[\x81\x01` \x85\x10\x15aW\x8BW\x81\x90P[aW\x9FaW\x97\x85aL\xFBV[\x83\x01\x82aM\xDBV[PP[PPPV[aW\xB0\x82a<\rV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aW\xC9WaW\xC8a@\x8FV[[aW\xD3\x82TaL\xB9V[aW\xDE\x82\x82\x85aWaV[_` \x90P`\x1F\x83\x11`\x01\x81\x14aX\x0FW_\x84\x15aW\xFDW\x82\x87\x01Q\x90P[aX\x07\x85\x82aNkV[\x86UPaXnV[`\x1F\x19\x84\x16aX\x1D\x86aWOV[_[\x82\x81\x10\x15aXDW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PaX\x1FV[\x86\x83\x10\x15aXaW\x84\x89\x01QaX]`\x1F\x89\x16\x82aNOV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_aX\x80\x82aF/V[aX\x8A\x81\x85aU\0V[\x93PaX\x9A\x81\x85` \x86\x01a<'V[\x80\x84\x01\x91PP\x92\x91PPV[_aX\xB1\x82\x84aXvV[\x91P\x81\x90P\x92\x91PPV[_`\xA0\x82\x01\x90PaX\xCF_\x83\x01\x88aB\x16V[aX\xDC` \x83\x01\x87aB\x16V[aX\xE9`@\x83\x01\x86aB\x16V[aX\xF6``\x83\x01\x85aCfV[aY\x03`\x80\x83\x01\x84aCuV[\x96\x95PPPPPPV[_`\xFF\x82\x16\x90P\x91\x90PV[aY\"\x81aY\rV[\x82RPPV[_`\x80\x82\x01\x90PaY;_\x83\x01\x87aB\x16V[aYH` \x83\x01\x86aY\x19V[aYU`@\x83\x01\x85aB\x16V[aYb``\x83\x01\x84aB\x16V[\x95\x94PPPPPV\xFECrsgenVerification(uint256 crsId,uint256 maxBitLength,bytes crsDigest)PrepKeygenVerification(uint256 prepKeygenId)KeygenVerification(uint256 prepKeygenId,uint256 keyId,KeyDigest[] keyDigests)KeyDigest(uint8 keyType,bytes digest)KeyDigest(uint8 keyType,bytes digest)",
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
    /**Event with signature `RetryKeygenReshare(uint256,uint256,uint256,uint8)` and selector `0x7cf0be262c7aec97bcb24f48cb25550722e8dd371d79c42f6698f3bb27005731`.
```solidity
event RetryKeygenReshare(uint256 prepKeygenId, uint256 keyId, uint256 epochId, IKMSGeneration.ParamsType paramsType);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RetryKeygenReshare {
        #[allow(missing_docs)]
        pub prepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for RetryKeygenReshare {
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
            const SIGNATURE: &'static str = "RetryKeygenReshare(uint256,uint256,uint256,uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                124u8, 240u8, 190u8, 38u8, 44u8, 122u8, 236u8, 151u8, 188u8, 178u8, 79u8,
                72u8, 203u8, 37u8, 85u8, 7u8, 34u8, 232u8, 221u8, 55u8, 29u8, 121u8,
                196u8, 47u8, 102u8, 152u8, 243u8, 187u8, 39u8, 0u8, 87u8, 49u8,
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
                    epochId: data.2,
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
        impl alloy_sol_types::private::IntoLogData for RetryKeygenReshare {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RetryKeygenReshare> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RetryKeygenReshare) -> alloy_sol_types::private::LogData {
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
    /**Function with signature `retryKeygenReshare(uint256)` and selector `0xd2c08a91`.
```solidity
function retryKeygenReshare(uint256 keyId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct retryKeygenReshareCall {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`retryKeygenReshare(uint256)`](retryKeygenReshareCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct retryKeygenReshareReturn {}
    #[allow(
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
            impl ::core::convert::From<retryKeygenReshareCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: retryKeygenReshareCall) -> Self {
                    (value.keyId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for retryKeygenReshareCall {
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
            impl ::core::convert::From<retryKeygenReshareReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: retryKeygenReshareReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for retryKeygenReshareReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl retryKeygenReshareReturn {
            fn _tokenize(
                &self,
            ) -> <retryKeygenReshareCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for retryKeygenReshareCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = retryKeygenReshareReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "retryKeygenReshare(uint256)";
            const SELECTOR: [u8; 4] = [210u8, 192u8, 138u8, 145u8];
            #[inline]
            fn new<'a>(
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
                retryKeygenReshareReturn::_tokenize(ret)
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
        reinitializeV2(reinitializeV2Call),
        #[allow(missing_docs)]
        retryKeygenReshare(retryKeygenReshareCall),
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
            [196u8, 17u8, 88u8, 116u8],
            [197u8, 91u8, 135u8, 36u8],
            [202u8, 163u8, 103u8, 219u8],
            [210u8, 192u8, 138u8, 145u8],
            [213u8, 47u8, 16u8, 235u8],
        ];
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
                Self::reinitializeV2(_) => {
                    <reinitializeV2Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::retryKeygenReshare(_) => {
                    <retryKeygenReshareCall as alloy_sol_types::SolCall>::SELECTOR
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
                    fn keygen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <keygenCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(KMSGenerationCalls::keygen)
                    }
                    keygen
                },
                {
                    fn retryKeygenReshare(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <retryKeygenReshareCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::retryKeygenReshare)
                    }
                    retryKeygenReshare
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
                    fn retryKeygenReshare(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <retryKeygenReshareCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::retryKeygenReshare)
                    }
                    retryKeygenReshare
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
                Self::reinitializeV2(inner) => {
                    <reinitializeV2Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::retryKeygenReshare(inner) => {
                    <retryKeygenReshareCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::reinitializeV2(inner) => {
                    <reinitializeV2Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::retryKeygenReshare(inner) => {
                    <retryKeygenReshareCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
        AddressEmptyCode(AddressEmptyCode),
        #[allow(missing_docs)]
        CrsNotGenerated(CrsNotGenerated),
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
        FailedCall(FailedCall),
        #[allow(missing_docs)]
        HostChainNotRegistered(HostChainNotRegistered),
        #[allow(missing_docs)]
        InvalidInitialization(InvalidInitialization),
        #[allow(missing_docs)]
        KeyNotGenerated(KeyNotGenerated),
        #[allow(missing_docs)]
        KmsAlreadySignedForCrsgen(KmsAlreadySignedForCrsgen),
        #[allow(missing_docs)]
        KmsAlreadySignedForKeygen(KmsAlreadySignedForKeygen),
        #[allow(missing_docs)]
        KmsAlreadySignedForPrepKeygen(KmsAlreadySignedForPrepKeygen),
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
            [14u8, 86u8, 207u8, 61u8],
            [38u8, 205u8, 117u8, 220u8],
            [42u8, 124u8, 110u8, 246u8],
            [51u8, 202u8, 31u8, 227u8],
            [57u8, 22u8, 114u8, 167u8],
            [76u8, 156u8, 140u8, 227u8],
            [82u8, 215u8, 37u8, 245u8],
            [111u8, 79u8, 115u8, 31u8],
            [132u8, 222u8, 19u8, 49u8],
            [152u8, 251u8, 149u8, 125u8],
            [153u8, 150u8, 179u8, 21u8],
            [170u8, 29u8, 73u8, 164u8],
            [174u8, 232u8, 99u8, 35u8],
            [179u8, 152u8, 151u8, 159u8],
            [182u8, 103u8, 156u8, 59u8],
            [214u8, 189u8, 162u8, 117u8],
            [215u8, 139u8, 206u8, 12u8],
            [215u8, 230u8, 188u8, 248u8],
            [218u8, 50u8, 208u8, 15u8],
            [224u8, 124u8, 141u8, 186u8],
            [246u8, 69u8, 238u8, 223u8],
            [249u8, 36u8, 160u8, 207u8],
            [249u8, 46u8, 232u8, 169u8],
            [252u8, 230u8, 152u8, 247u8],
            [252u8, 245u8, 166u8, 233u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for KMSGenerationErrors {
        const NAME: &'static str = "KMSGenerationErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 25usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AddressEmptyCode(_) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CrsNotGenerated(_) => {
                    <CrsNotGenerated as alloy_sol_types::SolError>::SELECTOR
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
                Self::KmsAlreadySignedForCrsgen(_) => {
                    <KmsAlreadySignedForCrsgen as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsAlreadySignedForKeygen(_) => {
                    <KmsAlreadySignedForKeygen as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsAlreadySignedForPrepKeygen(_) => {
                    <KmsAlreadySignedForPrepKeygen as alloy_sol_types::SolError>::SELECTOR
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
                Self::CrsNotGenerated(inner) => {
                    <CrsNotGenerated as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::CrsNotGenerated(inner) => {
                    <CrsNotGenerated as alloy_sol_types::SolError>::abi_encode_raw(
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
        ActivateCrs(ActivateCrs),
        #[allow(missing_docs)]
        ActivateKey(ActivateKey),
        #[allow(missing_docs)]
        CrsgenRequest(CrsgenRequest),
        #[allow(missing_docs)]
        EIP712DomainChanged(EIP712DomainChanged),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        KeygenRequest(KeygenRequest),
        #[allow(missing_docs)]
        PRSSInit(PRSSInit),
        #[allow(missing_docs)]
        PrepKeygenRequest(PrepKeygenRequest),
        #[allow(missing_docs)]
        RetryKeygenReshare(RetryKeygenReshare),
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
                34u8, 88u8, 183u8, 63u8, 174u8, 211u8, 63u8, 178u8, 226u8, 234u8, 69u8,
                68u8, 3u8, 190u8, 249u8, 116u8, 146u8, 12u8, 175u8, 104u8, 42u8, 179u8,
                167u8, 35u8, 72u8, 79u8, 207u8, 103u8, 85u8, 59u8, 22u8, 162u8,
            ],
            [
                63u8, 3u8, 143u8, 111u8, 136u8, 203u8, 48u8, 49u8, 183u8, 113u8, 133u8,
                136u8, 64u8, 58u8, 46u8, 194u8, 32u8, 87u8, 106u8, 134u8, 139u8, 224u8,
                125u8, 222u8, 76u8, 2u8, 184u8, 70u8, 202u8, 53u8, 46u8, 245u8,
            ],
            [
                120u8, 177u8, 121u8, 23u8, 109u8, 31u8, 25u8, 215u8, 194u8, 142u8, 128u8,
                130u8, 61u8, 235u8, 162u8, 98u8, 77u8, 162u8, 202u8, 46u8, 198u8, 75u8,
                23u8, 1u8, 243u8, 99u8, 42u8, 135u8, 201u8, 174u8, 220u8, 146u8,
            ],
            [
                124u8, 240u8, 190u8, 38u8, 44u8, 122u8, 236u8, 151u8, 188u8, 178u8, 79u8,
                72u8, 203u8, 37u8, 85u8, 7u8, 34u8, 232u8, 221u8, 55u8, 29u8, 121u8,
                196u8, 47u8, 102u8, 152u8, 243u8, 187u8, 39u8, 0u8, 87u8, 49u8,
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
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for KMSGenerationEvents {
        const NAME: &'static str = "KMSGenerationEvents";
        const COUNT: usize = 10usize;
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
                    <RetryKeygenReshare as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RetryKeygenReshare as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RetryKeygenReshare)
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
                Self::EIP712DomainChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::KeygenRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PRSSInit(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PrepKeygenRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RetryKeygenReshare(inner) => {
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
                Self::EIP712DomainChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::KeygenRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PRSSInit(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PrepKeygenRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RetryKeygenReshare(inner) => {
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
        ///Creates a new call builder for the [`reinitializeV2`] function.
        pub fn reinitializeV2(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, reinitializeV2Call, N> {
            self.call_builder(&reinitializeV2Call)
        }
        ///Creates a new call builder for the [`retryKeygenReshare`] function.
        pub fn retryKeygenReshare(
            &self,
            keyId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, retryKeygenReshareCall, N> {
            self.call_builder(&retryKeygenReshareCall { keyId })
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
        ///Creates a new event filter for the [`RetryKeygenReshare`] event.
        pub fn RetryKeygenReshare_filter(
            &self,
        ) -> alloy_contract::Event<&P, RetryKeygenReshare, N> {
            self.event_filter::<RetryKeygenReshare>()
        }
        ///Creates a new event filter for the [`Upgraded`] event.
        pub fn Upgraded_filter(&self) -> alloy_contract::Event<&P, Upgraded, N> {
            self.event_filter::<Upgraded>()
        }
    }
}
