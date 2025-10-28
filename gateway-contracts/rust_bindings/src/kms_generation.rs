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
    event PrepKeygenRequest(uint256 prepKeygenId, uint256 epochId, IKMSGeneration.ParamsType paramsType);
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
    ///0x60a06040523073ffffffffffffffffffffffffffffffffffffffff1660809073ffffffffffffffffffffffffffffffffffffffff1681525034801562000043575f80fd5b50620000546200005a60201b60201c565b620001c4565b5f6200006b6200015e60201b60201c565b9050805f0160089054906101000a900460ff1615620000b6576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b67ffffffffffffffff8016815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff16146200015b5767ffffffffffffffff815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d267ffffffffffffffff604051620001529190620001a9565b60405180910390a15b50565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b5f67ffffffffffffffff82169050919050565b620001a38162000185565b82525050565b5f602082019050620001be5f83018462000198565b92915050565b608051615514620001eb5f395f81816126be0152818161271301526129b501526155145ff3fe608060405260043610610108575f3560e01c8063589adb0e11610094578063ad3cb1cc11610063578063ad3cb1cc14610353578063baff211e1461037d578063c55b8724146103a7578063caa367db146103e4578063d52f10eb1461040c57610108565b8063589adb0e1461029657806362978787146102be57806384b0196e146102e6578063936608ae1461031657610108565b80633c02f834116100db5780633c02f834146101c457806345af261b146101ec5780634610ffe8146102285780634f1ef2861461025057806352d1902d1461026c57610108565b80630d8e6e2c1461010c57806316c713d91461013657806319f4f6321461017257806339f73810146101ae575b5f80fd5b348015610117575f80fd5b50610120610436565b60405161012d9190613779565b60405180910390f35b348015610141575f80fd5b5061015c600480360381019061015791906137dd565b6104b1565b60405161016991906138ef565b60405180910390f35b34801561017d575f80fd5b50610198600480360381019061019391906137dd565b610582565b6040516101a59190613982565b60405180910390f35b3480156101b9575f80fd5b506101c261062f565b005b3480156101cf575f80fd5b506101ea60048036038101906101e591906139be565b61087e565b005b3480156101f7575f80fd5b50610212600480360381019061020d91906137dd565b610a2d565b60405161021f9190613982565b60405180910390f35b348015610233575f80fd5b5061024e60048036038101906102499190613ab2565b610ac2565b005b61026a60048036038101906102659190613c95565b611007565b005b348015610277575f80fd5b50610280611026565b60405161028d9190613d07565b60405180910390f35b3480156102a1575f80fd5b506102bc60048036038101906102b79190613d20565b611057565b005b3480156102c9575f80fd5b506102e460048036038101906102df9190613d7d565b6113a9565b005b3480156102f1575f80fd5b506102fa61188a565b60405161030d9796959493929190613f1d565b60405180910390f35b348015610321575f80fd5b5061033c600480360381019061033791906137dd565b611993565b60405161034a92919061422f565b60405180910390f35b34801561035e575f80fd5b50610367611d3a565b6040516103749190613779565b60405180910390f35b348015610388575f80fd5b50610391611d73565b60405161039e9190614264565b60405180910390f35b3480156103b2575f80fd5b506103cd60048036038101906103c891906137dd565b611d8a565b6040516103db9291906142c5565b60405180910390f35b3480156103ef575f80fd5b5061040a600480360381019061040591906142fa565b61209c565b005b348015610417575f80fd5b50610420612286565b60405161042d9190614264565b60405180910390f35b60606040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506104775f61229d565b610481600161229d565b61048a5f61229d565b60405160200161049d94939291906143f3565b604051602081830303815290604052905090565b60605f6104bc612367565b90505f816003015f8581526020019081526020015f20549050816002015f8581526020019081526020015f205f8281526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561057457602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001906001019080831161052b575b505050505092505050919050565b5f8061058c612367565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff166105ef57826040517f84de13310000000000000000000000000000000000000000000000000000000081526004016105e69190614264565b60405180910390fd5b5f816006015f8581526020019081526020015f2054905081600d015f8281526020019081526020015f205f9054906101000a900460ff1692505050919050565b600161063961238e565b67ffffffffffffffff161461067a576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60025f6106856123b2565b9050805f0160089054906101000a900460ff16806106cd57508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15610704576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055506107bd6040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506040518060400160405280600181526020017f31000000000000000000000000000000000000000000000000000000000000008152506123d9565b5f6107c6612367565b905060f8600360058111156107de576107dd61390f565b5b901b816004018190555060f8600460058111156107fe576107fd61390f565b5b901b816005018190555060f860058081111561081d5761081c61390f565b5b901b8160090181905550505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2826040516108729190614473565b60405180910390a15050565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156108db573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108ff91906144a0565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461096e57336040517f0e56cf3d00000000000000000000000000000000000000000000000000000000815260040161096591906144cb565b60405180910390fd5b5f610977612367565b9050806009015f81548092919061098d90614511565b91905055505f816009015490508382600a015f8381526020019081526020015f20819055508282600d015f8381526020019081526020015f205f6101000a81548160ff021916908360018111156109e7576109e661390f565b5b02179055507f3f038f6f88cb3031b7718588403a2ec220576a868be07dde4c02b846ca352ef5818585604051610a1f93929190614558565b60405180910390a150505050565b5f80610a37612367565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff16610a9a57826040517fda32d00f000000000000000000000000000000000000000000000000000000008152600401610a919190614264565b60405180910390fd5b80600d015f8481526020019081526020015f205f9054906101000a900460ff16915050919050565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663e5275eaf336040518263ffffffff1660e01b8152600401610b0f91906144cb565b602060405180830381865afa158015610b2a573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b4e91906145c2565b610b8f57336040517faee86323000000000000000000000000000000000000000000000000000000008152600401610b8691906144cb565b60405180910390fd5b5f610b98612367565b90505f816006015f8881526020019081526020015f205490505f610bbe828989896123ef565b90505f610bcc8287876125c6565b9050835f015f8a81526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615610c6c5788816040517f98fb957d000000000000000000000000000000000000000000000000000000008152600401610c639291906145ed565b60405180910390fd5b6001845f015f8b81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f846002015f8b81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505f81805490509050856001015f8c81526020019081526020015f205f9054906101000a900460ff16158015610d8f5750610d8e8161262b565b5b15610ffa576001866001015f8d81526020019081526020015f205f6101000a81548160ff0219169083151502179055505f5b8a8a9050811015610e4557866007015f8d81526020019081526020015f208b8b83818110610df257610df1614614565b5b9050602002810190610e04919061464d565b908060018154018082558091505060019003905f5260205f2090600202015f909190919091508181610e369190614a89565b50508080600101915050610dc1565b5083866003015f8d81526020019081526020015f20819055508a86600801819055505f8167ffffffffffffffff811115610e8257610e81613b71565b5b604051908082528060200260200182016040528015610eb557816020015b6060815260200190600190039081610ea05790505b5090505f5b82811015610fba5773a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663e3b2a874858381548110610f0557610f04614614565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b8152600401610f4991906144cb565b5f60405180830381865afa158015610f63573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f82011682018060405250810190610f8b9190614bea565b60600151828281518110610fa257610fa1614614565b5b60200260200101819052508080600101915050610eba565b507feb85c26dbcad46b80a68a0f24cce7c2c90f0a1faded84184138839fc9e80a25b8c828d8d604051610ff09493929190614e02565b60405180910390a1505b5050505050505050505050565b61100f6126bc565b611018826127a2565b6110228282612895565b5050565b5f61102f6129b3565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663e5275eaf336040518263ffffffff1660e01b81526004016110a491906144cb565b602060405180830381865afa1580156110bf573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110e391906145c2565b61112457336040517faee8632300000000000000000000000000000000000000000000000000000000815260040161111b91906144cb565b60405180910390fd5b5f61112d612367565b90505f61113985612a3a565b90505f6111478286866125c6565b9050825f015f8781526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16156111e75785816040517f33ca1fe30000000000000000000000000000000000000000000000000000000081526004016111de9291906145ed565b60405180910390fd5b6001835f015f8881526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f836002015f8881526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550836001015f8881526020019081526020015f205f9054906101000a900460ff161580156113065750611305818054905061262b565b5b156113a0576001846001015f8981526020019081526020015f205f6101000a81548160ff02191690831515021790555082846003015f8981526020019081526020015f20819055505f846006015f8981526020019081526020015f205490507f78b179176d1f19d7c28e80823deba2624da2ca2ec64b1701f3632a87c9aedc928882604051611396929190614e47565b60405180910390a1505b50505050505050565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663e5275eaf336040518263ffffffff1660e01b81526004016113f691906144cb565b602060405180830381865afa158015611411573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061143591906145c2565b61147657336040517faee8632300000000000000000000000000000000000000000000000000000000815260040161146d91906144cb565b60405180910390fd5b5f61147f612367565b90505f81600a015f8881526020019081526020015f205490505f6114a588838989612a92565b90505f6114b38287876125c6565b9050835f015f8a81526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16156115535788816040517ffcf5a6e900000000000000000000000000000000000000000000000000000000815260040161154a9291906145ed565b60405180910390fd5b6001845f015f8b81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f846002015f8b81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505f81805490509050856001015f8c81526020019081526020015f205f9054906101000a900460ff1615801561167657506116758161262b565b5b1561187d576001866001015f8d81526020019081526020015f205f6101000a81548160ff021916908315150217905550898987600b015f8e81526020019081526020015f2091826116c8929190614968565b5083866003015f8d81526020019081526020015f20819055508a86600c01819055505f8167ffffffffffffffff81111561170557611704613b71565b5b60405190808252806020026020018201604052801561173857816020015b60608152602001906001900390816117235790505b5090505f5b8281101561183d5773a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663e3b2a87485838154811061178857611787614614565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b81526004016117cc91906144cb565b5f60405180830381865afa1580156117e6573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f8201168201806040525081019061180e9190614bea565b6060015182828151811061182557611824614614565b5b6020026020010181905250808060010191505061173d565b507f2258b73faed33fb2e2ea454403bef974920caf682ab3a723484fcf67553b16a28c828d8d6040516118739493929190614e9a565b60405180910390a1505b5050505050505050505050565b5f6060805f805f60605f61189c612b19565b90505f801b815f01541480156118b757505f801b8160010154145b6118f6576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016118ed90614f29565b60405180910390fd5b6118fe612b40565b611906612bde565b46305f801b5f67ffffffffffffffff81111561192557611924613b71565b5b6040519080825280602002602001820160405280156119535781602001602082028036833780820191505090505b507f0f0000000000000000000000000000000000000000000000000000000000000095949392919097509750975097509750975097505090919293949596565b6060805f61199f612367565b9050806001015f8581526020019081526020015f205f9054906101000a900460ff16611a0257836040517f84de13310000000000000000000000000000000000000000000000000000000081526004016119f99190614264565b60405180910390fd5b5f816003015f8681526020019081526020015f205490505f826002015f8781526020019081526020015f205f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020018280548015611ab957602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311611a70575b505050505090505f815190505f8167ffffffffffffffff811115611ae057611adf613b71565b5b604051908082528060200260200182016040528015611b1357816020015b6060815260200190600190039081611afe5790505b5090505f5b82811015611bf85773a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663e3b2a874858381518110611b6357611b62614614565b5b60200260200101516040518263ffffffff1660e01b8152600401611b8791906144cb565b5f60405180830381865afa158015611ba1573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f82011682018060405250810190611bc99190614bea565b60600151828281518110611be057611bdf614614565b5b60200260200101819052508080600101915050611b18565b5080856007015f8a81526020019081526020015f2080805480602002602001604051908101604052809291908181526020015f905b82821015611d26578382905f5260205f2090600202016040518060400160405290815f82015f9054906101000a900460ff166001811115611c7157611c7061390f565b5b6001811115611c8357611c8261390f565b5b8152602001600182018054611c979061479b565b80601f0160208091040260200160405190810160405280929190818152602001828054611cc39061479b565b8015611d0e5780601f10611ce557610100808354040283529160200191611d0e565b820191905f5260205f20905b815481529060010190602001808311611cf157829003601f168201915b50505050508152505081526020019060010190611c2d565b505050509050965096505050505050915091565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b5f80611d7d612367565b905080600c015491505090565b6060805f611d96612367565b9050806001015f8581526020019081526020015f205f9054906101000a900460ff16611df957836040517fda32d00f000000000000000000000000000000000000000000000000000000008152600401611df09190614264565b60405180910390fd5b5f816003015f8681526020019081526020015f205490505f826002015f8781526020019081526020015f205f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020018280548015611eb057602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311611e67575b505050505090505f815190505f8167ffffffffffffffff811115611ed757611ed6613b71565b5b604051908082528060200260200182016040528015611f0a57816020015b6060815260200190600190039081611ef55790505b5090505f5b82811015611fef5773a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663e3b2a874858381518110611f5a57611f59614614565b5b60200260200101516040518263ffffffff1660e01b8152600401611f7e91906144cb565b5f60405180830381865afa158015611f98573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f82011682018060405250810190611fc09190614bea565b60600151828281518110611fd757611fd6614614565b5b60200260200101819052508080600101915050611f0f565b508085600b015f8a81526020019081526020015f208080546120109061479b565b80601f016020809104026020016040519081016040528092919081815260200182805461203c9061479b565b80156120875780601f1061205e57610100808354040283529160200191612087565b820191905f5260205f20905b81548152906001019060200180831161206a57829003601f168201915b50505050509050965096505050505050915091565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156120f9573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061211d91906144a0565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461218c57336040517f0e56cf3d00000000000000000000000000000000000000000000000000000000815260040161218391906144cb565b60405180910390fd5b5f612195612367565b9050806004015f8154809291906121ab90614511565b91905055505f81600401549050816005015f8154809291906121cc90614511565b91905055505f8260050154905080836006015f8481526020019081526020015f208190555081836006015f8381526020019081526020015f20819055505f8484600d015f8581526020019081526020015f205f6101000a81548160ff0219169083600181111561223f5761223e61390f565b5b02179055507f02024007d96574dbc9d11328bfee9893e7c7bb4ef4aa806df33bfdf454eb5e6083828760405161227793929190614558565b60405180910390a15050505050565b5f80612290612367565b9050806008015491505090565b60605f60016122ab84612c7c565b0190505f8167ffffffffffffffff8111156122c9576122c8613b71565b5b6040519080825280601f01601f1916602001820160405280156122fb5781602001600182028036833780820191505090505b5090505f82602001820190505b60011561235c578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a858161235157612350614f47565b5b0494505f8503612308575b819350505050919050565b5f7f0b8fdb1f0a6356dd20a6cbc6f9668fac23b85f96575d10e333e603faa794ac00905090565b5f6123976123b2565b5f015f9054906101000a900467ffffffffffffffff16905090565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b6123e1612dcd565b6123eb8282612e0d565b5050565b5f808383905067ffffffffffffffff81111561240e5761240d613b71565b5b60405190808252806020026020018201604052801561243c5781602001602082028036833780820191505090505b5090505f5b84849050811015612540576040518060600160405280602581526020016154ef602591398051906020012085858381811061247f5761247e614614565b5b9050602002810190612491919061464d565b5f0160208101906124a29190614f74565b8686848181106124b5576124b4614614565b5b90506020028101906124c7919061464d565b80602001906124d69190614702565b6040516124e4929190614fcd565b60405180910390206040516020016124fe93929190614ff4565b6040516020818303038152906040528051906020012082828151811061252757612526614614565b5b6020026020010181815250508080600101915050612441565b506125bb6040518060a001604052806072815260200161547d607291398051906020012087878460405160200161257791906150da565b604051602081830303815290604052805190602001206040516020016125a094939291906150f0565b60405160208183030381529060405280519060200120612e5e565b915050949350505050565b5f806126158585858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050612e77565b905061262081612ea1565b809150509392505050565b5f8073a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663b4722bc46040518163ffffffff1660e01b8152600401602060405180830381865afa15801561268a573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906126ae9190615147565b905080831015915050919050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff16148061276957507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16612750612f71565b73ffffffffffffffffffffffffffffffffffffffff1614155b156127a0576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156127ff573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061282391906144a0565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461289257336040517f0e56cf3d00000000000000000000000000000000000000000000000000000000815260040161288991906144cb565b60405180910390fd5b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa9250505080156128fd57506040513d601f19601f820116820180604052508101906128fa919061519c565b60015b61293e57816040517f4c9c8ce300000000000000000000000000000000000000000000000000000000815260040161293591906144cb565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b81146129a457806040517faa1d49a400000000000000000000000000000000000000000000000000000000815260040161299b9190613d07565b60405180910390fd5b6129ae8383612fc4565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff1614612a38576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f612a8b6040518060600160405280602c8152602001615451602c91398051906020012083604051602001612a709291906151c7565b60405160208183030381529060405280519060200120612e5e565b9050919050565b5f612b0f60405180608001604052806046815260200161540b604691398051906020012086868686604051602001612acb929190614fcd565b60405160208183030381529060405280519060200120604051602001612af494939291906150f0565b60405160208183030381529060405280519060200120612e5e565b9050949350505050565b5f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100905090565b60605f612b4b612b19565b9050806002018054612b5c9061479b565b80601f0160208091040260200160405190810160405280929190818152602001828054612b889061479b565b8015612bd35780601f10612baa57610100808354040283529160200191612bd3565b820191905f5260205f20905b815481529060010190602001808311612bb657829003601f168201915b505050505091505090565b60605f612be9612b19565b9050806003018054612bfa9061479b565b80601f0160208091040260200160405190810160405280929190818152602001828054612c269061479b565b8015612c715780601f10612c4857610100808354040283529160200191612c71565b820191905f5260205f20905b815481529060010190602001808311612c5457829003601f168201915b505050505091505090565b5f805f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f0100000000000000008310612cd8577a184f03e93ff9f4daa797ed6e38ed64bf6a1f0100000000000000008381612cce57612ccd614f47565b5b0492506040810190505b6d04ee2d6d415b85acef81000000008310612d15576d04ee2d6d415b85acef81000000008381612d0b57612d0a614f47565b5b0492506020810190505b662386f26fc100008310612d4457662386f26fc100008381612d3a57612d39614f47565b5b0492506010810190505b6305f5e1008310612d6d576305f5e1008381612d6357612d62614f47565b5b0492506008810190505b6127108310612d92576127108381612d8857612d87614f47565b5b0492506004810190505b60648310612db55760648381612dab57612daa614f47565b5b0492506002810190505b600a8310612dc4576001810190505b80915050919050565b612dd5613036565b612e0b576040517fd7e6bcf800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b612e15612dcd565b5f612e1e612b19565b905082816002019081612e319190615246565b5081816003019081612e439190615246565b505f801b815f01819055505f801b8160010181905550505050565b5f612e70612e6a613054565b83613062565b9050919050565b5f805f80612e8586866130a2565b925092509250612e9582826130f7565b82935050505092915050565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663203d0114826040518263ffffffff1660e01b8152600401612eee91906144cb565b602060405180830381865afa158015612f09573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612f2d91906145c2565b612f6e57806040517f2a7c6ef6000000000000000000000000000000000000000000000000000000008152600401612f6591906144cb565b60405180910390fd5b50565b5f612f9d7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b613259565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b612fcd82613262565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f8151111561302957613023828261332b565b50613032565b6130316133ab565b5b5050565b5f61303f6123b2565b5f0160089054906101000a900460ff16905090565b5f61305d6133e7565b905090565b5f6040517f190100000000000000000000000000000000000000000000000000000000000081528360028201528260228201526042812091505092915050565b5f805f60418451036130e2575f805f602087015192506040870151915060608701515f1a90506130d48882858561344a565b9550955095505050506130f0565b5f600285515f1b9250925092505b9250925092565b5f600381111561310a5761310961390f565b5b82600381111561311d5761311c61390f565b5b031561325557600160038111156131375761313661390f565b5b82600381111561314a5761314961390f565b5b03613181576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600260038111156131955761319461390f565b5b8260038111156131a8576131a761390f565b5b036131ec57805f1c6040517ffce698f70000000000000000000000000000000000000000000000000000000081526004016131e39190614264565b60405180910390fd5b6003808111156131ff576131fe61390f565b5b8260038111156132125761321161390f565b5b0361325457806040517fd78bce0c00000000000000000000000000000000000000000000000000000000815260040161324b9190613d07565b60405180910390fd5b5b5050565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b036132bd57806040517f4c9c8ce30000000000000000000000000000000000000000000000000000000081526004016132b491906144cb565b60405180910390fd5b806132e97f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b613259565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f808473ffffffffffffffffffffffffffffffffffffffff16846040516133549190615345565b5f60405180830381855af49150503d805f811461338c576040519150601f19603f3d011682016040523d82523d5f602084013e613391565b606091505b50915091506133a1858383613531565b9250505092915050565b5f3411156133e5576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f6134116135be565b613419613634565b463060405160200161342f95949392919061535b565b60405160208183030381529060405280519060200120905090565b5f805f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c1115613486575f600385925092509250613527565b5f6001888888886040515f81526020016040526040516134a994939291906153c7565b6020604051602081039080840390855afa1580156134c9573d5f803e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff160361351a575f60015f801b93509350935050613527565b805f805f1b935093509350505b9450945094915050565b60608261354657613541826136ab565b6135b6565b5f825114801561356c57505f8473ffffffffffffffffffffffffffffffffffffffff163b145b156135ae57836040517f9996b3150000000000000000000000000000000000000000000000000000000081526004016135a591906144cb565b60405180910390fd5b8190506135b7565b5b9392505050565b5f806135c8612b19565b90505f6135d3612b40565b90505f815111156135ef57808051906020012092505050613631565b5f825f015490505f801b811461360a57809350505050613631565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f8061363e612b19565b90505f613649612bde565b90505f81511115613665578080519060200120925050506136a8565b5f826001015490505f801b8114613681578093505050506136a8565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f815111156136bd5780518082602001fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f81519050919050565b5f82825260208201905092915050565b5f5b8381101561372657808201518184015260208101905061370b565b5f8484015250505050565b5f601f19601f8301169050919050565b5f61374b826136ef565b61375581856136f9565b9350613765818560208601613709565b61376e81613731565b840191505092915050565b5f6020820190508181035f8301526137918184613741565b905092915050565b5f604051905090565b5f80fd5b5f80fd5b5f819050919050565b6137bc816137aa565b81146137c6575f80fd5b50565b5f813590506137d7816137b3565b92915050565b5f602082840312156137f2576137f16137a2565b5b5f6137ff848285016137c9565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61385a82613831565b9050919050565b61386a81613850565b82525050565b5f61387b8383613861565b60208301905092915050565b5f602082019050919050565b5f61389d82613808565b6138a78185613812565b93506138b283613822565b805f5b838110156138e25781516138c98882613870565b97506138d483613887565b9250506001810190506138b5565b5085935050505092915050565b5f6020820190508181035f8301526139078184613893565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b6002811061394d5761394c61390f565b5b50565b5f81905061395d8261393c565b919050565b5f61396c82613950565b9050919050565b61397c81613962565b82525050565b5f6020820190506139955f830184613973565b92915050565b600281106139a7575f80fd5b50565b5f813590506139b88161399b565b92915050565b5f80604083850312156139d4576139d36137a2565b5b5f6139e1858286016137c9565b92505060206139f2858286016139aa565b9150509250929050565b5f80fd5b5f80fd5b5f80fd5b5f8083601f840112613a1d57613a1c6139fc565b5b8235905067ffffffffffffffff811115613a3a57613a39613a00565b5b602083019150836020820283011115613a5657613a55613a04565b5b9250929050565b5f8083601f840112613a7257613a716139fc565b5b8235905067ffffffffffffffff811115613a8f57613a8e613a00565b5b602083019150836001820283011115613aab57613aaa613a04565b5b9250929050565b5f805f805f60608688031215613acb57613aca6137a2565b5b5f613ad8888289016137c9565b955050602086013567ffffffffffffffff811115613af957613af86137a6565b5b613b0588828901613a08565b9450945050604086013567ffffffffffffffff811115613b2857613b276137a6565b5b613b3488828901613a5d565b92509250509295509295909350565b613b4c81613850565b8114613b56575f80fd5b50565b5f81359050613b6781613b43565b92915050565b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b613ba782613731565b810181811067ffffffffffffffff82111715613bc657613bc5613b71565b5b80604052505050565b5f613bd8613799565b9050613be48282613b9e565b919050565b5f67ffffffffffffffff821115613c0357613c02613b71565b5b613c0c82613731565b9050602081019050919050565b828183375f83830152505050565b5f613c39613c3484613be9565b613bcf565b905082815260208101848484011115613c5557613c54613b6d565b5b613c60848285613c19565b509392505050565b5f82601f830112613c7c57613c7b6139fc565b5b8135613c8c848260208601613c27565b91505092915050565b5f8060408385031215613cab57613caa6137a2565b5b5f613cb885828601613b59565b925050602083013567ffffffffffffffff811115613cd957613cd86137a6565b5b613ce585828601613c68565b9150509250929050565b5f819050919050565b613d0181613cef565b82525050565b5f602082019050613d1a5f830184613cf8565b92915050565b5f805f60408486031215613d3757613d366137a2565b5b5f613d44868287016137c9565b935050602084013567ffffffffffffffff811115613d6557613d646137a6565b5b613d7186828701613a5d565b92509250509250925092565b5f805f805f60608688031215613d9657613d956137a2565b5b5f613da3888289016137c9565b955050602086013567ffffffffffffffff811115613dc457613dc36137a6565b5b613dd088828901613a5d565b9450945050604086013567ffffffffffffffff811115613df357613df26137a6565b5b613dff88828901613a5d565b92509250509295509295909350565b5f7fff0000000000000000000000000000000000000000000000000000000000000082169050919050565b613e4281613e0e565b82525050565b613e51816137aa565b82525050565b613e6081613850565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b613e98816137aa565b82525050565b5f613ea98383613e8f565b60208301905092915050565b5f602082019050919050565b5f613ecb82613e66565b613ed58185613e70565b9350613ee083613e80565b805f5b83811015613f10578151613ef78882613e9e565b9750613f0283613eb5565b925050600181019050613ee3565b5085935050505092915050565b5f60e082019050613f305f83018a613e39565b8181036020830152613f428189613741565b90508181036040830152613f568188613741565b9050613f656060830187613e48565b613f726080830186613e57565b613f7f60a0830185613cf8565b81810360c0830152613f918184613ec1565b905098975050505050505050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f82825260208201905092915050565b5f613fe2826136ef565b613fec8185613fc8565b9350613ffc818560208601613709565b61400581613731565b840191505092915050565b5f61401b8383613fd8565b905092915050565b5f602082019050919050565b5f61403982613f9f565b6140438185613fa9565b93508360208202850161405585613fb9565b805f5b8581101561409057848403895281516140718582614010565b945061407c83614023565b925060208a01995050600181019050614058565b50829750879550505050505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b600281106140dc576140db61390f565b5b50565b5f8190506140ec826140cb565b919050565b5f6140fb826140df565b9050919050565b61410b816140f1565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f61413582614111565b61413f818561411b565b935061414f818560208601613709565b61415881613731565b840191505092915050565b5f604083015f8301516141785f860182614102565b5060208301518482036020860152614190828261412b565b9150508091505092915050565b5f6141a88383614163565b905092915050565b5f602082019050919050565b5f6141c6826140a2565b6141d081856140ac565b9350836020820285016141e2856140bc565b805f5b8581101561421d57848403895281516141fe858261419d565b9450614209836141b0565b925060208a019950506001810190506141e5565b50829750879550505050505092915050565b5f6040820190508181035f830152614247818561402f565b9050818103602083015261425b81846141bc565b90509392505050565b5f6020820190506142775f830184613e48565b92915050565b5f82825260208201905092915050565b5f61429782614111565b6142a1818561427d565b93506142b1818560208601613709565b6142ba81613731565b840191505092915050565b5f6040820190508181035f8301526142dd818561402f565b905081810360208301526142f1818461428d565b90509392505050565b5f6020828403121561430f5761430e6137a2565b5b5f61431c848285016139aa565b91505092915050565b5f81905092915050565b5f614339826136ef565b6143438185614325565b9350614353818560208601613709565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f614393600283614325565b915061439e8261435f565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f6143dd600183614325565b91506143e8826143a9565b600182019050919050565b5f6143fe828761432f565b915061440982614387565b9150614415828661432f565b9150614420826143d1565b915061442c828561432f565b9150614437826143d1565b9150614443828461432f565b915081905095945050505050565b5f67ffffffffffffffff82169050919050565b61446d81614451565b82525050565b5f6020820190506144865f830184614464565b92915050565b5f8151905061449a81613b43565b92915050565b5f602082840312156144b5576144b46137a2565b5b5f6144c28482850161448c565b91505092915050565b5f6020820190506144de5f830184613e57565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61451b826137aa565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff820361454d5761454c6144e4565b5b600182019050919050565b5f60608201905061456b5f830186613e48565b6145786020830185613e48565b6145856040830184613973565b949350505050565b5f8115159050919050565b6145a18161458d565b81146145ab575f80fd5b50565b5f815190506145bc81614598565b92915050565b5f602082840312156145d7576145d66137a2565b5b5f6145e4848285016145ae565b91505092915050565b5f6040820190506146005f830185613e48565b61460d6020830184613e57565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b5f8235600160400383360303811261466857614667614641565b5b80830191505092915050565b60028110614680575f80fd5b50565b5f813561468f81614674565b80915050919050565b5f815f1b9050919050565b5f60ff6146af84614698565b9350801983169250808416831791505092915050565b5f6146cf826140df565b9050919050565b5f819050919050565b6146e8826146c5565b6146fb6146f4826146d6565b83546146a3565b8255505050565b5f808335600160200384360303811261471e5761471d614641565b5b80840192508235915067ffffffffffffffff8211156147405761473f614645565b5b60208301925060018202360383131561475c5761475b614649565b5b509250929050565b5f82905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f60028204905060018216806147b257607f821691505b6020821081036147c5576147c461476e565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f600883026148277fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff826147ec565b61483186836147ec565b95508019841693508086168417925050509392505050565b5f819050919050565b5f61486c614867614862846137aa565b614849565b6137aa565b9050919050565b5f819050919050565b61488583614852565b61489961489182614873565b8484546147f8565b825550505050565b5f90565b6148ad6148a1565b6148b881848461487c565b505050565b5b818110156148db576148d05f826148a5565b6001810190506148be565b5050565b601f821115614920576148f1816147cb565b6148fa846147dd565b81016020851015614909578190505b61491d614915856147dd565b8301826148bd565b50505b505050565b5f82821c905092915050565b5f6149405f1984600802614925565b1980831691505092915050565b5f6149588383614931565b9150826002028217905092915050565b6149728383614764565b67ffffffffffffffff81111561498b5761498a613b71565b5b614995825461479b565b6149a08282856148df565b5f601f8311600181146149cd575f84156149bb578287013590505b6149c5858261494d565b865550614a2c565b601f1984166149db866147cb565b5f5b82811015614a02578489013582556001820191506020850194506020810190506149dd565b86831015614a1f5784890135614a1b601f891682614931565b8355505b6001600288020188555050505b50505050505050565b614a40838383614968565b505050565b5f81015f830180614a5581614683565b9050614a6181846146df565b5050506001810160208301614a768185614702565b614a81818386614a35565b505050505050565b614a938282614a45565b5050565b5f80fd5b5f80fd5b5f67ffffffffffffffff821115614ab957614ab8613b71565b5b614ac282613731565b9050602081019050919050565b5f614ae1614adc84614a9f565b613bcf565b905082815260208101848484011115614afd57614afc613b6d565b5b614b08848285613709565b509392505050565b5f82601f830112614b2457614b236139fc565b5b8151614b34848260208601614acf565b91505092915050565b5f60808284031215614b5257614b51614a97565b5b614b5c6080613bcf565b90505f614b6b8482850161448c565b5f830152506020614b7e8482850161448c565b602083015250604082015167ffffffffffffffff811115614ba257614ba1614a9b565b5b614bae84828501614b10565b604083015250606082015167ffffffffffffffff811115614bd257614bd1614a9b565b5b614bde84828501614b10565b60608301525092915050565b5f60208284031215614bff57614bfe6137a2565b5b5f82015167ffffffffffffffff811115614c1c57614c1b6137a6565b5b614c2884828501614b3d565b91505092915050565b5f819050919050565b5f81359050614c4881614674565b92915050565b5f614c5c6020840184614c3a565b905092915050565b5f80fd5b5f80fd5b5f80fd5b5f8083356001602003843603038112614c8c57614c8b614c6c565b5b83810192508235915060208301925067ffffffffffffffff821115614cb457614cb3614c64565b5b600182023603831315614cca57614cc9614c68565b5b509250929050565b5f614cdd838561411b565b9350614cea838584613c19565b614cf383613731565b840190509392505050565b5f60408301614d0f5f840184614c4e565b614d1b5f860182614102565b50614d296020840184614c70565b8583036020870152614d3c838284614cd2565b925050508091505092915050565b5f614d558383614cfe565b905092915050565b5f82356001604003833603038112614d7857614d77614c6c565b5b82810191505092915050565b5f602082019050919050565b5f614d9b83856140ac565b935083602084028501614dad84614c31565b805f5b87811015614df0578484038952614dc78284614d5d565b614dd18582614d4a565b9450614ddc83614d84565b925060208a01995050600181019050614db0565b50829750879450505050509392505050565b5f606082019050614e155f830187613e48565b8181036020830152614e27818661402f565b90508181036040830152614e3c818486614d90565b905095945050505050565b5f604082019050614e5a5f830185613e48565b614e676020830184613e48565b9392505050565b5f614e79838561427d565b9350614e86838584613c19565b614e8f83613731565b840190509392505050565b5f606082019050614ead5f830187613e48565b8181036020830152614ebf818661402f565b90508181036040830152614ed4818486614e6e565b905095945050505050565b7f4549503731323a20556e696e697469616c697a656400000000000000000000005f82015250565b5f614f136015836136f9565b9150614f1e82614edf565b602082019050919050565b5f6020820190508181035f830152614f4081614f07565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f60208284031215614f8957614f886137a2565b5b5f614f9684828501614c3a565b91505092915050565b5f81905092915050565b5f614fb48385614f9f565b9350614fc1838584613c19565b82840190509392505050565b5f614fd9828486614fa9565b91508190509392505050565b614fee816140f1565b82525050565b5f6060820190506150075f830186613cf8565b6150146020830185614fe5565b6150216040830184613cf8565b949350505050565b5f81519050919050565b5f81905092915050565b5f819050602082019050919050565b61505581613cef565b82525050565b5f615066838361504c565b60208301905092915050565b5f602082019050919050565b5f61508882615029565b6150928185615033565b935061509d8361503d565b805f5b838110156150cd5781516150b4888261505b565b97506150bf83615072565b9250506001810190506150a0565b5085935050505092915050565b5f6150e5828461507e565b915081905092915050565b5f6080820190506151035f830187613cf8565b6151106020830186613e48565b61511d6040830185613e48565b61512a6060830184613cf8565b95945050505050565b5f81519050615141816137b3565b92915050565b5f6020828403121561515c5761515b6137a2565b5b5f61516984828501615133565b91505092915050565b61517b81613cef565b8114615185575f80fd5b50565b5f8151905061519681615172565b92915050565b5f602082840312156151b1576151b06137a2565b5b5f6151be84828501615188565b91505092915050565b5f6040820190506151da5f830185613cf8565b6151e76020830184613e48565b9392505050565b5f819050815f5260205f209050919050565b601f82111561524157615212816151ee565b61521b846147dd565b8101602085101561522a578190505b61523e615236856147dd565b8301826148bd565b50505b505050565b61524f826136ef565b67ffffffffffffffff81111561526857615267613b71565b5b615272825461479b565b61527d828285615200565b5f60209050601f8311600181146152ae575f841561529c578287015190505b6152a6858261494d565b86555061530d565b601f1984166152bc866151ee565b5f5b828110156152e3578489015182556001820191506020850194506020810190506152be565b8683101561530057848901516152fc601f891682614931565b8355505b6001600288020188555050505b505050505050565b5f61531f82614111565b6153298185614f9f565b9350615339818560208601613709565b80840191505092915050565b5f6153508284615315565b915081905092915050565b5f60a08201905061536e5f830188613cf8565b61537b6020830187613cf8565b6153886040830186613cf8565b6153956060830185613e48565b6153a26080830184613e57565b9695505050505050565b5f60ff82169050919050565b6153c1816153ac565b82525050565b5f6080820190506153da5f830187613cf8565b6153e760208301866153b8565b6153f46040830185613cf8565b6154016060830184613cf8565b9594505050505056fe43727367656e566572696669636174696f6e2875696e743235362063727349642c75696e74323536206d61784269744c656e6774682c62797465732063727344696765737429507265704b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e4964294b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e49642c75696e74323536206b657949642c4b65794469676573745b5d206b657944696765737473294b65794469676573742875696e7438206b6579547970652c627974657320646967657374294b65794469676573742875696e7438206b6579547970652c62797465732064696765737429
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP4\x80\x15b\0\0CW_\x80\xFD[Pb\0\0Tb\0\0Z` \x1B` \x1CV[b\0\x01\xC4V[_b\0\0kb\0\x01^` \x1B` \x1CV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15b\0\0\xB6W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14b\0\x01[Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Qb\0\x01R\x91\x90b\0\x01\xA9V[`@Q\x80\x91\x03\x90\xA1[PV[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[b\0\x01\xA3\x81b\0\x01\x85V[\x82RPPV[_` \x82\x01\x90Pb\0\x01\xBE_\x83\x01\x84b\0\x01\x98V[\x92\x91PPV[`\x80QaU\x14b\0\x01\xEB_9_\x81\x81a&\xBE\x01R\x81\x81a'\x13\x01Ra)\xB5\x01RaU\x14_\xF3\xFE`\x80`@R`\x046\x10a\x01\x08W_5`\xE0\x1C\x80cX\x9A\xDB\x0E\x11a\0\x94W\x80c\xAD<\xB1\xCC\x11a\0cW\x80c\xAD<\xB1\xCC\x14a\x03SW\x80c\xBA\xFF!\x1E\x14a\x03}W\x80c\xC5[\x87$\x14a\x03\xA7W\x80c\xCA\xA3g\xDB\x14a\x03\xE4W\x80c\xD5/\x10\xEB\x14a\x04\x0CWa\x01\x08V[\x80cX\x9A\xDB\x0E\x14a\x02\x96W\x80cb\x97\x87\x87\x14a\x02\xBEW\x80c\x84\xB0\x19n\x14a\x02\xE6W\x80c\x93f\x08\xAE\x14a\x03\x16Wa\x01\x08V[\x80c<\x02\xF84\x11a\0\xDBW\x80c<\x02\xF84\x14a\x01\xC4W\x80cE\xAF&\x1B\x14a\x01\xECW\x80cF\x10\xFF\xE8\x14a\x02(W\x80cO\x1E\xF2\x86\x14a\x02PW\x80cR\xD1\x90-\x14a\x02lWa\x01\x08V[\x80c\r\x8En,\x14a\x01\x0CW\x80c\x16\xC7\x13\xD9\x14a\x016W\x80c\x19\xF4\xF62\x14a\x01rW\x80c9\xF78\x10\x14a\x01\xAEW[_\x80\xFD[4\x80\x15a\x01\x17W_\x80\xFD[Pa\x01 a\x046V[`@Qa\x01-\x91\x90a7yV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01AW_\x80\xFD[Pa\x01\\`\x04\x806\x03\x81\x01\x90a\x01W\x91\x90a7\xDDV[a\x04\xB1V[`@Qa\x01i\x91\x90a8\xEFV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01}W_\x80\xFD[Pa\x01\x98`\x04\x806\x03\x81\x01\x90a\x01\x93\x91\x90a7\xDDV[a\x05\x82V[`@Qa\x01\xA5\x91\x90a9\x82V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xB9W_\x80\xFD[Pa\x01\xC2a\x06/V[\0[4\x80\x15a\x01\xCFW_\x80\xFD[Pa\x01\xEA`\x04\x806\x03\x81\x01\x90a\x01\xE5\x91\x90a9\xBEV[a\x08~V[\0[4\x80\x15a\x01\xF7W_\x80\xFD[Pa\x02\x12`\x04\x806\x03\x81\x01\x90a\x02\r\x91\x90a7\xDDV[a\n-V[`@Qa\x02\x1F\x91\x90a9\x82V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x023W_\x80\xFD[Pa\x02N`\x04\x806\x03\x81\x01\x90a\x02I\x91\x90a:\xB2V[a\n\xC2V[\0[a\x02j`\x04\x806\x03\x81\x01\x90a\x02e\x91\x90a<\x95V[a\x10\x07V[\0[4\x80\x15a\x02wW_\x80\xFD[Pa\x02\x80a\x10&V[`@Qa\x02\x8D\x91\x90a=\x07V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xA1W_\x80\xFD[Pa\x02\xBC`\x04\x806\x03\x81\x01\x90a\x02\xB7\x91\x90a= V[a\x10WV[\0[4\x80\x15a\x02\xC9W_\x80\xFD[Pa\x02\xE4`\x04\x806\x03\x81\x01\x90a\x02\xDF\x91\x90a=}V[a\x13\xA9V[\0[4\x80\x15a\x02\xF1W_\x80\xFD[Pa\x02\xFAa\x18\x8AV[`@Qa\x03\r\x97\x96\x95\x94\x93\x92\x91\x90a?\x1DV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03!W_\x80\xFD[Pa\x03<`\x04\x806\x03\x81\x01\x90a\x037\x91\x90a7\xDDV[a\x19\x93V[`@Qa\x03J\x92\x91\x90aB/V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03^W_\x80\xFD[Pa\x03ga\x1D:V[`@Qa\x03t\x91\x90a7yV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x88W_\x80\xFD[Pa\x03\x91a\x1DsV[`@Qa\x03\x9E\x91\x90aBdV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xB2W_\x80\xFD[Pa\x03\xCD`\x04\x806\x03\x81\x01\x90a\x03\xC8\x91\x90a7\xDDV[a\x1D\x8AV[`@Qa\x03\xDB\x92\x91\x90aB\xC5V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xEFW_\x80\xFD[Pa\x04\n`\x04\x806\x03\x81\x01\x90a\x04\x05\x91\x90aB\xFAV[a \x9CV[\0[4\x80\x15a\x04\x17W_\x80\xFD[Pa\x04 a\"\x86V[`@Qa\x04-\x91\x90aBdV[`@Q\x80\x91\x03\x90\xF3[```@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x04w_a\"\x9DV[a\x04\x81`\x01a\"\x9DV[a\x04\x8A_a\"\x9DV[`@Q` \x01a\x04\x9D\x94\x93\x92\x91\x90aC\xF3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[``_a\x04\xBCa#gV[\x90P_\x81`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\x02\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x82\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x05tW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x05+W[PPPPP\x92PPP\x91\x90PV[_\x80a\x05\x8Ca#gV[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x05\xEFW\x82`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05\xE6\x91\x90aBdV[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\r\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x92PPP\x91\x90PV[`\x01a\x069a#\x8EV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x06zW`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02_a\x06\x85a#\xB2V[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x06\xCDWP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x07\x04W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x07\xBD`@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa#\xD9V[_a\x07\xC6a#gV[\x90P`\xF8`\x03`\x05\x81\x11\x15a\x07\xDEWa\x07\xDDa9\x0FV[[\x90\x1B\x81`\x04\x01\x81\x90UP`\xF8`\x04`\x05\x81\x11\x15a\x07\xFEWa\x07\xFDa9\x0FV[[\x90\x1B\x81`\x05\x01\x81\x90UP`\xF8`\x05\x80\x81\x11\x15a\x08\x1DWa\x08\x1Ca9\x0FV[[\x90\x1B\x81`\t\x01\x81\x90UPP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x08r\x91\x90aDsV[`@Q\x80\x91\x03\x90\xA1PPV[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xDBW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xFF\x91\x90aD\xA0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\tnW3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\te\x91\x90aD\xCBV[`@Q\x80\x91\x03\x90\xFD[_a\twa#gV[\x90P\x80`\t\x01_\x81T\x80\x92\x91\x90a\t\x8D\x90aE\x11V[\x91\x90PUP_\x81`\t\x01T\x90P\x83\x82`\n\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82\x82`\r\x01_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a\t\xE7Wa\t\xE6a9\x0FV[[\x02\x17\x90UP\x7F?\x03\x8Fo\x88\xCB01\xB7q\x85\x88@:.\xC2 Wj\x86\x8B\xE0}\xDEL\x02\xB8F\xCA5.\xF5\x81\x85\x85`@Qa\n\x1F\x93\x92\x91\x90aEXV[`@Q\x80\x91\x03\x90\xA1PPPPV[_\x80a\n7a#gV[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\n\x9AW\x82`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\x91\x91\x90aBdV[`@Q\x80\x91\x03\x90\xFD[\x80`\r\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE5'^\xAF3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\x0F\x91\x90aD\xCBV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B*W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BN\x91\x90aE\xC2V[a\x0B\x8FW3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B\x86\x91\x90aD\xCBV[`@Q\x80\x91\x03\x90\xFD[_a\x0B\x98a#gV[\x90P_\x81`\x06\x01_\x88\x81R` \x01\x90\x81R` \x01_ T\x90P_a\x0B\xBE\x82\x89\x89\x89a#\xEFV[\x90P_a\x0B\xCC\x82\x87\x87a%\xC6V[\x90P\x83_\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x0ClW\x88\x81`@Q\x7F\x98\xFB\x95}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0Cc\x92\x91\x90aE\xEDV[`@Q\x80\x91\x03\x90\xFD[`\x01\x84_\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x84`\x02\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_\x81\x80T\x90P\x90P\x85`\x01\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\r\x8FWPa\r\x8E\x81a&+V[[\x15a\x0F\xFAW`\x01\x86`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_[\x8A\x8A\x90P\x81\x10\x15a\x0EEW\x86`\x07\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x8B\x8B\x83\x81\x81\x10a\r\xF2Wa\r\xF1aF\x14V[[\x90P` \x02\x81\x01\x90a\x0E\x04\x91\x90aFMV[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P\x81\x81a\x0E6\x91\x90aJ\x89V[PP\x80\x80`\x01\x01\x91PPa\r\xC1V[P\x83\x86`\x03\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x8A\x86`\x08\x01\x81\x90UP_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\x82Wa\x0E\x81a;qV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\xB5W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0E\xA0W\x90P[P\x90P_[\x82\x81\x10\x15a\x0F\xBAWs\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3\xB2\xA8t\x85\x83\x81T\x81\x10a\x0F\x05Wa\x0F\x04aF\x14V[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0FI\x91\x90aD\xCBV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FcW=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x8B\x91\x90aK\xEAV[``\x01Q\x82\x82\x81Q\x81\x10a\x0F\xA2Wa\x0F\xA1aF\x14V[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa\x0E\xBAV[P\x7F\xEB\x85\xC2m\xBC\xADF\xB8\nh\xA0\xF2L\xCE|,\x90\xF0\xA1\xFA\xDE\xD8A\x84\x13\x889\xFC\x9E\x80\xA2[\x8C\x82\x8D\x8D`@Qa\x0F\xF0\x94\x93\x92\x91\x90aN\x02V[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPPPPPV[a\x10\x0Fa&\xBCV[a\x10\x18\x82a'\xA2V[a\x10\"\x82\x82a(\x95V[PPV[_a\x10/a)\xB3V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE5'^\xAF3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\xA4\x91\x90aD\xCBV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xBFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xE3\x91\x90aE\xC2V[a\x11$W3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\x1B\x91\x90aD\xCBV[`@Q\x80\x91\x03\x90\xFD[_a\x11-a#gV[\x90P_a\x119\x85a*:V[\x90P_a\x11G\x82\x86\x86a%\xC6V[\x90P\x82_\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x11\xE7W\x85\x81`@Q\x7F3\xCA\x1F\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\xDE\x92\x91\x90aE\xEDV[`@Q\x80\x91\x03\x90\xFD[`\x01\x83_\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x83`\x02\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x83`\x01\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x13\x06WPa\x13\x05\x81\x80T\x90Pa&+V[[\x15a\x13\xA0W`\x01\x84`\x01\x01_\x89\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x82\x84`\x03\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x81\x90UP_\x84`\x06\x01_\x89\x81R` \x01\x90\x81R` \x01_ T\x90P\x7Fx\xB1y\x17m\x1F\x19\xD7\xC2\x8E\x80\x82=\xEB\xA2bM\xA2\xCA.\xC6K\x17\x01\xF3c*\x87\xC9\xAE\xDC\x92\x88\x82`@Qa\x13\x96\x92\x91\x90aNGV[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPV[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE5'^\xAF3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\xF6\x91\x90aD\xCBV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\x11W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x145\x91\x90aE\xC2V[a\x14vW3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14m\x91\x90aD\xCBV[`@Q\x80\x91\x03\x90\xFD[_a\x14\x7Fa#gV[\x90P_\x81`\n\x01_\x88\x81R` \x01\x90\x81R` \x01_ T\x90P_a\x14\xA5\x88\x83\x89\x89a*\x92V[\x90P_a\x14\xB3\x82\x87\x87a%\xC6V[\x90P\x83_\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x15SW\x88\x81`@Q\x7F\xFC\xF5\xA6\xE9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15J\x92\x91\x90aE\xEDV[`@Q\x80\x91\x03\x90\xFD[`\x01\x84_\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x84`\x02\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_\x81\x80T\x90P\x90P\x85`\x01\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x16vWPa\x16u\x81a&+V[[\x15a\x18}W`\x01\x86`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x89\x89\x87`\x0B\x01_\x8E\x81R` \x01\x90\x81R` \x01_ \x91\x82a\x16\xC8\x92\x91\x90aIhV[P\x83\x86`\x03\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x8A\x86`\x0C\x01\x81\x90UP_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\x05Wa\x17\x04a;qV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x178W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x17#W\x90P[P\x90P_[\x82\x81\x10\x15a\x18=Ws\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3\xB2\xA8t\x85\x83\x81T\x81\x10a\x17\x88Wa\x17\x87aF\x14V[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\xCC\x91\x90aD\xCBV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xE6W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\x0E\x91\x90aK\xEAV[``\x01Q\x82\x82\x81Q\x81\x10a\x18%Wa\x18$aF\x14V[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa\x17=V[P\x7F\"X\xB7?\xAE\xD3?\xB2\xE2\xEAED\x03\xBE\xF9t\x92\x0C\xAFh*\xB3\xA7#HO\xCFgU;\x16\xA2\x8C\x82\x8D\x8D`@Qa\x18s\x94\x93\x92\x91\x90aN\x9AV[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPPPPPV[_``\x80_\x80_``_a\x18\x9Ca+\x19V[\x90P_\x80\x1B\x81_\x01T\x14\x80\x15a\x18\xB7WP_\x80\x1B\x81`\x01\x01T\x14[a\x18\xF6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18\xED\x90aO)V[`@Q\x80\x91\x03\x90\xFD[a\x18\xFEa+@V[a\x19\x06a+\xDEV[F0_\x80\x1B_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19%Wa\x19$a;qV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x19SW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94\x93\x92\x91\x90\x97P\x97P\x97P\x97P\x97P\x97P\x97PP\x90\x91\x92\x93\x94\x95\x96V[``\x80_a\x19\x9Fa#gV[\x90P\x80`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x1A\x02W\x83`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19\xF9\x91\x90aBdV[`@Q\x80\x91\x03\x90\xFD[_\x81`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x82`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1A\xB9W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x1ApW[PPPPP\x90P_\x81Q\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\xE0Wa\x1A\xDFa;qV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1B\x13W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1A\xFEW\x90P[P\x90P_[\x82\x81\x10\x15a\x1B\xF8Ws\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3\xB2\xA8t\x85\x83\x81Q\x81\x10a\x1BcWa\x1BbaF\x14V[[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B\x87\x91\x90aD\xCBV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xA1W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xC9\x91\x90aK\xEAV[``\x01Q\x82\x82\x81Q\x81\x10a\x1B\xE0Wa\x1B\xDFaF\x14V[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa\x1B\x18V[P\x80\x85`\x07\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x1D&W\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x1CqWa\x1Cpa9\x0FV[[`\x01\x81\x11\x15a\x1C\x83Wa\x1C\x82a9\x0FV[[\x81R` \x01`\x01\x82\x01\x80Ta\x1C\x97\x90aG\x9BV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1C\xC3\x90aG\x9BV[\x80\x15a\x1D\x0EW\x80`\x1F\x10a\x1C\xE5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1D\x0EV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1C\xF1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x1C-V[PPPP\x90P\x96P\x96PPPPPP\x91P\x91V[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[_\x80a\x1D}a#gV[\x90P\x80`\x0C\x01T\x91PP\x90V[``\x80_a\x1D\x96a#gV[\x90P\x80`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x1D\xF9W\x83`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1D\xF0\x91\x90aBdV[`@Q\x80\x91\x03\x90\xFD[_\x81`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x82`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1E\xB0W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x1EgW[PPPPP\x90P_\x81Q\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xD7Wa\x1E\xD6a;qV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1F\nW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1E\xF5W\x90P[P\x90P_[\x82\x81\x10\x15a\x1F\xEFWs\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3\xB2\xA8t\x85\x83\x81Q\x81\x10a\x1FZWa\x1FYaF\x14V[[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1F~\x91\x90aD\xCBV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\x98W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xC0\x91\x90aK\xEAV[``\x01Q\x82\x82\x81Q\x81\x10a\x1F\xD7Wa\x1F\xD6aF\x14V[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa\x1F\x0FV[P\x80\x85`\x0B\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80\x80Ta \x10\x90aG\x9BV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta <\x90aG\x9BV[\x80\x15a \x87W\x80`\x1F\x10a ^Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a \x87V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a jW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x96P\x96PPPPPP\x91P\x91V[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \xF9W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\x1D\x91\x90aD\xA0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a!\x8CW3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a!\x83\x91\x90aD\xCBV[`@Q\x80\x91\x03\x90\xFD[_a!\x95a#gV[\x90P\x80`\x04\x01_\x81T\x80\x92\x91\x90a!\xAB\x90aE\x11V[\x91\x90PUP_\x81`\x04\x01T\x90P\x81`\x05\x01_\x81T\x80\x92\x91\x90a!\xCC\x90aE\x11V[\x91\x90PUP_\x82`\x05\x01T\x90P\x80\x83`\x06\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x81\x83`\x06\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UP_\x84\x84`\r\x01_\x85\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a\"?Wa\">a9\x0FV[[\x02\x17\x90UP\x7F\x02\x02@\x07\xD9et\xDB\xC9\xD1\x13(\xBF\xEE\x98\x93\xE7\xC7\xBBN\xF4\xAA\x80m\xF3;\xFD\xF4T\xEB^`\x83\x82\x87`@Qa\"w\x93\x92\x91\x90aEXV[`@Q\x80\x91\x03\x90\xA1PPPPPV[_\x80a\"\x90a#gV[\x90P\x80`\x08\x01T\x91PP\x90V[``_`\x01a\"\xAB\x84a,|V[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\xC9Wa\"\xC8a;qV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\"\xFBW\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x01\x82\x01\x90P[`\x01\x15a#\\W\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a#QWa#PaOGV[[\x04\x94P_\x85\x03a#\x08W[\x81\x93PPPP\x91\x90PV[_\x7F\x0B\x8F\xDB\x1F\ncV\xDD \xA6\xCB\xC6\xF9f\x8F\xAC#\xB8_\x96W]\x10\xE33\xE6\x03\xFA\xA7\x94\xAC\0\x90P\x90V[_a#\x97a#\xB2V[_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[a#\xE1a-\xCDV[a#\xEB\x82\x82a.\rV[PPV[_\x80\x83\x83\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$\x0EWa$\ra;qV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a$<W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_[\x84\x84\x90P\x81\x10\x15a%@W`@Q\x80``\x01`@R\x80`%\x81R` \x01aT\xEF`%\x919\x80Q\x90` \x01 \x85\x85\x83\x81\x81\x10a$\x7FWa$~aF\x14V[[\x90P` \x02\x81\x01\x90a$\x91\x91\x90aFMV[_\x01` \x81\x01\x90a$\xA2\x91\x90aOtV[\x86\x86\x84\x81\x81\x10a$\xB5Wa$\xB4aF\x14V[[\x90P` \x02\x81\x01\x90a$\xC7\x91\x90aFMV[\x80` \x01\x90a$\xD6\x91\x90aG\x02V[`@Qa$\xE4\x92\x91\x90aO\xCDV[`@Q\x80\x91\x03\x90 `@Q` \x01a$\xFE\x93\x92\x91\x90aO\xF4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x82\x81Q\x81\x10a%'Wa%&aF\x14V[[` \x02` \x01\x01\x81\x81RPP\x80\x80`\x01\x01\x91PPa$AV[Pa%\xBB`@Q\x80`\xA0\x01`@R\x80`r\x81R` \x01aT}`r\x919\x80Q\x90` \x01 \x87\x87\x84`@Q` \x01a%w\x91\x90aP\xDAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a%\xA0\x94\x93\x92\x91\x90aP\xF0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a.^V[\x91PP\x94\x93PPPPV[_\x80a&\x15\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa.wV[\x90Pa& \x81a.\xA1V[\x80\x91PP\x93\x92PPPV[_\x80s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xB4r+\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\x8AW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\xAE\x91\x90aQGV[\x90P\x80\x83\x10\x15\x91PP\x91\x90PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a'iWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a'Pa/qV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a'\xA0W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'\xFFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(#\x91\x90aD\xA0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a(\x92W3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a(\x89\x91\x90aD\xCBV[`@Q\x80\x91\x03\x90\xFD[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a(\xFDWP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\xFA\x91\x90aQ\x9CV[`\x01[a)>W\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a)5\x91\x90aD\xCBV[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14a)\xA4W\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a)\x9B\x91\x90a=\x07V[`@Q\x80\x91\x03\x90\xFD[a)\xAE\x83\x83a/\xC4V[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a*8W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_a*\x8B`@Q\x80``\x01`@R\x80`,\x81R` \x01aTQ`,\x919\x80Q\x90` \x01 \x83`@Q` \x01a*p\x92\x91\x90aQ\xC7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a.^V[\x90P\x91\x90PV[_a+\x0F`@Q\x80`\x80\x01`@R\x80`F\x81R` \x01aT\x0B`F\x919\x80Q\x90` \x01 \x86\x86\x86\x86`@Q` \x01a*\xCB\x92\x91\x90aO\xCDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a*\xF4\x94\x93\x92\x91\x90aP\xF0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a.^V[\x90P\x94\x93PPPPV[_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x90P\x90V[``_a+Ka+\x19V[\x90P\x80`\x02\x01\x80Ta+\\\x90aG\x9BV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta+\x88\x90aG\x9BV[\x80\x15a+\xD3W\x80`\x1F\x10a+\xAAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a+\xD3V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a+\xB6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[``_a+\xE9a+\x19V[\x90P\x80`\x03\x01\x80Ta+\xFA\x90aG\x9BV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta,&\x90aG\x9BV[\x80\x15a,qW\x80`\x1F\x10a,HWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a,qV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a,TW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[_\x80_\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a,\xD8Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81a,\xCEWa,\xCDaOGV[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a-\x15Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81a-\x0BWa-\naOGV[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10a-DWf#\x86\xF2o\xC1\0\0\x83\x81a-:Wa-9aOGV[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10a-mWc\x05\xF5\xE1\0\x83\x81a-cWa-baOGV[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10a-\x92Wa'\x10\x83\x81a-\x88Wa-\x87aOGV[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10a-\xB5W`d\x83\x81a-\xABWa-\xAAaOGV[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10a-\xC4W`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[a-\xD5a06V[a.\x0BW`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a.\x15a-\xCDV[_a.\x1Ea+\x19V[\x90P\x82\x81`\x02\x01\x90\x81a.1\x91\x90aRFV[P\x81\x81`\x03\x01\x90\x81a.C\x91\x90aRFV[P_\x80\x1B\x81_\x01\x81\x90UP_\x80\x1B\x81`\x01\x01\x81\x90UPPPPV[_a.pa.ja0TV[\x83a0bV[\x90P\x91\x90PV[_\x80_\x80a.\x85\x86\x86a0\xA2V[\x92P\x92P\x92Pa.\x95\x82\x82a0\xF7V[\x82\x93PPPP\x92\x91PPV[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c =\x01\x14\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a.\xEE\x91\x90aD\xCBV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/\tW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/-\x91\x90aE\xC2V[a/nW\x80`@Q\x7F*|n\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a/e\x91\x90aD\xCBV[`@Q\x80\x91\x03\x90\xFD[PV[_a/\x9D\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba2YV[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a/\xCD\x82a2bV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15a0)Wa0#\x82\x82a3+V[Pa02V[a01a3\xABV[[PPV[_a0?a#\xB2V[_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[_a0]a3\xE7V[\x90P\x90V[_`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x02\x82\x01R\x82`\"\x82\x01R`B\x81 \x91PP\x92\x91PPV[_\x80_`A\x84Q\x03a0\xE2W_\x80_` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90Pa0\xD4\x88\x82\x85\x85a4JV[\x95P\x95P\x95PPPPa0\xF0V[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15a1\nWa1\ta9\x0FV[[\x82`\x03\x81\x11\x15a1\x1DWa1\x1Ca9\x0FV[[\x03\x15a2UW`\x01`\x03\x81\x11\x15a17Wa16a9\x0FV[[\x82`\x03\x81\x11\x15a1JWa1Ia9\x0FV[[\x03a1\x81W`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15a1\x95Wa1\x94a9\x0FV[[\x82`\x03\x81\x11\x15a1\xA8Wa1\xA7a9\x0FV[[\x03a1\xECW\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a1\xE3\x91\x90aBdV[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15a1\xFFWa1\xFEa9\x0FV[[\x82`\x03\x81\x11\x15a2\x12Wa2\x11a9\x0FV[[\x03a2TW\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a2K\x91\x90a=\x07V[`@Q\x80\x91\x03\x90\xFD[[PPV[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a2\xBDW\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a2\xB4\x91\x90aD\xCBV[`@Q\x80\x91\x03\x90\xFD[\x80a2\xE9\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba2YV[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qa3T\x91\x90aSEV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a3\x8CW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a3\x91V[``\x91P[P\x91P\x91Pa3\xA1\x85\x83\x83a51V[\x92PPP\x92\x91PPV[_4\x11\x15a3\xE5W`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa4\x11a5\xBEV[a4\x19a64V[F0`@Q` \x01a4/\x95\x94\x93\x92\x91\x90aS[V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x80_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15a4\x86W_`\x03\x85\x92P\x92P\x92Pa5'V[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@Qa4\xA9\x94\x93\x92\x91\x90aS\xC7V[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a4\xC9W=_\x80>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a5\x1AW_`\x01_\x80\x1B\x93P\x93P\x93PPa5'V[\x80_\x80_\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[``\x82a5FWa5A\x82a6\xABV[a5\xB6V[_\x82Q\x14\x80\x15a5lWP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a5\xAEW\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a5\xA5\x91\x90aD\xCBV[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa5\xB7V[[\x93\x92PPPV[_\x80a5\xC8a+\x19V[\x90P_a5\xD3a+@V[\x90P_\x81Q\x11\x15a5\xEFW\x80\x80Q\x90` \x01 \x92PPPa61V[_\x82_\x01T\x90P_\x80\x1B\x81\x14a6\nW\x80\x93PPPPa61V[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x80a6>a+\x19V[\x90P_a6Ia+\xDEV[\x90P_\x81Q\x11\x15a6eW\x80\x80Q\x90` \x01 \x92PPPa6\xA8V[_\x82`\x01\x01T\x90P_\x80\x1B\x81\x14a6\x81W\x80\x93PPPPa6\xA8V[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x81Q\x11\x15a6\xBDW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15a7&W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa7\x0BV[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a7K\x82a6\xEFV[a7U\x81\x85a6\xF9V[\x93Pa7e\x81\x85` \x86\x01a7\tV[a7n\x81a71V[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra7\x91\x81\x84a7AV[\x90P\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x81\x90P\x91\x90PV[a7\xBC\x81a7\xAAV[\x81\x14a7\xC6W_\x80\xFD[PV[_\x815\x90Pa7\xD7\x81a7\xB3V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a7\xF2Wa7\xF1a7\xA2V[[_a7\xFF\x84\x82\x85\x01a7\xC9V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a8Z\x82a81V[\x90P\x91\x90PV[a8j\x81a8PV[\x82RPPV[_a8{\x83\x83a8aV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a8\x9D\x82a8\x08V[a8\xA7\x81\x85a8\x12V[\x93Pa8\xB2\x83a8\"V[\x80_[\x83\x81\x10\x15a8\xE2W\x81Qa8\xC9\x88\x82a8pV[\x97Pa8\xD4\x83a8\x87V[\x92PP`\x01\x81\x01\x90Pa8\xB5V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra9\x07\x81\x84a8\x93V[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x02\x81\x10a9MWa9La9\x0FV[[PV[_\x81\x90Pa9]\x82a9<V[\x91\x90PV[_a9l\x82a9PV[\x90P\x91\x90PV[a9|\x81a9bV[\x82RPPV[_` \x82\x01\x90Pa9\x95_\x83\x01\x84a9sV[\x92\x91PPV[`\x02\x81\x10a9\xA7W_\x80\xFD[PV[_\x815\x90Pa9\xB8\x81a9\x9BV[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a9\xD4Wa9\xD3a7\xA2V[[_a9\xE1\x85\x82\x86\x01a7\xC9V[\x92PP` a9\xF2\x85\x82\x86\x01a9\xAAV[\x91PP\x92P\x92\x90PV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12a:\x1DWa:\x1Ca9\xFCV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a::Wa:9a:\0V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a:VWa:Ua:\x04V[[\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12a:rWa:qa9\xFCV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:\x8FWa:\x8Ea:\0V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a:\xABWa:\xAAa:\x04V[[\x92P\x92\x90PV[_\x80_\x80_``\x86\x88\x03\x12\x15a:\xCBWa:\xCAa7\xA2V[[_a:\xD8\x88\x82\x89\x01a7\xC9V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:\xF9Wa:\xF8a7\xA6V[[a;\x05\x88\x82\x89\x01a:\x08V[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a;(Wa;'a7\xA6V[[a;4\x88\x82\x89\x01a:]V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[a;L\x81a8PV[\x81\x14a;VW_\x80\xFD[PV[_\x815\x90Pa;g\x81a;CV[\x92\x91PPV[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a;\xA7\x82a71V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a;\xC6Wa;\xC5a;qV[[\x80`@RPPPV[_a;\xD8a7\x99V[\x90Pa;\xE4\x82\x82a;\x9EV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a<\x03Wa<\x02a;qV[[a<\x0C\x82a71V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a<9a<4\x84a;\xE9V[a;\xCFV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a<UWa<Ta;mV[[a<`\x84\x82\x85a<\x19V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a<|Wa<{a9\xFCV[[\x815a<\x8C\x84\x82` \x86\x01a<'V[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a<\xABWa<\xAAa7\xA2V[[_a<\xB8\x85\x82\x86\x01a;YV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a<\xD9Wa<\xD8a7\xA6V[[a<\xE5\x85\x82\x86\x01a<hV[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[a=\x01\x81a<\xEFV[\x82RPPV[_` \x82\x01\x90Pa=\x1A_\x83\x01\x84a<\xF8V[\x92\x91PPV[_\x80_`@\x84\x86\x03\x12\x15a=7Wa=6a7\xA2V[[_a=D\x86\x82\x87\x01a7\xC9V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=eWa=da7\xA6V[[a=q\x86\x82\x87\x01a:]V[\x92P\x92PP\x92P\x92P\x92V[_\x80_\x80_``\x86\x88\x03\x12\x15a=\x96Wa=\x95a7\xA2V[[_a=\xA3\x88\x82\x89\x01a7\xC9V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=\xC4Wa=\xC3a7\xA6V[[a=\xD0\x88\x82\x89\x01a:]V[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=\xF3Wa=\xF2a7\xA6V[[a=\xFF\x88\x82\x89\x01a:]V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[_\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a>B\x81a>\x0EV[\x82RPPV[a>Q\x81a7\xAAV[\x82RPPV[a>`\x81a8PV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a>\x98\x81a7\xAAV[\x82RPPV[_a>\xA9\x83\x83a>\x8FV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a>\xCB\x82a>fV[a>\xD5\x81\x85a>pV[\x93Pa>\xE0\x83a>\x80V[\x80_[\x83\x81\x10\x15a?\x10W\x81Qa>\xF7\x88\x82a>\x9EV[\x97Pa?\x02\x83a>\xB5V[\x92PP`\x01\x81\x01\x90Pa>\xE3V[P\x85\x93PPPP\x92\x91PPV[_`\xE0\x82\x01\x90Pa?0_\x83\x01\x8Aa>9V[\x81\x81\x03` \x83\x01Ra?B\x81\x89a7AV[\x90P\x81\x81\x03`@\x83\x01Ra?V\x81\x88a7AV[\x90Pa?e``\x83\x01\x87a>HV[a?r`\x80\x83\x01\x86a>WV[a?\x7F`\xA0\x83\x01\x85a<\xF8V[\x81\x81\x03`\xC0\x83\x01Ra?\x91\x81\x84a>\xC1V[\x90P\x98\x97PPPPPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a?\xE2\x82a6\xEFV[a?\xEC\x81\x85a?\xC8V[\x93Pa?\xFC\x81\x85` \x86\x01a7\tV[a@\x05\x81a71V[\x84\x01\x91PP\x92\x91PPV[_a@\x1B\x83\x83a?\xD8V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a@9\x82a?\x9FV[a@C\x81\x85a?\xA9V[\x93P\x83` \x82\x02\x85\x01a@U\x85a?\xB9V[\x80_[\x85\x81\x10\x15a@\x90W\x84\x84\x03\x89R\x81Qa@q\x85\x82a@\x10V[\x94Pa@|\x83a@#V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa@XV[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[`\x02\x81\x10a@\xDCWa@\xDBa9\x0FV[[PV[_\x81\x90Pa@\xEC\x82a@\xCBV[\x91\x90PV[_a@\xFB\x82a@\xDFV[\x90P\x91\x90PV[aA\x0B\x81a@\xF1V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aA5\x82aA\x11V[aA?\x81\x85aA\x1BV[\x93PaAO\x81\x85` \x86\x01a7\tV[aAX\x81a71V[\x84\x01\x91PP\x92\x91PPV[_`@\x83\x01_\x83\x01QaAx_\x86\x01\x82aA\x02V[P` \x83\x01Q\x84\x82\x03` \x86\x01RaA\x90\x82\x82aA+V[\x91PP\x80\x91PP\x92\x91PPV[_aA\xA8\x83\x83aAcV[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aA\xC6\x82a@\xA2V[aA\xD0\x81\x85a@\xACV[\x93P\x83` \x82\x02\x85\x01aA\xE2\x85a@\xBCV[\x80_[\x85\x81\x10\x15aB\x1DW\x84\x84\x03\x89R\x81QaA\xFE\x85\x82aA\x9DV[\x94PaB\t\x83aA\xB0V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaA\xE5V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01RaBG\x81\x85a@/V[\x90P\x81\x81\x03` \x83\x01RaB[\x81\x84aA\xBCV[\x90P\x93\x92PPPV[_` \x82\x01\x90PaBw_\x83\x01\x84a>HV[\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aB\x97\x82aA\x11V[aB\xA1\x81\x85aB}V[\x93PaB\xB1\x81\x85` \x86\x01a7\tV[aB\xBA\x81a71V[\x84\x01\x91PP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01RaB\xDD\x81\x85a@/V[\x90P\x81\x81\x03` \x83\x01RaB\xF1\x81\x84aB\x8DV[\x90P\x93\x92PPPV[_` \x82\x84\x03\x12\x15aC\x0FWaC\x0Ea7\xA2V[[_aC\x1C\x84\x82\x85\x01a9\xAAV[\x91PP\x92\x91PPV[_\x81\x90P\x92\x91PPV[_aC9\x82a6\xEFV[aCC\x81\x85aC%V[\x93PaCS\x81\x85` \x86\x01a7\tV[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aC\x93`\x02\x83aC%V[\x91PaC\x9E\x82aC_V[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aC\xDD`\x01\x83aC%V[\x91PaC\xE8\x82aC\xA9V[`\x01\x82\x01\x90P\x91\x90PV[_aC\xFE\x82\x87aC/V[\x91PaD\t\x82aC\x87V[\x91PaD\x15\x82\x86aC/V[\x91PaD \x82aC\xD1V[\x91PaD,\x82\x85aC/V[\x91PaD7\x82aC\xD1V[\x91PaDC\x82\x84aC/V[\x91P\x81\x90P\x95\x94PPPPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[aDm\x81aDQV[\x82RPPV[_` \x82\x01\x90PaD\x86_\x83\x01\x84aDdV[\x92\x91PPV[_\x81Q\x90PaD\x9A\x81a;CV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aD\xB5WaD\xB4a7\xA2V[[_aD\xC2\x84\x82\x85\x01aD\x8CV[\x91PP\x92\x91PPV[_` \x82\x01\x90PaD\xDE_\x83\x01\x84a>WV[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_aE\x1B\x82a7\xAAV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03aEMWaELaD\xE4V[[`\x01\x82\x01\x90P\x91\x90PV[_``\x82\x01\x90PaEk_\x83\x01\x86a>HV[aEx` \x83\x01\x85a>HV[aE\x85`@\x83\x01\x84a9sV[\x94\x93PPPPV[_\x81\x15\x15\x90P\x91\x90PV[aE\xA1\x81aE\x8DV[\x81\x14aE\xABW_\x80\xFD[PV[_\x81Q\x90PaE\xBC\x81aE\x98V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aE\xD7WaE\xD6a7\xA2V[[_aE\xE4\x84\x82\x85\x01aE\xAEV[\x91PP\x92\x91PPV[_`@\x82\x01\x90PaF\0_\x83\x01\x85a>HV[aF\r` \x83\x01\x84a>WV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x825`\x01`@\x03\x836\x03\x03\x81\x12aFhWaFgaFAV[[\x80\x83\x01\x91PP\x92\x91PPV[`\x02\x81\x10aF\x80W_\x80\xFD[PV[_\x815aF\x8F\x81aFtV[\x80\x91PP\x91\x90PV[_\x81_\x1B\x90P\x91\x90PV[_`\xFFaF\xAF\x84aF\x98V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_aF\xCF\x82a@\xDFV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[aF\xE8\x82aF\xC5V[aF\xFBaF\xF4\x82aF\xD6V[\x83TaF\xA3V[\x82UPPPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aG\x1EWaG\x1DaFAV[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aG@WaG?aFEV[[` \x83\x01\x92P`\x01\x82\x026\x03\x83\x13\x15aG\\WaG[aFIV[[P\x92P\x92\x90PV[_\x82\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80aG\xB2W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aG\xC5WaG\xC4aGnV[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02aH'\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82aG\xECV[aH1\x86\x83aG\xECV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_aHlaHgaHb\x84a7\xAAV[aHIV[a7\xAAV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[aH\x85\x83aHRV[aH\x99aH\x91\x82aHsV[\x84\x84TaG\xF8V[\x82UPPPPV[_\x90V[aH\xADaH\xA1V[aH\xB8\x81\x84\x84aH|V[PPPV[[\x81\x81\x10\x15aH\xDBWaH\xD0_\x82aH\xA5V[`\x01\x81\x01\x90PaH\xBEV[PPV[`\x1F\x82\x11\x15aI WaH\xF1\x81aG\xCBV[aH\xFA\x84aG\xDDV[\x81\x01` \x85\x10\x15aI\tW\x81\x90P[aI\x1DaI\x15\x85aG\xDDV[\x83\x01\x82aH\xBDV[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_aI@_\x19\x84`\x08\x02aI%V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_aIX\x83\x83aI1V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[aIr\x83\x83aGdV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aI\x8BWaI\x8Aa;qV[[aI\x95\x82TaG\x9BV[aI\xA0\x82\x82\x85aH\xDFV[_`\x1F\x83\x11`\x01\x81\x14aI\xCDW_\x84\x15aI\xBBW\x82\x87\x015\x90P[aI\xC5\x85\x82aIMV[\x86UPaJ,V[`\x1F\x19\x84\x16aI\xDB\x86aG\xCBV[_[\x82\x81\x10\x15aJ\x02W\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PaI\xDDV[\x86\x83\x10\x15aJ\x1FW\x84\x89\x015aJ\x1B`\x1F\x89\x16\x82aI1V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[aJ@\x83\x83\x83aIhV[PPPV[_\x81\x01_\x83\x01\x80aJU\x81aF\x83V[\x90PaJa\x81\x84aF\xDFV[PPP`\x01\x81\x01` \x83\x01aJv\x81\x85aG\x02V[aJ\x81\x81\x83\x86aJ5V[PPPPPPV[aJ\x93\x82\x82aJEV[PPV[_\x80\xFD[_\x80\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aJ\xB9WaJ\xB8a;qV[[aJ\xC2\x82a71V[\x90P` \x81\x01\x90P\x91\x90PV[_aJ\xE1aJ\xDC\x84aJ\x9FV[a;\xCFV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15aJ\xFDWaJ\xFCa;mV[[aK\x08\x84\x82\x85a7\tV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aK$WaK#a9\xFCV[[\x81QaK4\x84\x82` \x86\x01aJ\xCFV[\x91PP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15aKRWaKQaJ\x97V[[aK\\`\x80a;\xCFV[\x90P_aKk\x84\x82\x85\x01aD\x8CV[_\x83\x01RP` aK~\x84\x82\x85\x01aD\x8CV[` \x83\x01RP`@\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aK\xA2WaK\xA1aJ\x9BV[[aK\xAE\x84\x82\x85\x01aK\x10V[`@\x83\x01RP``\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aK\xD2WaK\xD1aJ\x9BV[[aK\xDE\x84\x82\x85\x01aK\x10V[``\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15aK\xFFWaK\xFEa7\xA2V[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aL\x1CWaL\x1Ba7\xA6V[[aL(\x84\x82\x85\x01aK=V[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[_\x815\x90PaLH\x81aFtV[\x92\x91PPV[_aL\\` \x84\x01\x84aL:V[\x90P\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aL\x8CWaL\x8BaLlV[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aL\xB4WaL\xB3aLdV[[`\x01\x82\x026\x03\x83\x13\x15aL\xCAWaL\xC9aLhV[[P\x92P\x92\x90PV[_aL\xDD\x83\x85aA\x1BV[\x93PaL\xEA\x83\x85\x84a<\x19V[aL\xF3\x83a71V[\x84\x01\x90P\x93\x92PPPV[_`@\x83\x01aM\x0F_\x84\x01\x84aLNV[aM\x1B_\x86\x01\x82aA\x02V[PaM)` \x84\x01\x84aLpV[\x85\x83\x03` \x87\x01RaM<\x83\x82\x84aL\xD2V[\x92PPP\x80\x91PP\x92\x91PPV[_aMU\x83\x83aL\xFEV[\x90P\x92\x91PPV[_\x825`\x01`@\x03\x836\x03\x03\x81\x12aMxWaMwaLlV[[\x82\x81\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aM\x9B\x83\x85a@\xACV[\x93P\x83` \x84\x02\x85\x01aM\xAD\x84aL1V[\x80_[\x87\x81\x10\x15aM\xF0W\x84\x84\x03\x89RaM\xC7\x82\x84aM]V[aM\xD1\x85\x82aMJV[\x94PaM\xDC\x83aM\x84V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaM\xB0V[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_``\x82\x01\x90PaN\x15_\x83\x01\x87a>HV[\x81\x81\x03` \x83\x01RaN'\x81\x86a@/V[\x90P\x81\x81\x03`@\x83\x01RaN<\x81\x84\x86aM\x90V[\x90P\x95\x94PPPPPV[_`@\x82\x01\x90PaNZ_\x83\x01\x85a>HV[aNg` \x83\x01\x84a>HV[\x93\x92PPPV[_aNy\x83\x85aB}V[\x93PaN\x86\x83\x85\x84a<\x19V[aN\x8F\x83a71V[\x84\x01\x90P\x93\x92PPPV[_``\x82\x01\x90PaN\xAD_\x83\x01\x87a>HV[\x81\x81\x03` \x83\x01RaN\xBF\x81\x86a@/V[\x90P\x81\x81\x03`@\x83\x01RaN\xD4\x81\x84\x86aNnV[\x90P\x95\x94PPPPPV[\x7FEIP712: Uninitialized\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aO\x13`\x15\x83a6\xF9V[\x91PaO\x1E\x82aN\xDFV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaO@\x81aO\x07V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aO\x89WaO\x88a7\xA2V[[_aO\x96\x84\x82\x85\x01aL:V[\x91PP\x92\x91PPV[_\x81\x90P\x92\x91PPV[_aO\xB4\x83\x85aO\x9FV[\x93PaO\xC1\x83\x85\x84a<\x19V[\x82\x84\x01\x90P\x93\x92PPPV[_aO\xD9\x82\x84\x86aO\xA9V[\x91P\x81\x90P\x93\x92PPPV[aO\xEE\x81a@\xF1V[\x82RPPV[_``\x82\x01\x90PaP\x07_\x83\x01\x86a<\xF8V[aP\x14` \x83\x01\x85aO\xE5V[aP!`@\x83\x01\x84a<\xF8V[\x94\x93PPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[aPU\x81a<\xEFV[\x82RPPV[_aPf\x83\x83aPLV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aP\x88\x82aP)V[aP\x92\x81\x85aP3V[\x93PaP\x9D\x83aP=V[\x80_[\x83\x81\x10\x15aP\xCDW\x81QaP\xB4\x88\x82aP[V[\x97PaP\xBF\x83aPrV[\x92PP`\x01\x81\x01\x90PaP\xA0V[P\x85\x93PPPP\x92\x91PPV[_aP\xE5\x82\x84aP~V[\x91P\x81\x90P\x92\x91PPV[_`\x80\x82\x01\x90PaQ\x03_\x83\x01\x87a<\xF8V[aQ\x10` \x83\x01\x86a>HV[aQ\x1D`@\x83\x01\x85a>HV[aQ*``\x83\x01\x84a<\xF8V[\x95\x94PPPPPV[_\x81Q\x90PaQA\x81a7\xB3V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aQ\\WaQ[a7\xA2V[[_aQi\x84\x82\x85\x01aQ3V[\x91PP\x92\x91PPV[aQ{\x81a<\xEFV[\x81\x14aQ\x85W_\x80\xFD[PV[_\x81Q\x90PaQ\x96\x81aQrV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aQ\xB1WaQ\xB0a7\xA2V[[_aQ\xBE\x84\x82\x85\x01aQ\x88V[\x91PP\x92\x91PPV[_`@\x82\x01\x90PaQ\xDA_\x83\x01\x85a<\xF8V[aQ\xE7` \x83\x01\x84a>HV[\x93\x92PPPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[`\x1F\x82\x11\x15aRAWaR\x12\x81aQ\xEEV[aR\x1B\x84aG\xDDV[\x81\x01` \x85\x10\x15aR*W\x81\x90P[aR>aR6\x85aG\xDDV[\x83\x01\x82aH\xBDV[PP[PPPV[aRO\x82a6\xEFV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aRhWaRga;qV[[aRr\x82TaG\x9BV[aR}\x82\x82\x85aR\0V[_` \x90P`\x1F\x83\x11`\x01\x81\x14aR\xAEW_\x84\x15aR\x9CW\x82\x87\x01Q\x90P[aR\xA6\x85\x82aIMV[\x86UPaS\rV[`\x1F\x19\x84\x16aR\xBC\x86aQ\xEEV[_[\x82\x81\x10\x15aR\xE3W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PaR\xBEV[\x86\x83\x10\x15aS\0W\x84\x89\x01QaR\xFC`\x1F\x89\x16\x82aI1V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_aS\x1F\x82aA\x11V[aS)\x81\x85aO\x9FV[\x93PaS9\x81\x85` \x86\x01a7\tV[\x80\x84\x01\x91PP\x92\x91PPV[_aSP\x82\x84aS\x15V[\x91P\x81\x90P\x92\x91PPV[_`\xA0\x82\x01\x90PaSn_\x83\x01\x88a<\xF8V[aS{` \x83\x01\x87a<\xF8V[aS\x88`@\x83\x01\x86a<\xF8V[aS\x95``\x83\x01\x85a>HV[aS\xA2`\x80\x83\x01\x84a>WV[\x96\x95PPPPPPV[_`\xFF\x82\x16\x90P\x91\x90PV[aS\xC1\x81aS\xACV[\x82RPPV[_`\x80\x82\x01\x90PaS\xDA_\x83\x01\x87a<\xF8V[aS\xE7` \x83\x01\x86aS\xB8V[aS\xF4`@\x83\x01\x85a<\xF8V[aT\x01``\x83\x01\x84a<\xF8V[\x95\x94PPPPPV\xFECrsgenVerification(uint256 crsId,uint256 maxBitLength,bytes crsDigest)PrepKeygenVerification(uint256 prepKeygenId)KeygenVerification(uint256 prepKeygenId,uint256 keyId,KeyDigest[] keyDigests)KeyDigest(uint8 keyType,bytes digest)KeyDigest(uint8 keyType,bytes digest)",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405260043610610108575f3560e01c8063589adb0e11610094578063ad3cb1cc11610063578063ad3cb1cc14610353578063baff211e1461037d578063c55b8724146103a7578063caa367db146103e4578063d52f10eb1461040c57610108565b8063589adb0e1461029657806362978787146102be57806384b0196e146102e6578063936608ae1461031657610108565b80633c02f834116100db5780633c02f834146101c457806345af261b146101ec5780634610ffe8146102285780634f1ef2861461025057806352d1902d1461026c57610108565b80630d8e6e2c1461010c57806316c713d91461013657806319f4f6321461017257806339f73810146101ae575b5f80fd5b348015610117575f80fd5b50610120610436565b60405161012d9190613779565b60405180910390f35b348015610141575f80fd5b5061015c600480360381019061015791906137dd565b6104b1565b60405161016991906138ef565b60405180910390f35b34801561017d575f80fd5b50610198600480360381019061019391906137dd565b610582565b6040516101a59190613982565b60405180910390f35b3480156101b9575f80fd5b506101c261062f565b005b3480156101cf575f80fd5b506101ea60048036038101906101e591906139be565b61087e565b005b3480156101f7575f80fd5b50610212600480360381019061020d91906137dd565b610a2d565b60405161021f9190613982565b60405180910390f35b348015610233575f80fd5b5061024e60048036038101906102499190613ab2565b610ac2565b005b61026a60048036038101906102659190613c95565b611007565b005b348015610277575f80fd5b50610280611026565b60405161028d9190613d07565b60405180910390f35b3480156102a1575f80fd5b506102bc60048036038101906102b79190613d20565b611057565b005b3480156102c9575f80fd5b506102e460048036038101906102df9190613d7d565b6113a9565b005b3480156102f1575f80fd5b506102fa61188a565b60405161030d9796959493929190613f1d565b60405180910390f35b348015610321575f80fd5b5061033c600480360381019061033791906137dd565b611993565b60405161034a92919061422f565b60405180910390f35b34801561035e575f80fd5b50610367611d3a565b6040516103749190613779565b60405180910390f35b348015610388575f80fd5b50610391611d73565b60405161039e9190614264565b60405180910390f35b3480156103b2575f80fd5b506103cd60048036038101906103c891906137dd565b611d8a565b6040516103db9291906142c5565b60405180910390f35b3480156103ef575f80fd5b5061040a600480360381019061040591906142fa565b61209c565b005b348015610417575f80fd5b50610420612286565b60405161042d9190614264565b60405180910390f35b60606040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506104775f61229d565b610481600161229d565b61048a5f61229d565b60405160200161049d94939291906143f3565b604051602081830303815290604052905090565b60605f6104bc612367565b90505f816003015f8581526020019081526020015f20549050816002015f8581526020019081526020015f205f8281526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561057457602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001906001019080831161052b575b505050505092505050919050565b5f8061058c612367565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff166105ef57826040517f84de13310000000000000000000000000000000000000000000000000000000081526004016105e69190614264565b60405180910390fd5b5f816006015f8581526020019081526020015f2054905081600d015f8281526020019081526020015f205f9054906101000a900460ff1692505050919050565b600161063961238e565b67ffffffffffffffff161461067a576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60025f6106856123b2565b9050805f0160089054906101000a900460ff16806106cd57508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15610704576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055506107bd6040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506040518060400160405280600181526020017f31000000000000000000000000000000000000000000000000000000000000008152506123d9565b5f6107c6612367565b905060f8600360058111156107de576107dd61390f565b5b901b816004018190555060f8600460058111156107fe576107fd61390f565b5b901b816005018190555060f860058081111561081d5761081c61390f565b5b901b8160090181905550505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2826040516108729190614473565b60405180910390a15050565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156108db573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108ff91906144a0565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461096e57336040517f0e56cf3d00000000000000000000000000000000000000000000000000000000815260040161096591906144cb565b60405180910390fd5b5f610977612367565b9050806009015f81548092919061098d90614511565b91905055505f816009015490508382600a015f8381526020019081526020015f20819055508282600d015f8381526020019081526020015f205f6101000a81548160ff021916908360018111156109e7576109e661390f565b5b02179055507f3f038f6f88cb3031b7718588403a2ec220576a868be07dde4c02b846ca352ef5818585604051610a1f93929190614558565b60405180910390a150505050565b5f80610a37612367565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff16610a9a57826040517fda32d00f000000000000000000000000000000000000000000000000000000008152600401610a919190614264565b60405180910390fd5b80600d015f8481526020019081526020015f205f9054906101000a900460ff16915050919050565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663e5275eaf336040518263ffffffff1660e01b8152600401610b0f91906144cb565b602060405180830381865afa158015610b2a573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b4e91906145c2565b610b8f57336040517faee86323000000000000000000000000000000000000000000000000000000008152600401610b8691906144cb565b60405180910390fd5b5f610b98612367565b90505f816006015f8881526020019081526020015f205490505f610bbe828989896123ef565b90505f610bcc8287876125c6565b9050835f015f8a81526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615610c6c5788816040517f98fb957d000000000000000000000000000000000000000000000000000000008152600401610c639291906145ed565b60405180910390fd5b6001845f015f8b81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f846002015f8b81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505f81805490509050856001015f8c81526020019081526020015f205f9054906101000a900460ff16158015610d8f5750610d8e8161262b565b5b15610ffa576001866001015f8d81526020019081526020015f205f6101000a81548160ff0219169083151502179055505f5b8a8a9050811015610e4557866007015f8d81526020019081526020015f208b8b83818110610df257610df1614614565b5b9050602002810190610e04919061464d565b908060018154018082558091505060019003905f5260205f2090600202015f909190919091508181610e369190614a89565b50508080600101915050610dc1565b5083866003015f8d81526020019081526020015f20819055508a86600801819055505f8167ffffffffffffffff811115610e8257610e81613b71565b5b604051908082528060200260200182016040528015610eb557816020015b6060815260200190600190039081610ea05790505b5090505f5b82811015610fba5773a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663e3b2a874858381548110610f0557610f04614614565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b8152600401610f4991906144cb565b5f60405180830381865afa158015610f63573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f82011682018060405250810190610f8b9190614bea565b60600151828281518110610fa257610fa1614614565b5b60200260200101819052508080600101915050610eba565b507feb85c26dbcad46b80a68a0f24cce7c2c90f0a1faded84184138839fc9e80a25b8c828d8d604051610ff09493929190614e02565b60405180910390a1505b5050505050505050505050565b61100f6126bc565b611018826127a2565b6110228282612895565b5050565b5f61102f6129b3565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663e5275eaf336040518263ffffffff1660e01b81526004016110a491906144cb565b602060405180830381865afa1580156110bf573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110e391906145c2565b61112457336040517faee8632300000000000000000000000000000000000000000000000000000000815260040161111b91906144cb565b60405180910390fd5b5f61112d612367565b90505f61113985612a3a565b90505f6111478286866125c6565b9050825f015f8781526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16156111e75785816040517f33ca1fe30000000000000000000000000000000000000000000000000000000081526004016111de9291906145ed565b60405180910390fd5b6001835f015f8881526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f836002015f8881526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550836001015f8881526020019081526020015f205f9054906101000a900460ff161580156113065750611305818054905061262b565b5b156113a0576001846001015f8981526020019081526020015f205f6101000a81548160ff02191690831515021790555082846003015f8981526020019081526020015f20819055505f846006015f8981526020019081526020015f205490507f78b179176d1f19d7c28e80823deba2624da2ca2ec64b1701f3632a87c9aedc928882604051611396929190614e47565b60405180910390a1505b50505050505050565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663e5275eaf336040518263ffffffff1660e01b81526004016113f691906144cb565b602060405180830381865afa158015611411573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061143591906145c2565b61147657336040517faee8632300000000000000000000000000000000000000000000000000000000815260040161146d91906144cb565b60405180910390fd5b5f61147f612367565b90505f81600a015f8881526020019081526020015f205490505f6114a588838989612a92565b90505f6114b38287876125c6565b9050835f015f8a81526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16156115535788816040517ffcf5a6e900000000000000000000000000000000000000000000000000000000815260040161154a9291906145ed565b60405180910390fd5b6001845f015f8b81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f846002015f8b81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505f81805490509050856001015f8c81526020019081526020015f205f9054906101000a900460ff1615801561167657506116758161262b565b5b1561187d576001866001015f8d81526020019081526020015f205f6101000a81548160ff021916908315150217905550898987600b015f8e81526020019081526020015f2091826116c8929190614968565b5083866003015f8d81526020019081526020015f20819055508a86600c01819055505f8167ffffffffffffffff81111561170557611704613b71565b5b60405190808252806020026020018201604052801561173857816020015b60608152602001906001900390816117235790505b5090505f5b8281101561183d5773a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663e3b2a87485838154811061178857611787614614565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b81526004016117cc91906144cb565b5f60405180830381865afa1580156117e6573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f8201168201806040525081019061180e9190614bea565b6060015182828151811061182557611824614614565b5b6020026020010181905250808060010191505061173d565b507f2258b73faed33fb2e2ea454403bef974920caf682ab3a723484fcf67553b16a28c828d8d6040516118739493929190614e9a565b60405180910390a1505b5050505050505050505050565b5f6060805f805f60605f61189c612b19565b90505f801b815f01541480156118b757505f801b8160010154145b6118f6576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016118ed90614f29565b60405180910390fd5b6118fe612b40565b611906612bde565b46305f801b5f67ffffffffffffffff81111561192557611924613b71565b5b6040519080825280602002602001820160405280156119535781602001602082028036833780820191505090505b507f0f0000000000000000000000000000000000000000000000000000000000000095949392919097509750975097509750975097505090919293949596565b6060805f61199f612367565b9050806001015f8581526020019081526020015f205f9054906101000a900460ff16611a0257836040517f84de13310000000000000000000000000000000000000000000000000000000081526004016119f99190614264565b60405180910390fd5b5f816003015f8681526020019081526020015f205490505f826002015f8781526020019081526020015f205f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020018280548015611ab957602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311611a70575b505050505090505f815190505f8167ffffffffffffffff811115611ae057611adf613b71565b5b604051908082528060200260200182016040528015611b1357816020015b6060815260200190600190039081611afe5790505b5090505f5b82811015611bf85773a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663e3b2a874858381518110611b6357611b62614614565b5b60200260200101516040518263ffffffff1660e01b8152600401611b8791906144cb565b5f60405180830381865afa158015611ba1573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f82011682018060405250810190611bc99190614bea565b60600151828281518110611be057611bdf614614565b5b60200260200101819052508080600101915050611b18565b5080856007015f8a81526020019081526020015f2080805480602002602001604051908101604052809291908181526020015f905b82821015611d26578382905f5260205f2090600202016040518060400160405290815f82015f9054906101000a900460ff166001811115611c7157611c7061390f565b5b6001811115611c8357611c8261390f565b5b8152602001600182018054611c979061479b565b80601f0160208091040260200160405190810160405280929190818152602001828054611cc39061479b565b8015611d0e5780601f10611ce557610100808354040283529160200191611d0e565b820191905f5260205f20905b815481529060010190602001808311611cf157829003601f168201915b50505050508152505081526020019060010190611c2d565b505050509050965096505050505050915091565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b5f80611d7d612367565b905080600c015491505090565b6060805f611d96612367565b9050806001015f8581526020019081526020015f205f9054906101000a900460ff16611df957836040517fda32d00f000000000000000000000000000000000000000000000000000000008152600401611df09190614264565b60405180910390fd5b5f816003015f8681526020019081526020015f205490505f826002015f8781526020019081526020015f205f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020018280548015611eb057602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311611e67575b505050505090505f815190505f8167ffffffffffffffff811115611ed757611ed6613b71565b5b604051908082528060200260200182016040528015611f0a57816020015b6060815260200190600190039081611ef55790505b5090505f5b82811015611fef5773a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663e3b2a874858381518110611f5a57611f59614614565b5b60200260200101516040518263ffffffff1660e01b8152600401611f7e91906144cb565b5f60405180830381865afa158015611f98573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f82011682018060405250810190611fc09190614bea565b60600151828281518110611fd757611fd6614614565b5b60200260200101819052508080600101915050611f0f565b508085600b015f8a81526020019081526020015f208080546120109061479b565b80601f016020809104026020016040519081016040528092919081815260200182805461203c9061479b565b80156120875780601f1061205e57610100808354040283529160200191612087565b820191905f5260205f20905b81548152906001019060200180831161206a57829003601f168201915b50505050509050965096505050505050915091565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156120f9573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061211d91906144a0565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461218c57336040517f0e56cf3d00000000000000000000000000000000000000000000000000000000815260040161218391906144cb565b60405180910390fd5b5f612195612367565b9050806004015f8154809291906121ab90614511565b91905055505f81600401549050816005015f8154809291906121cc90614511565b91905055505f8260050154905080836006015f8481526020019081526020015f208190555081836006015f8381526020019081526020015f20819055505f8484600d015f8581526020019081526020015f205f6101000a81548160ff0219169083600181111561223f5761223e61390f565b5b02179055507f02024007d96574dbc9d11328bfee9893e7c7bb4ef4aa806df33bfdf454eb5e6083828760405161227793929190614558565b60405180910390a15050505050565b5f80612290612367565b9050806008015491505090565b60605f60016122ab84612c7c565b0190505f8167ffffffffffffffff8111156122c9576122c8613b71565b5b6040519080825280601f01601f1916602001820160405280156122fb5781602001600182028036833780820191505090505b5090505f82602001820190505b60011561235c578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a858161235157612350614f47565b5b0494505f8503612308575b819350505050919050565b5f7f0b8fdb1f0a6356dd20a6cbc6f9668fac23b85f96575d10e333e603faa794ac00905090565b5f6123976123b2565b5f015f9054906101000a900467ffffffffffffffff16905090565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b6123e1612dcd565b6123eb8282612e0d565b5050565b5f808383905067ffffffffffffffff81111561240e5761240d613b71565b5b60405190808252806020026020018201604052801561243c5781602001602082028036833780820191505090505b5090505f5b84849050811015612540576040518060600160405280602581526020016154ef602591398051906020012085858381811061247f5761247e614614565b5b9050602002810190612491919061464d565b5f0160208101906124a29190614f74565b8686848181106124b5576124b4614614565b5b90506020028101906124c7919061464d565b80602001906124d69190614702565b6040516124e4929190614fcd565b60405180910390206040516020016124fe93929190614ff4565b6040516020818303038152906040528051906020012082828151811061252757612526614614565b5b6020026020010181815250508080600101915050612441565b506125bb6040518060a001604052806072815260200161547d607291398051906020012087878460405160200161257791906150da565b604051602081830303815290604052805190602001206040516020016125a094939291906150f0565b60405160208183030381529060405280519060200120612e5e565b915050949350505050565b5f806126158585858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050612e77565b905061262081612ea1565b809150509392505050565b5f8073a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663b4722bc46040518163ffffffff1660e01b8152600401602060405180830381865afa15801561268a573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906126ae9190615147565b905080831015915050919050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff16148061276957507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16612750612f71565b73ffffffffffffffffffffffffffffffffffffffff1614155b156127a0576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156127ff573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061282391906144a0565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461289257336040517f0e56cf3d00000000000000000000000000000000000000000000000000000000815260040161288991906144cb565b60405180910390fd5b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa9250505080156128fd57506040513d601f19601f820116820180604052508101906128fa919061519c565b60015b61293e57816040517f4c9c8ce300000000000000000000000000000000000000000000000000000000815260040161293591906144cb565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b81146129a457806040517faa1d49a400000000000000000000000000000000000000000000000000000000815260040161299b9190613d07565b60405180910390fd5b6129ae8383612fc4565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff1614612a38576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f612a8b6040518060600160405280602c8152602001615451602c91398051906020012083604051602001612a709291906151c7565b60405160208183030381529060405280519060200120612e5e565b9050919050565b5f612b0f60405180608001604052806046815260200161540b604691398051906020012086868686604051602001612acb929190614fcd565b60405160208183030381529060405280519060200120604051602001612af494939291906150f0565b60405160208183030381529060405280519060200120612e5e565b9050949350505050565b5f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100905090565b60605f612b4b612b19565b9050806002018054612b5c9061479b565b80601f0160208091040260200160405190810160405280929190818152602001828054612b889061479b565b8015612bd35780601f10612baa57610100808354040283529160200191612bd3565b820191905f5260205f20905b815481529060010190602001808311612bb657829003601f168201915b505050505091505090565b60605f612be9612b19565b9050806003018054612bfa9061479b565b80601f0160208091040260200160405190810160405280929190818152602001828054612c269061479b565b8015612c715780601f10612c4857610100808354040283529160200191612c71565b820191905f5260205f20905b815481529060010190602001808311612c5457829003601f168201915b505050505091505090565b5f805f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f0100000000000000008310612cd8577a184f03e93ff9f4daa797ed6e38ed64bf6a1f0100000000000000008381612cce57612ccd614f47565b5b0492506040810190505b6d04ee2d6d415b85acef81000000008310612d15576d04ee2d6d415b85acef81000000008381612d0b57612d0a614f47565b5b0492506020810190505b662386f26fc100008310612d4457662386f26fc100008381612d3a57612d39614f47565b5b0492506010810190505b6305f5e1008310612d6d576305f5e1008381612d6357612d62614f47565b5b0492506008810190505b6127108310612d92576127108381612d8857612d87614f47565b5b0492506004810190505b60648310612db55760648381612dab57612daa614f47565b5b0492506002810190505b600a8310612dc4576001810190505b80915050919050565b612dd5613036565b612e0b576040517fd7e6bcf800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b612e15612dcd565b5f612e1e612b19565b905082816002019081612e319190615246565b5081816003019081612e439190615246565b505f801b815f01819055505f801b8160010181905550505050565b5f612e70612e6a613054565b83613062565b9050919050565b5f805f80612e8586866130a2565b925092509250612e9582826130f7565b82935050505092915050565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663203d0114826040518263ffffffff1660e01b8152600401612eee91906144cb565b602060405180830381865afa158015612f09573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612f2d91906145c2565b612f6e57806040517f2a7c6ef6000000000000000000000000000000000000000000000000000000008152600401612f6591906144cb565b60405180910390fd5b50565b5f612f9d7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b613259565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b612fcd82613262565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f8151111561302957613023828261332b565b50613032565b6130316133ab565b5b5050565b5f61303f6123b2565b5f0160089054906101000a900460ff16905090565b5f61305d6133e7565b905090565b5f6040517f190100000000000000000000000000000000000000000000000000000000000081528360028201528260228201526042812091505092915050565b5f805f60418451036130e2575f805f602087015192506040870151915060608701515f1a90506130d48882858561344a565b9550955095505050506130f0565b5f600285515f1b9250925092505b9250925092565b5f600381111561310a5761310961390f565b5b82600381111561311d5761311c61390f565b5b031561325557600160038111156131375761313661390f565b5b82600381111561314a5761314961390f565b5b03613181576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600260038111156131955761319461390f565b5b8260038111156131a8576131a761390f565b5b036131ec57805f1c6040517ffce698f70000000000000000000000000000000000000000000000000000000081526004016131e39190614264565b60405180910390fd5b6003808111156131ff576131fe61390f565b5b8260038111156132125761321161390f565b5b0361325457806040517fd78bce0c00000000000000000000000000000000000000000000000000000000815260040161324b9190613d07565b60405180910390fd5b5b5050565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b036132bd57806040517f4c9c8ce30000000000000000000000000000000000000000000000000000000081526004016132b491906144cb565b60405180910390fd5b806132e97f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b613259565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f808473ffffffffffffffffffffffffffffffffffffffff16846040516133549190615345565b5f60405180830381855af49150503d805f811461338c576040519150601f19603f3d011682016040523d82523d5f602084013e613391565b606091505b50915091506133a1858383613531565b9250505092915050565b5f3411156133e5576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f6134116135be565b613419613634565b463060405160200161342f95949392919061535b565b60405160208183030381529060405280519060200120905090565b5f805f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c1115613486575f600385925092509250613527565b5f6001888888886040515f81526020016040526040516134a994939291906153c7565b6020604051602081039080840390855afa1580156134c9573d5f803e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff160361351a575f60015f801b93509350935050613527565b805f805f1b935093509350505b9450945094915050565b60608261354657613541826136ab565b6135b6565b5f825114801561356c57505f8473ffffffffffffffffffffffffffffffffffffffff163b145b156135ae57836040517f9996b3150000000000000000000000000000000000000000000000000000000081526004016135a591906144cb565b60405180910390fd5b8190506135b7565b5b9392505050565b5f806135c8612b19565b90505f6135d3612b40565b90505f815111156135ef57808051906020012092505050613631565b5f825f015490505f801b811461360a57809350505050613631565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f8061363e612b19565b90505f613649612bde565b90505f81511115613665578080519060200120925050506136a8565b5f826001015490505f801b8114613681578093505050506136a8565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f815111156136bd5780518082602001fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f81519050919050565b5f82825260208201905092915050565b5f5b8381101561372657808201518184015260208101905061370b565b5f8484015250505050565b5f601f19601f8301169050919050565b5f61374b826136ef565b61375581856136f9565b9350613765818560208601613709565b61376e81613731565b840191505092915050565b5f6020820190508181035f8301526137918184613741565b905092915050565b5f604051905090565b5f80fd5b5f80fd5b5f819050919050565b6137bc816137aa565b81146137c6575f80fd5b50565b5f813590506137d7816137b3565b92915050565b5f602082840312156137f2576137f16137a2565b5b5f6137ff848285016137c9565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61385a82613831565b9050919050565b61386a81613850565b82525050565b5f61387b8383613861565b60208301905092915050565b5f602082019050919050565b5f61389d82613808565b6138a78185613812565b93506138b283613822565b805f5b838110156138e25781516138c98882613870565b97506138d483613887565b9250506001810190506138b5565b5085935050505092915050565b5f6020820190508181035f8301526139078184613893565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b6002811061394d5761394c61390f565b5b50565b5f81905061395d8261393c565b919050565b5f61396c82613950565b9050919050565b61397c81613962565b82525050565b5f6020820190506139955f830184613973565b92915050565b600281106139a7575f80fd5b50565b5f813590506139b88161399b565b92915050565b5f80604083850312156139d4576139d36137a2565b5b5f6139e1858286016137c9565b92505060206139f2858286016139aa565b9150509250929050565b5f80fd5b5f80fd5b5f80fd5b5f8083601f840112613a1d57613a1c6139fc565b5b8235905067ffffffffffffffff811115613a3a57613a39613a00565b5b602083019150836020820283011115613a5657613a55613a04565b5b9250929050565b5f8083601f840112613a7257613a716139fc565b5b8235905067ffffffffffffffff811115613a8f57613a8e613a00565b5b602083019150836001820283011115613aab57613aaa613a04565b5b9250929050565b5f805f805f60608688031215613acb57613aca6137a2565b5b5f613ad8888289016137c9565b955050602086013567ffffffffffffffff811115613af957613af86137a6565b5b613b0588828901613a08565b9450945050604086013567ffffffffffffffff811115613b2857613b276137a6565b5b613b3488828901613a5d565b92509250509295509295909350565b613b4c81613850565b8114613b56575f80fd5b50565b5f81359050613b6781613b43565b92915050565b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b613ba782613731565b810181811067ffffffffffffffff82111715613bc657613bc5613b71565b5b80604052505050565b5f613bd8613799565b9050613be48282613b9e565b919050565b5f67ffffffffffffffff821115613c0357613c02613b71565b5b613c0c82613731565b9050602081019050919050565b828183375f83830152505050565b5f613c39613c3484613be9565b613bcf565b905082815260208101848484011115613c5557613c54613b6d565b5b613c60848285613c19565b509392505050565b5f82601f830112613c7c57613c7b6139fc565b5b8135613c8c848260208601613c27565b91505092915050565b5f8060408385031215613cab57613caa6137a2565b5b5f613cb885828601613b59565b925050602083013567ffffffffffffffff811115613cd957613cd86137a6565b5b613ce585828601613c68565b9150509250929050565b5f819050919050565b613d0181613cef565b82525050565b5f602082019050613d1a5f830184613cf8565b92915050565b5f805f60408486031215613d3757613d366137a2565b5b5f613d44868287016137c9565b935050602084013567ffffffffffffffff811115613d6557613d646137a6565b5b613d7186828701613a5d565b92509250509250925092565b5f805f805f60608688031215613d9657613d956137a2565b5b5f613da3888289016137c9565b955050602086013567ffffffffffffffff811115613dc457613dc36137a6565b5b613dd088828901613a5d565b9450945050604086013567ffffffffffffffff811115613df357613df26137a6565b5b613dff88828901613a5d565b92509250509295509295909350565b5f7fff0000000000000000000000000000000000000000000000000000000000000082169050919050565b613e4281613e0e565b82525050565b613e51816137aa565b82525050565b613e6081613850565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b613e98816137aa565b82525050565b5f613ea98383613e8f565b60208301905092915050565b5f602082019050919050565b5f613ecb82613e66565b613ed58185613e70565b9350613ee083613e80565b805f5b83811015613f10578151613ef78882613e9e565b9750613f0283613eb5565b925050600181019050613ee3565b5085935050505092915050565b5f60e082019050613f305f83018a613e39565b8181036020830152613f428189613741565b90508181036040830152613f568188613741565b9050613f656060830187613e48565b613f726080830186613e57565b613f7f60a0830185613cf8565b81810360c0830152613f918184613ec1565b905098975050505050505050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f82825260208201905092915050565b5f613fe2826136ef565b613fec8185613fc8565b9350613ffc818560208601613709565b61400581613731565b840191505092915050565b5f61401b8383613fd8565b905092915050565b5f602082019050919050565b5f61403982613f9f565b6140438185613fa9565b93508360208202850161405585613fb9565b805f5b8581101561409057848403895281516140718582614010565b945061407c83614023565b925060208a01995050600181019050614058565b50829750879550505050505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b600281106140dc576140db61390f565b5b50565b5f8190506140ec826140cb565b919050565b5f6140fb826140df565b9050919050565b61410b816140f1565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f61413582614111565b61413f818561411b565b935061414f818560208601613709565b61415881613731565b840191505092915050565b5f604083015f8301516141785f860182614102565b5060208301518482036020860152614190828261412b565b9150508091505092915050565b5f6141a88383614163565b905092915050565b5f602082019050919050565b5f6141c6826140a2565b6141d081856140ac565b9350836020820285016141e2856140bc565b805f5b8581101561421d57848403895281516141fe858261419d565b9450614209836141b0565b925060208a019950506001810190506141e5565b50829750879550505050505092915050565b5f6040820190508181035f830152614247818561402f565b9050818103602083015261425b81846141bc565b90509392505050565b5f6020820190506142775f830184613e48565b92915050565b5f82825260208201905092915050565b5f61429782614111565b6142a1818561427d565b93506142b1818560208601613709565b6142ba81613731565b840191505092915050565b5f6040820190508181035f8301526142dd818561402f565b905081810360208301526142f1818461428d565b90509392505050565b5f6020828403121561430f5761430e6137a2565b5b5f61431c848285016139aa565b91505092915050565b5f81905092915050565b5f614339826136ef565b6143438185614325565b9350614353818560208601613709565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f614393600283614325565b915061439e8261435f565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f6143dd600183614325565b91506143e8826143a9565b600182019050919050565b5f6143fe828761432f565b915061440982614387565b9150614415828661432f565b9150614420826143d1565b915061442c828561432f565b9150614437826143d1565b9150614443828461432f565b915081905095945050505050565b5f67ffffffffffffffff82169050919050565b61446d81614451565b82525050565b5f6020820190506144865f830184614464565b92915050565b5f8151905061449a81613b43565b92915050565b5f602082840312156144b5576144b46137a2565b5b5f6144c28482850161448c565b91505092915050565b5f6020820190506144de5f830184613e57565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61451b826137aa565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff820361454d5761454c6144e4565b5b600182019050919050565b5f60608201905061456b5f830186613e48565b6145786020830185613e48565b6145856040830184613973565b949350505050565b5f8115159050919050565b6145a18161458d565b81146145ab575f80fd5b50565b5f815190506145bc81614598565b92915050565b5f602082840312156145d7576145d66137a2565b5b5f6145e4848285016145ae565b91505092915050565b5f6040820190506146005f830185613e48565b61460d6020830184613e57565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b5f8235600160400383360303811261466857614667614641565b5b80830191505092915050565b60028110614680575f80fd5b50565b5f813561468f81614674565b80915050919050565b5f815f1b9050919050565b5f60ff6146af84614698565b9350801983169250808416831791505092915050565b5f6146cf826140df565b9050919050565b5f819050919050565b6146e8826146c5565b6146fb6146f4826146d6565b83546146a3565b8255505050565b5f808335600160200384360303811261471e5761471d614641565b5b80840192508235915067ffffffffffffffff8211156147405761473f614645565b5b60208301925060018202360383131561475c5761475b614649565b5b509250929050565b5f82905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f60028204905060018216806147b257607f821691505b6020821081036147c5576147c461476e565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f600883026148277fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff826147ec565b61483186836147ec565b95508019841693508086168417925050509392505050565b5f819050919050565b5f61486c614867614862846137aa565b614849565b6137aa565b9050919050565b5f819050919050565b61488583614852565b61489961489182614873565b8484546147f8565b825550505050565b5f90565b6148ad6148a1565b6148b881848461487c565b505050565b5b818110156148db576148d05f826148a5565b6001810190506148be565b5050565b601f821115614920576148f1816147cb565b6148fa846147dd565b81016020851015614909578190505b61491d614915856147dd565b8301826148bd565b50505b505050565b5f82821c905092915050565b5f6149405f1984600802614925565b1980831691505092915050565b5f6149588383614931565b9150826002028217905092915050565b6149728383614764565b67ffffffffffffffff81111561498b5761498a613b71565b5b614995825461479b565b6149a08282856148df565b5f601f8311600181146149cd575f84156149bb578287013590505b6149c5858261494d565b865550614a2c565b601f1984166149db866147cb565b5f5b82811015614a02578489013582556001820191506020850194506020810190506149dd565b86831015614a1f5784890135614a1b601f891682614931565b8355505b6001600288020188555050505b50505050505050565b614a40838383614968565b505050565b5f81015f830180614a5581614683565b9050614a6181846146df565b5050506001810160208301614a768185614702565b614a81818386614a35565b505050505050565b614a938282614a45565b5050565b5f80fd5b5f80fd5b5f67ffffffffffffffff821115614ab957614ab8613b71565b5b614ac282613731565b9050602081019050919050565b5f614ae1614adc84614a9f565b613bcf565b905082815260208101848484011115614afd57614afc613b6d565b5b614b08848285613709565b509392505050565b5f82601f830112614b2457614b236139fc565b5b8151614b34848260208601614acf565b91505092915050565b5f60808284031215614b5257614b51614a97565b5b614b5c6080613bcf565b90505f614b6b8482850161448c565b5f830152506020614b7e8482850161448c565b602083015250604082015167ffffffffffffffff811115614ba257614ba1614a9b565b5b614bae84828501614b10565b604083015250606082015167ffffffffffffffff811115614bd257614bd1614a9b565b5b614bde84828501614b10565b60608301525092915050565b5f60208284031215614bff57614bfe6137a2565b5b5f82015167ffffffffffffffff811115614c1c57614c1b6137a6565b5b614c2884828501614b3d565b91505092915050565b5f819050919050565b5f81359050614c4881614674565b92915050565b5f614c5c6020840184614c3a565b905092915050565b5f80fd5b5f80fd5b5f80fd5b5f8083356001602003843603038112614c8c57614c8b614c6c565b5b83810192508235915060208301925067ffffffffffffffff821115614cb457614cb3614c64565b5b600182023603831315614cca57614cc9614c68565b5b509250929050565b5f614cdd838561411b565b9350614cea838584613c19565b614cf383613731565b840190509392505050565b5f60408301614d0f5f840184614c4e565b614d1b5f860182614102565b50614d296020840184614c70565b8583036020870152614d3c838284614cd2565b925050508091505092915050565b5f614d558383614cfe565b905092915050565b5f82356001604003833603038112614d7857614d77614c6c565b5b82810191505092915050565b5f602082019050919050565b5f614d9b83856140ac565b935083602084028501614dad84614c31565b805f5b87811015614df0578484038952614dc78284614d5d565b614dd18582614d4a565b9450614ddc83614d84565b925060208a01995050600181019050614db0565b50829750879450505050509392505050565b5f606082019050614e155f830187613e48565b8181036020830152614e27818661402f565b90508181036040830152614e3c818486614d90565b905095945050505050565b5f604082019050614e5a5f830185613e48565b614e676020830184613e48565b9392505050565b5f614e79838561427d565b9350614e86838584613c19565b614e8f83613731565b840190509392505050565b5f606082019050614ead5f830187613e48565b8181036020830152614ebf818661402f565b90508181036040830152614ed4818486614e6e565b905095945050505050565b7f4549503731323a20556e696e697469616c697a656400000000000000000000005f82015250565b5f614f136015836136f9565b9150614f1e82614edf565b602082019050919050565b5f6020820190508181035f830152614f4081614f07565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f60208284031215614f8957614f886137a2565b5b5f614f9684828501614c3a565b91505092915050565b5f81905092915050565b5f614fb48385614f9f565b9350614fc1838584613c19565b82840190509392505050565b5f614fd9828486614fa9565b91508190509392505050565b614fee816140f1565b82525050565b5f6060820190506150075f830186613cf8565b6150146020830185614fe5565b6150216040830184613cf8565b949350505050565b5f81519050919050565b5f81905092915050565b5f819050602082019050919050565b61505581613cef565b82525050565b5f615066838361504c565b60208301905092915050565b5f602082019050919050565b5f61508882615029565b6150928185615033565b935061509d8361503d565b805f5b838110156150cd5781516150b4888261505b565b97506150bf83615072565b9250506001810190506150a0565b5085935050505092915050565b5f6150e5828461507e565b915081905092915050565b5f6080820190506151035f830187613cf8565b6151106020830186613e48565b61511d6040830185613e48565b61512a6060830184613cf8565b95945050505050565b5f81519050615141816137b3565b92915050565b5f6020828403121561515c5761515b6137a2565b5b5f61516984828501615133565b91505092915050565b61517b81613cef565b8114615185575f80fd5b50565b5f8151905061519681615172565b92915050565b5f602082840312156151b1576151b06137a2565b5b5f6151be84828501615188565b91505092915050565b5f6040820190506151da5f830185613cf8565b6151e76020830184613e48565b9392505050565b5f819050815f5260205f209050919050565b601f82111561524157615212816151ee565b61521b846147dd565b8101602085101561522a578190505b61523e615236856147dd565b8301826148bd565b50505b505050565b61524f826136ef565b67ffffffffffffffff81111561526857615267613b71565b5b615272825461479b565b61527d828285615200565b5f60209050601f8311600181146152ae575f841561529c578287015190505b6152a6858261494d565b86555061530d565b601f1984166152bc866151ee565b5f5b828110156152e3578489015182556001820191506020850194506020810190506152be565b8683101561530057848901516152fc601f891682614931565b8355505b6001600288020188555050505b505050505050565b5f61531f82614111565b6153298185614f9f565b9350615339818560208601613709565b80840191505092915050565b5f6153508284615315565b915081905092915050565b5f60a08201905061536e5f830188613cf8565b61537b6020830187613cf8565b6153886040830186613cf8565b6153956060830185613e48565b6153a26080830184613e57565b9695505050505050565b5f60ff82169050919050565b6153c1816153ac565b82525050565b5f6080820190506153da5f830187613cf8565b6153e760208301866153b8565b6153f46040830185613cf8565b6154016060830184613cf8565b9594505050505056fe43727367656e566572696669636174696f6e2875696e743235362063727349642c75696e74323536206d61784269744c656e6774682c62797465732063727344696765737429507265704b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e4964294b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e49642c75696e74323536206b657949642c4b65794469676573745b5d206b657944696765737473294b65794469676573742875696e7438206b6579547970652c627974657320646967657374294b65794469676573742875696e7438206b6579547970652c62797465732064696765737429
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01\x08W_5`\xE0\x1C\x80cX\x9A\xDB\x0E\x11a\0\x94W\x80c\xAD<\xB1\xCC\x11a\0cW\x80c\xAD<\xB1\xCC\x14a\x03SW\x80c\xBA\xFF!\x1E\x14a\x03}W\x80c\xC5[\x87$\x14a\x03\xA7W\x80c\xCA\xA3g\xDB\x14a\x03\xE4W\x80c\xD5/\x10\xEB\x14a\x04\x0CWa\x01\x08V[\x80cX\x9A\xDB\x0E\x14a\x02\x96W\x80cb\x97\x87\x87\x14a\x02\xBEW\x80c\x84\xB0\x19n\x14a\x02\xE6W\x80c\x93f\x08\xAE\x14a\x03\x16Wa\x01\x08V[\x80c<\x02\xF84\x11a\0\xDBW\x80c<\x02\xF84\x14a\x01\xC4W\x80cE\xAF&\x1B\x14a\x01\xECW\x80cF\x10\xFF\xE8\x14a\x02(W\x80cO\x1E\xF2\x86\x14a\x02PW\x80cR\xD1\x90-\x14a\x02lWa\x01\x08V[\x80c\r\x8En,\x14a\x01\x0CW\x80c\x16\xC7\x13\xD9\x14a\x016W\x80c\x19\xF4\xF62\x14a\x01rW\x80c9\xF78\x10\x14a\x01\xAEW[_\x80\xFD[4\x80\x15a\x01\x17W_\x80\xFD[Pa\x01 a\x046V[`@Qa\x01-\x91\x90a7yV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01AW_\x80\xFD[Pa\x01\\`\x04\x806\x03\x81\x01\x90a\x01W\x91\x90a7\xDDV[a\x04\xB1V[`@Qa\x01i\x91\x90a8\xEFV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01}W_\x80\xFD[Pa\x01\x98`\x04\x806\x03\x81\x01\x90a\x01\x93\x91\x90a7\xDDV[a\x05\x82V[`@Qa\x01\xA5\x91\x90a9\x82V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xB9W_\x80\xFD[Pa\x01\xC2a\x06/V[\0[4\x80\x15a\x01\xCFW_\x80\xFD[Pa\x01\xEA`\x04\x806\x03\x81\x01\x90a\x01\xE5\x91\x90a9\xBEV[a\x08~V[\0[4\x80\x15a\x01\xF7W_\x80\xFD[Pa\x02\x12`\x04\x806\x03\x81\x01\x90a\x02\r\x91\x90a7\xDDV[a\n-V[`@Qa\x02\x1F\x91\x90a9\x82V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x023W_\x80\xFD[Pa\x02N`\x04\x806\x03\x81\x01\x90a\x02I\x91\x90a:\xB2V[a\n\xC2V[\0[a\x02j`\x04\x806\x03\x81\x01\x90a\x02e\x91\x90a<\x95V[a\x10\x07V[\0[4\x80\x15a\x02wW_\x80\xFD[Pa\x02\x80a\x10&V[`@Qa\x02\x8D\x91\x90a=\x07V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xA1W_\x80\xFD[Pa\x02\xBC`\x04\x806\x03\x81\x01\x90a\x02\xB7\x91\x90a= V[a\x10WV[\0[4\x80\x15a\x02\xC9W_\x80\xFD[Pa\x02\xE4`\x04\x806\x03\x81\x01\x90a\x02\xDF\x91\x90a=}V[a\x13\xA9V[\0[4\x80\x15a\x02\xF1W_\x80\xFD[Pa\x02\xFAa\x18\x8AV[`@Qa\x03\r\x97\x96\x95\x94\x93\x92\x91\x90a?\x1DV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03!W_\x80\xFD[Pa\x03<`\x04\x806\x03\x81\x01\x90a\x037\x91\x90a7\xDDV[a\x19\x93V[`@Qa\x03J\x92\x91\x90aB/V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03^W_\x80\xFD[Pa\x03ga\x1D:V[`@Qa\x03t\x91\x90a7yV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x88W_\x80\xFD[Pa\x03\x91a\x1DsV[`@Qa\x03\x9E\x91\x90aBdV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xB2W_\x80\xFD[Pa\x03\xCD`\x04\x806\x03\x81\x01\x90a\x03\xC8\x91\x90a7\xDDV[a\x1D\x8AV[`@Qa\x03\xDB\x92\x91\x90aB\xC5V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xEFW_\x80\xFD[Pa\x04\n`\x04\x806\x03\x81\x01\x90a\x04\x05\x91\x90aB\xFAV[a \x9CV[\0[4\x80\x15a\x04\x17W_\x80\xFD[Pa\x04 a\"\x86V[`@Qa\x04-\x91\x90aBdV[`@Q\x80\x91\x03\x90\xF3[```@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x04w_a\"\x9DV[a\x04\x81`\x01a\"\x9DV[a\x04\x8A_a\"\x9DV[`@Q` \x01a\x04\x9D\x94\x93\x92\x91\x90aC\xF3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[``_a\x04\xBCa#gV[\x90P_\x81`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\x02\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x82\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x05tW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x05+W[PPPPP\x92PPP\x91\x90PV[_\x80a\x05\x8Ca#gV[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x05\xEFW\x82`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05\xE6\x91\x90aBdV[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\r\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x92PPP\x91\x90PV[`\x01a\x069a#\x8EV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x06zW`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02_a\x06\x85a#\xB2V[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x06\xCDWP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x07\x04W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x07\xBD`@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa#\xD9V[_a\x07\xC6a#gV[\x90P`\xF8`\x03`\x05\x81\x11\x15a\x07\xDEWa\x07\xDDa9\x0FV[[\x90\x1B\x81`\x04\x01\x81\x90UP`\xF8`\x04`\x05\x81\x11\x15a\x07\xFEWa\x07\xFDa9\x0FV[[\x90\x1B\x81`\x05\x01\x81\x90UP`\xF8`\x05\x80\x81\x11\x15a\x08\x1DWa\x08\x1Ca9\x0FV[[\x90\x1B\x81`\t\x01\x81\x90UPP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x08r\x91\x90aDsV[`@Q\x80\x91\x03\x90\xA1PPV[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xDBW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xFF\x91\x90aD\xA0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\tnW3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\te\x91\x90aD\xCBV[`@Q\x80\x91\x03\x90\xFD[_a\twa#gV[\x90P\x80`\t\x01_\x81T\x80\x92\x91\x90a\t\x8D\x90aE\x11V[\x91\x90PUP_\x81`\t\x01T\x90P\x83\x82`\n\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82\x82`\r\x01_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a\t\xE7Wa\t\xE6a9\x0FV[[\x02\x17\x90UP\x7F?\x03\x8Fo\x88\xCB01\xB7q\x85\x88@:.\xC2 Wj\x86\x8B\xE0}\xDEL\x02\xB8F\xCA5.\xF5\x81\x85\x85`@Qa\n\x1F\x93\x92\x91\x90aEXV[`@Q\x80\x91\x03\x90\xA1PPPPV[_\x80a\n7a#gV[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\n\x9AW\x82`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\x91\x91\x90aBdV[`@Q\x80\x91\x03\x90\xFD[\x80`\r\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE5'^\xAF3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\x0F\x91\x90aD\xCBV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B*W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BN\x91\x90aE\xC2V[a\x0B\x8FW3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B\x86\x91\x90aD\xCBV[`@Q\x80\x91\x03\x90\xFD[_a\x0B\x98a#gV[\x90P_\x81`\x06\x01_\x88\x81R` \x01\x90\x81R` \x01_ T\x90P_a\x0B\xBE\x82\x89\x89\x89a#\xEFV[\x90P_a\x0B\xCC\x82\x87\x87a%\xC6V[\x90P\x83_\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x0ClW\x88\x81`@Q\x7F\x98\xFB\x95}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0Cc\x92\x91\x90aE\xEDV[`@Q\x80\x91\x03\x90\xFD[`\x01\x84_\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x84`\x02\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_\x81\x80T\x90P\x90P\x85`\x01\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\r\x8FWPa\r\x8E\x81a&+V[[\x15a\x0F\xFAW`\x01\x86`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_[\x8A\x8A\x90P\x81\x10\x15a\x0EEW\x86`\x07\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x8B\x8B\x83\x81\x81\x10a\r\xF2Wa\r\xF1aF\x14V[[\x90P` \x02\x81\x01\x90a\x0E\x04\x91\x90aFMV[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P\x81\x81a\x0E6\x91\x90aJ\x89V[PP\x80\x80`\x01\x01\x91PPa\r\xC1V[P\x83\x86`\x03\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x8A\x86`\x08\x01\x81\x90UP_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\x82Wa\x0E\x81a;qV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\xB5W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0E\xA0W\x90P[P\x90P_[\x82\x81\x10\x15a\x0F\xBAWs\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3\xB2\xA8t\x85\x83\x81T\x81\x10a\x0F\x05Wa\x0F\x04aF\x14V[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0FI\x91\x90aD\xCBV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FcW=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x8B\x91\x90aK\xEAV[``\x01Q\x82\x82\x81Q\x81\x10a\x0F\xA2Wa\x0F\xA1aF\x14V[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa\x0E\xBAV[P\x7F\xEB\x85\xC2m\xBC\xADF\xB8\nh\xA0\xF2L\xCE|,\x90\xF0\xA1\xFA\xDE\xD8A\x84\x13\x889\xFC\x9E\x80\xA2[\x8C\x82\x8D\x8D`@Qa\x0F\xF0\x94\x93\x92\x91\x90aN\x02V[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPPPPPV[a\x10\x0Fa&\xBCV[a\x10\x18\x82a'\xA2V[a\x10\"\x82\x82a(\x95V[PPV[_a\x10/a)\xB3V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE5'^\xAF3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\xA4\x91\x90aD\xCBV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xBFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xE3\x91\x90aE\xC2V[a\x11$W3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\x1B\x91\x90aD\xCBV[`@Q\x80\x91\x03\x90\xFD[_a\x11-a#gV[\x90P_a\x119\x85a*:V[\x90P_a\x11G\x82\x86\x86a%\xC6V[\x90P\x82_\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x11\xE7W\x85\x81`@Q\x7F3\xCA\x1F\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\xDE\x92\x91\x90aE\xEDV[`@Q\x80\x91\x03\x90\xFD[`\x01\x83_\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x83`\x02\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x83`\x01\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x13\x06WPa\x13\x05\x81\x80T\x90Pa&+V[[\x15a\x13\xA0W`\x01\x84`\x01\x01_\x89\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x82\x84`\x03\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x81\x90UP_\x84`\x06\x01_\x89\x81R` \x01\x90\x81R` \x01_ T\x90P\x7Fx\xB1y\x17m\x1F\x19\xD7\xC2\x8E\x80\x82=\xEB\xA2bM\xA2\xCA.\xC6K\x17\x01\xF3c*\x87\xC9\xAE\xDC\x92\x88\x82`@Qa\x13\x96\x92\x91\x90aNGV[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPV[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE5'^\xAF3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\xF6\x91\x90aD\xCBV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\x11W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x145\x91\x90aE\xC2V[a\x14vW3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14m\x91\x90aD\xCBV[`@Q\x80\x91\x03\x90\xFD[_a\x14\x7Fa#gV[\x90P_\x81`\n\x01_\x88\x81R` \x01\x90\x81R` \x01_ T\x90P_a\x14\xA5\x88\x83\x89\x89a*\x92V[\x90P_a\x14\xB3\x82\x87\x87a%\xC6V[\x90P\x83_\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x15SW\x88\x81`@Q\x7F\xFC\xF5\xA6\xE9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15J\x92\x91\x90aE\xEDV[`@Q\x80\x91\x03\x90\xFD[`\x01\x84_\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x84`\x02\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_\x81\x80T\x90P\x90P\x85`\x01\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x16vWPa\x16u\x81a&+V[[\x15a\x18}W`\x01\x86`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x89\x89\x87`\x0B\x01_\x8E\x81R` \x01\x90\x81R` \x01_ \x91\x82a\x16\xC8\x92\x91\x90aIhV[P\x83\x86`\x03\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x8A\x86`\x0C\x01\x81\x90UP_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\x05Wa\x17\x04a;qV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x178W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x17#W\x90P[P\x90P_[\x82\x81\x10\x15a\x18=Ws\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3\xB2\xA8t\x85\x83\x81T\x81\x10a\x17\x88Wa\x17\x87aF\x14V[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\xCC\x91\x90aD\xCBV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xE6W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\x0E\x91\x90aK\xEAV[``\x01Q\x82\x82\x81Q\x81\x10a\x18%Wa\x18$aF\x14V[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa\x17=V[P\x7F\"X\xB7?\xAE\xD3?\xB2\xE2\xEAED\x03\xBE\xF9t\x92\x0C\xAFh*\xB3\xA7#HO\xCFgU;\x16\xA2\x8C\x82\x8D\x8D`@Qa\x18s\x94\x93\x92\x91\x90aN\x9AV[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPPPPPV[_``\x80_\x80_``_a\x18\x9Ca+\x19V[\x90P_\x80\x1B\x81_\x01T\x14\x80\x15a\x18\xB7WP_\x80\x1B\x81`\x01\x01T\x14[a\x18\xF6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18\xED\x90aO)V[`@Q\x80\x91\x03\x90\xFD[a\x18\xFEa+@V[a\x19\x06a+\xDEV[F0_\x80\x1B_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19%Wa\x19$a;qV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x19SW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94\x93\x92\x91\x90\x97P\x97P\x97P\x97P\x97P\x97P\x97PP\x90\x91\x92\x93\x94\x95\x96V[``\x80_a\x19\x9Fa#gV[\x90P\x80`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x1A\x02W\x83`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19\xF9\x91\x90aBdV[`@Q\x80\x91\x03\x90\xFD[_\x81`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x82`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1A\xB9W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x1ApW[PPPPP\x90P_\x81Q\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\xE0Wa\x1A\xDFa;qV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1B\x13W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1A\xFEW\x90P[P\x90P_[\x82\x81\x10\x15a\x1B\xF8Ws\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3\xB2\xA8t\x85\x83\x81Q\x81\x10a\x1BcWa\x1BbaF\x14V[[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B\x87\x91\x90aD\xCBV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xA1W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xC9\x91\x90aK\xEAV[``\x01Q\x82\x82\x81Q\x81\x10a\x1B\xE0Wa\x1B\xDFaF\x14V[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa\x1B\x18V[P\x80\x85`\x07\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x1D&W\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x1CqWa\x1Cpa9\x0FV[[`\x01\x81\x11\x15a\x1C\x83Wa\x1C\x82a9\x0FV[[\x81R` \x01`\x01\x82\x01\x80Ta\x1C\x97\x90aG\x9BV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1C\xC3\x90aG\x9BV[\x80\x15a\x1D\x0EW\x80`\x1F\x10a\x1C\xE5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1D\x0EV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1C\xF1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x1C-V[PPPP\x90P\x96P\x96PPPPPP\x91P\x91V[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[_\x80a\x1D}a#gV[\x90P\x80`\x0C\x01T\x91PP\x90V[``\x80_a\x1D\x96a#gV[\x90P\x80`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x1D\xF9W\x83`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1D\xF0\x91\x90aBdV[`@Q\x80\x91\x03\x90\xFD[_\x81`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x82`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1E\xB0W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x1EgW[PPPPP\x90P_\x81Q\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xD7Wa\x1E\xD6a;qV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1F\nW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1E\xF5W\x90P[P\x90P_[\x82\x81\x10\x15a\x1F\xEFWs\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE3\xB2\xA8t\x85\x83\x81Q\x81\x10a\x1FZWa\x1FYaF\x14V[[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1F~\x91\x90aD\xCBV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\x98W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xC0\x91\x90aK\xEAV[``\x01Q\x82\x82\x81Q\x81\x10a\x1F\xD7Wa\x1F\xD6aF\x14V[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa\x1F\x0FV[P\x80\x85`\x0B\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80\x80Ta \x10\x90aG\x9BV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta <\x90aG\x9BV[\x80\x15a \x87W\x80`\x1F\x10a ^Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a \x87V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a jW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x96P\x96PPPPPP\x91P\x91V[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \xF9W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\x1D\x91\x90aD\xA0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a!\x8CW3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a!\x83\x91\x90aD\xCBV[`@Q\x80\x91\x03\x90\xFD[_a!\x95a#gV[\x90P\x80`\x04\x01_\x81T\x80\x92\x91\x90a!\xAB\x90aE\x11V[\x91\x90PUP_\x81`\x04\x01T\x90P\x81`\x05\x01_\x81T\x80\x92\x91\x90a!\xCC\x90aE\x11V[\x91\x90PUP_\x82`\x05\x01T\x90P\x80\x83`\x06\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x81\x83`\x06\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UP_\x84\x84`\r\x01_\x85\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a\"?Wa\">a9\x0FV[[\x02\x17\x90UP\x7F\x02\x02@\x07\xD9et\xDB\xC9\xD1\x13(\xBF\xEE\x98\x93\xE7\xC7\xBBN\xF4\xAA\x80m\xF3;\xFD\xF4T\xEB^`\x83\x82\x87`@Qa\"w\x93\x92\x91\x90aEXV[`@Q\x80\x91\x03\x90\xA1PPPPPV[_\x80a\"\x90a#gV[\x90P\x80`\x08\x01T\x91PP\x90V[``_`\x01a\"\xAB\x84a,|V[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\xC9Wa\"\xC8a;qV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\"\xFBW\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x01\x82\x01\x90P[`\x01\x15a#\\W\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a#QWa#PaOGV[[\x04\x94P_\x85\x03a#\x08W[\x81\x93PPPP\x91\x90PV[_\x7F\x0B\x8F\xDB\x1F\ncV\xDD \xA6\xCB\xC6\xF9f\x8F\xAC#\xB8_\x96W]\x10\xE33\xE6\x03\xFA\xA7\x94\xAC\0\x90P\x90V[_a#\x97a#\xB2V[_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[a#\xE1a-\xCDV[a#\xEB\x82\x82a.\rV[PPV[_\x80\x83\x83\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$\x0EWa$\ra;qV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a$<W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_[\x84\x84\x90P\x81\x10\x15a%@W`@Q\x80``\x01`@R\x80`%\x81R` \x01aT\xEF`%\x919\x80Q\x90` \x01 \x85\x85\x83\x81\x81\x10a$\x7FWa$~aF\x14V[[\x90P` \x02\x81\x01\x90a$\x91\x91\x90aFMV[_\x01` \x81\x01\x90a$\xA2\x91\x90aOtV[\x86\x86\x84\x81\x81\x10a$\xB5Wa$\xB4aF\x14V[[\x90P` \x02\x81\x01\x90a$\xC7\x91\x90aFMV[\x80` \x01\x90a$\xD6\x91\x90aG\x02V[`@Qa$\xE4\x92\x91\x90aO\xCDV[`@Q\x80\x91\x03\x90 `@Q` \x01a$\xFE\x93\x92\x91\x90aO\xF4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x82\x81Q\x81\x10a%'Wa%&aF\x14V[[` \x02` \x01\x01\x81\x81RPP\x80\x80`\x01\x01\x91PPa$AV[Pa%\xBB`@Q\x80`\xA0\x01`@R\x80`r\x81R` \x01aT}`r\x919\x80Q\x90` \x01 \x87\x87\x84`@Q` \x01a%w\x91\x90aP\xDAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a%\xA0\x94\x93\x92\x91\x90aP\xF0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a.^V[\x91PP\x94\x93PPPPV[_\x80a&\x15\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa.wV[\x90Pa& \x81a.\xA1V[\x80\x91PP\x93\x92PPPV[_\x80s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xB4r+\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\x8AW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\xAE\x91\x90aQGV[\x90P\x80\x83\x10\x15\x91PP\x91\x90PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a'iWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a'Pa/qV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a'\xA0W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'\xFFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(#\x91\x90aD\xA0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a(\x92W3`@Q\x7F\x0EV\xCF=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a(\x89\x91\x90aD\xCBV[`@Q\x80\x91\x03\x90\xFD[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a(\xFDWP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\xFA\x91\x90aQ\x9CV[`\x01[a)>W\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a)5\x91\x90aD\xCBV[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14a)\xA4W\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a)\x9B\x91\x90a=\x07V[`@Q\x80\x91\x03\x90\xFD[a)\xAE\x83\x83a/\xC4V[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a*8W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_a*\x8B`@Q\x80``\x01`@R\x80`,\x81R` \x01aTQ`,\x919\x80Q\x90` \x01 \x83`@Q` \x01a*p\x92\x91\x90aQ\xC7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a.^V[\x90P\x91\x90PV[_a+\x0F`@Q\x80`\x80\x01`@R\x80`F\x81R` \x01aT\x0B`F\x919\x80Q\x90` \x01 \x86\x86\x86\x86`@Q` \x01a*\xCB\x92\x91\x90aO\xCDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a*\xF4\x94\x93\x92\x91\x90aP\xF0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a.^V[\x90P\x94\x93PPPPV[_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x90P\x90V[``_a+Ka+\x19V[\x90P\x80`\x02\x01\x80Ta+\\\x90aG\x9BV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta+\x88\x90aG\x9BV[\x80\x15a+\xD3W\x80`\x1F\x10a+\xAAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a+\xD3V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a+\xB6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[``_a+\xE9a+\x19V[\x90P\x80`\x03\x01\x80Ta+\xFA\x90aG\x9BV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta,&\x90aG\x9BV[\x80\x15a,qW\x80`\x1F\x10a,HWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a,qV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a,TW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[_\x80_\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a,\xD8Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81a,\xCEWa,\xCDaOGV[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a-\x15Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81a-\x0BWa-\naOGV[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10a-DWf#\x86\xF2o\xC1\0\0\x83\x81a-:Wa-9aOGV[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10a-mWc\x05\xF5\xE1\0\x83\x81a-cWa-baOGV[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10a-\x92Wa'\x10\x83\x81a-\x88Wa-\x87aOGV[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10a-\xB5W`d\x83\x81a-\xABWa-\xAAaOGV[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10a-\xC4W`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[a-\xD5a06V[a.\x0BW`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a.\x15a-\xCDV[_a.\x1Ea+\x19V[\x90P\x82\x81`\x02\x01\x90\x81a.1\x91\x90aRFV[P\x81\x81`\x03\x01\x90\x81a.C\x91\x90aRFV[P_\x80\x1B\x81_\x01\x81\x90UP_\x80\x1B\x81`\x01\x01\x81\x90UPPPPV[_a.pa.ja0TV[\x83a0bV[\x90P\x91\x90PV[_\x80_\x80a.\x85\x86\x86a0\xA2V[\x92P\x92P\x92Pa.\x95\x82\x82a0\xF7V[\x82\x93PPPP\x92\x91PPV[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c =\x01\x14\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a.\xEE\x91\x90aD\xCBV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/\tW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/-\x91\x90aE\xC2V[a/nW\x80`@Q\x7F*|n\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a/e\x91\x90aD\xCBV[`@Q\x80\x91\x03\x90\xFD[PV[_a/\x9D\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba2YV[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a/\xCD\x82a2bV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15a0)Wa0#\x82\x82a3+V[Pa02V[a01a3\xABV[[PPV[_a0?a#\xB2V[_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[_a0]a3\xE7V[\x90P\x90V[_`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x02\x82\x01R\x82`\"\x82\x01R`B\x81 \x91PP\x92\x91PPV[_\x80_`A\x84Q\x03a0\xE2W_\x80_` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90Pa0\xD4\x88\x82\x85\x85a4JV[\x95P\x95P\x95PPPPa0\xF0V[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15a1\nWa1\ta9\x0FV[[\x82`\x03\x81\x11\x15a1\x1DWa1\x1Ca9\x0FV[[\x03\x15a2UW`\x01`\x03\x81\x11\x15a17Wa16a9\x0FV[[\x82`\x03\x81\x11\x15a1JWa1Ia9\x0FV[[\x03a1\x81W`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15a1\x95Wa1\x94a9\x0FV[[\x82`\x03\x81\x11\x15a1\xA8Wa1\xA7a9\x0FV[[\x03a1\xECW\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a1\xE3\x91\x90aBdV[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15a1\xFFWa1\xFEa9\x0FV[[\x82`\x03\x81\x11\x15a2\x12Wa2\x11a9\x0FV[[\x03a2TW\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a2K\x91\x90a=\x07V[`@Q\x80\x91\x03\x90\xFD[[PPV[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a2\xBDW\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a2\xB4\x91\x90aD\xCBV[`@Q\x80\x91\x03\x90\xFD[\x80a2\xE9\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba2YV[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qa3T\x91\x90aSEV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a3\x8CW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a3\x91V[``\x91P[P\x91P\x91Pa3\xA1\x85\x83\x83a51V[\x92PPP\x92\x91PPV[_4\x11\x15a3\xE5W`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa4\x11a5\xBEV[a4\x19a64V[F0`@Q` \x01a4/\x95\x94\x93\x92\x91\x90aS[V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x80_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15a4\x86W_`\x03\x85\x92P\x92P\x92Pa5'V[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@Qa4\xA9\x94\x93\x92\x91\x90aS\xC7V[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a4\xC9W=_\x80>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a5\x1AW_`\x01_\x80\x1B\x93P\x93P\x93PPa5'V[\x80_\x80_\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[``\x82a5FWa5A\x82a6\xABV[a5\xB6V[_\x82Q\x14\x80\x15a5lWP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a5\xAEW\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a5\xA5\x91\x90aD\xCBV[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa5\xB7V[[\x93\x92PPPV[_\x80a5\xC8a+\x19V[\x90P_a5\xD3a+@V[\x90P_\x81Q\x11\x15a5\xEFW\x80\x80Q\x90` \x01 \x92PPPa61V[_\x82_\x01T\x90P_\x80\x1B\x81\x14a6\nW\x80\x93PPPPa61V[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x80a6>a+\x19V[\x90P_a6Ia+\xDEV[\x90P_\x81Q\x11\x15a6eW\x80\x80Q\x90` \x01 \x92PPPa6\xA8V[_\x82`\x01\x01T\x90P_\x80\x1B\x81\x14a6\x81W\x80\x93PPPPa6\xA8V[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x81Q\x11\x15a6\xBDW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15a7&W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa7\x0BV[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a7K\x82a6\xEFV[a7U\x81\x85a6\xF9V[\x93Pa7e\x81\x85` \x86\x01a7\tV[a7n\x81a71V[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra7\x91\x81\x84a7AV[\x90P\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x81\x90P\x91\x90PV[a7\xBC\x81a7\xAAV[\x81\x14a7\xC6W_\x80\xFD[PV[_\x815\x90Pa7\xD7\x81a7\xB3V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a7\xF2Wa7\xF1a7\xA2V[[_a7\xFF\x84\x82\x85\x01a7\xC9V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a8Z\x82a81V[\x90P\x91\x90PV[a8j\x81a8PV[\x82RPPV[_a8{\x83\x83a8aV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a8\x9D\x82a8\x08V[a8\xA7\x81\x85a8\x12V[\x93Pa8\xB2\x83a8\"V[\x80_[\x83\x81\x10\x15a8\xE2W\x81Qa8\xC9\x88\x82a8pV[\x97Pa8\xD4\x83a8\x87V[\x92PP`\x01\x81\x01\x90Pa8\xB5V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra9\x07\x81\x84a8\x93V[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x02\x81\x10a9MWa9La9\x0FV[[PV[_\x81\x90Pa9]\x82a9<V[\x91\x90PV[_a9l\x82a9PV[\x90P\x91\x90PV[a9|\x81a9bV[\x82RPPV[_` \x82\x01\x90Pa9\x95_\x83\x01\x84a9sV[\x92\x91PPV[`\x02\x81\x10a9\xA7W_\x80\xFD[PV[_\x815\x90Pa9\xB8\x81a9\x9BV[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a9\xD4Wa9\xD3a7\xA2V[[_a9\xE1\x85\x82\x86\x01a7\xC9V[\x92PP` a9\xF2\x85\x82\x86\x01a9\xAAV[\x91PP\x92P\x92\x90PV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12a:\x1DWa:\x1Ca9\xFCV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a::Wa:9a:\0V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a:VWa:Ua:\x04V[[\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12a:rWa:qa9\xFCV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:\x8FWa:\x8Ea:\0V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a:\xABWa:\xAAa:\x04V[[\x92P\x92\x90PV[_\x80_\x80_``\x86\x88\x03\x12\x15a:\xCBWa:\xCAa7\xA2V[[_a:\xD8\x88\x82\x89\x01a7\xC9V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:\xF9Wa:\xF8a7\xA6V[[a;\x05\x88\x82\x89\x01a:\x08V[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a;(Wa;'a7\xA6V[[a;4\x88\x82\x89\x01a:]V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[a;L\x81a8PV[\x81\x14a;VW_\x80\xFD[PV[_\x815\x90Pa;g\x81a;CV[\x92\x91PPV[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a;\xA7\x82a71V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a;\xC6Wa;\xC5a;qV[[\x80`@RPPPV[_a;\xD8a7\x99V[\x90Pa;\xE4\x82\x82a;\x9EV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a<\x03Wa<\x02a;qV[[a<\x0C\x82a71V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a<9a<4\x84a;\xE9V[a;\xCFV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a<UWa<Ta;mV[[a<`\x84\x82\x85a<\x19V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a<|Wa<{a9\xFCV[[\x815a<\x8C\x84\x82` \x86\x01a<'V[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a<\xABWa<\xAAa7\xA2V[[_a<\xB8\x85\x82\x86\x01a;YV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a<\xD9Wa<\xD8a7\xA6V[[a<\xE5\x85\x82\x86\x01a<hV[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[a=\x01\x81a<\xEFV[\x82RPPV[_` \x82\x01\x90Pa=\x1A_\x83\x01\x84a<\xF8V[\x92\x91PPV[_\x80_`@\x84\x86\x03\x12\x15a=7Wa=6a7\xA2V[[_a=D\x86\x82\x87\x01a7\xC9V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=eWa=da7\xA6V[[a=q\x86\x82\x87\x01a:]V[\x92P\x92PP\x92P\x92P\x92V[_\x80_\x80_``\x86\x88\x03\x12\x15a=\x96Wa=\x95a7\xA2V[[_a=\xA3\x88\x82\x89\x01a7\xC9V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=\xC4Wa=\xC3a7\xA6V[[a=\xD0\x88\x82\x89\x01a:]V[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=\xF3Wa=\xF2a7\xA6V[[a=\xFF\x88\x82\x89\x01a:]V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[_\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a>B\x81a>\x0EV[\x82RPPV[a>Q\x81a7\xAAV[\x82RPPV[a>`\x81a8PV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a>\x98\x81a7\xAAV[\x82RPPV[_a>\xA9\x83\x83a>\x8FV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a>\xCB\x82a>fV[a>\xD5\x81\x85a>pV[\x93Pa>\xE0\x83a>\x80V[\x80_[\x83\x81\x10\x15a?\x10W\x81Qa>\xF7\x88\x82a>\x9EV[\x97Pa?\x02\x83a>\xB5V[\x92PP`\x01\x81\x01\x90Pa>\xE3V[P\x85\x93PPPP\x92\x91PPV[_`\xE0\x82\x01\x90Pa?0_\x83\x01\x8Aa>9V[\x81\x81\x03` \x83\x01Ra?B\x81\x89a7AV[\x90P\x81\x81\x03`@\x83\x01Ra?V\x81\x88a7AV[\x90Pa?e``\x83\x01\x87a>HV[a?r`\x80\x83\x01\x86a>WV[a?\x7F`\xA0\x83\x01\x85a<\xF8V[\x81\x81\x03`\xC0\x83\x01Ra?\x91\x81\x84a>\xC1V[\x90P\x98\x97PPPPPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a?\xE2\x82a6\xEFV[a?\xEC\x81\x85a?\xC8V[\x93Pa?\xFC\x81\x85` \x86\x01a7\tV[a@\x05\x81a71V[\x84\x01\x91PP\x92\x91PPV[_a@\x1B\x83\x83a?\xD8V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a@9\x82a?\x9FV[a@C\x81\x85a?\xA9V[\x93P\x83` \x82\x02\x85\x01a@U\x85a?\xB9V[\x80_[\x85\x81\x10\x15a@\x90W\x84\x84\x03\x89R\x81Qa@q\x85\x82a@\x10V[\x94Pa@|\x83a@#V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa@XV[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[`\x02\x81\x10a@\xDCWa@\xDBa9\x0FV[[PV[_\x81\x90Pa@\xEC\x82a@\xCBV[\x91\x90PV[_a@\xFB\x82a@\xDFV[\x90P\x91\x90PV[aA\x0B\x81a@\xF1V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aA5\x82aA\x11V[aA?\x81\x85aA\x1BV[\x93PaAO\x81\x85` \x86\x01a7\tV[aAX\x81a71V[\x84\x01\x91PP\x92\x91PPV[_`@\x83\x01_\x83\x01QaAx_\x86\x01\x82aA\x02V[P` \x83\x01Q\x84\x82\x03` \x86\x01RaA\x90\x82\x82aA+V[\x91PP\x80\x91PP\x92\x91PPV[_aA\xA8\x83\x83aAcV[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aA\xC6\x82a@\xA2V[aA\xD0\x81\x85a@\xACV[\x93P\x83` \x82\x02\x85\x01aA\xE2\x85a@\xBCV[\x80_[\x85\x81\x10\x15aB\x1DW\x84\x84\x03\x89R\x81QaA\xFE\x85\x82aA\x9DV[\x94PaB\t\x83aA\xB0V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaA\xE5V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01RaBG\x81\x85a@/V[\x90P\x81\x81\x03` \x83\x01RaB[\x81\x84aA\xBCV[\x90P\x93\x92PPPV[_` \x82\x01\x90PaBw_\x83\x01\x84a>HV[\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aB\x97\x82aA\x11V[aB\xA1\x81\x85aB}V[\x93PaB\xB1\x81\x85` \x86\x01a7\tV[aB\xBA\x81a71V[\x84\x01\x91PP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01RaB\xDD\x81\x85a@/V[\x90P\x81\x81\x03` \x83\x01RaB\xF1\x81\x84aB\x8DV[\x90P\x93\x92PPPV[_` \x82\x84\x03\x12\x15aC\x0FWaC\x0Ea7\xA2V[[_aC\x1C\x84\x82\x85\x01a9\xAAV[\x91PP\x92\x91PPV[_\x81\x90P\x92\x91PPV[_aC9\x82a6\xEFV[aCC\x81\x85aC%V[\x93PaCS\x81\x85` \x86\x01a7\tV[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aC\x93`\x02\x83aC%V[\x91PaC\x9E\x82aC_V[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aC\xDD`\x01\x83aC%V[\x91PaC\xE8\x82aC\xA9V[`\x01\x82\x01\x90P\x91\x90PV[_aC\xFE\x82\x87aC/V[\x91PaD\t\x82aC\x87V[\x91PaD\x15\x82\x86aC/V[\x91PaD \x82aC\xD1V[\x91PaD,\x82\x85aC/V[\x91PaD7\x82aC\xD1V[\x91PaDC\x82\x84aC/V[\x91P\x81\x90P\x95\x94PPPPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[aDm\x81aDQV[\x82RPPV[_` \x82\x01\x90PaD\x86_\x83\x01\x84aDdV[\x92\x91PPV[_\x81Q\x90PaD\x9A\x81a;CV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aD\xB5WaD\xB4a7\xA2V[[_aD\xC2\x84\x82\x85\x01aD\x8CV[\x91PP\x92\x91PPV[_` \x82\x01\x90PaD\xDE_\x83\x01\x84a>WV[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_aE\x1B\x82a7\xAAV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03aEMWaELaD\xE4V[[`\x01\x82\x01\x90P\x91\x90PV[_``\x82\x01\x90PaEk_\x83\x01\x86a>HV[aEx` \x83\x01\x85a>HV[aE\x85`@\x83\x01\x84a9sV[\x94\x93PPPPV[_\x81\x15\x15\x90P\x91\x90PV[aE\xA1\x81aE\x8DV[\x81\x14aE\xABW_\x80\xFD[PV[_\x81Q\x90PaE\xBC\x81aE\x98V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aE\xD7WaE\xD6a7\xA2V[[_aE\xE4\x84\x82\x85\x01aE\xAEV[\x91PP\x92\x91PPV[_`@\x82\x01\x90PaF\0_\x83\x01\x85a>HV[aF\r` \x83\x01\x84a>WV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x825`\x01`@\x03\x836\x03\x03\x81\x12aFhWaFgaFAV[[\x80\x83\x01\x91PP\x92\x91PPV[`\x02\x81\x10aF\x80W_\x80\xFD[PV[_\x815aF\x8F\x81aFtV[\x80\x91PP\x91\x90PV[_\x81_\x1B\x90P\x91\x90PV[_`\xFFaF\xAF\x84aF\x98V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_aF\xCF\x82a@\xDFV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[aF\xE8\x82aF\xC5V[aF\xFBaF\xF4\x82aF\xD6V[\x83TaF\xA3V[\x82UPPPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aG\x1EWaG\x1DaFAV[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aG@WaG?aFEV[[` \x83\x01\x92P`\x01\x82\x026\x03\x83\x13\x15aG\\WaG[aFIV[[P\x92P\x92\x90PV[_\x82\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80aG\xB2W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aG\xC5WaG\xC4aGnV[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02aH'\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82aG\xECV[aH1\x86\x83aG\xECV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_aHlaHgaHb\x84a7\xAAV[aHIV[a7\xAAV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[aH\x85\x83aHRV[aH\x99aH\x91\x82aHsV[\x84\x84TaG\xF8V[\x82UPPPPV[_\x90V[aH\xADaH\xA1V[aH\xB8\x81\x84\x84aH|V[PPPV[[\x81\x81\x10\x15aH\xDBWaH\xD0_\x82aH\xA5V[`\x01\x81\x01\x90PaH\xBEV[PPV[`\x1F\x82\x11\x15aI WaH\xF1\x81aG\xCBV[aH\xFA\x84aG\xDDV[\x81\x01` \x85\x10\x15aI\tW\x81\x90P[aI\x1DaI\x15\x85aG\xDDV[\x83\x01\x82aH\xBDV[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_aI@_\x19\x84`\x08\x02aI%V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_aIX\x83\x83aI1V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[aIr\x83\x83aGdV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aI\x8BWaI\x8Aa;qV[[aI\x95\x82TaG\x9BV[aI\xA0\x82\x82\x85aH\xDFV[_`\x1F\x83\x11`\x01\x81\x14aI\xCDW_\x84\x15aI\xBBW\x82\x87\x015\x90P[aI\xC5\x85\x82aIMV[\x86UPaJ,V[`\x1F\x19\x84\x16aI\xDB\x86aG\xCBV[_[\x82\x81\x10\x15aJ\x02W\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PaI\xDDV[\x86\x83\x10\x15aJ\x1FW\x84\x89\x015aJ\x1B`\x1F\x89\x16\x82aI1V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[aJ@\x83\x83\x83aIhV[PPPV[_\x81\x01_\x83\x01\x80aJU\x81aF\x83V[\x90PaJa\x81\x84aF\xDFV[PPP`\x01\x81\x01` \x83\x01aJv\x81\x85aG\x02V[aJ\x81\x81\x83\x86aJ5V[PPPPPPV[aJ\x93\x82\x82aJEV[PPV[_\x80\xFD[_\x80\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aJ\xB9WaJ\xB8a;qV[[aJ\xC2\x82a71V[\x90P` \x81\x01\x90P\x91\x90PV[_aJ\xE1aJ\xDC\x84aJ\x9FV[a;\xCFV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15aJ\xFDWaJ\xFCa;mV[[aK\x08\x84\x82\x85a7\tV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aK$WaK#a9\xFCV[[\x81QaK4\x84\x82` \x86\x01aJ\xCFV[\x91PP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15aKRWaKQaJ\x97V[[aK\\`\x80a;\xCFV[\x90P_aKk\x84\x82\x85\x01aD\x8CV[_\x83\x01RP` aK~\x84\x82\x85\x01aD\x8CV[` \x83\x01RP`@\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aK\xA2WaK\xA1aJ\x9BV[[aK\xAE\x84\x82\x85\x01aK\x10V[`@\x83\x01RP``\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aK\xD2WaK\xD1aJ\x9BV[[aK\xDE\x84\x82\x85\x01aK\x10V[``\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15aK\xFFWaK\xFEa7\xA2V[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aL\x1CWaL\x1Ba7\xA6V[[aL(\x84\x82\x85\x01aK=V[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[_\x815\x90PaLH\x81aFtV[\x92\x91PPV[_aL\\` \x84\x01\x84aL:V[\x90P\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aL\x8CWaL\x8BaLlV[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aL\xB4WaL\xB3aLdV[[`\x01\x82\x026\x03\x83\x13\x15aL\xCAWaL\xC9aLhV[[P\x92P\x92\x90PV[_aL\xDD\x83\x85aA\x1BV[\x93PaL\xEA\x83\x85\x84a<\x19V[aL\xF3\x83a71V[\x84\x01\x90P\x93\x92PPPV[_`@\x83\x01aM\x0F_\x84\x01\x84aLNV[aM\x1B_\x86\x01\x82aA\x02V[PaM)` \x84\x01\x84aLpV[\x85\x83\x03` \x87\x01RaM<\x83\x82\x84aL\xD2V[\x92PPP\x80\x91PP\x92\x91PPV[_aMU\x83\x83aL\xFEV[\x90P\x92\x91PPV[_\x825`\x01`@\x03\x836\x03\x03\x81\x12aMxWaMwaLlV[[\x82\x81\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aM\x9B\x83\x85a@\xACV[\x93P\x83` \x84\x02\x85\x01aM\xAD\x84aL1V[\x80_[\x87\x81\x10\x15aM\xF0W\x84\x84\x03\x89RaM\xC7\x82\x84aM]V[aM\xD1\x85\x82aMJV[\x94PaM\xDC\x83aM\x84V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaM\xB0V[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_``\x82\x01\x90PaN\x15_\x83\x01\x87a>HV[\x81\x81\x03` \x83\x01RaN'\x81\x86a@/V[\x90P\x81\x81\x03`@\x83\x01RaN<\x81\x84\x86aM\x90V[\x90P\x95\x94PPPPPV[_`@\x82\x01\x90PaNZ_\x83\x01\x85a>HV[aNg` \x83\x01\x84a>HV[\x93\x92PPPV[_aNy\x83\x85aB}V[\x93PaN\x86\x83\x85\x84a<\x19V[aN\x8F\x83a71V[\x84\x01\x90P\x93\x92PPPV[_``\x82\x01\x90PaN\xAD_\x83\x01\x87a>HV[\x81\x81\x03` \x83\x01RaN\xBF\x81\x86a@/V[\x90P\x81\x81\x03`@\x83\x01RaN\xD4\x81\x84\x86aNnV[\x90P\x95\x94PPPPPV[\x7FEIP712: Uninitialized\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aO\x13`\x15\x83a6\xF9V[\x91PaO\x1E\x82aN\xDFV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaO@\x81aO\x07V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aO\x89WaO\x88a7\xA2V[[_aO\x96\x84\x82\x85\x01aL:V[\x91PP\x92\x91PPV[_\x81\x90P\x92\x91PPV[_aO\xB4\x83\x85aO\x9FV[\x93PaO\xC1\x83\x85\x84a<\x19V[\x82\x84\x01\x90P\x93\x92PPPV[_aO\xD9\x82\x84\x86aO\xA9V[\x91P\x81\x90P\x93\x92PPPV[aO\xEE\x81a@\xF1V[\x82RPPV[_``\x82\x01\x90PaP\x07_\x83\x01\x86a<\xF8V[aP\x14` \x83\x01\x85aO\xE5V[aP!`@\x83\x01\x84a<\xF8V[\x94\x93PPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[aPU\x81a<\xEFV[\x82RPPV[_aPf\x83\x83aPLV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aP\x88\x82aP)V[aP\x92\x81\x85aP3V[\x93PaP\x9D\x83aP=V[\x80_[\x83\x81\x10\x15aP\xCDW\x81QaP\xB4\x88\x82aP[V[\x97PaP\xBF\x83aPrV[\x92PP`\x01\x81\x01\x90PaP\xA0V[P\x85\x93PPPP\x92\x91PPV[_aP\xE5\x82\x84aP~V[\x91P\x81\x90P\x92\x91PPV[_`\x80\x82\x01\x90PaQ\x03_\x83\x01\x87a<\xF8V[aQ\x10` \x83\x01\x86a>HV[aQ\x1D`@\x83\x01\x85a>HV[aQ*``\x83\x01\x84a<\xF8V[\x95\x94PPPPPV[_\x81Q\x90PaQA\x81a7\xB3V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aQ\\WaQ[a7\xA2V[[_aQi\x84\x82\x85\x01aQ3V[\x91PP\x92\x91PPV[aQ{\x81a<\xEFV[\x81\x14aQ\x85W_\x80\xFD[PV[_\x81Q\x90PaQ\x96\x81aQrV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aQ\xB1WaQ\xB0a7\xA2V[[_aQ\xBE\x84\x82\x85\x01aQ\x88V[\x91PP\x92\x91PPV[_`@\x82\x01\x90PaQ\xDA_\x83\x01\x85a<\xF8V[aQ\xE7` \x83\x01\x84a>HV[\x93\x92PPPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[`\x1F\x82\x11\x15aRAWaR\x12\x81aQ\xEEV[aR\x1B\x84aG\xDDV[\x81\x01` \x85\x10\x15aR*W\x81\x90P[aR>aR6\x85aG\xDDV[\x83\x01\x82aH\xBDV[PP[PPPV[aRO\x82a6\xEFV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aRhWaRga;qV[[aRr\x82TaG\x9BV[aR}\x82\x82\x85aR\0V[_` \x90P`\x1F\x83\x11`\x01\x81\x14aR\xAEW_\x84\x15aR\x9CW\x82\x87\x01Q\x90P[aR\xA6\x85\x82aIMV[\x86UPaS\rV[`\x1F\x19\x84\x16aR\xBC\x86aQ\xEEV[_[\x82\x81\x10\x15aR\xE3W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PaR\xBEV[\x86\x83\x10\x15aS\0W\x84\x89\x01QaR\xFC`\x1F\x89\x16\x82aI1V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_aS\x1F\x82aA\x11V[aS)\x81\x85aO\x9FV[\x93PaS9\x81\x85` \x86\x01a7\tV[\x80\x84\x01\x91PP\x92\x91PPV[_aSP\x82\x84aS\x15V[\x91P\x81\x90P\x92\x91PPV[_`\xA0\x82\x01\x90PaSn_\x83\x01\x88a<\xF8V[aS{` \x83\x01\x87a<\xF8V[aS\x88`@\x83\x01\x86a<\xF8V[aS\x95``\x83\x01\x85a>HV[aS\xA2`\x80\x83\x01\x84a>WV[\x96\x95PPPPPPV[_`\xFF\x82\x16\x90P\x91\x90PV[aS\xC1\x81aS\xACV[\x82RPPV[_`\x80\x82\x01\x90PaS\xDA_\x83\x01\x87a<\xF8V[aS\xE7` \x83\x01\x86aS\xB8V[aS\xF4`@\x83\x01\x85a<\xF8V[aT\x01``\x83\x01\x84a<\xF8V[\x95\x94PPPPPV\xFECrsgenVerification(uint256 crsId,uint256 maxBitLength,bytes crsDigest)PrepKeygenVerification(uint256 prepKeygenId)KeygenVerification(uint256 prepKeygenId,uint256 keyId,KeyDigest[] keyDigests)KeyDigest(uint8 keyType,bytes digest)KeyDigest(uint8 keyType,bytes digest)",
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
            [132u8, 176u8, 25u8, 110u8],
            [147u8, 102u8, 8u8, 174u8],
            [173u8, 60u8, 177u8, 204u8],
            [186u8, 255u8, 33u8, 30u8],
            [197u8, 91u8, 135u8, 36u8],
            [202u8, 163u8, 103u8, 219u8],
            [213u8, 47u8, 16u8, 235u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for KMSGenerationCalls {
        const NAME: &'static str = "KMSGenerationCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 18usize;
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
        PrepKeygenRequest(PrepKeygenRequest),
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
        const COUNT: usize = 8usize;
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
                Some(
                    <PrepKeygenRequest as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PrepKeygenRequest as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PrepKeygenRequest)
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
                Self::PrepKeygenRequest(inner) => {
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
                Self::PrepKeygenRequest(inner) => {
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
        ///Creates a new event filter for the [`PrepKeygenRequest`] event.
        pub fn PrepKeygenRequest_filter(
            &self,
        ) -> alloy_contract::Event<&P, PrepKeygenRequest, N> {
            self.event_filter::<PrepKeygenRequest>()
        }
        ///Creates a new event filter for the [`Upgraded`] event.
        pub fn Upgraded_filter(&self) -> alloy_contract::Event<&P, Upgraded, N> {
            self.event_filter::<Upgraded>()
        }
    }
}
